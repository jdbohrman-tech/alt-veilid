VERSION 0.8

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
ENV WASM_BINDGEN_CLI_VERSION=0.2.100
ENV RUST_VERSION=1.81.0
ENV RUSTUP_HOME=/usr/local/rustup
ENV RUSTUP_DIST_SERVER=https://static.rust-lang.org
ENV CARGO_HOME=/usr/local/cargo
ENV PATH=$PATH:/usr/local/cargo/bin:/usr/local/zig
ENV LD_LIBRARY_PATH=/usr/local/lib
ENV RUST_BACKTRACE=1
ENV RETRY_COUNT=12

WORKDIR /veilid

IF [ $(arch) = "x86_64" ]
    ENV DEFAULT_CARGO_TARGET = "x86_64-unknown-linux-gnu"
ELSE IF [ $(arch) = "aarch64" ]
    ENV DEFAULT_CARGO_TARGET = "aarch64-unknown-linux-gnu"
ELSE
    RUN echo "Unsupported host platform"
    RUN false
END

# Install build prerequisites & setup required directories
deps-base:
    RUN echo '\
        Acquire::Retries "'$RETRY_COUNT'";\
        Acquire::https::Timeout "240";\
        Acquire::http::Timeout "240";\
        APT::Get::Assume-Yes "true";\
        APT::Install-Recommends "false";\
        APT::Install-Suggests "false";\
        Debug::Acquire::https "true";\
        ' > /etc/apt/apt.conf.d/99custom
    RUN apt-get -y update
    RUN apt-get install -y ca-certificates iproute2 curl build-essential libssl-dev openssl file git pkg-config libdbus-1-dev libdbus-glib-1-dev libgirepository1.0-dev libcairo2-dev checkinstall unzip libncursesw5-dev libncurses5-dev
    IF [ $(arch) = "x86_64" ]
        RUN apt-get install -y gcc-aarch64-linux-gnu
    ELSE IF [ $(arch) = "aarch64" ]
        RUN apt-get install -y gcc-x86-64-linux-gnu
    ELSE
        RUN apt-get install -y gcc-aarch64-linux-gnu gcc-x86-64-linux-gnu
    END
    RUN curl --retry $RETRY_COUNT --retry-connrefused -O https://cmake.org/files/v$CMAKE_VERSION_MINOR/cmake-$CMAKE_VERSION_PATCH-linux-$(arch).sh
    RUN mkdir /opt/cmake
    RUN sh cmake-$CMAKE_VERSION_PATCH-linux-$(arch).sh --skip-license --prefix=/opt/cmake
    RUN ln -s /opt/cmake/bin/cmake /usr/local/bin/cmake

# Install Rust
deps-rust:
    FROM +deps-base
    RUN curl --retry $RETRY_COUNT --retry-connrefused --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain=$RUST_VERSION -y -c clippy --no-modify-path --profile minimal
    RUN chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
        rustup --version; \
        cargo --version; \
        rustc --version;
    RUN retry=0; until [ "$retry" -ge $RETRY_COUNT ]; do \
        rustup target add \
            # Linux
                x86_64-unknown-linux-gnu \
                aarch64-unknown-linux-gnu \
            # Android
                aarch64-linux-android \
                armv7-linux-androideabi \
                i686-linux-android \
                x86_64-linux-android \
            # WASM
                wasm32-unknown-unknown \
            && break; \
            retry=$((retry+1)); \
            echo "retry #$retry..."; \
            sleep 10; \
        done
    RUN cargo install wasm-pack wasm-tools --locked
    RUN cargo install -f wasm-bindgen-cli --locked --version $WASM_BINDGEN_CLI_VERSION
    # Caching tool
    RUN cargo install cargo-chef --locked
    # Install Linux cross-platform tooling
    RUN curl --retry $RETRY_COUNT --retry-connrefused -O https://ziglang.org/download/$ZIG_VERSION/zig-linux-$(arch)-$ZIG_VERSION.tar.xz
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
    WAIT
        BUILD +deps-rust
    END
    COPY +deps-rust/cargo /usr/local/cargo
    COPY +deps-rust/rustup /usr/local/rustup
    COPY +deps-rust/cargo-zigbuild /usr/local/cargo/bin/cargo-zigbuild
    COPY +deps-rust/zig /usr/local/zig
    RUN apt-get install -y openjdk-9-jdk-headless
    RUN mkdir /Android; mkdir /Android/Sdk
    RUN curl --retry $RETRY_COUNT --retry-connrefused -o /Android/cmdline-tools.zip https://dl.google.com/android/repository/commandlinetools-linux-9123335_latest.zip
    RUN cd /Android; unzip /Android/cmdline-tools.zip
    RUN yes | /Android/cmdline-tools/bin/sdkmanager --sdk_root=/Android/Sdk build-tools\;34.0.0 ndk\;27.0.12077973 cmake\;3.22.1 platform-tools platforms\;android-34 cmdline-tools\;latest
    RUN rm -rf /Android/cmdline-tools
    RUN apt-get clean

