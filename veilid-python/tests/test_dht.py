# Routing context veilid tests
from typing import Any, Awaitable, Callable, Optional
import pytest
import asyncio
import time
import os

import veilid
from veilid import ValueSubkey, Timestamp, SafetySelection
from veilid.types import ValueSeqNum, VeilidJSONEncoder

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
            veilid.DHTSchema.dflt(1), kind=veilid.CryptoKind.CRYPTO_KIND_VLD0
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

        #print("vd2: {}", vd2.__dict__)
        #print("vd3: {}", vd3.__dict__)

        assert vd2 == vd3

        await rc.close_dht_record(rec.key)
        await rc.delete_dht_record(rec.key)


@pytest.mark.asyncio
async def test_set_get_dht_value_with_owner(api_connection: veilid.VeilidAPI):
    rc = await api_connection.new_routing_context()
    async with rc:

        cs = await api_connection.best_crypto_system()
        async with cs:
            owner = await cs.generate_key_pair()

        rec = await rc.create_dht_record(veilid.DHTSchema.dflt(2), owner=owner)

        vd = await rc.set_dht_value(rec.key, ValueSubkey(0), b"BLAH BLAH BLAH")
        assert vd is None

        vd2 = await rc.get_dht_value(rec.key, ValueSubkey(0), False)
        assert vd2 is not None

        vd3 = await rc.get_dht_value(rec.key, ValueSubkey(0), True)
        assert vd3 is not None

        vd4 = await rc.get_dht_value(rec.key, ValueSubkey(1), False)
        assert vd4 is None

        #print("vd2: {}", vd2.__dict__)
        #print("vd3: {}", vd3.__dict__)

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
        #print(f"key:{key}")

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
        # Should have prior sequence number as its returned value because it exists online at seq 0
        vdtemp = await rc.set_dht_value(key, ValueSubkey(0), va, veilid.KeyPair.from_parts(owner, secret))
        assert vdtemp is not None
        assert vdtemp.data == vb
        assert vdtemp.seq == 0
        assert vdtemp.writer == owner

        # Should update the second time to seq 1
        vdtemp = await rc.set_dht_value(key, ValueSubkey(0), va, veilid.KeyPair.from_parts(owner, secret))
        assert vdtemp is None

        # Clean up
        await rc.close_dht_record(key)
        await rc.delete_dht_record(key)


