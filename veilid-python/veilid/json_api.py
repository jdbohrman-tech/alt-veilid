import asyncio
import importlib.resources as importlib_resources
import json
import os
import traceback

from typing import Awaitable, Callable, Optional, Self

from jsonschema import exceptions, validators

from . import schema
from .api import CryptoSystem, RoutingContext, TableDb, TableDbTransaction, VeilidAPI
from .error import raise_api_result
from .operations import (
    CryptoSystemOperation,
    Operation,
    RoutingContextOperation,
    TableDbOperation,
    TableDbTransactionOperation,
)
from .state import VeilidState, VeilidUpdate
from .types import (
    HashDistance,
    CryptoKind,
    DHTRecordDescriptor,
    DHTRecordReport,
    DHTReportScope,
    DHTSchema,
    HashDigest,
    KeyPair,
    NewPrivateRouteResult,
    Nonce,
    OperationId,
    PublicKey,
    RouteId,
    SafetySelection,
    SecretKey,
    Sequencing,
    SharedSecret,
    Signature,
    Stability,
    Timestamp,
    TypedKey,
    TypedKeyPair,
    TypedSignature,
    ValueData,
    ValueSubkey,
    VeilidJSONEncoder,
    VeilidVersion,
    urlsafe_b64decode_no_pad,
)

_STREAM_LIMIT = (65536 * 4)

##############################################################


def _get_schema_validator(schema):
    cls = validators.validator_for(schema)
    cls.check_schema(schema)
    validator = cls(schema)
    return validator


def _schema_validate(validator, instance):
    error = exceptions.best_match(validator.iter_errors(instance))
    if error is not None:
        raise error


_VALIDATOR_REQUEST = _get_schema_validator(
    json.loads((importlib_resources.files(schema) / "Request.json").read_text())
)
_VALIDATOR_RECV_MESSAGE = _get_schema_validator(
    json.loads((importlib_resources.files(schema) / "RecvMessage.json").read_text())
)


##############################################################


