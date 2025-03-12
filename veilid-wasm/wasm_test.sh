#!/bin/bash
set -eo pipefail

SCRIPTDIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
pushd "$SCRIPTDIR" &> /dev/null

WASM_PACK_FLAGS="--dev"
if [[ "$1" == "release" ]]; then
  WASM_PACK_FLAGS="--release"
  shift
fi

TEST_COMMAND="npm run test:headless"
if [[ "$1" == "interactive" ]]; then
  TEST_COMMAND="npm run test"
  shift
fi

# Build wasm into an npm package, output into ./pkg
wasm-pack build $WASM_PACK_FLAGS --target bundler --weak-refs

# Install test deps and run test suite
cd tests
npm install
original_tmpdir=$TMPDIR
mkdir -p ~/tmp
export TMPDIR=~/tmp
$TEST_COMMAND $@
export TMPDIR=$original_tmpdir

popd &> /dev/null
