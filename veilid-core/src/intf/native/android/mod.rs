mod get_directories;
pub use get_directories::*;

use crate::*;
use jni::{objects::GlobalRef, objects::JObject, JNIEnv, JavaVM};
use lazy_static::*;

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

pub fn veilid_core_setup_android(env: JNIEnv, ctx: JObject) {
    *ANDROID_GLOBALS.lock() = Some(AndroidGlobals {
        vm: env.get_java_vm().unwrap(),
        ctx: env.new_global_ref(ctx).unwrap(),
    });
}

pub fn is_android_ready() -> bool {
    ANDROID_GLOBALS.lock().is_some()
}

pub fn get_android_globals() -> (JavaVM, GlobalRef) {
    let globals_locked = ANDROID_GLOBALS.lock();
    let globals = globals_locked.as_ref().unwrap();
    let env = globals.vm.attach_current_thread_as_daemon().unwrap();
    let vm = env.get_java_vm().unwrap();
    let ctx = globals.ctx.clone();
    (vm, ctx)
}
