#!/bin/bash
VERSION=23.3

mkdir /tmp/protoc-install
pushd /tmp/protoc-install
curl -OL https://github.com/protocolbuffers/protobuf/releases/download/v$VERSION/protoc-$VERSION-linux-x86_64.zip
unzip protoc-$VERSION-linux-x86_64.zip
if [ "$EUID" -ne 0 ]; then
    if command -v checkinstall &> /dev/null; then 
        sudo checkinstall --pkgversion=$VERSION -y cp -r bin include /usr/local/
        cp *.deb ~
    else 
	sudo cp -r bin include /usr/local/
    fi
    popd
    sudo rm -rf /tmp/protoc-install
else
    if command -v checkinstall &> /dev/null; then 
        checkinstall --pkgversion=$VERSION -y cp -r bin include /usr/local/
        cp *.deb ~
    else 
        cp -r bin include /usr/local/
    fi
    popd
    rm -rf /tmp/protoc-install
fi
