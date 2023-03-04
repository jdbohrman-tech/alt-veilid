#!/bin/bash
trap "trap - SIGTERM && kill -- -$$" SIGINT SIGTERM EXIT
killall lldb-server 2> /dev/null
echo Running lldb-server
lldb-server platform --server --listen 127.0.0.1:6969 --gdbserver-port 6970 
