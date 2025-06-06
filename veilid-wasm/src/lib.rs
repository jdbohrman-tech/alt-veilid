#![cfg(all(target_arch = "wasm32", target_os = "unknown"))]
#![no_std]
#![recursion_limit = "256"]

/// Veilid WASM Bindings for Flutter/Dart, as well as Native Javascript
/// The Flutter/Dart bindings are in this lib.rs directly
/// The Native Javascript bindings are in the other files.
extern crate alloc;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::*;
use core::cell::RefCell;
use core::fmt::Debug;
use core::sync::atomic::{AtomicBool, Ordering};
use futures_util::FutureExt;
use gloo_utils::format::JsValueSerdeExt;
use js_sys::*;
use lazy_static::*;
use send_wrapper::*;
use serde::*;
use tracing_subscriber::prelude::*;
use tracing_subscriber::*;
use tsify::*;
use veilid_core::*;
use veilid_core::{tools::*, VeilidAPIError};
use veilid_tracing_wasm::*;
use wasm_bindgen_futures::*;

pub mod veilid_client_js;
pub mod veilid_crypto_js;
pub mod veilid_routing_context_js;
pub mod veilid_table_db_js;

mod wasm_helpers;
use wasm_helpers::*;

// API Singleton
lazy_static! {
    static ref VEILID_API: SendWrapper<RefCell<Option<veilid_core::VeilidAPI>>> =
        SendWrapper::new(RefCell::new(None));
    static ref FILTERS: SendWrapper<RefCell<BTreeMap<&'static str, veilid_core::VeilidLayerFilter>>> =
        SendWrapper::new(RefCell::new(BTreeMap::new()));
    static ref ROUTING_CONTEXTS: SendWrapper<RefCell<BTreeMap<u32, veilid_core::RoutingContext>>> =
        SendWrapper::new(RefCell::new(BTreeMap::new()));
    static ref TABLE_DBS: SendWrapper<RefCell<BTreeMap<u32, veilid_core::TableDB>>> =
        SendWrapper::new(RefCell::new(BTreeMap::new()));
    static ref TABLE_DB_TRANSACTIONS: SendWrapper<RefCell<BTreeMap<u32, veilid_core::TableDBTransaction>>> =
        SendWrapper::new(RefCell::new(BTreeMap::new()));
}

fn get_veilid_api() -> Result<veilid_core::VeilidAPI, veilid_core::VeilidAPIError> {
    (*VEILID_API)
        .borrow()
        .clone()
        .ok_or(veilid_core::VeilidAPIError::NotInitialized)
}

fn take_veilid_api() -> Result<veilid_core::VeilidAPI, veilid_core::VeilidAPIError> {
    (**VEILID_API)
        .take()
        .ok_or(veilid_core::VeilidAPIError::NotInitialized)
}

// Marshalling helpers
pub fn unmarshall(b64: String) -> APIResult<Vec<u8>> {
    data_encoding::BASE64URL_NOPAD
        .decode(b64.as_bytes())
        .map_err(|e| {
            VeilidAPIError::generic(format!(
                "error decoding base64url string '{}' into bytes: {}",
                b64, e
            ))
        })
}

#[must_use]
pub fn marshall(data: &[u8]) -> String {
    data_encoding::BASE64URL_NOPAD.encode(data)
}

// JSON Helpers for WASM
pub fn to_json<T: Serialize + Debug>(val: T) -> JsValue {
    JsValue::from_str(&serialize_json(val))
}

pub fn to_jsvalue<T>(val: T) -> JsValue
where
    JsValue: From<T>,
{
    JsValue::from(val)
}

pub fn from_json<T: de::DeserializeOwned + Debug>(
    val: JsValue,
) -> Result<T, veilid_core::VeilidAPIError> {
    let s = val
        .as_string()
        .ok_or_else(|| veilid_core::VeilidAPIError::ParseError {
            message: "Value is not String".to_owned(),
            value: String::new(),
        })?;
    deserialize_json(&s)
}

// Utility types for async API results
type APIResult<T> = Result<T, veilid_core::VeilidAPIError>;
const APIRESULT_UNDEFINED: APIResult<()> = APIResult::Ok(());

pub fn wrap_api_future_json<F, T>(future: F) -> Promise
where
    F: Future<Output = APIResult<T>> + 'static,
    T: Serialize + Debug + 'static,
{
    future_to_promise(future.map(|res| res.map(|v| to_json(v)).map_err(to_json)))
}

pub fn wrap_api_future_plain<F, T>(future: F) -> Promise
where
    F: Future<Output = APIResult<T>> + 'static,
    JsValue: From<T>,
    T: 'static,
{
    future_to_promise(future.map(|res| res.map(|v| to_jsvalue(v)).map_err(to_json)))
}

pub fn wrap_api_future_void<F>(future: F) -> Promise
where
    F: Future<Output = APIResult<()>> + 'static,
{
    future_to_promise(future.map(|res| res.map(|_| JsValue::UNDEFINED).map_err(to_json)))
}

/////////////////////////////////////////
// WASM-specific

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(all(target_arch = "wasm32", target_os = "unknown"), derive(Tsify))]
pub struct VeilidWASMConfigLoggingPerformance {
    pub enabled: bool,
    pub level: veilid_core::VeilidConfigLogLevel,
    pub logs_in_timings: bool,
    pub logs_in_console: bool,
    pub ignore_log_targets: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(all(target_arch = "wasm32", target_os = "unknown"), derive(Tsify))]
pub struct VeilidWASMConfigLoggingAPI {
    pub enabled: bool,
    pub level: veilid_core::VeilidConfigLogLevel,
    pub ignore_log_targets: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(all(target_arch = "wasm32", target_os = "unknown"), derive(Tsify))]
pub struct VeilidWASMConfigLogging {
    pub performance: VeilidWASMConfigLoggingPerformance,
    pub api: VeilidWASMConfigLoggingAPI,
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(
    all(target_arch = "wasm32", target_os = "unknown"),
    derive(Tsify),
    tsify(from_wasm_abi)
)]
pub struct VeilidWASMConfig {
    pub logging: VeilidWASMConfigLogging,
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(
    all(target_arch = "wasm32", target_os = "unknown"),
    derive(Tsify),
    tsify(from_wasm_abi, into_wasm_abi)
)]
pub struct VeilidRouteBlob {
    pub route_id: veilid_core::RouteId,
    #[serde(with = "veilid_core::as_human_base64")]
    #[cfg_attr(
        all(target_arch = "wasm32", target_os = "unknown"),
        tsify(type = "string")
    )]
    pub blob: Vec<u8>,
}

// WASM Bindings

#[wasm_bindgen()]
pub fn initialize_veilid_wasm() {
    console_error_panic_hook::set_once();
}

