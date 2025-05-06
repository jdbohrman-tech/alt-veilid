mod v0;
mod v1;

use super::*;

pub use v0::*;
pub use v1::*;

const SUPPORTED_BOOTSTRAP_TXT_VERSIONS: [u8; 2] =
    [BOOTSTRAP_TXT_VERSION_0, BOOTSTRAP_TXT_VERSION_1];

impl_veilid_log_facility!("net");

impl NetworkManager {
    /// TXT bootstrap request
    /// Sends a bootstrap request via DNS for TXT records and bootstraps with them
    #[instrument(level = "trace", target = "net", err, skip(self))]
    pub async fn txt_bootstrap(&self, hostname: String) -> EyreResult<Vec<Arc<PeerInfo>>> {
        // Get the minimum bootstrap version we are supporting
        // If no keys are available, allow v0.
        // If bootstrap keys are specified, require at least v1.
        let min_boot_version = self.config().with(|c| {
            if c.network.routing_table.bootstrap_keys.is_empty() {
                BOOTSTRAP_TXT_VERSION_0
            } else {
                BOOTSTRAP_TXT_VERSION_1
            }
        });

        // Resolve bootstrap servers and recurse their TXT entries
        // This only operates in the PublicInternet routing domain because DNS is inherently
        // for PublicInternet use. Other routing domains may use other mechanisms
        // such as LLMNR/MDNS/DNS-SD.
        let txt_strings =
            match pin_future!(self.resolve_bootstrap_txt_strings(hostname.clone())).await {
                Ok(v) => v,
                Err(e) => {
                    veilid_log!(self debug "Bootstrap resolution failure: {}", e);
                    return Err(e);
                }
            };

        // Heuristic to determine which version to parse
        // Take the first record and see if there is a '|' delimited string in it
        // If so, the first field is a record version number.
        // If no '|' delimited string is found, then this is a v0 hostname record
        let mut opt_max_version: Option<u8> = None;

        for txt_string in &txt_strings {
            let v = match txt_string.split_once('|').map(|x| u8::from_str(x.0)) {
                Some(Err(e)) => {
                    // Parse error, skip it
                    veilid_log!(self debug "malformed txt record in bootstrap response: {}\n{}", txt_string, e);
                    continue;
                }
                Some(Ok(v)) => {
                    // Version
                    if SUPPORTED_BOOTSTRAP_TXT_VERSIONS.contains(&v) {
                        v
                    } else {
                        veilid_log!(self debug "unsupported bootstrap record version: {}", txt_string);
                        continue;
                    }
                }
                None => {
                    // No '|' means try version 0
                    0u8
                }
            };

            if v < min_boot_version {
                veilid_log!(self debug "ignoring older bootstrap record version: {}", txt_string);
                continue;
            }

            opt_max_version = opt_max_version.map(|x| x.max(v)).or(Some(v));
        }
        let Some(max_version) = opt_max_version else {
            veilid_log!(self debug "no suitable txt record in bootstrap response");
            return Ok(vec![]);
        };

        // Process the best version available
        let bsrecs = match max_version {
            BOOTSTRAP_TXT_VERSION_0 => {
                // Resolve second-level hostname
                let record_strings = self.resolve_bootstrap_v0(hostname, txt_strings).await?;

                // Parse v0 records
                let bsrecs = match self.parse_bootstrap_v0(&record_strings) {
                    Ok(v) => v,
                    Err(e) => {
                        veilid_log!(self debug "Bootstrap v0 parsing failure: {}", e);
                        return Err(e);
                    }
                };

                veilid_log!(self debug "Bootstrap v0 resolution: {:#?}", bsrecs);

                bsrecs
            }
            BOOTSTRAP_TXT_VERSION_1 => {
                // Parse v1 records
                let bsrecs = match self.parse_bootstrap_v1(&txt_strings) {
                    Ok(v) => v,
                    Err(e) => {
                        veilid_log!(self debug "Bootstrap v1 parsing failure: {}", e);
                        return Err(e);
                    }
                };

                veilid_log!(self debug "Bootstrap v1 resolution: {:#?}", bsrecs);

                bsrecs
            }
            _ => {
                veilid_log!(self debug "unsupported bootstrap version");
                return Ok(vec![]);
            }
        };

        let routing_table = self.routing_table();

        let peers: Vec<Arc<PeerInfo>> = bsrecs
            .into_iter()
            .filter_map(|bsrec| {
                if routing_table.matches_own_node_id(bsrec.node_ids()) {
                    veilid_log!(self debug "Ignoring own node in bootstrap list");
                    None
                } else {
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

    /// Bootstrap resolution from TXT into strings
    /// This is cached as to minimize the number of outbound network requests on bootstrap servers
    #[instrument(level = "trace", skip(self), ret, err)]
    pub async fn resolve_bootstrap_txt_strings(&self, hostname: String) -> EyreResult<Vec<String>> {
        // Lookup hostname in cache
        let cur_ts = Timestamp::now();
        if let Some(res) = self.inner.lock().txt_lookup_cache.get(&hostname) {
            // Ensure timestamp has not expired
            if cur_ts < res.0 {
                // Return cached strings
                let txt_strings = res.1.clone();
                veilid_log!(self debug " Cached TXT: {:?} => {:?}", hostname, txt_strings);
                return Ok(txt_strings);
            }
        }

        // Not in cache or cache expired
        veilid_log!(self debug "Resolving bootstrap TXT: {:?}", hostname);

        // Get TXT record for bootstrap (bootstrap.veilid.net, bootstrap-v1.veilid.net, or similar)
        let txt_strings = match intf::txt_lookup(&hostname).await {
            Ok(v) => v,
            Err(e) => {
                veilid_log!(self warn
                    "Network may be down. No bootstrap resolution for '{}': {}",
                    hostname, e
                );
                return Ok(vec![]);
            }
        };
        veilid_log!(self debug " TXT: {:?} => {:?}", hostname, txt_strings);

        // Cache result if we have one
        if !txt_strings.is_empty() {
            let exp_ts = cur_ts + TXT_LOOKUP_EXPIRATION;
            self.inner
                .lock()
                .txt_lookup_cache
                .insert(hostname, (exp_ts, txt_strings.clone()));
        }

        Ok(txt_strings)
    }
}
