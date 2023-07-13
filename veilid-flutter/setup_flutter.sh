#!/bin/bash
SCRIPTDIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"

OS="unknown"
if [ "$(uname)" == "Linux" ]; then
    if [ ! "$(grep -Ei 'debian|buntu|mint' /etc/*release)" ]; then
        echo Not a supported Linux
        exit 1
    fi
    OS="linux"
elif [ "$(uname)" == "Darwin" ]; then
    OS="macos"
fi
if [ "$OS" == "unknown" ]; then
    echo "Not a supported operating system for this script"
    exit 1
fi

# ensure flutter is installed
if command -v flutter &> /dev/null; then 
    echo '[X] Flutter is available in the path'
else
    echo 'Flutter is not available in the path, install Flutter from here: https://docs.flutter.dev/get-started/install'
    exit 1
fi

# ensure dart is installed
if command -v dart &> /dev/null; then 
    echo '[X] Dart is available in the path'
else
    echo 'Dart is not available in the path, check your environment variables and that Flutter is installed correctly'
    exit 1
fi

# ensure cargo is installed
if command -v cargo &> /dev/null; then 
    echo '[X] Cargo is available in the path'
else
    echo 'Cargo is not available in the path, ensure Rust is installed correctly'
    exit 1
fi

# # install cargo cbindgen
# cargo install cbindgen

# # install dart ffigen
# dart pub global activate ffigen

# # install flutter_rust_bridge_codegen
# cargo install flutter_rust_bridge_codegen

# platform specific stuff
if [ "$OS" == "linux" ]; then
    # # ensure packages are installed
    # echo "Must sudo to root to install LLVM package:"
    # sudo apt-get install libclang-dev
    
    # ensure platforms are enabled in flutter
    flutter config --enable-linux-desktop --enable-android

elif [ "$OS" == "macos" ]; then

    # # ensure x86_64 homebrew is installed
    # if [ -f /usr/local/bin/brew ] &> /dev/null; then 
    #     echo '[X] x86_64 homebrew is available'
    # else
    #     echo 'x86_64 homebrew is not available, run this:'
    #     echo 'arch -x86_64 zsh'
    #     echo '/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"'
    #     exit 1
    # fi

    # ensure packages are installed
    # if [ "$BREW_USER" == "" ]; then
    #     BREW_USER=`ls -lad /usr/local/bin/. | cut -d\  -f4`
    #     echo "Must sudo to homebrew user \"$BREW_USER\" to install LLVM package:"
    # fi
    # sudo -H -u $BREW_USER arch -x86_64 /usr/local/bin/brew install llvm
    echo "Must sudo to root to install CocoaPods gem:"
    # sudo arch -x86_64 gem install ffi
    sudo arch -x86_64 gem install cocoapods

    if [ "$(uname -p)" == "arm" ]; then
        sudo softwareupdate --install-rosetta --agree-to-license
    fi

    # ensure platforms are enabled in flutter
    flutter config --enable-macos-desktop --enable-ios --enable-android
fi

# turn off analytics
flutter config --no-analytics
dart --disable-analytics

# run flutter doctor
flutter doctor -v
