#!/bin/bash
ARCH=$1
CARGO_ARCH=$2
CARGO_VERSION="$(/veilid/package/cargo_version.sh /veilid/veilid-server/Cargo.toml)"
    
# veilid-server spec file
cp -rf /veilid/package/rpm/veilid-server/veilid-server.spec /root/rpmbuild/SPECS/
/veilid/package/replace_variable.sh /root/rpmbuild/SPECS/veilid-server.spec CARGO_VERSION $CARGO_VERSION 
/veilid/package/replace_variable.sh /root/rpmbuild/SPECS/veilid-server.spec ARCH $ARCH
/veilid/package/replace_variable.sh /root/rpmbuild/SPECS/veilid-server.spec CARGO_ARCH $CARGO_ARCH

# build the rpm
rpmbuild --target "$ARCH" -bb /root/rpmbuild/SPECS/veilid-server.spec