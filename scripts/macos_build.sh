#!/bin/bash
SCRIPTDIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
pushd $SCRIPTDIR >/dev/null

CARGO=`which cargo`
CARGO=${CARGO:=~/.cargo/bin/cargo}
CARGO_DIR=$(dirname $CARGO)

CARGO_MANIFEST_PATH=$(python3 -c "import os; import json; print(json.loads(os.popen('$CARGO locate-project').read())['root'])")
CARGO_WORKSPACE_PATH=$(python3 -c "import os; import json; print(json.loads(os.popen('$CARGO locate-project --workspace').read())['root'])")
TARGET_PATH=$(python3 -c "import os; print(os.path.realpath(\"$CARGO_WORKSPACE_PATH/../target\"))")
PACKAGE_NAME=$1
shift

if [ "$CONFIGURATION" == "Debug" ]; then 
    EXTRA_CARGO_OPTIONS="$@"
    BUILD_MODE="debug"
else
    EXTRA_CARGO_OPTIONS="$@ --release"
    BUILD_MODE="release"
fi
ARCHS=${ARCHS:=arm64}

LIPO_OUT_NAME="lipo-darwin"

for arch in $ARCHS
do
    if [ "$arch" == "arm64" ]; then
        echo arm64
        CARGO_TARGET=aarch64-apple-darwin
        CARGO_TOOLCHAIN=
    elif [ "$arch" == "x86_64" ]; then
        echo x86_64
        CARGO_TARGET=x86_64-apple-darwin
        CARGO_TOOLCHAIN=
    else
        echo Unsupported ARCH: $arch
        continue
    fi

    # Choose arm64 brew for unit tests by default if we are on M1
    if [ -f /opt/homebrew/bin/brew ]; then
        HOMEBREW_DIR=/opt/homebrew/bin
    elif [ -f /usr/local/bin/brew ]; then
        HOMEBREW_DIR=/usr/local/bin
    else 
        HOMEBREW_DIR=$(dirname `which brew`)
    fi

    env -i PATH=/usr/bin:/bin:$HOMEBREW_DIR:$CARGO_DIR HOME="$HOME" USER="$USER" MACOSX_DEPLOYMENT_TARGET="$MACOSX_DEPLOYMENT_TARGET" cargo $CARGO_TOOLCHAIN build $EXTRA_CARGO_OPTIONS --target $CARGO_TARGET --manifest-path $CARGO_MANIFEST_PATH

    LIPOS="$LIPOS $TARGET_PATH/$CARGO_TARGET/$BUILD_MODE/lib$PACKAGE_NAME.dylib"

done

# Make lipo build
mkdir -p "$TARGET_PATH/$LIPO_OUT_NAME/$BUILD_MODE/"
lipo $LIPOS -create -output "$TARGET_PATH/$LIPO_OUT_NAME/$BUILD_MODE/lib$PACKAGE_NAME.dylib"

# Make most recent dylib available without build mode for flutter
cp "$TARGET_PATH/$LIPO_OUT_NAME/$BUILD_MODE/lib$PACKAGE_NAME.dylib" "$TARGET_PATH/$LIPO_OUT_NAME/lib$PACKAGE_NAME.dylib"

popd > /dev/null