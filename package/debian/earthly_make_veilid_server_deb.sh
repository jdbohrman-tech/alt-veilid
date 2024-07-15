#!/bin/bash
set -e

ARCH=$1
CARGO_ARCH=$2
IS_NIGHTLY=$3
BUILD_DATE=$(date '+%Y%m%d')
CARGO_VERSION="$(/veilid/package/cargo_version.sh /veilid/veilid-server/Cargo.toml)"
rm -rf /dpkg
mkdir -p /dpkg/out
    
# veilid-server dpkg control
cp -rf /veilid/package/debian/veilid-server /dpkg
/veilid/package/replace_variable.sh /dpkg/veilid-server/DEBIAN/control CARGO_VERSION $CARGO_VERSION 
/veilid/package/replace_variable.sh /dpkg/veilid-server/DEBIAN/control ARCH $ARCH
# veilid-server configuration
mkdir -p /dpkg/veilid-server/etc/veilid-server
cp -f /veilid/package/linux/veilid-server.conf /dpkg/veilid-server/etc/veilid-server/veilid-server.conf
# veilid-server systemd unit file
mkdir -p /dpkg/veilid-server/etc/systemd/system
cp -f /veilid/package/systemd/veilid-server.service /dpkg/veilid-server/etc/systemd/system
# veilid-server executable
mkdir -p /dpkg/veilid-server/usr/bin
cp -f /veilid/target/$CARGO_ARCH/release/veilid-server /dpkg/veilid-server/usr/bin
# pack it up
dpkg-deb -b /dpkg/veilid-server/
# Appropriatly name the package for STABLE or NIGHTLY release
if [ "$3" = "true" ]
then
    mv /dpkg/veilid-server.deb /dpkg/out/veilid-server-$BUILD_DATE\_$ARCH.deb
elif [ "$3" = "false" ]
then
    mv /dpkg/veilid-server.deb /dpkg/out/veilid-server-$CARGO_VERSION\_$ARCH.deb
else
    echo $3 "is not a valid state to determine if the build is STABLE or NIGHTLY"
fi