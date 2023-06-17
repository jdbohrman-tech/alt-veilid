# Crypto veilid tests

import pytest
import veilid
from veilid.api import CryptoSystem

from . import api_connection

##################################################################


@pytest.mark.asyncio
async def test_best_crypto_system(api_connection):
    bcs: CryptoSystem = await api_connection.best_crypto_system()

    assert await bcs.default_salt_length() == 16


@pytest.mark.asyncio
async def test_get_crypto_system(api_connection):
    cs: CryptoSystem = await api_connection.get_crypto_system(
        veilid.CryptoKind.CRYPTO_KIND_VLD0
    )

    assert await cs.default_salt_length() == 16

    # clean up handle early
    del cs


@pytest.mark.asyncio
async def test_get_crypto_system_invalid(api_connection):
    with pytest.raises(veilid.VeilidAPIErrorInvalidArgument) as exc:
        await api_connection.get_crypto_system(veilid.CryptoKind.CRYPTO_KIND_NONE)

    assert exc.value.context == "unsupported cryptosystem"
    assert exc.value.argument == "kind"
    assert exc.value.value == "NONE"


@pytest.mark.asyncio
async def test_hash_and_verify_password(api_connection):
    bcs = await api_connection.best_crypto_system()
    nonce = await bcs.random_nonce()
    salt = nonce.to_bytes()

    # Password match
    phash = await bcs.hash_password(b"abc123", salt)
    assert await bcs.verify_password(b"abc123", phash)

    # Password mismatch
    phash2 = await bcs.hash_password(b"abc1234", salt)
    assert not await bcs.verify_password(b"abc12345", phash)
