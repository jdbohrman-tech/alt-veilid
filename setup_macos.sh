#!/bin/bash
SCRIPTDIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"

if [ ! "$(uname)" == "Darwin" ]; then
    echo Not running on MacOS
    exit 1
fi

# ensure ANDROID_SDK_ROOT is defined and exists
if [ -d "$ANDROID_SDK_ROOT" ]; then
    echo '[X] $ANDROID_SDK_ROOT is defined and exists' 
else
    echo '$ANDROID_SDK_ROOT is not defined or does not exist'
    exit 1
fi

# ensure ANDROID_NDK_HOME is defined and exists
if [ -d "$ANDROID_NDK_HOME" ]; then
    echo '[X] $ANDROID_NDK_HOME is defined and exists' 
else
    echo '$ANDROID_NDK_HOME is not defined or does not exist'
    exit 1
fi

# ensure ndk is installed
if [ -f "$ANDROID_NDK_HOME/ndk-build" ]; then
    echo '[X] Android NDK is installed at the location $ANDROID_NDK_HOME' 
else
    echo 'Android NDK is not installed at the location $ANDROID_NDK_HOME'
    exit 1
fi

# ensure cmake is installed
if [ -d "$ANDROID_SDK_ROOT/cmake" ]; then
    echo '[X] Android SDK CMake is installed' 
else
    echo 'Android SDK CMake is not installed'
    exit 1
fi

# ensure emulator is installed
if [ -d "$ANDROID_SDK_ROOT/emulator" ]; then
    echo '[X] Android SDK emulator is installed' 
else
    echo 'Android SDK emulator is not installed'
    exit 1
fi

# ensure adb is installed
if command -v adb &> /dev/null; then 
    echo '[X] adb is available in the path'
else
    echo 'adb is not available in the path'
    exit 1
fi

# ensure brew is installed
if command -v brew &> /dev/null; then 
    echo '[X] brew is available in the path'
else
    echo 'brew is not available in the path'
    exit 1
fi

# ensure xcode is installed
if command -v xcode-select &> /dev/null; then 
    echo '[X] XCode is available in the path'
else
    echo 'XCode is not available in the path'
    exit 1
fi

# ensure rustup is installed
if command -v rustup &> /dev/null; then 
    echo '[X] rustup is available in the path'
else
    echo 'rustup is not available in the path'
    exit 1
fi

# ensure cargo is installed
if command -v cargo &> /dev/null; then 
    echo '[X] cargo is available in the path'
else
    echo 'cargo is not available in the path'
    exit 1
fi

# install targets
rustup target add aarch64-apple-darwin aarch64-apple-ios x86_64-apple-darwin x86_64-apple-ios wasm32-unknown-unknown aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android

# install cargo packages
cargo install wasm-bindgen-cli

# install bitcode compatible ios toolchain
# echo Manual Step:
# echo   install +ios-arm64-1.57.0 toolchain for bitcode from https://github.com/getditto/rust-bitcode/releases/latest and unzip
# echo   xattr -d -r com.apple.quarantine .
# echo   ./install.sh

# ensure we have command line tools
xcode-select --install

# ensure packages are installed
if [ "$BREW_USER" == "" ]; then
    if [ -d /opt/homebrew ]; then
        BREW_USER=`ls -lad /opt/homebrew/. | cut -d\  -f4`
        echo "Must sudo to homebrew user \"$BREW_USER\" to install capnp package:"
    elif [ -d /usr/local/Homebrew ]; then
        BREW_USER=`ls -lad /usr/local/Homebrew/. | cut -d\  -f4`
        echo "Must sudo to homebrew user \"$BREW_USER\" to install capnp package:"
    else
        echo "Homebrew is not installed in the normal place. Trying as current user"
        BREW_USER=`whoami`
    fi
fi
sudo -H -u $BREW_USER brew install capnp cmake wabt llvm protobuf

