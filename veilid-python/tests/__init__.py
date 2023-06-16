from typing import Callable, Awaitable
import os
import pytest
pytest_plugins = ('pytest_asyncio',)

import veilid


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

async def simple_connect_and_run(func: Callable[[veilid.VeilidAPI], Awaitable]):
    api = await veilid.json_api_connect(VEILID_SERVER, VEILID_SERVER_PORT, simple_update_callback)
    async with api:
        
        # purge routes to ensure we start fresh
        await api.debug("purge routes")
        
        await func(api)

async def simple_update_callback(update: veilid.VeilidUpdate):
    print("VeilidUpdate: {}".format(update))
