use super::*;
use tracing::level_filters::LevelFilter;
use tracing::subscriber::Interest;
use tracing_subscriber::layer;

struct VeilidLayerFilterInner {
    max_level: LevelFilter,
    ignore_list: Vec<String>,
}

#[derive(Clone)]
pub struct VeilidLayerFilter {
    inner: Arc<RwLock<VeilidLayerFilterInner>>,
}

impl VeilidLayerFilter {
    pub fn new(
        max_level: VeilidConfigLogLevel,
        ignore_log_targets: &[String],
    ) -> VeilidLayerFilter {
        let mut ignore_list = DEFAULT_LOG_FACILITIES_IGNORE_LIST
            .map(|x| x.to_owned())
            .to_vec();
        Self::apply_ignore_change_list(&mut ignore_list, ignore_log_targets);
        Self {
            inner: Arc::new(RwLock::new(VeilidLayerFilterInner {
                max_level: max_level.to_tracing_level_filter(),
                ignore_list,
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
                    .map(|x| x.to_owned())
                    .to_vec()
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
    fn enabled(&self, metadata: &tracing::Metadata<'_>, _: &layer::Context<'_, S>) -> bool {
        self.interesting(metadata)
    }

    fn callsite_enabled(&self, metadata: &'static tracing::Metadata<'static>) -> Interest {
        if self.interesting(metadata) {
            Interest::always()
        } else {
            Interest::never()
        }
    }

    fn max_level_hint(&self) -> Option<LevelFilter> {
        let inner = self.inner.read();
        Some(inner.max_level)
    }
}
