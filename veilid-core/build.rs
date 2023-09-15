use std::path::PathBuf;
use std::process::{Command, Stdio};

fn get_workspace_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .canonicalize()
        .expect("want workspace dir")
}
fn get_desired_capnp_version_string() -> String {
    std::fs::read_to_string(get_workspace_dir().join(".capnp_version"))
        .expect("should find .capnp_version file")
        .trim()
        .to_owned()
}

fn get_capnp_version_string() -> String {
    let output = Command::new("capnpc")
        .arg("--version")
        .stdout(Stdio::piped())
        .output()
        .expect("capnpc was not in the PATH");
    let s = String::from_utf8(output.stdout)
        .expect("'capnpc --version' output was not a valid string")
        .trim()
        .to_owned();

    if !s.starts_with("Cap'n Proto version ") {
        panic!("invalid capnpc version string: {}", s);
    }
    s[20..].to_owned()
}

fn get_desired_protoc_version_string() -> String {
    std::fs::read_to_string(get_workspace_dir().join(".protoc_version"))
        .expect("should find .protoc_version file")
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
        usize::from_str_radix(desired_capnp_version_string.split_once(".").unwrap().0, 10)
            .expect("should be valid int");

    if usize::from_str_radix(capnp_version_string.split_once(".").unwrap().0, 10)
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
        usize::from_str_radix(desired_protoc_version_string.split_once(".").unwrap().0, 10)
            .expect("should be valid int");
    if usize::from_str_radix(protoc_version_string.split_once(".").unwrap().0, 10)
        .expect("should be valid int")
        < desired_protoc_major_version
    {
        panic!(
            "capnproto version should be at least major version {} but is {}",
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
