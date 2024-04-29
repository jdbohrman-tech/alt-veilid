#!/bin/bash

mkdir workspace
cd workspace
cp ~/builds/C6yRimG-M/0/veilid/veilid/target/packages/*.rpm .
tar -cf amd64-rpms.tar *.rpm
scp *.tar gitlab-runner@10.116.0.5:~
cd ~
rm -rf workspace