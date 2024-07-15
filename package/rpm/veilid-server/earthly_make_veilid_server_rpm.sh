#!/bin/bash
ARCH=$1
CARGO_ARCH=$2
IS_NIGHTLY=$3
BUILD_DATE=$(date '+%Y%m%d')
CARGO_VERSION="$(/veilid/package/cargo_version.sh /veilid/veilid-server/Cargo.toml)"
    
# veilid-server spec file
cp -rf /veilid/package/rpm/veilid-server/veilid-server.spec /root/rpmbuild/SPECS/
# Select CARGO_VERSION for STABLE releases or BUILD_DATE for NIGHTLY releases
if [ "$3" = "true" ]
then
    /veilid/package/replace_variable.sh /root/rpmbuild/SPECS/veilid-server.spec RELEASE_VERSION $BUILD_DATE
elif [ "$3" = "false" ]
    /veilid/package/replace_variable.sh /root/rpmbuild/SPECS/veilid-server.spec RELEASE_VERSION $CARGO_VERSION
else
    echo $3 "is not a valid state to determine if the build is STABLE or NIGHTLY"
fi
/veilid/package/replace_variable.sh /root/rpmbuild/SPECS/veilid-server.spec ARCH $ARCH
/veilid/package/replace_variable.sh /root/rpmbuild/SPECS/veilid-server.spec CARGO_ARCH $CARGO_ARCH

# build the rpm
rpmbuild --target "$ARCH" -bb /root/rpmbuild/SPECS/veilid-server.spec