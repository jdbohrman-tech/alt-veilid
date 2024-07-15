#!/bin/bash
ARCH=$1
CARGO_ARCH=$2
IS_NIGHTLY=$3
BUILD_DATE=$(date '+%Y%m%d')
CARGO_VERSION="$(/veilid/package/cargo_version.sh /veilid/veilid-cli/Cargo.toml)"
    
# veilid-cli spec file
cp -rf /veilid/package/rpm/veilid-cli/veilid-cli.spec /root/rpmbuild/SPECS/
if [ "$3" = "true" ]
then
    /veilid/package/replace_variable.sh /root/rpmbuild/SPECS/veilid-cli.spec RELEASE_VERSION $BUILD_DATE
elif [ "$3" = "false" ]
    /veilid/package/replace_variable.sh /root/rpmbuild/SPECS/veilid-cli.spec RELEASE_VERSION $CARGO_VERSION
else
    echo $3 "is not a valid state to determine if the build is STABLE or NIGHTLY"
fi
/veilid/package/replace_variable.sh /root/rpmbuild/SPECS/veilid-cli.spec CARGO_VERSION $CARGO_VERSION 
/veilid/package/replace_variable.sh /root/rpmbuild/SPECS/veilid-cli.spec ARCH $ARCH
/veilid/package/replace_variable.sh /root/rpmbuild/SPECS/veilid-cli.spec CARGO_ARCH $CARGO_ARCH

# build the rpm
rpmbuild --target "x86_64" -bb /root/rpmbuild/SPECS/veilid-cli.spec