@pytest.mark.skipif(os.getenv("INTEGRATION") != "1", reason="integration test requires two servers running")
@pytest.mark.asyncio
async def test_watch_dht_values():

    value_change_queue: asyncio.Queue[veilid.VeilidUpdate] = asyncio.Queue()

    async def value_change_update_callback(update: veilid.VeilidUpdate):
        if update.kind == veilid.VeilidUpdateKind.VALUE_CHANGE:
            await value_change_queue.put(update)

    async def null_update_callback(update: veilid.VeilidUpdate):
        pass

    try:
        api0 = await veilid.api_connector(value_change_update_callback, 0)
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

        # Clear the change queue if record purge cancels old watches
        while True:
            try:
                upd = await asyncio.wait_for(value_change_queue.get(), timeout=3)
            except asyncio.TimeoutError:
                break

        # make routing contexts
        rc0 = await api0.new_routing_context()
        rc1 = await api1.new_routing_context()
        async with rc0, rc1:

            # Server 0: Make a DHT record
            rec0 = await rc0.create_dht_record(veilid.DHTSchema.dflt(10))

            # Server 0: Set some subkey we care about
            vd = await rc0.set_dht_value(rec0.key, ValueSubkey(3), b"BLAH")
            assert vd is None

            await sync(rc0, [rec0])

            # Server 0: Make a watch on all the subkeys
            active = await rc0.watch_dht_values(rec0.key)
            assert active

            # Server 1: Open the subkey
            rec1 = await rc1.open_dht_record(rec0.key, rec0.owner_key_pair())

            # Server 1: Now set the subkey and trigger an update
            vd = await rc1.set_dht_value(rec1.key, ValueSubkey(3), b"BLAH")
            assert vd is None
            await sync(rc1, [rec1])

            # Server 0: Now we should NOT get an update because the update is the same as our local copy
            upd = None
            try:
                upd = await asyncio.wait_for(value_change_queue.get(), timeout=10)
            except asyncio.TimeoutError:
                pass
            assert upd is None

            # Server 1: Now set subkey and trigger an update
            vd = await rc1.set_dht_value(rec1.key, ValueSubkey(3), b"BLAH BLAH")
            assert vd is None
            await sync(rc1, [rec1])

            # Server 0: Wait for the update
            upd = await asyncio.wait_for(value_change_queue.get(), timeout=10)

            # Server 0: Verify the update came back with the first changed subkey's data
            assert upd.detail.key == rec0.key
            assert upd.detail.count == 0xFFFFFFFE
            assert upd.detail.subkeys == [(3, 3)]
            assert upd.detail.value.data == b"BLAH BLAH"

            # Server 1: Now set subkey and trigger an update
            vd = await rc1.set_dht_value(rec1.key, ValueSubkey(4), b"BZORT")
            assert vd is None
            await sync(rc1, [rec1])

            # Server 0: Wait for the update
            upd = await asyncio.wait_for(value_change_queue.get(), timeout=10)

            # Server 0: Verify the update came back with the first changed subkey's data
            assert upd.detail.key == rec0.key
            assert upd.detail.count == 0xFFFFFFFD
            assert upd.detail.subkeys == [(4, 4)]
            assert upd.detail.value.data == b"BZORT"

            # Server 0: Cancel some subkeys we don't care about
            active = await rc0.cancel_dht_watch(rec0.key, [(ValueSubkey(0), ValueSubkey(3))])
            assert active

            # Server 1: Now set multiple subkeys and trigger an update
            vd = await asyncio.gather(*[rc1.set_dht_value(rec1.key, ValueSubkey(3), b"BLAH BLAH BLAH"), rc1.set_dht_value(rec1.key, ValueSubkey(4), b"BZORT BZORT")])
            assert vd == [None, None]
            await sync(rc1, [rec1])

            # Server 0: Wait for the update
            upd = await asyncio.wait_for(value_change_queue.get(), timeout=10)

            # Server 0: Verify only one update came back
            assert upd.detail.key == rec0.key
            assert upd.detail.count == 0xFFFFFFFC
            assert upd.detail.subkeys == [(4, 4)]
            assert upd.detail.value.data == b"BZORT BZORT"

            # Server 0: Now we should NOT get any other update
            upd = None
            try:
                upd = await asyncio.wait_for(value_change_queue.get(), timeout=10)
            except asyncio.TimeoutError:
                pass
            if upd is not None:
                print(f"bad update: {VeilidJSONEncoder.dumps(upd)}")
            assert upd is None

            # Now cancel the update
            active = await rc0.cancel_dht_watch(rec0.key, [(ValueSubkey(3), ValueSubkey(9))])
            assert not active

            # Server 0: Wait for the cancellation update
            upd = await asyncio.wait_for(value_change_queue.get(), timeout=10)

            # Server 0: Verify only one update came back
            assert upd.detail.key == rec0.key
            assert upd.detail.count == 0
            assert upd.detail.subkeys == []
            assert upd.detail.value is None

            # Now set multiple subkeys
            vd = await asyncio.gather(*[rc1.set_dht_value(rec1.key, ValueSubkey(3), b"BLAH BLAH BLAH BLAH"), rc1.set_dht_value(rec1.key, ValueSubkey(5), b"BZORT BZORT BZORT")])
            assert vd == [None, None]
            await sync(rc1, [rec1])

            # Now we should NOT get an update
            upd = None
            try:
                upd = await asyncio.wait_for(value_change_queue.get(), timeout=10)
            except asyncio.TimeoutError:
                pass
            if upd is not None:
                print(f"bad update: {VeilidJSONEncoder.dumps(upd)}")
            assert upd is None

            # Clean up
            await rc1.close_dht_record(rec1.key)
            await rc1.delete_dht_record(rec1.key)
            await rc0.close_dht_record(rec0.key)
            await rc0.delete_dht_record(rec0.key)


