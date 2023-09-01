#!/bin/bash
SCRIPTDIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"

pushd $SCRIPTDIR 2>/dev/null
if [[ "$1" == "wasm" ]]; then
    WASM_BINDGEN_TEST_TIMEOUT=120 wasm-pack test --firefox --headless --features=rt-wasm-bindgen
elif [[ "$1" == "ios" ]]; then
    SYMROOT=/tmp/testout
    APPNAME=veilidtools-tests
    BUNDLENAME=com.veilid.veilidtools-tests
    ID="$2"
    if [[ "$ID" == "" ]]; then 
        echo "No emulator ID specified"
        exit 1
    fi

    # Build for simulator
    xcrun xcodebuild -project src/tests/ios/$APPNAME/$APPNAME.xcodeproj/ -scheme $APPNAME -destination "generic/platform=iOS Simulator" SYMROOT=$SYMROOT

    # Run in temporary simulator
    xcrun simctl install $ID $SYMROOT/Debug-iphonesimulator/$APPNAME.app
    xcrun simctl spawn $ID log stream --level debug --predicate "subsystem == \"$BUNDLENAME\"" &
    xcrun simctl launch --console $ID $BUNDLENAME
    sleep 1 # Ensure the last log lines print
    kill -INT %1

    # Clean up build output
    rm -rf /tmp/testout

elif [[ "$1" == "android" ]]; then
    ID="$2"
    if [[ "$ID" == "" ]]; then 
        echo "No emulator ID specified"
        exit 1
    fi
    APPNAME=veilid_tools_android_tests
    APPID=com.veilid.veilid_tools_android_tests
    ACTIVITYNAME=MainActivity
    pushd src/tests/android/$APPNAME >/dev/null
    # Build apk
    ./gradlew assembleDebug
    # Wait for boot
    adb -s $ID wait-for-device
    # Install app
    adb -s $ID install -r ./app/build/outputs/apk/debug/app-debug.apk 
    # Start activity
    adb -s $ID shell am start-activity -W $APPID/.$ACTIVITYNAME
    # Get the pid of the program
    APP_PID=`adb -s $ID shell pidof -s $APPID`
    # Print the logcat
    adb -s $ID shell logcat --pid=$APP_PID veilid-tools:V *:S &
    # Wait for the pid to be done
    while [ "$(adb -s $ID shell pidof -s $APPID)" != "" ]; do
        sleep 1
    done
    # Terminate logcat
    kill %1
    # Finished
    popd >/dev/null

else
    cargo test -- --nocapture
    cargo test --features=tracing -- --nocapture
    cargo test --no-default-features --features=rt-async-std -- --nocapture
    cargo test --no-default-features --features=rt-async-std,tracing -- --nocapture
fi
popd 2>/dev/null