#!/bin/bash
set -e

ARCH=$1
CARGO_ARCH=$2
IS_NIGHTLY=$3
BUILD_DATE=$(date '+%Y%m%d')
CARGO_VERSION="$(/veilid/package/cargo_version.sh /veilid/veilid-cli/Cargo.toml)"
rm -rf /dpkg
mkdir -p /dpkg/out
    
# veilid-cli dpkg control
cp -rf /veilid/package/debian/veilid-cli /dpkg
# Appropriatly set vars for STABLE or NIGHTLY release
if [ "$3" = "true" ]
then
    /veilid/package/replace_variable.sh /dpkg/veilid-cli/DEBIAN/control CARGO_VERSION $BUILD_DATE
elif [ "$3" = "false" ]
then
    /veilid/package/replace_variable.sh /dpkg/veilid-cli/DEBIAN/control CARGO_VERSION $CARGO_VERSION 
else
    echo $3 "is not a valid state to determine if the build is STABLE or NIGHTLY"
fi
/veilid/package/replace_variable.sh /dpkg/veilid-cli/DEBIAN/control ARCH $ARCH
# veilid-cli executable
mkdir -p /dpkg/veilid-cli/usr/bin
cp -f /veilid/target/$CARGO_ARCH/release/veilid-cli /dpkg/veilid-cli/usr/bin
# pack it up
dpkg-deb -b /dpkg/veilid-cli/
# Appropriatly name the package for STABLE or NIGHTLY release
if [ "$3" = "true" ]
then
    mv /dpkg/veilid-cli.deb /dpkg/out/veilid-cli-$BUILD_DATE\_$ARCH.deb
elif [ "$3" = "false" ]
then
    mv /dpkg/veilid-cli.deb /dpkg/out/veilid-cli-$CARGO_VERSION\_$ARCH.deb
else
    echo $3 "is not a valid state to determine if the build is STABLE or NIGHTLY"
fi
echo "make veilid-cli deb process complete"