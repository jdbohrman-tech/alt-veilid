# Veilid Development

[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg)](code_of_conduct.md)

## Introduction

This guide covers setting up environments for core, Flutter/Dart, and Python development. See the relevent sections.

## Obtaining the source code

```shell
git clone https://gitlab.com/veilid/veilid.git
```

## Dependencies

### GNU/Linux

Development of Veilid on GNU/Linux requires a Debian variant such as Debian
itself, Ubuntu or Mint. Pull requests to support other distributions would be
welcome!

Running the setup script requires:

* Rust

#### Optionally, to build for Android:

* Android SDK and NDK

You may decide to use Android Studio [here](https://developer.android.com/studio)
to maintain your Android dependencies. If so, use the dependency manager
within your IDE. If you plan on using Flutter for Veilid development, the Android Studio
method is highly recommended as you may run into path problems with the 'flutter'
command line without it. If you do so, you may skip to
[Run Veilid setup script](#Run Veilid setup script).

* build-tools;33.0.1
* ndk;25.1.8937393
* cmake;3.22.1
* platform-tools
* platforms;android-33

#### Setup Dependencies using the CLI

You can automatically install the prerequisites using this script:

```shell
./dev-setup/install_linux_prerequisites.sh
```

Otherwise, you may choose to use Android `sdkmanager`. Follow the installation
instructions for `sdkmanager`
[here](https://developer.android.com/studio/command-line/sdkmanager), then use
the command line to install the requisite package versions:

```shell
sdkmanager --install "platform-tools"
sdkmanager --install "platforms;android-33"
sdkmanager --install "build-tools;33.0.1"
sdkmanager --install "ndk;25.1.8937393"
sdkmanager --install "cmake;3.22.1"
```

Export environment variables and add the Android SDK platform-tools directory to
your path.

```shell
cat << EOF >> ~/.profile 
export ANDROID_SDK_ROOT=<path to sdk>
export ANDROID_NDK_HOME=$ANDROID_SDK_ROOT/ndk/25.1.8937393
export PATH=\$PATH:$ANDROID_SDK_ROOT/platform-tools
EOF
```

#### Run Veilid setup script

Now you may run the Linux setup script to check your development environment and
pull the remaining Rust dependencies:

```shell
./dev-setup/setup_linux.sh
```

#### Run the veilid-flutter setup script (optional)

If you are developing Flutter applications or the flutter-veilid portion, you should
install Android Studio, and run the flutter setup script:

```shell
cd veilid-flutter
./setup_flutter.sh
```

### macOS

Development of Veilid on MacOS is possible on both Intel and ARM hardware.

Development requires:

* Xcode, preferably latest version
* Homebrew [here](https://brew.sh)
* Rust

#### Optionally, to build for Android:

* Android Studio
* Android SDK and NDK

You will need to use Android Studio [here](https://developer.android.com/studio)
to maintain your Android dependencies. Use the SDK Manager in the IDE to install the following packages (use package details view to select version):

* Android SDK Build Tools (33.0.1)
* NDK (Side-by-side) (25.1.8937393)
* Cmake (3.22.1)
* Android SDK 33
* Android SDK Command Line Tools (latest) (7.0/latest)

#### Setup command line environment

Export environment variables and add the Android SDK platform-tools directory to
your path.

```shell
cat << EOF >> ~/.zshenv
export ANDROID_SDK_ROOT=$HOME/Library/Android/sdk
export ANDROID_NDK_HOME=$HOME/Library/Android/sdk/ndk/25.1.8937393
export PATH=\$PATH:$HOME/Library/Android/sdk/platform-tools
EOF
```

#### Run Veilid setup script (macOS)

Now you may run the MacOS setup script to check your development environment and
pull the remaining Rust dependencies:

```shell
./dev-setup/setup_macos.sh
```

#### Run the veilid-flutter setup script (optional) (macOS)

If you are developing Flutter applications or the flutter-veilid portion, you should
install Android Studio, and run the flutter setup script:

```shell
cd veilid-flutter
./setup_flutter.sh
```

### Windows

For a simple installation allowing Rust development, follow these steps:

Install Git from <https://git-scm.com/download/win>

Install Rust from <https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe> (this may prompt you to run the Visual Studio Installer, and reboot, before proceeding).

Optional (only if modifying capnp schema): Ensure that capnp.exe (for Cap’n Proto 1.0.1) is in a directory in your path. For example, it can be obtained from the <https://capnproto.org/capnproto-c++-win32-1.0.1.zip> distribution. Please note that the Windows Package Manager Community Repository (i.e., winget) as of 2023-09-15 has version 0.10.4, which is not sufficient.

Start a Command Prompt window.

```shell
git clone https://gitlab.com/veilid/veilid.git
cd veilid
cargo build
cd target
cd debug
veilid-server.exe
```

This may result in a popup window from local security software, asking whether you wish to allow network access by veilid-server.exe.

To use the CLI, navigate to the above `debug` directory in another Command Prompt window (while veilid-server.exe is still running), and type:

```shell
veilid-cli.exe
```

## Running the Application(s)

### Veilid Server

In order to run the `veilid-server` locally:

```shell
cd ./veilid-server
cargo run
```

In order to see what options are available:

```shell
cargo run -- --help
```

#### Configuration

`veilid-server` has a wealth of configuration options. Further documentation on
the format of the `veilid-server.conf` file may be found [in the project /doc
directory](./doc/config/veilid-server-config.md).

When running `veilid-server` in a Unix-like environment, the application will
look for its config file under `/etc/veilid-server/`. If the config file is not
found in this location, `veilid-server` will follow the XDG user directory spec
and look in `~/.config/veilid-server`.

When running under Windows, the `veilid-server.conf` file may be created at
`C:\Users\<user>\AppData\Roaming\Veilid\Veilid\`, and when running under macOS,
at `/Users/<user>/Library/Application Support/org.Veilid.Veilid`.

### Veilid CLI

In order to connect to your local `veilid-server`:

```shell
cd ./veilid-cli
cargo run
```

Similar to `veilid-server`, you may see CLI options by typing:

```shell
cargo run -- --help
```

## Building the Application

### Linux Packages

Veilid server and cli can be built locally using the
[Earthly](https://earthly.dev/) framework. After [installing earthly on your
local machine](https://earthly.dev/get-earthly), you may use the `earthly` cli
to initiate a build:

```shell
earthly +package-linux
```

This will assemble all dependencies and build `.deb` packages for both amd64 and
arm64 platforms. Earthly, built on Docker, caches build layers, so after a
longer first build, subsequent builds should be much quicker.

During development, you may want to kick off specific build steps. To see a list
of the build steps configured, consult the `Earthfile`, or you may use the
`earthly` cli:

```shell
earthly ls
```
