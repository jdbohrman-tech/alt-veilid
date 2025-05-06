mod v0;
mod v1;

use super::*;
use v1::*;

impl_veilid_log_facility!("net");

impl NetworkManager {
    /// Direct bootstrap request
    /// Sends a bootstrap request to a dialinfo and returns the list of peers to bootstrap with
    /// If no bootstrap keys are specified, uses the v0 mechanism, otherwise uses the v1 mechanism
    #[instrument(level = "trace", target = "net", err, skip(self))]
    pub async fn direct_bootstrap(&self, dial_info: DialInfo) -> EyreResult<Vec<Arc<PeerInfo>>> {
        let direct_boot_version = self.config().with(|c| {
            if c.network.routing_table.bootstrap_keys.is_empty() {
                0
            } else {
                1
            }
        });

        if direct_boot_version == 0 {
            self.direct_bootstrap_v0(dial_info).await
        } else {
            self.direct_bootstrap_v1(dial_info).await
        }
    }

    /// Uses the bootstrap v0 (BOOT) mechanism
    #[instrument(level = "trace", target = "net", err, skip(self))]
    async fn direct_bootstrap_v0(&self, dial_info: DialInfo) -> EyreResult<Vec<Arc<PeerInfo>>> {
        let timeout_ms = self.config().with(|c| c.network.rpc.timeout_ms);
        // Send boot magic to requested peer address
        let data = BOOT_MAGIC.to_vec();

        let out_data: Vec<u8> = network_result_value_or_log!(self self
            .net()
            .send_recv_data_unbound_to_dial_info(dial_info, data, timeout_ms)
            .await? => [ format!(": dial_info={}, data.len={}", dial_info, data.len()) ]
        {
            return Ok(Vec::new());
        });

        let bootstrap_peerinfo_str =
            std::str::from_utf8(&out_data).wrap_err("bad utf8 in boot peerinfo")?;

        let bootstrap_peerinfo: Vec<PeerInfo> = match deserialize_json(bootstrap_peerinfo_str) {
            Ok(v) => v,
            Err(e) => {
                error!("{}", e);
                return Err(e).wrap_err("failed to deserialize peerinfo");
            }
        };

        Ok(bootstrap_peerinfo.into_iter().map(Arc::new).collect())
    }

    /// Uses the bootstrap v1 (B01T) mechanism
    #[instrument(level = "trace", target = "net", err, skip(self))]
    async fn direct_bootstrap_v1(&self, dial_info: DialInfo) -> EyreResult<Vec<Arc<PeerInfo>>> {
        let timeout_ms = self.config().with(|c| c.network.rpc.timeout_ms);

        // Send boot magic to requested peer address
        let data = B01T_MAGIC.to_vec();

        let out_data: Vec<u8> = network_result_value_or_log!(self self
            .net()
            .send_recv_data_unbound_to_dial_info(dial_info, data, timeout_ms)
            .await? => [ format!(": dial_info={}, data.len={}", dial_info, data.len()) ]
        {
            return Ok(Vec::new());
        });

        let bootv1response_str =
            std::str::from_utf8(&out_data).wrap_err("bad utf8 in bootstrap v1 records")?;

        veilid_log!(self debug "Direct bootstrap v1 response: {}", bootv1response_str);

        let bootv1response: BootV1Response = match deserialize_json(bootv1response_str) {
            Ok(v) => v,
            Err(e) => {
                error!("{}", e);
                return Err(e).wrap_err("failed to deserialize bootstrap v1 response");
            }
        };

        // Parse v1 records
        let bsrecs = match self.parse_bootstrap_v1(&bootv1response.records) {
            Ok(v) => v,
            Err(e) => {
                veilid_log!(self debug "Direct bootstrap v1 parsing failure: {}", e);
                return Err(e);
            }
        };

        veilid_log!(self debug "Direct bootstrap v1 resolution: {:#?}", bsrecs);

        // Returned bootstrapped peers
        let routing_table = self.routing_table();

        let peers: Vec<Arc<PeerInfo>> = bsrecs
            .into_iter()
            .filter_map(|bsrec| {
                if routing_table.matches_own_node_id(bsrec.node_ids()) {
                    veilid_log!(self debug "Ignoring own node in bootstrap list");
                    None
                } else {
                    // If signed peer info exists for this record, use it
                    // This is important for browser websocket bootstrapping where the
                    // dialinfo in the bootstrap record has an unspecified IP address,
                    // and as such, a routing domain can not be determined for it
                    // by the code that receives the FindNodeA result
                    for pi in bootv1response.peers.iter().cloned() {
                        if pi.node_ids().contains_any(bsrec.node_ids()) {
                            return Some(pi);
                        }
                    }

                    // Otherwise use an unsigned peerinfo and try to resolve it directly from the bootstrap record
                    // The bootstrap will be rejected if a FindNodeQ could not resolve the peer info

                    // Get crypto support from list of node ids
                    let crypto_support = bsrec.node_ids().kinds();

                    // Make unsigned SignedNodeInfo
                    let sni = SignedNodeInfo::Direct(SignedDirectNodeInfo::with_no_signature(
                        NodeInfo::new(
                            NetworkClass::InboundCapable, // Bootstraps are always inbound capable
                            ProtocolTypeSet::all(), // Bootstraps are always capable of all protocols
                            AddressTypeSet::all(),  // Bootstraps are always IPV4 and IPV6 capable
                            bsrec.envelope_support().to_vec(), // Envelope support is as specified in the bootstrap list
                            crypto_support, // Crypto support is derived from list of node ids
                            vec![],         // Bootstrap needs no capabilities
                            bsrec.dial_info_details().to_vec(), // Dial info is as specified in the bootstrap list
                        ),
                    ));

                    Some(Arc::new(PeerInfo::new(
                        RoutingDomain::PublicInternet,
                        bsrec.node_ids().clone(),
                        sni,
                    )))
                }
            })
            .collect();

        Ok(peers)
    }
}
