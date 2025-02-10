# Routing context veilid tests

from typing import Awaitable, Callable
import pytest
import asyncio
import time
import os

import veilid
from veilid import ValueSubkey

##################################################################
BOGUS_KEY = veilid.TypedKey.from_value(
    veilid.CryptoKind.CRYPTO_KIND_VLD0, veilid.PublicKey.from_bytes(b'                                '))


@pytest.mark.asyncio
async def test_get_dht_value_unopened(api_connection: veilid.VeilidAPI):
    rc = await api_connection.new_routing_context()
    
    async with rc:
        with pytest.raises(veilid.VeilidAPIError):
            await rc.get_dht_value(BOGUS_KEY, ValueSubkey(0), False)


@pytest.mark.asyncio
async def test_open_dht_record_nonexistent_no_writer(api_connection: veilid.VeilidAPI):
    rc = await api_connection.new_routing_context()
    async with rc:
        with pytest.raises(veilid.VeilidAPIError):
            await rc.open_dht_record(BOGUS_KEY, None)


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
        rec = await rc.create_dht_record(
            veilid.DHTSchema.dflt(1), veilid.CryptoKind.CRYPTO_KIND_VLD0
        )
        await rc.close_dht_record(rec.key)
        await rc.delete_dht_record(rec.key)


@pytest.mark.asyncio
async def test_get_dht_value_nonexistent(api_connection: veilid.VeilidAPI):
    rc = await api_connection.new_routing_context()
    async with rc:
        rec = await rc.create_dht_record(veilid.DHTSchema.dflt(1))
        assert await rc.get_dht_value(rec.key, ValueSubkey(0), False) is None
        await rc.close_dht_record(rec.key)
        await rc.delete_dht_record(rec.key)


@pytest.mark.asyncio
async def test_set_get_dht_value(api_connection: veilid.VeilidAPI):
    rc = await api_connection.new_routing_context()
    async with rc:
        rec = await rc.create_dht_record(veilid.DHTSchema.dflt(2))

        vd = await rc.set_dht_value(rec.key, ValueSubkey(0), b"BLAH BLAH BLAH")
        assert vd is None

        vd2 = await rc.get_dht_value(rec.key, ValueSubkey(0), False)
        assert vd2 is not None

        vd3 = await rc.get_dht_value(rec.key, ValueSubkey(0), True)
        assert vd3 is not None

        vd4 = await rc.get_dht_value(rec.key, ValueSubkey(1), False)
        assert vd4 is None

        print("vd2: {}", vd2.__dict__)
        print("vd3: {}", vd3.__dict__)

        assert vd2 == vd3

        await rc.close_dht_record(rec.key)
        await rc.delete_dht_record(rec.key)