static INITIALIZED: AtomicBool = AtomicBool::new(false);
#[wasm_bindgen()]
pub fn initialize_veilid_core(platform_config: String) {
    if INITIALIZED.swap(true, Ordering::AcqRel) {
        return;
    }
    let platform_config: VeilidWASMConfig = veilid_core::deserialize_json(&platform_config)
        .expect("failed to deserialize platform config json");

    // Set up subscriber and layers
    let subscriber = Registry::default();
    let mut layers = Vec::new();
    let mut filters = (*FILTERS).borrow_mut();

    // Performance logger
    if platform_config.logging.performance.enabled {
        let filter = veilid_core::VeilidLayerFilter::new(
            platform_config.logging.performance.level,
            &platform_config.logging.performance.ignore_log_targets,
            None,
        );
        let layer = WASMLayer::new(
            WASMLayerConfig::new()
                .with_report_logs_in_timings(platform_config.logging.performance.logs_in_timings)
                .with_console_config(if platform_config.logging.performance.logs_in_console {
                    ConsoleConfig::ReportWithConsoleColor
                } else {
                    ConsoleConfig::NoReporting
                })
                .with_field_filter(Some(Arc::new(|k| k != veilid_core::VEILID_LOG_KEY_FIELD))),
        )
        .with_filter(filter.clone());
        filters.insert("performance", filter);
        layers.push(layer.boxed());
    };

    // API logger
    if platform_config.logging.api.enabled {
        let filter = veilid_core::VeilidLayerFilter::new(
            platform_config.logging.api.level,
            &platform_config.logging.api.ignore_log_targets,
            None,
        );
        let layer = veilid_core::ApiTracingLayer::init().with_filter(filter.clone());
        filters.insert("api", filter);
        layers.push(layer.boxed());
    }

    let subscriber = subscriber.with(layers);
    subscriber
        .try_init()
        .map_err(|e| format!("failed to initialize logging: {}", e))
        .expect("failed to initalize WASM platform");
}

#[wasm_bindgen()]
pub fn change_log_level(layer: String, log_level: String) {
    let layer = if layer == "all" { "".to_owned() } else { layer };
    let Ok(log_level) = deserialize_json::<veilid_core::VeilidConfigLogLevel>(&log_level) else {
        return;
    };
    let filters = (*FILTERS).borrow();
    if layer.is_empty() {
        // Change all layers
        for f in filters.values() {
            f.set_max_level(log_level);
        }
    } else {
        // Change a specific layer
        if let Some(f) = filters.get(layer.as_str()) {
            f.set_max_level(log_level);
        }
    }
}

#[wasm_bindgen()]
pub fn change_log_ignore(layer: String, log_ignore: String) {
    let layer = if layer == "all" { "".to_owned() } else { layer };

    let filters = (*FILTERS).borrow();
    if layer.is_empty() {
        // Change all layers
        for f in filters.values() {
            f.set_ignore_list(Some(VeilidLayerFilter::apply_ignore_change(
                &f.ignore_list(),
                log_ignore.clone(),
            )));
        }
    } else {
        // Change a specific layer
        if let Some(f) = filters.get(layer.as_str()) {
            f.set_ignore_list(Some(VeilidLayerFilter::apply_ignore_change(
                &f.ignore_list(),
                log_ignore.clone(),
            )));
        }
    }
}

#[wasm_bindgen()]
pub fn startup_veilid_core(update_callback_js: Function, json_config: String) -> Promise {
    let update_callback_js = SendWrapper::new(update_callback_js);
    wrap_api_future_void(async move {
        let update_callback = Arc::new(move |update: VeilidUpdate| {
            let _ret =
                match Function::call1(&update_callback_js, &JsValue::UNDEFINED, &to_json(update)) {
                    Ok(v) => v,
                    Err(e) => {
                        console_log(&format!("calling update callback failed: {:?}", e));
                        return;
                    }
                };
        });

        if VEILID_API.borrow().is_some() {
            return Err(veilid_core::VeilidAPIError::AlreadyInitialized);
        }

        let veilid_api = veilid_core::api_startup_json(update_callback, json_config).await?;
        VEILID_API.replace(Some(veilid_api));
        APIRESULT_UNDEFINED
    })
}

#[wasm_bindgen()]
pub fn get_veilid_state() -> Promise {
    wrap_api_future_json(async move {
        let veilid_api = get_veilid_api()?;
        let core_state = veilid_api.get_state().await?;
        APIResult::Ok(core_state)
    })
}

#[wasm_bindgen()]
pub fn is_shutdown() -> APIResult<bool> {
    let veilid_api = get_veilid_api();
    if let Err(veilid_core::VeilidAPIError::NotInitialized) = veilid_api {
        return APIResult::Ok(true);
    }
    let veilid_api = veilid_api.unwrap();
    let is_shutdown = veilid_api.is_shutdown();
    APIResult::Ok(is_shutdown)
}

#[wasm_bindgen()]
pub fn attach() -> Promise {
    wrap_api_future_void(async move {
        let veilid_api = get_veilid_api()?;
        veilid_api.attach().await?;
        APIRESULT_UNDEFINED
    })
}

#[wasm_bindgen()]
pub fn detach() -> Promise {
    wrap_api_future_void(async move {
        let veilid_api = get_veilid_api()?;
        veilid_api.detach().await?;
        APIRESULT_UNDEFINED
    })
}

#[wasm_bindgen()]
pub fn shutdown_veilid_core() -> Promise {
    wrap_api_future_void(async move {
        let veilid_api = take_veilid_api()?;
        veilid_api.shutdown().await;
        APIRESULT_UNDEFINED
    })
}

fn add_routing_context(routing_context: veilid_core::RoutingContext) -> u32 {
    let mut next_id: u32 = 1;
    let mut rc = (*ROUTING_CONTEXTS).borrow_mut();
    while rc.contains_key(&next_id) {
        next_id += 1;
    }
    rc.insert(next_id, routing_context);
    next_id
}

#[wasm_bindgen()]
pub fn routing_context() -> Promise {
    wrap_api_future_plain(async move {
        let veilid_api = get_veilid_api()?;
        let routing_context = veilid_api.routing_context()?;
        let new_id = add_routing_context(routing_context);
        APIResult::Ok(new_id)
    })
}

#[wasm_bindgen()]
#[must_use]
pub fn release_routing_context(id: u32) -> i32 {
    let mut rc = (*ROUTING_CONTEXTS).borrow_mut();
    if rc.remove(&id).is_none() {
        return 0;
    }
    1
}

#[wasm_bindgen()]
#[must_use]
pub fn routing_context_with_default_safety(id: u32) -> u32 {
    let routing_context = {
        let rc = (*ROUTING_CONTEXTS).borrow();
        let Some(routing_context) = rc.get(&id) else {
            return 0;
        };
        routing_context.clone()
    };
    let Ok(routing_context) = routing_context.with_default_safety() else {
        return 0;
    };
    add_routing_context(routing_context)
}

#[wasm_bindgen()]
#[must_use]
pub fn routing_context_with_safety(id: u32, safety_selection: String) -> u32 {
    let safety_selection: veilid_core::SafetySelection =
        veilid_core::deserialize_json(&safety_selection).unwrap();

    let routing_context = {
        let rc = (*ROUTING_CONTEXTS).borrow();
        let Some(routing_context) = rc.get(&id) else {
            return 0;
        };
        routing_context.clone()
    };
    let Ok(routing_context) = routing_context.with_safety(safety_selection) else {
        return 0;
    };
    add_routing_context(routing_context)
}

