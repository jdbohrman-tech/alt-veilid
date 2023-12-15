import appdirs
import errno
import os
import socket
import sys
import re
from collections.abc import Callable
from functools import cache

from veilid.json_api import _JsonVeilidAPI

import veilid

ERRNO_PATTERN = re.compile(r"errno (\d+)", re.IGNORECASE)


class VeilidTestConnectionError(Exception):
    """The test client could not connect to the veilid-server."""

    pass


@cache
def server_info() -> tuple[str, int]:
    """Return the hostname and port of the test server."""
    VEILID_SERVER_NETWORK = os.getenv("VEILID_SERVER_NETWORK")
    if VEILID_SERVER_NETWORK is None:
        return "localhost", 5959

    hostname, *rest = VEILID_SERVER_NETWORK.split(":")
    if rest:
        return hostname, int(rest[0])
    return hostname, 5959

@cache
def ipc_info() -> str:
    """Return the path of the ipc socket of the test server."""
    VEILID_SERVER_IPC = os.getenv("VEILID_SERVER_IPC")
    if VEILID_SERVER_IPC is not None:
        return VEILID_SERVER_IPC

    if os.name == 'nt':
        return '\\\\.\\PIPE\\veilid-server\\ipc\\0'

    if os.name == 'posix':
        ipc_0_path = "/var/db/veilid-server/ipc/0"
        if os.path.exists(ipc_0_path):
            return ipc_0_path

    # hack to deal with rust's 'directories' crate case-inconsistency
    if sys.platform.startswith('darwin'):
        data_dir = appdirs.user_data_dir("Veilid","Veilid")
    else:
        data_dir = appdirs.user_data_dir("veilid","veilid")
    ipc_0_path = os.path.join(data_dir, "ipc", "0")
    return ipc_0_path


async def api_connector(callback: Callable) -> _JsonVeilidAPI:
    """Return an API connection if possible.

    If the connection fails due to an inability to connect to the
    server's socket, raise an easy-to-catch VeilidTestConnectionError.
    """

    ipc_path = ipc_info()    
    hostname, port = server_info()

    try:
        print(f"ipc_path: {ipc_path}")
        if os.path.exists(ipc_path):
            return await veilid.json_api_connect_ipc(ipc_path, callback)
        else:
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
                raise VeilidTestConnectionError
            raise

        # If not, use a regular expression to find all the errno values
        # in the combined error string. Check that all of them have the
        # code we're looking for.
        errnos = ERRNO_PATTERN.findall(str(exc))
        if all(int(err) == errno.ECONNREFUSED for err in errnos):
            raise VeilidTestConnectionError

        raise
