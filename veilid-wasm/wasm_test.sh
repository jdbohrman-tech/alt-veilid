#!/bin/bash
set -eo pipefail

WASM_BINDGEN_USE_NO_MODULE=true wasm-pack test --firefox "$@"
