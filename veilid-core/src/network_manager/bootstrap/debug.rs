use super::*;

impl_veilid_log_facility!("net");

impl NetworkManager {
    pub async fn debug_info_txtrecord(&self, signing_key_pair: TypedKeyPair) -> String {
        let routing_table = self.routing_table();

        let dial_info_details = routing_table.dial_info_details(RoutingDomain::PublicInternet);
        if dial_info_details.is_empty() {
            return "No PublicInternet DialInfo for TXT Record".to_owned();
        }
        let envelope_support = VALID_ENVELOPE_VERSIONS.to_vec();
        let node_ids = routing_table.node_ids();

        let mut out = "Bootstrap TXT Records:\n".to_owned();

        let bsrec = BootstrapRecord::new(
            node_ids,
            envelope_support,
            dial_info_details,
            Some(Timestamp::now().as_u64() / 1_000_000u64),
            vec![],
        );

        let dial_info_converter = BootstrapDialInfoConverter::default();

        match bsrec.to_v0_string(&dial_info_converter).await {
            Ok(v) => {
                //
                out += &format!("V0:\n{}\n", v);
            }
            Err(e) => {
                //
                out += &format!("V0 error: {}\n", e);
            }
        }

        match bsrec
            .to_v1_string(self, &dial_info_converter, signing_key_pair)
            .await
        {
            Ok(v) => {
                //
                out += &format!("V1:\n{}\n", v);
            }
            Err(e) => {
                //
                out += &format!("V1 error: {}\n", e);
            }
        }

        out
    }
}