@pytest.mark.skipif(os.getenv("INTEGRATION") != "1", reason="integration test requires two servers running")
@pytest.mark.skipif(os.getenv("STRESS") != "1", reason="stress test takes a long time")
@pytest.mark.asyncio
async def test_watch_many_dht_values():

    value_change_queue: asyncio.Queue[veilid.VeilidUpdate] = asyncio.Queue()

    async def value_change_update_callback(update: veilid.VeilidUpdate):
        if update.kind == veilid.VeilidUpdateKind.VALUE_CHANGE:
            await value_change_queue.put(update)

    async def null_update_callback(update: veilid.VeilidUpdate):
        pass

    try:
        api0 = await veilid.api_connector(value_change_update_callback, 0)
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
        # unsafe version for debugging
        rc0 = await (await api0.new_routing_context()).with_safety(SafetySelection.unsafe())
        rc1 = await (await api1.new_routing_context()).with_safety(SafetySelection.unsafe())
        # safe default version
        # rc0 = await api0.new_routing_context()
        # rc1 = await api1.new_routing_context()

        async with rc0, rc1:

            COUNT = 10
            records = []

            # Make and watch all records
            for n in range(COUNT):
                print(f"making record {n}")
                # Server 0: Make a DHT record
                records.append(await rc0.create_dht_record(veilid.DHTSchema.dflt(1)))

                # Server 0: Set some subkey we care about
                vd = await rc0.set_dht_value(records[n].key, ValueSubkey(0), b"BLAH")
                assert vd is None

                # Server 0: Make a watch on all the subkeys
                active = await rc0.watch_dht_values(records[n].key)
                assert active

            # Open and set all records
            missing_records = set()
            for (n, record) in enumerate(records):
                print(f"setting record {n}")

                # Server 1: Open the subkey
                _ignore = await rc1.open_dht_record(record.key, record.owner_key_pair())

                # Server 1: Now set the subkey and trigger an update
                vd = await rc1.set_dht_value(record.key, ValueSubkey(0), b"BLAH BLAH")
                assert vd is None

                missing_records.add(record.key)

            # Server 0: Now we should get an update for every change
            for n in range(len(records)):
                print(f"waiting for change {n}")

                # Server 0: Wait for the update
                try:
                    upd = await asyncio.wait_for(value_change_queue.get(), timeout=10)
                    missing_records.remove(upd.detail.key)
                except:
                    # Dump which records didn't get updates
                    for (m, record) in enumerate(records):
                        if record.key not in missing_records:
                            continue
                        print(f"missing update for record {m}: {record}")
                        info0 = await api0.debug(f"record info {record.key}")
                        info1 = await api1.debug(f"record info {record.key}")
                        print(f"from rc0: {info0}")
                        print(f"from rc1: {info1}")
                    raise

            # Clean up
            for record in records:
                await rc1.close_dht_record(record.key)
                await rc1.delete_dht_record(record.key)
                await rc0.close_dht_record(record.key)
                await rc0.delete_dht_record(record.key)

@pytest.mark.asyncio
async def test_inspect_dht_record(api_connection: veilid.VeilidAPI):
    rc = await api_connection.new_routing_context()
    async with rc:
        rec = await rc.create_dht_record(veilid.DHTSchema.dflt(2))

        vd = await rc.set_dht_value(rec.key, ValueSubkey(0), b"BLAH BLAH BLAH")
        assert vd is None

        rr = await rc.inspect_dht_record(rec.key, [], veilid.DHTReportScope.LOCAL)
        #print("rr: {}", rr.__dict__)
        assert rr.subkeys == [(0, 1)]
        assert rr.local_seqs == [0, None]
        assert rr.network_seqs == [None, None]

        await sync(rc, [rec])

        rr2 = await rc.inspect_dht_record(rec.key, [], veilid.DHTReportScope.SYNC_GET)
        #print("rr2: {}", rr2.__dict__)
        assert rr2.subkeys == [(0, 1)]
        assert rr2.local_seqs == [0, None]
        assert rr2.network_seqs == [0, None]

        await rc.close_dht_record(rec.key)
        await rc.delete_dht_record(rec.key)




async def _run_test_schema_limit(api_connection: veilid.VeilidAPI, open_record: Callable[[veilid.RoutingContext, int], Awaitable[tuple[veilid.DHTRecordDescriptor, Optional[veilid.KeyPair]]]], count: int, test_data: bytes):
    rc = await api_connection.new_routing_context()
    async with rc:
        (desc, writer) = await open_record(rc, count)
        print(f'{desc.key} {writer}')

        # write dht records on server 0
        records = []
        print(f'writing {count} subkeys')
        for n in range(count):
            await rc.set_dht_value(desc.key, ValueSubkey(n), test_data)
            print(f'  {n}')

        await sync(rc, [desc])

        await rc.close_dht_record(desc.key)

        # read dht records on server 0
        print(f'reading {count} subkeys')
        desc1 = await rc.open_dht_record(desc.key)
        for n in range(count):
            vd0 = await rc.get_dht_value(desc1.key, ValueSubkey(n))
            assert vd0.data == test_data
            print(f'  {n}')


