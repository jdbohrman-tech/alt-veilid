use crate::settings::*;
use crate::*;
use cfg_if::*;
#[cfg(feature = "rt-tokio")]
use console_subscriber::ConsoleLayer;
use opentelemetry::sdk::*;
use opentelemetry::*;
use opentelemetry_otlp::WithExportConfig;
use parking_lot::*;
use std::collections::BTreeMap;
use std::path::*;
use std::sync::Arc;
use tracing_appender::*;
use tracing_subscriber::prelude::*;
use tracing_subscriber::*;

struct VeilidLogsInner {
    _guard: Option<non_blocking::WorkerGuard>,
    filters: BTreeMap<&'static str, veilid_core::VeilidLayerFilter>,
}

#[derive(Clone)]
pub struct VeilidLogs {
    inner: Arc<Mutex<VeilidLogsInner>>,
}

impl VeilidLogs {
    pub fn setup(settings: Settings) -> EyreResult<VeilidLogs> {
        let settingsr = settings.read();

        // Set up subscriber and layers
        let subscriber = Registry::default();
        let mut layers = Vec::new();
        let mut filters = BTreeMap::new();

        // Error layer
        // XXX: Spantrace capture causes rwlock deadlocks/crashes
        // XXX:
        //layers.push(tracing_error::ErrorLayer::default().boxed());

        #[cfg(feature = "rt-tokio")]
        if settingsr.logging.console.enabled {
            let layer = ConsoleLayer::builder()
                .with_default_env()
                .spawn()
                .with_filter(
                    filter::Targets::new()
                        .with_target("tokio", Level::TRACE)
                        .with_target("runtime", Level::TRACE),
                );
            layers.push(layer.boxed());
        }

        // Terminal logger
        if settingsr.logging.terminal.enabled {
            let filter = veilid_core::VeilidLayerFilter::new(
                convert_loglevel(settingsr.logging.terminal.level),
                None,
            );
            let layer = fmt::Layer::new()
                .compact()
                .with_writer(std::io::stdout)
                .with_filter(filter.clone());
            filters.insert("terminal", filter);
            layers.push(layer.boxed());
        }

        // OpenTelemetry logger
        if settingsr.logging.otlp.enabled {
            let grpc_endpoint = settingsr.logging.otlp.grpc_endpoint.name.clone();

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
                    compile_error!("needs executor implementation")
                }
            }

            let tracer = opentelemetry_otlp::new_pipeline()
                .tracing()
                .with_exporter(exporter)
                .with_trace_config(opentelemetry::sdk::trace::config().with_resource(
                    Resource::new(vec![KeyValue::new(
                        opentelemetry_semantic_conventions::resource::SERVICE_NAME,
                        format!(
                                "veilid_server:{}",
                                hostname::get()
                                    .map(|s| s.to_string_lossy().into_owned())
                                    .unwrap_or_else(|_| "unknown".to_owned())
                            ),
                    )]),
                ))
                .install_batch(batch)
                .wrap_err("failed to install OpenTelemetry tracer")?;

            let filter = veilid_core::VeilidLayerFilter::new(
                convert_loglevel(settingsr.logging.otlp.level),
                None,
            );
            let layer = tracing_opentelemetry::layer()
                .with_tracer(tracer)
                .with_filter(filter.clone());
            filters.insert("otlp", filter);
            layers.push(layer.boxed());
        }

        // File logger
        let mut guard = None;
        if settingsr.logging.file.enabled {
            let log_path = Path::new(&settingsr.logging.file.path);
            let full_path = std::env::current_dir()
                .unwrap_or(PathBuf::from(MAIN_SEPARATOR.to_string()))
                .join(log_path);
            let log_parent = full_path
                .parent()
                .unwrap_or(Path::new(&MAIN_SEPARATOR.to_string()))
                .canonicalize()
                .wrap_err(format!(
                    "File log path parent does not exist: {}",
                    settingsr.logging.file.path
                ))?;
            let log_filename = full_path.file_name().ok_or(eyre!(
                "File log filename not specified in path: {}",
                settingsr.logging.file.path
            ))?;

            let appender = tracing_appender::rolling::never(log_parent, Path::new(log_filename));
            let (non_blocking_appender, non_blocking_guard) =
                tracing_appender::non_blocking(appender);
            guard = Some(non_blocking_guard);

            let filter = veilid_core::VeilidLayerFilter::new(
                convert_loglevel(settingsr.logging.file.level),
                None,
            );
            let layer = fmt::Layer::new()
                .compact()
                .with_writer(non_blocking_appender)
                .with_filter(filter.clone());

            filters.insert("file", filter);
            layers.push(layer.boxed());
        }

        // API logger
        if settingsr.logging.api.enabled {
            let filter = veilid_core::VeilidLayerFilter::new(
                convert_loglevel(settingsr.logging.api.level),
                None,
            );
            let layer = veilid_core::ApiTracingLayer::get().with_filter(filter.clone());
            filters.insert("api", filter);
            layers.push(layer.boxed());
        }

        // Systemd Journal logger
        cfg_if! {
            if #[cfg(target_os = "linux")] {
                if settingsr.logging.system.enabled {
                    let filter = veilid_core::VeilidLayerFilter::new(
                        convert_loglevel(settingsr.logging.system.level),
                        None,
                    );
                    let layer = tracing_journald::layer().wrap_err("failed to set up journald logging")?
                        .with_filter(filter.clone());
                    filters.insert("system", filter);
                    layers.push(layer.boxed());
                }
            }
        }

        let subscriber = subscriber.with(layers);
        subscriber
            .try_init()
            .wrap_err("failed to initialize logging")?;

        Ok(VeilidLogs {
            inner: Arc::new(Mutex::new(VeilidLogsInner {
                _guard: guard,
                filters,
            })),
        })
    }

    pub fn change_log_level(
        &self,
        layer: String,
        log_level: veilid_core::VeilidConfigLogLevel,
    ) -> Result<(), veilid_core::VeilidAPIError> {
        // get layer to change level on
        let layer = if layer == "all" { "".to_owned() } else { layer };

        // change log level on appropriate layer
        let inner = self.inner.lock();
        if layer.is_empty() {
            // Change all layers
            for f in inner.filters.values() {
                f.set_max_level(log_level);
            }
        } else {
            // Change a specific layer
            let f = match inner.filters.get(layer.as_str()) {
                Some(f) => f,
                None => {
                    return Err(veilid_core::VeilidAPIError::InvalidArgument {
                        context: "change_log_level".to_owned(),
                        argument: "layer".to_owned(),
                        value: layer,
                    });
                }
            };
            f.set_max_level(log_level);
        }
        Ok(())
    }
}
