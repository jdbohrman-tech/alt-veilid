# Basic veilid tests

import veilid
import pytest
from . import *

##################################################################

@pytest.mark.asyncio
async def test_connect():
    async def func(api: veilid.VeilidAPI):
        pass
    await simple_connect_and_run(func)


@pytest.mark.asyncio
async def test_get_node_id():
    async def func(api: veilid.VeilidAPI):
        # get our own node id
        state = await api.get_state()
        node_id = state.config.config.network.routing_table.node_id.pop()
    await simple_connect_and_run(func)

@pytest.mark.asyncio
async def test_fail_connect():
    with pytest.raises(Exception):
        api = await veilid.json_api_connect("fuahwelifuh32luhwafluehawea", 1, simple_update_callback)
        async with api:
            pass

@pytest.mark.asyncio
async def test_version():
    async def func(api: veilid.VeilidAPI):
        v = await api.veilid_version()
        print("veilid_version: {}".format(v.__dict__))
        vstr = await api.veilid_version_string()
        print("veilid_version_string: {}".format(vstr))
    await simple_connect_and_run(func)

