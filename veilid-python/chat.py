#!/usr/bin/env python

import asyncio
import sys

import veilid

QUIT = b"QUIT"


async def cb(*args, **kwargs):
    return
    print(f"{args=}")
    print(f"{kwargs=}")


async def chatter(rc: veilid.api.RoutingContext, key, send_channel: int, recv_channel: int):
    last_seq = -1

    send_subkey = veilid.types.ValueSubkey(send_channel)
    recv_subkey = veilid.types.ValueSubkey(recv_channel)

    while True:
        try:
            msg = input("SEND> ")
        except EOFError:
            print("Closing the chat.")
            await rc.set_dht_value(key, send_subkey, QUIT)
            return

        await rc.set_dht_value(key, send_subkey, msg.encode())

        while True:
            resp = await rc.get_dht_value(key, recv_subkey, True)
            if resp is None:
                continue
            if resp.seq == last_seq:
                continue

            if resp.data == QUIT:
                print("Other end closed the chat.")
                return

            print(f"RECV< {resp.data.decode()}")
            last_seq = resp.seq
            break


async def start():
    conn = await veilid.json_api_connect("localhost", 5959, cb)

    rc = await conn.new_routing_context()
    async with rc:
        rec = await rc.create_dht_record(veilid.DHTSchema.dflt(2))
        print(f"Chat key: {rec.key}")
        print(rec.owner)
        print(vars(rec))

        await chatter(rc, rec.key, 0, 1)

        await rc.close_dht_record(rec.key)
        await rc.delete_dht_record(rec.key)


async def respond(key, writer):
    conn = await veilid.json_api_connect("localhost", 5959, cb)

    rc = await conn.new_routing_context()
    async with rc:
        await chatter(rc, key, 1, 0)


async def clean(key):
    conn = await veilid.json_api_connect("localhost", 5959, cb)

    rc = await conn.new_routing_context()
    async with rc:
        await rc.close_dht_record(key)
        await rc.delete_dht_record(key)


if __name__ == "__main__":
    if sys.argv[1] == "--start":
        func = start()
    elif sys.argv[1] == "--respond":
        func = respond(sys.argv[2], sys.argv[3])
    elif sys.argv[1] == "--clean":
        func = clean(sys.argv[2])
    else:
        1 / 0

    asyncio.run(func)
