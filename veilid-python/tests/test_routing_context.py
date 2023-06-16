# Routing context veilid tests

import veilid
import pytest
import asyncio
import json
from . import *

##################################################################

@pytest.mark.asyncio
async def test_routing_contexts():
    async def func(api: veilid.VeilidAPI):
        rc = await api.new_routing_context()
        rcp = await rc.with_privacy()
        rcps = await rcp.with_sequencing(veilid.Sequencing.ENSURE_ORDERED)
        rcpsr = await rcps.with_custom_privacy(veilid.Stability.RELIABLE)
    await simple_connect_and_run(func)

@pytest.mark.asyncio
async def test_routing_context_app_message_loopback():

    app_message_queue = asyncio.Queue()

    async def app_message_queue_update_callback(update: veilid.VeilidUpdate):
        if update.kind == veilid.VeilidUpdateKind.APP_MESSAGE:
            await app_message_queue.put(update)

    api = await veilid.json_api_connect(VEILID_SERVER, VEILID_SERVER_PORT, app_message_queue_update_callback)
    async with api:
    
        # make a routing context that uses a safety route
        rc = await (await api.new_routing_context()).with_privacy()

        # get our own node id
        state = await api.get_state()
        node_id = state.config.config.network.routing_table.node_id.pop()

        # send an app message to our node id
        message = b"abcd1234"
        await rc.app_message(node_id, message)

        # we should get the same message back
        #update: veilid.VeilidUpdate = await asyncio.wait_for(app_message_queue.get(), timeout=10)
        #appmsg: veilid.VeilidAppMessage = update.detail
        #assert appmsg.message == message

