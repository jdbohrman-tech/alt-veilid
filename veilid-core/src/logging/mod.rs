mod api_tracing_layer;
mod veilid_layer_filter;

use super::*;

pub use api_tracing_layer::*;
pub use veilid_layer_filter::*;

#[macro_export]
macro_rules! fn_string {
    ($text:expr) => {
        || $text.to_string()
    };
}

#[macro_export]
macro_rules! log_net {
    (error $text:expr) => {error!(
        target: "net",
        "{}",
        $text,
    )};
    (error $fmt:literal, $($arg:expr),+) => {
        error!(target:"net", $fmt, $($arg),+);
    };
    (warn $text:expr) => {warn!(
        target: "net",
        "{}",
        $text,
    )};
    (warn $fmt:literal, $($arg:expr),+) => {
        warn!(target:"net", $fmt, $($arg),+);
    };
    (debug $text:expr) => {debug!(
        target: "net",
        "{}",
        $text,
    )};
    (debug $fmt:literal, $($arg:expr),+) => {
        debug!(target:"net", $fmt, $($arg),+);
    };
    ($text:expr) => {trace!(
        target: "net",
        "{}",
        $text,
    )};
    ($fmt:literal, $($arg:expr),+) => {
        trace!(target:"net", $fmt, $($arg),+);
    }
}

#[macro_export]
macro_rules! log_client_api {
    (error $text:expr) => {error!(
        target: "client_api",
        "{}",
        $text,
    )};
    (error $fmt:literal, $($arg:expr),+) => {
        error!(target:"client_api", $fmt, $($arg),+);
    };
    (warn $text:expr) => {warn!(
        target: "client_api",
        "{}",
        $text,
    )};
    (warn $fmt:literal, $($arg:expr),+) => {
        warn!(target:"client_api", $fmt, $($arg),+);
    };
    (debug $text:expr) => {debug!(
        target: "client_api",
        "{}",
        $text,
    )};
    (debug $fmt:literal, $($arg:expr),+) => {
        debug!(target:"client_api", $fmt, $($arg),+);
    };
    ($text:expr) => {trace!(
        target: "client_api",
        "{}",
        $text,
    )};
    ($fmt:literal, $($arg:expr),+) => {
        trace!(target:"client_api", $fmt, $($arg),+);
    }
}

#[macro_export]
macro_rules! log_network_result {
    (error $text:expr) => {error!(
        target: "network_result",
        "{}",
        $text,
    )};
    (error $fmt:literal, $($arg:expr),+) => {
        error!(target: "network_result", $fmt, $($arg),+);
    };
    (warn $text:expr) => {warn!(
        target: "network_result",
        "{}",
        $text,
    )};
    (warn $fmt:literal, $($arg:expr),+) => {
        warn!(target:"network_result", $fmt, $($arg),+);
    };
    (debug $text:expr) => {debug!(
        target: "network_result",
        "{}",
        $text,
    )};
    (debug $fmt:literal, $($arg:expr),+) => {
        debug!(target:"network_result", $fmt, $($arg),+);
    };
    ($text:expr) => {trace!(
        target: "network_result",
        "{}",
        $text,
    )};
    ($fmt:literal, $($arg:expr),+) => {
        trace!(target:"network_result", $fmt, $($arg),+);
    }
}

#[macro_export]
macro_rules! log_rpc {
    (error $text:expr) => { error!(
        target: "rpc",
        "{}",
        $text,
    )};
    (error $fmt:literal, $($arg:expr),+) => {
        error!(target:"rpc", $fmt, $($arg),+);
    };
    (warn $text:expr) => { warn!(
        target: "rpc",
        "{}",
        $text,
    )};
    (warn $fmt:literal, $($arg:expr),+) => {
        warn!(target:"rpc", $fmt, $($arg),+);
    };
    (debug $text:expr) => { debug!(
        target: "rpc",
        "{}",
        $text,
    )};
    (debug $fmt:literal, $($arg:expr),+) => {
        debug!(target:"rpc", $fmt, $($arg),+);
    };
    ($text:expr) => {trace!(
        target: "rpc",
        "{}",
        $text,
    )};
    ($fmt:literal, $($arg:expr),+) => {
        trace!(target:"rpc", $fmt, $($arg),+);
    }
}

#[macro_export]
macro_rules! log_dht {
    (error $text:expr) => { error!(
        target: "dht",
        "{}",
        $text,
    )};
    (error $fmt:literal, $($arg:expr),+) => {
        error!(target:"dht", $fmt, $($arg),+);
    };
    (warn $text:expr) => { warn!(
        target: "dht",
        "{}",
        $text,
    )};
    (warn $fmt:literal, $($arg:expr),+) => {
        warn!(target:"dht", $fmt, $($arg),+);
    };
    (debug $text:expr) => { debug!(
        target: "dht",
        "{}",
        $text,
    )};
    (debug $fmt:literal, $($arg:expr),+) => {
        debug!(target:"dht", $fmt, $($arg),+);
    };
    ($text:expr) => {trace!(
        target: "dht",
        "{}",
        $text,
    )};
    ($fmt:literal, $($arg:expr),+) => {
        trace!(target:"dht", $fmt, $($arg),+);
    }
}

