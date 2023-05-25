//! Test suite for Native
#![cfg(not(target_arch = "wasm32"))]
use crate::crypto::tests::*;
use crate::network_manager::tests::*;
use crate::routing_table::tests::*;
use crate::tests::common::*;
use crate::veilid_api::tests::*;
use crate::*;

///////////////////////////////////////////////////////////////////////////

#[allow(dead_code)]
pub async fn run_all_tests() {
    // iOS and Android tests also run these.
    info!("TEST: test_host_interface");
    test_host_interface::test_all().await;
    info!("TEST: test_types");
    test_types::test_all().await;
    info!("TEST: test_veilid_core");
    test_veilid_core::test_all().await;
    info!("TEST: test_veilid_config");
    test_veilid_config::test_all().await;
    info!("TEST: test_connection_table");
    test_connection_table::test_all().await;
    info!("TEST: test_signed_node_info");
    test_signed_node_info::test_all().await;
    info!("TEST: test_table_store");
    test_table_store::test_all().await;
    info!("TEST: test_protected_store");
    test_protected_store::test_all().await;
    info!("TEST: test_crypto");
    test_crypto::test_all().await;
    info!("TEST: test_envelope_receipt");
    test_envelope_receipt::test_all().await;
    info!("TEST: veilid_api::test_serialize");
    veilid_api::tests::test_serialize_rkyv::test_all().await;
    info!("TEST: routing_table::test_serialize");
    routing_table::tests::test_serialize::test_all().await;

    info!("Finished unit tests");
}

#[allow(dead_code)]
#[cfg(feature = "rt-tokio")]
pub fn block_on<F: Future<Output = T>, T>(f: F) -> T {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(f)
}

#[cfg(feature = "rt-async-std")]
#[allow(dead_code)]
pub fn block_on<F: Future<Output = T>, T>(f: F) -> T {
    async_std::task::block_on(f)
}

///////////////////////////////////////////////////////////////////////////
cfg_if! {
    if #[cfg(test)] {
        use serial_test::serial;
        use std::sync::Once;

        static SETUP_ONCE: Once = Once::new();

        pub fn setup() {
            SETUP_ONCE.call_once(|| {
                cfg_if! {
                    if #[cfg(feature = "tracing")] {
                        use tracing_subscriber::{filter, fmt, prelude::*};
                        let mut filters = filter::Targets::new().with_default(filter::LevelFilter::TRACE);
                        for ig in DEFAULT_LOG_IGNORE_LIST {
                            filters = filters.with_target(ig, filter::LevelFilter::OFF);
                        }
                        let fmt_layer = fmt::layer();
                        tracing_subscriber::registry()
                            .with(fmt_layer)
                            .with(filters)
                            .init();
                    }
                }
            });
        }

        #[test]
        #[serial]
        fn run_test_host_interface() {
            setup();
            block_on(async {
                test_host_interface::test_all().await;
            });
        }

        #[test]
        #[serial]
        fn run_test_dht_key() {
            setup();
            block_on(async {
                test_types::test_all().await;
            });
        }

        #[test]
        #[serial]
        fn run_test_veilid_core() {
            setup();
            block_on(async {
                test_veilid_core::test_all().await;
            });
        }

        #[test]
        #[serial]
        fn run_test_veilid_config() {
            setup();
            block_on(async {
                test_veilid_config::test_all().await;
            })
        }

        #[test]
        #[serial]
        fn run_test_connection_table() {
            setup();
            block_on(async {
                test_connection_table::test_all().await;
            })
        }

        #[test]
        #[serial]
        fn run_test_signed_node_info() {
            setup();
            block_on(async {
                test_signed_node_info::test_all().await;
            })
        }

        #[test]
        #[serial]
        fn run_test_table_store() {
            setup();
            block_on(async {
                test_table_store::test_all().await;
            })
        }

        #[test]
        #[serial]
        fn run_test_protected_store() {
            setup();
            block_on(async {
                test_protected_store::test_all().await;
            })
        }

        #[test]
        #[serial]
        fn run_test_crypto() {
            setup();
            block_on(async {
                test_crypto::test_all().await;
            })
        }

        #[test]
        #[serial]
        fn run_test_envelope_receipt() {
            setup();
            block_on(async {
                test_envelope_receipt::test_all().await;
            })
        }

        #[test]
        #[serial]
        fn run_test_serialize_rkyv() {
            setup();
            block_on(async {
                veilid_api::tests::test_serialize_rkyv::test_all().await;
            })
        }

        #[test]
        #[serial]
        fn run_test_routing_table_serialize() {
            setup();
            block_on(async {
                routing_table::tests::test_serialize::test_all().await;
            })
        }
    }
}
