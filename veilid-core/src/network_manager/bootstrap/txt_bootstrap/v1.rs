use super::*;

pub const BOOTSTRAP_TXT_VERSION_1: u8 = 1;

impl_veilid_log_facility!("net");

impl NetworkManager {
    /// Parse v1 bootstrap record strings into BootstrapRecord structs
    #[instrument(level = "trace", skip(self), ret, err)]
    pub fn parse_bootstrap_v1(
        &self,
        record_strings: &[String],
    ) -> EyreResult<Vec<BootstrapRecord>> {
        veilid_log!(self debug "Parsing v1 bootstraps: {:?}", record_strings);

        let signing_keys = self
            .config()
            .with(|c| c.network.routing_table.bootstrap_keys.clone());
        if signing_keys.is_empty() {
            veilid_log!(self warn "No signing keys in config. Proceeding with UNVERIFIED bootstrap.");
        }

        // For each record string resolve into BootstrapRecord pairs
        let dial_info_converter = BootstrapDialInfoConverter::default();

        let mut bootstrap_records: Vec<BootstrapRecord> = Vec::new();
        for record_string in record_strings {
            let Some(bootstrap_record) = BootstrapRecord::new_from_v1_str(
                self,
                &dial_info_converter,
                record_string,
                &signing_keys,
            )?
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