class _JsonVeilidAPI(VeilidAPI):
    reader: Optional[asyncio.StreamReader]
    writer: Optional[asyncio.StreamWriter]
    update_callback: Callable[[VeilidUpdate], Awaitable]
    handle_recv_messages_task: Optional[asyncio.Task]
    validate_schema: bool
    done: bool
    # Shared Mutable State
    lock: asyncio.Lock
    next_id: int
    in_flight_requests: dict[int, asyncio.Future]

    def __init__(
        self,
        reader: asyncio.StreamReader,
        writer: asyncio.StreamWriter,
        update_callback: Callable[[VeilidUpdate], Awaitable],
        validate_schema: bool = True,
    ):
        super().__init__()

        self.reader = reader
        self.writer = writer
        self.update_callback = update_callback
        self.validate_schema = validate_schema
        self.done = False
        self.handle_recv_messages_task = None
        self.lock = asyncio.Lock()
        self.next_id = 1
        self.in_flight_requests = dict()

    async def _cleanup_close(self):
        await self.lock.acquire()
        try:
            self.reader = None
            assert self.writer is not None
            try:
                self.writer.close()
                await self.writer.wait_closed()
            except:
                # Already closed
                pass
            self.writer = None

            for reqid, reqfuture in self.in_flight_requests.items():
                reqfuture.cancel()

        finally:
            self.lock.release()

    def is_done(self) -> bool:
        return self.done

    async def release(self):
        # Take the task
        await self.lock.acquire()
        try:
            if self.handle_recv_messages_task is None:
                return
            handle_recv_messages_task = self.handle_recv_messages_task
            self.handle_recv_messages_task = None
        finally:
            self.lock.release()
        # Cancel it
        handle_recv_messages_task.cancel()
        await handle_recv_messages_task

        self.done = True

    @classmethod
    async def connect(
        cls, host: str, port: int, update_callback: Callable[[VeilidUpdate], Awaitable]
    ) -> Self:
        reader, writer = await asyncio.open_connection(host, port, limit=_STREAM_LIMIT)
        veilid_api = cls(reader, writer, update_callback)
        veilid_api.handle_recv_messages_task = asyncio.create_task(
            veilid_api.handle_recv_messages(), name="JsonVeilidAPI.handle_recv_messages"
        )
        return veilid_api

    @classmethod
    async def connect_ipc(
        cls, ipc_path: str, update_callback: Callable[[VeilidUpdate], Awaitable]
    ) -> Self:

        if os.name=='nt': 
            async def open_windows_pipe(path=None, *,
                                   limit=65536, **kwds):
                """Similar to `open_unix_connection` but works with Windows Named Pipes."""
                loop = asyncio.events.get_running_loop()

                reader = asyncio.StreamReader(limit=limit, loop=loop)
                protocol = asyncio.StreamReaderProtocol(reader, loop=loop)
                transport, _ = await loop.create_pipe_connection(
                    lambda: protocol, path, **kwds)
                writer = asyncio.StreamWriter(transport, protocol, reader, loop)
                return reader, writer
            reader, writer = await open_windows_pipe(ipc_path, limit=_STREAM_LIMIT)
        else:
            reader, writer = await asyncio.open_unix_connection(ipc_path, limit=_STREAM_LIMIT)

        veilid_api = cls(reader, writer, update_callback)
        veilid_api.handle_recv_messages_task = asyncio.create_task(
            veilid_api.handle_recv_messages(), name="JsonVeilidAPI.handle_recv_messages"
            ) 
        return veilid_api

    async def handle_recv_message_response(self, j: dict):
        id = j["id"]
        await self.lock.acquire()
        try:
            # Get and remove the in-flight request
            reqfuture = self.in_flight_requests.pop(id, None)
        finally:
            self.lock.release()
        # Resolve the request's future to the response json
        if reqfuture is not None:
            reqfuture.set_result(j)
        else:
            print(f"Missing id: {id}, you may be missing a '.release()' or 'async with'")

    async def handle_recv_messages(self):
        # Read lines until we're done
        try:
            assert self.reader is not None
            while True:
                linebytes = await self.reader.readline()
                if not linebytes.endswith(b"\n"):
                    break

                # Parse line as ndjson
                j = json.loads(linebytes.strip())

                # print(f"j: {j}")

                if self.validate_schema:
                    _schema_validate(_VALIDATOR_RECV_MESSAGE, j)
                # Process the message
                if j["type"] == "Response":
                    await self.handle_recv_message_response(j)
                elif j["type"] == "Update":
                    await self.update_callback(VeilidUpdate.from_json(j))
        except ValueError:
            pass
        except asyncio.CancelledError:
            pass
        finally:
            await self._cleanup_close()

    async def allocate_request_future(self, id: int) -> asyncio.Future:
        reqfuture = asyncio.get_running_loop().create_future()

        await self.lock.acquire()
        try:
            self.in_flight_requests[id] = reqfuture
        finally:
            self.lock.release()

        return reqfuture

    async def cancel_request_future(self, id: int):
        await self.lock.acquire()
        try:
            reqfuture = self.in_flight_requests.pop(id, None)
            if reqfuture is not None:
                reqfuture.cancel()
        finally:
            self.lock.release()

    def send_one_way_ndjson_request(self, op: Operation, **kwargs):
        if self.writer is None:
            return

        # Make NDJSON string for request
        # Always use id 0 because no reply will be received for one-way requests
        req = {"id": 0, "op": op}
        for k, v in kwargs.items():
            req[k] = v
        reqstr = VeilidJSONEncoder.dumps(req) + "\n"
        reqbytes = reqstr.encode()

        if self.validate_schema:
            _schema_validate(_VALIDATOR_REQUEST, json.loads(reqbytes))

        # Send to socket without waitings
        self.writer.write(reqbytes)

    async def send_ndjson_request(
        self, op: Operation, validate: Optional[Callable[[dict, dict], None]] = None, **kwargs
    ) -> dict:
        # Get next id
        await self.lock.acquire()
        try:
            id = self.next_id
            self.next_id += 1
            writer = self.writer

            if self.writer is None:
                raise AssertionError("Don't send requests on a closed VeilidAPI object")
        finally:
            self.lock.release()

        # Make NDJSON string for request
        req = {"id": id, "op": op}
        for k, v in kwargs.items():
            req[k] = v
        reqstr = VeilidJSONEncoder.dumps(req) + "\n"
        reqbytes = reqstr.encode()

        if self.validate_schema:
            _schema_validate(_VALIDATOR_REQUEST, json.loads(reqbytes))

        # Allocate future for request
        reqfuture = await self.allocate_request_future(id)

        # Send to socket
        try:
            assert writer is not None
            writer.write(reqbytes)
            await writer.drain()
        except Exception:
            # Send failed, release future
            await self.cancel_request_future(id)
            raise

        # Wait for response
        response = await reqfuture

        # Validate if we have a validator
        if response["op"] != req["op"]:
            raise ValueError(f"Response op does not match request op: {response['op']} != {req['op']}")
        if validate is not None:
            validate(req, response)

        return response

    async def control(self, args: list[str]) -> str:
        return raise_api_result(await self.send_ndjson_request(Operation.CONTROL, args=args))

    async def get_state(self) -> VeilidState:
        return VeilidState.from_json(
            raise_api_result(await self.send_ndjson_request(Operation.GET_STATE))
        )
    
    async def is_shutdown(self) -> bool:
        return raise_api_result(await self.send_ndjson_request(Operation.IS_SHUTDOWN))

    async def attach(self):
        raise_api_result(await self.send_ndjson_request(Operation.ATTACH))

    async def detach(self):
        raise_api_result(await self.send_ndjson_request(Operation.DETACH))

    async def new_private_route(self) -> tuple[RouteId, bytes]:
        return NewPrivateRouteResult.from_json(
            raise_api_result(await self.send_ndjson_request(Operation.NEW_PRIVATE_ROUTE))
        ).to_tuple()

    async def new_custom_private_route(
        self, kinds: list[CryptoKind], stability: Stability, sequencing: Sequencing
    ) -> tuple[RouteId, bytes]:
        assert isinstance(kinds, list)
        for k in kinds:
            assert isinstance(k, CryptoKind)
        assert isinstance(stability, Stability)
        assert isinstance(sequencing, Sequencing)

        return NewPrivateRouteResult.from_json(
            raise_api_result(
                await self.send_ndjson_request(
                    Operation.NEW_CUSTOM_PRIVATE_ROUTE,
                    kinds=kinds,
                    stability=stability,
                    sequencing=sequencing,
                )
            )
        ).to_tuple()

    async def import_remote_private_route(self, blob: bytes) -> RouteId:
        assert isinstance(blob, bytes)

        return RouteId(
            raise_api_result(
                await self.send_ndjson_request(Operation.IMPORT_REMOTE_PRIVATE_ROUTE, blob=blob)
            )
        )

    async def release_private_route(self, route_id: RouteId):
        assert isinstance(route_id, RouteId)

        raise_api_result(
            await self.send_ndjson_request(Operation.RELEASE_PRIVATE_ROUTE, route_id=route_id)
        )

    async def app_call_reply(self, call_id: OperationId, message: bytes):
        assert isinstance(call_id, OperationId)
        assert isinstance(message, bytes)

        raise_api_result(
            await self.send_ndjson_request(
                Operation.APP_CALL_REPLY, call_id=call_id, message=message
            )
        )

    async def new_routing_context(self) -> RoutingContext:
        rc_id = raise_api_result(await self.send_ndjson_request(Operation.NEW_ROUTING_CONTEXT))
        return _JsonRoutingContext(self, rc_id)

    async def open_table_db(self, name: str, column_count: int) -> TableDb:
        assert isinstance(name, str)
        assert isinstance(column_count, int)

        db_id = raise_api_result(
            await self.send_ndjson_request(
                Operation.OPEN_TABLE_DB, name=name, column_count=column_count
            )
        )
        return _JsonTableDb(self, db_id)

    async def delete_table_db(self, name: str) -> bool:
        assert isinstance(name, str)

        return raise_api_result(
            await self.send_ndjson_request(Operation.DELETE_TABLE_DB, name=name)
        )

    async def get_crypto_system(self, kind: CryptoKind) -> CryptoSystem:
        assert isinstance(kind, CryptoKind)

        cs_id = raise_api_result(
            await self.send_ndjson_request(Operation.GET_CRYPTO_SYSTEM, kind=kind)
        )
        return _JsonCryptoSystem(self, cs_id)

    async def best_crypto_system(self) -> CryptoSystem:
        cs_id = raise_api_result(await self.send_ndjson_request(Operation.BEST_CRYPTO_SYSTEM))
        return _JsonCryptoSystem(self, cs_id)

    async def verify_signatures(
        self, node_ids: list[TypedKey], data: bytes, signatures: list[TypedSignature]
    ) -> Optional[list[TypedKey]]:
        assert isinstance(node_ids, list)
        for ni in node_ids:
            assert isinstance(ni, TypedKey)
        assert isinstance(data, bytes)
        for sig in signatures:
            assert isinstance(sig, TypedSignature)

        out = raise_api_result(
                await self.send_ndjson_request(
                    Operation.VERIFY_SIGNATURES,
                    node_ids=node_ids,
                    data=data,
                    signatures=signatures,
                )
            )
        if out is None:
            return out
        return list(
            map(
                lambda x: TypedKey(x),
                out
            )
        )

    async def generate_signatures(
        self, data: bytes, key_pairs: list[TypedKeyPair]
    ) -> list[TypedSignature]:
        assert isinstance(data, bytes)
        assert isinstance(key_pairs, list)
        for kp in key_pairs:
            assert isinstance(kp, TypedKeyPair)

        return list(
            map(
                lambda x: TypedSignature(x),
                raise_api_result(
                    await self.send_ndjson_request(
                        Operation.GENERATE_SIGNATURES, data=data, key_pairs=key_pairs
                    )
                ),
            )
        )

    async def generate_key_pair(self, kind: CryptoKind) -> list[TypedKeyPair]:
        assert isinstance(kind, CryptoKind)

        return list(
            map(
                lambda x: TypedKeyPair(x),
                raise_api_result(
                    await self.send_ndjson_request(Operation.GENERATE_KEY_PAIR, kind=kind)
                ),
            )
        )

    async def now(self) -> Timestamp:
        return Timestamp(raise_api_result(await self.send_ndjson_request(Operation.NOW)))

    async def debug(self, command: str) -> str:
        assert isinstance(command, str)
        return raise_api_result(await self.send_ndjson_request(Operation.DEBUG, command=command))

    async def veilid_version_string(self) -> str:
        return raise_api_result(await self.send_ndjson_request(Operation.VEILID_VERSION_STRING))

    async def veilid_version(self) -> VeilidVersion:
        v = await self.send_ndjson_request(Operation.VEILID_VERSION)
        return VeilidVersion(v["major"], v["minor"], v["patch"])

    async def veilid_features(self) -> list[str]:
        return raise_api_result(await self.send_ndjson_request(Operation.VEILID_FEATURES))

    async def default_veilid_config(self) -> str:
        return raise_api_result(await self.send_ndjson_request(Operation.DEFAULT_VEILID_CONFIG))