@pytest.mark.asyncio
async def test_open_writer_dht_value(api_connection: veilid.VeilidAPI):
    rc = await api_connection.new_routing_context()
    async with rc:
        rec = await rc.create_dht_record(veilid.DHTSchema.dflt(2))
        key = rec.key
        owner = rec.owner
        secret = rec.owner_secret
        print(f"key:{key}")

        cs = await api_connection.get_crypto_system(rec.key.kind())
        async with cs:
            assert await cs.validate_key_pair(owner, secret)
            other_keypair = await cs.generate_key_pair()

        va = b"Qwertyuiop Asdfghjkl Zxcvbnm"
        vb = b"1234567890"
        vc = b"!@#$%^&*()"

        # Test subkey writes
        vdtemp = await rc.set_dht_value(key, ValueSubkey(1), va)
        assert vdtemp is None

        vdtemp = await rc.get_dht_value(key, ValueSubkey(1), False)
        assert vdtemp.data == va
        assert vdtemp.seq == 0
        assert vdtemp.writer == owner

        vdtemp = await rc.get_dht_value(key, ValueSubkey(0), False)
        assert vdtemp is None

        vdtemp = await rc.set_dht_value(key, ValueSubkey(0), vb)
        assert vdtemp is None

        await sync(rc, [rec])

        vdtemp = await rc.get_dht_value(key, ValueSubkey(0), True)
        assert vdtemp.data == vb

        vdtemp = await rc.get_dht_value(key, ValueSubkey(1), True)
        assert vdtemp.data == va

        # Equal value should not trigger sequence number update
        vdtemp = await rc.set_dht_value(key, ValueSubkey(1), va)
        assert vdtemp is None

        # Different value should trigger sequence number update
        vdtemp = await rc.set_dht_value(key, ValueSubkey(1), vb)
        assert vdtemp is None

        # Now that we initialized some subkeys
        # and verified they stored correctly
        # Delete things locally and reopen and see if we can write
        # with the same writer key
        await sync(rc, [rec])

        await rc.close_dht_record(key)
        await rc.delete_dht_record(key)

        rec = await rc.open_dht_record(key, veilid.KeyPair.from_parts(owner, secret))
        assert rec is not None
        assert rec.key == key
        assert rec.owner == owner
        assert rec.owner_secret == secret
        assert rec.schema.kind == veilid.DHTSchemaKind.DFLT
        assert rec.schema.o_cnt == 2

        # Verify subkey 1 can be set before it is get but newer is available online
        vdtemp = await rc.set_dht_value(key, ValueSubkey(1), vc)
        assert vdtemp is not None
        assert vdtemp.data == vb
        assert vdtemp.seq == 1
        assert vdtemp.writer == owner

        # Verify subkey 1 can be set a second time and it updates because seq is newer
        vdtemp = await rc.set_dht_value(key, ValueSubkey(1), vc)
        assert vdtemp is None

        await sync(rc, [rec])

        # Verify the network got the subkey update with a refresh check
        vdtemp = await rc.get_dht_value(key, ValueSubkey(1), True)
        assert vdtemp is not None
        assert vdtemp.data == vc
        assert vdtemp.seq == 2
        assert vdtemp.writer == owner

        # Delete things locally and reopen and see if we can write
        # with a different writer key (should fail)

        await rc.close_dht_record(key)
        await rc.delete_dht_record(key)

        rec = await rc.open_dht_record(key, other_keypair)
        assert rec is not None
        assert rec.key == key
        assert rec.owner == owner
        assert rec.owner_secret is None
        assert rec.schema.kind == veilid.DHTSchemaKind.DFLT
        assert rec.schema.o_cnt == 2

        # Verify subkey 1 can NOT be set because we have the wrong writer
        with pytest.raises(veilid.VeilidAPIError):
            await rc.set_dht_value(key, ValueSubkey(1), va)

        # Verify subkey 0 can NOT be set because we have the wrong writer
        with pytest.raises(veilid.VeilidAPIError):
            await rc.set_dht_value(key, ValueSubkey(0), va)

        # Verify subkey 0 can be set because override with the right writer
        vdtemp = await rc.set_dht_value(key, ValueSubkey(0), va, veilid.KeyPair.from_parts(owner, secret))
        assert vdtemp is None

        # Clean up
        await rc.close_dht_record(key)
        await rc.delete_dht_record(key)


