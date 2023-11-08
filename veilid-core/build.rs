use glob::glob;
use std::{
    env, fs, io,
    path::Path,
    process::{Command, Stdio},
};

const CAPNP_VERSION: &str = "1.0.1";

fn get_desired_capnp_version_string() -> String {
    CAPNP_VERSION.to_string()
}

fn get_capnp_version_string() -> String {
    let output = Command::new("capnp")
        .arg("--version")
        .stdout(Stdio::piped())
        .output()
        .expect("capnp was not in the PATH");
    let s = String::from_utf8(output.stdout)
        .expect("'capnp --version' output was not a valid string")
        .trim()
        .to_owned();

    if !s.starts_with("Cap'n Proto version ") {
        panic!("invalid capnp version string: {}", s);
    }
    s[20..].to_owned()
}

fn is_input_file_outdated<P1, P2>(input: P1, output: P2) -> io::Result<bool>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
{
    let out_meta = fs::metadata(output);
    if let Ok(meta) = out_meta {
        let output_mtime = meta.modified()?;

        // if input file is more recent than our output, we are outdated
        let input_meta = fs::metadata(input)?;
        let input_mtime = input_meta.modified()?;

        Ok(input_mtime > output_mtime)
    } else {
        // output file not found, we are outdated
        Ok(true)
    }
}

fn do_capnp_build() {
    let desired_capnp_version_string = get_desired_capnp_version_string();
    let capnp_version_string = get_capnp_version_string();

    // Check capnp version
    let desired_capnp_major_version = desired_capnp_version_string
        .split_once('.')
        .unwrap()
        .0
        .parse::<usize>()
        .expect("should be valid int");

    if capnp_version_string
        .split_once('.')
        .unwrap()
        .0
        .parse::<usize>()
        .expect("should be valid int")
        != desired_capnp_major_version
    {
        panic!(
            "capnproto version should be major version 1, preferably {} but is {}",
            desired_capnp_version_string, capnp_version_string
        );
    } else if capnp_version_string != desired_capnp_version_string {
        println!(
            "cargo:warning=capnproto version may be untested: {}",
            capnp_version_string
        );
    }

    ::capnpc::CompilerCommand::new()
        .file("proto/veilid.capnp")
        .output_path(".")
        .run()
        .expect("compiling schema");
}

// Fix for missing __extenddftf2 on Android x86_64 Emulator
fn fix_android_emulator() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    if target_arch == "x86_64" && target_os == "android" {
        let missing_library = "clang_rt.builtins-x86_64-android";
        let android_ndk_home = env::var("ANDROID_NDK_HOME").expect("ANDROID_NDK_HOME not set");
        let lib_path = glob(&format!("{android_ndk_home}/**/lib{missing_library}.a"))
            .expect("failed to glob")
            .next()
            .expect("Need libclang_rt.builtins-x86_64-android.a")
            .unwrap();
        let lib_dir = lib_path.parent().unwrap();
        println!("cargo:rustc-link-search={}", lib_dir.display());
        println!("cargo:rustc-link-lib=static={missing_library}");
    }
}

fn main() {
    if std::env::var("DOCS_RS").is_ok()
        || std::env::var("CARGO_CFG_DOC").is_ok()
        || std::env::var("BUILD_DOCS").is_ok()
    {
        return;
    }

    if is_input_file_outdated("./proto/veilid.capnp", "./proto/veilid_capnp.rs").unwrap() {
        do_capnp_build();
    }

    fix_android_emulator();
}
