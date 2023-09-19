pub mod test_crypto;
pub mod test_envelope_receipt;
pub mod test_types;

use super::*;
use crate::tests::common::test_veilid_config::*;

async fn crypto_tests_startup() -> VeilidAPI {
    trace!("crypto_tests: starting");
    let (update_callback, config_callback) = setup_veilid_core();

    api_startup(update_callback, config_callback)
        .await
        .expect("startup failed")
}

async fn crypto_tests_shutdown(api: VeilidAPI) {
    trace!("crypto_tests: shutting down");
    api.shutdown().await;
    trace!("crypto_tests: finished");
}
