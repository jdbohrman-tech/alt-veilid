use crate::core_context::*;
use crate::veilid_api::*;
use crate::*;
use core::fmt::Write;
use tracing_subscriber::*;

struct ApiTracingLayerInner {
    update_callbacks: HashMap<String, UpdateCallback>,
}

/// API Tracing layer for 'tracing' subscribers
///
/// For normal application use one should call ApiTracingLayer::init() and insert the
/// layer into your subscriber before calling api_startup() or api_startup_json().
///
/// For apps that call api_startup() many times concurrently with different 'namespace' or
/// 'program_name', you may want to disable api tracing as it can slow the system down
/// considerably. In those cases, deferring to buffered disk-based logging files is probably a better idea.
/// At the very least, no more verbose than info-level logging is recommended when using API tracing
/// with many copies of Veilid running.

#[derive(Clone)]
#[must_use]
pub struct ApiTracingLayer {}

static API_LOGGER_INNER: Mutex<Option<ApiTracingLayerInner>> = Mutex::new(None);
static API_LOGGER_ENABLED: AtomicBool = AtomicBool::new(false);

impl ApiTracingLayer {
    /// Initialize an ApiTracingLayer singleton
    ///
    /// This must be inserted into your tracing subscriber before you
    /// call api_startup() or api_startup_json() if you are going to use api tracing.
    pub fn init() -> ApiTracingLayer {
        ApiTracingLayer {}
    }

    fn new_inner() -> ApiTracingLayerInner {
        ApiTracingLayerInner {
            update_callbacks: HashMap::new(),
        }
    }

    #[instrument(level = "debug", skip(update_callback))]
    pub(crate) async fn add_callback(
        log_key: String,
        update_callback: UpdateCallback,
    ) -> VeilidAPIResult<()> {
        let mut inner = API_LOGGER_INNER.lock();
        if inner.is_none() {
            *inner = Some(Self::new_inner());
        }
        if inner
            .as_ref()
            .unwrap()
            .update_callbacks
            .contains_key(&log_key)
        {
            apibail_already_initialized!();
        }
        inner
            .as_mut()
            .unwrap()
            .update_callbacks
            .insert(log_key, update_callback);

        API_LOGGER_ENABLED.store(true, Ordering::Release);

        return Ok(());
    }

    #[instrument(level = "debug")]
    pub(crate) async fn remove_callback(log_key: String) -> VeilidAPIResult<()> {
        let mut inner = API_LOGGER_INNER.lock();
        if inner.is_none() {
            apibail_not_initialized!();
        }
        if inner
            .as_mut()
            .unwrap()
            .update_callbacks
            .remove(&log_key)
            .is_none()
        {
            apibail_not_initialized!();
        }
        if inner.as_mut().unwrap().update_callbacks.is_empty() {
            *inner = None;
            API_LOGGER_ENABLED.store(false, Ordering::Release);
        }

        Ok(())
    }

    fn emit_log(&self, meta: &'static Metadata<'static>, log_key: &str, message: &str) {
        let opt_update_cb = if let Some(inner) = &mut *API_LOGGER_INNER.lock() {
            inner.update_callbacks.get(log_key).cloned()
        } else {
            None
        };
        let Some(update_cb) = opt_update_cb else {
            return;
        };

        let level = *meta.level();
        let target = meta.target();
        let log_level = VeilidLogLevel::from_tracing_level(level);

        let origin = match level {
            Level::ERROR | Level::WARN => meta
                .file()
                .and_then(|file| {
                    meta.line()
                        .map(|ln| format!("{}:{}", simplify_file(file), ln))
                })
                .unwrap_or_default(),
            Level::INFO => "".to_owned(),
            Level::DEBUG | Level::TRACE => meta
                .file()
                .and_then(|file| {
                    meta.line().map(|ln| {
                        format!(
                            "{}{}:{}",
                            if target.is_empty() {
                                "".to_owned()
                            } else {
                                format!("[{}] ", target)
                            },
                            simplify_file(file),
                            ln
                        )
                    })
                })
                .unwrap_or_default(),
        };

        let message = format!("{} {}", origin, message).trim().to_owned();

        let backtrace = if log_level <= VeilidLogLevel::Error {
            let bt = backtrace::Backtrace::new();
            Some(format!("{:?}", bt))
        } else {
            None
        };

        let log_update = VeilidUpdate::Log(Box::new(VeilidLog {
            log_level,
            message,
            backtrace,
        }));

        (update_cb)(log_update.clone());
    }
}

pub struct SpanDuration {
    start: Timestamp,
    end: Timestamp,
}

