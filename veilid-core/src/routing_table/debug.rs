use super::*;
use routing_table::tasks::bootstrap::BOOTSTRAP_TXT_VERSION_0;

impl RoutingTable {
    pub(crate) async fn debug_info_txtrecord(&self) -> String {
        let mut out = String::new();

        let gdis = self.dial_info_details(RoutingDomain::PublicInternet);
        if gdis.is_empty() {
            out += "No TXT Record\n";
        } else {
            let mut short_urls = Vec::new();
            let mut some_hostname = Option::<String>::None;
            for gdi in gdis {
                let (short_url, hostname) = gdi.dial_info.to_short().await;
                if let Some(h) = &some_hostname {
                    if h != &hostname {
                        return format!(
                            "Inconsistent hostnames for dial info: {} vs {}",
                            some_hostname.unwrap(),
                            hostname
                        );
                    }
                } else {
                    some_hostname = Some(hostname);
                }

                short_urls.push(short_url);
            }
            if some_hostname.is_none() || short_urls.is_empty() {
                return "No dial info for bootstrap host".to_owned();
            }
            short_urls.sort();
            short_urls.dedup();

            let valid_envelope_versions = VALID_ENVELOPE_VERSIONS.map(|x| x.to_string()).join(",");
            let node_ids = self
                .unlocked_inner
                .node_ids()
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(",");
            out += "TXT Record:\n";
            out += &format!(
                "{}|{}|{}|{}|",
                BOOTSTRAP_TXT_VERSION_0,
                valid_envelope_versions,
                node_ids,
                some_hostname.unwrap()
            );
            out += &short_urls.join(",");
            out += "\n";
        }
        out
    }

    pub(crate) fn debug_info_nodeinfo(&self) -> String {
        let mut out = String::new();
        let inner = self.inner.read();
        out += "Routing Table Info:\n";

        out += &format!("   Node Ids: {}\n", self.unlocked_inner.node_ids());
        out += &format!(
            "   Self Latency Stats Accounting: {:#?}\n\n",
            inner.self_latency_stats_accounting
        );
        out += &format!(
            "   Self Transfer Stats Accounting: {:#?}\n\n",
            inner.self_transfer_stats_accounting
        );
        out += &format!(
            "   Self Transfer Stats: {:#?}\n\n",
            inner.self_transfer_stats
        );

        out
    }

    pub(crate) fn debug_info_dialinfo(&self) -> String {
        let ldis = self.dial_info_details(RoutingDomain::LocalNetwork);
        let gdis = self.dial_info_details(RoutingDomain::PublicInternet);
        let mut out = String::new();

        out += "Local Network Dial Info Details:\n";
        for (n, ldi) in ldis.iter().enumerate() {
            out += &format!("  {:>2}: {:?}\n", n, ldi);
        }
        out += "Public Internet Dial Info Details:\n";
        for (n, gdi) in gdis.iter().enumerate() {
            out += &format!("  {:>2}: {:?}\n", n, gdi);
        }
        out
    }

    pub(crate) fn debug_info_peerinfo(&self, routing_domain: RoutingDomain) -> String {
        let mut out = String::new();
        out += &format!(
            "{:?} PeerInfo:\n  {:#?}\n",
            routing_domain,
            self.get_own_peer_info(routing_domain)
        );
        out
    }

    pub(crate) fn debug_info_entries(&self, min_state: BucketEntryState) -> String {
        let inner = self.inner.read();
        let inner = &*inner;
        let cur_ts = get_aligned_timestamp();

        let mut out = String::new();

        out += &format!("Entries: {}\n", inner.bucket_entry_count());
        out += &format!("   Live:\n");
        for ec in inner.cached_entry_counts() {
            let routing_domain = ec.0 .0;
            let crypto_kind = ec.0 .1;
            let count = ec.1;
            out += &format!("  {:?}:{}: {}\n", routing_domain, crypto_kind, count);
        }
        for ck in &VALID_CRYPTO_KINDS {
            let mut b = 0;
            let blen = inner.buckets[ck].len();
            while b < blen {
                let filtered_entries: Vec<(&PublicKey, &Arc<BucketEntry>)> = inner.buckets[ck][b]
                    .entries()
                    .filter(|e| {
                        let state = e.1.with(inner, |_rti, e| e.state(cur_ts));
                        state >= min_state
                    })
                    .collect();
                if !filtered_entries.is_empty() {
                    out += &format!("{} Bucket #{}:\n", ck, b);
                    for e in filtered_entries {
                        let state = e.1.with(inner, |_rti, e| e.state(cur_ts));
                        out += &format!(
                            "    {} [{}] {}\n",
                            e.0.encode(),
                            match state {
                                BucketEntryState::Reliable => "R",
                                BucketEntryState::Unreliable => "U",
                                BucketEntryState::Dead => "D",
                            },
                            e.1.with(inner, |_rti, e| {
                                e.peer_stats()
                                    .latency
                                    .as_ref()
                                    .map(|l| {
                                        format!(
                                            "{:.2}ms",
                                            timestamp_to_secs(l.average.as_u64()) * 1000.0
                                        )
                                    })
                                    .unwrap_or_else(|| "???.??ms".to_string())
                            })
                        );
                    }
                }
                b += 1;
            }
        }

        out
    }

    pub(crate) fn debug_info_entry(&self, node_ref: NodeRef) -> String {
        let mut out = String::new();
        out += &node_ref.operate(|_rt, e| format!("{:#?}\n", e));
        out
    }

    pub(crate) fn debug_info_buckets(&self, min_state: BucketEntryState) -> String {
        let inner = self.inner.read();
        let inner = &*inner;
        let cur_ts = get_aligned_timestamp();

        let mut out = String::new();
        const COLS: usize = 16;
        out += "Buckets:\n";
        for ck in &VALID_CRYPTO_KINDS {
            out += &format!("  {}:\n", ck);
            let rows = inner.buckets[ck].len() / COLS;
            let mut r = 0;
            let mut b = 0;
            while r < rows {
                let mut c = 0;
                out += format!("    {:>3}: ", b).as_str();
                while c < COLS {
                    let mut cnt = 0;
                    for e in inner.buckets[ck][b].entries() {
                        if e.1.with(inner, |_rti, e| e.state(cur_ts) >= min_state) {
                            cnt += 1;
                        }
                    }
                    out += format!("{:>3} ", cnt).as_str();
                    b += 1;
                    c += 1;
                }
                out += "\n";
                r += 1;
            }
        }

        out
    }
}
