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

ENV ZIG_VERSION=0.13.0
ENV CMAKE_VERSION_MINOR=3.30
ENV CMAKE_VERSION_PATCH=3.30.1
ENV WASM_BINDGEN_CLI_VERSION=0.2.93
ENV RUST_VERSION=1.81.0
ENV RUSTUP_HOME=/usr/local/rustup
ENV RUSTUP_DIST_SERVER=https://static.rust-lang.org
ENV CARGO_HOME=/usr/local/cargo
ENV PATH=$PATH:/usr/local/cargo/bin:/usr/local/zig
ENV LD_LIBRARY_PATH=/usr/local/lib
ENV RUST_BACKTRACE=1

WORKDIR /veilid

# Install build prerequisites & setup required directories
deps-base:
    RUN apt-get -y update
    RUN apt-get install -y iproute2 curl build-essential libssl-dev openssl file git pkg-config libdbus-1-dev libdbus-glib-1-dev libgirepository1.0-dev libcairo2-dev checkinstall unzip libncursesw5-dev libncurses5-dev
    RUN curl -O https://cmake.org/files/v$CMAKE_VERSION_MINOR/cmake-$CMAKE_VERSION_PATCH-linux-$(arch).sh
    RUN mkdir /opt/cmake
    RUN sh cmake-$CMAKE_VERSION_PATCH-linux-$(arch).sh --skip-license --prefix=/opt/cmake
    RUN ln -s /opt/cmake/bin/cmake /usr/local/bin/cmake

# Install Rust
deps-rust:
    FROM +deps-base
    RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain=$RUST_VERSION -y -c clippy --no-modify-path --profile minimal
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
    RUN cargo install wasm-pack
    RUN cargo install -f wasm-bindgen-cli --version $WASM_BINDGEN_CLI_VERSION
    # Caching tool
    RUN cargo install cargo-chef
    # Install Linux cross-platform tooling
    RUN curl -O https://ziglang.org/download/$ZIG_VERSION/zig-linux-$(arch)-$ZIG_VERSION.tar.xz
    RUN tar -C /usr/local -xJf zig-linux-$(arch)-$ZIG_VERSION.tar.xz
    RUN mv /usr/local/zig-linux-$(arch)-$ZIG_VERSION /usr/local/zig
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
    RUN yes | /Android/cmdline-tools/bin/sdkmanager --sdk_root=/Android/Sdk build-tools\;34.0.0 ndk\;26.3.11579264 cmake\;3.22.1 platform-tools platforms\;android-34 cmdline-tools\;latest
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
    ELSE IF [ "$BASE" = "uncached" ]
        FROM +deps-linux
    ELSE
        ARG CI_REGISTRY_IMAGE=registry.gitlab.com/veilid/veilid
        FROM $CI_REGISTRY_IMAGE/build-cache:latest
        # FROM registry.gitlab.com/veilid/build-cache:latest
    END
    COPY --dir .cargo build_docs.sh files scripts veilid-cli veilid-core veilid-server veilid-tools veilid-flutter veilid-wasm Cargo.lock Cargo.toml /veilid

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
    RUN cargo clippy --manifest-path=veilid-wasm/Cargo.toml --target wasm32-unknown-unknown

# Build
build-release:
    FROM +code-linux
    RUN cargo build --release -p veilid-server -p veilid-cli -p veilid-tools -p veilid-core
    SAVE ARTIFACT ./target/release AS LOCAL ./target/earthly/release

build:
    FROM +code-linux
    RUN cargo build -p veilid-server -p veilid-cli -p veilid-tools -p veilid-core
    SAVE ARTIFACT ./target/debug AS LOCAL ./target/earthly/debug

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
    ENV PATH=$PATH:/Android/Sdk/ndk/26.3.11579264/toolchains/llvm/prebuilt/linux-x86_64/bin/
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
unit-tests-clippy-linux:
    FROM +code-linux
    RUN cargo clippy

unit-tests-clippy-wasm-linux:
    FROM +code-linux
    RUN cargo clippy --manifest-path=veilid-wasm/Cargo.toml --target wasm32-unknown-unknown

