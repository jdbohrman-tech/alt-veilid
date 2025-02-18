pub static DEFAULT_LOG_FACILITIES_IGNORE_LIST: &[&str] = &[
    "mio",
    "h2",
    "hyper",
    "tower",
    "tonic",
    "tokio",
    "runtime",
    "tokio_util",
    "want",
    "serial_test",
    "async_std",
    "async_io",
    "polling",
    "rustls",
    "async_tungstenite",
    "tungstenite",
    "netlink_proto",
    "netlink_sys",
    "hickory_resolver",
    "hickory_proto",
    "attohttpc",
    "ws_stream_wasm",
    "keyvaluedb_web",
    "veilid_api",
    "network_result",
    "dht",
    "fanout",
    "receipt",
    "rpc_message",
    #[cfg(feature = "geolocation")]
    "maxminddb",
];

pub static FLAME_LOG_FACILITIES_IGNORE_LIST: &[&str] = &[
    "mio",
    "h2",
    "hyper",
    "tower",
    "tonic",
    "tokio",
    "runtime",
    "tokio_util",
    "want",
    "serial_test",
    "async_std",
    "async_io",
    "polling",
    "rustls",
    "async_tungstenite",
    "tungstenite",
    "netlink_proto",
    "netlink_sys",
    "hickory_resolver",
    "hickory_proto",
    "attohttpc",
    "ws_stream_wasm",
    #[cfg(feature = "geolocation")]
    "maxminddb",
];

pub static DEFAULT_LOG_FACILITIES_ENABLED_LIST: &[&str] = &[
    "net",
    "rpc",
    "rtab",
    "stor",
    "client_api",
    "pstore",
    "tstore",
    "crypto",
    "veilid_debug",
];

pub static DURATION_LOG_FACILITIES: &[&str] = &["veilid_api"];

#[macro_export]
macro_rules! impl_veilid_log_facility {
    ($facility:literal) => {
        const __VEILID_LOG_FACILITY: &'static str = $facility;
    };
}

#[macro_export]
macro_rules! fn_string {
    ($text:expr) => {
        || $text.to_string()
    };
}

