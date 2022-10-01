use crate::dart_isolate_wrapper::*;
use crate::tools::*;
use allo_isolate::*;
use cfg_if::*;
use ffi_support::*;
use lazy_static::*;
use opentelemetry::sdk::*;
use opentelemetry::*;
use opentelemetry_otlp::WithExportConfig;
use parking_lot::Mutex;
use serde::*;
use std::collections::BTreeMap;
use std::os::raw::c_char;
use std::sync::Arc;
use tracing::*;
use tracing_subscriber::prelude::*;
use tracing_subscriber::*;

// Globals
lazy_static! {
    static ref VEILID_API: AsyncMutex<Option<veilid_core::VeilidAPI>> = AsyncMutex::new(None);
    static ref FILTERS: Mutex<BTreeMap<&'static str, veilid_core::VeilidLayerFilter>> =
        Mutex::new(BTreeMap::new());
}

async fn get_veilid_api() -> Result<veilid_core::VeilidAPI, veilid_core::VeilidAPIError> {
    let api_lock = VEILID_API.lock().await;
    api_lock
        .as_ref()
        .cloned()
        .ok_or(veilid_core::VeilidAPIError::NotInitialized)
}

async fn take_veilid_api() -> Result<veilid_core::VeilidAPI, veilid_core::VeilidAPIError> {
    let mut api_lock = VEILID_API.lock().await;
    api_lock
        .take()
        .ok_or(veilid_core::VeilidAPIError::NotInitialized)
}

/////////////////////////////////////////
// FFI Helpers

// Declare external routine to release ffi strings
define_string_destructor!(free_string);

// Utility types for async API results
type APIResult<T> = Result<T, veilid_core::VeilidAPIError>;
const APIRESULT_VOID: APIResult<()> = APIResult::Ok(());

/////////////////////////////////////////
// FFI-specific cofnig

#[derive(Debug, Deserialize, Serialize)]
pub struct VeilidFFIConfigLoggingTerminal {
    pub enabled: bool,
    pub level: veilid_core::VeilidConfigLogLevel,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VeilidFFIConfigLoggingOtlp {
    pub enabled: bool,
    pub level: veilid_core::VeilidConfigLogLevel,
    pub grpc_endpoint: String,
    pub service_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VeilidFFIConfigLoggingApi {
    pub enabled: bool,
    pub level: veilid_core::VeilidConfigLogLevel,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VeilidFFIConfigLogging {
    pub terminal: VeilidFFIConfigLoggingTerminal,
    pub otlp: VeilidFFIConfigLoggingOtlp,
    pub api: VeilidFFIConfigLoggingApi,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VeilidFFIConfig {
    pub logging: VeilidFFIConfigLogging,
}

/////////////////////////////////////////
// Initializer
#[no_mangle]
#[instrument]
pub extern "C" fn initialize_veilid_flutter(dart_post_c_object_ptr: ffi::DartPostCObjectFnType) {
    unsafe {
        store_dart_post_cobject(dart_post_c_object_ptr);
    }

    use std::sync::Once;
    static INIT_BACKTRACE: Once = Once::new();
    INIT_BACKTRACE.call_once(move || {
        std::env::set_var("RUST_BACKTRACE", "1");
        std::panic::set_hook(Box::new(move |panic_info| {
            let (file, line) = if let Some(loc) = panic_info.location() {
                (loc.file(), loc.line())
            } else {
                ("<unknown>", 0)
            };
            error!("### Rust `panic!` hit at file '{}', line {}", file, line);
            if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
                error!("panic payload: {:?}", s);
            } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
                error!("panic payload: {:?}", s);
            } else if let Some(a) = panic_info.payload().downcast_ref::<std::fmt::Arguments>() {
                error!("panic payload: {:?}", a);
            } else {
                error!("no panic payload");
            }
            error!(
                "  Complete stack trace:\n{:?}\n",
                backtrace::Backtrace::new()
            );

            // And stop the process, no recovery is going to be possible here
            error!("aborting!");
            std::process::abort();
        }));
    });
}

//////////////////////////////////////////////////////////////////////////////////
/// C-compatible FFI Functions

#[no_mangle]
#[instrument]
pub extern "C" fn initialize_veilid_core(platform_config: FfiStr) {
    let platform_config = platform_config.into_opt_string();
    let platform_config: VeilidFFIConfig = veilid_core::deserialize_opt_json(platform_config)
        .expect("failed to deserialize plaform config json");

    // Set up subscriber and layers
    let subscriber = Registry::default();
    let mut layers = Vec::new();
    let mut filters = (*FILTERS).lock();

    // Terminal logger
    if platform_config.logging.terminal.enabled {
        let filter =
            veilid_core::VeilidLayerFilter::new(platform_config.logging.terminal.level, None);
        let layer = fmt::Layer::new()
            .compact()
            .with_writer(std::io::stdout)
            .with_filter(filter.clone());
        filters.insert("terminal", filter);
        layers.push(layer.boxed());
    };

    // OpenTelemetry logger
    if platform_config.logging.otlp.enabled {
        let grpc_endpoint = platform_config.logging.otlp.grpc_endpoint.clone();

        cfg_if! {
            if #[cfg(feature="rt-async-std")] {
                let exporter = opentelemetry_otlp::new_exporter()
                    .grpcio()
                    .with_endpoint(grpc_endpoint);
                let batch = opentelemetry::runtime::AsyncStd;
            } else if #[cfg(feature="rt-tokio")] {
                let exporter = opentelemetry_otlp::new_exporter()
                    .tonic()
                    .with_endpoint(format!("http://{}", grpc_endpoint));
                let batch = opentelemetry::runtime::Tokio;
            }
        }

        let tracer =
            opentelemetry_otlp::new_pipeline()
                .tracing()
                .with_exporter(exporter)
                .with_trace_config(opentelemetry::sdk::trace::config().with_resource(
                    Resource::new(vec![KeyValue::new(
                        opentelemetry_semantic_conventions::resource::SERVICE_NAME,
                        format!(
                        "{}:{}",
                        platform_config.logging.otlp.service_name,
                        hostname::get()
                            .map(|s| s.to_string_lossy().into_owned())
                            .unwrap_or_else(|_| "unknown".to_owned())),
                    )]),
                ))
                .install_batch(batch)
                .map_err(|e| format!("failed to install OpenTelemetry tracer: {}", e))
                .unwrap();

        let filter = veilid_core::VeilidLayerFilter::new(platform_config.logging.otlp.level, None);
        let layer = tracing_opentelemetry::layer()
            .with_tracer(tracer)
            .with_filter(filter.clone());
        filters.insert("otlp", filter);
        layers.push(layer.boxed());
    }

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
        .expect("failed to initalize ffi platform");
}

