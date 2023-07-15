#!/bin/bash
set -e

ARCH=$1
CARGO_ARCH=$2
CARGO_VERSION="$(/veilid/package/cargo_version.sh /veilid/veilid-cli/Cargo.toml)"
rm -rf /dpkg
mkdir -p /dpkg/out
    
# veilid-cli dpkg control
cp -rf /veilid/package/debian/veilid-cli /dpkg
/veilid/package/replace_variable.sh /dpkg/veilid-cli/DEBIAN/control CARGO_VERSION $CARGO_VERSION
/veilid/package/replace_variable.sh /dpkg/veilid-cli/DEBIAN/control ARCH $ARCH
# veilid-cli executable
mkdir -p /dpkg/veilid-cli/usr/bin
cp -f /veilid/target/$CARGO_ARCH/release/veilid-cli /dpkg/veilid-cli/usr/bin
# pack it up
dpkg-deb -b /dpkg/veilid-cli/
mv /dpkg/veilid-cli.deb /dpkg/out/veilid-cli-$CARGO_VERSION\_$ARCH.deb