######################################################


def validate_rc_op(request: dict, response: dict):
    if response["rc_op"] != request["rc_op"]:
        raise ValueError(f"Response rc_op does not match request rc_op: {response["rc_op"]} != {request["rc_op"]}")


class _JsonRoutingContext(RoutingContext):
    api: _JsonVeilidAPI
    rc_id: int
    done: bool

    def __init__(self, api: _JsonVeilidAPI, rc_id: int):
        super().__init__()

        self.api = api
        self.rc_id = rc_id
        self.done = False

    def __del__(self):
        if not self.done:
            # attempt to clean up server-side anyway
            self.api.send_one_way_ndjson_request(
                Operation.ROUTING_CONTEXT, rc_id=self.rc_id, rc_op=RoutingContextOperation.RELEASE
            )

            # complain
            raise AssertionError("Should have released routing context before dropping object")

    def is_done(self) -> bool:
        return self.done

    async def release(self):
        if self.done:
            return
        await self.api.send_ndjson_request(
            Operation.ROUTING_CONTEXT,
            validate=validate_rc_op,
            rc_id=self.rc_id,
            rc_op=RoutingContextOperation.RELEASE,
        )
        self.done = True

    async def with_default_safety(self, release=True) -> Self:
        assert isinstance(release, bool)

        new_rc_id = raise_api_result(
            await self.api.send_ndjson_request(
                Operation.ROUTING_CONTEXT,
                validate=validate_rc_op,
                rc_id=self.rc_id,
                rc_op=RoutingContextOperation.WITH_DEFAULT_SAFETY,
            )
        )
        if release:
            await self.release()
        return self.__class__(self.api, new_rc_id)

    async def with_safety(self, safety_selection: SafetySelection, release=True) -> Self:
        assert isinstance(safety_selection, SafetySelection)
        assert isinstance(release, bool)

        new_rc_id = raise_api_result(
            await self.api.send_ndjson_request(
                Operation.ROUTING_CONTEXT,
                validate=validate_rc_op,
                rc_id=self.rc_id,
                rc_op=RoutingContextOperation.WITH_SAFETY,
                safety_selection=safety_selection,
            )
        )
        if release:
            await self.release()
        return self.__class__(self.api, new_rc_id)

    async def with_sequencing(self, sequencing: Sequencing, release=True) -> Self:
        assert isinstance(sequencing, Sequencing)
        assert isinstance(release, bool)

        new_rc_id = raise_api_result(
            await self.api.send_ndjson_request(
                Operation.ROUTING_CONTEXT,
                validate=validate_rc_op,
                rc_id=self.rc_id,
                rc_op=RoutingContextOperation.WITH_SEQUENCING,
                sequencing=sequencing,
            )
        )
        if release:
            await self.release()
        return self.__class__(self.api, new_rc_id)

    async def safety(
        self
    ) -> SafetySelection:
        return SafetySelection.from_json(
            raise_api_result(
                await self.api.send_ndjson_request(
                    Operation.ROUTING_CONTEXT,
                    validate=validate_rc_op,
                    rc_id=self.rc_id,
                    rc_op=RoutingContextOperation.SAFETY,
                )
            )
        )
    async def app_call(self, target: TypedKey | RouteId, message: bytes) -> bytes:
        assert isinstance(target, TypedKey) or isinstance(target, RouteId)
        assert isinstance(message, bytes)

        return urlsafe_b64decode_no_pad(
            raise_api_result(
                await self.api.send_ndjson_request(
                    Operation.ROUTING_CONTEXT,
                    validate=validate_rc_op,
                    rc_id=self.rc_id,
                    rc_op=RoutingContextOperation.APP_CALL,
                    target=target,
                    message=message,
                )
            )
        )

    async def app_message(self, target: TypedKey | RouteId, message: bytes):
        assert isinstance(target, TypedKey) or isinstance(target, RouteId)
        assert isinstance(message, bytes)

        raise_api_result(
            await self.api.send_ndjson_request(
                Operation.ROUTING_CONTEXT,
                validate=validate_rc_op,
                rc_id=self.rc_id,
                rc_op=RoutingContextOperation.APP_MESSAGE,
                target=target,
                message=message,
            )
        )

    async def create_dht_record(
        self, schema: DHTSchema, owner: Optional[KeyPair] = None, kind: Optional[CryptoKind] = None
    ) -> DHTRecordDescriptor:
        assert isinstance(schema, DHTSchema)
        assert owner is None or isinstance(owner, KeyPair)
        assert kind is None or isinstance(kind, CryptoKind)

        return DHTRecordDescriptor.from_json(
            raise_api_result(
                await self.api.send_ndjson_request(
                    Operation.ROUTING_CONTEXT,
                    validate=validate_rc_op,
                    rc_id=self.rc_id,
                    rc_op=RoutingContextOperation.CREATE_DHT_RECORD,
                    kind=kind,
                    owner=owner,
                    schema=schema,
                )
            )
        )

    async def open_dht_record(
        self, key: TypedKey, writer: Optional[KeyPair] = None
    ) -> DHTRecordDescriptor:
        assert isinstance(key, TypedKey)
        assert writer is None or isinstance(writer, KeyPair)

        return DHTRecordDescriptor.from_json(
            raise_api_result(
                await self.api.send_ndjson_request(
                    Operation.ROUTING_CONTEXT,
                    validate=validate_rc_op,
                    rc_id=self.rc_id,
                    rc_op=RoutingContextOperation.OPEN_DHT_RECORD,
                    key=key,
                    writer=writer,
                )
            )
        )

    async def close_dht_record(self, key: TypedKey):
        assert isinstance(key, TypedKey)

        raise_api_result(
            await self.api.send_ndjson_request(
                Operation.ROUTING_CONTEXT,
                validate=validate_rc_op,
                rc_id=self.rc_id,
                rc_op=RoutingContextOperation.CLOSE_DHT_RECORD,
                key=key,
            )
        )

    async def delete_dht_record(self, key: TypedKey):
        assert isinstance(key, TypedKey)

        raise_api_result(
            await self.api.send_ndjson_request(
                Operation.ROUTING_CONTEXT,
                validate=validate_rc_op,
                rc_id=self.rc_id,
                rc_op=RoutingContextOperation.DELETE_DHT_RECORD,
                key=key,
            )
        )

    async def get_dht_value(
        self, key: TypedKey, subkey: ValueSubkey, force_refresh: bool = False
    ) -> Optional[ValueData]:
        assert isinstance(key, TypedKey)
        assert isinstance(subkey, ValueSubkey)
        assert isinstance(force_refresh, bool)

        ret = raise_api_result(
            await self.api.send_ndjson_request(
                Operation.ROUTING_CONTEXT,
                validate=validate_rc_op,
                rc_id=self.rc_id,
                rc_op=RoutingContextOperation.GET_DHT_VALUE,
                key=key,
                subkey=subkey,
                force_refresh=force_refresh,
            )
        )
        return None if ret is None else ValueData.from_json(ret)

    async def set_dht_value(
        self, key: TypedKey, subkey: ValueSubkey, data: bytes, writer: Optional[KeyPair] = None
    ) -> Optional[ValueData]:
        assert isinstance(key, TypedKey)
        assert isinstance(subkey, ValueSubkey)
        assert isinstance(data, bytes)
        assert writer is None or isinstance(writer, KeyPair)

        ret = raise_api_result(
            await self.api.send_ndjson_request(
                Operation.ROUTING_CONTEXT,
                validate=validate_rc_op,
                rc_id=self.rc_id,
                rc_op=RoutingContextOperation.SET_DHT_VALUE,
                key=key,
                subkey=subkey,
                data=data,
                writer=writer,
            )
        )
        return None if ret is None else ValueData.from_json(ret)

    async def watch_dht_values(
        self,
        key: TypedKey,
        subkeys: list[tuple[ValueSubkey, ValueSubkey]] = [],
        expiration: Timestamp = Timestamp(0),
        count: int = 0xFFFFFFFF,
    ) -> bool:
        assert isinstance(key, TypedKey)
        assert isinstance(subkeys, list)
        for s in subkeys:
            assert isinstance(s, tuple)
            assert isinstance(s[0], ValueSubkey)
            assert isinstance(s[1], ValueSubkey)
        assert isinstance(expiration, Timestamp)
        assert isinstance(count, int)

        return raise_api_result(
            await self.api.send_ndjson_request(
                Operation.ROUTING_CONTEXT,
                validate=validate_rc_op,
                rc_id=self.rc_id,
                rc_op=RoutingContextOperation.WATCH_DHT_VALUES,
                key=key,
                subkeys=subkeys,
                expiration=str(expiration),
                count=count,
            )
        )


    async def cancel_dht_watch(
        self, key: TypedKey, subkeys: list[tuple[ValueSubkey, ValueSubkey]] = []
    ) -> bool:
        assert isinstance(key, TypedKey)
        assert isinstance(subkeys, list)
        for s in subkeys:
            assert isinstance(s, tuple)
            assert isinstance(s[0], ValueSubkey)
            assert isinstance(s[1], ValueSubkey)

        return raise_api_result(
            await self.api.send_ndjson_request(
                Operation.ROUTING_CONTEXT,
                validate=validate_rc_op,
                rc_id=self.rc_id,
                rc_op=RoutingContextOperation.CANCEL_DHT_WATCH,
                key=key,
                subkeys=subkeys,
            )
        )

    async def inspect_dht_record(
        self,
        key: TypedKey,
        subkeys: list[tuple[ValueSubkey, ValueSubkey]],
        scope: DHTReportScope = DHTReportScope.LOCAL,
    ) -> DHTRecordReport:
        assert isinstance(key, TypedKey)
        assert isinstance(subkeys, list)
        for s in subkeys:
            assert isinstance(s, tuple)
            assert isinstance(s[0], ValueSubkey)
            assert isinstance(s[1], ValueSubkey)
        assert isinstance(scope, DHTReportScope)

        return DHTRecordReport.from_json(            
            raise_api_result(
                await self.api.send_ndjson_request(
                    Operation.ROUTING_CONTEXT,
                    validate=validate_rc_op,
                    rc_id=self.rc_id,
                    rc_op=RoutingContextOperation.INSPECT_DHT_RECORD,
                    key=key,
                    subkeys=subkeys,
                    scope=scope,
                )
            )
        )
        



