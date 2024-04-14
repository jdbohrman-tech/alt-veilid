#!/bin/bash
set -eo pipefail

if [ $(id -u) -eq 0 ]; then
  echo "Don't run this as root"
  exit
fi

SCRIPTDIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"

if [[ "$(uname)" != "Linux" ]]; then
  echo Not running Linux
  exit 1
fi

if ! lsb_release -d | grep -qEi 'debian|buntu|mint|pop\!\_os' && [ -z "$(command -v dnf)" ]; then
  echo Not a supported Linux
  exit 1
fi

while true; do
  read -p "Did you install Android SDK? Y/N " response

  case $response in
  [yY])
    echo Checking android setup...

    # ensure ANDROID_HOME is defined and exists
    if [ -d "$ANDROID_HOME" ]; then
      echo '[X] $ANDROID_HOME is defined and exists'
    else
      echo '$ANDROID_HOME is not defined or does not exist'
      exit 1
    fi

    # ensure Android Command Line Tools exist
    if [ -d "$ANDROID_HOME/cmdline-tools/latest/bin" ]; then
      echo '[X] Android command line tools are installed'
    else
      echo 'Android command line tools are not installed'
      exit 1
    fi

    # ensure ndk is installed
    ANDROID_NDK_HOME="$ANDROID_HOME/ndk/26.3.11579264"
    if [ -f "$ANDROID_NDK_HOME/ndk-build" ]; then
      echo '[X] Android NDK is installed at the location $ANDROID_NDK_HOME'
    else
      echo 'Android NDK is not installed at the location $ANDROID_NDK_HOME'
      exit 1
    fi

    # ensure cmake is installed
    if [ -d "$ANDROID_HOME/cmake" ]; then
      echo '[X] Android SDK CMake is installed'
    else
      echo 'Android SDK CMake is not installed'
      exit 1
    fi

    # ensure emulator is installed
    if [ -d "$ANDROID_HOME/emulator" ]; then
      echo '[X] Android SDK emulator is installed'
    else
      echo 'Android SDK emulator is not installed'
      exit 1
    fi

    # ensure adb is installed
    if command -v adb &>/dev/null; then
      echo '[X] adb is available in the path'
    else
      echo 'adb is not available in the path'
      exit 1
    fi
    break
    ;;
  [nN])
    echo Skipping Android SDK config check...
    break
    ;;
  *) echo invalid response ;;
  esac
done

# ensure rustup is installed
if command -v rustup &>/dev/null; then
  echo '[X] rustup is available in the path'
else
  echo 'rustup is not available in the path'
  exit 1
fi

# ensure cargo is installed
if command -v cargo &>/dev/null; then
  echo '[X] cargo is available in the path'
else
  echo 'cargo is not available in the path'
  exit 1
fi

# ensure pip3 is installed
if command -v pip3 &>/dev/null; then
  echo '[X] pip3 is available in the path'
else
  echo 'pip3 is not available in the path'
  exit 1
fi

# install targets
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android wasm32-unknown-unknown

# install cargo packages
cargo install wasm-bindgen-cli wasm-pack cargo-edit

# install pip packages
pip3 install --upgrade bumpversion

# install capnp
while true; do
  read -p "Will you be modifying the capnproto schema? Y/N (say N if unsure)" response

  case $response in
  [yY])
    echo Installing capnproto...

    # Install capnproto using the same mechanism as our earthly build
    $SCRIPTDIR/../scripts/earthly/install_capnproto.sh

    break
    ;;
  [nN])
    echo Skipping capnproto installation...
    break
    ;;
  *) echo invalid response ;;
  esac
done
