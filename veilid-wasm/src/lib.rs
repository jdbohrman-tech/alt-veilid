// wasm-bindgen and clippy don't play well together yet
#![allow(clippy::all)]
#![cfg(target_arch = "wasm32")]
#![no_std]

extern crate alloc;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::*;
use core::any::{Any, TypeId};
use core::cell::RefCell;
use futures_util::FutureExt;
use js_sys::*;
use lazy_static::*;
use send_wrapper::*;
use serde::*;
use tracing::*;
use tracing_subscriber::prelude::*;
use tracing_subscriber::*;
use tracing_wasm::{WASMLayerConfigBuilder, *};
use veilid_core::xx::*;
use veilid_core::*;
use wasm_bindgen_futures::*;

// Allocator
extern crate wee_alloc;
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

static SETUP_ONCE: Once = Once::new();
pub fn setup() -> () {
    SETUP_ONCE.call_once(|| {});
}

// API Singleton
lazy_static! {
    static ref VEILID_API: SendWrapper<RefCell<Option<veilid_core::VeilidAPI>>> =
        SendWrapper::new(RefCell::new(None));
    static ref FILTERS: SendWrapper<RefCell<BTreeMap<&'static str, veilid_core::VeilidLayerFilter>>> =
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

// JSON Marshalling
pub fn serialize_json<T: Serialize>(val: T) -> String {
    serde_json::to_string(&val).expect("failed to serialize json value")
}

pub fn deserialize_json<T: de::DeserializeOwned>(
    arg: &str,
) -> Result<T, veilid_core::VeilidAPIError> {
    serde_json::from_str(arg).map_err(|e| veilid_core::VeilidAPIError::ParseError {
        message: e.to_string(),
        value: String::new(),
    })
}

pub fn to_json<T: Serialize>(val: T) -> JsValue {
    JsValue::from_str(&serialize_json(val))
}

pub fn from_json<T: de::DeserializeOwned>(val: JsValue) -> Result<T, veilid_core::VeilidAPIError> {
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

pub fn wrap_api_future<F, T>(future: F) -> Promise
where
    F: Future<Output = APIResult<T>> + 'static,
    T: Serialize + 'static,
{
    future_to_promise(future.map(|res| {
        res.map(|v| {
            if TypeId::of::<()>() == v.type_id() {
                JsValue::UNDEFINED
            } else {
                to_json(v)
            }
        })
        .map_err(|e| to_json(e))
    }))
}

/////////////////////////////////////////
// WASM-specific cofnig

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

// WASM Bindings

#[wasm_bindgen()]
pub fn initialize_veilid_wasm() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen()]
pub fn initialize_veilid_core(platform_config: String) {
    let platform_config: VeilidWASMConfig = veilid_core::deserialize_json(&platform_config)
        .expect("failed to deserialize platform config json");

    // Set up subscriber and layers
    let subscriber = Registry::default();
    let mut layers = Vec::new();
    let mut filters = (*FILTERS).borrow_mut();

    // Performance logger
    if platform_config.logging.performance.enabled {
        let filter =
            veilid_core::VeilidLayerFilter::new(platform_config.logging.performance.level, None);
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
        let filter = veilid_core::VeilidLayerFilter::new(platform_config.logging.api.level, None);
        let layer = veilid_core::ApiTracingLayer::get().with_filter(filter.clone());
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
    wrap_api_future(async move {
        let update_callback = Arc::new(move |update: VeilidUpdate| {
            let _ret =
                match Function::call1(&update_callback_js, &JsValue::UNDEFINED, &to_json(update)) {
                    Ok(v) => v,
                    Err(e) => {
                        error!("calling update callback failed: {:?}", e);
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
    wrap_api_future(async move {
        let veilid_api = get_veilid_api()?;
        let core_state = veilid_api.get_state().await?;
        Ok(core_state)
    })
}

#[wasm_bindgen()]
pub fn attach() -> Promise {
    wrap_api_future(async move {
        let veilid_api = get_veilid_api()?;
        veilid_api.attach().await?;
        APIRESULT_UNDEFINED
    })
}

#[wasm_bindgen()]
pub fn detach() -> Promise {
    wrap_api_future(async move {
        let veilid_api = get_veilid_api()?;
        veilid_api.detach().await?;
        APIRESULT_UNDEFINED
    })
}

#[wasm_bindgen()]
pub fn shutdown_veilid_core() -> Promise {
    wrap_api_future(async move {
        let veilid_api = take_veilid_api()?;
        veilid_api.shutdown().await;
        APIRESULT_UNDEFINED
    })
}

#[wasm_bindgen()]
pub fn debug(command: String) -> Promise {
    wrap_api_future(async move {
        let veilid_api = get_veilid_api()?;
        let out = veilid_api.debug(command).await?;
        Ok(out)
    })
}

#[wasm_bindgen()]
pub fn app_call_reply(id: String, message: String) -> Promise {
    wrap_api_future(async move {
        let id = match id.parse() {
            Ok(v) => v,
            Err(e) => {
                return APIResult::Err(veilid_core::VeilidAPIError::invalid_argument(e, "id", id))
            }
        };
        let message = data_encoding::BASE64URL_NOPAD
            .decode(message.as_bytes())
            .map_err(|e| veilid_core::VeilidAPIError::invalid_argument(e, "message", message))?;
        let veilid_api = get_veilid_api()?;
        let out = veilid_api.app_call_reply(id, message).await?;
        Ok(out)
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
    JsValue::from_serde(&vv).unwrap()
}
