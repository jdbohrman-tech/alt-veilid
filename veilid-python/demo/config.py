"""Load and save configuration."""

import json
from pathlib import Path

KEYFILE = Path(".demokeys")


def read_keys() -> dict:
    """Load the stored keys from disk."""

    try:
        keydata = KEYFILE.read_text()
    except FileNotFoundError:
        return {
            "self": None,
            "peers": {},
        }

    return json.loads(keydata)


def write_keys(keydata: dict):
    """Save the keys to disk."""

    KEYFILE.write_text(json.dumps(keydata, indent=2))