# @pytest.mark.skipif(os.getenv("INTEGRATION") != "1", reason="integration test requires two servers running")
@pytest.mark.skip(reason = "don't work yet")
@pytest.mark.asyncio
async def test_watch_dht_values():

    value_change_queue: asyncio.Queue[veilid.VeilidUpdate] = asyncio.Queue()

    async def value_change_update_callback(update: veilid.VeilidUpdate):
        if update.kind == veilid.VeilidUpdateKind.VALUE_CHANGE:
            await value_change_queue.put(update)

    try:
        api = await veilid.api_connector(value_change_update_callback)
    except veilid.VeilidConnectionError:
        pytest.skip("Unable to connect to veilid-server.")

    # Make two routing contexts, one with and one without safety
    # So we can pretend to be a different node and get the watch updates
    # Normally they would not get sent if the set comes from the same target
    # as the watch's target
    
    # XXX: this logic doesn't work because our node still suppresses updates
    # XXX: if the value hasn't changed in the local record store
    rcWatch = await api.new_routing_context()
    
    rcSet = await (await api.new_routing_context()).with_safety(veilid.SafetySelection.unsafe())
    async with rcWatch, rcSet:
        # Make a DHT record
        rec = await rcWatch.create_dht_record(veilid.DHTSchema.dflt(10))

        # Set some subkey we care about
        vd = await rcWatch.set_dht_value(rec.key, ValueSubkey(3), b"BLAH BLAH BLAH")
        assert vd is None

        # Make a watch on that subkey
        ts = await rcWatch.watch_dht_values(rec.key, [], 0, 0xFFFFFFFF)
        assert ts != 0

        # Reopen without closing to change routing context and not lose watch
        rec = await rcSet.open_dht_record(rec.key, rec.owner_key_pair())
        
        # Now set the subkey and trigger an update
        vd = await rcSet.set_dht_value(rec.key, ValueSubkey(3), b"BLAH")
        assert vd is None
        
        # Now we should NOT get an update because the update is the same as our local copy
        update = None
        try:
            update = await asyncio.wait_for(value_change_queue.get(), timeout=5)
        except asyncio.TimeoutError:
            pass
        assert update is None

        # Now set multiple subkeys and trigger an update
        vd = await asyncio.gather(*[rcSet.set_dht_value(rec.key, ValueSubkey(3), b"BLAH BLAH"), rcSet.set_dht_value(rec.key, ValueSubkey(4), b"BZORT")])
        assert vd == [None, None]

        # Wait for the update
        upd = await asyncio.wait_for(value_change_queue.get(), timeout=5)

        # Verify the update came back but we don't get a new value because the sequence number is the same
        assert upd.detail.key == rec.key
        assert upd.detail.count == 0xFFFFFFFD
        assert upd.detail.subkeys == [(3, 4)]
        assert upd.detail.value is None

        # Reopen without closing to change routing context and not lose watch
        rec = await rcWatch.open_dht_record(rec.key, rec.owner_key_pair())

        # Cancel some subkeys we don't care about
        still_active = await rcWatch.cancel_dht_watch(rec.key, [(ValueSubkey(0), ValueSubkey(2))])
        assert still_active

        # Reopen without closing to change routing context and not lose watch
        rec = await rcSet.open_dht_record(rec.key, rec.owner_key_pair())

        # Now set multiple subkeys and trigger an update
        vd = await asyncio.gather(*[rcSet.set_dht_value(rec.key, ValueSubkey(3), b"BLAH BLAH BLAH"), rcSet.set_dht_value(rec.key, ValueSubkey(5), b"BZORT BZORT")])
        assert vd == [None, None]

        # Wait for the update, this longer timeout seems to help the flaky check below
        upd = await asyncio.wait_for(value_change_queue.get(), timeout=10)

        # Verify the update came back but we don't get a new value because the sequence number is the same
        assert upd.detail.key == rec.key

        # This check is flaky on slow connections and often fails with different counts
        assert upd.detail.count == 0xFFFFFFFC
        assert upd.detail.subkeys == [(3, 3), (5, 5)]
        assert upd.detail.value is None

        # Reopen without closing to change routing context and not lose watch
        rec = await rcWatch.open_dht_record(rec.key, rec.owner_key_pair())

        # Now cancel the update
        still_active = await rcWatch.cancel_dht_watch(rec.key, [(ValueSubkey(3), ValueSubkey(9))])
        assert not still_active

        # Reopen without closing to change routing context and not lose watch
        rec = await rcSet.open_dht_record(rec.key, rec.owner_key_pair())

        # Now set multiple subkeys
        vd = await asyncio.gather(*[rcSet.set_dht_value(rec.key, ValueSubkey(3), b"BLAH BLAH BLAH BLAH"), rcSet.set_dht_value(rec.key, ValueSubkey(5), b"BZORT BZORT BZORT")])
        assert vd == [None, None]
        
        # Now we should NOT get an update
        update = None
        try:
            update = await asyncio.wait_for(value_change_queue.get(), timeout=5)
        except asyncio.TimeoutError:
            pass
        assert update is None

        # Clean up
        await rcSet.close_dht_record(rec.key)
        await rcSet.delete_dht_record(rec.key)


@pytest.mark.asyncio
async def test_inspect_dht_record(api_connection: veilid.VeilidAPI):
    rc = await api_connection.new_routing_context()
    async with rc:
        rec = await rc.create_dht_record(veilid.DHTSchema.dflt(2))

        vd = await rc.set_dht_value(rec.key, ValueSubkey(0), b"BLAH BLAH BLAH")
        assert vd is None

        rr = await rc.inspect_dht_record(rec.key, [], veilid.DHTReportScope.LOCAL)
        print("rr: {}", rr.__dict__)
        assert rr.subkeys == [[0,1]]
        assert rr.local_seqs == [0, 0xFFFFFFFF]
        assert rr.network_seqs == []

        rr2 = await rc.inspect_dht_record(rec.key, [], veilid.DHTReportScope.SYNC_GET)
        print("rr2: {}", rr2.__dict__)
        assert rr2.subkeys == [[0,1]]
        assert rr2.local_seqs == [0, 0xFFFFFFFF]
        assert rr2.network_seqs == [0, 0xFFFFFFFF]

        await rc.close_dht_record(rec.key)
        await rc.delete_dht_record(rec.key)




