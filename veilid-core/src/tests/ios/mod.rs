use super::native::*;
use crate::*;
use backtrace::Backtrace;
use std::panic;
use tracing_subscriber::{fmt, prelude::*};

#[no_mangle]
#[allow(dead_code)]
pub extern "C" fn run_veilid_core_tests() {
    std::thread::spawn(|| {
        block_on(async {
            veilid_core_setup_ios_tests();
            run_all_tests().await;
        })
    });
}

pub fn veilid_core_setup_ios_tests() {
    // Set up subscriber and layers
    let filter = VeilidLayerFilter::new(VeilidConfigLogLevel::Trace, None);
    tracing_subscriber::registry()
        .with(OsLogger::new("com.veilid.veilidtools-tests", "default").with_filter(filter))
        .init();

    panic::set_hook(Box::new(|panic_info| {
        let bt = Backtrace::new();
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
