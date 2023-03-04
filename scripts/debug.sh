#!/bin/bash
trap "trap - SIGTERM && kill -- -$$" SIGINT SIGTERM EXIT
killall lldb-server 2> /dev/null
echo Running lldb-server
pushd /tmp > /dev/null
sudo -u veilid lldb-server platform --server --listen 127.0.0.1:6969 --gdbserver-port 6970 
popd > /dev/null