#!/bin/bash
SCRIPTDIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
pushd $SCRIPTDIR/.. >/dev/null

cargo-zigbuild clippy --target x86_64-unknown-linux-gnu
cargo-zigbuild clippy --target x86_64-unknown-linux-gnu  --manifest-path=veilid-server/Cargo.toml --no-default-features --features=default-async-std
cargo-zigbuild clippy --target x86_64-pc-windows-gnu
cargo-zigbuild clippy --target aarch64-apple-darwin
cargo clippy --manifest-path=veilid-wasm/Cargo.toml --target wasm32-unknown-unknown

popd >/dev/null