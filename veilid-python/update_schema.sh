#!/bin/bash
set -eo pipefail
SCRIPTDIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"

VEILID_SERVER=$SCRIPTDIR/../target/debug/veilid-server

# Ensure executable exists
if [ ! -f "$VEILID_SERVER" ]; then
    echo "$VEILID_SERVER does not exist. Build with cargo build."
    exit 1
fi

# Produce schema from veilid-server
$VEILID_SERVER --emit-schema Request > $SCRIPTDIR/veilid_python/schema/Request.json
$VEILID_SERVER --emit-schema RecvMessage > $SCRIPTDIR/veilid_python/schema/RecvMessage.json


