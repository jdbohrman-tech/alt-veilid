// wasm-bindgen and clippy don't play well together yet
#![allow(clippy::all)]
#![cfg(target_arch = "wasm32")]
#![no_std]

extern crate alloc;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::*;
use core::cell::RefCell;
use core::fmt::Debug;
use futures_util::FutureExt;
use gloo_utils::format::JsValueSerdeExt;
use js_sys::*;
use lazy_static::*;
use send_wrapper::*;
use serde::*;
use tracing_subscriber::prelude::*;
use tracing_subscriber::*;
use tracing_wasm::{WASMLayerConfigBuilder, *};
use veilid_core::tools::*;
use veilid_core::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::*;

// Allocator
extern crate wee_alloc;
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

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

// Parse target
fn parse_target(s: String) -> APIResult<veilid_core::Target> {
    // Is this a route id?
    if let Ok(rrid) = veilid_core::RouteId::from_str(&s) {
        let veilid_api = get_veilid_api()?;
        let routing_table = veilid_api.routing_table()?;
        let rss = routing_table.route_spec_store();

        // Is this a valid remote route id? (can't target allocated routes)
        if rss.is_route_id_remote(&rrid) {
            return Ok(veilid_core::Target::PrivateRoute(rrid));
        }
    }

    // Is this a node id?
    if let Ok(nid) = veilid_core::PublicKey::from_str(&s) {
        return Ok(veilid_core::Target::NodeId(nid));
    }

    Err(veilid_core::VeilidAPIError::invalid_target())
}

// Utility types for async API results
type APIResult<T> = Result<T, veilid_core::VeilidAPIError>;
const APIRESULT_UNDEFINED: APIResult<()> = APIResult::Ok(());

pub fn wrap_api_future_json<F, T>(future: F) -> Promise
where
    F: Future<Output = APIResult<T>> + 'static,
    T: Serialize + Debug + 'static,
{
    future_to_promise(future.map(|res| res.map(|v| to_json(v)).map_err(|e| to_json(e))))
}

pub fn wrap_api_future_plain<F, T>(future: F) -> Promise
where
    F: Future<Output = APIResult<T>> + 'static,
    JsValue: From<T>,
    T: 'static,
{
    future_to_promise(future.map(|res| res.map(|v| to_jsvalue(v)).map_err(|e| to_json(e))))
}

pub fn wrap_api_future_void<F>(future: F) -> Promise
where
    F: Future<Output = APIResult<()>> + 'static,
{
    future_to_promise(future.map(|res| res.map(|_| JsValue::UNDEFINED).map_err(|e| to_json(e))))
}

/////////////////////////////////////////
// WASM-specific

