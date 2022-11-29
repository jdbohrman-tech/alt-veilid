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
# Install AVD image
$ANDROID_SDK_ROOT/tools/bin/sdkmanager --install "$AVD_IMAGE"
# Make AVD
echo "no" | $ANDROID_SDK_ROOT/tools/bin/avdmanager --verbose create avd --force --name "$AVD_NAME" --package "$AVD_IMAGE" --tag "$AVD_TAG" --abi "$ANDROID_ABI" --device "$AVD_DEVICE"
# Run emulator
$ANDROID_SDK_ROOT/emulator/emulator -avd testavd -no-snapshot -no-boot-anim -no-window &
( trap exit SIGINT ; read -r -d '' _ </dev/tty ) ## wait for Ctrl-C
kill %1
wait
