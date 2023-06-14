import json;
import asyncio;
from typing import Callable, Awaitable

from .api import *;
from .state import *
from .config import *
from .error import *
from .types import *
from .operations import *

class _JsonRoutingContext(RoutingContext):
    api: VeilidAPI    
    rc_id: int
    
    def __init__(self, api: VeilidAPI, rc_id: int):
        self.api = api
        self.rc_id = rc_id

    async def with_privacy(self) -> Self:
        new_rc_id = raise_api_result(await self.send_ndjson_request(Operation.ROUTING_CONTEXT, 
            rc_id = self.rc_id, 
            rc_op = RoutingContextOperation.WITH_PRIVACY))
        return _JsonRoutingContext(self.api, new_rc_id)
        
    async def with_custom_privacy(self, stability: Stability) -> Self:
        new_rc_id = raise_api_result(await self.send_ndjson_request(Operation.ROUTING_CONTEXT, 
            rc_id = self.rc_id, 
            rc_op = RoutingContextOperation.WITH_CUSTOM_PRIVACY, 
            stability = stability))
        return _JsonRoutingContext(self.api, new_rc_id)
    async def with_sequencing(self, sequencing: Sequencing) -> Self:
        new_rc_id = raise_api_result(await self.send_ndjson_request(Operation.ROUTING_CONTEXT, 
            rc_id = self.rc_id, 
            rc_op = RoutingContextOperation.WITH_SEQUENCING, 
            sequencing = sequencing))
        return _JsonRoutingContext(self.api, new_rc_id)
    async def app_call(self, target: TypedKey | RouteId, request: bytes) -> bytes:
        return urlsafe_b64decode_no_pad(raise_api_result(await self.send_ndjson_request(Operation.ROUTING_CONTEXT, 
            rc_id = self.rc_id, 
            rc_op = RoutingContextOperation.APP_CALL,
            target = target,
            request = request)))
    async def app_message(self, target: TypedKey | RouteId, message: bytes):
        raise_api_result(await self.send_ndjson_request(Operation.ROUTING_CONTEXT, 
            rc_id = self.rc_id, 
            rc_op = RoutingContextOperation.APP_MESSAGE,
            target = target,
            message = message))
    async def create_dht_record(self, kind: CryptoKind, schema: DHTSchema) -> DHTRecordDescriptor:
        return DHTRecordDescriptor.from_json(raise_api_result(await self.send_ndjson_request(Operation.ROUTING_CONTEXT, 
            rc_id = self.rc_id, 
            rc_op = RoutingContextOperation.CREATE_DHT_RECORD,
            kind = kind,
            schema = schema)))
    async def open_dht_record(self, key: TypedKey, writer: Optional[KeyPair]) -> DHTRecordDescriptor:
        pass
    async def close_dht_record(self, key: TypedKey):
        pass
    async def delete_dht_record(self, key: TypedKey):
        pass
    async def get_dht_value(self, key: TypedKey, subkey: ValueSubkey, force_refresh: bool) -> Optional[ValueData]:
        pass
    async def set_dht_value(self, key: TypedKey, subkey: ValueSubkey, data: bytes) -> Optional[ValueData]:
        pass
    async def watch_dht_values(self, key: TypedKey, subkeys: list[(ValueSubkey, ValueSubkey)], expiration: Timestamp, count: int) -> Timestamp:
        pass
    async def cancel_dht_values(self, key: TypedKey, subkeys: list[(ValueSubkey, ValueSubkey)]) -> bool:
        pass
    

class _JsonTableDBTransaction(TableDBTransaction):
    async def commit(self):
        pass
    async def rollback(self):
        pass
    async def store(self, col: int, key: bytes, value: bytes):
        pass
    async def delete(self, col: int, key: bytes):
        pass

class _JsonTableDB(TableDB):
    async def get_column_count(self) -> int:
        pass
    async def get_keys(self, col: int) -> list[str]:
        pass
    async def transact(self) -> TableDBTransaction:
        pass
    async def store(self, col: int, key: bytes, value: bytes):
        pass
    async def load(self, col: int, key: bytes) -> Optional[bytes]:
        pass
    async def delete(self, col: int, key: bytes) -> Optional[bytes]:
        pass

