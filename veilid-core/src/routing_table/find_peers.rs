use super::*;

impl RoutingTable {
    /// Utility to find the closest nodes to a particular key, preferring reliable nodes first,
    /// including possibly our own node and nodes further away from the key than our own,
    /// returning their peer info
    pub fn find_preferred_closest_peers(
        &self,
        key: TypedKey,
        capabilities: &[Capability],
    ) -> NetworkResult<Vec<PeerInfo>> {
        if !self.has_valid_network_class(RoutingDomain::PublicInternet) {
            // Our own node info is not yet available, drop this request.
            return NetworkResult::service_unavailable(
                "Not finding closest peers because our network class is still invalid",
            );
        }
        if Crypto::validate_crypto_kind(key.kind).is_err() {
            return NetworkResult::invalid_message("invalid crypto kind");
        }

        // find N nodes closest to the target node in our routing table
        let own_peer_info = self.get_own_peer_info(RoutingDomain::PublicInternet);
        let filter = Box::new(
            move |rti: &RoutingTableInner, opt_entry: Option<Arc<BucketEntry>>| {
                // Ensure only things that are valid/signed in the PublicInternet domain are returned
                if !rti.filter_has_valid_signed_node_info(
                    RoutingDomain::PublicInternet,
                    true,
                    opt_entry.clone(),
                ) {
                    return false;
                }
                // Ensure capabilities are met
                match opt_entry {
                    Some(entry) => entry.with(rti, |_rti, e| {
                        e.has_capabilities(RoutingDomain::PublicInternet, capabilities)
                    }),
                    None => own_peer_info
                        .signed_node_info()
                        .node_info()
                        .has_capabilities(capabilities),
                }
            },
        ) as RoutingTableEntryFilter;
        let filters = VecDeque::from([filter]);

        let node_count = {
            let c = self.config.get();
            c.network.dht.max_find_node_count as usize
        };

        let own_peer_info = self.get_own_peer_info(RoutingDomain::PublicInternet);
        let closest_nodes = match self.find_preferred_closest_nodes(
            node_count,
            key,
            filters,
            // transform
            |rti, entry| {
                rti.transform_to_peer_info(RoutingDomain::PublicInternet, &own_peer_info, entry)
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
    /// Can filter based on a particular set of capabiltiies
    pub fn find_preferred_peers_closer_to_key(
        &self,
        key: TypedKey,
        required_capabilities: Vec<Capability>,
    ) -> NetworkResult<Vec<PeerInfo>> {
        // add node information for the requesting node to our routing table
        let crypto_kind = key.kind;
        let own_node_id = self.node_id(crypto_kind);

        // find N nodes closest to the target node in our routing table
        // ensure the nodes returned are only the ones closer to the target node than ourself
        let Some(vcrypto) = self.crypto().get(crypto_kind) else {
            return NetworkResult::invalid_message("unsupported cryptosystem");
        };
        let own_distance = vcrypto.distance(&own_node_id.value, &key.value);
        let vcrypto2 = vcrypto.clone();

        let filter = Box::new(
            move |rti: &RoutingTableInner, opt_entry: Option<Arc<BucketEntry>>| {
                // Exclude our own node
                let Some(entry) = opt_entry else {
                    return false;
                };
                // Ensure only things that have a minimum set of capabilities are returned
                entry.with(rti, |rti, e| {
                    if !e.has_capabilities(RoutingDomain::PublicInternet, &required_capabilities) {
                        return false;
                    }
                    // Ensure only things that are valid/signed in the PublicInternet domain are returned
                    if !rti.filter_has_valid_signed_node_info(
                        RoutingDomain::PublicInternet,
                        true,
                        Some(entry.clone()),
                    ) {
                        return false;
                    }
                    // Ensure things further from the key than our own node are not included
                    let Some(entry_node_id) = e.node_ids().get(crypto_kind) else {
                        return false;
                    };
                    let entry_distance = vcrypto.distance(&entry_node_id.value, &key.value);
                    if entry_distance >= own_distance {
                        return false;
                    }
                    true
                })
            },
        ) as RoutingTableEntryFilter;
        let filters = VecDeque::from([filter]);

        let node_count = {
            let c = self.config.get();
            c.network.dht.max_find_node_count as usize
        };

        //
        let closest_nodes = match self.find_preferred_closest_nodes(
            node_count,
            key,
            filters,
            // transform
            |rti, entry| {
                entry.unwrap().with(rti, |_rti, e| {
                    e.make_peer_info(RoutingDomain::PublicInternet).unwrap()
                })
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
        let valid = match Self::verify_peers_closer(vcrypto2, own_node_id, key, &closest_nodes) {
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
    pub(crate) fn verify_peers_closer(
        vcrypto: CryptoSystemVersion,
        key_far: TypedKey,
        key_near: TypedKey,
        peers: &[PeerInfo],
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
            let d_near = vcrypto.distance(&key_near.value, &key_peer.value);
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
                warn!("{}", warning);
                closer = false;
                break;
            }
        }

        Ok(closer)
    }
}
