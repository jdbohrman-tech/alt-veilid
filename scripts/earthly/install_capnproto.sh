#!/bin/bash
SCRIPTDIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
if [ -f ".capnp_version" ]; then 
    CAPNPROTO_VERSION=$(cat ".capnp_version")
else
    CAPNPROTO_VERSION=$(cat "$SCRIPTDIR/../../.capnp_version")
fi

mkdir /tmp/capnproto-install
pushd /tmp/capnproto-install
curl -O https://capnproto.org/capnproto-c++-${CAPNPROTO_VERSION}.tar.gz
tar zxf capnproto-c++-${CAPNPROTO_VERSION}.tar.gz
cd capnproto-c++-${CAPNPROTO_VERSION}
./configure --without-openssl
make -j$1 check
if [ "$EUID" -ne 0 ]; then
    if command -v checkinstall &> /dev/null; then 
        sudo checkinstall -y
        cp *.deb ~
    else 
        sudo make install

    fi
    popd
    sudo rm -rf /tmp/capnproto-install
else
    if command -v checkinstall &> /dev/null; then 
        checkinstall -y
        cp *.deb ~
    else 
        make install
    fi
    popd
    rm -rf /tmp/capnproto-install
fi