#[no_mangle]
#[instrument(level = "debug")]
pub extern "C" fn change_log_level(layer: FfiStr, log_level: FfiStr) {
    // get layer to change level on
    let layer = layer.into_opt_string().unwrap_or("all".to_owned());
    let layer = if layer == "all" { "".to_owned() } else { layer };

    // get log level to change layer to
    let log_level = log_level.into_opt_string();
    let log_level: veilid_core::VeilidConfigLogLevel =
        veilid_core::deserialize_opt_json(log_level).unwrap();

    // change log level on appropriate layer
    let filters = (*FILTERS).lock();
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

#[no_mangle]
#[instrument]
pub extern "C" fn startup_veilid_core(port: i64, stream_port: i64, config: FfiStr) {
    let config = config.into_opt_string();
    let stream = DartIsolateStream::new(stream_port);
    DartIsolateWrapper::new(port).spawn_result_json(async move {
        let config_json = match config {
            Some(v) => v,
            None => {
                let err = veilid_core::VeilidAPIError::MissingArgument {
                    context: "startup_veilid_core".to_owned(),
                    argument: "config".to_owned(),
                };
                return APIResult::Err(err);
            }
        };

        let mut api_lock = VEILID_API.lock().await;
        if api_lock.is_some() {
            return APIResult::Err(veilid_core::VeilidAPIError::AlreadyInitialized);
        }

        let sink = stream.clone();
        let update_callback = Arc::new(move |update: veilid_core::VeilidUpdate| {
            let sink = sink.clone();
            match update {
                veilid_core::VeilidUpdate::Shutdown => {
                    sink.close();
                }
                _ => {
                    sink.item_json(update);
                }
            }
        });

        let veilid_api = veilid_core::api_startup_json(update_callback, config_json).await?;
        *api_lock = Some(veilid_api);

        APIRESULT_VOID
    });
}

#[no_mangle]
pub extern "C" fn get_veilid_state(port: i64) {
    DartIsolateWrapper::new(port).spawn_result_json(async move {
        let veilid_api = get_veilid_api().await?;
        let core_state = veilid_api.get_state().await?;
        APIResult::Ok(core_state)
    });
}

#[no_mangle]
pub extern "C" fn attach(port: i64) {
    DartIsolateWrapper::new(port).spawn_result_json(async move {
        let veilid_api = get_veilid_api().await?;
        veilid_api.attach().await?;
        APIRESULT_VOID
    });
}

#[no_mangle]
pub extern "C" fn detach(port: i64) {
    DartIsolateWrapper::new(port).spawn_result_json(async move {
        let veilid_api = get_veilid_api().await?;
        veilid_api.detach().await?;
        APIRESULT_VOID
    });
}

#[no_mangle]
#[instrument]
pub extern "C" fn shutdown_veilid_core(port: i64) {
    DartIsolateWrapper::new(port).spawn_result_json(async move {
        let veilid_api = take_veilid_api().await?;
        veilid_api.shutdown().await;
        APIRESULT_VOID
    });
}

#[no_mangle]
pub extern "C" fn debug(port: i64, command: FfiStr) {
    let command = command.into_opt_string().unwrap_or_default();
    DartIsolateWrapper::new(port).spawn_result(async move {
        let veilid_api = get_veilid_api().await?;
        let out = veilid_api.debug(command).await?;
        APIResult::Ok(out)
    });
}

#[no_mangle]
pub extern "C" fn app_call_reply(port: i64, id: FfiStr, message: FfiStr) {
    let id = id.into_opt_string().unwrap_or_default();
    let message = message.into_opt_string().unwrap_or_default();
    DartIsolateWrapper::new(port).spawn_result(async move {
        let id = match id.parse() {
            Ok(v) => v,
            Err(e) => {
                return APIResult::Err(veilid_core::VeilidAPIError::invalid_argument(e, "id", id))
            }
        };
        let message = data_encoding::BASE64URL_NOPAD
            .decode(message.as_bytes())
            .map_err(|e| veilid_core::VeilidAPIError::invalid_argument(e, "message", message))?;
        let veilid_api = get_veilid_api().await?;
        veilid_api.app_call_reply(id, message).await?;
        APIRESULT_VOID
    });
}

#[no_mangle]
pub extern "C" fn veilid_version_string() -> *mut c_char {
    veilid_core::veilid_version_string().into_ffi_value()
}

#[repr(C)]
pub struct VeilidVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

#[no_mangle]
#[instrument]
pub extern "C" fn veilid_version() -> VeilidVersion {
    let (major, minor, patch) = veilid_core::veilid_version();
    VeilidVersion {
        major,
        minor,
        patch,
    }
}
