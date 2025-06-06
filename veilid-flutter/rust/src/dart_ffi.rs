use crate::dart_isolate_wrapper::*;
use allo_isolate::*;
use cfg_if::*;
use data_encoding::BASE64URL_NOPAD;
use ffi_support::*;
use lazy_static::*;
use opentelemetry::sdk::*;
use opentelemetry::*;
use opentelemetry_otlp::WithExportConfig;
use parking_lot::Mutex;
use serde::*;
use std::io::Write;
use std::os::raw::c_char;
use std::sync::Arc;
use tracing::*;
use tracing_flame::FlameLayer;
use tracing_subscriber::prelude::*;
use veilid_core::{tools::*, Encodable};

// Detect flutter load/unload
cfg_if! {
    if #[cfg(feature="debug-load")] {
        #[ctor::ctor]
        fn onload() {
            cfg_if! {
                if #[cfg(target_os="android")] {
                    use android_log_sys::*;
                    use std::ffi::{CString, c_int, c_char};
                    unsafe {
                        let tag = CString::new("veilid").unwrap();
                        let text = CString::new(">>> VEILID-FLUTTER LOADED <<<").unwrap();
                        __android_log_write(LogPriority::INFO as c_int, tag.as_ptr() as *const c_char, text.as_ptr() as *const c_char);
                    }
                } else {
                    libc_print::libc_println!(">>> VEILID-FLUTTER LOADED <<<");
                }
            }
        }
        #[ctor::dtor]
        fn onunload() {
            cfg_if! {
                if #[cfg(target_os="android")] {
                    use android_log_sys::*;
                    use std::ffi::{CString, c_int, c_char};
                    unsafe {
                        let tag = CString::new("veilid").unwrap();
                        let text = CString::new(">>> VEILID-FLUTTER UNLOADED <<<").unwrap();
                        __android_log_write(LogPriority::INFO as c_int, tag.as_ptr() as *const c_char, text.as_ptr() as *const c_char);
                    }
                } else {
                    libc_print::libc_println!(">>> VEILID-FLUTTER UNLOADED <<<");
                }
            }
        }
    }
}

// Globals
lazy_static! {
    static ref CORE_INITIALIZED: Mutex<bool> = Mutex::new(false);
    static ref VEILID_API: AsyncMutex<Option<veilid_core::VeilidAPI>> = AsyncMutex::new(None);
    static ref FILTERS: Mutex<BTreeMap<&'static str, veilid_core::VeilidLayerFilter>> =
        Mutex::new(BTreeMap::new());
    static ref ROUTING_CONTEXTS: Mutex<BTreeMap<u32, veilid_core::RoutingContext>> =
        Mutex::new(BTreeMap::new());
    static ref TABLE_DBS: Mutex<BTreeMap<u32, veilid_core::TableDB>> = Mutex::new(BTreeMap::new());
    static ref TABLE_DB_TRANSACTIONS: Mutex<BTreeMap<u32, veilid_core::TableDBTransaction>> =
        Mutex::new(BTreeMap::new());
    static ref FLAME_GUARD: Mutex<Option<tracing_flame::FlushGuard<std::io::BufWriter<std::fs::File>>>> =
        Mutex::new(None);
}

async fn get_veilid_api() -> veilid_core::VeilidAPIResult<veilid_core::VeilidAPI> {
    let api_lock = VEILID_API.lock().await;
    api_lock
        .as_ref()
        .cloned()
        .ok_or(veilid_core::VeilidAPIError::NotInitialized)
}

/////////////////////////////////////////
// FFI Helpers

// Declare external routine to release ffi strings
define_string_destructor!(free_string);

// Utility types for async API results
type APIResult<T> = veilid_core::VeilidAPIResult<T>;
const APIRESULT_VOID: APIResult<()> = APIResult::Ok(());

/////////////////////////////////////////
// FFI-specific

