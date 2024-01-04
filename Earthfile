VERSION 0.7

########################################################################################################################
## ARGUMENTS
##
## CI_REGISTRY_IMAGE - used so that forks can refer to themselves, e.g. to use the fork's own registry cache in the
## `+build-linux-cache` target, and defaulting to `registry.gitlab.com/veilid/veilid` if not specified
##
## BASE - tells the build whether it should run in the default mode which runs the complete build, or run by starting
## with the remote `container` value which uses `build-cache:latest` as set up in the projects Container Registry
##
########################################################################################################################

# Start with older Ubuntu to ensure GLIBC symbol versioning support for older linux
# Ensure we are using an amd64 platform because some of these targets use cross-platform tooling
FROM ubuntu:18.04
ENV ZIG_VERSION=0.11.0-dev.3978+711b4e93e
ENV RUSTUP_HOME=/usr/local/rustup
ENV CARGO_HOME=/usr/local/cargo
ENV PATH=$PATH:/usr/local/cargo/bin:/usr/local/zig
ENV LD_LIBRARY_PATH=/usr/local/lib
WORKDIR /veilid

# Install build prerequisites & setup required directories
deps-base:
    RUN apt-get -y update
    RUN apt-get install -y iproute2 curl build-essential cmake libssl-dev openssl file git pkg-config libdbus-1-dev libdbus-glib-1-dev libgirepository1.0-dev libcairo2-dev checkinstall unzip libncursesw5-dev libncurses5-dev

# Install Rust
deps-rust:
    FROM +deps-base
    RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y -c clippy --no-modify-path --profile minimal
    RUN chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
        rustup --version; \
        cargo --version; \
        rustc --version;
    # Linux
    RUN rustup target add x86_64-unknown-linux-gnu
    RUN rustup target add aarch64-unknown-linux-gnu
    # Android
    RUN rustup target add aarch64-linux-android
    RUN rustup target add armv7-linux-androideabi
    RUN rustup target add i686-linux-android
    RUN rustup target add x86_64-linux-android
    # WASM
    RUN rustup target add wasm32-unknown-unknown
    # Caching tool
    RUN cargo install cargo-chef
    # Install Linux cross-platform tooling
    RUN curl -O https://ziglang.org/builds/zig-linux-x86_64-$ZIG_VERSION.tar.xz
    RUN tar -C /usr/local -xJf zig-linux-x86_64-$ZIG_VERSION.tar.xz
    RUN mv /usr/local/zig-linux-x86_64-$ZIG_VERSION /usr/local/zig
    RUN cargo install cargo-zigbuild
    SAVE ARTIFACT $RUSTUP_HOME rustup
    SAVE ARTIFACT $CARGO_HOME cargo
    SAVE ARTIFACT /usr/local/cargo/bin/cargo-zigbuild
    SAVE ARTIFACT /usr/local/zig

# Install android tooling
deps-android:
    FROM +deps-base
    BUILD +deps-rust
    COPY +deps-rust/cargo /usr/local/cargo
    COPY +deps-rust/rustup /usr/local/rustup
    COPY +deps-rust/cargo-zigbuild /usr/local/cargo/bin/cargo-zigbuild
    COPY +deps-rust/zig /usr/local/zig
    RUN apt-get install -y openjdk-9-jdk-headless
    RUN mkdir /Android; mkdir /Android/Sdk
    RUN curl -o /Android/cmdline-tools.zip https://dl.google.com/android/repository/commandlinetools-linux-9123335_latest.zip
    RUN cd /Android; unzip /Android/cmdline-tools.zip
    RUN yes | /Android/cmdline-tools/bin/sdkmanager --sdk_root=/Android/Sdk build-tools\;33.0.1 ndk\;25.1.8937393 cmake\;3.22.1 platform-tools platforms\;android-33 cmdline-tools\;latest
    RUN rm -rf /Android/cmdline-tools
    RUN apt-get clean

# Just linux build not android
deps-linux:
    FROM +deps-base
    BUILD +deps-rust
    COPY +deps-rust/cargo /usr/local/cargo
    COPY +deps-rust/rustup /usr/local/rustup
    COPY +deps-rust/cargo-zigbuild /usr/local/cargo/bin/cargo-zigbuild
    COPY +deps-rust/zig /usr/local/zig

