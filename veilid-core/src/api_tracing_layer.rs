use crate::core_context::*;
use crate::veilid_api::*;
use crate::*;
use core::fmt::Write;
use once_cell::sync::OnceCell;
use tracing_subscriber::*;

struct ApiLoggerInner {
    update_callback: UpdateCallback,
}

#[derive(Clone)]
pub struct ApiTracingLayer {
    inner: Arc<Mutex<Option<ApiLoggerInner>>>,
}

static API_LOGGER: OnceCell<ApiTracingLayer> = OnceCell::new();

impl ApiTracingLayer {
    fn new_inner(update_callback: UpdateCallback) -> ApiLoggerInner {
        ApiLoggerInner { update_callback }
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
            let level = meta.level();
            let log_level = VeilidLogLevel::from_tracing_level(*level);

            let origin = meta
                .file()
                .and_then(|file| meta.line().map(|ln| format!("{}:{}", file, ln)))
                .unwrap_or_default();

            let message = format!("{} {}", origin, recorder);

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
            //if self.is_following_args {
            // following args
            //    writeln!(self.display).unwrap();
            //} else {
            // first arg
            write!(self.display, " ").unwrap();
            //self.is_following_args = true;
            //}
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
