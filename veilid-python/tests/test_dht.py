# Routing context veilid tests

import veilid
import pytest
import asyncio
import json
from . import *

##################################################################
BOGUS_KEY = veilid.TypedKey.from_value(veilid.CryptoKind.CRYPTO_KIND_VLD0, veilid.PublicKey.from_bytes(b'                                '))

@pytest.mark.asyncio
async def test_get_dht_value_unopened(api_connection: veilid.VeilidAPI):
    rc = await api_connection.new_routing_context()
    async with rc:
        with pytest.raises(veilid.VeilidAPIError):
            out = await rc.get_dht_value(BOGUS_KEY, veilid.ValueSubkey(0), False)


@pytest.mark.asyncio
async def test_open_dht_record_nonexistent_no_writer(api_connection: veilid.VeilidAPI):
    rc = await api_connection.new_routing_context()
    async with rc:
        with pytest.raises(veilid.VeilidAPIError):
            out = await rc.open_dht_record(BOGUS_KEY, None)

@pytest.mark.asyncio
async def test_close_dht_record_nonexistent(api_connection: veilid.VeilidAPI):
    rc = await api_connection.new_routing_context()
    async with rc:
        with pytest.raises(veilid.VeilidAPIError):
            await rc.close_dht_record(BOGUS_KEY)

@pytest.mark.asyncio
async def test_delete_dht_record_nonexistent(api_connection: veilid.VeilidAPI):
    rc = await api_connection.new_routing_context()
    async with rc:
        with pytest.raises(veilid.VeilidAPIError):
            await rc.delete_dht_record(BOGUS_KEY)
        
@pytest.mark.asyncio
async def test_create_delete_dht_record_simple(api_connection: veilid.VeilidAPI):
    rc = await api_connection.new_routing_context()
    async with rc:
        rec = await rc.create_dht_record(veilid.CryptoKind.CRYPTO_KIND_VLD0, veilid.DHTSchema.dflt(1))
        await rc.close_dht_record(rec.key)
        await rc.delete_dht_record(rec.key)

@pytest.mark.asyncio
async def test_get_dht_value_nonexistent(api_connection: veilid.VeilidAPI):
    rc = await api_connection.new_routing_context()
    async with rc:
        rec = await rc.create_dht_record(veilid.CryptoKind.CRYPTO_KIND_VLD0, veilid.DHTSchema.dflt(1))
        assert await rc.get_dht_value(rec.key, 0, False) == None
        await rc.close_dht_record(rec.key)
        await rc.delete_dht_record(rec.key)

@pytest.mark.asyncio
async def test_set_get_dht_value(api_connection: veilid.VeilidAPI):
    rc = await api_connection.new_routing_context()
    async with rc:
        rec = await rc.create_dht_record(veilid.CryptoKind.CRYPTO_KIND_VLD0, veilid.DHTSchema.dflt(1))
        
        vd = await rc.set_dht_value(rec.key, 0, b"BLAH BLAH BLAH")
        assert vd != None
        
        vd2 = await rc.get_dht_value(rec.key, 0, False)
        assert vd2 != None
        
        print("vd: {}", vd.__dict__)
        print("vd2: {}", vd2.__dict__)

        assert vd == vd2

        await rc.close_dht_record(rec.key)
        await rc.delete_dht_record(rec.key)

