VERSION 0.6

# Start with older Ubuntu to ensure GLIBC symbol versioning support for older linux
# Ensure we are using an amd64 platform because some of these targets use cross-platform tooling
FROM ubuntu:18.04

# Install build prerequisites
deps-base:
    RUN apt-get -y update
    RUN apt-get install -y iproute2 curl build-essential cmake libssl-dev openssl file git pkg-config libdbus-1-dev libdbus-glib-1-dev libgirepository1.0-dev libcairo2-dev checkinstall unzip libncursesw5-dev libncurses5-dev

# Install Cap'n Proto
deps-capnp:
    FROM +deps-base
    COPY .capnp_version /
    COPY scripts/earthly/install_capnproto.sh /
    RUN /bin/bash /install_capnproto.sh 1; rm /install_capnproto.sh .capnp_version

# Install protoc
deps-protoc:
    FROM +deps-capnp
    COPY .protoc_version /
    COPY scripts/earthly/install_protoc.sh /
    RUN /bin/bash /install_protoc.sh; rm /install_protoc.sh .protoc_version

# Install Rust
deps-rust:
    FROM +deps-protoc
    ENV RUSTUP_HOME=/usr/local/rustup
    ENV CARGO_HOME=/usr/local/cargo
    ENV PATH=/usr/local/cargo/bin:$PATH
    RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y -c clippy --no-modify-path --profile minimal 
    RUN chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
        rustup --version; \
        cargo --version; \
        rustc --version;
    # ARM64 Linux
    RUN rustup target add aarch64-unknown-linux-gnu
    # Android
    RUN rustup target add aarch64-linux-android
    RUN rustup target add armv7-linux-androideabi
    RUN rustup target add i686-linux-android
    RUN rustup target add x86_64-linux-android
    # WASM
    RUN rustup target add wasm32-unknown-unknown

# Install Linux cross-platform tooling
deps-cross:
    FROM +deps-rust
    RUN curl https://ziglang.org/builds/zig-linux-x86_64-0.11.0-dev.3978+711b4e93e.tar.xz | tar -C /usr/local -xJf -
    RUN mv /usr/local/zig-linux-x86_64-0.11.0-dev.3978+711b4e93e /usr/local/zig
    ENV PATH=$PATH:/usr/local/zig
    RUN cargo install cargo-zigbuild
    RUN rustup target add x86_64-unknown-linux-gnu
    RUN rustup target add aarch64-unknown-linux-gnu

# Install android tooling
deps-android:
    FROM +deps-cross
    RUN apt-get install -y openjdk-9-jdk-headless
    RUN mkdir /Android; mkdir /Android/Sdk
    RUN curl -o /Android/cmdline-tools.zip https://dl.google.com/android/repository/commandlinetools-linux-9123335_latest.zip
    RUN cd /Android; unzip /Android/cmdline-tools.zip
    RUN yes | /Android/cmdline-tools/bin/sdkmanager --sdk_root=/Android/Sdk build-tools\;33.0.1 ndk\;25.1.8937393 cmake\;3.22.1 platform-tools platforms\;android-33 cmdline-tools\;latest
    RUN rm -rf /Android/cmdline-tools
    RUN apt-get clean
    
# Just linux build not android
deps-linux:
    FROM +deps-cross
    RUN apt-get clean

# Code + Linux deps
code-linux:
    FROM +deps-linux
    COPY --dir .cargo .capnp_version .protoc_version files scripts veilid-cli veilid-core veilid-server veilid-tools veilid-flutter veilid-wasm Cargo.lock Cargo.toml /veilid
    RUN cat /veilid/scripts/earthly/cargo-linux/config.toml >> /veilid/.cargo/config.toml
    WORKDIR /veilid

# Code + Linux + Android deps
code-android:
    FROM +deps-android
    COPY --dir .cargo .capnp_version .protoc_version files scripts veilid-cli veilid-core veilid-server veilid-tools veilid-flutter veilid-wasm Cargo.lock Cargo.toml /veilid
    RUN cat /veilid/scripts/earthly/cargo-linux/config.toml >> /veilid/.cargo/config.toml
    RUN cat /veilid/scripts/earthly/cargo-android/config.toml >> /veilid/.cargo/config.toml
    WORKDIR /veilid

# Clippy only
clippy:
    FROM +code-linux
    ARG CI_PROJECT_PATH=veilid/veilid
    RUN cargo clippy
    SAVE IMAGE --push registry.gitl ab.com/$CI_PROJECT_PATH/clippy:latest

# Build
build-release:
    FROM +code-linux
    ARG CI_PROJECT_PATH=veilid/veilid
    RUN cargo build --release -p veilid-server -p veilid-cli -p veilid-tools -p veilid-core
    SAVE ARTIFACT ./target/release AS LOCAL ./target/artifacts/x86_64-unknown-linux-gnu
    SAVE IMAGE --push registry.gitlab.com/$CI_PROJECT_PATH/build-release:latest