#[derive(Debug, Deserialize, Serialize)]
pub struct VeilidWASMConfigLoggingPerformance {
    pub enabled: bool,
    pub level: veilid_core::VeilidConfigLogLevel,
    pub logs_in_timings: bool,
    pub logs_in_console: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VeilidWASMConfigLoggingAPI {
    pub enabled: bool,
    pub level: veilid_core::VeilidConfigLogLevel,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VeilidWASMConfigLogging {
    pub performance: VeilidWASMConfigLoggingPerformance,
    pub api: VeilidWASMConfigLoggingAPI,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VeilidWASMConfig {
    pub logging: VeilidWASMConfigLogging,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VeilidRouteBlob {
    pub route_id: veilid_core::RouteId,
    #[serde(with = "veilid_core::json_as_base64")]
    pub blob: Vec<u8>,
}

// WASM Bindings

#[wasm_bindgen()]
pub fn initialize_veilid_wasm() {
    console_error_panic_hook::set_once();
}

static SETUP_ONCE: Once = Once::new();
#[wasm_bindgen()]
pub fn initialize_veilid_core(platform_config: String) {
    SETUP_ONCE.call_once(|| {
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
                None,
            );
            let layer = WASMLayer::new(
                WASMLayerConfigBuilder::new()
                    .set_report_logs_in_timings(platform_config.logging.performance.logs_in_timings)
                    .set_console_config(if platform_config.logging.performance.logs_in_console {
                        ConsoleConfig::ReportWithConsoleColor
                    } else {
                        ConsoleConfig::NoReporting
                    })
                    .build(),
            )
            .with_filter(filter.clone());
            filters.insert("performance", filter);
            layers.push(layer.boxed());
        };

        // API logger
        if platform_config.logging.api.enabled {
            let filter =
                veilid_core::VeilidLayerFilter::new(platform_config.logging.api.level, None);
            let layer = veilid_core::ApiTracingLayer::get().with_filter(filter.clone());
            filters.insert("api", filter);
            layers.push(layer.boxed());
        }

        let subscriber = subscriber.with(layers);
        subscriber
            .try_init()
            .map_err(|e| format!("failed to initialize logging: {}", e))
            .expect("failed to initalize WASM platform");
    });
}

#[wasm_bindgen()]
pub fn change_log_level(layer: String, log_level: String) {
    let layer = if layer == "all" { "".to_owned() } else { layer };
    let log_level: veilid_core::VeilidConfigLogLevel = deserialize_json(&log_level).unwrap();
    let filters = (*FILTERS).borrow();
    if layer.is_empty() {
        // Change all layers
        for f in filters.values() {
            f.set_max_level(log_level);
        }
    } else {
        // Change a specific layer
        let f = filters.get(layer.as_str()).unwrap();
        f.set_max_level(log_level);
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
        let routing_context = veilid_api.routing_context();
        let new_id = add_routing_context(routing_context);
        APIResult::Ok(new_id)
    })
}

#[wasm_bindgen()]
pub fn release_routing_context(id: u32) -> i32 {
    let mut rc = (*ROUTING_CONTEXTS).borrow_mut();
    if rc.remove(&id).is_none() {
        return 0;
    }
    return 1;
}

#[wasm_bindgen()]
pub fn routing_context_with_privacy(id: u32) -> u32 {
    let rc = (*ROUTING_CONTEXTS).borrow();
    let Some(routing_context) = rc.get(&id) else {
        return 0;
    };
    let Ok(routing_context) = routing_context.clone().with_privacy() else {
        return 0;
    };
    let new_id = add_routing_context(routing_context);
    new_id
}

#[wasm_bindgen()]
pub fn routing_context_with_custom_privacy(id: u32, stability: String) -> u32 {
    let stability: veilid_core::Stability = veilid_core::deserialize_json(&stability).unwrap();

    let rc = (*ROUTING_CONTEXTS).borrow();
    let Some(routing_context) = rc.get(&id) else {
        return 0;
    };
    let Ok(routing_context) = routing_context.clone().with_custom_privacy(stability) else {
        return 0;
    };
    let new_id = add_routing_context(routing_context);
    new_id
}

#[wasm_bindgen()]
pub fn routing_context_with_sequencing(id: u32, sequencing: String) -> u32 {
    let sequencing: veilid_core::Sequencing = veilid_core::deserialize_json(&sequencing).unwrap();

    let rc = (*ROUTING_CONTEXTS).borrow();
    let Some(routing_context) = rc.get(&id) else {
        return 0;
    };
    let routing_context = routing_context.clone().with_sequencing(sequencing);
    let new_id = add_routing_context(routing_context);
    new_id
}

#[wasm_bindgen()]
pub fn routing_context_app_call(id: u32, target: String, request: String) -> Promise {
    let request: Vec<u8> = data_encoding::BASE64URL_NOPAD
        .decode(request.as_bytes())
        .unwrap();
    wrap_api_future_plain(async move {
        let veilid_api = get_veilid_api()?;
        let routing_table = veilid_api.routing_table()?;

        let routing_context = {
            let rc = (*ROUTING_CONTEXTS).borrow();
            let Some(routing_context) = rc.get(&id) else {
                return APIResult::Err(veilid_core::VeilidAPIError::invalid_argument("routing_context_app_call", "id", id));
            };
            routing_context.clone()
        };

        let target = parse_target(target)?;
        let answer = routing_context.app_call(target, request).await?;
        let answer = data_encoding::BASE64URL_NOPAD.encode(&answer);
        APIResult::Ok(answer)
    })
}

#[wasm_bindgen()]
pub fn routing_context_app_message(id: u32, target: String, message: String) -> Promise {
    let message: Vec<u8> = data_encoding::BASE64URL_NOPAD
        .decode(message.as_bytes())
        .unwrap();
    wrap_api_future_void(async move {
        let veilid_api = get_veilid_api()?;
        let routing_table = veilid_api.routing_table()?;

        let routing_context = {
            let rc = (*ROUTING_CONTEXTS).borrow();
            let Some(routing_context) = rc.get(&id) else {
                return APIResult::Err(veilid_core::VeilidAPIError::invalid_argument("routing_context_app_call", "id", id));
            };
            routing_context.clone()
        };

        let target = parse_target(target)?;
        routing_context.app_message(target, message).await?;
        APIRESULT_UNDEFINED
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
    let stability: veilid_core::Stability = veilid_core::deserialize_json(&stability).unwrap();
    let sequencing: veilid_core::Sequencing = veilid_core::deserialize_json(&sequencing).unwrap();

    wrap_api_future_json(async move {
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
    let blob: Vec<u8> = data_encoding::BASE64URL_NOPAD
        .decode(blob.as_bytes())
        .unwrap();
    wrap_api_future_plain(async move {
        let veilid_api = get_veilid_api()?;

        let key = veilid_api.import_remote_private_route(blob)?;

        APIResult::Ok(key.encode())
    })
}

#[wasm_bindgen()]
pub fn release_private_route(route_id: String) -> Promise {
    let route_id: veilid_core::RouteId = veilid_core::deserialize_json(&route_id).unwrap();
    wrap_api_future_void(async move {
        let veilid_api = get_veilid_api()?;
        veilid_api.release_private_route(route_id)?;
        APIRESULT_UNDEFINED
    })
}

#[wasm_bindgen()]
pub fn app_call_reply(id: String, message: String) -> Promise {
    let message: Vec<u8> = data_encoding::BASE64URL_NOPAD
        .decode(message.as_bytes())
        .unwrap();
    wrap_api_future_void(async move {
        let id = match id.parse() {
            Ok(v) => v,
            Err(e) => {
                return APIResult::Err(veilid_core::VeilidAPIError::invalid_argument(e, "id", id))
            }
        };
        let veilid_api = get_veilid_api()?;
        veilid_api.app_call_reply(id, message).await?;
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
pub fn release_table_db(id: u32) -> i32 {
    let mut tdbs = (*TABLE_DBS).borrow_mut();
    if tdbs.remove(&id).is_none() {
        return 0;
    }
    return 1;
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
pub fn table_db_get_column_count(id: u32) -> u32 {
    let table_dbs = (*TABLE_DBS).borrow();
    let Some(table_db) = table_dbs.get(&id) else {
        return 0;
    };
    let Ok(cc) = table_db.clone().get_column_count() else {
        return 0;
    };
    return cc;
}

#[wasm_bindgen()]
pub fn table_db_get_keys(id: u32, col: u32) -> Option<String> {
    let table_dbs = (*TABLE_DBS).borrow();
    let Some(table_db) = table_dbs.get(&id) else {
        return None;
    };
    let Ok(keys) = table_db.clone().get_keys(col) else {
        return None;
    };
    let keys: Vec<String> = keys
        .into_iter()
        .map(|k| data_encoding::BASE64URL_NOPAD.encode(&k))
        .collect();
    let out = veilid_core::serialize_json(keys);
    Some(out)
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
pub fn table_db_transact(id: u32) -> u32 {
    let table_dbs = (*TABLE_DBS).borrow();
    let Some(table_db) = table_dbs.get(&id) else {
        return 0;
    };
    let tdbt = table_db.clone().transact();
    let tdbtid = add_table_db_transaction(tdbt);
    return tdbtid;
}

#[wasm_bindgen()]
pub fn release_table_db_transaction(id: u32) -> i32 {
    let mut tdbts = (*TABLE_DB_TRANSACTIONS).borrow_mut();
    if tdbts.remove(&id).is_none() {
        return 0;
    }
    return 1;
}

#[wasm_bindgen()]
pub fn table_db_transaction_commit(id: u32) -> Promise {
    wrap_api_future_void(async move {
        let tdbt = {
            let tdbts = (*TABLE_DB_TRANSACTIONS).borrow();
            let Some(tdbt) = tdbts.get(&id) else {
                return APIResult::Err(veilid_core::VeilidAPIError::invalid_argument("table_db_transaction_commit", "id", id));
            };
            tdbt.clone()
        };

        tdbt.commit()
            .await
            .map_err(veilid_core::VeilidAPIError::generic)?;
        APIRESULT_UNDEFINED
    })
}

#[wasm_bindgen()]
pub fn table_db_transaction_rollback(id: u32) -> Promise {
    wrap_api_future_void(async move {
        let tdbt = {
            let tdbts = (*TABLE_DB_TRANSACTIONS).borrow();
            let Some(tdbt) = tdbts.get(&id) else {
                return APIResult::Err(veilid_core::VeilidAPIError::invalid_argument("table_db_transaction_rollback", "id", id));
            };
            tdbt.clone()
        };

        tdbt.rollback();
        APIRESULT_UNDEFINED
    })
}

#[wasm_bindgen()]
pub fn table_db_transaction_store(id: u32, col: u32, key: String, value: String) -> Promise {
    let key: Vec<u8> = data_encoding::BASE64URL_NOPAD
        .decode(key.as_bytes())
        .unwrap();
    let value: Vec<u8> = data_encoding::BASE64URL_NOPAD
        .decode(value.as_bytes())
        .unwrap();
    wrap_api_future_void(async move {
        let tdbt = {
            let tdbts = (*TABLE_DB_TRANSACTIONS).borrow();
            let Some(tdbt) = tdbts.get(&id) else {
                return APIResult::Err(veilid_core::VeilidAPIError::invalid_argument("table_db_transaction_store", "id", id));
            };
            tdbt.clone()
        };

        tdbt.store(col, &key, &value);
        APIRESULT_UNDEFINED
    })
}

#[wasm_bindgen()]
pub fn table_db_transaction_delete(id: u32, col: u32, key: String) -> Promise {
    let key: Vec<u8> = data_encoding::BASE64URL_NOPAD
        .decode(key.as_bytes())
        .unwrap();
    wrap_api_future_void(async move {
        let tdbt = {
            let tdbts = (*TABLE_DB_TRANSACTIONS).borrow();
            let Some(tdbt) = tdbts.get(&id) else {
                return APIResult::Err(veilid_core::VeilidAPIError::invalid_argument("table_db_transaction_delete", "id", id));
            };
            tdbt.clone()
        };

        tdbt.delete(col, &key);
        APIRESULT_UNDEFINED
    })
}

#[wasm_bindgen()]
pub fn table_db_store(id: u32, col: u32, key: String, value: String) -> Promise {
    let key: Vec<u8> = data_encoding::BASE64URL_NOPAD
        .decode(key.as_bytes())
        .unwrap();
    let value: Vec<u8> = data_encoding::BASE64URL_NOPAD
        .decode(value.as_bytes())
        .unwrap();
    wrap_api_future_void(async move {
        let table_db = {
            let table_dbs = (*TABLE_DBS).borrow();
            let Some(table_db) = table_dbs.get(&id) else {
                return APIResult::Err(veilid_core::VeilidAPIError::invalid_argument("table_db_store", "id", id));
            };
            table_db.clone()
        };

        table_db
            .store(col, &key, &value)
            .await
            .map_err(veilid_core::VeilidAPIError::generic)?;
        APIRESULT_UNDEFINED
    })
}

#[wasm_bindgen()]
pub fn table_db_load(id: u32, col: u32, key: String) -> Promise {
    let key: Vec<u8> = data_encoding::BASE64URL_NOPAD
        .decode(key.as_bytes())
        .unwrap();
    wrap_api_future_plain(async move {
        let table_db = {
            let table_dbs = (*TABLE_DBS).borrow();
            let Some(table_db) = table_dbs.get(&id) else {
                return APIResult::Err(veilid_core::VeilidAPIError::invalid_argument("table_db_load", "id", id));
            };
            table_db.clone()
        };

        let out = table_db
            .load(col, &key)
            .map_err(veilid_core::VeilidAPIError::generic)?;
        let out = out.map(|x| data_encoding::BASE64URL_NOPAD.encode(&x));
        APIResult::Ok(out)
    })
}

#[wasm_bindgen()]
pub fn table_db_delete(id: u32, col: u32, key: String) -> Promise {
    let key: Vec<u8> = data_encoding::BASE64URL_NOPAD
        .decode(key.as_bytes())
        .unwrap();
    wrap_api_future_plain(async move {
        let table_db = {
            let table_dbs = (*TABLE_DBS).borrow();
            let Some(table_db) = table_dbs.get(&id) else {
                return APIResult::Err(veilid_core::VeilidAPIError::invalid_argument("table_db_delete", "id", id));
            };
            table_db.clone()
        };

        let out = table_db
            .delete(col, &key)
            .await
            .map_err(veilid_core::VeilidAPIError::generic)?;
        APIResult::Ok(out)
    })
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
pub fn veilid_version_string() -> String {
    veilid_core::veilid_version_string()
}

#[derive(Serialize)]
pub struct VeilidVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

#[wasm_bindgen()]
pub fn veilid_version() -> JsValue {
    let (major, minor, patch) = veilid_core::veilid_version();
    let vv = VeilidVersion {
        major,
        minor,
        patch,
    };
    <JsValue as JsValueSerdeExt>::from_serde(&vv).unwrap()
}