#[wasm_bindgen()]
#[must_use]
pub fn routing_context_with_sequencing(id: u32, sequencing: String) -> u32 {
    let sequencing: veilid_core::Sequencing = veilid_core::deserialize_json(&sequencing).unwrap();

    let routing_context = {
        let rc = (*ROUTING_CONTEXTS).borrow();
        let Some(routing_context) = rc.get(&id) else {
            return 0;
        };
        routing_context.clone()
    };
    let routing_context = routing_context.with_sequencing(sequencing);
    add_routing_context(routing_context)
}

fn get_routing_context(id: u32, func_name: &str) -> APIResult<veilid_core::RoutingContext> {
    let rc = (*ROUTING_CONTEXTS).borrow();
    let Some(routing_context) = rc.get(&id) else {
        return APIResult::Err(veilid_core::VeilidAPIError::invalid_argument(
            func_name, "id", id,
        ));
    };
    Ok(routing_context.clone())
}

#[wasm_bindgen()]
pub fn routing_context_safety(id: u32) -> Promise {
    wrap_api_future_json(async move {
        let routing_context = get_routing_context(id, "routing_context_safety")?;

        let safety_selection = routing_context.safety();
        APIResult::Ok(safety_selection)
    })
}

#[wasm_bindgen()]
pub fn routing_context_app_call(id: u32, target_string: String, request: String) -> Promise {
    wrap_api_future_plain(async move {
        let request: Vec<u8> = data_encoding::BASE64URL_NOPAD
            .decode(request.as_bytes())
            .map_err(VeilidAPIError::generic)?;

        let routing_context = get_routing_context(id, "routing_context_app_call")?;

        let veilid_api = get_veilid_api()?;
        let target = veilid_api.parse_as_target(target_string)?;
        let answer = routing_context.app_call(target, request).await?;
        let answer = data_encoding::BASE64URL_NOPAD.encode(&answer);
        APIResult::Ok(answer)
    })
}

#[wasm_bindgen()]
pub fn routing_context_app_message(id: u32, target_string: String, message: String) -> Promise {
    wrap_api_future_void(async move {
        let message: Vec<u8> = data_encoding::BASE64URL_NOPAD
            .decode(message.as_bytes())
            .map_err(VeilidAPIError::generic)?;
        let routing_context = get_routing_context(id, "routing_context_app_message")?;

        let veilid_api = get_veilid_api()?;
        let target = veilid_api.parse_as_target(target_string)?;
        routing_context.app_message(target, message).await?;
        APIRESULT_UNDEFINED
    })
}

#[wasm_bindgen()]
pub fn routing_context_create_dht_record(
    id: u32,
    schema: String,
    owner: Option<String>,
    kind: u32,
) -> Promise {
    wrap_api_future_json(async move {
        let crypto_kind = if kind == 0 {
            None
        } else {
            Some(veilid_core::CryptoKind::from(kind))
        };
        let owner: Option<veilid_core::KeyPair> = match owner {
            Some(s) => Some(veilid_core::deserialize_json(&s).map_err(VeilidAPIError::generic)?),
            None => None,
        };
        let schema: veilid_core::DHTSchema =
            veilid_core::deserialize_json(&schema).map_err(VeilidAPIError::generic)?;

        let routing_context = get_routing_context(id, "routing_context_create_dht_record")?;

        let dht_record_descriptor = routing_context
            .create_dht_record(schema, owner, crypto_kind)
            .await?;
        APIResult::Ok(dht_record_descriptor)
    })
}

#[wasm_bindgen()]
pub fn routing_context_open_dht_record(id: u32, key: String, writer: Option<String>) -> Promise {
    wrap_api_future_json(async move {
        let key: veilid_core::TypedRecordKey =
            veilid_core::deserialize_json(&key).map_err(VeilidAPIError::generic)?;
        let writer: Option<veilid_core::KeyPair> = match writer {
            Some(s) => Some(veilid_core::deserialize_json(&s).map_err(VeilidAPIError::generic)?),
            None => None,
        };

        let routing_context = get_routing_context(id, "routing_context_open_dht_record")?;

        let dht_record_descriptor = routing_context.open_dht_record(key, writer).await?;
        APIResult::Ok(dht_record_descriptor)
    })
}

#[wasm_bindgen()]
pub fn routing_context_close_dht_record(id: u32, key: String) -> Promise {
    wrap_api_future_void(async move {
        let key: veilid_core::TypedRecordKey =
            veilid_core::deserialize_json(&key).map_err(VeilidAPIError::generic)?;

        let routing_context = get_routing_context(id, "routing_context_close_dht_record")?;

        routing_context.close_dht_record(key).await?;
        APIRESULT_UNDEFINED
    })
}

#[wasm_bindgen()]
pub fn routing_context_delete_dht_record(id: u32, key: String) -> Promise {
    wrap_api_future_void(async move {
        let key: veilid_core::TypedRecordKey =
            veilid_core::deserialize_json(&key).map_err(VeilidAPIError::generic)?;

        let routing_context = get_routing_context(id, "routing_context_delete_dht_record")?;

        routing_context.delete_dht_record(key).await?;
        APIRESULT_UNDEFINED
    })
}

#[wasm_bindgen()]
pub fn routing_context_get_dht_value(
    id: u32,
    key: String,
    subkey: u32,
    force_refresh: bool,
) -> Promise {
    wrap_api_future_json(async move {
        let key: veilid_core::TypedRecordKey =
            veilid_core::deserialize_json(&key).map_err(VeilidAPIError::generic)?;

        let routing_context = get_routing_context(id, "routing_context_get_dht_value")?;

        let res = routing_context
            .get_dht_value(key, subkey, force_refresh)
            .await?;
        APIResult::Ok(res)
    })
}

#[wasm_bindgen()]
pub fn routing_context_set_dht_value(
    id: u32,
    key: String,
    subkey: u32,
    data: String,
    writer: Option<String>,
) -> Promise {
    wrap_api_future_json(async move {
        let key: veilid_core::TypedRecordKey =
            veilid_core::deserialize_json(&key).map_err(VeilidAPIError::generic)?;
        let data: Vec<u8> = data_encoding::BASE64URL_NOPAD
            .decode(data.as_bytes())
            .map_err(VeilidAPIError::generic)?;
        let writer: Option<veilid_core::KeyPair> = match writer {
            Some(s) => veilid_core::deserialize_json(&s).map_err(VeilidAPIError::generic)?,
            None => None,
        };

        let routing_context = get_routing_context(id, "routing_context_set_dht_value")?;

        let res = routing_context
            .set_dht_value(key, subkey, data, writer)
            .await?;
        APIResult::Ok(res)
    })
}

#[wasm_bindgen()]
pub fn routing_context_watch_dht_values(
    id: u32,
    key: String,
    subkeys: String,
    expiration: String,
    count: u32,
) -> Promise {
    wrap_api_future_plain(async move {
        let key: veilid_core::TypedRecordKey =
            veilid_core::deserialize_json(&key).map_err(VeilidAPIError::generic)?;
        let subkeys: veilid_core::ValueSubkeyRangeSet =
            veilid_core::deserialize_json(&subkeys).map_err(VeilidAPIError::generic)?;
        let expiration = veilid_core::Timestamp::new(
            u64::from_str(&expiration).map_err(VeilidAPIError::generic)?,
        );

        let routing_context = get_routing_context(id, "routing_context_watch_dht_values")?;

        let res = routing_context
            .watch_dht_values(key, Some(subkeys), Some(expiration), Some(count))
            .await?;
        APIResult::Ok(res)
    })
}

