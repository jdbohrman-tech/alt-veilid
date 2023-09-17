use std::path::PathBuf;
use std::process::{Command, Stdio};

fn search_file<T: AsRef<str>, P: AsRef<str>>(start: T, name: P) -> Option<PathBuf> {
    let start_path = PathBuf::from(start.as_ref()).canonicalize().ok();
    let mut path = start_path.as_deref();
    while let Some(some_path) = path {
        let file_path = some_path.join(name.as_ref());
        if file_path.exists() {
            return Some(file_path.to_owned());
        }
        path = some_path.parent();
    }
    None
}

fn get_desired_capnp_version_string() -> String {
    let capnp_path = search_file(env!("CARGO_MANIFEST_DIR"), ".capnp_version")
        .expect("should find .capnp_version file");
    std::fs::read_to_string(&capnp_path)
        .unwrap_or_else(|_| panic!("can't read .capnp_version file here: {:?}",
            capnp_path))
        .trim()
        .to_owned()
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

fn get_desired_protoc_version_string() -> String {
    let protoc_path = search_file(env!("CARGO_MANIFEST_DIR"), ".protoc_version")
        .expect("should find .protoc_version file");
    std::fs::read_to_string(&protoc_path)
        .unwrap_or_else(|_| panic!("can't read .protoc_version file here: {:?}",
            protoc_path))
        .trim()
        .to_owned()
}

fn get_protoc_version_string() -> String {
    let output = Command::new("protoc")
        .arg("--version")
        .stdout(Stdio::piped())
        .output()
        .expect("protoc was not in the PATH");
    let s = String::from_utf8(output.stdout)
        .expect("'protoc --version' output was not a valid string")
        .trim()
        .to_owned();

    if !s.starts_with("libprotoc ") {
        panic!("invalid protoc version string: {}", s);
    }
    s[10..].to_owned()
}

fn main() {
    let desired_capnp_version_string = get_desired_capnp_version_string();
    let capnp_version_string = get_capnp_version_string();
    let desired_protoc_version_string = get_desired_protoc_version_string();
    let protoc_version_string = get_protoc_version_string();

    // Check capnp version
    let desired_capnp_major_version =
        usize::from_str_radix(desired_capnp_version_string.split_once('.').unwrap().0, 10)
            .expect("should be valid int");

    if usize::from_str_radix(capnp_version_string.split_once('.').unwrap().0, 10)
        .expect("should be valid int")
        != desired_capnp_major_version
    {
        panic!(
            "capnproto version should be major version 1, preferably {} but is {}",
            desired_capnp_version_string, capnp_version_string
        );
    } else if capnp_version_string != desired_capnp_version_string {
        println!(
            "capnproto version may be untested: {}",
            capnp_version_string
        );
    }

    // Check protoc version
    let desired_protoc_major_version =
        usize::from_str_radix(desired_protoc_version_string.split_once('.').unwrap().0, 10)
            .expect("should be valid int");
    if usize::from_str_radix(protoc_version_string.split_once('.').unwrap().0, 10)
        .expect("should be valid int")
        < desired_protoc_major_version
    {
        panic!(
            "protoc version should be at least major version {} but is {}",
            desired_protoc_major_version, protoc_version_string
        );
    } else if protoc_version_string != desired_protoc_version_string {
        println!("protoc version may be untested: {}", protoc_version_string);
    }

    ::capnpc::CompilerCommand::new()
        .file("proto/veilid.capnp")
        .run()
        .expect("compiling schema");
}
