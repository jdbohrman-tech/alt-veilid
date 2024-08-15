import appdirs
import errno
import os
import sys
import re
from collections.abc import Callable
from functools import cache

import veilid
from veilid.json_api import _JsonVeilidAPI


ERRNO_PATTERN = re.compile(r"errno (\d+)", re.IGNORECASE)


class VeilidConnectionError(Exception):
    """The client could not connect to the veilid-server."""

    pass


@cache
def server_info(subindex: int = 0) -> tuple[str, int]:
    """Return the hostname and port of the server."""
    VEILID_SERVER_NETWORK = os.getenv("VEILID_SERVER_NETWORK")
    if VEILID_SERVER_NETWORK is None:
        return "localhost", 5959 + subindex

    hostname, *rest = VEILID_SERVER_NETWORK.split(":")
    if rest:
        return hostname, int(rest[0]) + subindex
    return hostname, 5959 + subindex


def ipc_path_exists(path: str) -> bool:
    """Determine if an IPC socket exists in a platform independent way."""
    if os.name == 'nt':
        if not path.upper().startswith("\\\\.\\PIPE\\"):
            return False
        return path[9:] in os.listdir("\\\\.\\PIPE")
    else:
        return os.path.exists(path)


@cache
def ipc_info(subindex: int = 0) -> str:
    """Return the path of the ipc socket of the server."""
    VEILID_SERVER_IPC = os.getenv("VEILID_SERVER_IPC")
    if VEILID_SERVER_IPC is not None:
        return VEILID_SERVER_IPC

    if os.name == 'nt':
        return f'\\\\.\\PIPE\\veilid-server\\{subindex}'

    ipc_path = f"/var/db/veilid-server/ipc/{subindex}"
    if os.path.exists(ipc_path):
        return ipc_path

    # hack to deal with rust's 'directories' crate case-inconsistency
    if sys.platform.startswith('darwin'):
        data_dir = appdirs.user_data_dir("org.Veilid.Veilid")
    else:
        data_dir = appdirs.user_data_dir("veilid", "veilid")
    ipc_path = os.path.join(data_dir, "ipc", str(subindex))
    return ipc_path


async def api_connector(callback: Callable, subindex: int = 0) -> _JsonVeilidAPI:
    """Return an API connection if possible.

    If the connection fails due to an inability to connect to the
    server's socket, raise an easy-to-catch VeilidConnectionError.
    """

    ipc_path = ipc_info(subindex)

    try:
        if ipc_path_exists(ipc_path):
            return await veilid.json_api_connect_ipc(ipc_path, callback)
        else:
            hostname, port = server_info(subindex)
            return await veilid.json_api_connect(hostname, port, callback)
    except OSError as exc:
        # This is a little goofy. The underlying Python library handles
        # connection errors in 2 ways, depending on how many connections
        # it attempted to make:
        #
        # - If it only tried to connect to one IP address socket, the
        # library propagates the one single OSError it got.
        #
        # - If it tried to connect to multiple sockets, perhaps because
        # the hostname resolved to several addresses (e.g. "localhost"
        # => 127.0.0.1 and ::1), then the library raises one exception
        # with all the failure exception strings joined together.

        # If errno is set, it's the first kind of exception. Check that
        # it's the code we expected.
        if exc.errno is not None:
            if exc.errno == errno.ECONNREFUSED:
                raise VeilidConnectionError
            raise

        # If not, use a regular expression to find all the errno values
        # in the combined error string. Check that all of them have the
        # code we're looking for.
        errnos = ERRNO_PATTERN.findall(str(exc))
        if all(int(err) == errno.ECONNREFUSED for err in errnos):
            raise VeilidConnectionError

        raise
