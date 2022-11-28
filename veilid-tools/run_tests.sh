#!/bin/bash
SCRIPTDIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"

pushd $SCRIPTDIR 2>/dev/null
if [[ "$1" == "wasm" ]]; then
    WASM_BINDGEN_TEST_TIMEOUT=120 wasm-pack test --chrome --headless
elif [[ "$1" == "ios" ]]; then
    SYMROOT=/tmp/testout
    APPNAME=veilidtools-tests
    BUNDLENAME=com.veilid.veilidtools-tests

    xcrun xcodebuild -project src/tests/ios/$APPNAME/$APPNAME.xcodeproj/ -scheme $APPNAME -destination "generic/platform=iOS Simulator" SYMROOT=$SYMROOT
    ID=$(xcrun simctl create test-iphone com.apple.CoreSimulator.SimDeviceType.iPhone-14-Pro com.apple.CoreSimulator.SimRuntime.iOS-16-1 2>/dev/null)
    xcrun simctl boot $ID
    xcrun simctl bootstatus $ID
    xcrun simctl install $ID $SYMROOT/Debug-iphonesimulator/$APPNAME.app
    xcrun simctl launch --console $ID $BUNDLENAME
    xcrun simctl delete all
    rm -rf /tmp/testout
else
    cargo test --features=rt-tokio
    cargo test --features=rt-async-std
    cargo test --features=rt-tokio,log --no-default-features
    cargo test --features=rt-async-std,log --no-default-features
fi
popd 2>/dev/null