#!/bin/bash
SCRIPTDIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"

set -eo pipefail

get_abs_filename() {
    # $1 : relative filename
    echo "$(cd "$(dirname "$1")" && pwd)/$(basename "$1")"
}

pushd $SCRIPTDIR &> /dev/null

if [ -f /usr/local/opt/llvm/bin/llvm-dwarfdump ]; then
    DWARFDUMP=/usr/local/opt/llvm/bin/llvm-dwarfdump
elif [ -f /opt/homebrew/llvm/bin/llvm-dwarfdump ]; then
    DWARFDUMP=/opt/homebrew/llvm/bin/llvm-dwarfdump
else 
    DWARFDUMP=`which llvm-dwarfdump`
    if [[ "${DWARFDUMP}" == "" ]]; then
        echo llvm-dwarfdump not found
    fi
fi


if [[ "$1" == "release" ]]; then  
    OUTPUTDIR=../target/wasm32-unknown-unknown/release/pkg
    INPUTDIR=../target/wasm32-unknown-unknown/release

    # Path to, but not including, the cargo workspace ("veilid")
    WORKSPACE_PARENT=$(dirname $(dirname $(cargo locate-project --workspace --message-format=plain)))
    # Do not include said path in wasm blob output
    RUSTFLAGS="--remap-path-prefix=$WORKSPACE_PARENT=/home/user $RUSTFLAGS"
    # Do not include user home directory in wasm blob output
    RUSTFLAGS="--remap-path-prefix=$HOME=/home/user $RUSTFLAGS"
    # Explicitly mark RUSTFLAGS as an environment variable, so it's passed to cargo
    export RUSTFLAGS

    cargo build --target wasm32-unknown-unknown --release
    mkdir -p $OUTPUTDIR
    wasm-bindgen --out-dir $OUTPUTDIR --target web --weak-refs $INPUTDIR/veilid_wasm.wasm
    wasm-strip $OUTPUTDIR/veilid_wasm_bg.wasm
else
    OUTPUTDIR=../target/wasm32-unknown-unknown/debug/pkg
    INPUTDIR=../target/wasm32-unknown-unknown/debug

    RUSTFLAGS="-O -g $RUSTFLAGS" cargo build --target wasm32-unknown-unknown
    mkdir -p $OUTPUTDIR
    wasm-bindgen --out-dir $OUTPUTDIR --target web --weak-refs --keep-debug --debug $INPUTDIR/veilid_wasm.wasm
    ./wasm-sourcemap.py $OUTPUTDIR/veilid_wasm_bg.wasm -o $OUTPUTDIR/veilid_wasm_bg.wasm.map --dwarfdump $DWARFDUMP
    # wasm-strip $OUTPUTDIR/veilid_wasm_bg.wasm
fi

popd &> /dev/null

# Print for use with scripts
echo SUCCESS:OUTPUTDIR=$(get_abs_filename $OUTPUTDIR)