#[macro_export]
macro_rules! veilid_log {
    // ERROR //////////////////////////////////////////////////////////////////////////
    // veilid_log!(self error "message")
    ($self_expr:ident error $text:expr) => {error!(
        target: self::__VEILID_LOG_FACILITY,
        __VEILID_LOG_KEY = $self_expr.log_key(),
        "{}",
        $text,
    )};
    // veilid_log!(self error target: "facility", "message")
    ($self_expr:ident error target: $target:expr, $text:expr) => {error!(
        target: $target,
        __VEILID_LOG_KEY = $self_expr.log_key(),
        "{}",
        $text,
    )};
    // veilid_log!(self error "data: {}", data)
    ($self_expr:ident error $fmt:literal, $($arg:expr),+) => {error!(
        target: self::__VEILID_LOG_FACILITY,
        __VEILID_LOG_KEY = $self_expr.log_key(),
        $fmt, $($arg),+);
    };
    // veilid_log!(self error target: "facility", "data: {}", data)
    ($self_expr:ident error target: $target:expr, $fmt:literal, $($arg:expr),+) => {error!(
        target: $target,
        __VEILID_LOG_KEY = $self_expr.log_key(),
        $fmt, $($arg),+);
    };
    // veilid_log!(self error field=value, ?other_field)
    ($self_expr:ident error $($k:ident).+ = $($fields:tt)*) => {error!(
        target: self::__VEILID_LOG_FACILITY,
        __VEILID_LOG_KEY = $self_expr.log_key(),
        $($k).+ = $($fields)*
    )};
    // veilid_log!(self error target: "facility", field=value, ?other_field)
    ($self_expr:ident error target: $target:expr, $($k:ident).+ = $($fields:tt)*) => {error!(
        target: $target,
        __VEILID_LOG_KEY = $self_expr.log_key(),
        $($k).+ = $($fields)*
    )};

    // WARN //////////////////////////////////////////////////////////////////////////
    // veilid_log!(self warn "message")
    ($self_expr:ident warn $text:expr) => {warn!(
        target: self::__VEILID_LOG_FACILITY,
        __VEILID_LOG_KEY = $self_expr.log_key(),
        "{}",
        $text,
    )};
    // veilid_log!(self warn target: "facility", "message")
    ($self_expr:ident warn target: $target:expr, $text:expr) => {warn!(
        target: $target,
        __VEILID_LOG_KEY = $self_expr.log_key(),
        "{}",
        $text,
    )};
    // veilid_log!(self warn "data: {}", data)
    ($self_expr:ident warn $fmt:literal, $($arg:expr),+) => {warn!(
        target: self::__VEILID_LOG_FACILITY,
        __VEILID_LOG_KEY = $self_expr.log_key(),
        $fmt, $($arg),+);
    };
    // veilid_log!(self warn target: "facility", "data: {}", data)
    ($self_expr:ident warn target: $target:expr, $fmt:literal, $($arg:expr),+) => {warn!(
        target: $target,
        __VEILID_LOG_KEY = $self_expr.log_key(),
        $fmt, $($arg),+);
    };
    // veilid_log!(self warn field=value, ?other_field)
    ($self_expr:ident warn $($k:ident).+ = $($fields:tt)*) => {warn!(
        target: self::__VEILID_LOG_FACILITY,
        __VEILID_LOG_KEY = $self_expr.log_key(),
        $($k).+ = $($fields)*
    )};
    // veilid_log!(self warn target: "facility", field=value, ?other_field)
    ($self_expr:ident warn target: $target:expr, $($k:ident).+ = $($fields:tt)*) => {warn!(
        target: $target,
        __VEILID_LOG_KEY = $self_expr.log_key(),
        $($k).+ = $($fields)*
    )};

    // INFO //////////////////////////////////////////////////////////////////////////
    // veilid_log!(self info "message")
    ($self_expr:ident info $text:expr) => {info!(
        target: self::__VEILID_LOG_FACILITY,
        __VEILID_LOG_KEY = $self_expr.log_key(),
        "{}",
        $text,
    )};
    // veilid_log!(self info target: "facility", "message")
    ($self_expr:ident info target: $target:expr, $text:expr) => {info!(
        target: $target,
        __VEILID_LOG_KEY = $self_expr.log_key(),
        "{}",
        $text,
    )};
    // veilid_log!(self info "data: {}", data)
    ($self_expr:ident info $fmt:literal, $($arg:expr),+) => {info!(
        target: self::__VEILID_LOG_FACILITY,
        __VEILID_LOG_KEY = $self_expr.log_key(),
        $fmt, $($arg),+);
    };
    // veilid_log!(self info target: "facility", "data: {}", data)
    ($self_expr:ident info target: $target:expr, $fmt:literal, $($arg:expr),+) => {info!(
        target: $target,
        __VEILID_LOG_KEY = $self_expr.log_key(),
        $fmt, $($arg),+);
    };
    // veilid_log!(self info field=value, ?other_field)
    ($self_expr:ident info $($k:ident).+ = $($fields:tt)*) => {info!(
        target: self::__VEILID_LOG_FACILITY,
        __VEILID_LOG_KEY = $self_expr.log_key(),
        $($k).+ = $($fields)*
    )};
    // veilid_log!(self info target: "facility", field=value, ?other_field)
    ($self_expr:ident info target: $target:expr, $($k:ident).+ = $($fields:tt)*) => {info!(
        target: $target,
        __VEILID_LOG_KEY = $self_expr.log_key(),
        $($k).+ = $($fields)*
    )};

    // DEBUG //////////////////////////////////////////////////////////////////////////
    // veilid_log!(self debug "message")
    ($self_expr:ident debug $text:expr) => {debug!(
        target: self::__VEILID_LOG_FACILITY,
        __VEILID_LOG_KEY = $self_expr.log_key(),
        "{}",
        $text,
    )};
    // veilid_log!(self debug target: "facility", "message")
    ($self_expr:ident debug target: $target:expr, $text:expr) => {debug!(
        target: $target,
        __VEILID_LOG_KEY = $self_expr.log_key(),
        "{}",
        $text,
    )};
    // veilid_log!(self debug "data: {}", data)
    ($self_expr:ident debug $fmt:literal, $($arg:expr),+) => {debug!(
        target: self::__VEILID_LOG_FACILITY,
        __VEILID_LOG_KEY = $self_expr.log_key(),
        $fmt, $($arg),+);
    };
    // veilid_log!(self debug target: "facility", "data: {}", data)
    ($self_expr:ident debug target: $target:expr, $fmt:literal, $($arg:expr),+) => {debug!(
        target: $target,
        __VEILID_LOG_KEY = $self_expr.log_key(),
        $fmt, $($arg),+);
    };
    // veilid_log!(self debug field=value, ?other_field)
    ($self_expr:ident debug $($k:ident).+ = $($fields:tt)*) => {debug!(
        target: self::__VEILID_LOG_FACILITY,
        __VEILID_LOG_KEY = $self_expr.log_key(),
        $($k).+ = $($fields)*
    )};
    // veilid_log!(self debug target: "facility", field=value, ?other_field)
    ($self_expr:ident debug target: $target:expr, $($k:ident).+ = $($fields:tt)*) => {debug!(
        target: $target,
        __VEILID_LOG_KEY = $self_expr.log_key(),
        $($k).+ = $($fields)*
    )};

    // TRACE //////////////////////////////////////////////////////////////////////////
    // veilid_log!(self trace "message")
    ($self_expr:ident trace $text:expr) => {trace!(
        target: self::__VEILID_LOG_FACILITY,
        __VEILID_LOG_KEY = $self_expr.log_key(),
        "{}",
        $text,
    )};
    // veilid_log!(self trace target: "facility", "message")
    ($self_expr:ident trace target: $target:expr, $text:expr) => {trace!(
        target: $target,
        __VEILID_LOG_KEY = $self_expr.log_key(),
        "{}",
        $text,
    )};
    // veilid_log!(self trace "data: {}", data)
    ($self_expr:ident trace $fmt:literal, $($arg:expr),+) => {trace!(
        target: self::__VEILID_LOG_FACILITY,
        __VEILID_LOG_KEY = $self_expr.log_key(),
        $fmt, $($arg),+);
    };
    // veilid_log!(self trace target: "facility", "data: {}", data)
    ($self_expr:ident trace target: $target:expr, $fmt:literal, $($arg:expr),+) => {trace!(
        target: $target,
        __VEILID_LOG_KEY = $self_expr.log_key(),
        $fmt, $($arg),+);
    };
    // veilid_log!(self trace field=value, ?other_field)
    ($self_expr:ident trace $($k:ident).+ = $($fields:tt)*) => {trace!(
        target: self::__VEILID_LOG_FACILITY,
        __VEILID_LOG_KEY = $self_expr.log_key(),
        $($k).+ = $($fields)*
    )};
    // veilid_log!(self trace target: "facility", field=value, ?other_field)
    ($self_expr:ident trace target: $target:expr, $($k:ident).+ = $($fields:tt)*) => {trace!(
        target: $target,
        __VEILID_LOG_KEY = $self_expr.log_key(),
        $($k).+ = $($fields)*
    )};
}
