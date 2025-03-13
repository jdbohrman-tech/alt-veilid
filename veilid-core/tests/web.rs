//! Test suite for the Web and headless browsers.
#![cfg(all(target_arch = "wasm32", target_os = "unknown"))]
#![recursion_limit = "256"]

use parking_lot::Once;
use serial_test::serial;
use tracing::*;
use veilid_core::tests::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

static SETUP_ONCE: Once = Once::new();
pub fn setup() -> () {
    SETUP_ONCE.call_once(|| {
        console_error_panic_hook::set_once();

        let config = veilid_tracing_wasm::WASMLayerConfig::new()
            .with_report_logs_in_timings(false)
            .with_max_level(Level::TRACE)
            .with_console_config(veilid_tracing_wasm::ConsoleConfig::ReportWithoutConsoleColor);
        veilid_tracing_wasm::set_as_global_default_with_config(config);
    });
}

#[wasm_bindgen_test]
#[serial]
async fn wasm_test_types() {
    setup();
    test_types::test_all().await;
}

#[wasm_bindgen_test]
#[serial]
async fn wasm_test_veilid_core() {
    setup();
    test_veilid_core::test_all().await;
}

#[wasm_bindgen_test]
#[serial]
async fn wasm_test_veilid_config() {
    setup();
    test_veilid_config::test_all().await;
}

#[wasm_bindgen_test]
#[serial]
async fn wasm_test_connection_table() {
    setup();
    test_connection_table::test_all().await;
}

#[wasm_bindgen_test]
#[serial]
async fn wasm_test_signed_node_info() {
    setup();
    test_signed_node_info::test_all().await;
}

#[wasm_bindgen_test]
#[serial]
async fn wasm_test_table_store() {
    setup();
    test_table_store::test_all().await;
}

#[wasm_bindgen_test]
#[serial]
async fn wasm_test_protected_store() {
    setup();
    test_protected_store::test_all().await;
}

#[wasm_bindgen_test]
#[serial]
async fn wasm_test_crypto() {
    setup();
    test_crypto::test_all().await;
}

#[wasm_bindgen_test]
#[serial]
async fn wasm_test_envelope_receipt() {
    setup();
    test_envelope_receipt::test_all().await;
}

#[wasm_bindgen_test]
#[serial]
async fn wasm_test_serialize_json() {
    setup();
    test_serialize_json::test_all().await;
}

#[wasm_bindgen_test]
#[serial]
async fn wasm_test_serialize_routing_table() {
    setup();
    test_serialize_routing_table::test_all().await;
}
/* Should be an integration test not a unit test
#[wasm_bindgen_test]
#[serial]
async fn wasm_test_dht() {
    setup();
    test_dht::test_all().await;
}
*/
