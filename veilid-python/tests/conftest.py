"""Common test fixtures."""

from typing import AsyncGenerator

import pytest
import pytest_asyncio

import veilid
from veilid.json_api import _JsonVeilidAPI

pytest_plugins = ("pytest_asyncio",)

async def simple_update_callback(update: veilid.VeilidUpdate):
    pass


@pytest_asyncio.fixture
async def api_connection() -> AsyncGenerator[_JsonVeilidAPI, None]:
    try:
        api = await veilid.api_connector(simple_update_callback)
    except veilid.VeilidConnectionError:
        pytest.skip("Unable to connect to veilid-server.")

    async with api:
        # purge routes to ensure we start fresh
        await api.debug("purge routes")

        yield api
