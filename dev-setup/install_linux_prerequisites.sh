#!/bin/bash
set -eo pipefail

if [ $(id -u) -eq 0 ]; then 
    echo "Don't run this as root"
    exit
fi

if [ ! -z "$(command -v apt)" ]; then
    # Install APT dependencies
    sudo apt update -y
    sudo apt install -y openjdk-17-jdk-headless iproute2 curl build-essential cmake libssl-dev openssl file git pkg-config libdbus-1-dev libdbus-glib-1-dev libgirepository1.0-dev libcairo2-dev checkinstall unzip llvm wabt python3-pip
elif [ ! -z "$(command -v dnf)" ]; then
    # DNF (formerly yum)
    sudo dnf update -y
    # libgirepository -> gobject-introspection
    # iproute2 -> iproute
    # openjdk-17-jdk-headless -> java-11-openjdk-headless
    # checkinstall does not appear to be a thing in Fedora 38 repos
    #
    # Seems like iproute and file might come preinstalled but I put
    # them in anyway
    #
    # Also Fedora doesn't come with pip
    sudo dnf install -y java-17-openjdk-headless iproute curl cmake openssl-devel openssl git file pkg-config dbus-devel dbus-glib gobject-introspection-devel cairo-devel unzip llvm wabt python3-pip gcc-c++
    # build-essentials
    sudo dnf groupinstall -y 'Development Tools'
fi


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
export ANDROID_HOME=\$HOME/Android/Sdk
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