build-linux-cache:
    FROM +deps-linux
    RUN mkdir veilid-cli veilid-core veilid-server veilid-tools veilid-wasm veilid-flutter veilid-flutter/rust
    COPY --dir .cargo scripts Cargo.lock Cargo.toml .
    COPY veilid-cli/Cargo.toml veilid-cli
    COPY veilid-core/Cargo.toml veilid-core
    COPY veilid-server/Cargo.toml veilid-server
    COPY veilid-tools/Cargo.toml veilid-tools
    COPY veilid-flutter/rust/Cargo.lock veilid-flutter/rust/Cargo.toml veilid-flutter/rust
    COPY veilid-wasm/Cargo.toml veilid-wasm
    RUN cat /veilid/scripts/earthly/cargo-linux/config.toml >> .cargo/config.toml
    RUN cargo chef prepare --recipe-path recipe.json
    RUN cargo chef cook --recipe-path recipe.json
    RUN echo $PROJECT_PATH
    SAVE ARTIFACT target
    ARG CI_REGISTRY_IMAGE=registry.gitlab.com/veilid/veilid
    SAVE IMAGE --push $CI_REGISTRY_IMAGE/build-cache:latest

code-linux:
    # This target will either use the full earthly cache of local use (+build-linux-cache), or will use a containerized
    # version of the +build-linux-cache from the registry
    ARG BASE=local
    IF [ "$BASE" = "local" ]
        FROM +build-linux-cache
    ELSE
        ARG CI_REGISTRY_IMAGE=registry.gitlab.com/veilid/veilid
        FROM $CI_REGISTRY_IMAGE/build-cache:latest
        # FROM registry.gitlab.com/veilid/build-cache:latest
    END
    COPY --dir .cargo files scripts veilid-cli veilid-core veilid-server veilid-tools veilid-flutter veilid-wasm Cargo.lock Cargo.toml /veilid

# Code + Linux + Android deps
code-android:
    FROM +deps-android
    COPY --dir .cargo files scripts veilid-cli veilid-core veilid-server veilid-tools veilid-flutter veilid-wasm Cargo.lock Cargo.toml /veilid
    RUN cat /veilid/scripts/earthly/cargo-linux/config.toml >> /veilid/.cargo/config.toml
    RUN cat /veilid/scripts/earthly/cargo-android/config.toml >> /veilid/.cargo/config.toml

# Clippy only
clippy:
    FROM +code-linux
    RUN cargo clippy

# Build
build-release:
    FROM +code-linux
    RUN cargo build --release -p veilid-server -p veilid-cli -p veilid-tools -p veilid-core
    SAVE ARTIFACT ./target/release AS LOCAL ./target/release

build:
    FROM +code-linux
    RUN cargo build -p veilid-server -p veilid-cli -p veilid-tools -p veilid-core
    SAVE ARTIFACT ./target/debug AS LOCAL ./target/debug

build-linux-amd64:
    FROM +code-linux
    RUN cargo zigbuild --target x86_64-unknown-linux-gnu --release -p veilid-server -p veilid-cli -p veilid-tools -p veilid-core
    SAVE ARTIFACT ./target/x86_64-unknown-linux-gnu AS LOCAL ./target/artifacts/x86_64-unknown-linux-gnu

build-linux-amd64-debug:
    FROM +code-linux
    RUN cargo zigbuild --target x86_64-unknown-linux-gnu -p veilid-server -p veilid-cli -p veilid-tools -p veilid-core
    SAVE ARTIFACT ./target/x86_64-unknown-linux-gnu AS LOCAL ./target/artifacts/x86_64-unknown-linux-gnu

build-linux-arm64:
    FROM +code-linux
    RUN cargo zigbuild --target aarch64-unknown-linux-gnu --release -p veilid-server -p veilid-cli -p veilid-tools -p veilid-core
    SAVE ARTIFACT ./target/aarch64-unknown-linux-gnu AS LOCAL ./target/artifacts/aarch64-unknown-linux-gnu

build-android:
    FROM +code-android
    WORKDIR /veilid/veilid-core
    ENV PATH=$PATH:/Android/Sdk/ndk/25.1.8937393/toolchains/llvm/prebuilt/linux-x86_64/bin/
    RUN cargo build --target aarch64-linux-android --release
    RUN cargo build --target armv7-linux-androideabi --release
    RUN cargo build --target i686-linux-android --release
    RUN cargo build --target x86_64-linux-android --release
    WORKDIR /veilid
    SAVE ARTIFACT ./target/aarch64-linux-android AS LOCAL ./target/artifacts/aarch64-linux-android
    SAVE ARTIFACT ./target/armv7-linux-androideabi AS LOCAL ./target/artifacts/armv7-linux-androideabi
    SAVE ARTIFACT ./target/i686-linux-android AS LOCAL ./target/artifacts/i686-linux-android
    SAVE ARTIFACT ./target/x86_64-linux-android AS LOCAL ./target/artifacts/x86_64-linux-android

# Unit tests
unit-tests-linux:
    FROM +code-linux
    ENV RUST_BACKTRACE=1
    RUN cargo test -p veilid-server -p veilid-cli -p veilid-tools -p veilid-core

# TODO: Change t0 cross so that they work on any platform
unit-tests-linux-amd64:
    FROM +code-linux
    ENV RUST_BACKTRACE=1
    RUN cargo test --target x86_64-unknown-linux-gnu --release -p veilid-server -p veilid-cli -p veilid-tools -p veilid-core