#[wasm_bindgen()]
pub fn routing_context_cancel_dht_watch(id: u32, key: String, subkeys: String) -> Promise {
    wrap_api_future_plain(async move {
        let key: veilid_core::TypedRecordKey =
            veilid_core::deserialize_json(&key).map_err(VeilidAPIError::generic)?;
        let subkeys: veilid_core::ValueSubkeyRangeSet =
            veilid_core::deserialize_json(&subkeys).map_err(VeilidAPIError::generic)?;

        let routing_context = get_routing_context(id, "routing_context_cancel_dht_watch")?;

        let res = routing_context.cancel_dht_watch(key, Some(subkeys)).await?;
        APIResult::Ok(res)
    })
}

#[wasm_bindgen()]
pub fn routing_context_inspect_dht_record(
    id: u32,
    key: String,
    subkeys: String,
    scope: String,
) -> Promise {
    wrap_api_future_json(async move {
        let key: veilid_core::TypedRecordKey =
            veilid_core::deserialize_json(&key).map_err(VeilidAPIError::generic)?;
        let subkeys: veilid_core::ValueSubkeyRangeSet =
            veilid_core::deserialize_json(&subkeys).map_err(VeilidAPIError::generic)?;
        let scope: veilid_core::DHTReportScope =
            veilid_core::deserialize_json(&scope).map_err(VeilidAPIError::generic)?;

        let routing_context = get_routing_context(id, "routing_context_inspect_dht_record")?;

        let res = routing_context
            .inspect_dht_record(key, Some(subkeys), scope)
            .await?;

        APIResult::Ok(res)
    })
}

#[wasm_bindgen()]
pub fn new_private_route() -> Promise {
    wrap_api_future_json(async move {
        let veilid_api = get_veilid_api()?;

        let (route_id, blob) = veilid_api.new_private_route().await?;

        let route_blob = VeilidRouteBlob { route_id, blob };

        APIResult::Ok(route_blob)
    })
}

#[wasm_bindgen()]
pub fn new_custom_private_route(stability: String, sequencing: String) -> Promise {
    wrap_api_future_json(async move {
        let stability: veilid_core::Stability =
            veilid_core::deserialize_json(&stability).map_err(VeilidAPIError::generic)?;
        let sequencing: veilid_core::Sequencing =
            veilid_core::deserialize_json(&sequencing).map_err(VeilidAPIError::generic)?;

        let veilid_api = get_veilid_api()?;

        let (route_id, blob) = veilid_api
            .new_custom_private_route(&veilid_core::VALID_CRYPTO_KINDS, stability, sequencing)
            .await?;

        let route_blob = VeilidRouteBlob { route_id, blob };

        APIResult::Ok(route_blob)
    })
}

#[wasm_bindgen()]
pub fn import_remote_private_route(blob: String) -> Promise {
    wrap_api_future_plain(async move {
        let blob: Vec<u8> = data_encoding::BASE64URL_NOPAD
            .decode(blob.as_bytes())
            .map_err(VeilidAPIError::generic)?;
        let veilid_api = get_veilid_api()?;

        let key = veilid_api.import_remote_private_route(blob)?;

        APIResult::Ok(key.encode())
    })
}

#[wasm_bindgen()]
pub fn release_private_route(route_id: String) -> Promise {
    wrap_api_future_void(async move {
        let route_id: veilid_core::RouteId =
            veilid_core::RouteId::try_decode(&route_id).map_err(VeilidAPIError::generic)?;
        let veilid_api = get_veilid_api()?;
        veilid_api.release_private_route(route_id)?;
        APIRESULT_UNDEFINED
    })
}

#[wasm_bindgen()]
pub fn app_call_reply(call_id: String, message: String) -> Promise {
    wrap_api_future_void(async move {
        let message: Vec<u8> = data_encoding::BASE64URL_NOPAD
            .decode(message.as_bytes())
            .map_err(VeilidAPIError::generic)?;
        let call_id = match call_id.parse() {
            Ok(v) => v,
            Err(e) => {
                return APIResult::Err(veilid_core::VeilidAPIError::invalid_argument(
                    e, "call_id", call_id,
                ))
            }
        };
        let veilid_api = get_veilid_api()?;
        veilid_api.app_call_reply(call_id, message).await?;
        APIRESULT_UNDEFINED
    })
}

fn add_table_db(table_db: veilid_core::TableDB) -> u32 {
    let mut next_id: u32 = 1;
    let mut tdbs = (*TABLE_DBS).borrow_mut();
    while tdbs.contains_key(&next_id) {
        next_id += 1;
    }
    tdbs.insert(next_id, table_db);
    next_id
}

#[wasm_bindgen()]
pub fn open_table_db(name: String, column_count: u32) -> Promise {
    wrap_api_future_plain(async move {
        let veilid_api = get_veilid_api()?;
        let tstore = veilid_api.table_store()?;
        let table_db = tstore
            .open(&name, column_count)
            .await
            .map_err(veilid_core::VeilidAPIError::generic)?;
        let new_id = add_table_db(table_db);
        APIResult::Ok(new_id)
    })
}

#[wasm_bindgen()]
#[must_use]
pub fn release_table_db(id: u32) -> i32 {
    let mut tdbs = (*TABLE_DBS).borrow_mut();
    if tdbs.remove(&id).is_none() {
        return 0;
    }
    1
}

#[wasm_bindgen()]
pub fn delete_table_db(name: String) -> Promise {
    wrap_api_future_plain(async move {
        let veilid_api = get_veilid_api()?;
        let tstore = veilid_api.table_store()?;
        let deleted = tstore
            .delete(&name)
            .await
            .map_err(veilid_core::VeilidAPIError::generic)?;
        APIResult::Ok(deleted)
    })
}

#[wasm_bindgen()]
#[must_use]
pub fn table_db_get_column_count(id: u32) -> u32 {
    let table_dbs = (*TABLE_DBS).borrow();
    let Some(table_db) = table_dbs.get(&id) else {
        return 0;
    };
    let Ok(cc) = table_db.clone().get_column_count() else {
        return 0;
    };
    cc
}

fn get_table_db(id: u32, func_name: &str) -> APIResult<veilid_core::TableDB> {
    let table_dbs = (*TABLE_DBS).borrow();
    let Some(table_db) = table_dbs.get(&id) else {
        return APIResult::Err(veilid_core::VeilidAPIError::invalid_argument(
            func_name, "id", id,
        ));
    };
    Ok(table_db.clone())
}

#[wasm_bindgen()]
pub fn table_db_get_keys(id: u32, col: u32) -> Promise {
    wrap_api_future_json(async move {
        let table_db = get_table_db(id, "table_db_get_keys")?;

        let keys = table_db.clone().get_keys(col).await?;
        let out: Vec<String> = keys
            .into_iter()
            .map(|k| data_encoding::BASE64URL_NOPAD.encode(&k))
            .collect();
        APIResult::Ok(out)
    })
}

