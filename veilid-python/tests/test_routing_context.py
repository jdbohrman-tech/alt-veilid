# Routing context veilid tests

import asyncio

import pytest
import veilid
from veilid.types import OperationId

from .conftest import server_info

##################################################################


@pytest.mark.asyncio
async def test_routing_contexts(api_connection: veilid.VeilidAPI):
    rc = await api_connection.new_routing_context()
    async with rc:
        rcp = await rc.with_privacy(release = False)
        async with rcp:
            rcps = await rcp.with_sequencing(veilid.Sequencing.ENSURE_ORDERED, release = False)
            async with rcps:
                rcpscp = await rcps.with_custom_privacy(veilid.Stability.RELIABLE, release = False)
                await rcpscp.release()


@pytest.mark.asyncio
async def test_routing_context_app_message_loopback():
    # Seriously, mypy?
    app_message_queue: asyncio.Queue = asyncio.Queue()

    async def app_message_queue_update_callback(update: veilid.VeilidUpdate):
        if update.kind == veilid.VeilidUpdateKind.APP_MESSAGE:
            await app_message_queue.put(update)

    hostname, port = server_info()
    api = await veilid.json_api_connect(
        hostname, port, app_message_queue_update_callback
    )
    async with api:
        # purge routes to ensure we start fresh
        await api.debug("purge routes")

        # make a routing context that uses a safety route
        rc = await (await api.new_routing_context()).with_privacy()
        async with rc:
        
            # make a new local private route
            prl, blob = await api.new_private_route()

            # import it as a remote route as well so we can send to it
            prr = await api.import_remote_private_route(blob)

            # send an app message to our own private route
            message = b"abcd1234"
            await rc.app_message(prr, message)

            # we should get the same message back
            update: veilid.VeilidUpdate = await asyncio.wait_for(
                app_message_queue.get(), timeout=10
            )

            assert isinstance(update.detail, veilid.VeilidAppMessage)
            assert update.detail.message == message


@pytest.mark.asyncio
async def test_routing_context_app_call_loopback():
    app_call_queue: asyncio.Queue = asyncio.Queue()

    async def app_call_queue_update_callback(update: veilid.VeilidUpdate):
        if update.kind == veilid.VeilidUpdateKind.APP_CALL:
            await app_call_queue.put(update)

    hostname, port = server_info()
    api = await veilid.json_api_connect(hostname, port, app_call_queue_update_callback)
    async with api:
        # purge routes to ensure we start fresh
        await api.debug("purge routes")

        # make a routing context that uses a safety route
        rc = await (await api.new_routing_context()).with_privacy()
        async with rc:
    
            # make a new local private route
            prl, blob = await api.new_private_route()

            # import it as a remote route as well so we can send to it
            prr = await api.import_remote_private_route(blob)

            # send an app message to our own private route
            request = b"abcd1234"
            app_call_task = asyncio.create_task(
                rc.app_call(prr, request), name="app call task"
            )

            # we should get the same request back
            update: veilid.VeilidUpdate = await asyncio.wait_for(
                app_call_queue.get(), timeout=10
            )
            appcall = update.detail

            assert isinstance(appcall, veilid.VeilidAppCall)
            assert appcall.message == request

            # now we reply to the request
            reply = b"qwer5678"
            await api.app_call_reply(appcall.call_id, reply)

            # now we should get the reply from the call
            result = await app_call_task
            assert result == reply
