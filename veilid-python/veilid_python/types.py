import time
import json
import base64

from enum import StrEnum
from typing import Self, Optional, Any

def urlsafe_b64encode_no_pad(b: bytes) -> str:
    """
    Removes any `=` used as padding from the encoded string.
    """
    encoded = str(base64.urlsafe_b64encode(b))
    return encoded.rstrip("=")


def urlsafe_b64decode_no_pad(s: str) -> bytes:
    """
    Adds back in the required padding before decoding.
    """
    padding = 4 - (len(s) % 4)
    string = string + ("=" * padding)
    return base64.urlsafe_b64decode(s)

class VeilidJSONEncoder(json.JSONEncoder):
    def default(self, o):
        if isinstance(o, bytes):
            return urlsafe_b64encode_no_pad(o)
        if hasattr(o, "to_json") and callable(o.to_json):
            return o.to_json()
        return json.JSONEncoder.default(self, o)
    
    @staticmethod
    def dumps(req: Any, *args, **kwargs) -> str:
        return json.dumps(req, cls = VeilidJSONEncoder, *args, **kwargs)


class Timestamp(int):
    pass

class TimestampDuration(int):
    pass

class ByteCount(int):
    pass

class OperationId(int):
    pass

class RouteId(str):
    pass

class TypedKey(str):
    pass

class TypedKeyPair(str):
    pass

class TypedSignature(str):
    pass

class CryptoKey(str):
    pass

class CryptoKeyDistance(str):
    pass

class PublicKey(CryptoKey):
    pass

class SecretKey(CryptoKey):
    pass

class SharedSecret(CryptoKey):
    pass

class Signature(str):
    pass

class Nonce(str):
    pass

class KeyPair(str):
    pass

class HashDigest(CryptoKey):
    pass

class ValueSubkey(int):
    pass

class ValueSeqNum(int):
    pass

class VeilidVersion:
    _major: int
    _minor: int
    _patch: int
    def __init__(self, major: int, minor: int, patch: int):
        self._major = major
        self._minor = minor
        self._patch = patch
    @property
    def major(self):
        return self._major
    @property
    def minor(self):
        return self._minor
    @property
    def patch(self):
        return self._patch

class VeilidLogLevel(StrEnum):
    ERROR = 'Error'
    WARN = 'Warn'
    INFO = 'Info'
    DEBUG = 'Debug'
    TRACE = 'Trace'

class NewPrivateRouteResult:
    route_id: RouteId
    blob: bytes

    def __init__(self, route_id: RouteId, blob: bytes):
        self.route_id = route_id
        self.blob = blob

    @staticmethod
    def from_json(j: dict) -> Self:
        return NewPrivateRouteResult(
            RouteId(j['route_id']),
            urlsafe_b64decode_no_pad(j['blob']))

class CryptoKind(StrEnum):
    CRYPTO_KIND_NONE = "NONE"
    CRYPTO_KIND_VLD0 = "VLD0"

class Stability(StrEnum):
    LOW_LATENCY = "LowLatency"
    RELIABLE = "Reliable"

class Sequencing(StrEnum):
    NO_PREFERENCE = "NoPreference"
    PREFER_ORDERED = "PreferOrdered"
    ENSURE_ORDERED = "EnsureOrdered"

class DHTSchemaKind(StrEnum):
    DFLT = "DFLT"
    SMPL = "SMPL"

class DHTSchemaSMPLMember:
    m_key: PublicKey
    m_cnt: int
    def __init__(self, m_key: PublicKey, m_cnt: int):
        self.m_key = m_key
        self.m_cnt = m_cnt
    @staticmethod
    def from_json(j: dict) -> Self:
        return DHTSchemaSMPLMember(
            PublicKey(j['m_key']),
            j['m_cnt'])
    def to_json(self) -> dict:
        return self.__dict__
    
class DHTSchema:
    kind: DHTSchemaKind
    
    def __init__(self, kind: DHTSchemaKind, **kwargs):
        self.kind = kind
        for k, v in kwargs.items():
            setattr(self, k, v)
    
    @staticmethod
    def dflt(o_cnt: int) -> Self:
        Self(DHTSchemaKind.DFLT, o_cnt = o_cnt)
    
    @staticmethod
    def smpl(o_cnt: int, members: list[DHTSchemaSMPLMember]) -> Self:
        Self(DHTSchemaKind.SMPL, o_cnt = o_cnt, members = members)

    @staticmethod
    def from_json(j: dict) -> Self:
        if DHTSchemaKind(j['kind']) == DHTSchemaKind.DFLT:
            return DHTSchema.dflt(j['o_cnt'])
        if DHTSchemaKind(j['kind']) == DHTSchemaKind.SMPL:
            return DHTSchema.smpl(
                j['o_cnt'],
                map(lambda x: DHTSchemaSMPLMember.from_json(x), j['members']))
        raise Exception("Unknown DHTSchema kind", j['kind'])

    def to_json(self) -> dict:
        return self.__dict__

class DHTRecordDescriptor:
    key: TypedKey
    owner: PublicKey
    owner_secret: Optional[SecretKey]
    schema: DHTSchema

    def __init__(self, key: TypedKey, owner: PublicKey, owner_secret: Optional[SecretKey], schema: DHTSchema):
        self.key = key
        self.owner = owner
        self.owner_secret = owner_secret
        self.schema = schema
    
    @staticmethod
    def from_json(j: dict) -> Self:
        DHTRecordDescriptor(
            TypedKey(j['key']),
            PublicKey(j['owner']),
            None if j['owner_secret'] is None else SecretKey(j['owner_secret']),
            DHTSchema.from_json(j['schema']))

    def to_json(self) -> dict:
        return self.__dict__

class ValueData:
    seq: ValueSeqNum
    data: bytes
    writer: PublicKey
    
    def __init__(self, seq: ValueSeqNum, data: bytes, writer: PublicKey):
        self.seq = seq
        self.data = data
        self.writer = writer
        
    @staticmethod
    def from_json(j: dict) -> Self:
        DHTRecordDescriptor(
            ValueSeqNum(j['seq']),
            urlsafe_b64decode_no_pad(j['data']),
            PublicKey(j['writer']))

    def to_json(self) -> dict:
        return self.__dict__

