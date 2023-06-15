import pytest
pytest_plugins = ('pytest_asyncio',)

import os

##################################################################
VEILID_SERVER = os.getenv("VEILID_SERVER")
if VEILID_SERVER is not None:
    vsparts = VEILID_SERVER.split(":") 
    VEILID_SERVER = vsparts[0]
    if len(vsparts) == 2:
        VEILID_SERVER_PORT = int(vsparts[1])
    else:
        VEILID_SERVER_PORT = 5959
else:
    VEILID_SERVER = "localhost"
    VEILID_SERVER_PORT = 5959

##################################################################

async def simple_update_callback(update):
    print("VeilidUpdate: {}".format(update))
