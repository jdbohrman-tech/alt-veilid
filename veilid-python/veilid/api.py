from abc import ABC, abstractmethod
from typing import Optional, Self

from . import types
from .state import VeilidState


class RoutingContext(ABC):
    ref_count: int

    def __init__(
        self,
    ):
        self.ref_count = 0

    async def __aenter__(self) -> Self:
        self.ref_count += 1
        return self

    async def __aexit__(self, *excinfo):
        self.ref_count -= 1
        if self.ref_count == 0 and not self.is_done():
            await self.release()

    @abstractmethod
    def is_done(self) -> bool:
        pass

    @abstractmethod
    async def release(self):
        pass

    @abstractmethod
    async def with_default_safety(self, release=True) -> Self:
        pass

    @abstractmethod
    async def with_safety(
        self, safety_selection: types.SafetySelection, release=True
    ) -> Self:
        pass

    @abstractmethod
    async def with_sequencing(self, sequencing: types.Sequencing, release=True) -> Self:
        pass

    @abstractmethod
    async def safety(self) -> types.SafetySelection:
        pass

    @abstractmethod
    async def app_call(self, target: types.TypedKey | types.RouteId, request: bytes) -> bytes:
        pass

    @abstractmethod
    async def app_message(self, target: types.TypedKey | types.RouteId, message: bytes):
        pass

    @abstractmethod
    async def create_dht_record(
        self, schema: types.DHTSchema, owner: Optional[types.KeyPair] = None, kind: Optional[types.CryptoKind] = None
    ) -> types.DHTRecordDescriptor:
        pass

    @abstractmethod
    async def open_dht_record(
        self, key: types.TypedKey, writer: Optional[types.KeyPair] = None
    ) -> types.DHTRecordDescriptor:
        pass

    @abstractmethod
    async def close_dht_record(self, key: types.TypedKey):
        pass

    @abstractmethod
    async def delete_dht_record(self, key: types.TypedKey):
        pass

    @abstractmethod
    async def get_dht_value(
        self, key: types.TypedKey, subkey: types.ValueSubkey, force_refresh: bool = False
    ) -> Optional[types.ValueData]:
        pass

    @abstractmethod
    async def set_dht_value(
        self, key: types.TypedKey, subkey: types.ValueSubkey, data: bytes, writer: Optional[types.KeyPair] = None
    ) -> Optional[types.ValueData]:
        pass

    @abstractmethod
    async def watch_dht_values(
        self,
        key: types.TypedKey,
        subkeys: list[tuple[types.ValueSubkey, types.ValueSubkey]] = [],
        expiration: types.Timestamp = types.Timestamp(0),
        count: int = 0xFFFFFFFF,
    ) -> bool:
        pass

    @abstractmethod
    async def cancel_dht_watch(
        self,
        key: types.TypedKey,
        subkeys: list[tuple[types.ValueSubkey, types.ValueSubkey]] = [],
    ) -> bool:
        pass

    @abstractmethod
    async def inspect_dht_record(
        self,
        key: types.TypedKey,
        subkeys: list[tuple[types.ValueSubkey, types.ValueSubkey]],
        scope: types.DHTReportScope = types.DHTReportScope.LOCAL,
    ) -> types.DHTRecordReport:
        pass



class TableDbTransaction(ABC):
    ref_count: int

    def __init__(
        self,
    ):
        self.ref_count = 0

    async def __aenter__(self) -> Self:
        self.ref_count += 1
        return self

    async def __aexit__(self, *excinfo):
        self.ref_count -= 1
        if self.ref_count == 0 and not self.is_done():
            await self.release()

    @abstractmethod
    def is_done(self) -> bool:
        pass

    @abstractmethod
    async def commit(self):
        pass

    @abstractmethod
    async def rollback(self):
        pass

    @abstractmethod
    async def store(self, key: bytes, value: bytes, col: int = 0):
        pass

    @abstractmethod
    async def delete(self, key: bytes, col: int = 0):
        pass


class TableDb(ABC):
    ref_count: int

    def __init__(
        self,
    ):
        self.ref_count = 0

    async def __aenter__(self) -> Self:
        self.ref_count += 1
        return self

    async def __aexit__(self, *excinfo):
        self.ref_count -= 1
        if self.ref_count == 0 and not self.is_done():
            await self.release()

    @abstractmethod
    def is_done(self) -> bool:
        pass

    @abstractmethod
    async def release(self):
        pass

    @abstractmethod
    async def get_column_count(self) -> int:
        pass

    @abstractmethod
    async def get_keys(self, col: int = 0) -> list[bytes]:
        pass

    @abstractmethod
    async def transact(self) -> TableDbTransaction:
        pass

    @abstractmethod
    async def store(self, key: bytes, value: bytes, col: int = 0):
        pass

    @abstractmethod
    async def load(self, key: bytes, col: int = 0) -> Optional[bytes]:
        pass

    @abstractmethod
    async def delete(self, key: bytes, col: int = 0) -> Optional[bytes]:
        pass


