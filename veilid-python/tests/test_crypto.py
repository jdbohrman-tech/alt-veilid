# Crypto veilid tests

import veilid
import pytest
from . import *

##################################################################

@pytest.mark.asyncio
async def test_best_crypto_system():
    async def func(api: veilid.VeilidAPI):
        bcs = await api.best_crypto_system()
    await simple_connect_and_run(func)
        
@pytest.mark.asyncio
async def test_get_crypto_system():
    async def func(api: veilid.VeilidAPI):
        cs = await api.get_crypto_system(veilid.CryptoKind.CRYPTO_KIND_VLD0)
        # clean up handle early
        del cs
    await simple_connect_and_run(func)
        
@pytest.mark.asyncio
async def test_get_crypto_system_invalid():
    async def func(api: veilid.VeilidAPI):
        with pytest.raises(veilid.VeilidAPIError):
            cs = await api.get_crypto_system(veilid.CryptoKind.CRYPTO_KIND_NONE)
    await simple_connect_and_run(func)

@pytest.mark.asyncio
async def test_hash_and_verify_password():
    async def func(api: veilid.VeilidAPI):
        bcs = await api.best_crypto_system()
        nonce = await bcs.random_nonce()
        salt = nonce.to_bytes()
        # Password match
        phash = await bcs.hash_password(b"abc123", salt)
        assert await bcs.verify_password(b"abc123", phash)
        # Password mismatch
        phash2 = await bcs.hash_password(b"abc1234", salt)
        assert not await bcs.verify_password(b"abc12345", phash)
    await simple_connect_and_run(func)
