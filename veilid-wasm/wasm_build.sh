#!/bin/bash
SCRIPTDIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"

set -eo pipefail

get_abs_filename() {
    # $1 : relative filename
    echo "$(cd "$(dirname "$1")" && pwd)/$(basename "$1")"
}

pushd $SCRIPTDIR &> /dev/null

if [[ "$1" == "release" ]]; then  
    OUTPUTDIR=$SCRIPTDIR/../target/wasm32-unknown-unknown/release/pkg
    INPUTDIR=$SCRIPTDIR/../target/wasm32-unknown-unknown/release

    ./wasm_remap_paths.sh cargo build --target wasm32-unknown-unknown --release
    mkdir -p $OUTPUTDIR
    wasm-bindgen --out-dir $OUTPUTDIR --target web --weak-refs $INPUTDIR/veilid_wasm.wasm
    wasm-tools strip $OUTPUTDIR/veilid_wasm_bg.wasm -o $OUTPUTDIR/veilid_wasm_bg.wasm.stripped
    mv $OUTPUTDIR/veilid_wasm_bg.wasm.stripped $OUTPUTDIR/veilid_wasm_bg.wasm
else
    OUTPUTDIR=$SCRIPTDIR/../target/wasm32-unknown-unknown/debug/pkg
    INPUTDIR=$SCRIPTDIR/../target/wasm32-unknown-unknown/debug

    RUSTFLAGS="-O -g $RUSTFLAGS" cargo build --target wasm32-unknown-unknown
    mkdir -p $OUTPUTDIR
    wasm-bindgen --out-dir $OUTPUTDIR --target web --weak-refs --keep-debug --debug $INPUTDIR/veilid_wasm.wasm
fi

popd &> /dev/null

# Print for use with scripts
echo SUCCESS:OUTPUTDIR=$(get_abs_filename $OUTPUTDIR)
