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
            // use tracing_subscriber::{filter, fmt, prelude::*};
            // let mut filters = filter::Targets::new();
            // for ig in DEFAULT_LOG_IGNORE_LIST {
            //     filters = filters.with_target(ig, filter::LevelFilter::OFF);
            // }
            // let fmt_layer = fmt::layer();
            // tracing_subscriber::registry()
            //     .with(filters)
            //     .with(filter::LevelFilter::TRACE)
            //     .with(fmt_layer)
            //     .init();

            let mut filters = filter::Targets::new();
            for ig in DEFAULT_LOG_IGNORE_LIST {
                filters = filters.with_target(ig, filter::LevelFilter::OFF);
            }
            tracing_subscriber::registry()
                .with(filters)
                .with(filter::LevelFilter::TRACE)
                .with(OsLogger::new("com.veilid.veilidtools-tests", "default"))
                .init();
        } else {
            // use simplelog::*;
            // let mut logs: Vec<Box<dyn SharedLogger>> = Vec::new();
            // let mut cb = ConfigBuilder::new();
            // for ig in DEFAULT_LOG_IGNORE_LIST {
            //     cb.add_filter_ignore_str(ig);
            // }
            // logs.push(TermLogger::new(
            //     LevelFilter::Trace,
            //     cb.build(),
            //     TerminalMode::Mixed,
            //     ColorChoice::Auto,
            // ));
            // CombinedLogger::init(logs).expect("logger init error");

            OsLogger::new("com.veilid.veilidtools-tests", "default")
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
