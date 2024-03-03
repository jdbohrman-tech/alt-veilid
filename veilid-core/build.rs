use glob::glob;
use sha2::{Digest, Sha256};
use std::fs::OpenOptions;
use std::io::BufRead;
use std::io::Write;
use std::{
    env, io,
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
        .expect("capnp was not in the PATH, and is required for the build when you have changed any .capnp files");
    let s = String::from_utf8(output.stdout)
        .expect("'capnp --version' output was not a valid string")
        .trim()
        .to_owned();

    if !s.starts_with("Cap'n Proto version ") {
        panic!("invalid capnp version string: {}", s);
    }
    s[20..].to_owned()
}

fn is_input_file_outdated<P, Q>(input: P, output: Q) -> io::Result<bool>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let Some(out_bh) = get_build_hash(output) else {
        // output file not found or no build hash, we are outdated
        return Ok(true);
    };

    let in_bh = make_build_hash(input);

    Ok(out_bh != in_bh)
}

fn calculate_hash(lines: std::io::Lines<std::io::BufReader<std::fs::File>>) -> Vec<u8> {
    let mut hasher = Sha256::new();
    // Build hash of lines, ignoring EOL conventions
    for l in lines {
        let l = l.unwrap();
        hasher.update(l.as_bytes());
        hasher.update(b"\n");
    }
    let out = hasher.finalize();
    out.to_vec()
}

fn get_build_hash<Q: AsRef<Path>>(output_path: Q) -> Option<Vec<u8>> {
    let lines = std::io::BufReader::new(std::fs::File::open(output_path).ok()?).lines();
    for l in lines {
        let l = l.unwrap();
        if let Some(rest) = l.strip_prefix("//BUILDHASH:") {
            return Some(hex::decode(rest).unwrap());
        }
    }
    None
}

fn make_build_hash<P: AsRef<Path>>(input_path: P) -> Vec<u8> {
    let input_path = input_path.as_ref();
    let lines = std::io::BufReader::new(std::fs::File::open(input_path).unwrap()).lines();
    calculate_hash(lines)
}

fn append_hash<P: AsRef<Path>, Q: AsRef<Path>>(input_path: P, output_path: Q) {
    let input_path = input_path.as_ref();
    let output_path = output_path.as_ref();
    let lines = std::io::BufReader::new(std::fs::File::open(input_path).unwrap()).lines();
    let h = calculate_hash(lines);
    let mut out_file = OpenOptions::new().append(true).open(output_path).unwrap();
    writeln!(out_file, "\n//BUILDHASH:{}", hex::encode(h)).unwrap();
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

    // If successful, append a hash of the input to the output file
    append_hash("proto/veilid.capnp", "proto/veilid_capnp.rs");
}

// Fix for missing __extenddftf2 on Android x86_64 Emulator
fn fix_android_emulator() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    if target_arch == "x86_64" && target_os == "android" {
        let missing_library = "clang_rt.builtins-x86_64-android";
        let android_home = env::var("ANDROID_HOME").expect("ANDROID_HOME not set");
        let lib_path = glob(&format!(
            "{android_home}/ndk/25.1.8937393/**/lib{missing_library}.a"
        ))
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
        println!("cargo:warning=rebuilding proto/veilid_capnp.rs because it has changed from the last generation of proto/veilid.capnp");
        do_capnp_build();
    }

    fix_android_emulator();
}
