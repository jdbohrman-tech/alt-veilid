use super::native::*;
use super::*;

use std::backtrace::Backtrace;
use std::panic;

#[no_mangle]
pub extern "C" fn run_veilid_tools_tests() {
    veilid_tools_setup_ios_tests();
    block_on(async {
        run_all_tests().await;
    })
}

pub fn veilid_tools_setup_ios_tests() {
    cfg_if! {
        if #[cfg(feature = "tracing")] {
            use tracing::level_filters::LevelFilter;
            use tracing_oslog::OsLogger;
            use tracing_subscriber::prelude::*;
            use tracing_subscriber::filter::Targets;

            let mut filters = Targets::new();
            for ig in DEFAULT_LOG_IGNORE_LIST {
                filters = filters.with_target(ig, LevelFilter::OFF);
            }
            tracing_subscriber::registry()
                .with(OsLogger::new("com.veilid.veilidtools-tests", "default"))
                .with(LevelFilter::TRACE)
                .with(filters)
                .init();
        } else {
            use oslog::OsLogger;
            use log::LevelFilter;

            OsLogger::new("com.veilid.veilidtools-tests")
                .level_filter(LevelFilter::Trace)
                .init()
                .unwrap();
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
