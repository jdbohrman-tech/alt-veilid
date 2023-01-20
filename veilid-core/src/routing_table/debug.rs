use super::*;
use routing_table::tasks::bootstrap::BOOTSTRAP_TXT_VERSION;

impl RoutingTable {
    pub(crate) fn debug_info_nodeinfo(&self) -> String {
        let mut out = String::new();
        let inner = self.inner.read();
        out += "Routing Table Info:\n";

        out += &format!("   Node Id: {}\n", self.unlocked_inner.node_id.encode());
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

            out += "TXT Record:\n";
            out += &format!(
                "{},{},{},{},{}",
                BOOTSTRAP_TXT_VERSION,
                MIN_CRYPTO_VERSION,
                MAX_CRYPTO_VERSION,
                self.node_id().encode(),
                some_hostname.unwrap()
            );
            for short_url in short_urls {
                out += &format!(",{}", short_url);
            }
            out += "\n";
        }
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

        out += "LocalNetwork PeerInfo:\n";
        out += &format!(
            "  {:#?}\n",
            self.get_own_peer_info(RoutingDomain::LocalNetwork)
        );

        out += "PublicInternet PeerInfo:\n";
        out += &format!(
            "  {:#?}\n",
            self.get_own_peer_info(RoutingDomain::PublicInternet)
        );

        out
    }

    pub(crate) fn debug_info_entries(&self, limit: usize, min_state: BucketEntryState) -> String {
        let inner = self.inner.read();
        let inner = &*inner;
        let cur_ts = get_aligned_timestamp();

        let mut out = String::new();

        let blen = inner.buckets.len();
        let mut b = 0;
        let mut cnt = 0;
        out += &format!("Entries: {}\n", inner.bucket_entry_count);
        while b < blen {
            let filtered_entries: Vec<(&DHTKey, &Arc<BucketEntry>)> = inner.buckets[b]
                .entries()
                .filter(|e| {
                    let state = e.1.with(inner, |_rti, e| e.state(cur_ts));
                    state >= min_state
                })
                .collect();
            if !filtered_entries.is_empty() {
                out += &format!("  Bucket #{}:\n", b);
                for e in filtered_entries {
                    let state = e.1.with(inner, |_rti, e| e.state(cur_ts));
                    out += &format!(
                        "    {} [{}]\n",
                        e.0.encode(),
                        match state {
                            BucketEntryState::Reliable => "R",
                            BucketEntryState::Unreliable => "U",
                            BucketEntryState::Dead => "D",
                        }
                    );

                    cnt += 1;
                    if cnt >= limit {
                        break;
                    }
                }
                if cnt >= limit {
                    break;
                }
            }
            b += 1;
        }

        out
    }

    pub(crate) fn debug_info_entry(&self, node_id: DHTKey) -> String {
        let mut out = String::new();
        out += &format!("Entry {:?}:\n", node_id);
        if let Some(nr) = self.lookup_node_ref(node_id) {
            out += &nr.operate(|_rt, e| format!("{:#?}\n", e));
        } else {
            out += "Entry not found\n";
        }

        out
    }

    pub(crate) fn debug_info_buckets(&self, min_state: BucketEntryState) -> String {
        let inner = self.inner.read();
        let inner = &*inner;
        let cur_ts = get_aligned_timestamp();

        let mut out = String::new();
        const COLS: usize = 16;
        let rows = inner.buckets.len() / COLS;
        let mut r = 0;
        let mut b = 0;
        out += "Buckets:\n";
        while r < rows {
            let mut c = 0;
            out += format!("  {:>3}: ", b).as_str();
            while c < COLS {
                let mut cnt = 0;
                for e in inner.buckets[b].entries() {
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

        out
    }
}
