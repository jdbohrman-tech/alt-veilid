// xxx : support for android older than API 24, if we need it someday
//mod android_get_if_addrs;
//pub use android_get_if_addrs::*;

mod get_directories;
pub use get_directories::*;

use crate::veilid_config::VeilidConfigLogLevel;
use crate::xx::*;
use crate::*;
use backtrace::Backtrace;
use jni::errors::Result as JniResult;
use jni::{objects::GlobalRef, objects::JObject, objects::JString, JNIEnv, JavaVM};
use lazy_static::*;
use std::panic;
use tracing::*;
use tracing_subscriber::prelude::*;
use tracing_subscriber::*;

pub struct AndroidGlobals {
    pub vm: JavaVM,
    pub ctx: GlobalRef,
}

impl Drop for AndroidGlobals {
    fn drop(&mut self) {
        // Ensure we're attached before dropping GlobalRef
        self.vm.attach_current_thread_as_daemon().unwrap();
    }
}

lazy_static! {
    pub static ref ANDROID_GLOBALS: Arc<Mutex<Option<AndroidGlobals>>> = Arc::new(Mutex::new(None));
}

pub fn veilid_core_setup_android_no_log<'a>(env: JNIEnv<'a>, ctx: JObject<'a>) {
    *ANDROID_GLOBALS.lock() = Some(AndroidGlobals {
        vm: env.get_java_vm().unwrap(),
        ctx: env.new_global_ref(ctx).unwrap(),
    });
}

pub fn veilid_core_setup_android<'a>(
    env: JNIEnv<'a>,
    ctx: JObject<'a>,
    log_tag: &'a str,
    log_level: VeilidConfigLogLevel,
) {
    // Set up subscriber and layers
    let subscriber = Registry::default();
    let mut layers = Vec::new();
    let mut filters = BTreeMap::new();
    let filter = VeilidLayerFilter::new(log_level, None);
    let layer = tracing_android::layer(log_tag)
        .expect("failed to set up android logging")
        .with_filter(filter.clone());
    filters.insert("system", filter);
    layers.push(layer.boxed());

    let subscriber = subscriber.with(layers);
    subscriber
        .try_init()
        .expect("failed to init android tracing");

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

    veilid_core_setup_android_no_log(env, ctx);
}

pub fn get_android_globals() -> (JavaVM, GlobalRef) {
    let globals_locked = ANDROID_GLOBALS.lock();
    let globals = globals_locked.as_ref().unwrap();
    let env = globals.vm.attach_current_thread_as_daemon().unwrap();
    let vm = env.get_java_vm().unwrap();
    let ctx = globals.ctx.clone();
    (vm, ctx)
}

pub fn with_null_local_frame<'b, T, F>(env: JNIEnv<'b>, s: i32, f: F) -> JniResult<T>
where
    F: FnOnce() -> JniResult<T>,
{
    env.push_local_frame(s)?;
    let out = f();
    env.pop_local_frame(JObject::null())?;
    out
}
