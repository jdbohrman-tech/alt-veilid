// LogThru
// Pass errors through and log them simultaneously via map_err()
// Also contains common log facilities (net, rpc, rtab, stor, pstore, crypto, etc )

use super::*;

pub fn map_to_string<X: ToString>(arg: X) -> String {
    arg.to_string()
}

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

#[macro_export]
macro_rules! logthru_net {
    ($($level:ident)?) => {
        logthru!($($level)? "net")
    };
    ($($level:ident)? $text:literal) => {
        logthru!($($level)? "net", $text)
    };
    ($($level:ident)? $fmt:literal, $($arg:expr),+) => {
        logthru!($($level)? "net", $fmt, $($arg),+)
    }
}
#[macro_export]
macro_rules! logthru_rpc {
    ($($level:ident)?) => {
        logthru!($($level)? "rpc")
    };
    ($($level:ident)? $text:literal) => {
        logthru!($($level)? "rpc", $text)
    };
    ($($level:ident)? $fmt:literal, $($arg:expr),+) => {
        logthru!($($level)? "rpc", $fmt, $($arg),+)
    }
}
#[macro_export]
macro_rules! logthru_rtab {
    ($($level:ident)?) => {
        logthru!($($level)? "rtab")
    };
    ($($level:ident)? $text:literal) => {
        logthru!($($level)? "rtab", $text)
    };
    ($($level:ident)? $fmt:literal, $($arg:expr),+) => {
        logthru!($($level)? "rtab", $fmt, $($arg),+)
    }
}
#[macro_export]
macro_rules! logthru_stor {
    ($($level:ident)?) => {
        logthru!($($level)? "stor")
    };
    ($($level:ident)? $text:literal) => {
        logthru!($($level)? "stor", $text)
    };
    ($($level:ident)? $fmt:literal, $($arg:expr),+) => {
        logthru!($($level)? "stor", $fmt, $($arg),+)
    }
}
#[macro_export]
macro_rules! logthru_pstore {
    ($($level:ident)?) => {
        logthru!($($level)? "pstore")
    };
    ($($level:ident)? $text:literal) => {
        logthru!($($level)? "pstore", $text)
    };
    ($($level:ident)? $fmt:literal, $($arg:expr),+) => {
        logthru!($($level)? "pstore", $fmt, $($arg),+)
    }
}
#[macro_export]
macro_rules! logthru_crypto {
    ($($level:ident)?) => {
        logthru!($($level)? "crypto")
    };
    ($($level:ident)? $text:literal) => {
        logthru!($($level)? "crypto", $text)
    };
    ($($level:ident)? $fmt:literal, $($arg:expr),+) => {
        logthru!($($level)? "crypto", $fmt, $($arg),+)
    }
}

#[macro_export]
macro_rules! logthru {
    // error
    (error $target:literal) => (|e__| {
        error!(
            target: $target,
            "[{:?}]",
            e__,
        );
        e__
    });
    (error $target:literal, $text:literal) => (|e__| {
        error!(
            target: $target,
            "[{:?}] {}",
            e__,
            $text
        );
        e__
    });
    (error $target:literal, $fmt:literal, $($arg:expr),+) => (|e__| {
        error!(
            target: $target,
            concat!("[{:?}] ", $fmt),
            e__,
            $($arg),+
        );
        e__
    });
    // warn
    (warn $target:literal) => (|e__| {
        warn!(
            target: $target,
            "[{:?}]",
            e__,
        );
        e__
    });
    (warn $target:literal, $text:literal) => (|e__| {
        warn!(
            target: $target,
            "[{:?}] {}",
            e__,
            $text
        );
        e__
    });
    (warn $target:literal, $fmt:literal, $($arg:expr),+) => (|e__| {
        warn!(
            target: $target,
            concat!("[{:?}] ", $fmt),
            e__,
            $($arg),+
        );
        e__
    });
    // debug
    (debug $target:literal) => (|e__| {
        debug!(
            target: $target,
            "[{:?}]",
            e__,
        );
        e__
    });
    (debug $target:literal, $text:literal) => (|e__| {
        debug!(
            target: $target,
            "[{:?}] {}",
            e__,
            $text
        );
        e__
    });
    (debug $target:literal, $fmt:literal, $($arg:expr),+) => (|e__| {
        debug!(
            target: $target,
            concat!("[{:?}] ", $fmt),
            e__,
            $($arg),+
        );
        e__
    });
    // trace
    ($target:literal) => (|e__| {
        trace!(
            target: $target,
            "[{:?}]",
            e__,
        );
        e__
    });
    ($target:literal, $text:literal) => (|e__| {
        trace!(
            target: $target,
            "[{:?}] {}",
            e__,
            $text
        );
        e__
    });
    ($target:literal, $fmt:literal, $($arg:expr),+) => (|e__| {
        trace!(
            target: $target,
            concat!("[{:?}] ", $fmt),
            e__,
            $($arg),+
        );
        e__
    })
}
