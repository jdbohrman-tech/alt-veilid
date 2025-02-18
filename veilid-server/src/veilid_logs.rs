use crate::settings::*;
use crate::*;
use cfg_if::*;
#[cfg(feature = "rt-tokio")]
use console_subscriber::ConsoleLayer;

cfg_if::cfg_if! {
    if #[cfg(feature = "opentelemetry-otlp")] {
        use opentelemetry_sdk::*;
        use opentelemetry_otlp::WithExportConfig;
    }
}

use parking_lot::*;
use std::collections::BTreeMap;
use std::path::*;
use std::sync::Arc;
use tracing_appender::*;
#[cfg(feature = "flame")]
use tracing_flame::FlameLayer;
#[cfg(all(unix, feature = "perfetto"))]
use tracing_perfetto::PerfettoLayer;
use tracing_subscriber::prelude::*;
use tracing_subscriber::*;
use veilid_core::VEILID_LOG_KEY_FIELD;

struct VeilidLogsInner {
    _file_guard: Option<non_blocking::WorkerGuard>,
    #[cfg(feature = "flame")]
    _flame_guard: Option<tracing_flame::FlushGuard<std::io::BufWriter<std::fs::File>>>,
    filters: BTreeMap<&'static str, veilid_core::VeilidLayerFilter>,
}

#[derive(Clone)]
pub struct VeilidLogs {
    inner: Arc<Mutex<VeilidLogsInner>>,
}