class _JsonCryptoSystem(CryptoSystem):
    async def cached_dh(self, key: PublicKey, secret: SecretKey) -> SharedSecret:
        pass
    async def compute_dh(self, key: PublicKey, secret: SecretKey) -> SharedSecret:
        pass
    async def random_bytes(self, len: int) -> bytes:
        pass
    async def default_salt_length(self) -> int:
        pass
    async def hash_password(self, password: bytes, salt: bytes) -> str:
        pass
    async def verify_password(self, password: bytes, password_hash: str) -> bool:
        pass
    async def derive_shared_secret(self, password: bytes, salt: bytes) -> SharedSecret:
        pass
    async def random_nonce(self) -> Nonce:
        pass
    async def random_shared_secret(self) -> SharedSecret:
        pass
    async def generate_key_pair(self) -> KeyPair:
        pass
    async def generate_hash(self, data: bytes) -> HashDigest:
        pass
    async def validate_key_pair(self, key: PublicKey, secret: SecretKey) -> bool:
        pass
    async def validate_hash(self, data: bytes, hash_digest: HashDigest) -> bool:
        pass
    async def distance(self, key1: CryptoKey, key2: CryptoKey) -> CryptoKeyDistance:
        pass
    async def sign(self, key: PublicKey, secret: SecretKey, data: bytes) -> Signature:
        pass
    async def verify(self, key: PublicKey, data: bytes, signature: Signature):
        pass
    async def aead_overhead(self) -> int:
        pass
    async def decrypt_aead(self, body: bytes, nonce: Nonce, shared_secret: SharedSecret, associated_data: Optional[bytes]) -> bytes:
        pass
    async def encrypt_aead(self, body: bytes, nonce: Nonce, shared_secret: SharedSecret, associated_data: Optional[bytes]) -> bytes:
        pass
    async def crypt_no_auth(self, body: bytes, nonce: Nonce, shared_secret: SharedSecret) -> bytes:
        pass


