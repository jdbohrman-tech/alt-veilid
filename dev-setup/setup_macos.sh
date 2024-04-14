#!/bin/bash
set -eo pipefail

SCRIPTDIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"

if [ ! "$(uname)" == "Darwin" ]; then
	echo Not running on MacOS
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

		# ensure Android SDK packages are installed
		$ANDROID_HOME/cmdline-tools/latest/bin/sdkmanager build-tools\;34.0.0 ndk\;26.3.11579264 cmake\;3.22.1 platform-tools platforms\;android-34

		# ensure ANDROID_NDK_HOME is defined and exists
		ANDROID_NDK_HOME="$ANDROID_HOME/ndk/26.3.11579264"
		if [ -d "$ANDROID_NDK_HOME" ]; then
			echo '[X] Android NDK is defined and exists'
		else
			echo 'Android NDK is not defined or does not exist'
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

# ensure brew is installed
if command -v brew &>/dev/null; then
	echo '[X] brew is available in the path'
else
	echo 'brew is not available in the path'
	exit 1
fi

# ensure xcode is installed
if command -v xcode-select &>/dev/null; then
	echo '[X] XCode is available in the path'
else
	echo 'XCode is not available in the path'
	exit 1
fi

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

# ensure Java 17 is the active version
JAVA_VERSION=$(java -version 2>&1 | head -n 1 | cut -d\" -f2)
if [ "$JAVA_VERSION" == "17" ]; then
	echo '[X] Java 17 is available in the path'
else
	echo 'Java 17 is not available in the path'
	exit 1
fi

# ensure we have command line tools
xcode-select --install 2>/dev/null || true
until [ -d /Library/Developer/CommandLineTools/usr/bin ]; do
	sleep 5
done

# install packages
# if $BREW_USER is set, run brew as that user, otherwise run it regularly
# this allows for developers who have brew installed as a different user to run this script
if [ -z "$BREW_USER" ]; then
	BREW_COMMAND="brew"
else
	BREW_COMMAND="sudo -H -u $BREW_USER brew"
fi

$BREW_COMMAND install capnp cmake wabt llvm jq

# install targets
rustup target add aarch64-apple-darwin aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-darwin x86_64-apple-ios wasm32-unknown-unknown aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android

# install cargo packages
cargo install wasm-bindgen-cli wasm-pack cargo-edit

# install pip packages
pip3 install --upgrade bumpversion

if command -v pod &>/dev/null; then
	echo '[X] CocoaPods is available in the path'
else
	echo 'CocoaPods is not available in the path, installing it now'
	$BREW_COMMAND install cocoapods
fi
