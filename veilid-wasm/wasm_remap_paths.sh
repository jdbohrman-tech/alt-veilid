#!/bin/bash

set -eo pipefail

# Path to, but not including, the cargo workspace ("veilid")
WORKSPACE_PARENT=$(dirname $(dirname $(cargo locate-project --workspace --message-format=plain))); \
# Do not include said path in wasm blob output
RUSTFLAGS="--remap-path-prefix=$WORKSPACE_PARENT=/home/user $RUSTFLAGS"; \
# Do not include user home directory in wasm blob output
RUSTFLAGS="--remap-path-prefix=$HOME=/home/user $RUSTFLAGS"; \
# Explicitly mark RUSTFLAGS as an environment variable, so it's passed to cargo
export RUSTFLAGS

# Run the rest of the command line
$@