fn add_table_db_transaction(tdbt: veilid_core::TableDBTransaction) -> u32 {
    let mut next_id: u32 = 1;
    let mut tdbts = (*TABLE_DB_TRANSACTIONS).borrow_mut();
    while tdbts.contains_key(&next_id) {
        next_id += 1;
    }
    tdbts.insert(next_id, tdbt);
    next_id
}

#[wasm_bindgen()]
#[must_use]
pub fn table_db_transact(id: u32) -> u32 {
    let table_dbs = (*TABLE_DBS).borrow();
    let Some(table_db) = table_dbs.get(&id) else {
        return 0;
    };
    let tdbt = table_db.clone().transact();
    add_table_db_transaction(tdbt)
}

#[wasm_bindgen()]
#[must_use]
pub fn release_table_db_transaction(id: u32) -> i32 {
    let mut tdbts = (*TABLE_DB_TRANSACTIONS).borrow_mut();
    if tdbts.remove(&id).is_none() {
        return 0;
    }
    1
}

fn get_table_db_transaction(
    id: u32,
    func_name: &str,
) -> APIResult<veilid_core::TableDBTransaction> {
    let tdbts = (*TABLE_DB_TRANSACTIONS).borrow();
    let Some(tdbt) = tdbts.get(&id) else {
        return APIResult::Err(veilid_core::VeilidAPIError::invalid_argument(
            func_name, "id", id,
        ));
    };
    Ok(tdbt.clone())
}

#[wasm_bindgen()]
pub fn table_db_transaction_commit(id: u32) -> Promise {
    wrap_api_future_void(async move {
        let tdbt = get_table_db_transaction(id, "table_db_transaction_commit")?;

        tdbt.commit().await?;
        APIRESULT_UNDEFINED
    })
}

#[wasm_bindgen()]
pub fn table_db_transaction_rollback(id: u32) -> Promise {
    wrap_api_future_void(async move {
        let tdbt = get_table_db_transaction(id, "table_db_transaction_rollback")?;

        tdbt.rollback();
        APIRESULT_UNDEFINED
    })
}

#[wasm_bindgen()]
pub fn table_db_transaction_store(id: u32, col: u32, key: String, value: String) -> Promise {
    wrap_api_future_void(async move {
        let key: Vec<u8> = data_encoding::BASE64URL_NOPAD
            .decode(key.as_bytes())
            .map_err(VeilidAPIError::generic)?;
        let value: Vec<u8> = data_encoding::BASE64URL_NOPAD
            .decode(value.as_bytes())
            .map_err(VeilidAPIError::generic)?;

        let tdbt = get_table_db_transaction(id, "table_db_transaction_store")?;

        tdbt.store(col, &key, &value)?;
        APIRESULT_UNDEFINED
    })
}

#[wasm_bindgen()]
pub fn table_db_transaction_delete(id: u32, col: u32, key: String) -> Promise {
    wrap_api_future_void(async move {
        let key: Vec<u8> = data_encoding::BASE64URL_NOPAD
            .decode(key.as_bytes())
            .map_err(VeilidAPIError::generic)?;

        let tdbt = get_table_db_transaction(id, "table_db_transaction_delete")?;

        tdbt.delete(col, &key)?;
        APIRESULT_UNDEFINED
    })
}

#[wasm_bindgen()]
pub fn table_db_store(id: u32, col: u32, key: String, value: String) -> Promise {
    wrap_api_future_void(async move {
        let key: Vec<u8> = data_encoding::BASE64URL_NOPAD
            .decode(key.as_bytes())
            .map_err(VeilidAPIError::generic)?;
        let value: Vec<u8> = data_encoding::BASE64URL_NOPAD
            .decode(value.as_bytes())
            .map_err(VeilidAPIError::generic)?;
        let table_db = get_table_db(id, "table_db_store")?;

        table_db.store(col, &key, &value).await?;
        APIRESULT_UNDEFINED
    })
}

#[wasm_bindgen()]
pub fn table_db_load(id: u32, col: u32, key: String) -> Promise {
    wrap_api_future_plain(async move {
        let key: Vec<u8> = data_encoding::BASE64URL_NOPAD
            .decode(key.as_bytes())
            .map_err(VeilidAPIError::generic)?;
        let table_db = get_table_db(id, "table_db_load")?;

        let out = table_db.load(col, &key).await?;
        let out = out.map(|x| data_encoding::BASE64URL_NOPAD.encode(&x));
        APIResult::Ok(out)
    })
}

#[wasm_bindgen()]
pub fn table_db_delete(id: u32, col: u32, key: String) -> Promise {
    wrap_api_future_plain(async move {
        let key: Vec<u8> = data_encoding::BASE64URL_NOPAD
            .decode(key.as_bytes())
            .map_err(VeilidAPIError::generic)?;
        let table_db = get_table_db(id, "table_db_delete")?;

        let out = table_db.delete(col, &key).await?;
        let out = out.map(|x| data_encoding::BASE64URL_NOPAD.encode(&x));
        APIResult::Ok(out)
    })
}

#[wasm_bindgen()]
#[must_use]
pub fn valid_crypto_kinds() -> String {
    veilid_core::serialize_json(
        veilid_core::VALID_CRYPTO_KINDS
            .iter()
            .map(|k| (*k).into())
            .collect::<Vec<u32>>(),
    )
}

#[wasm_bindgen()]
#[must_use]
pub fn best_crypto_kind() -> u32 {
    veilid_core::best_crypto_kind().into()
}

#[wasm_bindgen()]
pub fn verify_signatures(node_ids: String, data: String, signatures: String) -> Promise {
    wrap_api_future_json(async move {
        let node_ids: Vec<veilid_core::TypedPublicKey> =
            veilid_core::deserialize_json(&node_ids).map_err(VeilidAPIError::generic)?;

        let data: Vec<u8> = data_encoding::BASE64URL_NOPAD
            .decode(data.as_bytes())
            .map_err(VeilidAPIError::generic)?;

        let typed_signatures: Vec<veilid_core::TypedSignature> =
            veilid_core::deserialize_json(&signatures).map_err(VeilidAPIError::generic)?;
        let veilid_api = get_veilid_api()?;
        let crypto = veilid_api.crypto()?;
        let out = crypto.verify_signatures(&node_ids, &data, &typed_signatures)?;
        APIResult::Ok(out)
    })
}

#[wasm_bindgen()]
pub fn generate_signatures(data: String, key_pairs: String) -> Promise {
    wrap_api_future_json(async move {
        let data: Vec<u8> = data_encoding::BASE64URL_NOPAD
            .decode(data.as_bytes())
            .map_err(VeilidAPIError::generic)?;

        let key_pairs: Vec<veilid_core::TypedKeyPair> =
            veilid_core::deserialize_json(&key_pairs).map_err(VeilidAPIError::generic)?;
        let veilid_api = get_veilid_api()?;
        let crypto = veilid_api.crypto()?;
        let out = crypto.generate_signatures(&data, &key_pairs, |k, s| {
            veilid_core::TypedSignature::new(k.kind, s)
        })?;
        APIResult::Ok(out)
    })
}