class _JsonVeilidAPI(VeilidAPI):
    reader: asyncio.StreamReader
    writer: asyncio.StreamWriter
    update_callback: Callable[[VeilidUpdate], Awaitable]
    handle_recv_messages_task: Optional[asyncio.Task]
    # Shared Mutable State
    lock: asyncio.Lock
    next_id: int
    in_flight_requests: dict
    
    def __init__(self, reader: asyncio.StreamReader, writer: asyncio.StreamWriter, update_callback: Callable[[VeilidUpdate], Awaitable]):
        self.reader = reader
        self.writer = writer
        self.update_callback = update_callback
        self.handle_recv_messages_task = None
        self.lock = asyncio.Lock()
        self.next_id = 1
        self.in_flight_requests = dict()

    @staticmethod
    async def connect(host: str, port: int, update_callback: Callable[[VeilidUpdate], Awaitable]) -> Self:
        reader, writer = await asyncio.open_connection(host, port)
        veilid_api = _JsonVeilidAPI(reader, writer, update_callback)
        veilid_api.handle_recv_messages_task = asyncio.create_task(veilid_api.handle_recv_messages(), name = "JsonVeilidAPI.handle_recv_messages")
        return veilid_api
    
    async def handle_recv_message_response(self, j: dict):
        id = j['id']
        await self.lock.acquire()
        try:
            # Get and remove the in-flight request
            reqfuture = self.in_flight_requests.pop(id, None)
        finally:
            self.lock.release()
        # Resolve the request's future to the response json
        reqfuture.set_result(j)

    async def handle_recv_messages(self):
        # Read lines until we're done
        try:
            while True:
                linebytes = await self.reader.readline()
                if not linebytes.endswith(b'\n'):
                    break
                    
                # Parse line as ndjson
                j = json.loads(linebytes.strip())

                # Process the message
                if j['type'] == "Response":
                    await self.handle_recv_message_response(j)
                elif j['type'] == "Update":
                    await self.update_callback(VeilidUpdate.from_json(j))
        except:
            pass
        finally:
            self.reader = None
            self.writer.close()
            await self.writer.wait_closed()
            self.writer = None
            
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
            reqfuture.cancel()
        finally:
            self.lock.release()

    async def send_ndjson_request(self, op: Operation, **kwargs) -> dict:

        # Get next id
        await self.lock.acquire()
        try:
            id = self.next_id
            self.next_id += 1
        finally:
            self.lock.release()

        # Make NDJSON string for request
        req = { "id": id, "op": op }
        for k, v in kwargs.items():
            setattr(req, k, v)
        reqstr = VeilidJSONEncoder.dumps(req) + "\n"
        reqbytes = reqstr.encode()
    
        # Allocate future for request
        reqfuture = await self.allocate_request_future(id)

        # Send to socket
        try:
            self.writer.write(reqbytes)
            await self.writer.drain()
        finally:
            # Send failed, release future
            self.cancel_request_future(id)

        # Wait for response
        response = await reqfuture

        return response

    async def control(self, args: list[str]) -> str:
        return raise_api_result(await self.send_ndjson_request(Operation.CONTROL, args = args))
    async def get_state(self) -> VeilidState:
        return VeilidState.from_json(raise_api_result(await self.send_ndjson_request(Operation.GET_STATE)))
    async def attach(self):
        raise_api_result(await self.send_ndjson_request(Operation.ATTACH))
    async def detach(self):
        raise_api_result(await self.send_ndjson_request(Operation.DETACH))
    async def new_private_route(self) -> NewPrivateRouteResult:
        return NewPrivateRouteResult.from_json(raise_api_result(await self.send_ndjson_request(Operation.NEW_PRIVATE_ROUTE)))
    async def new_custom_private_route(self, kinds: list[CryptoKind], stability: Stability, sequencing: Sequencing) -> NewPrivateRouteResult:
        return NewPrivateRouteResult.from_json(raise_api_result(
            await self.send_ndjson_request(Operation.NEW_CUSTOM_PRIVATE_ROUTE,
                kinds = kinds,
                stability = stability,
                sequencing = sequencing)
            ))
    async def import_remote_private_route(self, blob: bytes) -> RouteId:
        return RouteId(raise_api_result(
            await self.send_ndjson_request(Operation.IMPORT_REMOTE_PRIVATE_ROUTE,
                blob = blob)
            ))
    async def release_private_route(self, route_id: RouteId):
        raise_api_result(
            await self.send_ndjson_request(Operation.RELEASE_PRIVATE_ROUTE,
                route_id = route_id)
            )
    async def app_call_reply(self, call_id: OperationId, message: bytes):
        raise_api_result(
            await self.send_ndjson_request(Operation.APP_CALL_REPLY,
                call_id = call_id,
                message = message)
            )
    async def new_routing_context(self) -> RoutingContext:
        rc_id = raise_api_result(await self.send_ndjson_request(Operation.NEW_ROUTING_CONTEXT))
        return RoutingContext(self, rc_id)
        
    async def open_table_db(self, name: str, column_count: int) -> TableDB:
        pass
    async def delete_table_db(self, name: str):
        pass
    async def get_crypto_system(self, kind: CryptoKind) -> CryptoSystem:
        pass
    async def best_crypto_system(self) -> CryptoSystem:
        pass
    async def verify_signatures(self, node_ids: list[TypedKey], data: bytes, signatures: list[TypedSignature]) -> list[TypedKey]:
        pass
    async def generate_signatures(self, data: bytes, key_pairs: list[TypedKeyPair]) -> list[TypedSignature]:
        pass
    async def generate_key_pair(self, kind: CryptoKind) -> list[TypedKeyPair]:
        pass
    async def now(self) -> Timestamp:
        pass
    async def debug(self, command: str) -> str:
        pass
    async def veilid_version_string(self) -> str:
        pass
    async def veilid_version(self) -> VeilidVersion:
        pass


def json_api_connect(host:str, port:int) -> VeilidAPI:
    return _JsonVeilidAPI.connect(host, port)