# Just linux build not android
deps-linux:
    FROM +deps-base
    WAIT
        BUILD +deps-rust
    END
    COPY +deps-rust/cargo /usr/local/cargo
    COPY +deps-rust/rustup /usr/local/rustup
    COPY +deps-rust/cargo-zigbuild /usr/local/cargo/bin/cargo-zigbuild
    COPY +deps-rust/zig /usr/local/zig

# Make a cache image with downloaded and built dependencies
build-linux-cache:
    FROM +deps-linux
    RUN mkdir veilid-cli veilid-core veilid-server veilid-tools veilid-wasm veilid-flutter veilid-flutter/rust
    COPY --keep-ts --dir .cargo scripts Cargo.lock Cargo.toml .
    COPY --keep-ts veilid-cli/Cargo.toml veilid-cli
    COPY --keep-ts veilid-core/Cargo.toml veilid-core
    COPY --keep-ts veilid-server/Cargo.toml veilid-server
    COPY --keep-ts veilid-tools/Cargo.toml veilid-tools
    COPY --keep-ts veilid-flutter/rust/Cargo.toml veilid-flutter/rust
    COPY --keep-ts veilid-wasm/Cargo.toml veilid-wasm/wasm_remap_paths.sh veilid-wasm
    RUN cargo chef prepare --recipe-path recipe.json
    RUN cargo chef cook --profile=test --tests --target $DEFAULT_CARGO_TARGET --recipe-path recipe.json -p veilid-server -p veilid-cli -p veilid-tools -p veilid-core
    RUN cargo chef cook --zigbuild --release --target x86_64-unknown-linux-gnu --recipe-path recipe.json -p veilid-server -p veilid-cli -p veilid-tools -p veilid-core
    RUN cargo chef cook --zigbuild --release --target aarch64-unknown-linux-gnu --recipe-path recipe.json -p veilid-server -p veilid-cli -p veilid-tools -p veilid-core
    RUN veilid-wasm/wasm_remap_paths.sh cargo chef cook --zigbuild --release --target wasm32-unknown-unknown --recipe-path recipe.json -p veilid-wasm
    ARG CI_REGISTRY_IMAGE=registry.gitlab.com/veilid/veilid
    SAVE IMAGE --push $CI_REGISTRY_IMAGE/build-cache:latest

# Import the whole veilid code repository from the earthly host
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
    COPY --keep-ts --dir .cargo build_docs.sh files scripts veilid-cli veilid-core veilid-server veilid-tools veilid-flutter veilid-wasm Cargo.lock Cargo.toml /veilid

# Code + Linux + Android deps
code-android:
    FROM +deps-android
    COPY --keep-ts --dir .cargo files scripts veilid-cli veilid-core veilid-server veilid-tools veilid-flutter veilid-wasm Cargo.lock Cargo.toml /veilid
    COPY --keep-ts scripts/earthly/cargo-android/config.toml /veilid/.cargo/config.toml

# Clippy only
clippy:
    FROM +code-linux
    RUN cargo clippy --target x86_64-unknown-linux-gnu
    RUN cargo clippy --manifest-path=veilid-wasm/Cargo.toml --target wasm32-unknown-unknown

# Build
build-linux-amd64:
    FROM +code-linux
    # Ensure we have enough memory
    IF [ $(free -wmt | grep Total | awk  '{print $2}') -lt 7500 ]
        RUN echo "not enough container memory to build. increase build host memory."
        RUN false
    END
    RUN cargo zigbuild --target x86_64-unknown-linux-gnu --release -p veilid-server -p veilid-cli -p veilid-tools -p veilid-core
    SAVE ARTIFACT ./target/x86_64-unknown-linux-gnu AS LOCAL ./target/artifacts/x86_64-unknown-linux-gnu

build-linux-arm64:
    FROM +code-linux
    RUN cargo zigbuild --target aarch64-unknown-linux-gnu --release -p veilid-server -p veilid-cli -p veilid-tools -p veilid-core
    SAVE ARTIFACT ./target/aarch64-unknown-linux-gnu AS LOCAL ./target/artifacts/aarch64-unknown-linux-gnu

build-android:
    FROM +code-android
    WORKDIR /veilid/veilid-core
    ENV PATH=$PATH:/Android/Sdk/ndk/27.0.12077973/toolchains/llvm/prebuilt/linux-x86_64/bin/
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
    RUN cargo clippy --target $DEFAULT_CARGO_TARGET

unit-tests-clippy-wasm-linux:
    FROM +code-linux
    RUN cargo clippy --manifest-path=veilid-wasm/Cargo.toml --target wasm32-unknown-unknown