unit-tests-linux-arm64:
    FROM +code-linux
    ENV RUST_BACKTRACE=1
    RUN cargo test --target aarch64-unknown-linux-gnu --release -p veilid-server -p veilid-cli -p veilid-tools -p veilid-core

# Package 
package-linux-amd64-deb:
    FROM +build-linux-amd64
    #################################
    ### DEBIAN DPKG .DEB FILES
    #################################
    COPY --dir package /veilid
    # veilid-server
    RUN /veilid/package/debian/earthly_make_veilid_server_deb.sh amd64 x86_64-unknown-linux-gnu
    SAVE ARTIFACT --keep-ts /dpkg/out/*.deb AS LOCAL ./target/packages/
    # veilid-cli
    RUN /veilid/package/debian/earthly_make_veilid_cli_deb.sh amd64 x86_64-unknown-linux-gnu
    # save artifacts
    SAVE ARTIFACT --keep-ts /dpkg/out/*.deb AS LOCAL ./target/packages/

package-linux-amd64-rpm:
    FROM --platform amd64 rockylinux:8
    RUN yum install -y createrepo rpm-build rpm-sign yum-utils rpmdevtools
    RUN rpmdev-setuptree
    #################################
    ### RPMBUILD .RPM FILES
    #################################
    RUN mkdir -p /veilid/target
    COPY --dir .cargo files scripts veilid-cli veilid-core veilid-server veilid-tools veilid-flutter veilid-wasm Cargo.lock Cargo.toml package /veilid
    COPY +build-linux-amd64/x86_64-unknown-linux-gnu /veilid/target/x86_64-unknown-linux-gnu
    RUN mkdir -p /rpm-work-dir/veilid-server
    # veilid-server
    RUN veilid/package/rpm/veilid-server/earthly_make_veilid_server_rpm.sh x86_64 x86_64-unknown-linux-gnu
    #SAVE ARTIFACT --keep-ts /root/rpmbuild/RPMS/x86_64/*.rpm AS LOCAL ./target/packages/
    # veilid-cli
    RUN veilid/package/rpm/veilid-cli/earthly_make_veilid_cli_rpm.sh x86_64 x86_64-unknown-linux-gnu
    # save artifacts
    SAVE ARTIFACT --keep-ts /root/rpmbuild/RPMS/x86_64/*.rpm AS LOCAL ./target/packages/
    
package-linux-arm64-deb:
    FROM +build-linux-arm64
    #################################
    ### DEBIAN DPKG .DEB FILES
    #################################
    COPY --dir package /veilid
    # veilid-server
    RUN /veilid/package/debian/earthly_make_veilid_server_deb.sh arm64 aarch64-unknown-linux-gnu
    SAVE ARTIFACT --keep-ts /dpkg/out/*.deb AS LOCAL ./target/packages/
    # veilid-cli
    RUN /veilid/package/debian/earthly_make_veilid_cli_deb.sh arm64 aarch64-unknown-linux-gnu
    # save artifacts
    SAVE ARTIFACT --keep-ts /dpkg/out/*.deb AS LOCAL ./target/packages/

package-linux-arm64-rpm:
    FROM --platform arm64 rockylinux:8
    RUN yum install -y createrepo rpm-build rpm-sign yum-utils rpmdevtools
    RUN rpmdev-setuptree
    #################################
    ### RPMBUILD .RPM FILES
    #################################
    RUN mkdir -p /veilid/target
    COPY --dir .cargo files scripts veilid-cli veilid-core veilid-server veilid-tools veilid-flutter veilid-wasm Cargo.lock Cargo.toml package /veilid
    COPY +build-linux-arm64/aarch64-unknown-linux-gnu /veilid/target/aarch64-unknown-linux-gnu
    RUN mkdir -p /rpm-work-dir/veilid-server
    # veilid-server
    RUN veilid/package/rpm/veilid-server/earthly_make_veilid_server_rpm.sh aarch64 aarch64-unknown-linux-gnu
    #SAVE ARTIFACT --keep-ts /root/rpmbuild/RPMS/aarch64/*.rpm AS LOCAL ./target/packages/
    # veilid-cli
    RUN veilid/package/rpm/veilid-cli/earthly_make_veilid_cli_rpm.sh aarch64 aarch64-unknown-linux-gnu
    # save artifacts
    SAVE ARTIFACT --keep-ts /root/rpmbuild/RPMS/aarch64/*.rpm AS LOCAL ./target/packages/

package-linux-amd64:
    BUILD +package-linux-amd64-deb
    BUILD +package-linux-amd64-rpm

package-linux-arm64:
    BUILD +package-linux-arm64-deb
    BUILD +package-linux-arm64-rpm
    
package-linux:
    BUILD +package-linux-amd64
    BUILD +package-linux-arm64
