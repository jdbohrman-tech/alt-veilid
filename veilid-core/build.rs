use std::process::{Command, Stdio};

const CAPNP_VERSION: &str = "1.0.1"; // Keep in sync with scripts/install_capnp.sh
const PROTOC_VERSION: &str = "24.3"; // Keep in sync with scripts/install_protoc.sh

fn get_desired_capnp_version_string() -> String {
    CAPNP_VERSION.to_string()
}

fn get_desired_protoc_version_string() -> String {
    PROTOC_VERSION.to_string()
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
    #[cfg(doc)]
    return;

    #[cfg(not(doc))]
    {
        let desired_capnp_version_string = get_desired_capnp_version_string();
        let capnp_version_string = get_capnp_version_string();
        let desired_protoc_version_string = get_desired_protoc_version_string();
        let protoc_version_string = get_protoc_version_string();

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
                "capnproto version may be untested: {}",
                capnp_version_string
            );
        }

        // Check protoc version
        let desired_protoc_major_version = desired_protoc_version_string
            .split_once('.')
            .unwrap()
            .0
            .parse::<usize>()
            .expect("should be valid int");
        if protoc_version_string
            .split_once('.')
            .unwrap()
            .0
            .parse::<usize>()
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
}