#[wasm_bindgen()]
pub fn generate_key_pair(kind: u32) -> Promise {
    let kind: veilid_core::CryptoKind = veilid_core::CryptoKind::from(kind);

    wrap_api_future_json(async move {
        let out = veilid_core::Crypto::generate_keypair(kind)?;
        APIResult::Ok(out)
    })
}

#[wasm_bindgen()]
pub fn crypto_cached_dh(kind: u32, key: String, secret: String) -> Promise {
    wrap_api_future_json(async move {
        let kind: veilid_core::CryptoKind = veilid_core::CryptoKind::from(kind);

        let key: veilid_core::PublicKey =
            veilid_core::deserialize_json(&key).map_err(VeilidAPIError::generic)?;
        let secret: veilid_core::SecretKey =
            veilid_core::deserialize_json(&secret).map_err(VeilidAPIError::generic)?;

        let veilid_api = get_veilid_api()?;
        let crypto = veilid_api.crypto()?;
        let csv = crypto.get(kind).ok_or_else(|| {
            veilid_core::VeilidAPIError::invalid_argument(
                "crypto_cached_dh",
                "kind",
                kind.to_string(),
            )
        })?;
        let out = csv.cached_dh(&key, &secret)?;
        APIResult::Ok(out)
    })
}

#[wasm_bindgen()]
pub fn crypto_compute_dh(kind: u32, key: String, secret: String) -> Promise {
    wrap_api_future_json(async move {
        let kind: veilid_core::CryptoKind = veilid_core::CryptoKind::from(kind);

        let key: veilid_core::PublicKey =
            veilid_core::deserialize_json(&key).map_err(VeilidAPIError::generic)?;
        let secret: veilid_core::SecretKey =
            veilid_core::deserialize_json(&secret).map_err(VeilidAPIError::generic)?;

        let veilid_api = get_veilid_api()?;
        let crypto = veilid_api.crypto()?;
        let csv = crypto.get(kind).ok_or_else(|| {
            veilid_core::VeilidAPIError::invalid_argument(
                "crypto_compute_dh",
                "kind",
                kind.to_string(),
            )
        })?;
        let out = csv.compute_dh(&key, &secret)?;
        APIResult::Ok(out)
    })
}

#[wasm_bindgen()]
pub fn crypto_generate_shared_secret(
    kind: u32,
    key: String,
    secret: String,
    domain: String,
) -> Promise {
    wrap_api_future_json(async move {
        let kind: veilid_core::CryptoKind = veilid_core::CryptoKind::from(kind);

        let key: veilid_core::PublicKey =
            veilid_core::deserialize_json(&key).map_err(VeilidAPIError::generic)?;
        let secret: veilid_core::SecretKey =
            veilid_core::deserialize_json(&secret).map_err(VeilidAPIError::generic)?;
        let domain: Vec<u8> = data_encoding::BASE64URL_NOPAD
            .decode(domain.as_bytes())
            .map_err(VeilidAPIError::generic)?;

        let veilid_api = get_veilid_api()?;
        let crypto = veilid_api.crypto()?;
        let csv = crypto.get(kind).ok_or_else(|| {
            veilid_core::VeilidAPIError::invalid_argument(
                "crypto_generate_shared_secret",
                "kind",
                kind.to_string(),
            )
        })?;
        let out = csv.generate_shared_secret(&key, &secret, &domain)?;
        APIResult::Ok(out)
    })
}

#[wasm_bindgen()]
pub fn crypto_random_bytes(kind: u32, len: u32) -> Promise {
    let kind: veilid_core::CryptoKind = veilid_core::CryptoKind::from(kind);

    wrap_api_future_plain(async move {
        let veilid_api = get_veilid_api()?;
        let crypto = veilid_api.crypto()?;
        let csv = crypto.get(kind).ok_or_else(|| {
            veilid_core::VeilidAPIError::invalid_argument(
                "crypto_random_bytes",
                "kind",
                kind.to_string(),
            )
        })?;
        let out = csv.random_bytes(len);
        let out = data_encoding::BASE64URL_NOPAD.encode(&out);
        APIResult::Ok(out)
    })
}

#[wasm_bindgen()]
pub fn crypto_default_salt_length(kind: u32) -> Promise {
    let kind: veilid_core::CryptoKind = veilid_core::CryptoKind::from(kind);

    wrap_api_future_plain(async move {
        let veilid_api = get_veilid_api()?;
        let crypto = veilid_api.crypto()?;
        let csv = crypto.get(kind).ok_or_else(|| {
            veilid_core::VeilidAPIError::invalid_argument(
                "crypto_default_salt_length",
                "kind",
                kind.to_string(),
            )
        })?;
        let out = csv.default_salt_length();
        APIResult::Ok(out)
    })
}

#[wasm_bindgen()]
pub fn crypto_hash_password(kind: u32, password: String, salt: String) -> Promise {
    wrap_api_future_plain(async move {
        let kind: veilid_core::CryptoKind = veilid_core::CryptoKind::from(kind);
        let password: Vec<u8> = data_encoding::BASE64URL_NOPAD
            .decode(password.as_bytes())
            .map_err(VeilidAPIError::generic)?;
        let salt: Vec<u8> = data_encoding::BASE64URL_NOPAD
            .decode(salt.as_bytes())
            .map_err(VeilidAPIError::generic)?;

        let veilid_api = get_veilid_api()?;
        let crypto = veilid_api.crypto()?;
        let csv = crypto.get(kind).ok_or_else(|| {
            veilid_core::VeilidAPIError::invalid_argument(
                "crypto_hash_password",
                "kind",
                kind.to_string(),
            )
        })?;
        let out = csv.hash_password(&password, &salt)?;
        APIResult::Ok(out)
    })
}

#[wasm_bindgen()]
pub fn crypto_verify_password(kind: u32, password: String, password_hash: String) -> Promise {
    wrap_api_future_plain(async move {
        let kind: veilid_core::CryptoKind = veilid_core::CryptoKind::from(kind);
        let password: Vec<u8> = data_encoding::BASE64URL_NOPAD
            .decode(password.as_bytes())
            .map_err(VeilidAPIError::generic)?;
        let veilid_api = get_veilid_api()?;
        let crypto = veilid_api.crypto()?;
        let csv = crypto.get(kind).ok_or_else(|| {
            veilid_core::VeilidAPIError::invalid_argument(
                "crypto_verify_password",
                "kind",
                kind.to_string(),
            )
        })?;
        let out = csv.verify_password(&password, &password_hash)?;
        APIResult::Ok(out)
    })
}

#[wasm_bindgen()]
pub fn crypto_derive_shared_secret(kind: u32, password: String, salt: String) -> Promise {
    wrap_api_future_json(async move {
        let kind: veilid_core::CryptoKind = veilid_core::CryptoKind::from(kind);
        let password: Vec<u8> = data_encoding::BASE64URL_NOPAD
            .decode(password.as_bytes())
            .map_err(VeilidAPIError::generic)?;
        let salt: Vec<u8> = data_encoding::BASE64URL_NOPAD
            .decode(salt.as_bytes())
            .map_err(VeilidAPIError::generic)?;

        let veilid_api = get_veilid_api()?;
        let crypto = veilid_api.crypto()?;
        let csv = crypto.get(kind).ok_or_else(|| {
            veilid_core::VeilidAPIError::invalid_argument(
                "crypto_derive_shared_secret",
                "kind",
                kind.to_string(),
            )
        })?;
        let out = csv.derive_shared_secret(&password, &salt)?;
        APIResult::Ok(out)
    })
}