######################################################


def validate_tx_op(request: dict, response: dict):
    if response["tx_op"] != request["tx_op"]:
        raise ValueError(f"Response tx_op does not match request tx_op: {response['tx_op']} != {request['tx_op']}")


class _JsonTableDbTransaction(TableDbTransaction):
    api: _JsonVeilidAPI
    tx_id: int
    done: bool

    def __init__(self, api: _JsonVeilidAPI, tx_id: int):
        super().__init__()

        self.api = api
        self.tx_id = tx_id
        self.done = False

    def __del__(self):
        if not self.done:
            # attempt to clean up server-side anyway
            self.api.send_one_way_ndjson_request(
                Operation.TABLE_DB_TRANSACTION,
                tx_id=self.tx_id,
                tx_op=TableDbTransactionOperation.ROLLBACK,
            )

            # complain
            raise AssertionError(
                "Should have committed or rolled back transaction before dropping object"
            )

    def is_done(self) -> bool:
        return self.done

    async def commit(self):
        if self.done:
            raise AssertionError("Transaction is already done")

        raise_api_result(
            await self.api.send_ndjson_request(
                Operation.TABLE_DB_TRANSACTION,
                validate=validate_tx_op,
                tx_id=self.tx_id,
                tx_op=TableDbTransactionOperation.COMMIT,
            )
        )
        self.done = True

    async def rollback(self):
        if self.done:
            raise AssertionError("Transaction is already done")
        await self.api.send_ndjson_request(
            Operation.TABLE_DB_TRANSACTION,
            validate=validate_tx_op,
            tx_id=self.tx_id,
            tx_op=TableDbTransactionOperation.ROLLBACK,
        )
        self.done = True

    async def store(self, key: bytes, value: bytes, col: int = 0):
        assert isinstance(key, bytes)
        assert isinstance(value, bytes)
        assert isinstance(col, int)

        await self.api.send_ndjson_request(
            Operation.TABLE_DB_TRANSACTION,
            validate=validate_tx_op,
            tx_id=self.tx_id,
            tx_op=TableDbTransactionOperation.STORE,
            col=col,
            key=key,
            value=value,
        )

    async def delete(self, key: bytes, col: int = 0):
        assert isinstance(key, bytes)
        assert isinstance(col, int)

        await self.api.send_ndjson_request(
            Operation.TABLE_DB_TRANSACTION,
            validate=validate_tx_op,
            tx_id=self.tx_id,
            tx_op=TableDbTransactionOperation.DELETE,
            col=col,
            key=key,
        )