@pytest.mark.asyncio
async def test_schema_limit_dflt(api_connection: veilid.VeilidAPI):

    async def open_record(rc: veilid.RoutingContext, count: int) -> tuple[veilid.DHTRecordDescriptor, Optional[veilid.KeyPair]]:
        schema = veilid.DHTSchema.dflt(count)
        desc = await rc.create_dht_record(schema)
        return (desc, desc.owner_key_pair())


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

        return (desc, writer_keypair)

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
                print(f'  {n}: key={desc.key} owner={desc.owner_key_pair()}')

                await rc0.set_dht_value(desc.key, ValueSubkey(0), TEST_DATA)

            await sync(rc0, records)

            # read dht records on server 1
            print(f'reading {COUNT} records')
            n = 0
            for desc in records:
                print(f'  {n}: key={desc.key} owner={desc.owner_key_pair()}')
                n += 1

                desc1 = await rc1.open_dht_record(desc.key)
                vd1 = await rc1.get_dht_value(desc1.key, ValueSubkey(0))
                assert vd1.data == TEST_DATA
                await rc1.close_dht_record(desc1.key)
                


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


@pytest.mark.skipif(os.getenv("STRESS") != "1", reason="stress test takes a long time")
@pytest.mark.asyncio
async def test_dht_write_read_full_subkeys_local():

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

            # Number of records
            COUNT = 8
            # Number of subkeys per record
            SUBKEY_COUNT = 32
            # Nonce to encrypt test data
            NONCE = veilid.Nonce.from_bytes(b"A"*24)
            # Secret to encrypt test data
            SECRET = veilid.SharedSecret.from_bytes(b"A"*32)
            # Max subkey size
            MAX_SUBKEY_SIZE = min(32768, 1024*1024/SUBKEY_COUNT)
            # MAX_SUBKEY_SIZE = 256

            # write dht records on server 0
            records = []
            subkey_data_list = []
            schema = veilid.DHTSchema.dflt(SUBKEY_COUNT)
            print(f'writing {COUNT} records with full subkeys')
            init_futures = set()
            for n in range(COUNT):

                # Make encrypted data that is consistent and hard to compress
                subkey_data = bytes(chr(ord("A")+n)*MAX_SUBKEY_SIZE, 'ascii')
                print(f"subkey_data({n}):len={len(subkey_data)}")

                cs = await api0.best_crypto_system()
                async with cs:
                    subkey_data = await cs.crypt_no_auth(subkey_data, NONCE, SECRET)
                subkey_data_list.append(subkey_data)


                desc = await rc0.create_dht_record(schema)
                records.append(desc)

                for i in range(SUBKEY_COUNT):
                    init_futures.add(rc0.set_dht_value(desc.key, ValueSubkey(i), subkey_data))

                print(f'  {n}: {desc.key} {desc.owner}:{desc.owner_secret}')

            # Wait for all records to synchronize, with progress bars
            await sync_win(rc0, records, SUBKEY_COUNT, init_futures)

            for desc0 in records:
                await rc0.close_dht_record(desc0.key)

            await api0.debug("record purge local")
            await api0.debug("record purge remote")

            # read dht records on server 0
            print(f'reading {COUNT} records')
            for n, desc0 in enumerate(records):
                desc1 = await rc0.open_dht_record(desc0.key)

                for i in range(SUBKEY_COUNT):
                    vd0 = None
                    while vd0 == None:
                        vd0 = await rc0.get_dht_value(desc1.key, ValueSubkey(i), force_refresh=True)
                        if vd0 != None:
                            assert vd0.data == subkey_data_list[n]
                            break
                        time.sleep(1)
                        print(f"retrying record {n} subkey {i}")


                await rc0.close_dht_record(desc1.key)

                print(f'  {n}')


async def sync(rc: veilid.RoutingContext, records: list[veilid.DHTRecordDescriptor]):
    syncrecords = records.copy()
    if len(syncrecords) == 0:
        return
    while True:
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
        if len(syncrecords) == 0:
            break
        print(f'  syncing {len(syncrecords)} records {subkeysleft} subkeys left')
        time.sleep(1)


