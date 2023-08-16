use super::*;

/// Log level for VeilidCore
#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy, Serialize, Deserialize, JsonSchema, Tsify,
)]
#[tsify(namespace)]
pub enum VeilidLogLevel {
    Error = 1,
    Warn = 2,
    Info = 3,
    Debug = 4,
    Trace = 5,
}

impl VeilidLogLevel {
    pub fn from_tracing_level(level: tracing::Level) -> VeilidLogLevel {
        match level {
            tracing::Level::ERROR => VeilidLogLevel::Error,
            tracing::Level::WARN => VeilidLogLevel::Warn,
            tracing::Level::INFO => VeilidLogLevel::Info,
            tracing::Level::DEBUG => VeilidLogLevel::Debug,
            tracing::Level::TRACE => VeilidLogLevel::Trace,
        }
    }
    pub fn from_log_level(level: log::Level) -> VeilidLogLevel {
        match level {
            log::Level::Error => VeilidLogLevel::Error,
            log::Level::Warn => VeilidLogLevel::Warn,
            log::Level::Info => VeilidLogLevel::Info,
            log::Level::Debug => VeilidLogLevel::Debug,
            log::Level::Trace => VeilidLogLevel::Trace,
        }
    }
    pub fn to_tracing_level(&self) -> tracing::Level {
        match self {
            Self::Error => tracing::Level::ERROR,
            Self::Warn => tracing::Level::WARN,
            Self::Info => tracing::Level::INFO,
            Self::Debug => tracing::Level::DEBUG,
            Self::Trace => tracing::Level::TRACE,
        }
    }
    pub fn to_log_level(&self) -> log::Level {
        match self {
            Self::Error => log::Level::Error,
            Self::Warn => log::Level::Warn,
            Self::Info => log::Level::Info,
            Self::Debug => log::Level::Debug,
            Self::Trace => log::Level::Trace,
        }
    }
}

impl FromStr for VeilidLogLevel {
    type Err = VeilidAPIError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Error" => Self::Error,
            "Warn" => Self::Warn,
            "Info" => Self::Info,
            "Debug" => Self::Debug,
            "Trace" => Self::Trace,
            _ => {
                apibail_invalid_argument!("Can't convert str", "s", s);
            }
        })
    }
}
impl fmt::Display for VeilidLogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let text = match self {
            Self::Error => "Error",
            Self::Warn => "Warn",
            Self::Info => "Info",
            Self::Debug => "Debug",
            Self::Trace => "Trace",
        };
        write!(f, "{}", text)
    }
}
/// A VeilidCore log message with optional backtrace
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Tsify)]
pub struct VeilidLog {
    pub log_level: VeilidLogLevel,
    pub message: String,
    #[tsify(optional)]
    pub backtrace: Option<String>,
}
