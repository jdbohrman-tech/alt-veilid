from abc import ABC, abstractmethod
from typing import Self

from .state import *
from .config import *
from .error import *
from .types import *

class RoutingContext(ABC):
    @abstractmethod  
    async def with_privacy(self) -> Self:
        pass
    @abstractmethod  
    async def with_custom_privacy(self, stability: Stability) -> Self:
        pass
    @abstractmethod  
    async def with_sequencing(self, sequencing: Sequencing) -> Self:
        pass
    @abstractmethod  
    async def app_call(self, target: TypedKey | RouteId, request: bytes) -> bytes:
        pass
    @abstractmethod  
    async def app_message(self, target: TypedKey | RouteId, message: bytes):
        pass
    @abstractmethod  
    async def create_dht_record(self, kind: CryptoKind, schema: DHTSchema) -> DHTRecordDescriptor:
        pass
    @abstractmethod  
    async def open_dht_record(self, key: TypedKey, writer: Optional[KeyPair]) -> DHTRecordDescriptor:
        pass
    @abstractmethod  
    async def close_dht_record(self, key: TypedKey):
        pass
    @abstractmethod  
    async def delete_dht_record(self, key: TypedKey):
        pass
    @abstractmethod  
    async def get_dht_value(self, key: TypedKey, subkey: ValueSubkey, force_refresh: bool) -> Optional[ValueData]:
        pass
    @abstractmethod  
    async def set_dht_value(self, key: TypedKey, subkey: ValueSubkey, data: bytes) -> Optional[ValueData]:
        pass
    @abstractmethod  
    async def watch_dht_values(self, key: TypedKey, subkeys: list[(ValueSubkey, ValueSubkey)], expiration: Timestamp, count: int) -> Timestamp:
        pass
    @abstractmethod  
    async def cancel_dht_values(self, key: TypedKey, subkeys: list[(ValueSubkey, ValueSubkey)]) -> bool:
        pass
    

class TableDBTransaction(ABC):
    @abstractmethod  
    async def commit(self):
        pass
    @abstractmethod  
    async def rollback(self):
        pass
    @abstractmethod  
    async def store(self, col: int, key: bytes, value: bytes):
        pass
    @abstractmethod  
    async def delete(self, col: int, key: bytes):
        pass

class TableDB(ABC):
    @abstractmethod  
    async def get_column_count(self) -> int:
        pass
    @abstractmethod  
    async def get_keys(self, col: int) -> list[str]:
        pass
    @abstractmethod  
    async def transact(self) -> TableDBTransaction:
        pass
    @abstractmethod  
    async def store(self, col: int, key: bytes, value: bytes):
        pass
    @abstractmethod  
    async def load(self, col: int, key: bytes) -> Optional[bytes]:
        pass
    @abstractmethod  
    async def delete(self, col: int, key: bytes) -> Optional[bytes]:
        pass

class CryptoSystem(ABC):
    @abstractmethod  
    async def cached_dh(self, key: PublicKey, secret: SecretKey) -> SharedSecret:
        pass
    @abstractmethod  
    async def compute_dh(self, key: PublicKey, secret: SecretKey) -> SharedSecret:
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
    async def derive_shared_secret(self, password: bytes, salt: bytes) -> SharedSecret:
        pass
    @abstractmethod  
    async def random_nonce(self) -> Nonce:
        pass
    @abstractmethod  
    async def random_shared_secret(self) -> SharedSecret:
        pass
    @abstractmethod  
    async def generate_key_pair(self) -> KeyPair:
        pass
    @abstractmethod  
    async def generate_hash(self, data: bytes) -> HashDigest:
        pass
    @abstractmethod  
    async def validate_key_pair(self, key: PublicKey, secret: SecretKey) -> bool:
        pass
    @abstractmethod  
    async def validate_hash(self, data: bytes, hash_digest: HashDigest) -> bool:
        pass
    @abstractmethod  
    async def distance(self, key1: CryptoKey, key2: CryptoKey) -> CryptoKeyDistance:
        pass
    @abstractmethod  
    async def sign(self, key: PublicKey, secret: SecretKey, data: bytes) -> Signature:
        pass
    @abstractmethod  
    async def verify(self, key: PublicKey, data: bytes, signature: Signature):
        pass
    @abstractmethod  
    async def aead_overhead(self) -> int:
        pass
    @abstractmethod  
    async def decrypt_aead(self, body: bytes, nonce: Nonce, shared_secret: SharedSecret, associated_data: Optional[bytes]) -> bytes:
        pass
    @abstractmethod  
    async def encrypt_aead(self, body: bytes, nonce: Nonce, shared_secret: SharedSecret, associated_data: Optional[bytes]) -> bytes:
        pass
    @abstractmethod  
    async def crypt_no_auth(self, body: bytes, nonce: Nonce, shared_secret: SharedSecret) -> bytes:
        pass


class VeilidAPI(ABC):
    @abstractmethod  
    async def control(self, args: list[str]) -> str:
        pass
    @abstractmethod  
    async def get_state(self) -> VeilidState:
        pass
    @abstractmethod  
    async def attach(self):
        pass
    @abstractmethod  
    async def detach(self):
        pass
    @abstractmethod  
    async def new_private_route(self) -> NewPrivateRouteResult:
        pass
    @abstractmethod  
    async def new_custom_private_route(self, kinds: list[CryptoKind], stability: Stability, sequencing: Sequencing) -> NewPrivateRouteResult:
        pass
    @abstractmethod  
    async def import_remote_private_route(self, blob: bytes) -> RouteId:
        pass
    @abstractmethod  
    async def release_private_route(self, route_id: RouteId):
        pass
    @abstractmethod  
    async def app_call_reply(self, call_id: OperationId, message: bytes):
        pass
    @abstractmethod  
    async def new_routing_context(self) -> RoutingContext:
        pass
    @abstractmethod  
    async def open_table_db(self, name: str, column_count: int) -> TableDB:
        pass
    @abstractmethod  
    async def delete_table_db(self, name: str):
        pass
    @abstractmethod  
    async def get_crypto_system(self, kind: CryptoKind) -> CryptoSystem:
        pass
    @abstractmethod  
    async def best_crypto_system(self) -> CryptoSystem:
        pass
    @abstractmethod  
    async def verify_signatures(self, node_ids: list[TypedKey], data: bytes, signatures: list[TypedSignature]) -> list[TypedKey]:
        pass
    @abstractmethod  
    async def generate_signatures(self, data: bytes, key_pairs: list[TypedKeyPair]) -> list[TypedSignature]:
        pass
    @abstractmethod  
    async def generate_key_pair(self, kind: CryptoKind) -> list[TypedKeyPair]:
        pass
    @abstractmethod  
    async def now(self) -> Timestamp:
        pass
    @abstractmethod  
    async def debug(self, command: str) -> str:
        pass
    @abstractmethod  
    async def veilid_version_string(self) -> str:
        pass
    @abstractmethod  
    async def veilid_version(self) -> VeilidVersion:
        pass
