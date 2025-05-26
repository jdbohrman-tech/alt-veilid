use super::*;

impl RoutingTable {
    /// Utility to find the closest nodes to a particular key, preferring reliable nodes first,
    /// including possibly our own node and nodes further away from the key than our own,
    /// returning their peer info
    #[instrument(level = "trace", target = "rtab", skip_all)]
    pub fn find_preferred_closest_peers(
        &self,
        routing_domain: RoutingDomain,
        key: TypedNodeId,
        capabilities: &[VeilidCapability],
    ) -> NetworkResult<Vec<Arc<PeerInfo>>> {
        if Crypto::validate_crypto_kind(key.kind).is_err() {
            return NetworkResult::invalid_message("invalid crypto kind");
        }

        let Some(published_peer_info) = self.get_published_peer_info(routing_domain) else {
            return NetworkResult::service_unavailable(
                "Not finding closest peers because our network class is still invalid",
            );
        };

        // find N nodes closest to the target node in our routing table
        let filter = Box::new(
            |rti: &RoutingTableInner, opt_entry: Option<Arc<BucketEntry>>| {
                // Ensure only things that are valid/signed in the chosen routing domain are returned
                if !rti.filter_has_valid_signed_node_info(routing_domain, true, opt_entry.clone()) {
                    return false;
                }
                // Ensure capabilities are met
                match opt_entry {
                    Some(entry) => entry.with(rti, |_rti, e| {
                        e.has_all_capabilities(routing_domain, capabilities)
                    }),
                    None => published_peer_info
                        .signed_node_info()
                        .node_info()
                        .has_all_capabilities(capabilities),
                }
            },
        ) as RoutingTableEntryFilter;
        let filters = VecDeque::from([filter]);

        let node_count = self
            .config()
            .with(|c| c.network.dht.max_find_node_count as usize);

        let closest_nodes = match self.find_preferred_closest_nodes(
            node_count,
            key.into(),
            filters,
            // transform
            |rti, entry| {
                rti.transform_to_peer_info(routing_domain, published_peer_info.clone(), entry)
            },
        ) {
            Ok(v) => v,
            Err(e) => {
                error!("failed to find closest nodes for key {}: {}", key, e);
                return NetworkResult::invalid_message("failed to find closest nodes for key");
            }
        };

        NetworkResult::value(closest_nodes)
    }

    /// Utility to find nodes that are closer to a key than our own node,
    /// preferring reliable nodes first, and returning their peer info
    /// Can filter based on a particular set of capabilities
    #[instrument(level = "trace", target = "rtab", skip_all)]
    pub fn find_preferred_peers_closer_to_key(
        &self,
        routing_domain: RoutingDomain,
        key: TypedRecordKey,
        required_capabilities: Vec<VeilidCapability>,
    ) -> NetworkResult<Vec<Arc<PeerInfo>>> {
        // add node information for the requesting node to our routing table
        let crypto_kind = key.kind;
        let own_node_id = self.node_id(crypto_kind);

        // find N nodes closest to the target node in our routing table
        // ensure the nodes returned are only the ones closer to the target node than ourself
        let crypto = self.crypto();
        let Some(vcrypto) = crypto.get(crypto_kind) else {
            return NetworkResult::invalid_message("unsupported cryptosystem");
        };
        let vcrypto = &vcrypto;

        let own_distance = vcrypto.distance(
            &HashDigest::from(own_node_id.value),
            &HashDigest::from(key.value),
        );

        let filter = Box::new(
            move |rti: &RoutingTableInner, opt_entry: Option<Arc<BucketEntry>>| {
                // Exclude our own node
                let Some(entry) = opt_entry else {
                    return false;
                };
                // Ensure only things that have a minimum set of capabilities are returned
                entry.with(rti, |rti, e| {
                    if !e.has_all_capabilities(routing_domain, &required_capabilities) {
                        return false;
                    }
                    // Ensure only things that are valid/signed in the PublicInternet domain are returned
                    if !rti.filter_has_valid_signed_node_info(
                        routing_domain,
                        true,
                        Some(entry.clone()),
                    ) {
                        return false;
                    }
                    // Ensure things further from the key than our own node are not included
                    let Some(entry_node_id) = e.node_ids().get(crypto_kind) else {
                        return false;
                    };
                    let entry_distance = vcrypto.distance(
                        &HashDigest::from(entry_node_id.value),
                        &HashDigest::from(key.value),
                    );
                    if entry_distance >= own_distance {
                        return false;
                    }
                    true
                })
            },
        ) as RoutingTableEntryFilter;
        let filters = VecDeque::from([filter]);

        let node_count = self
            .config()
            .with(|c| c.network.dht.max_find_node_count as usize);

        //
        let closest_nodes = match self.find_preferred_closest_nodes(
            node_count,
            key.into(),
            filters,
            // transform
            |rti, entry| {
                entry
                    .unwrap()
                    .with(rti, |_rti, e| e.get_peer_info(routing_domain).unwrap())
            },
        ) {
            Ok(v) => v,
            Err(e) => {
                error!("failed to find closest nodes for key {}: {}", key, e);
                return NetworkResult::invalid_message("failed to find closest nodes for key");
            }
        };

        // Validate peers returned are, in fact, closer to the key than the node we sent this to
        // This same test is used on the other side so we vet things here
        let valid = match Self::verify_peers_closer(
            vcrypto,
            own_node_id.into(),
            key.into(),
            &closest_nodes,
        ) {
            Ok(v) => v,
            Err(e) => {
                panic!("missing cryptosystem in peers node ids: {}", e);
            }
        };
        if !valid {
            error!(
                "non-closer peers returned: own_node_id={:#?} key={:#?} closest_nodes={:#?}",
                own_node_id, key, closest_nodes
            );
        }

        NetworkResult::value(closest_nodes)
    }

    /// Determine if set of peers is closer to key_near than key_far is to key_near
    #[instrument(level = "trace", target = "rtab", skip_all, err)]
    pub fn verify_peers_closer(
        vcrypto: &crypto::CryptoSystemGuard<'_>,
        key_far: TypedHashDigest,
        key_near: TypedHashDigest,
        peers: &[Arc<PeerInfo>],
    ) -> EyreResult<bool> {
        let kind = vcrypto.kind();

        if key_far.kind != kind || key_near.kind != kind {
            bail!("keys all need the same cryptosystem");
        }

        let mut closer = true;
        let d_far = vcrypto.distance(&key_far.value, &key_near.value);
        for peer in peers {
            let Some(key_peer) = peer.node_ids().get(kind) else {
                bail!("peers need to have a key with the same cryptosystem");
            };
            let d_near = vcrypto.distance(&key_near.value, &key_peer.value.into());
            if d_far < d_near {
                let warning = format!(
                    r#"peer: {}
near (key): {}
far (self): {}
    d_near: {}
     d_far: {}
       cmp: {:?}"#,
                    key_peer.value,
                    key_near.value,
                    key_far.value,
                    d_near,
                    d_far,
                    d_near.cmp(&d_far)
                );
                let crypto = vcrypto.crypto();
                veilid_log!(crypto warn "{}", warning);
                closer = false;
                break;
            }
        }

        Ok(closer)
    }
}