#[derive(Debug, Deserialize, Serialize)]
pub struct VeilidFFIConfigLoggingTerminal {
    pub enabled: bool,
    pub level: veilid_core::VeilidConfigLogLevel,
    pub ignore_log_targets: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VeilidFFIConfigLoggingOtlp {
    pub enabled: bool,
    pub level: veilid_core::VeilidConfigLogLevel,
    pub grpc_endpoint: String,
    pub service_name: String,
    pub ignore_log_targets: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VeilidFFIConfigLoggingApi {
    pub enabled: bool,
    pub level: veilid_core::VeilidConfigLogLevel,
    pub ignore_log_targets: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VeilidFFIConfigLoggingFlame {
    pub enabled: bool,
    pub path: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VeilidFFIConfigLogging {
    pub terminal: VeilidFFIConfigLoggingTerminal,
    pub otlp: VeilidFFIConfigLoggingOtlp,
    pub api: VeilidFFIConfigLoggingApi,
    pub flame: VeilidFFIConfigLoggingFlame,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VeilidFFIConfig {
    pub logging: VeilidFFIConfigLogging,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VeilidFFIRouteBlob {
    pub route_id: veilid_core::RouteId,
    #[serde(with = "veilid_core::as_human_base64")]
    pub blob: Vec<u8>,
}

/////////////////////////////////////////
// Initializer
#[no_mangle]
#[instrument]
pub extern "C" fn initialize_veilid_flutter(
    dart_post_c_object_ptr: ffi::DartPostCObjectFnType,
    crash_path: FfiStr,
) {
    unsafe {
        store_dart_post_cobject(dart_post_c_object_ptr);
    }
    let crash_path = crash_path.into_opt_string().unwrap_or_default();

    use std::sync::Once;
    static INIT_BACKTRACE: Once = Once::new();
    INIT_BACKTRACE.call_once(move || {
        std::env::set_var("RUST_BACKTRACE", "1");
        std::panic::set_hook(Box::new(move |panic_info| {
            let crash_file = if crash_path.is_empty() {
                None
            } else {
                Some(std::fs::File::create(&crash_path).unwrap())
            };

            let (file, line) = if let Some(loc) = panic_info.location() {
                (loc.file(), loc.line())
            } else {
                ("<unknown>", 0)
            };

            let mut out = String::new();

            out += &format!("### Rust `panic!` hit at file '{}', line {}\n", file, line);
            if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
                out += &format!("panic payload: {:?}\n", s);
            } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
                out += &format!("panic payload: {:?}\n", s);
            } else if let Some(a) = panic_info.payload().downcast_ref::<std::fmt::Arguments>() {
                out += &format!("panic payload: {:?}\n", a);
            } else {
                out += "no panic payload\n";
            }
            out += &format!(
                "  Complete stack trace:\n{:?}\n",
                backtrace::Backtrace::new()
            );

            if let Some(mut crash_file) = crash_file {
                write!(crash_file, "{}", out).unwrap();
                crash_file.flush().unwrap();
            } else {
                eprintln!("{}", out);
            }

            // And stop the process, no recovery is going to be possible here
            eprintln!("aborting!");
            std::process::abort();
        }));
    });
}

//////////////////////////////////////////////////////////////////////////////////
// C-compatible FFI Functions

#[no_mangle]
#[instrument]
pub extern "C" fn initialize_veilid_core(platform_config: FfiStr) {
    // Only do this once, ever
    // Until we have Dart native finalizers running on hot-restart, this will cause a crash if run more than once
    {
        let mut core_init = CORE_INITIALIZED.lock();
        if *core_init {
            return;
        }
        *core_init = true;
    }

    let platform_config = platform_config.into_opt_string();
    let platform_config: VeilidFFIConfig = veilid_core::deserialize_opt_json(platform_config)
        .expect("failed to deserialize plaform config json");

    // Set up subscriber and layers
    let subscriber = tracing_subscriber::Registry::default();
    let mut layers = Vec::new();
    let mut filters = (*FILTERS).lock();

    let mut fields_to_strip = HashSet::<&'static str>::new();
    fields_to_strip.insert(veilid_core::VEILID_LOG_KEY_FIELD);

    // Terminal logger
    if platform_config.logging.terminal.enabled {
        cfg_if! {
            if #[cfg(target_os = "android")] {
                let filter =
                    veilid_core::VeilidLayerFilter::new(platform_config.logging.terminal.level, &platform_config.logging.terminal.ignore_log_targets, None);
                let layer = paranoid_android::layer("veilid-flutter")
                    .map_fmt_fields(|f| veilid_core::FmtStripFields::new(f, fields_to_strip.clone()))
                    .with_ansi(false)
                    .with_filter(filter.clone());
                filters.insert("terminal", filter);
                layers.push(layer.boxed());
            } else if #[cfg(target_os = "ios")] {
                let filter =
                    veilid_core::VeilidLayerFilter::new(platform_config.logging.terminal.level, &platform_config.logging.terminal.ignore_log_targets, None);
                let layer = tracing_subscriber::fmt::Layer::new()
                    .compact()
                    .map_fmt_fields(|f| veilid_core::FmtStripFields::new(f, fields_to_strip.clone()))
                    .with_ansi(false)
                    .with_writer(std::io::stdout)
                    .with_filter(filter.clone());
                filters.insert("terminal", filter);
                layers.push(layer.boxed());
            } else {
                let filter =
                    veilid_core::VeilidLayerFilter::new(platform_config.logging.terminal.level, &platform_config.logging.terminal.ignore_log_targets, None);
                let layer = tracing_subscriber::fmt::Layer::new()
                    .compact()
                    .map_fmt_fields(|f| veilid_core::FmtStripFields::new(f, fields_to_strip.clone()))
                    .with_writer(std::io::stdout)
                    .with_filter(filter.clone());
                filters.insert("terminal", filter);
                layers.push(layer.boxed());
            }
        }
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
            } else {
                compile_error!("needs executor implementation");
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

        let filter = veilid_core::VeilidLayerFilter::new(
            platform_config.logging.otlp.level,
            &platform_config.logging.otlp.ignore_log_targets,
            None,
        );
        let layer = tracing_opentelemetry::layer()
            .with_tracer(tracer)
            .with_filter(filter.clone());
        filters.insert("otlp", filter);
        layers.push(layer.boxed());
    }

    // Flamegraph logger
    if platform_config.logging.flame.enabled {
        let filter = veilid_core::VeilidLayerFilter::new_no_default(
            veilid_core::VeilidConfigLogLevel::Trace,
            &veilid_core::FLAME_LOG_FACILITIES_IGNORE_LIST
                .iter()
                .map(|&x| x.to_string())
                .collect::<Vec<_>>(),
            None,
        );
        let (flame_layer, guard) =
            FlameLayer::with_file(&platform_config.logging.flame.path).unwrap();
        *FLAME_GUARD.lock() = Some(guard);

        // Do not include this in change_log_level changes, so we keep trace level
        // filters.insert("flame", filter.clone());

        layers.push(
            flame_layer
                .with_threads_collapsed(true)
                .with_empty_samples(false)
                .with_filter(filter)
                .boxed(),
        );
    }

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
        .expect("failed to initalize ffi platform");
}

#[no_mangle]
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
pub extern "C" fn change_log_ignore(layer: FfiStr, log_ignore: FfiStr) {
    // get layer to change level on
    let layer = layer.into_opt_string().unwrap_or("all".to_owned());
    let layer = if layer == "all" { "".to_owned() } else { layer };

    // get changes to make
    let log_ignore = log_ignore.into_opt_string().unwrap_or_default();

    // change log level on appropriate layer
    let filters = (*FILTERS).lock();
    if layer.is_empty() {
        // Change all layers
        for f in filters.values() {
            f.set_ignore_list(Some(veilid_core::VeilidLayerFilter::apply_ignore_change(
                &f.ignore_list(),
                log_ignore.clone(),
            )));
        }
    } else {
        // Change a specific layer
        let f = filters.get(layer.as_str()).unwrap();
        f.set_ignore_list(Some(veilid_core::VeilidLayerFilter::apply_ignore_change(
            &f.ignore_list(),
            log_ignore.clone(),
        )));
    }
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn startup_veilid_core(port: i64, stream_port: i64, config: FfiStr) {
    let config = config.into_opt_string();
    let stream = DartIsolateStream::new(stream_port);
    DartIsolateWrapper::new(port).spawn_result(
        async move {
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
        }
        .in_current_span(),
    );
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn get_veilid_state(port: i64) {
    DartIsolateWrapper::new(port).spawn_result_json(
        async move {
            let veilid_api = get_veilid_api().await?;
            let core_state = veilid_api.get_state().await?;
            APIResult::Ok(core_state)
        }
        .in_current_span(),
    );
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn is_shutdown(port: i64) {
    DartIsolateWrapper::new(port).spawn_result(
        async move {
            let veilid_api = get_veilid_api().await;
            if let Err(veilid_core::VeilidAPIError::NotInitialized) = veilid_api {
                return APIResult::Ok(true);
            }
            let veilid_api = veilid_api.unwrap();
            let is_shutdown = veilid_api.is_shutdown();
            APIResult::Ok(is_shutdown)
        }
        .in_current_span(),
    );
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn attach(port: i64) {
    DartIsolateWrapper::new(port).spawn_result(
        async move {
            let veilid_api = get_veilid_api().await?;
            veilid_api.attach().await?;
            APIRESULT_VOID
        }
        .in_current_span(),
    );
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn detach(port: i64) {
    DartIsolateWrapper::new(port).spawn_result(
        async move {
            let veilid_api = get_veilid_api().await?;
            veilid_api.detach().await?;
            APIRESULT_VOID
        }
        .in_current_span(),
    );
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn shutdown_veilid_core(port: i64) {
    DartIsolateWrapper::new(port).spawn_result(
        async move {
            let mut api_lock = VEILID_API.lock().await;
            let veilid_api = api_lock
                .take()
                .ok_or(veilid_core::VeilidAPIError::NotInitialized)?;
            veilid_api.shutdown().await;
            APIRESULT_VOID
        }
        .in_current_span(),
    );
}

fn add_routing_context(
    rc: &mut BTreeMap<u32, veilid_core::RoutingContext>,
    routing_context: veilid_core::RoutingContext,
) -> u32 {
    let mut next_id: u32 = 1;
    while rc.contains_key(&next_id) {
        next_id += 1;
    }
    rc.insert(next_id, routing_context);
    next_id
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn routing_context(port: i64) {
    DartIsolateWrapper::new(port).spawn_result(
        async move {
            let veilid_api = get_veilid_api().await?;
            let routing_context = veilid_api.routing_context()?;
            let mut rc = ROUTING_CONTEXTS.lock();
            let new_id = add_routing_context(&mut rc, routing_context);
            APIResult::Ok(new_id)
        }
        .in_current_span(),
    );
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn release_routing_context(id: u32) -> i32 {
    let mut rc = ROUTING_CONTEXTS.lock();
    if rc.remove(&id).is_none() {
        return 0;
    }
    1
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn routing_context_with_default_safety(id: u32) -> u32 {
    let mut rc = ROUTING_CONTEXTS.lock();
    let Some(routing_context) = rc.get(&id) else {
        return 0;
    };
    let Ok(routing_context) = routing_context.clone().with_default_safety() else {
        return 0;
    };

    add_routing_context(&mut rc, routing_context)
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn routing_context_with_safety(id: u32, safety_selection: FfiStr) -> u32 {
    let safety_selection: veilid_core::SafetySelection =
        veilid_core::deserialize_opt_json(safety_selection.into_opt_string()).unwrap();

    let mut rc = ROUTING_CONTEXTS.lock();
    let Some(routing_context) = rc.get(&id) else {
        return 0;
    };
    let Ok(routing_context) = routing_context.clone().with_safety(safety_selection) else {
        return 0;
    };

    add_routing_context(&mut rc, routing_context)
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn routing_context_with_sequencing(id: u32, sequencing: FfiStr) -> u32 {
    let sequencing: veilid_core::Sequencing =
        veilid_core::deserialize_opt_json(sequencing.into_opt_string()).unwrap();

    let mut rc = ROUTING_CONTEXTS.lock();
    let Some(routing_context) = rc.get(&id) else {
        return 0;
    };
    let routing_context = routing_context.clone().with_sequencing(sequencing);

    add_routing_context(&mut rc, routing_context)
}

fn get_routing_context(id: u32, func_name: &str) -> APIResult<veilid_core::RoutingContext> {
    let rc = ROUTING_CONTEXTS.lock();
    let Some(routing_context) = rc.get(&id) else {
        return APIResult::Err(veilid_core::VeilidAPIError::invalid_argument(
            func_name, "id", id,
        ));
    };
    Ok(routing_context.clone())
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn routing_context_safety(port: i64, id: u32) {
    DartIsolateWrapper::new(port).spawn_result_json(async move {
        let routing_context = get_routing_context(id, "routing_context_safety")?;
        APIResult::Ok(routing_context.safety())
    });
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn routing_context_app_call(port: i64, id: u32, target: FfiStr, request: FfiStr) {
    let target_string: String = target.into_opt_string().unwrap();
    let request: Vec<u8> = data_encoding::BASE64URL_NOPAD
        .decode(request.into_opt_string().unwrap().as_bytes())
        .unwrap();
    DartIsolateWrapper::new(port).spawn_result(
        async move {
            let routing_context = get_routing_context(id, "routing_context_app_call")?;

            let veilid_api = get_veilid_api().await?;
            let target = veilid_api.parse_as_target(target_string)?;
            let answer = routing_context.app_call(target, request).await?;
            let answer = data_encoding::BASE64URL_NOPAD.encode(&answer);
            APIResult::Ok(answer)
        }
        .in_current_span(),
    );
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn routing_context_app_message(port: i64, id: u32, target: FfiStr, message: FfiStr) {
    let target_string: String = target.into_opt_string().unwrap();
    let message: Vec<u8> = data_encoding::BASE64URL_NOPAD
        .decode(message.into_opt_string().unwrap().as_bytes())
        .unwrap();
    DartIsolateWrapper::new(port).spawn_result(
        async move {
            let routing_context = get_routing_context(id, "routing_context_app_message")?;

            let veilid_api = get_veilid_api().await?;
            let target = veilid_api.parse_as_target(target_string)?;
            routing_context.app_message(target, message).await?;
            APIRESULT_VOID
        }
        .in_current_span(),
    );
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn routing_context_create_dht_record(
    port: i64,
    id: u32,
    schema: FfiStr,
    owner: FfiStr,
    kind: u32,
) {
    let crypto_kind = if kind == 0 {
        None
    } else {
        Some(veilid_core::CryptoKind::from(kind))
    };
    let schema: veilid_core::DHTSchema =
        veilid_core::deserialize_opt_json(schema.into_opt_string()).unwrap();
    let owner: Option<veilid_core::KeyPair> = owner
        .into_opt_string()
        .map(|s| veilid_core::deserialize_json(&s).unwrap());

    DartIsolateWrapper::new(port).spawn_result_json(
        async move {
            let routing_context = get_routing_context(id, "routing_context_create_dht_record")?;

            let dht_record_descriptor = routing_context
                .create_dht_record(schema, owner, crypto_kind)
                .await?;
            APIResult::Ok(dht_record_descriptor)
        }
        .in_current_span(),
    );
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn routing_context_open_dht_record(port: i64, id: u32, key: FfiStr, writer: FfiStr) {
    let key: veilid_core::TypedRecordKey =
        veilid_core::deserialize_opt_json(key.into_opt_string()).unwrap();
    let writer: Option<veilid_core::KeyPair> = writer
        .into_opt_string()
        .map(|s| veilid_core::deserialize_json(&s).unwrap());
    DartIsolateWrapper::new(port).spawn_result_json(
        async move {
            let routing_context = get_routing_context(id, "routing_context_open_dht_record")?;

            let dht_record_descriptor = routing_context.open_dht_record(key, writer).await?;
            APIResult::Ok(dht_record_descriptor)
        }
        .in_current_span(),
    );
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn routing_context_close_dht_record(port: i64, id: u32, key: FfiStr) {
    let key: veilid_core::TypedRecordKey =
        veilid_core::deserialize_opt_json(key.into_opt_string()).unwrap();
    DartIsolateWrapper::new(port).spawn_result(
        async move {
            let routing_context = get_routing_context(id, "routing_context_close_dht_record")?;

            routing_context.close_dht_record(key).await?;
            APIRESULT_VOID
        }
        .in_current_span(),
    );
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn routing_context_delete_dht_record(port: i64, id: u32, key: FfiStr) {
    let key: veilid_core::TypedRecordKey =
        veilid_core::deserialize_opt_json(key.into_opt_string()).unwrap();
    DartIsolateWrapper::new(port).spawn_result(
        async move {
            let routing_context = get_routing_context(id, "routing_context_delete_dht_record")?;

            routing_context.delete_dht_record(key).await?;
            APIRESULT_VOID
        }
        .in_current_span(),
    );
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn routing_context_get_dht_value(
    port: i64,
    id: u32,
    key: FfiStr,
    subkey: u32,
    force_refresh: bool,
) {
    let key: veilid_core::TypedRecordKey =
        veilid_core::deserialize_opt_json(key.into_opt_string()).unwrap();
    DartIsolateWrapper::new(port).spawn_result_json(
        async move {
            let routing_context = get_routing_context(id, "routing_context_get_dht_value")?;

            let res = routing_context
                .get_dht_value(key, subkey, force_refresh)
                .await?;
            APIResult::Ok(res)
        }
        .in_current_span(),
    );
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn routing_context_set_dht_value(
    port: i64,
    id: u32,
    key: FfiStr,
    subkey: u32,
    data: FfiStr,
    writer: FfiStr,
) {
    let key: veilid_core::TypedRecordKey =
        veilid_core::deserialize_opt_json(key.into_opt_string()).unwrap();
    let data: Vec<u8> = data_encoding::BASE64URL_NOPAD
        .decode(data.into_opt_string().unwrap().as_bytes())
        .unwrap();
    let writer: Option<veilid_core::KeyPair> = writer
        .into_opt_string()
        .map(|s| veilid_core::deserialize_json(&s).unwrap());

    DartIsolateWrapper::new(port).spawn_result_json(
        async move {
            let routing_context = get_routing_context(id, "routing_context_set_dht_value")?;

            let res = routing_context
                .set_dht_value(key, subkey, data, writer)
                .await?;
            APIResult::Ok(res)
        }
        .in_current_span(),
    );
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn routing_context_watch_dht_values(
    port: i64,
    id: u32,
    key: FfiStr,
    subkeys: FfiStr,
    expiration: u64,
    count: u32,
) {
    let key: veilid_core::TypedRecordKey =
        veilid_core::deserialize_opt_json(key.into_opt_string()).unwrap();
    let subkeys: veilid_core::ValueSubkeyRangeSet =
        veilid_core::deserialize_opt_json(subkeys.into_opt_string()).unwrap();
    let expiration = veilid_core::Timestamp::from(expiration);

    DartIsolateWrapper::new(port).spawn_result(
        async move {
            let routing_context = get_routing_context(id, "routing_context_watch_dht_values")?;

            let res = routing_context
                .watch_dht_values(key, Some(subkeys), Some(expiration), Some(count))
                .await?;
            APIResult::Ok(res)
        }
        .in_current_span(),
    );
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn routing_context_cancel_dht_watch(
    port: i64,
    id: u32,
    key: FfiStr,
    subkeys: FfiStr,
) {
    let key: veilid_core::TypedRecordKey =
        veilid_core::deserialize_opt_json(key.into_opt_string()).unwrap();
    let subkeys: veilid_core::ValueSubkeyRangeSet =
        veilid_core::deserialize_opt_json(subkeys.into_opt_string()).unwrap();

    DartIsolateWrapper::new(port).spawn_result(
        async move {
            let routing_context = get_routing_context(id, "routing_context_cancel_dht_watch")?;

            let res = routing_context.cancel_dht_watch(key, Some(subkeys)).await?;
            APIResult::Ok(res)
        }
        .in_current_span(),
    );
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn routing_context_inspect_dht_record(
    port: i64,
    id: u32,
    key: FfiStr,
    subkeys: FfiStr,
    scope: FfiStr,
) {
    let key: veilid_core::TypedRecordKey =
        veilid_core::deserialize_opt_json(key.into_opt_string()).unwrap();
    let subkeys: veilid_core::ValueSubkeyRangeSet =
        veilid_core::deserialize_opt_json(subkeys.into_opt_string()).unwrap();
    let scope: veilid_core::DHTReportScope =
        veilid_core::deserialize_opt_json(scope.into_opt_string()).unwrap();

    DartIsolateWrapper::new(port).spawn_result_json(
        async move {
            let routing_context = get_routing_context(id, "routing_context_inspect_dht_record")?;

            let res = routing_context
                .inspect_dht_record(key, Some(subkeys), scope)
                .await?;
            APIResult::Ok(res)
        }
        .in_current_span(),
    );
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn new_private_route(port: i64) {
    DartIsolateWrapper::new(port).spawn_result_json(
        async move {
            let veilid_api = get_veilid_api().await?;

            let (route_id, blob) = veilid_api.new_private_route().await?;

            let route_blob = VeilidFFIRouteBlob { route_id, blob };

            APIResult::Ok(route_blob)
        }
        .in_current_span(),
    );
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn new_custom_private_route(port: i64, stability: FfiStr, sequencing: FfiStr) {
    let stability: veilid_core::Stability =
        veilid_core::deserialize_opt_json(stability.into_opt_string()).unwrap();
    let sequencing: veilid_core::Sequencing =
        veilid_core::deserialize_opt_json(sequencing.into_opt_string()).unwrap();

    DartIsolateWrapper::new(port).spawn_result_json(
        async move {
            let veilid_api = get_veilid_api().await?;

            let (route_id, blob) = veilid_api
                .new_custom_private_route(&veilid_core::VALID_CRYPTO_KINDS, stability, sequencing)
                .await?;

            let route_blob = VeilidFFIRouteBlob { route_id, blob };

            APIResult::Ok(route_blob)
        }
        .in_current_span(),
    );
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn import_remote_private_route(port: i64, blob: FfiStr) {
    let blob: Vec<u8> = data_encoding::BASE64URL_NOPAD
        .decode(blob.into_opt_string().unwrap().as_bytes())
        .unwrap();
    DartIsolateWrapper::new(port).spawn_result(
        async move {
            let veilid_api = get_veilid_api().await?;

            let route_id = veilid_api.import_remote_private_route(blob)?.encode();

            APIResult::Ok(route_id)
        }
        .in_current_span(),
    );
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn release_private_route(port: i64, route_id: FfiStr) {
    let route_id = veilid_core::RouteId::try_decode(route_id.into_string()).unwrap();
    DartIsolateWrapper::new(port).spawn_result(
        async move {
            let veilid_api = get_veilid_api().await?;
            veilid_api.release_private_route(route_id)?;
            APIRESULT_VOID
        }
        .in_current_span(),
    );
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn app_call_reply(port: i64, call_id: FfiStr, message: FfiStr) {
    let call_id = call_id.into_opt_string().unwrap_or_default();
    let message = data_encoding::BASE64URL_NOPAD
        .decode(message.into_opt_string().unwrap().as_bytes())
        .unwrap();

    DartIsolateWrapper::new(port).spawn_result(
        async move {
            let call_id = match call_id.parse() {
                Ok(v) => v,
                Err(e) => {
                    return APIResult::Err(veilid_core::VeilidAPIError::invalid_argument(
                        e, "call_id", call_id,
                    ))
                }
            };

            let veilid_api = get_veilid_api().await?;
            veilid_api.app_call_reply(call_id, message).await?;
            APIRESULT_VOID
        }
        .in_current_span(),
    );
}

fn add_table_db(table_db: veilid_core::TableDB) -> u32 {
    let mut next_id: u32 = 1;
    let mut rc = TABLE_DBS.lock();
    while rc.contains_key(&next_id) {
        next_id += 1;
    }
    rc.insert(next_id, table_db);
    next_id
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn open_table_db(port: i64, name: FfiStr, column_count: u32) {
    let name = name.into_opt_string().unwrap_or_default();
    DartIsolateWrapper::new(port).spawn_result(
        async move {
            let veilid_api = get_veilid_api().await?;
            let tstore = veilid_api.table_store()?;
            let table_db = tstore
                .open(&name, column_count)
                .await
                .map_err(veilid_core::VeilidAPIError::generic)?;
            let new_id = add_table_db(table_db);
            APIResult::Ok(new_id)
        }
        .in_current_span(),
    );
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn release_table_db(id: u32) -> i32 {
    let mut rc = TABLE_DBS.lock();
    if rc.remove(&id).is_none() {
        return 0;
    }
    1
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn delete_table_db(port: i64, name: FfiStr) {
    let name = name.into_opt_string().unwrap_or_default();
    DartIsolateWrapper::new(port).spawn_result(
        async move {
            let veilid_api = get_veilid_api().await?;
            let tstore = veilid_api.table_store()?;
            let deleted = tstore
                .delete(&name)
                .await
                .map_err(veilid_core::VeilidAPIError::generic)?;
            APIResult::Ok(deleted)
        }
        .in_current_span(),
    );
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn table_db_get_column_count(id: u32) -> u32 {
    let table_dbs = TABLE_DBS.lock();
    let Some(table_db) = table_dbs.get(&id) else {
        return 0;
    };
    let Ok(cc) = table_db.clone().get_column_count() else {
        return 0;
    };
    cc
}

fn get_table_db(id: u32, func_name: &str) -> APIResult<veilid_core::TableDB> {
    let table_dbs = TABLE_DBS.lock();
    let Some(table_db) = table_dbs.get(&id) else {
        return APIResult::Err(veilid_core::VeilidAPIError::invalid_argument(
            func_name, "id", id,
        ));
    };
    Ok(table_db.clone())
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn table_db_get_keys(port: i64, id: u32, col: u32) {
    DartIsolateWrapper::new(port).spawn_result_json(
        async move {
            let table_db = get_table_db(id, "table_db_get_keys")?;

            let keys = table_db.get_keys(col).await?;
            let out: Vec<String> = keys
                .into_iter()
                .map(|k| BASE64URL_NOPAD.encode(&k))
                .collect();
            APIResult::Ok(out)
        }
        .in_current_span(),
    );
}

fn add_table_db_transaction(tdbt: veilid_core::TableDBTransaction) -> u32 {
    let mut next_id: u32 = 1;
    let mut tdbts = TABLE_DB_TRANSACTIONS.lock();
    while tdbts.contains_key(&next_id) {
        next_id += 1;
    }
    tdbts.insert(next_id, tdbt);
    next_id
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn table_db_transact(id: u32) -> u32 {
    let table_dbs = TABLE_DBS.lock();
    let Some(table_db) = table_dbs.get(&id) else {
        return 0;
    };
    let tdbt = table_db.clone().transact();

    add_table_db_transaction(tdbt)
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn release_table_db_transaction(id: u32) -> i32 {
    let mut tdbts = TABLE_DB_TRANSACTIONS.lock();
    if tdbts.remove(&id).is_none() {
        return 0;
    }
    1
}

fn get_table_db_transaction(
    id: u32,
    func_name: &str,
) -> APIResult<veilid_core::TableDBTransaction> {
    let tdbts = TABLE_DB_TRANSACTIONS.lock();
    let Some(tdbt) = tdbts.get(&id) else {
        return APIResult::Err(veilid_core::VeilidAPIError::invalid_argument(
            func_name, "id", id,
        ));
    };
    Ok(tdbt.clone())
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn table_db_transaction_commit(port: i64, id: u32) {
    DartIsolateWrapper::new(port).spawn_result(
        async move {
            let tdbt = get_table_db_transaction(id, "table_db_transaction_commit")?;

            tdbt.commit().await?;
            APIRESULT_VOID
        }
        .in_current_span(),
    );
}
#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn table_db_transaction_rollback(port: i64, id: u32) {
    DartIsolateWrapper::new(port).spawn_result(
        async move {
            let tdbt = get_table_db_transaction(id, "table_db_transaction_rollback")?;

            tdbt.rollback();
            APIRESULT_VOID
        }
        .in_current_span(),
    );
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn table_db_transaction_store(
    port: i64,
    id: u32,
    col: u32,
    key: FfiStr,
    value: FfiStr,
) {
    let key: Vec<u8> = data_encoding::BASE64URL_NOPAD
        .decode(key.into_opt_string().unwrap().as_bytes())
        .unwrap();
    let value: Vec<u8> = data_encoding::BASE64URL_NOPAD
        .decode(value.into_opt_string().unwrap().as_bytes())
        .unwrap();
    DartIsolateWrapper::new(port).spawn_result(
        async move {
            let tdbt = get_table_db_transaction(id, "table_db_transaction_store")?;

            tdbt.store(col, &key, &value)?;
            APIRESULT_VOID
        }
        .in_current_span(),
    );
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn table_db_transaction_delete(port: i64, id: u32, col: u32, key: FfiStr) {
    let key: Vec<u8> = data_encoding::BASE64URL_NOPAD
        .decode(key.into_opt_string().unwrap().as_bytes())
        .unwrap();
    DartIsolateWrapper::new(port).spawn_result(
        async move {
            let tdbt = get_table_db_transaction(id, "table_db_transaction_delete")?;

            tdbt.delete(col, &key)?;
            APIRESULT_VOID
        }
        .in_current_span(),
    );
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn table_db_store(port: i64, id: u32, col: u32, key: FfiStr, value: FfiStr) {
    let key: Vec<u8> = data_encoding::BASE64URL_NOPAD
        .decode(key.into_opt_string().unwrap().as_bytes())
        .unwrap();
    let value: Vec<u8> = data_encoding::BASE64URL_NOPAD
        .decode(value.into_opt_string().unwrap().as_bytes())
        .unwrap();
    DartIsolateWrapper::new(port).spawn_result(
        async move {
            let table_db = get_table_db(id, "table_db_store")?;

            table_db.store(col, &key, &value).await?;
            APIRESULT_VOID
        }
        .in_current_span(),
    );
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn table_db_load(port: i64, id: u32, col: u32, key: FfiStr) {
    let key: Vec<u8> = data_encoding::BASE64URL_NOPAD
        .decode(key.into_opt_string().unwrap().as_bytes())
        .unwrap();
    DartIsolateWrapper::new(port).spawn_result(
        async move {
            let table_db = get_table_db(id, "table_db_load")?;

            let out = table_db.load(col, &key).await?;
            let out = out.map(|x| data_encoding::BASE64URL_NOPAD.encode(&x));
            APIResult::Ok(out)
        }
        .in_current_span(),
    );
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn table_db_delete(port: i64, id: u32, col: u32, key: FfiStr) {
    let key: Vec<u8> = data_encoding::BASE64URL_NOPAD
        .decode(key.into_opt_string().unwrap().as_bytes())
        .unwrap();
    DartIsolateWrapper::new(port).spawn_result(
        async move {
            let table_db = get_table_db(id, "table_db_delete")?;

            let out = table_db.delete(col, &key).await?;
            let out = out.map(|x| data_encoding::BASE64URL_NOPAD.encode(&x));
            APIResult::Ok(out)
        }
        .in_current_span(),
    );
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn valid_crypto_kinds() -> *mut c_char {
    veilid_core::serialize_json(
        veilid_core::VALID_CRYPTO_KINDS
            .iter()
            .map(|k| (*k).into())
            .collect::<Vec<u32>>(),
    )
    .into_ffi_value()
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn best_crypto_kind() -> u32 {
    veilid_core::best_crypto_kind().into()
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn verify_signatures(port: i64, node_ids: FfiStr, data: FfiStr, signatures: FfiStr) {
    let node_ids: Vec<veilid_core::TypedPublicKey> =
        veilid_core::deserialize_opt_json(node_ids.into_opt_string()).unwrap();

    let data: Vec<u8> = data_encoding::BASE64URL_NOPAD
        .decode(data.into_opt_string().unwrap().as_bytes())
        .unwrap();

    let typed_signatures: Vec<veilid_core::TypedSignature> =
        veilid_core::deserialize_opt_json(signatures.into_opt_string()).unwrap();

    DartIsolateWrapper::new(port).spawn_result_json(
        async move {
            let veilid_api = get_veilid_api().await?;
            let crypto = veilid_api.crypto()?;
            let out = crypto.verify_signatures(&node_ids, &data, &typed_signatures)?;
            APIResult::Ok(out)
        }
        .in_current_span(),
    );
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn generate_signatures(port: i64, data: FfiStr, key_pairs: FfiStr) {
    let data: Vec<u8> = data_encoding::BASE64URL_NOPAD
        .decode(data.into_opt_string().unwrap().as_bytes())
        .unwrap();

    let key_pairs: Vec<veilid_core::TypedKeyPair> =
        veilid_core::deserialize_opt_json(key_pairs.into_opt_string()).unwrap();

    DartIsolateWrapper::new(port).spawn_result_json(
        async move {
            let veilid_api = get_veilid_api().await?;
            let crypto = veilid_api.crypto()?;
            let out = crypto.generate_signatures(&data, &key_pairs, |k, s| {
                veilid_core::TypedSignature::new(k.kind, s)
            })?;
            APIResult::Ok(out)
        }
        .in_current_span(),
    );
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn generate_key_pair(port: i64, kind: u32) {
    let kind: veilid_core::CryptoKind = veilid_core::CryptoKind::from(kind);

    DartIsolateWrapper::new(port).spawn_result_json(
        async move {
            let out = veilid_core::Crypto::generate_keypair(kind)?;
            APIResult::Ok(out)
        }
        .in_current_span(),
    );
}

#[no_mangle]
pub extern "C" fn crypto_cached_dh(port: i64, kind: u32, key: FfiStr, secret: FfiStr) {
    let kind: veilid_core::CryptoKind = veilid_core::CryptoKind::from(kind);

    let key: veilid_core::PublicKey =
        veilid_core::deserialize_opt_json(key.into_opt_string()).unwrap();
    let secret: veilid_core::SecretKey =
        veilid_core::deserialize_opt_json(secret.into_opt_string()).unwrap();

    DartIsolateWrapper::new(port).spawn_result_json(
        async move {
            let veilid_api = get_veilid_api().await?;
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
        }
        .in_current_span(),
    );
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn crypto_compute_dh(port: i64, kind: u32, key: FfiStr, secret: FfiStr) {
    let kind: veilid_core::CryptoKind = veilid_core::CryptoKind::from(kind);

    let key: veilid_core::PublicKey =
        veilid_core::deserialize_opt_json(key.into_opt_string()).unwrap();
    let secret: veilid_core::SecretKey =
        veilid_core::deserialize_opt_json(secret.into_opt_string()).unwrap();

    DartIsolateWrapper::new(port).spawn_result_json(
        async move {
            let veilid_api = get_veilid_api().await?;
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
        }
        .in_current_span(),
    );
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn crypto_generate_shared_secret(
    port: i64,
    kind: u32,
    key: FfiStr,
    secret: FfiStr,
    domain: FfiStr,
) {
    let kind: veilid_core::CryptoKind = veilid_core::CryptoKind::from(kind);

    let key: veilid_core::PublicKey =
        veilid_core::deserialize_opt_json(key.into_opt_string()).unwrap();
    let secret: veilid_core::SecretKey =
        veilid_core::deserialize_opt_json(secret.into_opt_string()).unwrap();
    let domain: Vec<u8> = data_encoding::BASE64URL_NOPAD
        .decode(domain.into_opt_string().unwrap().as_bytes())
        .unwrap();

    DartIsolateWrapper::new(port).spawn_result_json(
        async move {
            let veilid_api = get_veilid_api().await?;
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
        }
        .in_current_span(),
    );
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn crypto_random_bytes(port: i64, kind: u32, len: u32) {
    let kind: veilid_core::CryptoKind = veilid_core::CryptoKind::from(kind);

    DartIsolateWrapper::new(port).spawn_result(
        async move {
            let veilid_api = get_veilid_api().await?;
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
        }
        .in_current_span(),
    );
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn crypto_default_salt_length(port: i64, kind: u32) {
    let kind: veilid_core::CryptoKind = veilid_core::CryptoKind::from(kind);

    DartIsolateWrapper::new(port).spawn_result(
        async move {
            let veilid_api = get_veilid_api().await?;
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
        }
        .in_current_span(),
    );
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn crypto_hash_password(port: i64, kind: u32, password: FfiStr, salt: FfiStr) {
    let kind: veilid_core::CryptoKind = veilid_core::CryptoKind::from(kind);
    let password: Vec<u8> = data_encoding::BASE64URL_NOPAD
        .decode(password.into_opt_string().unwrap().as_bytes())
        .unwrap();
    let salt: Vec<u8> = data_encoding::BASE64URL_NOPAD
        .decode(salt.into_opt_string().unwrap().as_bytes())
        .unwrap();

    DartIsolateWrapper::new(port).spawn_result(
        async move {
            let veilid_api = get_veilid_api().await?;
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
        }
        .in_current_span(),
    );
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn crypto_verify_password(
    port: i64,
    kind: u32,
    password: FfiStr,
    password_hash: FfiStr,
) {
    let kind: veilid_core::CryptoKind = veilid_core::CryptoKind::from(kind);
    let password: Vec<u8> = data_encoding::BASE64URL_NOPAD
        .decode(password.into_opt_string().unwrap().as_bytes())
        .unwrap();
    let password_hash = password_hash.into_opt_string().unwrap();

    DartIsolateWrapper::new(port).spawn_result(
        async move {
            let veilid_api = get_veilid_api().await?;
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
        }
        .in_current_span(),
    );
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn crypto_derive_shared_secret(
    port: i64,
    kind: u32,
    password: FfiStr,
    salt: FfiStr,
) {
    let kind: veilid_core::CryptoKind = veilid_core::CryptoKind::from(kind);
    let password: Vec<u8> = data_encoding::BASE64URL_NOPAD
        .decode(password.into_opt_string().unwrap().as_bytes())
        .unwrap();
    let salt: Vec<u8> = data_encoding::BASE64URL_NOPAD
        .decode(salt.into_opt_string().unwrap().as_bytes())
        .unwrap();

    DartIsolateWrapper::new(port).spawn_result_json(
        async move {
            let veilid_api = get_veilid_api().await?;
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
        }
        .in_current_span(),
    );
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn crypto_random_nonce(port: i64, kind: u32) {
    let kind: veilid_core::CryptoKind = veilid_core::CryptoKind::from(kind);

    DartIsolateWrapper::new(port).spawn_result_json(
        async move {
            let veilid_api = get_veilid_api().await?;
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
        }
        .in_current_span(),
    );
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn crypto_random_shared_secret(port: i64, kind: u32) {
    let kind: veilid_core::CryptoKind = veilid_core::CryptoKind::from(kind);

    DartIsolateWrapper::new(port).spawn_result_json(
        async move {
            let veilid_api = get_veilid_api().await?;
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
        }
        .in_current_span(),
    );
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn crypto_generate_key_pair(port: i64, kind: u32) {
    let kind: veilid_core::CryptoKind = veilid_core::CryptoKind::from(kind);

    DartIsolateWrapper::new(port).spawn_result_json(
        async move {
            let veilid_api = get_veilid_api().await?;
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
        }
        .in_current_span(),
    );
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn crypto_generate_hash(port: i64, kind: u32, data: FfiStr) {
    let kind: veilid_core::CryptoKind = veilid_core::CryptoKind::from(kind);

    let data: Vec<u8> = data_encoding::BASE64URL_NOPAD
        .decode(data.into_opt_string().unwrap().as_bytes())
        .unwrap();

    DartIsolateWrapper::new(port).spawn_result_json(
        async move {
            let veilid_api = get_veilid_api().await?;
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
        }
        .in_current_span(),
    );
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn crypto_validate_key_pair(port: i64, kind: u32, key: FfiStr, secret: FfiStr) {
    let kind: veilid_core::CryptoKind = veilid_core::CryptoKind::from(kind);

    let key: veilid_core::PublicKey =
        veilid_core::deserialize_opt_json(key.into_opt_string()).unwrap();
    let secret: veilid_core::SecretKey =
        veilid_core::deserialize_opt_json(secret.into_opt_string()).unwrap();

    DartIsolateWrapper::new(port).spawn_result(
        async move {
            let veilid_api = get_veilid_api().await?;
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
        }
        .in_current_span(),
    );
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn crypto_validate_hash(port: i64, kind: u32, data: FfiStr, hash: FfiStr) {
    let kind: veilid_core::CryptoKind = veilid_core::CryptoKind::from(kind);

    let data: Vec<u8> = data_encoding::BASE64URL_NOPAD
        .decode(data.into_opt_string().unwrap().as_bytes())
        .unwrap();

    let hash: veilid_core::HashDigest =
        veilid_core::deserialize_opt_json(hash.into_opt_string()).unwrap();

    DartIsolateWrapper::new(port).spawn_result(
        async move {
            let veilid_api = get_veilid_api().await?;
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
        }
        .in_current_span(),
    );
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn crypto_distance(port: i64, kind: u32, key1: FfiStr, key2: FfiStr) {
    let kind: veilid_core::CryptoKind = veilid_core::CryptoKind::from(kind);

    let key1: veilid_core::HashDigest =
        veilid_core::deserialize_opt_json(key1.into_opt_string()).unwrap();
    let key2: veilid_core::HashDigest =
        veilid_core::deserialize_opt_json(key2.into_opt_string()).unwrap();

    DartIsolateWrapper::new(port).spawn_result_json(
        async move {
            let veilid_api = get_veilid_api().await?;
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
        }
        .in_current_span(),
    );
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn crypto_sign(port: i64, kind: u32, key: FfiStr, secret: FfiStr, data: FfiStr) {
    let kind: veilid_core::CryptoKind = veilid_core::CryptoKind::from(kind);

    let key: veilid_core::PublicKey =
        veilid_core::deserialize_opt_json(key.into_opt_string()).unwrap();
    let secret: veilid_core::SecretKey =
        veilid_core::deserialize_opt_json(secret.into_opt_string()).unwrap();
    let data: Vec<u8> = data_encoding::BASE64URL_NOPAD
        .decode(data.into_opt_string().unwrap().as_bytes())
        .unwrap();

    DartIsolateWrapper::new(port).spawn_result_json(
        async move {
            let veilid_api = get_veilid_api().await?;
            let crypto = veilid_api.crypto()?;
            let csv = crypto.get(kind).ok_or_else(|| {
                veilid_core::VeilidAPIError::invalid_argument(
                    "crypto_sign",
                    "kind",
                    kind.to_string(),
                )
            })?;
            let out = csv.sign(&key, &secret, &data)?;
            APIResult::Ok(out)
        }
        .in_current_span(),
    );
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn crypto_verify(
    port: i64,
    kind: u32,
    key: FfiStr,
    data: FfiStr,
    signature: FfiStr,
) {
    let kind: veilid_core::CryptoKind = veilid_core::CryptoKind::from(kind);

    let key: veilid_core::PublicKey =
        veilid_core::deserialize_opt_json(key.into_opt_string()).unwrap();
    let data: Vec<u8> = data_encoding::BASE64URL_NOPAD
        .decode(data.into_opt_string().unwrap().as_bytes())
        .unwrap();
    let signature: veilid_core::Signature =
        veilid_core::deserialize_opt_json(signature.into_opt_string()).unwrap();

    DartIsolateWrapper::new(port).spawn_result(
        async move {
            let veilid_api = get_veilid_api().await?;
            let crypto = veilid_api.crypto()?;
            let csv = crypto.get(kind).ok_or_else(|| {
                veilid_core::VeilidAPIError::invalid_argument(
                    "crypto_verify",
                    "kind",
                    kind.to_string(),
                )
            })?;
            let out = csv.verify(&key, &data, &signature)?;
            APIResult::Ok(out)
        }
        .in_current_span(),
    );
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn crypto_aead_overhead(port: i64, kind: u32) {
    let kind: veilid_core::CryptoKind = veilid_core::CryptoKind::from(kind);

    DartIsolateWrapper::new(port).spawn_result(
        async move {
            let veilid_api = get_veilid_api().await?;
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
        }
        .in_current_span(),
    );
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn crypto_decrypt_aead(
    port: i64,
    kind: u32,
    body: FfiStr,
    nonce: FfiStr,
    shared_secret: FfiStr,
    associated_data: FfiStr,
) {
    let kind: veilid_core::CryptoKind = veilid_core::CryptoKind::from(kind);

    let body: Vec<u8> = data_encoding::BASE64URL_NOPAD
        .decode(body.into_opt_string().unwrap().as_bytes())
        .unwrap();

    let nonce: veilid_core::Nonce =
        veilid_core::deserialize_opt_json(nonce.into_opt_string()).unwrap();

    let shared_secret: veilid_core::SharedSecret =
        veilid_core::deserialize_opt_json(shared_secret.into_opt_string()).unwrap();

    let associated_data: Option<Vec<u8>> = associated_data
        .into_opt_string()
        .map(|s| data_encoding::BASE64URL_NOPAD.decode(s.as_bytes()).unwrap());

    DartIsolateWrapper::new(port).spawn_result(
        async move {
            let veilid_api = get_veilid_api().await?;
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
        }
        .in_current_span(),
    );
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn crypto_encrypt_aead(
    port: i64,
    kind: u32,
    body: FfiStr,
    nonce: FfiStr,
    shared_secret: FfiStr,
    associated_data: FfiStr,
) {
    let kind: veilid_core::CryptoKind = veilid_core::CryptoKind::from(kind);

    let body: Vec<u8> = data_encoding::BASE64URL_NOPAD
        .decode(body.into_opt_string().unwrap().as_bytes())
        .unwrap();

    let nonce: veilid_core::Nonce =
        veilid_core::deserialize_opt_json(nonce.into_opt_string()).unwrap();

    let shared_secret: veilid_core::SharedSecret =
        veilid_core::deserialize_opt_json(shared_secret.into_opt_string()).unwrap();

    let associated_data: Option<Vec<u8>> = associated_data
        .into_opt_string()
        .map(|s| data_encoding::BASE64URL_NOPAD.decode(s.as_bytes()).unwrap());

    DartIsolateWrapper::new(port).spawn_result(
        async move {
            let veilid_api = get_veilid_api().await?;
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
        }
        .in_current_span(),
    );
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn crypto_crypt_no_auth(
    port: i64,
    kind: u32,
    body: FfiStr,
    nonce: FfiStr,
    shared_secret: FfiStr,
) {
    let kind: veilid_core::CryptoKind = veilid_core::CryptoKind::from(kind);

    let mut body: Vec<u8> = data_encoding::BASE64URL_NOPAD
        .decode(body.into_opt_string().unwrap().as_bytes())
        .unwrap();

    let nonce: veilid_core::Nonce =
        veilid_core::deserialize_opt_json(nonce.into_opt_string()).unwrap();

    let shared_secret: veilid_core::SharedSecret =
        veilid_core::deserialize_opt_json(shared_secret.into_opt_string()).unwrap();

    DartIsolateWrapper::new(port).spawn_result(
        async move {
            let veilid_api = get_veilid_api().await?;
            let crypto = veilid_api.crypto()?;
            let csv = crypto.get(kind).ok_or_else(|| {
                veilid_core::VeilidAPIError::invalid_argument(
                    "crypto_crypt_no_auth",
                    "kind",
                    kind.to_string(),
                )
            })?;
            csv.crypt_in_place_no_auth(&mut body, &nonce, &shared_secret);
            let body = data_encoding::BASE64URL_NOPAD.encode(&body);
            APIResult::Ok(body)
        }
        .in_current_span(),
    );
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn now() -> u64 {
    veilid_core::Timestamp::now().as_u64()
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn debug(port: i64, command: FfiStr) {
    let command = command.into_opt_string().unwrap_or_default();
    DartIsolateWrapper::new(port).spawn_result(
        async move {
            let veilid_api = get_veilid_api().await?;
            let out = veilid_api.debug(command).await?;
            APIResult::Ok(out)
        }
        .in_current_span(),
    );
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
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
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn veilid_version() -> VeilidVersion {
    let (major, minor, patch) = veilid_core::veilid_version();
    VeilidVersion {
        major,
        minor,
        patch,
    }
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn default_veilid_config() -> *mut c_char {
    veilid_core::default_veilid_config().into_ffi_value()
}

#[no_mangle]
#[instrument(level = "trace", target = "ffi", skip_all)]
pub extern "C" fn veilid_features() -> *mut c_char {
    serde_json::to_string(&veilid_core::veilid_features())
        .expect("Failed to serialize features")
        .into_ffi_value()
}
