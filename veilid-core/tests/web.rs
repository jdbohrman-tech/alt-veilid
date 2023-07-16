//! Test suite for the Web and headless browsers.
#![cfg(target_arch = "wasm32")]
#![recursion_limit = "256"]

use cfg_if::*;
use parking_lot::Once;
use veilid_core::tests::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

extern crate wee_alloc;
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

static SETUP_ONCE: Once = Once::new();
pub fn setup() -> () {
    SETUP_ONCE.call_once(|| {
        console_error_panic_hook::set_once();
        cfg_if! {
            if #[cfg(feature = "tracing")] {
                let mut builder = tracing_wasm::WASMLayerConfigBuilder::new();
                builder.set_report_logs_in_timings(false);
                builder.set_max_level(Level::TRACE);
                builder.set_console_config(tracing_wasm::ConsoleConfig::ReportWithConsoleColor);
                tracing_wasm::set_as_global_default_with_config(builder.build());
            } else {
                wasm_logger::init(wasm_logger::Config::default());
            }
        }
    });
}

#[wasm_bindgen_test]
async fn wasm_test_host_interface() {
    setup();
    test_host_interface::test_all().await;
}

#[wasm_bindgen_test]
async fn wasm_test_types() {
    setup();
    test_types::test_all().await;
}

#[wasm_bindgen_test]
async fn wasm_test_veilid_core() {
    setup();
    test_veilid_core::test_all().await;
}

#[wasm_bindgen_test]
async fn wasm_test_veilid_config() {
    setup();
    test_veilid_config::test_all().await;
}

#[wasm_bindgen_test]
async fn wasm_test_connection_table() {
    setup();
    test_connection_table::test_all().await;
}

#[wasm_bindgen_test]
async fn wasm_test_signed_node_info() {
    setup();
    test_signed_node_info::test_all().await;
}

#[wasm_bindgen_test]
async fn wasm_test_table_store() {
    setup();
    test_table_store::test_all().await;
}

#[wasm_bindgen_test]
async fn wasm_test_protected_store() {
    setup();
    test_protected_store::test_all().await;
}

#[wasm_bindgen_test]
async fn wasm_test_crypto() {
    setup();
    test_crypto::test_all().await;
}

#[wasm_bindgen_test]
async fn wasm_test_envelope_receipt() {
    setup();
    test_envelope_receipt::test_all().await;
}

#[wasm_bindgen_test]
async fn wasm_test_serialize_json() {
    setup();
    test_serialize_json::test_all().await;
}

#[wasm_bindgen_test]
async fn wasm_test_serialize_routing_table() {
    setup();
    test_serialize_routing_table::test_all().await;
}
