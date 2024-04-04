use crate::core_context::*;
use crate::veilid_api::*;
use crate::*;
use core::fmt::Write;
use once_cell::sync::OnceCell;
use tracing_subscriber::*;

struct ApiTracingLayerInner {
    update_callback: UpdateCallback,
}

#[derive(Clone)]
pub struct ApiTracingLayer {
    inner: Arc<Mutex<Option<ApiTracingLayerInner>>>,
}

static API_LOGGER: OnceCell<ApiTracingLayer> = OnceCell::new();

impl ApiTracingLayer {
    fn new_inner(update_callback: UpdateCallback) -> ApiTracingLayerInner {
        ApiTracingLayerInner { update_callback }
    }

    #[instrument(level = "debug", skip(update_callback))]
    pub async fn init(update_callback: UpdateCallback) {
        let api_logger = API_LOGGER.get_or_init(|| ApiTracingLayer {
            inner: Arc::new(Mutex::new(None)),
        });
        let apilogger_inner = Some(Self::new_inner(update_callback));
        *api_logger.inner.lock() = apilogger_inner;
    }

    #[instrument(level = "debug")]
    pub async fn terminate() {
        if let Some(api_logger) = API_LOGGER.get() {
            let mut inner = api_logger.inner.lock();
            *inner = None;
        }
    }

    pub fn get() -> ApiTracingLayer {
        API_LOGGER
            .get_or_init(|| ApiTracingLayer {
                inner: Arc::new(Mutex::new(None)),
            })
            .clone()
    }

    fn emit_log(&self, inner: &mut ApiTracingLayerInner, meta: &Metadata<'_>, message: String) {
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
                                format!("[{}]", target)
                            },
                            simplify_file(file),
                            ln
                        )
                    })
                })
                .unwrap_or_default(),
        };

        let message = format!("{}{}", origin, message).trim().to_owned();

        let backtrace = if log_level <= VeilidLogLevel::Error {
            let bt = backtrace::Backtrace::new();
            Some(format!("{:?}", bt))
        } else {
            None
        };

        (inner.update_callback)(VeilidUpdate::Log(Box::new(VeilidLog {
            log_level,
            message,
            backtrace,
        })))
    }
}

pub struct SpanDuration {
    start: Timestamp,
    end: Timestamp,
}

fn simplify_file(file: &str) -> String {
    let path = std::path::Path::new(file);
    let path_component_count = path.iter().count();
    if path.ends_with("mod.rs") && path_component_count >= 2 {
        let outpath: std::path::PathBuf = path.iter().skip(path_component_count - 2).collect();
        outpath.to_string_lossy().to_string()
    } else if let Some(filename) = path.file_name() {
        filename.to_string_lossy().to_string()
    } else {
        file.to_string()
    }
}

impl<S: Subscriber + for<'a> registry::LookupSpan<'a>> Layer<S> for ApiTracingLayer {
    fn on_new_span(
        &self,
        attrs: &tracing::span::Attributes<'_>,
        id: &tracing::Id,
        ctx: layer::Context<'_, S>,
    ) {
        if let Some(_inner) = &mut *self.inner.lock() {
            let mut new_debug_record = StringRecorder::new();
            attrs.record(&mut new_debug_record);

            if let Some(span_ref) = ctx.span(id) {
                span_ref
                    .extensions_mut()
                    .insert::<StringRecorder>(new_debug_record);
                if crate::DURATION_LOG_FACILITIES.contains(&attrs.metadata().target()) {
                    span_ref
                        .extensions_mut()
                        .insert::<SpanDuration>(SpanDuration {
                            start: get_aligned_timestamp(),
                            end: Timestamp::default(),
                        });
                }
            }
        }
    }

    fn on_close(&self, id: span::Id, ctx: layer::Context<'_, S>) {
        if let Some(inner) = &mut *self.inner.lock() {
            if let Some(span_ref) = ctx.span(&id) {
                if let Some(span_duration) = span_ref.extensions_mut().get_mut::<SpanDuration>() {
                    span_duration.end = get_aligned_timestamp();
                    let duration = span_duration.end.saturating_sub(span_duration.start);
                    let meta = span_ref.metadata();
                    self.emit_log(
                        inner,
                        meta,
                        format!(
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
    }

    fn on_record(
        &self,
        id: &tracing::Id,
        values: &tracing::span::Record<'_>,
        ctx: layer::Context<'_, S>,
    ) {
        if let Some(_inner) = &mut *self.inner.lock() {
            if let Some(span_ref) = ctx.span(id) {
                if let Some(debug_record) = span_ref.extensions_mut().get_mut::<StringRecorder>() {
                    values.record(debug_record);
                }
            }
        }
    }

    fn on_event(&self, event: &tracing::Event<'_>, _ctx: layer::Context<'_, S>) {
        if let Some(inner) = &mut *self.inner.lock() {
            let mut recorder = StringRecorder::new();
            event.record(&mut recorder);
            let meta = event.metadata();
            self.emit_log(inner, meta, recorder.to_string());
        }
    }
}

struct StringRecorder {
    display: String,
    //is_following_args: bool,
}
impl StringRecorder {
    fn new() -> Self {
        StringRecorder {
            display: String::new(),
            //      is_following_args: false,
        }
    }
}

impl tracing::field::Visit for StringRecorder {
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

impl core::fmt::Display for StringRecorder {
    fn fmt(&self, mut f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if !self.display.is_empty() {
            write!(&mut f, " {}", self.display)
        } else {
            Ok(())
        }
    }
}

impl core::default::Default for StringRecorder {
    fn default() -> Self {
        StringRecorder::new()
    }
}
