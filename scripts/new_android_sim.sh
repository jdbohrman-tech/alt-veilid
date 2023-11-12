#!/bin/bash

UNAME_M=`uname -m`
if [[ "$UNAME_M" == "arm64" ]]; then
    ANDROID_ABI=arm64-v8a
elif [[ "$UNAME_M" == "x86_64" ]]; then
    ANDROID_ABI=x86
else 
    echo "Unknown platform"
    exit 1
fi
AVD_NAME="testavd"
AVD_TAG="google_atd"
AVD_IMAGE="system-images;android-30;$AVD_TAG;$ANDROID_ABI"
AVD_DEVICE="Nexus 10"

SDKMANAGER=$ANDROID_HOME/tools/bin/sdkmanager
AVDMANAGER=$ANDROID_HOME/tools/bin/avdmanager
if ! command -v $SDKMANAGER; then
    SDKMANAGER=$ANDROID_HOME/cmdline-tools/latest/bin/sdkmanager
    AVDMANAGER=$ANDROID_HOME/cmdline-tools/latest/bin/avdmanager
    if ! command -v $SDKMANAGER; then
        echo "Can't find 'sdkmanager' in the usual places."
        exit
    fi
fi
EMULATOR=$ANDROID_HOME/emulator/emulator
if ! command -v $EMULATOR; then
    echo "Can't find 'emulator' in the usual places."
    exit
fi

# Install AVD image
$SDKMANAGER --install "$AVD_IMAGE"
# Make AVD
echo "no" | $AVDMANAGER --verbose create avd --force --name "$AVD_NAME" --package "$AVD_IMAGE" --tag "$AVD_TAG" --abi "$ANDROID_ABI" --device "$AVD_DEVICE"
# Run emulator
$ANDROID_HOME/emulator/emulator -avd testavd -no-snapshot -no-boot-anim -no-window &
( trap exit SIGINT ; read -r -d '' _ </dev/tty ) ## wait for Ctrl-C
kill %1
wait
