//! Test suite for Native
#![cfg(not(target_arch = "wasm32"))]

mod test_assembly_buffer;
mod test_async_peek_stream;
mod test_network_interfaces;

use super::*;

//////////////////////////////////////////////////////////////////////////////////
// Allow access to tests from non cfg(test), as required for android and ios tests

#[allow(dead_code)]
pub async fn run_all_tests() {
    info!("TEST: exec_test_host_interface");
    test_host_interface::test_all().await;
    info!("TEST: exec_test_network_interfaces");
    test_network_interfaces::test_all().await;
    info!("TEST: exec_test_async_peek_stream");
    test_async_peek_stream::test_all().await;
    info!("TEST: exec_test_async_tag_lock");
    test_async_tag_lock::test_all().await;
    info!("TEST: exec_test_assembly_buffer");
    test_assembly_buffer::test_all().await;

    info!("Finished unit tests");
}

#[cfg(feature = "rt-tokio")]
#[allow(dead_code)]
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
                        use tracing_subscriber::{EnvFilter, fmt, prelude::*};
                        let mut env_filter = EnvFilter::builder().from_env_lossy();
                        for ig in DEFAULT_LOG_IGNORE_LIST {
                            env_filter = env_filter.add_directive(format!("{}=off", ig).parse().unwrap());
                        }
                        let fmt_layer = fmt::layer();
                        tracing_subscriber::registry()
                            .with(fmt_layer)
                            .with(env_filter)
                            .init();
                    } else {
                        use simplelog::*;
                        if let Ok(level_filter) = LevelFilter::from_str(&std::env::var("RUST_LOG").unwrap_or_default()) {
                            let mut cb = ConfigBuilder::new();
                            for ig in DEFAULT_LOG_IGNORE_LIST {
                                cb.add_filter_ignore_str(ig);
                            }
                            TestLogger::init(level_filter, cb.build()).unwrap();
                        }
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
        fn run_test_network_interfaces() {
            setup();
            block_on(async {
                test_network_interfaces::test_all().await;
            });
        }

        #[test]
        #[serial]
        fn run_test_async_peek_stream() {
            setup();
            block_on(async {
                test_async_peek_stream::test_all().await;
            });
        }

        #[test]
        #[serial]
        fn run_test_async_tag_lock() {
            setup();
            block_on(async {
                test_async_tag_lock::test_all().await;
            });
        }

        #[test]
        #[serial]
        fn run_test_assembly_buffer() {
            setup();
            block_on(async {
                test_assembly_buffer::test_all().await;
            });
        }
    }
}
