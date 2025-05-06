use super::*;
use futures_util::StreamExt as _;

pub const BOOTSTRAP_TXT_VERSION_0: u8 = 0;

impl_veilid_log_facility!("net");

impl NetworkManager {
    /// Bootstrap resolution from TXT into strings
    #[instrument(level = "trace", skip(self), ret, err)]
    pub async fn resolve_bootstrap_v0(
        &self,
        hostname: String,
        txt_strings: Vec<String>,
    ) -> EyreResult<Vec<String>> {
        veilid_log!(self debug "Resolving v0 bootstraps: {:?}", txt_strings);

        // Resolve from bootstrap root to bootstrap hostnames
        let mut bsnames = Vec::<String>::new();

        for txt_string in txt_strings {
            // Split the bootstrap name record by commas
            for rec in txt_string.split(',') {
                let rec = rec.trim();
                // If the name specified is fully qualified, go with it
                let bsname = if rec.ends_with('.') {
                    rec.to_string()
                }
                // If the name is not fully qualified, prepend it to the bootstrap name
                else {
                    format!("{}.{}", rec, hostname)
                };

                // Add to the list of bootstrap name to look up
                bsnames.push(bsname);
            }
        }

        // Get bootstrap nodes from hostnames concurrently
        let mut unord = FuturesUnordered::new();
        for bsname in bsnames {
            unord.push(
                async move {
                    // look up bootstrap node txt records
                    let bsnirecords = match intf::txt_lookup(&bsname).await {
                        Err(e) => {
                            veilid_log!(self warn
                                "Network may be down. Bootstrap node txt lookup failed for {}: {}",
                                bsname, e
                            );
                            return None;
                        }
                        Ok(v) => v,
                    };
                    veilid_log!(self debug " TXT: {:?} => {:?}", bsname, bsnirecords);

                    Some(bsnirecords)
                }
                .instrument(Span::current()),
            );
        }

        let mut all_records: HashSet<String> = HashSet::new();
        while let Some(bootstrap_records) = unord.next().await {
            let Some(bootstrap_records) = bootstrap_records else {
                continue;
            };
            for br in bootstrap_records {
                all_records.insert(br);
            }
        }

        let mut all_records_sorted = all_records.into_iter().collect::<Vec<_>>();
        all_records_sorted.sort();

        Ok(all_records_sorted)
    }

    /// Parse v0 bootstrap record strings into BootstrapRecord structs
    #[instrument(level = "trace", skip(self), ret, err)]
    pub fn parse_bootstrap_v0(
        &self,
        record_strings: &[String],
    ) -> EyreResult<Vec<BootstrapRecord>> {
        veilid_log!(self debug "Parsing v0 bootstraps: {:?}", record_strings);

        // For each record string resolve into BootstrapRecord pairs
        let dial_info_converter = BootstrapDialInfoConverter::default();

        let mut bootstrap_records: Vec<BootstrapRecord> = Vec::new();
        for record_string in record_strings {
            let Some(bootstrap_record) =
                BootstrapRecord::new_from_v0_str(self, &dial_info_converter, record_string)?
            else {
                continue;
            };

            bootstrap_records.push(bootstrap_record);
        }

        let mut merged_bootstrap_records: Vec<BootstrapRecord> = Vec::new();

        for mut bsrec in bootstrap_records {
            let mut mbi = 0;
            while mbi < merged_bootstrap_records.len() {
                let mbr = &mut merged_bootstrap_records[mbi];
                if mbr.node_ids().contains_any(bsrec.node_ids()) {
                    // Merge record, pop this one out
                    let mbr = merged_bootstrap_records.remove(mbi);
                    bsrec.merge(mbr);
                } else {
                    // No overlap, go to next record
                    mbi += 1;
                }
            }
            // Append merged record
            merged_bootstrap_records.push(bsrec);
        }

        Ok(merged_bootstrap_records)
    }
}
