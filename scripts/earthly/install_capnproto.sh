#!/bin/bash
mkdir /tmp/capnproto-install
pushd /tmp/capnproto-install
curl -O https://capnproto.org/capnproto-c++-0.10.2.tar.gz
tar zxf capnproto-c++-0.10.2.tar.gz
cd capnproto-c++-0.10.2
./configure --without-openssl
make -j6 check
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