fn simplify_file(file: &'static str) -> &'static str {
    file.static_transform(|file| {
        let out = {
            let path = std::path::Path::new(file);
            let path_component_count = path.iter().count();
            if path.ends_with("mod.rs") && path_component_count >= 2 {
                let outpath: std::path::PathBuf =
                    path.iter().skip(path_component_count - 2).collect();
                outpath.to_string_lossy().to_string()
            } else if let Some(filename) = path.file_name() {
                filename.to_string_lossy().to_string()
            } else {
                file.to_string()
            }
        };
        out.to_static_str()
    })
}

impl<S: Subscriber + for<'a> registry::LookupSpan<'a>> Layer<S> for ApiTracingLayer {
    fn on_new_span(
        &self,
        attrs: &tracing::span::Attributes<'_>,
        id: &tracing::Id,
        ctx: layer::Context<'_, S>,
    ) {
        if !API_LOGGER_ENABLED.load(Ordering::Acquire) {
            // Optimization if api logger has no callbacks
            return;
        }

        let mut new_debug_record = VeilidKeyedStringRecorder::new();
        attrs.record(&mut new_debug_record);

        if let Some(span_ref) = ctx.span(id) {
            span_ref
                .extensions_mut()
                .insert::<VeilidKeyedStringRecorder>(new_debug_record);
            if crate::DURATION_LOG_FACILITIES.contains(&attrs.metadata().target()) {
                span_ref
                    .extensions_mut()
                    .insert::<SpanDuration>(SpanDuration {
                        start: Timestamp::now(),
                        end: Timestamp::default(),
                    });
            }
        }
    }

    fn on_close(&self, id: span::Id, ctx: layer::Context<'_, S>) {
        if !API_LOGGER_ENABLED.load(Ordering::Acquire) {
            // Optimization if api logger has no callbacks
            return;
        }
        if let Some(span_ref) = ctx.span(&id) {
            if let Some(span_duration) = span_ref.extensions_mut().get_mut::<SpanDuration>() {
                span_duration.end = Timestamp::now();
                let duration = span_duration.end.saturating_sub(span_duration.start);
                let meta = span_ref.metadata();

                let mut extensions = span_ref.extensions_mut();
                let log_key =
                    if let Some(span_ksr) = extensions.get_mut::<VeilidKeyedStringRecorder>() {
                        span_ksr.log_key()
                    } else {
                        ""
                    };

                self.emit_log(
                    meta,
                    log_key,
                    &format!(
                        " {}{}: duration={}",
                        span_ref
                            .parent()
                            .map(|p| format!("{}::", p.name()))
                            .unwrap_or_default(),
                        span_ref.name(),
                        format_opt_ts(Some(duration))
                    ),
                );
            }
        }
    }

    fn on_record(
        &self,
        id: &tracing::Id,
        values: &tracing::span::Record<'_>,
        ctx: layer::Context<'_, S>,
    ) {
        if !API_LOGGER_ENABLED.load(Ordering::Acquire) {
            // Optimization if api logger has no callbacks
            return;
        }
        if let Some(span_ref) = ctx.span(id) {
            if let Some(debug_record) = span_ref
                .extensions_mut()
                .get_mut::<VeilidKeyedStringRecorder>()
            {
                values.record(debug_record);
            }
        }
    }

    fn on_event(&self, event: &tracing::Event<'_>, _ctx: layer::Context<'_, S>) {
        if !API_LOGGER_ENABLED.load(Ordering::Acquire) {
            // Optimization if api logger has no callbacks
            return;
        }
        let mut recorder = VeilidKeyedStringRecorder::new();
        event.record(&mut recorder);
        let meta = event.metadata();
        self.emit_log(meta, recorder.log_key(), recorder.display());
    }
}

struct VeilidKeyedStringRecorder {
    log_key: String,
    display: String,
}
impl VeilidKeyedStringRecorder {
    fn new() -> Self {
        VeilidKeyedStringRecorder {
            log_key: String::new(),
            display: String::new(),
        }
    }
    fn display(&self) -> &str {
        &self.display
    }
    fn log_key(&self) -> &str {
        &self.log_key
    }
}

impl tracing::field::Visit for VeilidKeyedStringRecorder {
    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        if field.name() == VEILID_LOG_KEY_FIELD {
            self.log_key = value.to_owned();
        }
    }
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn core::fmt::Debug) {
        if field.name() == "message" {
            if !self.display.is_empty() {
                self.display = format!("{:?}\n{}", value, self.display)
            } else {
                self.display = format!("{:?}", value)
            }
        } else {
            write!(self.display, " ").unwrap();
            write!(self.display, "{} = {:?};", field.name(), value).unwrap();
        }
    }
}