######################################################


def validate_db_op(request: dict, response: dict):
    if response["db_op"] != request["db_op"]:
        raise ValueError(f"Response db_op does not match request db_op: {response['db_op']} != {request['db_op']}")


class _JsonTableDb(TableDb):
    api: _JsonVeilidAPI
    db_id: int
    done: bool

    def __init__(self, api: _JsonVeilidAPI, db_id: int):
        super().__init__()

        self.api = api
        self.db_id = db_id
        self.done = False

    def __del__(self):
        if not self.done:
            # attempt to clean up server-side anyway
            self.api.send_one_way_ndjson_request(
                Operation.TABLE_DB, db_id=self.db_id, db_op=TableDbOperation.RELEASE
            )

            # complain
            raise AssertionError("Should have released table db before dropping object")

    def is_done(self) -> bool:
        return self.done

    async def release(self):
        if self.done:
            return
        await self.api.send_ndjson_request(
            Operation.TABLE_DB,
            validate=validate_db_op,
            db_id=self.db_id,
            db_op=TableDbOperation.RELEASE,
        )
        self.done = True

    async def get_column_count(self) -> int:
        return raise_api_result(
            await self.api.send_ndjson_request(
                Operation.TABLE_DB,
                validate=validate_db_op,
                db_id=self.db_id,
                db_op=TableDbOperation.GET_COLUMN_COUNT,
            )
        )

    async def get_keys(self, col: int = 0) -> list[bytes]:
        assert isinstance(col, int)

        return list(
            map(
                lambda x: urlsafe_b64decode_no_pad(x),
                raise_api_result(
                    await self.api.send_ndjson_request(
                        Operation.TABLE_DB,
                        validate=validate_db_op,
                        db_id=self.db_id,
                        db_op=TableDbOperation.GET_KEYS,
                        col=col,
                    )
                ),
            )
        )

    async def transact(self) -> TableDbTransaction:
        tx_id = raise_api_result(
            await self.api.send_ndjson_request(
                Operation.TABLE_DB,
                validate=validate_db_op,
                db_id=self.db_id,
                db_op=TableDbOperation.TRANSACT,
            )
        )
        return _JsonTableDbTransaction(self.api, tx_id)

    async def store(self, key: bytes, value: bytes, col: int = 0):
        assert isinstance(key, bytes)
        assert isinstance(value, bytes)
        assert isinstance(col, int)

        return raise_api_result(
            await self.api.send_ndjson_request(
                Operation.TABLE_DB,
                validate=validate_db_op,
                db_id=self.db_id,
                db_op=TableDbOperation.STORE,
                col=col,
                key=key,
                value=value,
            )
        )

    async def load(self, key: bytes, col: int = 0) -> Optional[bytes]:
        assert isinstance(key, bytes)
        assert isinstance(col, int)

        res = raise_api_result(
            await self.api.send_ndjson_request(
                Operation.TABLE_DB,
                validate=validate_db_op,
                db_id=self.db_id,
                db_op=TableDbOperation.LOAD,
                col=col,
                key=key,
            )
        )
        return None if res is None else urlsafe_b64decode_no_pad(res)

    async def delete(self, key: bytes, col: int = 0) -> Optional[bytes]:
        assert isinstance(key, bytes)
        assert isinstance(col, int)

        res = raise_api_result(
            await self.api.send_ndjson_request(
                Operation.TABLE_DB,
                validate=validate_db_op,
                db_id=self.db_id,
                db_op=TableDbOperation.DELETE,
                col=col,
                key=key,
            )
        )
        return None if res is None else urlsafe_b64decode_no_pad(res)


