use super::native::*;
use crate::*;
use backtrace::Backtrace;
use jni::{
    objects::GlobalRef, objects::JClass, objects::JObject, objects::JString, JNIEnv, JavaVM,
};
use lazy_static::*;
use std::panic;
use tracing_subscriber::{filter, fmt, prelude::*};

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_com_veilid_veilid_1core_1android_1tests_MainActivity_run_1tests(
    env: JNIEnv,
    _class: JClass,
    ctx: JObject,
) {
    crate::intf::utils::android::veilid_core_setup_android_tests(env, ctx);
    block_on(async {
        run_all_tests().await;
    })
}

pub fn veilid_core_setup_android_tests(env: JNIEnv, ctx: JObject) {
    // Set up subscriber and layers
    let filter = VeilidLayerFilter::new(VeilidConfigLogLevel::Trace, None);
    let layer = tracing_android::layer("veilid-core").expect("failed to set up android logging");
    tracing_subscriber::registry()
        .with(filter)
        .with(layer)
        .init();

    // Set up panic hook for backtraces
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

    veilid_core_setup_android(env, ctx);
}
