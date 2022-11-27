#!/bin/bash
if [[ "$1" == "wasm" ]]; then
    WASM_BINDGEN_TEST_TIMEOUT=120 wasm-pack test --chrome --headless
else
    cargo test --features=rt-tokio
    cargo test --features=rt-async-std
    cargo test --features=rt-tokio,log --no-default-features
    cargo test --features=rt-async-std,log --no-default-features
fi