#[macro_export]
macro_rules! log_rtab {
    (error $text:expr) => { error!(
        target: "rtab",
        "{}",
        $text,
    )};
    (error $fmt:literal, $($arg:expr),+) => {
        error!(target:"rtab", $fmt, $($arg),+);
    };
    (warn $text:expr) => { warn!(
        target: "rtab",
        "{}",
        $text,
    )};
    (warn $fmt:literal, $($arg:expr),+) => {
        warn!(target:"rtab", $fmt, $($arg),+);
    };
    (debug $text:expr) => { debug!(
        target: "rtab",
        "{}",
        $text,
    )};
    (debug $fmt:literal, $($arg:expr),+) => {
        debug!(target:"rtab", $fmt, $($arg),+);
    };
    ($text:expr) => {trace!(
        target: "rtab",
        "{}",
        $text,
    )};
    ($fmt:literal, $($arg:expr),+) => {
        trace!(target:"rtab", $fmt, $($arg),+);
    }
}

#[macro_export]
macro_rules! log_stor {
    (error $text:expr) => { error!(
        target: "stor",
        "{}",
        $text,
    )};
    (error $fmt:literal, $($arg:expr),+) => {
        error!(target:"stor", $fmt, $($arg),+);
    };
    (warn $text:expr) => { warn!(
        target: "stor",
        "{}",
        $text,
    )};
    (warn $fmt:literal, $($arg:expr),+) => {
        warn!(target:"stor", $fmt, $($arg),+);
    };
    (debug $text:expr) => { debug!(
        target: "stor",
        "{}",
        $text,
    )};
    (debug $fmt:literal, $($arg:expr),+) => {
        debug!(target:"stor", $fmt, $($arg),+);
    };
    ($text:expr) => {trace!(
        target: "stor",
        "{}",
        $text,
    )};
    ($fmt:literal, $($arg:expr),+) => {
        trace!(target:"stor", $fmt, $($arg),+);
    }
}

#[macro_export]
macro_rules! log_pstore {
    (error $text:expr) => { error!(
        target: "pstore",
        "{}",
        $text,
    )};
    (error $fmt:literal, $($arg:expr),+) => {
        error!(target:"pstore", $fmt, $($arg),+);
    };
    (warn $text:expr) => { warn!(
        target: "pstore",
        "{}",
        $text,
    )};
    (warn $fmt:literal, $($arg:expr),+) => {
        warn!(target:"pstore", $fmt, $($arg),+);
    };
    (debug $text:expr) => { debug!(
        target: "pstore",
        "{}",
        $text,
    )};
    (debug $fmt:literal, $($arg:expr),+) => {
        debug!(target:"pstore", $fmt, $($arg),+);
    };
    ($text:expr) => {trace!(
        target: "pstore",
        "{}",
        $text,
    )};
    ($fmt:literal, $($arg:expr),+) => {
        trace!(target:"pstore", $fmt, $($arg),+);
    }
}

#[macro_export]
macro_rules! log_tstore {
    (error $text:expr) => { error!(
        target: "tstore",
        "{}",
        $text,
    )};
    (error $fmt:literal, $($arg:expr),+) => {
        error!(target:"tstore", $fmt, $($arg),+);
    };
    (warn $text:expr) => { warn!(
        target: "tstore",
        "{}",
        $text,
    )};
    (warn $fmt:literal, $($arg:expr),+) => {
        warn!(target:"tstore", $fmt, $($arg),+);
    };
    (debug $text:expr) => { debug!(
        target: "tstore",
        "{}",
        $text,
    )};
    (debug $fmt:literal, $($arg:expr),+) => {
        debug!(target:"tstore", $fmt, $($arg),+);
    };
    ($text:expr) => {trace!(
        target: "tstore",
        "{}",
        $text,
    )};
    ($fmt:literal, $($arg:expr),+) => {
        trace!(target:"tstore", $fmt, $($arg),+);
    }
}

#[macro_export]
macro_rules! log_crypto {
    (error $text:expr) => { error!(
        target: "crypto",
        "{}",
        $text,
    )};
    (error $fmt:literal, $($arg:expr),+) => {
        error!(target:"crypto", $fmt, $($arg),+);
    };
    (warn $text:expr) => { warn!(
        target: "crypto",
        "{}",
        $text,
    )};
    (warn $fmt:literal, $($arg:expr),+) => {
        warn!(target:"crypto", $fmt, $($arg),+);
    };
    ($text:expr) => {trace!(
        target: "crypto",
        "{}",
        $text,
    )};
    ($fmt:literal, $($arg:expr),+) => {
        trace!(target:"crypto", $fmt, $($arg),+);
    }
}
