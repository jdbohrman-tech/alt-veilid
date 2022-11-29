use super::*;

//use jni::errors::Result as JniResult;
use jni::{objects::GlobalRef, objects::JObject, JNIEnv, JavaVM};
use lazy_static::*;
use std::backtrace::Backtrace;
use std::panic;

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

pub fn veilid_tools_setup_android_no_log<'a>(env: JNIEnv<'a>, ctx: JObject<'a>) {
    *ANDROID_GLOBALS.lock() = Some(AndroidGlobals {
        vm: env.get_java_vm().unwrap(),
        ctx: env.new_global_ref(ctx).unwrap(),
    });
}

pub fn veilid_tools_setup<'a>(env: JNIEnv<'a>, ctx: JObject<'a>, log_tag: &'a str) {
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
            let layer = tracing_android::layer(log_tag)
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

    veilid_tools_setup_android_no_log(env, ctx);
}

// pub fn get_android_globals() -> (JavaVM, GlobalRef) {
//     let globals_locked = ANDROID_GLOBALS.lock();
//     let globals = globals_locked.as_ref().unwrap();
//     let env = globals.vm.attach_current_thread_as_daemon().unwrap();
//     let vm = env.get_java_vm().unwrap();
//     let ctx = globals.ctx.clone();
//     (vm, ctx)
// }

// pub fn with_null_local_frame<'b, T, F>(env: JNIEnv<'b>, s: i32, f: F) -> JniResult<T>
// where
//     F: FnOnce() -> JniResult<T>,
// {
//     env.push_local_frame(s)?;
//     let out = f();
//     env.pop_local_frame(JObject::null())?;
//     out
// }
