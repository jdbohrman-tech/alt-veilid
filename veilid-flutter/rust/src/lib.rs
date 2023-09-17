#![deny(clippy::all)]
#![allow(clippy::comparison_chain, clippy::upper_case_acronyms)]
#![deny(unused_must_use)]
#![recursion_limit = "256"]

mod dart_ffi;
mod dart_isolate_wrapper;
mod tools;

#[cfg(target_os = "android")]
use jni::{objects::JClass, objects::JObject, JNIEnv};

#[cfg(target_os = "android")]
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_com_veilid_veilid_VeilidPlugin_init_1android(
    env: JNIEnv,
    _class: JClass,
    ctx: JObject,
) {
    veilid_core::veilid_core_setup_android(env, ctx);
}