fn make_primary_log_key_filter(
    subnode_index: u16,
    empty_namespace_enabled: bool,
) -> veilid_core::VeilidLayerLogKeyFilter {
    let namespace = subnode_namespace(subnode_index);
    let log_key = veilid_core::VeilidLayerFilter::make_veilid_log_key(PROGRAM_NAME, &namespace);
    Arc::new(move |ns| {
        if ns.is_empty() {
            return empty_namespace_enabled;
        }
        ns == log_key
    })
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

        let mut fields_to_strip = HashSet::<&'static str>::new();
        fields_to_strip.insert(VEILID_LOG_KEY_FIELD);

        #[cfg(feature = "rt-tokio")]
        if settingsr.logging.console.enabled {
            let filter = veilid_core::VeilidLayerFilter::new_no_default(
                veilid_core::VeilidConfigLogLevel::Trace,
                &[],
                None,
            );

            let layer = ConsoleLayer::builder()
                .with_default_env()
                .spawn()
                .with_filter(filter);
            layers.push(layer.boxed());
        }

        // Terminal logger
        if settingsr.logging.terminal.enabled {
            let timer = time::format_description::parse("[hour]:[minute]:[second]")
                .expect("invalid time format");

            // Get time offset for local timezone from UTC
            // let time_offset =
            //     time::UtcOffset::current_local_offset().unwrap_or(time::UtcOffset::UTC);
            // nerd fight: https://www.reddit.com/r/learnrust/comments/1bgc4p7/time_crate_never_manages_to_get_local_time/
            // Use chrono instead of time crate to get local offset
            let offset_in_sec = chrono::Local::now().offset().local_minus_utc();
            let time_offset =
                time::UtcOffset::from_whole_seconds(offset_in_sec).expect("invalid utc offset");
            let timer = fmt::time::OffsetTime::new(time_offset, timer);

            let filter = veilid_core::VeilidLayerFilter::new(
                convert_loglevel(settingsr.logging.terminal.level),
                &settingsr.logging.terminal.ignore_log_targets,
                Some(make_primary_log_key_filter(
                    settingsr.testing.subnode_index,
                    true,
                )),
            );
            let layer = fmt::Layer::new()
                .compact()
                .map_fmt_fields(|f| veilid_core::FmtStripFields::new(f, fields_to_strip.clone()))
                .with_timer(timer)
                .with_ansi(true)
                .with_writer(std::io::stdout)
                .with_filter(filter.clone());

            filters.insert("terminal", filter);
            layers.push(layer.boxed());
        }

        // Flamegraph logger
        #[cfg(feature = "flame")]
        let mut flame_guard = None;
        #[cfg(feature = "flame")]
        if settingsr.logging.flame.enabled {
            let filter = veilid_core::VeilidLayerFilter::new_no_default(
                veilid_core::VeilidConfigLogLevel::Trace,
                &veilid_core::FLAME_LOG_FACILITIES_IGNORE_LIST
                    .iter()
                    .map(|&x| x.to_string())
                    .collect::<Vec<_>>(),
                None,
            );
            let (flame_layer, guard) = FlameLayer::with_file(&settingsr.logging.flame.path)?;
            flame_guard = Some(guard);
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

        // Perfetto logger
        #[cfg(all(unix, feature = "perfetto"))]
        if settingsr.logging.perfetto.enabled {
            let filter = veilid_core::VeilidLayerFilter::new_no_default(
                veilid_core::VeilidConfigLogLevel::Trace,
                &veilid_core::FLAME_LOG_FACILITIES_IGNORE_LIST
                    .iter()
                    .map(|&x| x.to_string())
                    .collect::<Vec<_>>(),
                None,
            );
            let perfetto_layer = PerfettoLayer::new(std::sync::Mutex::new(std::fs::File::create(
                &settingsr.logging.perfetto.path,
            )?));

            // Do not include this in change_log_level changes, so we keep trace level
            // filters.insert("flame", filter.clone());
            layers.push(
                perfetto_layer
                    .with_debug_annotations(true)
                    .with_filter(filter)
                    .boxed(),
            );
        }

        // OpenTelemetry logger
        #[cfg(feature = "opentelemetry-otlp")]
        if settingsr.logging.otlp.enabled {
            let grpc_endpoint = settingsr.logging.otlp.grpc_endpoint.name.clone();

            cfg_if! {
                if #[cfg(feature="rt-async-std")] {
                    let exporter = opentelemetry_otlp::new_exporter()
                        .grpcio()
                        .with_endpoint(grpc_endpoint);
                    let batch = opentelemetry_sdk::runtime::AsyncStd;
                } else if #[cfg(feature="rt-tokio")] {
                    let exporter = opentelemetry_otlp::new_exporter()
                        .tonic()
                        .with_endpoint(format!("http://{}", grpc_endpoint));
                    let batch = opentelemetry_sdk::runtime::Tokio;
                } else {
                    compile_error!("needs executor implementation");
                }
            }

            let tracer = opentelemetry_otlp::new_pipeline()
                .tracing()
                .with_exporter(exporter)
                .with_trace_config(opentelemetry_sdk::trace::Config::default().with_resource(
                    Resource::new(vec![opentelemetry::KeyValue::new(
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
                &settingsr.logging.otlp.ignore_log_targets,
                None,
            );
            let layer = tracing_opentelemetry::layer()
                .with_tracer(tracer)
                .with_filter(filter.clone());
            filters.insert("otlp", filter);
            layers.push(layer.boxed());
        }

        // File logger
        let mut file_guard = None;
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
            file_guard = Some(non_blocking_guard);

            let filter = veilid_core::VeilidLayerFilter::new(
                convert_loglevel(settingsr.logging.file.level),
                &settingsr.logging.file.ignore_log_targets,
                Some(make_primary_log_key_filter(
                    settingsr.testing.subnode_index,
                    true,
                )),
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
                &settingsr.logging.api.ignore_log_targets,
                None,
            );
            let layer = veilid_core::ApiTracingLayer::init().with_filter(filter.clone());
            filters.insert("api", filter);
            layers.push(layer.boxed());
        }

        // Systemd Journal logger
        cfg_if! {
            if #[cfg(target_os = "linux")] {
                if settingsr.logging.system.enabled {
                    let filter = veilid_core::VeilidLayerFilter::new(
                        convert_loglevel(settingsr.logging.system.level),
                        &settingsr.logging.system.ignore_log_targets,
                        Some(make_primary_log_key_filter(
                            settingsr.testing.subnode_index,
                            true,
                        )),
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
                _file_guard: file_guard,
                #[cfg(feature = "flame")]
                _flame_guard: flame_guard,
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

    pub fn change_log_ignore(
        &self,
        layer: String,
        log_ignore: String,
    ) -> Result<(), veilid_core::VeilidAPIError> {
        // get layer to change level on
        let layer = if layer == "all" { "".to_owned() } else { layer };

        // change log level on appropriate layer
        let inner = self.inner.lock();
        if layer.is_empty() {
            // Change all layers
            for f in inner.filters.values() {
                f.set_ignore_list(Some(veilid_core::VeilidLayerFilter::apply_ignore_change(
                    &f.ignore_list(),
                    log_ignore.clone(),
                )));
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
            f.set_ignore_list(Some(veilid_core::VeilidLayerFilter::apply_ignore_change(
                &f.ignore_list(),
                log_ignore.clone(),
            )));
        }
        Ok(())
    }
}