#[wasm_bindgen()]
pub fn crypto_random_nonce(kind: u32) -> Promise {
    wrap_api_future_json(async move {
        let kind: veilid_core::CryptoKind = veilid_core::CryptoKind::from(kind);

        let veilid_api = get_veilid_api()?;
        let crypto = veilid_api.crypto()?;
        let csv = crypto.get(kind).ok_or_else(|| {
            veilid_core::VeilidAPIError::invalid_argument(
                "crypto_random_nonce",
                "kind",
                kind.to_string(),
            )
        })?;
        let out = csv.random_nonce();
        APIResult::Ok(out)
    })
}

#[wasm_bindgen()]
pub fn crypto_random_shared_secret(kind: u32) -> Promise {
    let kind: veilid_core::CryptoKind = veilid_core::CryptoKind::from(kind);

    wrap_api_future_json(async move {
        let veilid_api = get_veilid_api()?;
        let crypto = veilid_api.crypto()?;
        let csv = crypto.get(kind).ok_or_else(|| {
            veilid_core::VeilidAPIError::invalid_argument(
                "crypto_random_shared_secret",
                "kind",
                kind.to_string(),
            )
        })?;
        let out = csv.random_shared_secret();
        APIResult::Ok(out)
    })
}

#[wasm_bindgen()]
pub fn crypto_generate_key_pair(kind: u32) -> Promise {
    let kind: veilid_core::CryptoKind = veilid_core::CryptoKind::from(kind);

    wrap_api_future_json(async move {
        let veilid_api = get_veilid_api()?;
        let crypto = veilid_api.crypto()?;
        let csv = crypto.get(kind).ok_or_else(|| {
            veilid_core::VeilidAPIError::invalid_argument(
                "crypto_generate_key_pair",
                "kind",
                kind.to_string(),
            )
        })?;
        let out = csv.generate_keypair();
        APIResult::Ok(out)
    })
}

#[wasm_bindgen()]
pub fn crypto_generate_hash(kind: u32, data: String) -> Promise {
    wrap_api_future_json(async move {
        let kind: veilid_core::CryptoKind = veilid_core::CryptoKind::from(kind);

        let data: Vec<u8> = data_encoding::BASE64URL_NOPAD
            .decode(data.as_bytes())
            .map_err(VeilidAPIError::generic)?;

        let veilid_api = get_veilid_api()?;
        let crypto = veilid_api.crypto()?;
        let csv = crypto.get(kind).ok_or_else(|| {
            veilid_core::VeilidAPIError::invalid_argument(
                "crypto_generate_hash",
                "kind",
                kind.to_string(),
            )
        })?;
        let out = csv.generate_hash(&data);
        APIResult::Ok(out)
    })
}

#[wasm_bindgen()]
pub fn crypto_validate_key_pair(kind: u32, key: String, secret: String) -> Promise {
    wrap_api_future_plain(async move {
        let kind: veilid_core::CryptoKind = veilid_core::CryptoKind::from(kind);

        let key: veilid_core::PublicKey =
            veilid_core::deserialize_json(&key).map_err(VeilidAPIError::generic)?;
        let secret: veilid_core::SecretKey =
            veilid_core::deserialize_json(&secret).map_err(VeilidAPIError::generic)?;

        let veilid_api = get_veilid_api()?;
        let crypto = veilid_api.crypto()?;
        let csv = crypto.get(kind).ok_or_else(|| {
            veilid_core::VeilidAPIError::invalid_argument(
                "crypto_validate_key_pair",
                "kind",
                kind.to_string(),
            )
        })?;
        let out = csv.validate_keypair(&key, &secret);
        APIResult::Ok(out)
    })
}

#[wasm_bindgen()]
pub fn crypto_validate_hash(kind: u32, data: String, hash: String) -> Promise {
    wrap_api_future_plain(async move {
        let kind: veilid_core::CryptoKind = veilid_core::CryptoKind::from(kind);

        let data: Vec<u8> = data_encoding::BASE64URL_NOPAD
            .decode(data.as_bytes())
            .map_err(VeilidAPIError::generic)?;

        let hash: veilid_core::HashDigest =
            veilid_core::deserialize_json(&hash).map_err(VeilidAPIError::generic)?;

        let veilid_api = get_veilid_api()?;
        let crypto = veilid_api.crypto()?;
        let csv = crypto.get(kind).ok_or_else(|| {
            veilid_core::VeilidAPIError::invalid_argument(
                "crypto_validate_hash",
                "kind",
                kind.to_string(),
            )
        })?;
        let out = csv.validate_hash(&data, &hash);
        APIResult::Ok(out)
    })
}

#[wasm_bindgen()]
pub fn crypto_distance(kind: u32, key1: String, key2: String) -> Promise {
    wrap_api_future_json(async move {
        let kind: veilid_core::CryptoKind = veilid_core::CryptoKind::from(kind);

        let key1: veilid_core::HashDigest =
            veilid_core::deserialize_json(&key1).map_err(VeilidAPIError::generic)?;
        let key2: veilid_core::HashDigest =
            veilid_core::deserialize_json(&key2).map_err(VeilidAPIError::generic)?;

        let veilid_api = get_veilid_api()?;
        let crypto = veilid_api.crypto()?;
        let csv = crypto.get(kind).ok_or_else(|| {
            veilid_core::VeilidAPIError::invalid_argument(
                "crypto_distance",
                "kind",
                kind.to_string(),
            )
        })?;
        let out = csv.distance(&key1, &key2);
        APIResult::Ok(out)
    })
}

#[wasm_bindgen()]
pub fn crypto_sign(kind: u32, key: String, secret: String, data: String) -> Promise {
    wrap_api_future_json(async move {
        let kind: veilid_core::CryptoKind = veilid_core::CryptoKind::from(kind);

        let key: veilid_core::PublicKey =
            veilid_core::deserialize_json(&key).map_err(VeilidAPIError::generic)?;
        let secret: veilid_core::SecretKey =
            veilid_core::deserialize_json(&secret).map_err(VeilidAPIError::generic)?;

        let data: Vec<u8> = data_encoding::BASE64URL_NOPAD
            .decode(data.as_bytes())
            .map_err(VeilidAPIError::generic)?;

        let veilid_api = get_veilid_api()?;
        let crypto = veilid_api.crypto()?;
        let csv = crypto.get(kind).ok_or_else(|| {
            veilid_core::VeilidAPIError::invalid_argument("crypto_sign", "kind", kind.to_string())
        })?;
        let out = csv.sign(&key, &secret, &data)?;
        APIResult::Ok(out)
    })
}

