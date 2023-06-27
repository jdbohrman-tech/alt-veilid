import os
from functools import cache
from typing import AsyncGenerator

import pytest_asyncio
import veilid
from veilid.json_api import _JsonVeilidAPI

pytest_plugins = ("pytest_asyncio",)


@cache
def server_info() -> tuple[str, int]:
    """Return the hostname and port of the test server."""
    VEILID_SERVER = os.getenv("VEILID_SERVER")
    if VEILID_SERVER is None:
        return "localhost", 5959

    hostname, *rest = VEILID_SERVER.split(":")
    if rest:
        return hostname, int(rest[0])
    return hostname, 5959


async def simple_update_callback(update: veilid.VeilidUpdate):
    print(f"VeilidUpdate: {update}")


@pytest_asyncio.fixture
async def api_connection() -> AsyncGenerator[_JsonVeilidAPI, None]:
    hostname, port = server_info()
    api = await veilid.json_api_connect(hostname, port, simple_update_callback)
    async with api:
        # purge routes to ensure we start fresh
        await api.debug("purge routes")

        yield api
