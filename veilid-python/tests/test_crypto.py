# Crypto veilid_python tests

import veilid_python
import pytest
from . import *

##################################################################

@pytest.mark.asyncio
async def test_best_crypto_system():
    async with await veilid_python.json_api_connect(VEILID_SERVER, VEILID_SERVER_PORT, simple_update_callback) as api:
        bcs = await api.best_crypto_system()
        # let handle dangle for test
        # del bcs
        
@pytest.mark.asyncio
async def test_get_crypto_system():
    async with await veilid_python.json_api_connect(VEILID_SERVER, VEILID_SERVER_PORT, simple_update_callback) as api:
        cs = await api.get_crypto_system(veilid_python.CryptoKind.CRYPTO_KIND_VLD0)
        # clean up handle early
        del cs
        
@pytest.mark.asyncio
async def test_get_crypto_system_invalid():
    async with await veilid_python.json_api_connect(VEILID_SERVER, VEILID_SERVER_PORT, simple_update_callback) as api:
        with pytest.raises(veilid_python.VeilidAPIError):
            cs = await api.get_crypto_system(veilid_python.CryptoKind.CRYPTO_KIND_NONE)

