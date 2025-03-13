//! Test suite for the Web and headless browsers.
#![cfg(all(target_arch = "wasm32", target_os = "unknown"))]

use cfg_if::*;
use parking_lot::Once;
use veilid_tools::tests::*;

use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

static SETUP_ONCE: Once = Once::new();
pub fn setup() -> () {
    SETUP_ONCE.call_once(|| {
        console_error_panic_hook::set_once();
        cfg_if! {
            if #[cfg(feature = "tracing")] {
                let config = veilid_tracing_wasm::WASMLayerConfig::new()
                    .with_report_logs_in_timings(false);
                    .with_max_level(Level::TRACE);
                    .with_console_config(veilid_tracing_wasm::ConsoleConfig::ReportWithoutConsoleColor);
                veilid_tracing_wasm::set_as_global_default_with_config(config);
            } else {
                wasm_logger::init(wasm_logger::Config::default());
            }
        }
    });
}

#[wasm_bindgen_test]
async fn run_test_host_interface() {
    setup();

    test_host_interface::test_all().await;
}

#[wasm_bindgen_test]
async fn run_test_event_bus() {
    setup();

    test_event_bus::test_all().await;
}

#[wasm_bindgen_test]
async fn run_test_async_tag_lock() {
    setup();

    test_async_tag_lock::test_all().await;
}

#[wasm_bindgen_test]
async fn run_test_startup_lock() {
    setup();

    test_startup_lock::test_all().await;
}
