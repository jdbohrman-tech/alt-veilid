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

const CAPNP_VERSION: &str = "1.0.2";

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
    let (Some(out_bh), Some(out_capnp_hash)) = get_build_hash_and_capnp_version_hash(output) else {
        // output file not found or no build hash, we are outdated
        println!("cargo:warning=Output file not found or no build hash.");
        return Ok(true);
    };

    // Check if desired CAPNP_VERSION hash has changed
    let mut hasher = Sha256::new();
    hasher.update(get_desired_capnp_version_string().as_bytes());
    let capnp_hash = hasher.finalize().to_vec();

    let in_bh = make_build_hash(input);

    if out_bh != in_bh {
        println!("cargo:warning=Build hash has changed.");
        return Ok(true);
    }

    if out_capnp_hash != capnp_hash {
        println!("cargo:warning=Capnp desired version hash has changed.");
        return Ok(true);
    }

    Ok(false)
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

fn get_build_hash_and_capnp_version_hash<Q: AsRef<Path>>(
    output_path: Q,
) -> (Option<Vec<u8>>, Option<Vec<u8>>) {
    let output_file = match std::fs::File::open(output_path).ok() {
        // Returns a file handle if the file exists
        Some(f) => f,
        // Returns None, None if the file does not exist
        None => return (None, None),
    };
    let lines = std::io::BufReader::new(output_file).lines();
    let mut build_hash = None;
    let mut capnp_version_hash = None;
    for l in lines {
        let l = l.unwrap();
        if let Some(rest) = l.strip_prefix("//BUILDHASH:") {
            build_hash = Some(hex::decode(rest).unwrap());
        } else if let Some(rest) = l.strip_prefix("//CAPNPDESIREDVERSIONHASH:") {
            capnp_version_hash = Some(hex::decode(rest).unwrap());
        }
    }
    (build_hash, capnp_version_hash)
}

fn make_build_hash<P: AsRef<Path>>(input_path: P) -> Vec<u8> {
    let input_path = input_path.as_ref();
    let lines = std::io::BufReader::new(std::fs::File::open(input_path).unwrap()).lines();
    calculate_hash(lines)
}

fn append_hash_and_desired_capnp_version_hash<P: AsRef<Path>, Q: AsRef<Path>>(
    input_path: P,
    output_path: Q,
) {
    let input_path = input_path.as_ref();
    let output_path = output_path.as_ref();
    let lines = std::io::BufReader::new(std::fs::File::open(input_path).unwrap()).lines();
    let h = calculate_hash(lines);

    let mut out_file = OpenOptions::new().append(true).open(output_path).unwrap();
    writeln!(out_file, "\n//BUILDHASH:{}", hex::encode(h)).unwrap();

    let mut hasher = Sha256::new();
    hasher.update(get_desired_capnp_version_string().as_bytes());
    writeln!(
        out_file,
        "\n//CAPNPDESIREDVERSIONHASH:{}",
        hex::encode(hasher.finalize())
    )
    .unwrap();
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
    // Also append a hash of the desired capnp version to the output file
    append_hash_and_desired_capnp_version_hash("proto/veilid.capnp", "proto/veilid_capnp.rs");
}

// Fix for missing __extenddftf2 on Android x86_64 Emulator
fn fix_android_emulator() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    if target_arch == "x86_64" && target_os == "android" {
        let missing_library = "clang_rt.builtins-x86_64-android";
        let android_home = env::var("ANDROID_HOME")
            .or(env::var("ANDROID_SDK_ROOT"))
            .expect("ANDROID_HOME or ANDROID_SDK_ROOT not set");
        let lib_path = glob(&format!(
            "{android_home}/ndk/26.3.11579264/**/lib{missing_library}.a"
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

#[cfg(feature = "geolocation")]
fn download_file(url: &str, filename: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error>> {
    let content = reqwest::blocking::get(url)?.bytes()?;
    std::fs::write(filename, content)?;
    Ok(())
}

#[cfg(feature = "geolocation")]
fn download_geoip_database_files() -> Result<(), Box<dyn std::error::Error>> {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR")?;
    let target_dir = std::path::PathBuf::from(manifest_dir).join("../target");

    // Source: https://github.com/sapics/ip-location-db
    let files = [
        (
            "https://cdn.jsdelivr.net/npm/@ip-location-db/asn-country-mmdb/asn-country-ipv4.mmdb",
            Path::new(&target_dir).join("ipv4.mmdb"),
        ),
        (
            "https://cdn.jsdelivr.net/npm/@ip-location-db/asn-country-mmdb/asn-country-ipv6.mmdb",
            Path::new(&target_dir).join("ipv6.mmdb"),
        ),
    ];

    for (url, filename) in files {
        if !filename.exists() {
            println!("Downloading {url}");
            download_file(url, &filename)?;
            continue;
        }

        let modified = std::fs::metadata(&filename)?.modified()?;
        let now = std::time::SystemTime::now();
        let time_diff = now.duration_since(modified)?;

        if time_diff > std::time::Duration::from_secs(60 * 60 * 24) {
            println!("Downloading {url}");
            download_file(url, &filename)?;
        }
    }

    Ok(())
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

    #[cfg(feature = "geolocation")]
    download_geoip_database_files().expect("failed to download geoip database files");
}
