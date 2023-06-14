# Basic veilid_python tests

import veilid_python
import pytest
import os

pytest_plugins = ('pytest_asyncio',)

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

async def _simple_update_callback(update):
    print("VeilidUpdate: {}".format(update))

@pytest.mark.asyncio
async def test_connect():
    async with await veilid_python.json_api_connect(VEILID_SERVER, VEILID_SERVER_PORT, _simple_update_callback) as api:
        pass

@pytest.mark.asyncio
async def test_fail_connect():
    with pytest.raises(Exception):
        async with await veilid_python.json_api_connect("fuahwelifuh32luhwafluehawea", 1, _simple_update_callback) as api:
            pass

@pytest.mark.asyncio
async def test_version():
    async with await veilid_python.json_api_connect(VEILID_SERVER, VEILID_SERVER_PORT, _simple_update_callback) as api:
        v = await api.veilid_version()
        print("veilid_version: {}".format(v.__dict__))
        vstr = await api.veilid_version_string()
        print("veilid_version_string: {}".format(vstr))
