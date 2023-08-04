"""Load and save configuration."""

import json
from pathlib import Path

import veilid

KEYFILE = Path(".demokeys")


def read_keys() -> dict:
    """Load the stored keys from disk."""

    try:
        raw = KEYFILE.read_text()
    except FileNotFoundError:
        return {
            "self": None,
            "peers": {},
        }

    keys = json.loads(raw)
    if keys["self"] is not None:
        keys["self"] = veilid.KeyPair(keys["self"])
    for name, pubkey in keys["peers"].items():
        keys["peers"][name] = veilid.PublicKey(pubkey)
    return keys


def write_keys(keydata: dict):
    """Save the keys to disk."""

    KEYFILE.write_text(json.dumps(keydata, indent=2))
