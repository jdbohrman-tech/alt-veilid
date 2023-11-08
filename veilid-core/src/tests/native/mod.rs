//! Test suite for Native
#![cfg(not(target_arch = "wasm32"))]
use crate::tests::*;
use crate::*;

///////////////////////////////////////////////////////////////////////////

#[allow(dead_code)]
pub async fn run_all_tests() {
    // iOS and Android tests also run these.
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
    info!("TEST: veilid_api::tests::test_serialize_json");
    veilid_api::tests::test_serialize_json::test_all().await;
    info!("TEST: routing_table::test_serialize_routing_table");
    routing_table::tests::test_serialize_routing_table::test_all().await;
    info!("TEST: test_dht");
    test_dht::test_all().await;

    info!("Finished unit tests");
}

cfg_if::cfg_if! {
    if #[cfg(feature = "rt-async-std")] {
        #[allow(dead_code)]
        pub fn block_on<F: Future<Output = T>, T>(f: F) -> T {
            async_std::task::block_on(f)
        }
    } else if #[cfg(feature = "rt-tokio")] {
        #[allow(dead_code)]
        pub fn block_on<F: Future<Output = T>, T>(f: F) -> T {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(f)
        }
    } else {
        compile_error!("needs executor implementation")
    }
}

///////////////////////////////////////////////////////////////////////////
cfg_if! {
    if #[cfg(test)] {
        use serial_test::serial;
        use std::sync::Once;
        use paste::paste;

        macro_rules! run_test {
            // Nearly all test runner code is cookie cutter, and copy-pasting makes it too easy to make a typo.

            // Pass in a module and test module, and we'll run its `test_all`.
            ($parent_module:ident, $test_module:ident) => {
                paste! {
                    #[test]
                    #[serial]
                    fn [<run_ $parent_module _ $test_module>]() {
                        setup();
                        block_on(async {
                            $parent_module::tests::$test_module::test_all().await;
                        })
                    }
                }
            };

            // Pass in a test module name, and we'll run its `test_all`.
            ($test_module:ident) => {
                paste! {
                    #[test]
                    #[serial]
                    fn [<run_ $test_module>]() {
                        setup();
                        block_on(async {
                            $test_module::test_all().await;
                        })
                    }
                }
            };
        }

        static SETUP_ONCE: Once = Once::new();

        pub fn setup() {
            SETUP_ONCE.call_once(|| {
                use tracing_subscriber::{EnvFilter, filter::LevelFilter, fmt, prelude::*};
                let mut env_filter = EnvFilter::builder().with_default_directive(LevelFilter::INFO.into()).from_env_lossy();
                for ig in DEFAULT_LOG_IGNORE_LIST {
                    env_filter = env_filter.add_directive(format!("{}=off", ig).parse().unwrap());
                }
                let fmt_layer = fmt::layer();
                tracing_subscriber::registry()
                    .with(fmt_layer)
                    .with(env_filter)
                    .init();
            });
        }

        run_test!(test_types);

        run_test!(test_veilid_core);

        run_test!(test_veilid_config);

        run_test!(test_connection_table);

        run_test!(test_signed_node_info);

        run_test!(test_table_store);

        run_test!(test_protected_store);

        run_test!(test_crypto);

        run_test!(test_envelope_receipt);

        run_test!(veilid_api, test_serialize_json);

        run_test!(routing_table, test_serialize_routing_table);

        run_test!(test_dht);
    }
}
