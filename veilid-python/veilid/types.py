import base64
import json
from enum import StrEnum
from functools import total_ordering
from typing import Any, Optional, Self

####################################################################


def urlsafe_b64encode_no_pad(b: bytes) -> str:
    """
    Removes any `=` used as padding from the encoded string.
    """
    return base64.urlsafe_b64encode(b).decode().rstrip("=")


def urlsafe_b64decode_no_pad(s: str) -> bytes:
    """
    Adds back in the required padding before decoding.
    """
    padding = 4 - (len(s) % 4)
    s = s + ("=" * padding)
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
        return json.dumps(req, cls=VeilidJSONEncoder, *args, **kwargs)


####################################################################


class VeilidLogLevel(StrEnum):
    ERROR = "Error"
    WARN = "Warn"
    INFO = "Info"
    DEBUG = "Debug"
    TRACE = "Trace"


class CryptoKind(StrEnum):
    CRYPTO_KIND_NONE = "NONE"
    CRYPTO_KIND_VLD0 = "VLD0"


class VeilidCapability(StrEnum):
    CAP_ROUTE = "ROUT"
    CAP_TUNNEL = "TUNL"
    CAP_SIGNAL = "SGNL"
    CAP_RELAY = "RLAY"
    CAP_VALIDATE_DIAL_INFO = "DIAL"
    CAP_DHT = "DHTV"
    CAP_DHT_WATCH = "DHTW"
    CAP_APPMESSAGE = "APPM"
    CAP_BLOCKSTORE = "BLOC"


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


class SafetySelectionKind(StrEnum):
    UNSAFE = "Unsafe"
    SAFE = "Safe"


class DHTReportScope(StrEnum):
    LOCAL = "Local"
    SYNC_GET = "SyncGet"
    SYNC_SET = "SyncSet"
    UPDATE_GET = "UpdateGet"
    UPDATE_SET = "UpdateSet"


####################################################################


class Timestamp(int):
    pass


class TimestampDuration(int):
    pass


class ByteCount(int):
    pass


class OperationId(str):
    pass


class RouteId(str):
    pass


class EncodedString(str):
    def to_bytes(self) -> bytes:
        return urlsafe_b64decode_no_pad(self)

    @classmethod
    def from_bytes(cls, b: bytes) -> Self:
        assert isinstance(b, bytes)
        return cls(urlsafe_b64encode_no_pad(b))


class HashDistance(EncodedString):
    pass


class PublicKey(EncodedString):
    pass


class SecretKey(EncodedString):
    pass


class SharedSecret(EncodedString):
    pass


class HashDigest(EncodedString):
    pass


class Signature(EncodedString):
    pass


class Nonce(EncodedString):
    pass


class KeyPair(str):
    @classmethod
    def from_parts(cls, key: PublicKey, secret: SecretKey) -> Self:
        assert isinstance(key, PublicKey)
        assert isinstance(secret, SecretKey)
        return cls(f"{key}:{secret}")

    def key(self) -> PublicKey:
        return PublicKey(self.split(":", 1)[0])

    def secret(self) -> SecretKey:
        return SecretKey(self.split(":", 1)[1])

    def to_parts(self) -> tuple[PublicKey, SecretKey]:
        public, secret = self.split(":", 1)
        return (PublicKey(public), SecretKey(secret))


class CryptoTyped(str):
    def kind(self) -> CryptoKind:
        if self[4] != ":":
            raise ValueError("Not CryptoTyped")
        return CryptoKind(self[0:4])

    def _value(self) -> str:
        if self[4] != ":":
            raise ValueError("Not CryptoTyped")
        return self[5:]


class TypedKey(CryptoTyped):
    @classmethod
    def from_value(cls, kind: CryptoKind, value: PublicKey) -> Self:
        assert isinstance(kind, CryptoKind)
        assert isinstance(value, PublicKey)
        return cls(f"{kind}:{value}")

    def value(self) -> PublicKey:
        return PublicKey(self._value())


class TypedSecret(CryptoTyped):
    @classmethod
    def from_value(cls, kind: CryptoKind, value: SecretKey) -> Self:
        assert isinstance(kind, CryptoKind)
        assert isinstance(value, SecretKey)
        return cls(f"{kind}:{value}")

    def value(self) -> SecretKey:
        return SecretKey(self._value())


class TypedKeyPair(CryptoTyped):
    @classmethod
    def from_value(cls, kind: CryptoKind, value: KeyPair) -> Self:
        assert isinstance(kind, CryptoKind)
        assert isinstance(value, KeyPair)
        return cls(f"{kind}:{value}")

    def value(self) -> KeyPair:
        return KeyPair(self._value())