unit-tests-docs-linux:
    FROM +code-linux
    RUN ./build_docs.sh
        
unit-tests-native-linux:
    FROM +code-linux
    RUN cargo test --tests --target $DEFAULT_CARGO_TARGET -p veilid-server -p veilid-cli -p veilid-tools -p veilid-core

unit-tests-wasm-linux:
    FROM +code-linux
    # Just run build now because actual unit tests require network access
    # which should be moved to a separate integration test
    RUN veilid-wasm/wasm_build.sh release

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
    COPY --keep-ts --dir package /veilid
    # veilid-server
    RUN /veilid/package/debian/earthly_make_veilid_server_deb.sh amd64 x86_64-unknown-linux-gnu "$IS_NIGHTLY"
    SAVE ARTIFACT --keep-ts /dpkg/out/*.deb AS LOCAL ./target/packages/
    # veilid-cli
    RUN /veilid/package/debian/earthly_make_veilid_cli_deb.sh amd64 x86_64-unknown-linux-gnu "$IS_NIGHTLY"
    # save artifacts
    SAVE ARTIFACT --keep-ts /dpkg/out/*.deb AS LOCAL ./target/packages/

package-linux-amd64-rpm:
    ARG IS_NIGHTLY="false"
    FROM --platform linux/amd64 rockylinux:9
    RUN yum install -y createrepo rpm-build rpm-sign yum-utils rpmdevtools
    RUN rpmdev-setuptree
    #################################
    ### RPMBUILD .RPM FILES
    #################################
    RUN mkdir -p /veilid/target
    RUN mkdir -p /veilid/veilid-cli /veilid/veilid-server
    COPY --keep-ts veilid-cli/Cargo.toml /veilid/veilid-cli
    COPY --keep-ts veilid-server/Cargo.toml /veilid/veilid-server
    COPY --keep-ts --dir package /veilid
    COPY --keep-ts +build-linux-amd64/x86_64-unknown-linux-gnu /veilid/target/x86_64-unknown-linux-gnu
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
    COPY --keep-ts --dir package /veilid
    # veilid-server
    RUN /veilid/package/debian/earthly_make_veilid_server_deb.sh arm64 aarch64-unknown-linux-gnu "$IS_NIGHTLY"
    SAVE ARTIFACT --keep-ts /dpkg/out/*.deb AS LOCAL ./target/packages/
    # veilid-cli
    RUN /veilid/package/debian/earthly_make_veilid_cli_deb.sh arm64 aarch64-unknown-linux-gnu "$IS_NIGHTLY"
    # save artifacts
    SAVE ARTIFACT --keep-ts /dpkg/out/*.deb AS LOCAL ./target/packages/

package-linux-arm64-rpm:
    ARG IS_NIGHTLY="false"
    FROM --platform linux/arm64 rockylinux:9
    RUN yum install -y createrepo rpm-build rpm-sign yum-utils rpmdevtools
    RUN rpmdev-setuptree
    #################################
    ### RPMBUILD .RPM FILES
    #################################
    RUN mkdir -p /veilid/target
    RUN mkdir -p /veilid/veilid-cli /veilid/veilid-server
    COPY --keep-ts veilid-cli/Cargo.toml /veilid/veilid-cli
    COPY --keep-ts veilid-server/Cargo.toml /veilid/veilid-server
    COPY --keep-ts --dir package /veilid
    COPY --keep-ts +build-linux-arm64/aarch64-unknown-linux-gnu /veilid/target/aarch64-unknown-linux-gnu
    RUN mkdir -p /rpm-work-dir/veilid-server
    # veilid-server
    RUN veilid/package/rpm/veilid-server/earthly_make_veilid_server_rpm.sh aarch64 aarch64-unknown-linux-gnu "$IS_NIGHTLY"
    #SAVE ARTIFACT --keep-ts /root/rpmbuild/RPMS/aarch64/*.rpm AS LOCAL ./target/packages/
    # veilid-cli
    RUN veilid/package/rpm/veilid-cli/earthly_make_veilid_cli_rpm.sh aarch64 aarch64-unknown-linux-gnu "$IS_NIGHTLY"
    # save artifacts
    SAVE ARTIFACT --keep-ts /root/rpmbuild/RPMS/aarch64/*.rpm AS LOCAL ./target/packages/

package-linux-amd64:
    WAIT
        BUILD +package-linux-amd64-deb
    END
    WAIT
        BUILD +package-linux-amd64-rpm
    END

package-linux-arm64:
    WAIT
        BUILD +package-linux-arm64-deb
    END
    WAIT
        BUILD +package-linux-arm64-rpm
    END
    
package-linux:
    WAIT
        BUILD +package-linux-amd64
    END
    WAIT
        BUILD +package-linux-arm64
    END