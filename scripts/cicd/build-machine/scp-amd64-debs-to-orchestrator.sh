#!/bin/bash

mkdir workspace
cd workspace
# if a new GitLab runner is created, the identifier below that follows build/ will be invalid
# it might be available as a runner variable but IDK
cp ~/builds/t338Uo9fn/0/veilid/veilid/target/packages/*.deb .
tar -cf amd64-debs.tar *.deb
scp *.tar gitlab-runner@10.116.0.5:~
cd ../
rm -rf workspace