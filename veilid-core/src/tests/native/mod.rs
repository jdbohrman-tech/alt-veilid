//! Test suite for Native
#![cfg(not(target_arch = "wasm32"))]
use crate::crypto::tests::*;
use crate::network_manager::tests::*;
use crate::tests::common::*;
use crate::*;

#[cfg(all(target_os = "android", feature = "android_tests"))]
use jni::{objects::JClass, objects::JObject, JNIEnv};

#[cfg(all(target_os = "android", feature = "android_tests"))]
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_com_veilid_veilidcore_veilidcore_1android_1tests_MainActivity_run_1tests(
    env: JNIEnv,
    _class: JClass,
    ctx: JObject,
) {
    crate::intf::utils::android::veilid_core_setup_android(
        env,
        ctx,
        "veilid_core",
        crate::veilid_config::VeilidConfigLogLevel::Trace,
    );
    run_all_tests();
}

#[cfg(all(target_os = "ios", feature = "ios_tests"))]
#[no_mangle]
pub extern "C" fn run_veilid_core_tests() {
    let log_path: std::path::PathBuf = [
        std::env::var("HOME").unwrap().as_str(),
        "Documents",
        "veilid-core.log",
    ]
    .iter()
    .collect();
    crate::intf::utils::ios_test_setup::veilid_core_setup(
        "veilid-core",
        Some(Level::Trace),
        Some((Level::Trace, log_path.as_path())),
    );
    run_all_tests();
}

///////////////////////////////////////////////////////////////////////////

#[allow(dead_code)]
pub fn run_all_tests() {
    info!("TEST: exec_test_host_interface");
    exec_test_host_interface();
    info!("TEST: exec_test_dht_key");
    exec_test_dht_key();
    info!("TEST: exec_test_veilid_core");
    exec_test_veilid_core();
    info!("TEST: exec_test_veilid_config");
    exec_test_veilid_config();
    info!("TEST: exec_test_connection_table");
    exec_test_connection_table();
    info!("TEST: exec_test_table_store");
    exec_test_table_store();
    info!("TEST: exec_test_protected_store");
    exec_test_protected_store();
    info!("TEST: exec_test_crypto");
    exec_test_crypto();
    info!("TEST: exec_test_envelope_receipt");
    exec_test_envelope_receipt();

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
fn exec_test_dht_key() {
    block_on(async {
        test_dht_key::test_all().await;
    });
}
fn exec_test_veilid_core() {
    block_on(async {
        test_veilid_core::test_all().await;
    });
}
fn exec_test_veilid_config() {
    block_on(async {
        test_veilid_config::test_all().await;
    })
}
fn exec_test_connection_table() {
    block_on(async {
        test_connection_table::test_all().await;
    })
}
fn exec_test_table_store() {
    block_on(async {
        test_table_store::test_all().await;
    })
}
fn exec_test_protected_store() {
    block_on(async {
        test_protected_store::test_all().await;
    })
}
fn exec_test_crypto() {
    block_on(async {
        test_crypto::test_all().await;
    })
}
fn exec_test_envelope_receipt() {
    block_on(async {
        test_envelope_receipt::test_all().await;
    })
}

///////////////////////////////////////////////////////////////////////////
cfg_if! {
    if #[cfg(test)] {
        use serial_test::serial;
        use simplelog::*;
        use std::sync::Once;

        static SETUP_ONCE: Once = Once::new();

        pub fn setup() {
            SETUP_ONCE.call_once(|| {
                let mut cb = ConfigBuilder::new();
                for ig in crate::DEFAULT_LOG_IGNORE_LIST {
                    cb.add_filter_ignore_str(ig);
                }
                TestLogger::init(LevelFilter::Trace, cb.build()).unwrap();
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
        fn run_test_dht_key() {
            setup();
            exec_test_dht_key();
        }

        #[test]
        #[serial]
        fn run_test_veilid_core() {
            setup();
            exec_test_veilid_core();
        }

        #[test]
        #[serial]
        fn run_test_veilid_config() {
            setup();
            exec_test_veilid_config();
        }

        #[test]
        #[serial]
        fn run_test_connection_table() {
            setup();
            exec_test_connection_table();
        }

        #[test]
        #[serial]
        fn run_test_table_store() {
            setup();
            exec_test_table_store();
        }

        #[test]
        #[serial]
        fn run_test_protected_store() {
            setup();
            exec_test_protected_store();
        }

        #[test]
        #[serial]
        fn run_test_crypto() {
            setup();
            exec_test_crypto();
        }

        #[test]
        #[serial]
        fn run_test_envelope_receipt() {
            setup();
            exec_test_envelope_receipt();
        }

    }
}
