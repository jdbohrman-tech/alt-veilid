use super::native::*;
use super::*;

use std::backtrace::Backtrace;
use std::panic;

#[no_mangle]
pub extern "C" fn run_veilid_tools_tests() {
    crate::tests::ios::veilid_tools_setup_ios_tests();
    run_all_tests();
}

pub fn veilid_tools_setup_ios_tests() {
    cfg_if! {
        if #[cfg(feature = "tracing")] {
            use tracing_subscriber::{filter, fmt, prelude::*};
            let mut filters = filter::Targets::new();
            for ig in DEFAULT_LOG_IGNORE_LIST {
                filters = filters.with_target(ig, filter::LevelFilter::OFF);
            }
            let fmt_layer = fmt::layer();
            tracing_subscriber::registry()
                .with(filters)
                .with(filter::LevelFilter::TRACE)
                .with(fmt_layer)
                .init();
        } else {
            use simplelog::*;
            let mut logs: Vec<Box<dyn SharedLogger>> = Vec::new();
            let mut cb = ConfigBuilder::new();
            for ig in DEFAULT_LOG_IGNORE_LIST {
                cb.add_filter_ignore_str(ig);
            }
            logs.push(TermLogger::new(
                LevelFilter::Trace,
                cb.build(),
                TerminalMode::Mixed,
                ColorChoice::Auto,
            ));
            CombinedLogger::init(logs).expect("logger init error");
        }
    }

    panic::set_hook(Box::new(|panic_info| {
        let bt = Backtrace::capture();
        if let Some(location) = panic_info.location() {
            error!(
                "panic occurred in file '{}' at line {}",
                location.file(),
                location.line(),
            );
        } else {
            error!("panic occurred but can't get location information...");
        }
        if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            error!("panic payload: {:?}", s);
        } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            error!("panic payload: {:?}", s);
        } else if let Some(a) = panic_info.payload().downcast_ref::<std::fmt::Arguments>() {
            error!("panic payload: {:?}", a);
        } else {
            error!("no panic payload");
        }
        error!("Backtrace:\n{:?}", bt);
    }));
}