async def _run_test_schema_limit(api_connection: veilid.VeilidAPI, open_record: Callable[[veilid.RoutingContext, int], Awaitable[tuple[veilid.TypedKey, veilid.PublicKey, veilid.SecretKey]]], count: int, test_data: bytes, ):
    rc = await api_connection.new_routing_context()
    async with rc:
        (key, owner, secret) = await open_record(rc, count)
        print(f'{key} {owner}:{secret}')

        # write dht records on server 0
        records = []
        print(f'writing {count} subkeys')
        for n in range(count):
            await rc.set_dht_value(key, ValueSubkey(n), test_data)
            print(f'  {n}')

        print('syncing records to the network')

        while True:
            donerecords = set()
            subkeysleft = 0

            rr = await rc.inspect_dht_record(key, [])
            left = 0; [left := left + (x[1]-x[0]+1) for x in rr.offline_subkeys]
            if left == 0:
                break
            print(f'  {left} subkeys left')
            time.sleep(1)

        await rc.close_dht_record(key)

        await api_connection.debug("record purge local")
        await api_connection.debug("record purge remote")

        # read dht records on server 0
        print(f'reading {count} subkeys')
        desc1 = await rc.open_dht_record(key)
        for n in range(count):
            vd0 = await rc.get_dht_value(key, ValueSubkey(n), force_refresh=True)
            assert vd0.data == test_data
            print(f'  {n}')


@pytest.mark.asyncio
async def test_schema_limit_dflt(api_connection: veilid.VeilidAPI):

    async def open_record(rc: veilid.RoutingContext, count: int) -> tuple[veilid.TypedKey, veilid.PublicKey, veilid.SecretKey]:
        schema = veilid.DHTSchema.dflt(count)
        desc = await rc.create_dht_record(schema)
        return (desc.key, desc.owner, desc.owner_secret)


    print("Test with maximum number of subkeys before lower limit hit")
    TEST_DATA = b"A" * 32768
    COUNT = 32
    await _run_test_schema_limit(api_connection, open_record, COUNT, TEST_DATA)

    print("Test with maximum number of subkeys before lower limit hit + 1 extra byte")
    with pytest.raises(Exception):
        TEST_DATA = b"A" * 32769
        COUNT = 32
        await _run_test_schema_limit(api_connection, open_record, COUNT, TEST_DATA)

    print("Test with minimum number of subkeys with lower limit exceeded")
    TEST_DATA = b"A" * 31775
    COUNT = 33
    await _run_test_schema_limit(api_connection, open_record, COUNT, TEST_DATA)

    print("Test with minimum number of subkeys with lower limit exceeded + 1 extra byte")
    with pytest.raises(Exception):
        TEST_DATA = b"A" * 31776
        COUNT = 33
        await _run_test_schema_limit(api_connection, open_record, COUNT, TEST_DATA)


@pytest.mark.asyncio
async def test_schema_limit_smpl(api_connection: veilid.VeilidAPI):

    async def open_record(rc: veilid.RoutingContext, count: int) -> tuple[veilid.TypedKey, veilid.PublicKey, veilid.SecretKey]:
        cs = await api_connection.best_crypto_system()
        async with cs:
            writer_keypair = await cs.generate_key_pair()

        schema = veilid.DHTSchema.smpl(0, [veilid.DHTSchemaSMPLMember(writer_keypair.key(), count)])
        desc = await rc.create_dht_record(schema)
        await rc.open_dht_record(desc.key, writer_keypair)

        return (desc.key, writer_keypair.key(), writer_keypair.secret())

    print("Test with maximum number of subkeys before lower limit hit")
    TEST_DATA = b"A" * 32768
    COUNT = 32
    await _run_test_schema_limit(api_connection, open_record, COUNT, TEST_DATA)

    print("Test with maximum number of subkeys before lower limit hit + 1 extra byte")
    with pytest.raises(Exception):
        TEST_DATA = b"A" * 32769
        COUNT = 32
        await _run_test_schema_limit(api_connection, open_record, COUNT, TEST_DATA)

    print("Test with minimum number of subkeys with lower limit exceeded")
    TEST_DATA = b"A" * 31775
    COUNT = 33
    await _run_test_schema_limit(api_connection, open_record, COUNT, TEST_DATA)

    print("Test with minimum number of subkeys with lower limit exceeded + 1 extra byte")
    with pytest.raises(Exception):
        TEST_DATA = b"A" * 31776
        COUNT = 33
        await _run_test_schema_limit(api_connection, open_record, COUNT, TEST_DATA)