unit-tests-docs-linux:
    FROM +code-linux
    RUN ./build_docs.sh
        
unit-tests-native-linux:
    FROM +code-linux
    RUN cargo test -p veilid-server -p veilid-cli -p veilid-tools -p veilid-core

unit-tests-wasm-linux:
    FROM +code-linux
    # Just run build now because actual unit tests require network access
    # which should be moved to a separate integration test
    RUN veilid-wasm/wasm_build.sh

unit-tests-linux:
    WAIT
        BUILD +unit-tests-clippy-linux
    END
    WAIT
        BUILD +unit-tests-clippy-wasm-linux
    END
    WAIT
        BUILD +unit-tests-docs-linux
    END
    WAIT
        BUILD +unit-tests-native-linux
    END
    WAIT
        BUILD +unit-tests-wasm-linux
    END

# Package 
package-linux-amd64-deb:
    ARG IS_NIGHTLY="false"
    FROM +build-linux-amd64
    #################################
    ### DEBIAN DPKG .DEB FILES
    #################################
    COPY --dir package /veilid
    # veilid-server
    RUN /veilid/package/debian/earthly_make_veilid_server_deb.sh amd64 x86_64-unknown-linux-gnu "$IS_NIGHTLY"
    SAVE ARTIFACT --keep-ts /dpkg/out/*.deb AS LOCAL ./target/packages/
    # veilid-cli
    RUN /veilid/package/debian/earthly_make_veilid_cli_deb.sh amd64 x86_64-unknown-linux-gnu "$IS_NIGHTLY"
    # save artifacts
    SAVE ARTIFACT --keep-ts /dpkg/out/*.deb AS LOCAL ./target/packages/

package-linux-amd64-rpm:
    ARG IS_NIGHTLY="false"
    FROM --platform amd64 rockylinux:9
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
    RUN veilid/package/rpm/veilid-server/earthly_make_veilid_server_rpm.sh x86_64 x86_64-unknown-linux-gnu "$IS_NIGHTLY"
    #SAVE ARTIFACT --keep-ts /root/rpmbuild/RPMS/x86_64/*.rpm AS LOCAL ./target/packages/
    # veilid-cli
    RUN veilid/package/rpm/veilid-cli/earthly_make_veilid_cli_rpm.sh x86_64 x86_64-unknown-linux-gnu "$IS_NIGHTLY"
    # save artifacts
    SAVE ARTIFACT --keep-ts /root/rpmbuild/RPMS/x86_64/*.rpm AS LOCAL ./target/packages/
    
package-linux-arm64-deb:
    ARG IS_NIGHTLY="false"
    FROM +build-linux-arm64
    #################################
    ### DEBIAN DPKG .DEB FILES
    #################################
    COPY --dir package /veilid
    # veilid-server
    RUN /veilid/package/debian/earthly_make_veilid_server_deb.sh arm64 aarch64-unknown-linux-gnu "$IS_NIGHTLY"
    SAVE ARTIFACT --keep-ts /dpkg/out/*.deb AS LOCAL ./target/packages/
    # veilid-cli
    RUN /veilid/package/debian/earthly_make_veilid_cli_deb.sh arm64 aarch64-unknown-linux-gnu "$IS_NIGHTLY"
    # save artifacts
    SAVE ARTIFACT --keep-ts /dpkg/out/*.deb AS LOCAL ./target/packages/

package-linux-arm64-rpm:
    ARG IS_NIGHTLY="false"
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
    RUN veilid/package/rpm/veilid-server/earthly_make_veilid_server_rpm.sh aarch64 aarch64-unknown-linux-gnu "$IS_NIGHTLY"
    #SAVE ARTIFACT --keep-ts /root/rpmbuild/RPMS/aarch64/*.rpm AS LOCAL ./target/packages/
    # veilid-cli
    RUN veilid/package/rpm/veilid-cli/earthly_make_veilid_cli_rpm.sh aarch64 aarch64-unknown-linux-gnu "$IS_NIGHTLY"
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