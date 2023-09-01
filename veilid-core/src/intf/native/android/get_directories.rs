use super::*;
use jni::errors::Result as JniResult;
use jni::objects::JString;

#[allow(dead_code)]
pub fn get_files_dir() -> String {
    let aglock = ANDROID_GLOBALS.lock();
    let ag = aglock.as_ref().unwrap();
    let mut env = ag.vm.attach_current_thread().unwrap();

    env.with_local_frame(64, |env| {
        // context.getFilesDir().getAbsolutePath()
        let file = env
            .call_method(ag.ctx.as_obj(), "getFilesDir", "()Ljava/io/File;", &[])
            .unwrap()
            .l()
            .unwrap();
        let path = env
            .call_method(file, "getAbsolutePath", "()Ljava/lang/String;", &[])
            .unwrap()
            .l()
            .unwrap();

        let jstr = JString::from(path);
        let jstrval = env.get_string(&jstr).unwrap();
        JniResult::Ok(String::from(jstrval.to_string_lossy()))
    })
    .unwrap()
}

#[allow(dead_code)]
pub fn get_cache_dir() -> String {
    let aglock = ANDROID_GLOBALS.lock();
    let ag = aglock.as_ref().unwrap();
    let mut env = ag.vm.attach_current_thread().unwrap();

    env.with_local_frame(64, |env| {
        // context.getCacheDir().getAbsolutePath()
        let file = env
            .call_method(ag.ctx.as_obj(), "getCacheDir", "()Ljava/io/File;", &[])
            .unwrap()
            .l()
            .unwrap();
        let path = env
            .call_method(file, "getAbsolutePath", "()Ljava/lang/String;", &[])
            .unwrap()
            .l()
            .unwrap();

        let jstr = JString::from(path);
        let jstrval = env.get_string(&jstr).unwrap();
        JniResult::Ok(String::from(jstrval.to_string_lossy()))
    })
    .unwrap()
}
