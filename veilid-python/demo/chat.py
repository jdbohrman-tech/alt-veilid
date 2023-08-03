#!/usr/bin/env python

"""A simple chat server using Veilid's DHT."""

import argparse
import asyncio
import sys

import config

import veilid

QUIT = b"QUIT"


async def noop_callback(*args, **kwargs):
    """In the real world, we'd use this to process interesting incoming events."""

    return


async def chatter(
    router: veilid.api.RoutingContext,
    crypto_system: veilid.CryptoSystem,
    key: veilid.TypedKey,
    send_subkey: veilid.ValueSubkey,
    recv_subkey: veilid.ValueSubkey,
):
    """Read input, write it to the DHT, and print the response from the DHT."""

    last_seq = -1

    # Prime the pumps. Especially when starting the conversation, this
    # causes the DHT key to propagate to the network.
    await router.set_dht_value(key, send_subkey, b"Hello from the world!")

    while True:
        try:
            msg = input("SEND> ")
        except EOFError:
            # Cat got your tongue? Hang up.
            print("Closing the chat.")
            await router.set_dht_value(key, send_subkey, QUIT)
            return

        # Write the input message to the DHT key.
        await router.set_dht_value(key, send_subkey, msg.encode())

        # In the real world, don't do this. People may tease you for it.
        # This is meant to be easy to understand for demonstration
        # purposes, not a great pattern. Instead, you'd want to use the
        # callback function to handle events asynchronously.
        while True:
            # Try to get an updated version of the receiving subkey.
            resp = await router.get_dht_value(key, recv_subkey, True)
            if resp is None:
                continue

            # If the other party hasn't sent a newer message, try again.
            if resp.seq == last_seq:
                continue

            if resp.data == QUIT:
                print("Other end closed the chat.")
                return

            print(f"RECV< {resp.data.decode()}")
            last_seq = resp.seq
            break


async def start(host: str, port: int, name: str):
    """Begin a conversation with a friend."""

    conn = await veilid.json_api_connect(host, port, noop_callback)

    keys = config.read_keys()
    my_keypair = keys["self"]
    their_key = keys["peers"][name]

    members = [
        veilid.DHTSchemaSMPLMember(my_keypair.key(), 1),
        veilid.DHTSchemaSMPLMember(their_key, 1),
    ]

    router = await (await conn.new_routing_context()).with_privacy()
    crypto_system = await conn.get_crypto_system(veilid.CryptoKind.CRYPTO_KIND_VLD0)
    async with router, crypto_system:
        record = await router.create_dht_record(veilid.DHTSchema.smpl(0, members))
        print(f"New chat key: {record.key}")
        print("Give that to your friend!")

        # Close this key first. We'll reopen it for writing with our saved key.
        await router.close_dht_record(record.key)

        await router.open_dht_record(record.key, my_keypair)

        try:
            # Write to the 1st subkey and read from the 2nd.
            await chatter(router, crypto_system, record.key, 0, 1)
        finally:
            await router.close_dht_record(record.key)
            await router.delete_dht_record(record.key)


async def respond(host: str, port: int, key: str):
    """Reply to a friend's chat."""

    conn = await veilid.json_api_connect(host, port, noop_callback)

    keys = config.read_keys()
    my_keypair = keys["self"]

    router = await (await conn.new_routing_context()).with_privacy()
    crypto_system = await conn.get_crypto_system(veilid.CryptoKind.CRYPTO_KIND_VLD0)
    async with router, crypto_system:
        await router.open_dht_record(key, my_keypair)

        # As the responder, we're writing to the 2nd subkey and reading from the 1st.
        await chatter(router, crypto_system, key, 1, 0)


async def keygen(host: str, port: int):
    """Generate a keypair."""

    conn = await veilid.json_api_connect(host, port, noop_callback)

    crypto_system = await conn.get_crypto_system(veilid.CryptoKind.CRYPTO_KIND_VLD0)
    async with crypto_system:
        my_keypair = await crypto_system.generate_key_pair()

    keys = config.read_keys()
    if keys["self"]:
        print("You already have a keypair.")
        sys.exit(1)

    keys["self"] = my_keypair
    config.write_keys(keys)

    print(f"Your new public key is {my_keypair.key()}. Share it with your friends!")


async def add_friend(host: str, port: int, name: str, pubkey: str):
    """Add a friend's public key."""

    keys = config.read_keys()
    keys["peers"][name] = pubkey
    config.write_keys(keys)


async def clean(host: str, port: int, key: str):
    """Delete a DHT key."""

    conn = await veilid.json_api_connect(host, port, noop_callback)

    router = await (await conn.new_routing_context()).with_privacy()
    async with router:
        await router.close_dht_record(key)
        await router.delete_dht_record(key)


def handle_command_line(arglist: list[str]):
    """Process the command line.

    This isn't the interesting part."""

    parser = argparse.ArgumentParser(description="Veilid chat demonstration")
    parser.add_argument("--host", default="localhost", help="Address of the Veilid server host.")
    parser.add_argument("--port", type=int, default=5959, help="Port of the Veilid server.")

    subparsers = parser.add_subparsers(required=True)

    cmd_start = subparsers.add_parser("start", help=start.__doc__)
    cmd_start.add_argument("name", help="Your friend's name")
    cmd_start.set_defaults(func=start)

    cmd_respond = subparsers.add_parser("respond", help=respond.__doc__)
    cmd_respond.add_argument("key", help="The chat's DHT key")
    cmd_respond.set_defaults(func=respond)

    cmd_keygen = subparsers.add_parser("keygen", help=keygen.__doc__)
    cmd_keygen.set_defaults(func=keygen)

    cmd_add_friend = subparsers.add_parser("add-friend", help=add_friend.__doc__)
    cmd_add_friend.add_argument("name", help="Your friend's name")
    cmd_add_friend.add_argument("pubkey", help="Your friend's public key")
    cmd_add_friend.set_defaults(func=add_friend)

    cmd_clean = subparsers.add_parser("clean", help=clean.__doc__)
    cmd_clean.add_argument("key", help="DHT key to delete")
    cmd_clean.set_defaults(func=clean)

    args = parser.parse_args(arglist)
    kwargs = args.__dict__
    func = kwargs.pop("func")

    asyncio.run(func(**kwargs))


if __name__ == "__main__":
    handle_command_line(sys.argv[1:])