######################################################


def validate_cs_op(request: dict, response: dict):
    if response["cs_op"] != request["cs_op"]:
        raise ValueError(f"Response cs_op does not match request cs_op: {response['cs_op']} != {request['cs_op']}")


class _JsonCryptoSystem(CryptoSystem):
    api: _JsonVeilidAPI
    cs_id: int
    done: bool

    def __init__(self, api: _JsonVeilidAPI, cs_id: int):
        super().__init__()

        self.api = api
        self.cs_id = cs_id
        self.done = False

    def __del__(self):
        if not self.done:
            # attempt to clean up server-side anyway
            self.api.send_one_way_ndjson_request(
                Operation.CRYPTO_SYSTEM, cs_id=self.cs_id, cs_op=CryptoSystemOperation.RELEASE
            )

            # complain
            raise AssertionError("Should have released crypto system before dropping object")
    
    async def kind(self) -> CryptoKind:
        return CryptoKind(
            raise_api_result(
                await self.api.send_ndjson_request(
                    Operation.CRYPTO_SYSTEM,
                    validate=validate_cs_op,
                    cs_id=self.cs_id,
                    cs_op=CryptoSystemOperation.KIND,
                )
            )
        )

    def is_done(self) -> bool:
        return self.done

    async def release(self):
        if self.done:
            return
        await self.api.send_ndjson_request(
            Operation.CRYPTO_SYSTEM,
            validate=validate_cs_op,
            cs_id=self.cs_id,
            cs_op=CryptoSystemOperation.RELEASE,
        )
        self.done = True

    async def cached_dh(self, key: PublicKey, secret: SecretKey) -> SharedSecret:
        assert isinstance(key, PublicKey)
        assert isinstance(secret, SecretKey)

        return SharedSecret(
            raise_api_result(
                await self.api.send_ndjson_request(
                    Operation.CRYPTO_SYSTEM,
                    validate=validate_cs_op,
                    cs_id=self.cs_id,
                    cs_op=CryptoSystemOperation.CACHED_DH,
                    key=key,
                    secret=secret,
                )
            )
        )

    async def compute_dh(self, key: PublicKey, secret: SecretKey) -> SharedSecret:
        assert isinstance(key, PublicKey)
        assert isinstance(secret, SecretKey)

        return SharedSecret(
            raise_api_result(
                await self.api.send_ndjson_request(
                    Operation.CRYPTO_SYSTEM,
                    validate=validate_cs_op,
                    cs_id=self.cs_id,
                    cs_op=CryptoSystemOperation.COMPUTE_DH,
                    key=key,
                    secret=secret,
                )
            )
        )

    async def generate_shared_secret(self, key: PublicKey, secret: SecretKey, domain: bytes) -> SharedSecret:
        assert isinstance(key, PublicKey)
        assert isinstance(secret, SecretKey)
        assert isinstance(domain, bytes)

        return SharedSecret(
            raise_api_result(
                await self.api.send_ndjson_request(
                    Operation.CRYPTO_SYSTEM,
                    validate=validate_cs_op,
                    cs_id=self.cs_id,
                    cs_op=CryptoSystemOperation.GENERATE_SHARED_SECRET,
                    key=key,
                    secret=secret,
                    domain=domain,
                )
            )
        )

    async def random_bytes(self, len: int) -> bytes:
        assert isinstance(len, int)

        return urlsafe_b64decode_no_pad(
            raise_api_result(
                await self.api.send_ndjson_request(
                    Operation.CRYPTO_SYSTEM,
                    validate=validate_cs_op,
                    cs_id=self.cs_id,
                    cs_op=CryptoSystemOperation.RANDOM_BYTES,
                    len=len,
                )
            )
        )

    async def default_salt_length(self) -> int:
        return raise_api_result(
            await self.api.send_ndjson_request(
                Operation.CRYPTO_SYSTEM,
                validate=validate_cs_op,
                cs_id=self.cs_id,
                cs_op=CryptoSystemOperation.DEFAULT_SALT_LENGTH,
            )
        )

    async def hash_password(self, password: bytes, salt: bytes) -> str:
        assert isinstance(password, bytes)
        assert isinstance(salt, bytes)

        return raise_api_result(
            await self.api.send_ndjson_request(
                Operation.CRYPTO_SYSTEM,
                validate=validate_cs_op,
                cs_id=self.cs_id,
                cs_op=CryptoSystemOperation.HASH_PASSWORD,
                password=password,
                salt=salt,
            )
        )

    async def verify_password(self, password: bytes, password_hash: str) -> bool:
        assert isinstance(password, bytes)
        assert isinstance(password_hash, str)

        return raise_api_result(
            await self.api.send_ndjson_request(
                Operation.CRYPTO_SYSTEM,
                validate=validate_cs_op,
                cs_id=self.cs_id,
                cs_op=CryptoSystemOperation.VERIFY_PASSWORD,
                password=password,
                password_hash=password_hash,
            )
        )

    async def derive_shared_secret(self, password: bytes, salt: bytes) -> SharedSecret:
        assert isinstance(password, bytes)
        assert isinstance(salt, bytes)

        return SharedSecret(
            raise_api_result(
                await self.api.send_ndjson_request(
                    Operation.CRYPTO_SYSTEM,
                    validate=validate_cs_op,
                    cs_id=self.cs_id,
                    cs_op=CryptoSystemOperation.DERIVE_SHARED_SECRET,
                    password=password,
                    salt=salt,
                )
            )
        )

    async def random_nonce(self) -> Nonce:
        return Nonce(
            raise_api_result(
                await self.api.send_ndjson_request(
                    Operation.CRYPTO_SYSTEM,
                    validate=validate_cs_op,
                    cs_id=self.cs_id,
                    cs_op=CryptoSystemOperation.RANDOM_NONCE,
                )
            )
        )

    async def random_shared_secret(self) -> SharedSecret:
        return SharedSecret(
            raise_api_result(
                await self.api.send_ndjson_request(
                    Operation.CRYPTO_SYSTEM,
                    validate=validate_cs_op,
                    cs_id=self.cs_id,
                    cs_op=CryptoSystemOperation.RANDOM_SHARED_SECRET,
                )
            )
        )

    async def generate_key_pair(self) -> KeyPair:
        return KeyPair(
            raise_api_result(
                await self.api.send_ndjson_request(
                    Operation.CRYPTO_SYSTEM,
                    validate=validate_cs_op,
                    cs_id=self.cs_id,
                    cs_op=CryptoSystemOperation.GENERATE_KEY_PAIR,
                )
            )
        )

    async def generate_hash(self, data: bytes) -> HashDigest:
        assert isinstance(data, bytes)

        return HashDigest(
            raise_api_result(
                await self.api.send_ndjson_request(
                    Operation.CRYPTO_SYSTEM,
                    validate=validate_cs_op,
                    cs_id=self.cs_id,
                    cs_op=CryptoSystemOperation.GENERATE_HASH,
                    data=data,
                )
            )
        )

    async def validate_key_pair(self, key: PublicKey, secret: SecretKey) -> bool:
        assert isinstance(key, PublicKey)
        assert isinstance(secret, SecretKey)

        return raise_api_result(
            await self.api.send_ndjson_request(
                Operation.CRYPTO_SYSTEM,
                validate=validate_cs_op,
                cs_id=self.cs_id,
                cs_op=CryptoSystemOperation.VALIDATE_KEY_PAIR,
                key=key,
                secret=secret,
            )
        )

    async def validate_hash(self, data: bytes, hash_digest: HashDigest) -> bool:
        assert isinstance(data, bytes)
        assert isinstance(hash_digest, HashDigest)

        return raise_api_result(
            await self.api.send_ndjson_request(
                Operation.CRYPTO_SYSTEM,
                validate=validate_cs_op,
                cs_id=self.cs_id,
                cs_op=CryptoSystemOperation.VALIDATE_HASH,
                data=data,
                hash_digest=hash_digest,
            )
        )

    async def distance(self, key1: HashDigest, key2: HashDigest) -> HashDistance:
        assert isinstance(key1, HashDigest)
        assert isinstance(key2, HashDigest)

        return HashDistance(
            raise_api_result(
                await self.api.send_ndjson_request(
                    Operation.CRYPTO_SYSTEM,
                    validate=validate_cs_op,
                    cs_id=self.cs_id,
                    cs_op=CryptoSystemOperation.DISTANCE,
                    key1=key1,
                    key2=key2,
                )
            )
        )

    async def sign(self, key: PublicKey, secret: SecretKey, data: bytes) -> Signature:
        assert isinstance(key, PublicKey)
        assert isinstance(secret, SecretKey)
        assert isinstance(data, bytes)

        return Signature(
            raise_api_result(
                await self.api.send_ndjson_request(
                    Operation.CRYPTO_SYSTEM,
                    validate=validate_cs_op,
                    cs_id=self.cs_id,
                    cs_op=CryptoSystemOperation.SIGN,
                    key=key,
                    secret=secret,
                    data=data,
                )
            )
        )

    async def verify(self, key: PublicKey, data: bytes, signature: Signature):
        assert isinstance(key, PublicKey)
        assert isinstance(data, bytes)
        assert isinstance(signature, Signature)

        return raise_api_result(
            await self.api.send_ndjson_request(
                Operation.CRYPTO_SYSTEM,
                validate=validate_cs_op,
                cs_id=self.cs_id,
                cs_op=CryptoSystemOperation.VERIFY,
                key=key,
                data=data,
                signature=signature,
            )
        )

    async def aead_overhead(self) -> int:
        return raise_api_result(
            await self.api.send_ndjson_request(
                Operation.CRYPTO_SYSTEM,
                validate=validate_cs_op,
                cs_id=self.cs_id,
                cs_op=CryptoSystemOperation.AEAD_OVERHEAD,
            )
        )

    async def decrypt_aead(
        self,
        body: bytes,
        nonce: Nonce,
        shared_secret: SharedSecret,
        associated_data: Optional[bytes],
    ) -> bytes:
        assert isinstance(body, bytes)
        assert isinstance(nonce, Nonce)
        assert isinstance(shared_secret, SharedSecret)
        assert associated_data is None or isinstance(associated_data, bytes)

        return urlsafe_b64decode_no_pad(
            raise_api_result(
                await self.api.send_ndjson_request(
                    Operation.CRYPTO_SYSTEM,
                    validate=validate_cs_op,
                    cs_id=self.cs_id,
                    cs_op=CryptoSystemOperation.DECRYPT_AEAD,
                    body=body,
                    nonce=nonce,
                    shared_secret=shared_secret,
                    associated_data=associated_data,
                )
            )
        )

    async def encrypt_aead(
        self,
        body: bytes,
        nonce: Nonce,
        shared_secret: SharedSecret,
        associated_data: Optional[bytes],
    ) -> bytes:
        assert isinstance(body, bytes)
        assert isinstance(nonce, Nonce)
        assert isinstance(shared_secret, SharedSecret)
        assert associated_data is None or isinstance(associated_data, bytes)

        return urlsafe_b64decode_no_pad(
            raise_api_result(
                await self.api.send_ndjson_request(
                    Operation.CRYPTO_SYSTEM,
                    validate=validate_cs_op,
                    cs_id=self.cs_id,
                    cs_op=CryptoSystemOperation.ENCRYPT_AEAD,
                    body=body,
                    nonce=nonce,
                    shared_secret=shared_secret,
                    associated_data=associated_data,
                )
            )
        )

    async def crypt_no_auth(self, body: bytes, nonce: Nonce, shared_secret: SharedSecret) -> bytes:
        assert isinstance(body, bytes)
        assert isinstance(nonce, Nonce)
        assert isinstance(shared_secret, SharedSecret)
        return urlsafe_b64decode_no_pad(
            raise_api_result(
                await self.api.send_ndjson_request(
                    Operation.CRYPTO_SYSTEM,
                    validate=validate_cs_op,
                    cs_id=self.cs_id,
                    cs_op=CryptoSystemOperation.CRYPT_NO_AUTH,
                    body=body,
                    nonce=nonce,
                    shared_secret=shared_secret,
                )
            )
        )


######################################################


async def json_api_connect(
    host: str, port: int, update_callback: Callable[[VeilidUpdate], Awaitable]
) -> _JsonVeilidAPI:
    return await _JsonVeilidAPI.connect(host, port, update_callback)

async def json_api_connect_ipc(
    ipc_path: str, update_callback: Callable[[VeilidUpdate], Awaitable]
) -> _JsonVeilidAPI:
    return await _JsonVeilidAPI.connect_ipc(ipc_path, update_callback)
