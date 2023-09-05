//! Test suite for the Web and headless browsers.
//! These tests only work with WASM_BINDGEN_USE_NO_MODULE=true env var,
//!     as otherwise there's no way to access the generated wasm bindings from inside JS.

#![cfg(target_arch = "wasm32")]

extern crate alloc;
extern crate wasm_bindgen_test;
use js_sys::*;
use parking_lot::Once;
use veilid_wasm::*;
use wasm_bindgen::*;
use wasm_bindgen_futures::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

static SETUP_ONCE: Once = Once::new();
pub fn setup() -> () {
    SETUP_ONCE.call_once(|| {
        console_log!("setup()");
        console_error_panic_hook::set_once();
        init_callbacks();
    })
}

fn init_callbacks() {
    assert_eq!(js_sys::eval(r#"
    window.sleep = (milliseconds) => { return new Promise(resolve => setTimeout(resolve, milliseconds)) };
    window.stateChangeCallback = async (stateChange) => { 
        delete stateChange.peers; // makes logs less verbose
        console.log("State change: ", JSON.stringify(stateChange, null, 2)); 
    };
    window.veilidCoreInitConfig = {
        logging: {
          api: {
            enabled: true,
            level: 'Info',
          },
          performance: {
            enabled: false,
            level: 'Info',
            logs_in_timings: false,
            logs_in_console: false,
          },
        },
      };
      
    window.veilidCoreStartupConfig = {
        program_name: 'veilid-wasm-test',
        namespace: '',
        capabilities: {
          disable: [],
        },
        protected_store: {
          allow_insecure_fallback: true,
          always_use_insecure_storage: true,
          directory: '',
          delete: false,
          device_encryption_key_password: 'some-user-secret-value',
          // "new_device_encryption_key_password": "an-updated-user-secret-value"
        },
        table_store: {
          directory: '',
          delete: false,
        },
        block_store: {
          directory: '',
          delete: false,
        },
        network: {
          connection_initial_timeout_ms: 2000,
          connection_inactivity_timeout_ms: 60000,
          max_connections_per_ip4: 32,
          max_connections_per_ip6_prefix: 32,
          max_connections_per_ip6_prefix_size: 56,
          max_connection_frequency_per_min: 128,
          client_whitelist_timeout_ms: 300000,
          reverse_connection_receipt_time_ms: 5000,
          hole_punch_receipt_time_ms: 5000,
          network_key_password: '',
          disable_capabilites: [],
          routing_table: {
            node_id: [],
            node_id_secret: [],
            bootstrap: [
                'ws://bootstrap.veilid.net:5150/ws',
            ],
            limit_over_attached: 64,
            limit_fully_attached: 32,
            limit_attached_strong: 16,
            limit_attached_good: 8,
            limit_attached_weak: 4,
          },
          rpc: {
            concurrency: 0,
            queue_size: 1024,
            max_timestamp_behind_ms: 10000,
            max_timestamp_ahead_ms: 10000,
            timeout_ms: 5000,
            max_route_hop_count: 4,
            default_route_hop_count: 1,
          },
          dht: {
            max_find_node_count: 20,
            resolve_node_timeout_ms: 10000,
            resolve_node_count: 1,
            resolve_node_fanout: 4,
            get_value_timeout_ms: 10000,
            get_value_count: 3,
            get_value_fanout: 4,
            set_value_timeout_ms: 10000,
            set_value_count: 5,
            set_value_fanout: 4,
            min_peer_count: 20,
            min_peer_refresh_time_ms: 60000,
            validate_dial_info_receipt_time_ms: 2000,
            local_subkey_cache_size: 128,
            local_max_subkey_cache_memory_mb: 256,
            remote_subkey_cache_size: 1024,
            remote_max_records: 65536,
            remote_max_subkey_cache_memory_mb: 256,
            remote_max_storage_space_mb: 0,
          },
          upnp: true,
          detect_address_changes: true,
          restricted_nat_retries: 0,
          tls: {
            certificate_path: '',
            private_key_path: '',
            connection_initial_timeout_ms: 2000,
          },
          application: {
            https: {
              enabled: false,
              listen_address: ':5150',
              path: 'app',
            },
            http: {
              enabled: false,
              listen_address: ':5150',
              path: 'app',
            },
          },
          protocol: {
            udp: {
              enabled: false,
              socket_pool_size: 0,
              listen_address: '',
            },
            tcp: {
              connect: false,
              listen: false,
              max_connections: 32,
              listen_address: '',
            },
            ws: {
              connect: true,
              listen: true,
              max_connections: 16,
              listen_address: ':5150',
              path: 'ws',
            },
            wss: {
              connect: true,
              listen: false,
              max_connections: 16,
              listen_address: '',
              path: 'ws',
            },
          },
        },
      };
    true
    "#).expect("failed to eval"), JsValue::TRUE);
}

/// Helper for converting an eval Promise result into a JsValue
async fn eval_promise(source: &str) -> JsValue {
    JsFuture::from(
        eval(source)
            .expect("Failed to eval")
            .dyn_into::<Promise>()
            .unwrap(),
    )
    .await
    .unwrap()
}

// ----------------------------------------------------------------

// TODO: now that veilidClient uses a single instance of VeilidAPI,
//   subsequent tests fail because veilidCore has already been initialized.
#[wasm_bindgen_test()]
async fn test_kitchen_sink() {
    setup();

    let res = eval_promise(
        r#"
            (async function () {
                const { veilidClient } = wasm_bindgen; // only accessible in no_module mode.
                veilidClient.initializeCore(window.veilidCoreInitConfig);
                await veilidClient.startupCore(window.stateChangeCallback, JSON.stringify(window.veilidCoreStartupConfig));

                console.log(veilidClient.versionString());
                await veilidClient.attach();

                await sleep(10000);
                await veilidClient.detach();
                await veilidClient.shutdownCore();

                return true;
            })();
        "#,
    ).await;

    assert_eq!(res, JsValue::TRUE);
}