class TypedSignature(CryptoTyped):
    @classmethod
    def from_value(cls, kind: CryptoKind, value: Signature) -> Self:
        assert isinstance(kind, CryptoKind)
        assert isinstance(value, Signature)
        return cls(f"{kind}:{value}")

    def value(self) -> Signature:
        return Signature(self._value())


class ValueSubkey(int):
    pass


class ValueSeqNum(int):
    pass

####################################################################


@total_ordering
class VeilidVersion:
    _major: int
    _minor: int
    _patch: int

    def __init__(self, major: int, minor: int, patch: int):
        self._major = major
        self._minor = minor
        self._patch = patch

    def __lt__(self, other):
        if other is None:
            return False
        if self._major < other._major:
            return True
        if self._major > other._major:
            return False
        if self._minor < other._minor:
            return True
        if self._minor > other._minor:
            return False
        if self._patch < other._patch:
            return True
        return False

    def __eq__(self, other):
        return (
            isinstance(other, VeilidVersion)
            and self.data == other.data
            and self.seq == other.seq
            and self.writer == other.writer
        )

    @property
    def major(self):
        return self._major

    @property
    def minor(self):
        return self._minor

    @property
    def patch(self):
        return self._patch


class NewPrivateRouteResult:
    route_id: RouteId
    blob: bytes

    def __init__(self, route_id: RouteId, blob: bytes):
        assert isinstance(route_id, RouteId)
        assert isinstance(blob, bytes)

        self.route_id = route_id
        self.blob = blob

    def to_tuple(self) -> tuple[RouteId, bytes]:
        return (self.route_id, self.blob)

    @classmethod
    def from_json(cls, j: dict) -> Self:
        return cls(RouteId(j["route_id"]), urlsafe_b64decode_no_pad(j["blob"]))


class DHTSchemaSMPLMember:
    m_key: PublicKey
    m_cnt: int

    def __init__(self, m_key: PublicKey, m_cnt: int):
        assert isinstance(m_key, PublicKey)
        assert isinstance(m_cnt, int)

        self.m_key = m_key
        self.m_cnt = m_cnt

    @classmethod
    def from_json(cls, j: dict) -> Self:
        return cls(PublicKey(j["m_key"]), j["m_cnt"])

    def to_json(self) -> dict:
        return self.__dict__


class DHTSchema:
    kind: DHTSchemaKind

    def __init__(self, kind: DHTSchemaKind, **kwargs):
        self.kind = kind
        for k, v in kwargs.items():
            setattr(self, k, v)

    @classmethod
    def dflt(cls, o_cnt: int) -> Self:
        assert isinstance(o_cnt, int)
        return cls(DHTSchemaKind.DFLT, o_cnt=o_cnt)

    @classmethod
    def smpl(cls, o_cnt: int, members: list[DHTSchemaSMPLMember]) -> Self:
        assert isinstance(o_cnt, int)
        assert isinstance(members, list)
        for m in members:
            assert isinstance(m, DHTSchemaSMPLMember)
        return cls(DHTSchemaKind.SMPL, o_cnt=o_cnt, members=members)

    @classmethod
    def from_json(cls, j: dict) -> Self:
        if DHTSchemaKind(j["kind"]) == DHTSchemaKind.DFLT:
            return cls.dflt(j["o_cnt"])
        if DHTSchemaKind(j["kind"]) == DHTSchemaKind.SMPL:
            return cls.smpl(
                j["o_cnt"],
                [DHTSchemaSMPLMember.from_json(member) for member in j["members"]],
            )
        raise Exception("Unknown DHTSchema kind", j["kind"])

    def to_json(self) -> dict:
        return self.__dict__


class DHTRecordDescriptor:
    key: TypedKey
    owner: PublicKey
    owner_secret: Optional[SecretKey]
    schema: DHTSchema

    def __init__(
        self,
        key: TypedKey,
        owner: PublicKey,
        owner_secret: Optional[SecretKey],
        schema: DHTSchema,
    ):
        self.key = key
        self.owner = owner
        self.owner_secret = owner_secret
        self.schema = schema

    def __repr__(self) -> str:
        return f"<{self.__class__.__name__}(key={self.key!r}, owner={self.owner!r}, owner_secret={self.owner_secret!r}, schema={self.schema!r})>"

    def owner_key_pair(self) -> Optional[KeyPair]:
        return KeyPair.from_parts(self.owner, self.owner_secret)

    @classmethod
    def from_json(cls, j: dict) -> Self:
        return cls(
            TypedKey(j["key"]),
            PublicKey(j["owner"]),
            None if j["owner_secret"] is None else SecretKey(j["owner_secret"]),
            DHTSchema.from_json(j["schema"]),
        )

    def to_json(self) -> dict:
        return self.__dict__



