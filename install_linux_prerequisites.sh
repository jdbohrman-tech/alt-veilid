#!/bin/bash
set -eo pipefail

if [ $(id -u) -eq 0 ]; then 
    echo "Don't run this as root"
    exit
fi

# Install APT dependencies
sudo apt update -y
sudo apt install -y openjdk-11-jdk-headless iproute2 curl build-essential cmake libssl-dev openssl file git pkg-config libdbus-1-dev libdbus-glib-1-dev libgirepository1.0-dev libcairo2-dev checkinstall unzip llvm wabt checkinstall

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y -c clippy --profile default
source "$HOME/.cargo/env"

#ask if they want to install optional android sdk (and install if yes)
while true; do
read -p "Do you want to install Android SDK (optional) Y/N) " response

case $response in
[yY] ) echo Installing Android SDK...;
# Install Android SDK
mkdir $HOME/Android; mkdir $HOME/Android/Sdk
curl -o $HOME/Android/cmdline-tools.zip https://dl.google.com/android/repository/commandlinetools-linux-9123335_latest.zip
cd $HOME/Android; unzip $HOME/Android/cmdline-tools.zip
$HOME/Android/cmdline-tools/bin/sdkmanager --sdk_root=$HOME/Android/Sdk build-tools\;33.0.1 ndk\;25.1.8937393 cmake\;3.22.1 platform-tools platforms\;android-33 cmdline-tools\;latest emulator
cd $HOME
rm -rf $HOME/Android/cmdline-tools $HOME/Android/cmdline-tools.zip

# Add environment variables
cat >> $HOME/.profile <<END
source "\$HOME/.cargo/env"
export PATH=\$PATH:\$HOME/Android/Sdk/ndk/25.1.8937393/toolchains/llvm/prebuilt/linux-x86_64/bin:\$HOME/Android/Sdk/platform-tools:\$HOME/Android/Sdk/cmdline-tools/latest/bin
export ANDROID_NDK_HOME=\$HOME/Android/Sdk/ndk/25.1.8937393
export ANDROID_SDK_ROOT=\$HOME/Android/Sdk
END
break ;;
[nN] ) echo Skipping Android SDK;
cat >> $HOME/.profile <<END
source "\$HOME/.cargo/env"
END
break;;

* ) echo invalid response;;
esac
done

echo Complete! Exit and reopen the shell and continue with ./setup_linux.sh