async def sync_win(
        rc: veilid.RoutingContext,
        records: list[veilid.DHTRecordDescriptor],
        subkey_count: int,
        init_futures: set[Awaitable[Any]]
    ):
    import curses

    screen = curses.initscr()

    curses.start_color()
    curses.init_pair(1, curses.COLOR_BLACK, curses.COLOR_BLUE)
    curses.init_pair(2, curses.COLOR_BLACK, curses.COLOR_CYAN)
    curses.init_pair(3, curses.COLOR_BLACK, curses.COLOR_YELLOW)
    curses.init_pair(4, curses.COLOR_BLACK, curses.COLOR_GREEN)

    HEIGHT=len(records) + 3
    GRAPHWIDTH = subkey_count
    WIDTH=GRAPHWIDTH + 4 + 1 + 43 + 2

    cur_lines = curses.LINES
    cur_cols = curses.COLS
    win = curses.newwin(HEIGHT, WIDTH,
        max(0, int(cur_lines/2) - int(HEIGHT/2)),
        max(0, int(cur_cols/2) - int(WIDTH/2)))
    win.clear()
    win.border(0,0,0,0)
    win.nodelay(True)

    # Record inspection and completion state

    # Records we are done inspecting and have finished sync
    donerecords: set[veilid.TypedKey] = set()
    # Records we are currently inspecting that are in the futures set
    futurerecords: set[veilid.TypedKey] = set()
    # All the futures we are waiting for
    futures = set()
    # The record report state
    recordreports: dict[veilid.TypedKey, veilid.DHTRecordReport] = dict()

    # Add initial futures with None key
    for fut in init_futures:
        async def _do_init_fut(fut):
            return (None, await fut)
        futures.add(asyncio.create_task(_do_init_fut(fut)))

    # Loop until all records are completed
    while len(donerecords) != len(records):

        # Update the futures with inspects for unfinished records
        for n, desc in enumerate(records):
            if desc.key in donerecords or desc.key in futurerecords:
                continue
            async def _do_inspect(key: veilid.TypedKey):
                return (key, await rc.inspect_dht_record(key, []))
            futures.add(asyncio.create_task(_do_inspect(desc.key)))
            futurerecords.add(desc.key)

        # Wait for some futures to finish
        done, futures = await asyncio.wait(futures, return_when = asyncio.FIRST_COMPLETED)

        # Process finished futures into the state
        for rr_fut in done:
            key: veilid.TypedKey
            rr: veilid.DHTRecordReport
            key, rr = await rr_fut
            if key is not None:
                futurerecords.remove(key)

                if len(rr.subkeys) == 1 and rr.subkeys[0] == (0, subkey_count-1) and None not in rr.local_seqs and len(rr.offline_subkeys) == 0:
                    if key in recordreports:
                        del recordreports[key]
                    donerecords.add(key)
                else:
                    recordreports[key] = rr

        # Re-render the state
        if cur_lines != curses.LINES or cur_cols != curses.COLS:
            cur_lines = curses.LINES
            cur_cols = curses.COLS
            win.move(
                max(0, int(cur_lines/2) - int(HEIGHT/2)),
                max(0, int(cur_cols/2) - int(WIDTH/2)))
            win.border(0,0,0,0)
        win.addstr(1, 1, "syncing records to the network", curses.color_pair(0))
        for n, rr in enumerate(records):
            key = rr.key
            win.addstr(n+2, GRAPHWIDTH+1, key, curses.color_pair(0))

            if key in donerecords:
                win.addstr(n+2, 1, " " * subkey_count, curses.color_pair(4))
            elif key in recordreports:
                rr = recordreports[key]
                win.addstr(n+2, 1, " " * subkey_count, curses.color_pair(1))
                for (a,b) in rr.subkeys:
                    for m in range(a, b+1):
                        if rr.local_seqs[m] != None:
                            win.addstr(n+2, m+1, " ", curses.color_pair(2))
                for (a,b) in rr.offline_subkeys:
                    win.addstr(n+2, a+1, " " * (b-a+1), curses.color_pair(3))
            else:
                win.addstr(n+2, 1, " " * subkey_count, curses.color_pair(1))

        win.refresh()
        time.sleep(.5)


    curses.endwin()

