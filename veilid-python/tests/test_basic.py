# Basic veilid_python tests

import veilid_python
import pytest
from . import *

##################################################################

@pytest.mark.asyncio
async def test_connect():
    async with await veilid_python.json_api_connect(VEILID_SERVER, VEILID_SERVER_PORT, simple_update_callback) as api:
        pass

@pytest.mark.asyncio
async def test_fail_connect():
    with pytest.raises(Exception):
        async with await veilid_python.json_api_connect("fuahwelifuh32luhwafluehawea", 1, simple_update_callback) as api:
            pass

@pytest.mark.asyncio
async def test_version():
    async with await veilid_python.json_api_connect(VEILID_SERVER, VEILID_SERVER_PORT, simple_update_callback) as api:
        v = await api.veilid_version()
        print("veilid_version: {}".format(v.__dict__))
        vstr = await api.veilid_version_string()
        print("veilid_version_string: {}".format(vstr))