class CryptoSystem(ABC):
    ref_count: int

    def __init__(
        self,
    ):
        self.ref_count = 0

    async def __aenter__(self) -> Self:
        self.ref_count += 1
        return self

    async def __aexit__(self, *excinfo):
        self.ref_count -= 1
        if self.ref_count == 0 and not self.is_done():
            await self.release()

    @abstractmethod
    async def kind(self) -> types.CryptoKind:
        pass

    @abstractmethod
    def is_done(self) -> bool:
        pass

    @abstractmethod
    async def release(self):
        pass

    @abstractmethod
    async def cached_dh(self, key: types.PublicKey, secret: types.SecretKey) -> types.SharedSecret:
        pass

    @abstractmethod
    async def compute_dh(
        self, key: types.PublicKey, secret: types.SecretKey
    ) -> types.SharedSecret:
        pass

    @abstractmethod
    async def generate_shared_secret(
        self, key: types.PublicKey, secret: types.SecretKey, domain: bytes
    ) -> types.SharedSecret:
        pass

    @abstractmethod
    async def random_bytes(self, len: int) -> bytes:
        pass

    @abstractmethod
    async def default_salt_length(self) -> int:
        pass

    @abstractmethod
    async def hash_password(self, password: bytes, salt: bytes) -> str:
        pass

    @abstractmethod
    async def verify_password(self, password: bytes, password_hash: str) -> bool:
        pass

    @abstractmethod
    async def derive_shared_secret(self, password: bytes, salt: bytes) -> types.SharedSecret:
        pass

    @abstractmethod
    async def random_nonce(self) -> types.Nonce:
        pass

    @abstractmethod
    async def random_shared_secret(self) -> types.SharedSecret:
        pass

    @abstractmethod
    async def generate_key_pair(self) -> types.KeyPair:
        pass

    @abstractmethod
    async def generate_hash(self, data: bytes) -> types.HashDigest:
        pass

    @abstractmethod
    async def validate_key_pair(self, key: types.PublicKey, secret: types.SecretKey) -> bool:
        pass

    @abstractmethod
    async def validate_hash(self, data: bytes, hash_digest: types.HashDigest) -> bool:
        pass

    @abstractmethod
    async def distance(
        self, key1: types.HashDigest, key2: types.HashDigest
    ) -> types.HashDistance:
        pass

    @abstractmethod
    async def sign(
        self, key: types.PublicKey, secret: types.SecretKey, data: bytes
    ) -> types.Signature:
        pass

    @abstractmethod
    async def verify(self, key: types.PublicKey, data: bytes, signature: types.Signature) -> bool:
        pass

    @abstractmethod
    async def aead_overhead(self) -> int:
        pass

    @abstractmethod
    async def decrypt_aead(
        self,
        body: bytes,
        nonce: types.Nonce,
        shared_secret: types.SharedSecret,
        associated_data: Optional[bytes],
    ) -> bytes:
        pass

    @abstractmethod
    async def encrypt_aead(
        self,
        body: bytes,
        nonce: types.Nonce,
        shared_secret: types.SharedSecret,
        associated_data: Optional[bytes],
    ) -> bytes:
        pass

    @abstractmethod
    async def crypt_no_auth(
        self, body: bytes, nonce: types.Nonce, shared_secret: types.SharedSecret
    ) -> bytes:
        pass


class VeilidAPI(ABC):
    ref_count: int

    def __init__(
        self,
    ):
        self.ref_count = 0

    async def __aenter__(self) -> Self:
        self.ref_count += 1
        return self

    async def __aexit__(self, *excinfo):
        self.ref_count -= 1
        if self.ref_count == 0 and not self.is_done():
            await self.release()

    @abstractmethod
    def is_done(self) -> bool:
        pass

    @abstractmethod
    async def release(self):
        pass

    @abstractmethod
    async def control(self, args: list[str]) -> str:
        pass

    @abstractmethod
    async def get_state(self) -> VeilidState:
        pass

    @abstractmethod
    async def is_shutdown(self) -> bool:
        pass

    @abstractmethod
    async def attach(self):
        pass

    @abstractmethod
    async def detach(self):
        pass

    @abstractmethod
    async def new_private_route(self) -> tuple[types.RouteId, bytes]:
        pass

    @abstractmethod
    async def new_custom_private_route(
        self,
        kinds: list[types.CryptoKind],
        stability: types.Stability,
        sequencing: types.Sequencing,
    ) -> tuple[types.RouteId, bytes]:
        pass

    @abstractmethod
    async def import_remote_private_route(self, blob: bytes) -> types.RouteId:
        pass

    @abstractmethod
    async def release_private_route(self, route_id: types.RouteId):
        pass

    @abstractmethod
    async def app_call_reply(self, call_id: types.OperationId, message: bytes):
        pass

    @abstractmethod
    async def new_routing_context(self) -> RoutingContext:
        pass

    @abstractmethod
    async def open_table_db(self, name: str, column_count: int) -> TableDb:
        pass

    @abstractmethod
    async def delete_table_db(self, name: str) -> bool:
        pass

    @abstractmethod
    async def get_crypto_system(self, kind: types.CryptoKind) -> CryptoSystem:
        pass

    @abstractmethod
    async def best_crypto_system(self) -> CryptoSystem:
        pass

    @abstractmethod
    async def verify_signatures(
        self,
        node_ids: list[types.TypedKey],
        data: bytes,
        signatures: list[types.TypedSignature],
    ) -> Optional[list[types.TypedKey]]:
        pass

    @abstractmethod
    async def generate_signatures(
        self, data: bytes, key_pairs: list[types.TypedKeyPair]
    ) -> list[types.TypedSignature]:
        pass

    @abstractmethod
    async def generate_key_pair(self, kind: types.CryptoKind) -> list[types.TypedKeyPair]:
        pass

    @abstractmethod
    async def now(self) -> types.Timestamp:
        pass

    @abstractmethod
    async def debug(self, command: str) -> str:
        pass

    @abstractmethod
    async def veilid_version_string(self) -> str:
        pass

    @abstractmethod
    async def veilid_features(self) -> list[str]:
        pass

    @abstractmethod
    async def veilid_version(self) -> types.VeilidVersion:
        pass

    @abstractmethod
    async def default_veilid_config(self) -> str:
        pass