@pytest.mark.skipif(os.getenv("INTEGRATION") != "1", reason="integration test requires two servers running")
@pytest.mark.asyncio
async def test_dht_integration_writer_reader():
    
    async def null_update_callback(update: veilid.VeilidUpdate):
        pass    

    try:
        api0 = await veilid.api_connector(null_update_callback, 0)
    except veilid.VeilidConnectionError:
        pytest.skip("Unable to connect to veilid-server 0.")

    try:
        api1 = await veilid.api_connector(null_update_callback, 1)
    except veilid.VeilidConnectionError:
        pytest.skip("Unable to connect to veilid-server 1.")

    async with api0, api1:
        # purge local and remote record stores to ensure we start fresh
        await api0.debug("record purge local")
        await api0.debug("record purge remote")
        await api1.debug("record purge local")
        await api1.debug("record purge remote")

        # make routing contexts
        rc0 = await api0.new_routing_context()
        rc1 = await api1.new_routing_context()
        async with rc0, rc1:

            COUNT = 100
            TEST_DATA = b"test data"

            # write dht records on server 0
            records = []
            schema = veilid.DHTSchema.dflt(1)
            print(f'writing {COUNT} records')
            for n in range(COUNT):
                desc = await rc0.create_dht_record(schema)
                records.append(desc)

                await rc0.set_dht_value(desc.key, ValueSubkey(0), TEST_DATA)

                print(f'  {n}')
            
            print('syncing records to the network')
            recleft = len(records)
            for desc0 in records:
                while True:
                    rr = await rc0.inspect_dht_record(desc0.key, [])
                    left = 0; [left := left + (x[1]-x[0]+1) for x in rr.offline_subkeys]
                    if left == 0:
                        await rc0.close_dht_record(desc0.key)
                        break
                    print(f'  {recleft} records {left} subkeys left')
                    time.sleep(0.1)
                recleft-=1

            # read dht records on server 1
            print(f'reading {COUNT} records')
            n = 0
            for desc0 in records:
                desc1 = await rc1.open_dht_record(desc0.key)
                vd1 = await rc1.get_dht_value(desc1.key, ValueSubkey(0))
                assert vd1.data == TEST_DATA
                await rc1.close_dht_record(desc1.key)
                
                print(f'  {n}')
                n += 1


@pytest.mark.skipif(os.getenv("STRESS") != "1", reason="stress test takes a long time")
@pytest.mark.asyncio
async def test_dht_write_read_local():
    
    async def null_update_callback(update: veilid.VeilidUpdate):
        pass

    try:
        api0 = await veilid.api_connector(null_update_callback, 0)
    except veilid.VeilidConnectionError:
        pytest.skip("Unable to connect to veilid-server 0.")

    async with api0:
        # purge local and remote record stores to ensure we start fresh
        await api0.debug("record purge local")
        await api0.debug("record purge remote")
        
        # make routing contexts
        rc0 = await api0.new_routing_context()
        async with rc0:

            # Previously COUNT was set to 500, which causes these tests to take
            # 10s of minutes on slow connections or debug veilid-server builds
            COUNT = 100
            TEST_DATA = b"ABCD"*1024
            TEST_DATA2 = b"ABCD"*4096

            # write dht records on server 0
            records = []
            schema = veilid.DHTSchema.dflt(2)
            print(f'writing {COUNT} records')
            for n in range(COUNT):
                desc = await rc0.create_dht_record(schema)
                records.append(desc)

                await rc0.set_dht_value(desc.key, ValueSubkey(0), TEST_DATA)
                await rc0.set_dht_value(desc.key, ValueSubkey(1), TEST_DATA2)

                print(f'  {n}: {desc.key} {desc.owner}:{desc.owner_secret}')
            
            await sync(rc0, records)

            for desc0 in records:
                await rc0.close_dht_record(desc0.key)

            await api0.debug("record purge local")
            await api0.debug("record purge remote")

            # read dht records on server 0
            print(f'reading {COUNT} records')
            n = 0
            for desc0 in records:
                desc1 = await rc0.open_dht_record(desc0.key)
                
                vd0 = await rc0.get_dht_value(desc1.key, ValueSubkey(0), force_refresh=True)
                assert vd0.data == TEST_DATA
                
                vd1 = await rc0.get_dht_value(desc1.key, ValueSubkey(1), force_refresh=True)
                assert vd1.data == TEST_DATA2
                await rc0.close_dht_record(desc1.key)
                
                print(f'  {n}')
                n += 1

async def sync(rc: veilid.RoutingContext, records: list[veilid.DHTRecordDescriptor]):
    print('syncing records to the network')
    syncrecords = records.copy()
    while len(syncrecords) > 0:
        donerecords = set()
        subkeysleft = 0
        for desc in records:
            rr = await rc.inspect_dht_record(desc.key, [])
            left = 0; [left := left + (x[1]-x[0]+1) for x in rr.offline_subkeys]
            if left == 0:
                donerecords.add(desc)
            else:
                subkeysleft += left
        syncrecords = [x for x in syncrecords if x not in donerecords]
        print(f'  {len(syncrecords)} records {subkeysleft} subkeys left')
        time.sleep(1)
