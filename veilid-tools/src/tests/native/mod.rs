//! Test suite for Native
#![cfg(not(target_arch = "wasm32"))]

mod test_async_peek_stream;

use super::*;

#[cfg(all(target_os = "android", feature = "android_tests"))]
use jni::{objects::JClass, objects::JObject, JNIEnv};

#[cfg(all(target_os = "android", feature = "android_tests"))]
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_com_veilid_veilidtools_veilidtools_1android_1tests_MainActivity_run_1tests(
    env: JNIEnv,
    _class: JClass,
    ctx: JObject,
) {
    crate::tests::android::veilid_tools_setup(env, ctx, "veilid-tools");
    run_all_tests();
}

#[cfg(all(target_os = "ios", feature = "ios_tests"))]
#[no_mangle]
#[allow(dead_code)]
pub extern "C" fn run_veilid_tools_tests() {
    crate::tests::ios::veilid_tools_setup().expect("setup failed");
    run_all_tests();
}

///////////////////////////////////////////////////////////////////////////

#[allow(dead_code)]
pub fn run_all_tests() {
    info!("TEST: exec_test_host_interface");
    exec_test_host_interface();
    info!("TEST: exec_test_async_peek_stream");
    exec_test_async_peek_stream();
    info!("TEST: exec_test_async_tag_lock");
    exec_test_async_tag_lock();

    info!("Finished unit tests");
}

#[cfg(feature = "rt-tokio")]
fn block_on<F: Future<Output = T>, T>(f: F) -> T {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let local = tokio::task::LocalSet::new();
    local.block_on(&rt, f)
}
#[cfg(feature = "rt-async-std")]
fn block_on<F: Future<Output = T>, T>(f: F) -> T {
    async_std::task::block_on(f)
}

fn exec_test_host_interface() {
    block_on(async {
        test_host_interface::test_all().await;
    });
}
fn exec_test_async_peek_stream() {
    block_on(async {
        test_async_peek_stream::test_all().await;
    })
}
fn exec_test_async_tag_lock() {
    block_on(async {
        test_async_tag_lock::test_all().await;
    })
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
                        let mut filters = filter::Targets::new();
                        for ig in DEFAULT_LOG_IGNORE_LIST {
                            filters = filters.with_target(ig, filter::LevelFilter::OFF);
                        }
                        let fmt_layer = fmt::layer();
                        tracing_subscriber::registry()
                            .with(filters)
                            .with(filter::LevelFilter::TRACE)
                            .with(fmt_layer)
                            .init();
                    } else {
                        use simplelog::*;
                        let mut cb = ConfigBuilder::new();
                        for ig in DEFAULT_LOG_IGNORE_LIST {
                            cb.add_filter_ignore_str(ig);
                        }
                        TestLogger::init(LevelFilter::Trace, cb.build()).unwrap();
                    }
                }

            });
        }

        #[test]
        #[serial]
        fn run_test_host_interface() {
            setup();
            exec_test_host_interface();
        }

        #[test]
        #[serial]
        fn run_test_async_peek_stream() {
            setup();
            exec_test_async_peek_stream();
        }

        #[test]
        #[serial]
        fn run_test_async_tag_lock() {
            setup();
            exec_test_async_tag_lock();
        }
    }
}