#[wasm_bindgen()]
pub fn crypto_verify(kind: u32, key: String, data: String, signature: String) -> Promise {
    wrap_api_future_plain(async move {
        let kind: veilid_core::CryptoKind = veilid_core::CryptoKind::from(kind);

        let key: veilid_core::PublicKey =
            veilid_core::deserialize_json(&key).map_err(VeilidAPIError::generic)?;
        let data: Vec<u8> = data_encoding::BASE64URL_NOPAD
            .decode(data.as_bytes())
            .map_err(VeilidAPIError::generic)?;
        let signature: veilid_core::Signature =
            veilid_core::deserialize_json(&signature).map_err(VeilidAPIError::generic)?;

        let veilid_api = get_veilid_api()?;
        let crypto = veilid_api.crypto()?;
        let csv = crypto.get(kind).ok_or_else(|| {
            veilid_core::VeilidAPIError::invalid_argument("crypto_verify", "kind", kind.to_string())
        })?;
        let out = csv.verify(&key, &data, &signature)?;
        APIResult::Ok(out)
    })
}

#[wasm_bindgen()]
pub fn crypto_aead_overhead(kind: u32) -> Promise {
    wrap_api_future_plain(async move {
        let kind: veilid_core::CryptoKind = veilid_core::CryptoKind::from(kind);

        let veilid_api = get_veilid_api()?;
        let crypto = veilid_api.crypto()?;
        let csv = crypto.get(kind).ok_or_else(|| {
            veilid_core::VeilidAPIError::invalid_argument(
                "crypto_aead_overhead",
                "kind",
                kind.to_string(),
            )
        })?;
        let out = csv.aead_overhead();
        APIResult::Ok(out)
    })
}

#[wasm_bindgen()]
pub fn crypto_decrypt_aead(
    kind: u32,
    body: String,
    nonce: String,
    shared_secret: String,
    associated_data: Option<String>,
) -> Promise {
    wrap_api_future_plain(async move {
        let kind: veilid_core::CryptoKind = veilid_core::CryptoKind::from(kind);

        let body: Vec<u8> = data_encoding::BASE64URL_NOPAD
            .decode(body.as_bytes())
            .map_err(VeilidAPIError::generic)?;

        let nonce: veilid_core::Nonce =
            veilid_core::deserialize_json(&nonce).map_err(VeilidAPIError::generic)?;

        let shared_secret: veilid_core::SharedSecret =
            veilid_core::deserialize_json(&shared_secret).map_err(VeilidAPIError::generic)?;

        let associated_data: Option<Vec<u8>> = match associated_data {
            Some(ad) => Some(
                data_encoding::BASE64URL_NOPAD
                    .decode(ad.as_bytes())
                    .map_err(VeilidAPIError::generic)?,
            ),
            None => None,
        };

        let veilid_api = get_veilid_api()?;
        let crypto = veilid_api.crypto()?;
        let csv = crypto.get(kind).ok_or_else(|| {
            veilid_core::VeilidAPIError::invalid_argument(
                "crypto_decrypt_aead",
                "kind",
                kind.to_string(),
            )
        })?;
        let out = csv.decrypt_aead(
            &body,
            &nonce,
            &shared_secret,
            match &associated_data {
                Some(ad) => Some(ad.as_slice()),
                None => None,
            },
        )?;
        let out = data_encoding::BASE64URL_NOPAD.encode(&out);
        APIResult::Ok(out)
    })
}

#[wasm_bindgen()]
pub fn crypto_encrypt_aead(
    kind: u32,
    body: String,
    nonce: String,
    shared_secret: String,
    associated_data: Option<String>,
) -> Promise {
    wrap_api_future_plain(async move {
        let kind: veilid_core::CryptoKind = veilid_core::CryptoKind::from(kind);

        let body: Vec<u8> = data_encoding::BASE64URL_NOPAD
            .decode(body.as_bytes())
            .map_err(VeilidAPIError::generic)?;

        let nonce: veilid_core::Nonce =
            veilid_core::deserialize_json(&nonce).map_err(VeilidAPIError::generic)?;

        let shared_secret: veilid_core::SharedSecret =
            veilid_core::deserialize_json(&shared_secret).map_err(VeilidAPIError::generic)?;

        let associated_data: Option<Vec<u8>> = match associated_data {
            Some(ad) => Some(
                data_encoding::BASE64URL_NOPAD
                    .decode(ad.as_bytes())
                    .map_err(VeilidAPIError::generic)?,
            ),
            None => None,
        };

        let veilid_api = get_veilid_api()?;
        let crypto = veilid_api.crypto()?;
        let csv = crypto.get(kind).ok_or_else(|| {
            veilid_core::VeilidAPIError::invalid_argument(
                "crypto_encrypt_aead",
                "kind",
                kind.to_string(),
            )
        })?;
        let out = csv.encrypt_aead(
            &body,
            &nonce,
            &shared_secret,
            match &associated_data {
                Some(ad) => Some(ad.as_slice()),
                None => None,
            },
        )?;
        let out = data_encoding::BASE64URL_NOPAD.encode(&out);
        APIResult::Ok(out)
    })
}

#[wasm_bindgen()]
pub fn crypto_crypt_no_auth(
    kind: u32,
    body: String,
    nonce: String,
    shared_secret: String,
) -> Promise {
    wrap_api_future_plain(async move {
        let kind: veilid_core::CryptoKind = veilid_core::CryptoKind::from(kind);

        let mut body: Vec<u8> = data_encoding::BASE64URL_NOPAD
            .decode(body.as_bytes())
            .map_err(VeilidAPIError::generic)?;

        let nonce: veilid_core::Nonce =
            veilid_core::deserialize_json(&nonce).map_err(VeilidAPIError::generic)?;

        let shared_secret: veilid_core::SharedSecret =
            veilid_core::deserialize_json(&shared_secret).map_err(VeilidAPIError::generic)?;

        let veilid_api = get_veilid_api()?;
        let crypto = veilid_api.crypto()?;
        let csv = crypto.get(kind).ok_or_else(|| {
            veilid_core::VeilidAPIError::invalid_argument(
                "crypto_crypt_no_auth",
                "kind",
                kind.to_string(),
            )
        })?;
        csv.crypt_in_place_no_auth(&mut body, &nonce, &shared_secret);
        let out = data_encoding::BASE64URL_NOPAD.encode(&body);
        APIResult::Ok(out)
    })
}

#[wasm_bindgen()]
#[must_use]
pub fn now() -> String {
    veilid_core::Timestamp::now().as_u64().to_string()
}

#[wasm_bindgen()]
pub fn debug(command: String) -> Promise {
    wrap_api_future_plain(async move {
        let veilid_api = get_veilid_api()?;
        let out = veilid_api.debug(command).await?;
        APIResult::Ok(out)
    })
}

#[wasm_bindgen()]
#[must_use]
pub fn veilid_version_string() -> String {
    veilid_core::veilid_version_string()
}

#[derive(Serialize)]
#[cfg_attr(all(target_arch = "wasm32", target_os = "unknown"), derive(Tsify))]
#[tsify(into_wasm_abi)]
pub struct VeilidVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

#[wasm_bindgen()]
#[must_use]
pub fn veilid_version() -> JsValue {
    let (major, minor, patch) = veilid_core::veilid_version();
    let vv = VeilidVersion {
        major,
        minor,
        patch,
    };
    <JsValue as JsValueSerdeExt>::from_serde(&vv).unwrap()
}

#[wasm_bindgen()]
#[must_use]
pub fn default_veilid_config() -> String {
    veilid_core::default_veilid_config()
}
