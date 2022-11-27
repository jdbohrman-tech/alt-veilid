#!/bin/bash

SCRIPTDIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
CARGO_MANIFEST_PATH=$(python -c "import os; print(os.path.realpath(\"$SCRIPTDIR/Cargo.toml\"))")
# echo CARGO_MANIFEST_PATH: $CARGO_MANIFEST_PATH 

if [ "$CONFIGURATION" == "Debug" ]; then 
    EXTRA_CARGO_OPTIONS="$@"
else
    EXTRA_CARGO_OPTIONS="$@ --release"
fi
ARCHS=${ARCHS:=arm64}
for arch in $ARCHS
do
    if [ "$arch" == "arm64" ]; then
        echo arm64
        CARGO_TARGET=aarch64-apple-ios
        #CARGO_TOOLCHAIN=+ios-arm64-1.57.0
        CARGO_TOOLCHAIN=
    elif [ "$arch" == "x86_64" ]; then
        echo x86_64
        CARGO_TARGET=x86_64-apple-ios
        CARGO_TOOLCHAIN=
    else
        echo Unsupported ARCH: $arch
        continue
    fi

    CARGO=`which cargo`
    CARGO=${CARGO:=~/.cargo/bin/cargo}
    CARGO_DIR=$(dirname $CARGO)

    # Choose arm64 brew for unit tests by default if we are on M1
    if [ -f /opt/homebrew/bin/brew ]; then
        HOMEBREW_DIR=/opt/homebrew/bin
    elif [ -f /usr/local/bin/brew ]; then
        HOMEBREW_DIR=/usr/local/bin
    else 
        HOMEBREW_DIR=$(dirname `which brew`)
    fi

    env -i PATH=/usr/bin:/bin:$HOMEBREW_DIR:$CARGO_DIR HOME="$HOME" USER="$USER" cargo $CARGO_TOOLCHAIN build $EXTRA_CARGO_OPTIONS --target $CARGO_TARGET --manifest-path $CARGO_MANIFEST_PATH
done

