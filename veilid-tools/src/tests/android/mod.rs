use super::native::*;
use super::*;

use jni::{objects::GlobalRef, objects::JObject, JNIEnv, JavaVM};
use lazy_static::*;
use std::backtrace::Backtrace;
use std::panic;

use jni::{objects::JClass, objects::JObject, JNIEnv};

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_com_veilid_veilid_1tools_1android_1tests_MainActivity_run_1tests(
    _env: JNIEnv,
    _class: JClass,
    _ctx: JObject,
) {
    veilid_tools_setup_android_tests();
    block_on(async {
        run_all_tests().await;
    })
}

pub fn veilid_tools_setup_android_tests() {
    cfg_if! {
        if #[cfg(feature = "tracing")] {
            use tracing::*;
            use tracing_subscriber::prelude::*;
            use tracing_subscriber::*;

            let mut filters = filter::Targets::new();
            for ig in DEFAULT_LOG_IGNORE_LIST {
                filters = filters.with_target(ig, filter::LevelFilter::OFF);
            }

            // Set up subscriber and layers
            let subscriber = Registry::default();
            let mut layers = Vec::new();
            let layer = tracing_android::layer("veilid-tools")
                .expect("failed to set up android logging")
                .with_filter(filter::LevelFilter::TRACE)
                .with_filter(filters);
            layers.push(layer.boxed());

            let subscriber = subscriber.with(layers);
            subscriber
                .try_init()
                .expect("failed to init android tracing");
        } else {
            let mut builder = android_logd_logger::builder();
            builder.tag(log_tag);
            builder.prepend_module(true);
            builder.filter_level(LevelFilter::Trace);
            for ig in DEFAULT_LOG_IGNORE_LIST {
                builder.filter_module(ig, LevelFilter::Off);
            }
            builder.init();
        }
    }

    // Set up panic hook for backtraces
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
