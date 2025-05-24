use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SignedNodeInfo {
    Direct(SignedDirectNodeInfo),
    Relayed(SignedRelayedNodeInfo),
}

impl fmt::Display for SignedNodeInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Direct(arg0) => {
                writeln!(f, "direct:")?;
                write!(f, "{}", indent_all_string(arg0))?;
                Ok(())
            }
            Self::Relayed(arg0) => {
                writeln!(f, "relayed:")?;
                write!(f, "{}", indent_all_string(&arg0))?;
                Ok(())
            }
        }
    }
}

impl SignedNodeInfo {
    pub fn validate(
        &self,
        node_ids: &TypedNodeIdGroup,
        crypto: &Crypto,
    ) -> VeilidAPIResult<TypedNodeIdGroup> {
        match self {
            SignedNodeInfo::Direct(d) => d.validate(node_ids, crypto),
            SignedNodeInfo::Relayed(r) => r.validate(node_ids, crypto),
        }
    }

    pub fn has_any_signature(&self) -> bool {
        match self {
            SignedNodeInfo::Direct(d) => d.has_any_signature(),
            SignedNodeInfo::Relayed(r) => r.has_any_signature(),
        }
    }

    pub fn timestamp(&self) -> Timestamp {
        match self {
            SignedNodeInfo::Direct(d) => d.timestamp(),
            SignedNodeInfo::Relayed(r) => r.timestamp(),
        }
    }
    pub fn node_info(&self) -> &NodeInfo {
        match self {
            SignedNodeInfo::Direct(d) => d.node_info(),
            SignedNodeInfo::Relayed(r) => r.node_info(),
        }
    }
    pub fn relay_ids(&self) -> TypedNodeIdGroup {
        match self {
            SignedNodeInfo::Direct(_) => TypedNodeIdGroup::new(),
            SignedNodeInfo::Relayed(r) => r.relay_ids().clone(),
        }
    }
    pub fn relay_info(&self) -> Option<&NodeInfo> {
        match self {
            SignedNodeInfo::Direct(_) => None,
            SignedNodeInfo::Relayed(r) => Some(r.relay_info().node_info()),
        }
    }
    pub fn relay_peer_info(&self, routing_domain: RoutingDomain) -> Option<Arc<PeerInfo>> {
        match self {
            SignedNodeInfo::Direct(_) => None,
            SignedNodeInfo::Relayed(r) => Some(Arc::new(PeerInfo::new(
                routing_domain,
                r.relay_ids().clone(),
                SignedNodeInfo::Direct(r.relay_info().clone()),
            ))),
        }
    }
    pub fn has_any_dial_info(&self) -> bool {
        self.node_info().has_dial_info()
            || self
                .relay_info()
                .map(|relay_ni| relay_ni.has_dial_info())
                .unwrap_or_default()
    }

    pub fn has_sequencing_matched_dial_info(&self, sequencing: Sequencing) -> bool {
        // Check our dial info
        for did in self.node_info().dial_info_detail_list() {
            match sequencing {
                Sequencing::NoPreference | Sequencing::PreferOrdered => return true,
                Sequencing::EnsureOrdered => {
                    if did.dial_info.protocol_type().is_ordered() {
                        return true;
                    }
                }
            }
        }
        // Check our relay if we have one
        self.relay_info()
            .map(|relay_ni| {
                for did in relay_ni.dial_info_detail_list() {
                    match sequencing {
                        Sequencing::NoPreference | Sequencing::PreferOrdered => return true,
                        Sequencing::EnsureOrdered => {
                            if did.dial_info.protocol_type().is_ordered() {
                                return true;
                            }
                        }
                    }
                }
                false
            })
            .unwrap_or_default()
    }

    #[cfg(feature = "geolocation")]
    /// Get geolocation info of node and its relays.
    pub fn get_geolocation_info(&self, routing_domain: RoutingDomain) -> GeolocationInfo {
        if routing_domain != RoutingDomain::PublicInternet {
            // Country code is irrelevant for local network
            return GeolocationInfo::new(None, vec![]);
        }

        let get_node_country_code = |node_info: &NodeInfo| {
            let country_codes = node_info
                .dial_info_detail_list()
                .iter()
                .map(|did| match &did.dial_info {
                    DialInfo::UDP(di) => di.socket_address.ip_addr(),
                    DialInfo::TCP(di) => di.socket_address.ip_addr(),
                    DialInfo::WS(di) => di.socket_address.ip_addr(),
                    DialInfo::WSS(di) => di.socket_address.ip_addr(),
                })
                .map(geolocation::query_country_code)
                .collect::<Vec<_>>();

            if country_codes.is_empty() {
                return None;
            }

            // Indexing cannot panic, guarded by a check above
            let cc0 = country_codes[0];

            if !country_codes.iter().all(|cc| cc.is_some() && *cc == cc0) {
                // Lookup failed for some address or results are different
                return None;
            }

            cc0
        };

        match self {
            SignedNodeInfo::Direct(sni) => {
                GeolocationInfo::new(get_node_country_code(sni.node_info()), vec![])
            }
            SignedNodeInfo::Relayed(sni) => {
                let relay_cc = get_node_country_code(sni.relay_info().node_info());

                GeolocationInfo::new(get_node_country_code(sni.node_info()), vec![relay_cc])
            }
        }
    }

    /// Compare this SignedNodeInfo to another one
    /// Exclude the signature and timestamp and any other fields that are not
    /// semantically valuable
    pub fn equivalent(&self, other: &SignedNodeInfo) -> bool {
        match self {
            SignedNodeInfo::Direct(d) => match other {
                SignedNodeInfo::Direct(pd) => d.equivalent(pd),
                SignedNodeInfo::Relayed(_) => false,
            },
            SignedNodeInfo::Relayed(r) => match other {
                SignedNodeInfo::Direct(_) => false,
                SignedNodeInfo::Relayed(pr) => r.equivalent(pr),
            },
        }
    }
}
