use super::*;
use tracing::level_filters::LevelFilter;
use tracing::subscriber::Interest;
use tracing_subscriber::layer;

struct VeilidLayerFilterInner {
    max_level: LevelFilter,
    ignore_list: Vec<String>,
    log_key_filter: Option<VeilidLayerLogKeyFilter>,
    empty_log_key_enabled: bool,
}

#[derive(Clone)]
pub struct VeilidLayerFilter {
    inner: Arc<RwLock<VeilidLayerFilterInner>>,
}
pub const VEILID_LOG_KEY_FIELD: &str = "__VEILID_LOG_KEY";

pub type VeilidLayerLogKeyFilter = Arc<dyn Fn(&str) -> bool + Send + Sync>;

impl VeilidLayerFilter {
    pub fn make_veilid_log_key(program_name: &str, namespace: &str) -> &'static str {
        if namespace.is_empty() {
            program_name.to_static_str()
        } else {
            format!("{}|{}", program_name, namespace).to_static_str()
        }
    }

    pub fn new(
        max_level: VeilidConfigLogLevel,
        ignore_change_list: &[String],
        log_key_filter: Option<VeilidLayerLogKeyFilter>,
    ) -> VeilidLayerFilter {
        let mut ignore_list = DEFAULT_LOG_FACILITIES_IGNORE_LIST
            .iter()
            .map(|&x| x.to_owned())
            .collect::<Vec<_>>();
        Self::apply_ignore_change_list(&mut ignore_list, ignore_change_list);
        Self {
            inner: Arc::new(RwLock::new(VeilidLayerFilterInner {
                max_level: max_level.to_tracing_level_filter(),
                ignore_list,
                empty_log_key_enabled: log_key_filter.as_ref().map(|lkf| lkf("")).unwrap_or(true),
                log_key_filter,
            })),
        }
    }
    pub fn new_no_default(
        max_level: VeilidConfigLogLevel,
        ignore_list: &[String],
        namespace_filter: Option<VeilidLayerLogKeyFilter>,
    ) -> VeilidLayerFilter {
        Self {
            inner: Arc::new(RwLock::new(VeilidLayerFilterInner {
                max_level: max_level.to_tracing_level_filter(),
                ignore_list: ignore_list.to_vec(),
                empty_log_key_enabled: namespace_filter.as_ref().map(|lkf| lkf("")).unwrap_or(true),
                log_key_filter: namespace_filter,
            })),
        }
    }

    pub fn max_level(&self) -> VeilidConfigLogLevel {
        let inner = self.inner.read();
        VeilidConfigLogLevel::from_tracing_level_filter(inner.max_level)
    }

    pub fn ignore_list(&self) -> Vec<String> {
        let inner = self.inner.read();
        inner.ignore_list.clone()
    }

    pub fn set_max_level(&self, level: VeilidConfigLogLevel) {
        {
            let mut inner = self.inner.write();
            inner.max_level = level.to_tracing_level_filter();
        }
        callsite::rebuild_interest_cache();
    }

    pub fn set_ignore_list(&self, ignore_list: Option<Vec<String>>) {
        {
            let mut inner = self.inner.write();
            inner.ignore_list = ignore_list.unwrap_or_else(|| {
                DEFAULT_LOG_FACILITIES_IGNORE_LIST
                    .iter()
                    .map(|&x| x.to_owned())
                    .collect::<Vec<_>>()
            });
        }
        callsite::rebuild_interest_cache();
    }

    fn interesting(&self, metadata: &tracing::Metadata<'_>) -> bool {
        let inner = self.inner.read();

        if *metadata.level() > inner.max_level {
            return false;
        }
        let skip = inner
            .ignore_list
            .iter()
            .any(|v| metadata.target().starts_with(&**v));
        if skip {
            return false;
        }

        if !inner.empty_log_key_enabled && metadata.fields().field(VEILID_LOG_KEY_FIELD).is_none() {
            return false;
        }

        true
    }

    pub fn apply_ignore_change(ignore_list: &[String], target_change: String) -> Vec<String> {
        let mut ignore_list = ignore_list.to_vec();
        let target_change = target_change
            .split(',')
            .map(|c| c.trim().to_owned())
            .collect::<Vec<String>>();
        Self::apply_ignore_change_list(&mut ignore_list, &target_change);
        ignore_list
    }

    pub fn apply_ignore_change_list(ignore_list: &mut Vec<String>, target_change: &[String]) {
        for change in target_change {
            if change.is_empty() {
                continue;
            }
            if change == "all" {
                *ignore_list = [
                    DEFAULT_LOG_FACILITIES_ENABLED_LIST.to_vec(),
                    DEFAULT_LOG_FACILITIES_IGNORE_LIST.to_vec(),
                ]
                .concat()
                .into_iter()
                .map(|x| x.to_owned())
                .collect();
                continue;
            } else if change == "none" {
                ignore_list.clear();
                continue;
            } else if change == "default" {
                *ignore_list = [DEFAULT_LOG_FACILITIES_IGNORE_LIST.to_vec()]
                    .concat()
                    .into_iter()
                    .map(|x| x.to_owned())
                    .collect();
                continue;
            } else if let Some(target) = change.strip_prefix('-') {
                ignore_list.retain(|x| x != target);
            } else if !ignore_list.contains(change) {
                ignore_list.push(change.to_string());
            }
        }
    }
}

impl<S: tracing::Subscriber> layer::Filter<S> for VeilidLayerFilter {
    fn enabled(&self, metadata: &tracing::Metadata<'_>, _cx: &layer::Context<'_, S>) -> bool {
        self.interesting(metadata)
    }

    fn callsite_enabled(&self, metadata: &'static tracing::Metadata<'static>) -> Interest {
        if self.interesting(metadata) {
            Interest::sometimes()
        } else {
            Interest::never()
        }
    }

    fn max_level_hint(&self) -> Option<LevelFilter> {
        let inner = self.inner.read();
        Some(inner.max_level)
    }

    fn event_enabled(&self, event: &Event<'_>, _cx: &layer::Context<'_, S>) -> bool {
        let (opt_log_key_filter, empty_log_key_enabled) = {
            let inner = self.inner.read();
            (inner.log_key_filter.clone(), inner.empty_log_key_enabled)
        };
        if let Some(log_key_filter) = opt_log_key_filter {
            let mut visitor = LogKeyFilterVisitor::new(log_key_filter);
            event.record(&mut visitor);
            visitor.enabled().unwrap_or(empty_log_key_enabled)
        } else {
            true
        }
    }
}

struct LogKeyFilterVisitor {
    filter: VeilidLayerLogKeyFilter,
    enabled: Option<bool>,
}
impl LogKeyFilterVisitor {
    pub fn new(filter: VeilidLayerLogKeyFilter) -> Self {
        LogKeyFilterVisitor {
            filter,
            enabled: None,
        }
    }
    pub fn enabled(&self) -> Option<bool> {
        self.enabled
    }
}

impl tracing::field::Visit for LogKeyFilterVisitor {
    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        if field.name() == VEILID_LOG_KEY_FIELD {
            self.enabled = Some((self.filter)(value));
        }
    }
    fn record_debug(&mut self, _field: &tracing::field::Field, _value: &dyn fmt::Debug) {}
}
