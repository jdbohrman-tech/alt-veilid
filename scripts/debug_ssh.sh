#!/bin/bash
SCRIPTDIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"

SSHHOST=$1
if [[ "$SSHHOST" == "" ]]; then
    SSHHOST="bootstrap-1.dev.veilid.net"
fi

echo Copying debug script
scp -q $SCRIPTDIR/debug.sh $SSHHOST:/tmp/debug.sh
echo Connecting to debug server
ssh -t $SSHHOST -L 6969:127.0.0.1:6969 -L 6970:127.0.0.1:6970 'bash /tmp/debug.sh'