build:
    FROM +code-linux
    ARG CI_PROJECT_PATH=veilid/veilid
    RUN cargo build -p veilid-server -p veilid-cli -p veilid-tools -p veilid-core
    SAVE ARTIFACT ./target/debug AS LOCAL ./target/artifacts/x86_64-unknown-linux-gnu
    SAVE IMAGE --push registry.gitlab.com/$CI_PROJECT_PATH/build:latest

build-linux-amd64:
    FROM +code-linux
    ARG CI_PROJECT_PATH=veilid/veilid
    RUN cargo build --target x86_64-unknown-linux-gnu --release -p veilid-server -p veilid-cli -p veilid-tools -p veilid-core
    SAVE ARTIFACT ./target/x86_64-unknown-linux-gnu AS LOCAL ./target/artifacts/x86_64-unknown-linux-gnu
    SAVE IMAGE --push registry.gitlab.com/$CI_PROJECT_PATH/build-linux-amd64:latest

build-linux-amd64-debug:
    FROM +code-linux
    ARG CI_PROJECT_PATH=veilid/veilid
    RUN cargo build --target x86_64-unknown-linux-gnu -p veilid-server -p veilid-cli -p veilid-tools -p veilid-core
    SAVE ARTIFACT ./target/x86_64-unknown-linux-gnu AS LOCAL ./target/artifacts/x86_64-unknown-linux-gnu
    SAVE IMAGE --push registry.gitlab.com/$CI_PROJECT_PATH/build-linux-amd64-debug:latest

build-linux-arm64:
    FROM +code-linux
    ARG CI_PROJECT_PATH=veilid/veilid
    RUN cargo zigbuild --target aarch64-unknown-linux-gnu --release -p veilid-server -p veilid-cli -p veilid-tools -p veilid-core
    SAVE ARTIFACT ./target/aarch64-unknown-linux-gnu AS LOCAL ./target/artifacts/aarch64-unknown-linux-gnu
    SAVE IMAGE --push registry.gitlab.com/$CI_PROJECT_PATH/build-linux-arm64:latest

# build-android:
#     FROM +code-android
#     ARG CI_PROJECT_PATH=veilid/veilid
#     WORKDIR /veilid/veilid-core
#     ENV PATH=$PATH:/Android/Sdk/ndk/25.1.8937393/toolchains/llvm/prebuilt/linux-x86_64/bin/
#     RUN cargo build --target aarch64-linux-android --release
#     RUN cargo build --target armv7-linux-androideabi --release
#     RUN cargo build --target i686-linux-android --release
#     RUN cargo build --target x86_64-linux-android --release
#     WORKDIR /veilid
#     SAVE ARTIFACT ./target/aarch64-linux-android AS LOCAL ./target/artifacts/aarch64-linux-android
#     SAVE ARTIFACT ./target/armv7-linux-androideabi AS LOCAL ./target/artifacts/armv7-linux-androideabi
#     SAVE ARTIFACT ./target/i686-linux-android AS LOCAL ./target/artifacts/i686-linux-android
#     SAVE ARTIFACT ./target/x86_64-linux-android AS LOCAL ./target/artifacts/x86_64-linux-android
#     SAVE IMAGE --push registry.gitlab.com/$CI_PROJECT_PATH/build-android:latest

# Unit tests
unit-tests-linux:
    FROM +code-linux
    ARG CI_PROJECT_PATH=veilid/veilid
    ENV RUST_BACKTRACE=1
    RUN cargo test --release -p veilid-server -p veilid-cli -p veilid-tools -p veilid-core
    SAVE IMAGE --push registry.gitlab.com/$CI_PROJECT_PATH/unit-tests-linux:latest

# TODO: Change t0 cross so that they work on any platform
unit-tests-linux-amd64:
    FROM +code-linux
    ARG CI_PROJECT_PATH=veilid/veilid
    ENV RUST_BACKTRACE=1
    RUN cargo test --target x86_64-unknown-linux-gnu --release -p veilid-server -p veilid-cli -p veilid-tools -p veilid-core
    SAVE IMAGE --push registry.gitlab.com/$CI_PROJECT_PATH/unit-tests-linux-amd64:latest

unit-tests-linux-arm64:
    FROM +code-linux
    ARG CI_PROJECT_PATH=veilid/veilid
    ENV RUST_BACKTRACE=1
    RUN cargo test --target aarch64-unknown-linux-gnu --release -p veilid-server -p veilid-cli -p veilid-tools -p veilid-core
    SAVE IMAGE --push registry.gitlab.com/$CI_PROJECT_PATH/unit-tests-linux-arm64:latest

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