class DHTRecordReport:
    subkeys: list[tuple[ValueSubkey, ValueSubkey]]
    offline_subkeys: list[tuple[ValueSubkey, ValueSubkey]]
    local_seqs: list[Optional[ValueSeqNum]]
    network_seqs: list[Optional[ValueSeqNum]]

    def __init__(
        self,
        subkeys: list[tuple[ValueSubkey, ValueSubkey]],
        offline_subkeys: list[tuple[ValueSubkey, ValueSubkey]],
        local_seqs: list[Optional[ValueSeqNum]],
        network_seqs: list[Optional[ValueSeqNum]],
    ):
        self.subkeys = subkeys
        self.offline_subkeys = offline_subkeys
        self.local_seqs = local_seqs
        self.network_seqs = network_seqs

    def __repr__(self) -> str:
        return f"<{self.__class__.__name__}(subkeys={self.subkeys!r}, offline_subkeys={self.offline_subkeys!r}, local_seqs={self.local_seqs!r}, network_seqs={self.network_seqs!r})>"

    @classmethod
    def from_json(cls, j: dict) -> Self:
        return cls(
            [(p[0], p[1]) for p in j["subkeys"]],
            [(p[0], p[1]) for p in j["offline_subkeys"]],
            [(ValueSeqNum(s) if s is not None else None) for s in j["local_seqs"] ],
            [(ValueSeqNum(s) if s is not None else None) for s in j["network_seqs"] ],
        )

    def to_json(self) -> dict:
        return self.__dict__


@total_ordering
class ValueData:
    seq: ValueSeqNum
    data: bytes
    writer: PublicKey

    def __init__(self, seq: ValueSeqNum, data: bytes, writer: PublicKey):
        self.seq = seq
        self.data = data
        self.writer = writer

    def __repr__(self) -> str:
        return f"<{self.__class__.__name__}(seq={self.seq!r}, data={self.data!r}, writer={self.writer!r})>"

    def __lt__(self, other):
        if other is None:
            return True
        if self.data < other.data:
            return True
        if self.data > other.data:
            return False
        if self.seq < other.seq:
            return True
        if self.seq > other.seq:
            return False
        if self.writer < other.writer:
            return True
        return False

    def __eq__(self, other):
        return (
            isinstance(other, ValueData)
            and self.data == other.data
            and self.seq == other.seq
            and self.writer == other.writer
        )

    @classmethod
    def from_json(cls, j: dict) -> Self:
        return cls(
            ValueSeqNum(j["seq"]),
            urlsafe_b64decode_no_pad(j["data"]),
            PublicKey(j["writer"]),
        )

    def to_json(self) -> dict:
        return self.__dict__


####################################################################


class SafetySpec:
    preferred_route: Optional[RouteId]
    hop_count: int
    stability: Stability
    sequencing: Sequencing

    def __init__(
        self,
        preferred_route: Optional[RouteId],
        hop_count: int,
        stability: Stability,
        sequencing: Sequencing,
    ):
        self.preferred_route = preferred_route
        self.hop_count = hop_count
        self.stability = stability
        self.sequencing = sequencing

    @classmethod
    def from_json(cls, j: dict) -> Self:
        return cls(
            RouteId(j["preferred_route"]) if "preferred_route" in j else None,
            j["hop_count"],
            Stability(j["stability"]),
            Sequencing(j["sequencing"]),
        )

    def to_json(self) -> dict:
        return self.__dict__


class SafetySelection:
    kind: SafetySelectionKind

    def __init__(self, kind: SafetySelectionKind, **kwargs):
        self.kind = kind
        for k, v in kwargs.items():
            setattr(self, k, v)

    @classmethod
    def unsafe(cls, sequencing: Sequencing = Sequencing.PREFER_ORDERED) -> Self:
        return cls(SafetySelectionKind.UNSAFE, sequencing=sequencing)

    @classmethod
    def safe(cls, safety_spec: SafetySpec) -> Self:
        return cls(SafetySelectionKind.SAFE, safety_spec=safety_spec)

    @classmethod
    def from_json(cls, j: dict) -> Self:
        if "Safe" in j:
            return cls.safe(SafetySpec.from_json(j["Safe"]))
        elif "Unsafe" in j:
            return cls.unsafe(Sequencing(j["Unsafe"]))
        raise Exception("Invalid SafetySelection")

    def to_json(self) -> dict:
        if self.kind == SafetySelectionKind.UNSAFE:
            return {"Unsafe": self.sequencing}
        elif self.kind == SafetySelectionKind.SAFE:
            return {"Safe": self.safety_spec.to_json()}
        else:
            raise Exception("Invalid SafetySelection")
