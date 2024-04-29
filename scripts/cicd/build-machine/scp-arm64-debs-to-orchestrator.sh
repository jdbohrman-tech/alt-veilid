#!/bin/bash

mkdir workspace
cd workspace
cp ~/builds/7TYBLKUtG/0/veilid/veilid/target/packages/*.deb .
tar -cf arm64-debs.tar *.deb
scp *.tar gitlab-runner@10.116.0.5:~
cd ~
rm -rf workspace