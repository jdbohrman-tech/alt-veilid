from enum import StrEnum
from typing import Optional, Self

from .config import VeilidConfig
from .types import (
    ByteCount,
    RouteId,
    Timestamp,
    TimestampDuration,
    TypedKey,
    ValueData,
    ValueSubkey,
    VeilidLogLevel,
    OperationId,
    urlsafe_b64decode_no_pad,
)


class AttachmentState(StrEnum):
    DETACHED = "Detached"
    ATTACHING = "Attaching"
    ATTACHED_WEAK = "AttachedWeak"
    ATTACHED_GOOD = "AttachedGood"
    ATTACHED_STRONG = "AttachedStrong"
    FULLY_ATTACHED = "FullyAttached"
    OVER_ATTACHED = "OverAttached"
    DETACHING = "Detaching"


class VeilidStateAttachment:
    state: AttachmentState
    public_internet_ready: bool
    local_network_ready: bool
    uptime: TimestampDuration
    attached_uptime: Optional[TimestampDuration]

    def __init__(
        self,
        state: AttachmentState,
        public_internet_ready: bool,
        local_network_ready: bool,
        uptime: TimestampDuration,
        attached_uptime: Optional[TimestampDuration],
    ):
        self.state = state
        self.public_internet_ready = public_internet_ready
        self.local_network_ready = local_network_ready
        self.uptime = uptime
        self.attached_uptime = attached_uptime

    @classmethod
    def from_json(cls, j: dict) -> Self:
        """JSON object hook"""
        return cls(
            AttachmentState(j["state"]),
            j["public_internet_ready"],
            j["local_network_ready"],
            j["uptime"],
            j["attached_uptime"],
        )

    def to_json(self) -> dict:
        return self.__dict__


class AnswerStats:
    span: TimestampDuration
    questions: int
    answers: int
    lost_answers: int
    consecutive_answers_maximum: int
    consecutive_answers_average: int
    consecutive_answers_minimum: int
    consecutive_lost_answers_maximum: int
    consecutive_lost_answers_average: int
    consecutive_lost_answers_minimum: int

    def __init__(
        self,
        span: TimestampDuration,
        questions: int,
        answers: int,
        lost_answers: int,
        consecutive_answers_maximum: int,
        consecutive_answers_average: int,
        consecutive_answers_minimum: int,
        consecutive_lost_answers_maximum: int,
        consecutive_lost_answers_average: int,
        consecutive_lost_answers_minimum: int,
    ):
        self.span = span
        self.questions = questions
        self.answers = answers
        self.lost_answers = lost_answers
        self.consecutive_answers_maximum = consecutive_answers_maximum
        self.consecutive_answers_average = consecutive_answers_average
        self.consecutive_answers_minimum = consecutive_answers_minimum
        self.consecutive_lost_answers_maximum = consecutive_lost_answers_maximum
        self.consecutive_lost_answers_average = consecutive_lost_answers_average
        self.consecutive_lost_answers_minimum = consecutive_lost_answers_minimum


    @classmethod
    def from_json(cls, j: dict) -> Self:
        """JSON object hook"""
        return cls(
            j["span"],
            j["questions"],
            j["answers"],
            j["lost_answers"],
            j["consecutive_answers_maximum"],
            j["consecutive_answers_average"],
            j["consecutive_answers_minimum"],
            j["consecutive_lost_answers_maximum"],
            j["consecutive_lost_answers_average"],
            j["consecutive_lost_answers_minimum"],
        )

    def to_json(self) -> dict:
        return self.__dict__

class RPCStats:
    messages_sent: int
    messages_rcvd: int
    questions_in_flight: int
    last_question_ts: Optional[Timestamp]
    last_seen_ts: Optional[Timestamp]
    first_consecutive_seen_ts: Optional[Timestamp]
    recent_lost_answers_unordered: int
    recent_lost_answers_ordered: int
    failed_to_send: int
    answer_unordered: AnswerStats
    answer_ordered: AnswerStats

    def __init__(
        self,
        messages_sent: int,
        messages_rcvd: int,
        questions_in_flight: int,
        last_question_ts: Optional[Timestamp],
        last_seen_ts: Optional[Timestamp],
        first_consecutive_seen_ts: Optional[Timestamp],
        recent_lost_answers_unordered: int,
        recent_lost_answers_ordered: int,
        failed_to_send: int,
        answer_unordered: AnswerStats,
        answer_ordered: AnswerStats,
    ):
        self.messages_sent = messages_sent
        self.messages_rcvd = messages_rcvd
        self.questions_in_flight = questions_in_flight
        self.last_question_ts = last_question_ts
        self.last_seen_ts = last_seen_ts
        self.first_consecutive_seen_ts = first_consecutive_seen_ts
        self.recent_lost_answers_unordered = recent_lost_answers_unordered
        self.recent_lost_answers_ordered = recent_lost_answers_ordered
        self.failed_to_send = failed_to_send
        self.answer_unordered = answer_unordered
        self.answer_ordered = answer_ordered

    @classmethod
    def from_json(cls, j: dict) -> Self:
        """JSON object hook"""
        return cls(
            j["messages_sent"],
            j["messages_rcvd"],
            j["questions_in_flight"],
            None if j["last_question_ts"] is None else Timestamp(j["last_question_ts"]),
            None if j["last_seen_ts"] is None else Timestamp(j["last_seen_ts"]),
            None
            if j["first_consecutive_seen_ts"] is None
            else Timestamp(j["first_consecutive_seen_ts"]),
            j["recent_lost_answers_unordered"],
            j["recent_lost_answers_ordered"],
            j["failed_to_send"],
            AnswerStats.from_json(j["answer_unordered"]),
            AnswerStats.from_json(j["answer_ordered"]),
        )

    def to_json(self) -> dict:
        return self.__dict__


class LatencyStats:
    fastest: TimestampDuration
    average: TimestampDuration
    slowest: TimestampDuration
    tm90: TimestampDuration
    tm75: TimestampDuration
    p90: TimestampDuration
    p75: TimestampDuration

    def __init__(
        self,
        fastest: TimestampDuration,
        average: TimestampDuration,
        slowest: TimestampDuration,
        tm90: TimestampDuration,
        tm75: TimestampDuration,
        p90: TimestampDuration,
        p75: TimestampDuration,
    ):
        self.fastest = fastest
        self.average = average
        self.slowest = slowest
        self.tm90 = tm90
        self.tm75 = tm75
        self.p90 = p90
        self.p75 = p75

    @classmethod
    def from_json(cls, j: dict) -> Self:
        """JSON object hook"""
        return cls(
            TimestampDuration(j["fastest"]),
            TimestampDuration(j["average"]),
            TimestampDuration(j["slowest"]),
            TimestampDuration(j["tm90"]),
            TimestampDuration(j["tm75"]),
            TimestampDuration(j["p90"]),
            TimestampDuration(j["p75"]),
        )

    def to_json(self) -> dict:
        return self.__dict__


class TransferStats:
    total: ByteCount
    maximum: ByteCount
    average: ByteCount
    minimum: ByteCount

    def __init__(
        self,
        total: ByteCount,
        maximum: ByteCount,
        average: ByteCount,
        minimum: ByteCount,
    ):
        self.total = total
        self.maximum = maximum
        self.average = average
        self.minimum = minimum

    @classmethod
    def from_json(cls, j: dict) -> Self:
        """JSON object hook"""
        return cls(
            ByteCount(j["total"]),
            ByteCount(j["maximum"]),
            ByteCount(j["average"]),
            ByteCount(j["minimum"]),
        )


class TransferStatsDownUp:
    down: TransferStats
    up: TransferStats

    def __init__(self, down: TransferStats, up: TransferStats):
        self.down = down
        self.up = up

    @classmethod
    def from_json(cls, j: dict) -> Self:
        """JSON object hook"""
        return cls(TransferStats.from_json(j["down"]), TransferStats.from_json(j["up"]))

class StateReasonStats:
    can_not_send: TimestampDuration
    too_many_lost_answers: TimestampDuration
    no_ping_response: TimestampDuration
    failed_to_send: TimestampDuration
    lost_answers: TimestampDuration
    not_seen_consecutively: TimestampDuration
    in_unreliable_ping_span: TimestampDuration

    def __init__(
        self,
        can_not_send: TimestampDuration,
        too_many_lost_answers: TimestampDuration,
        no_ping_response: TimestampDuration,
        failed_to_send: TimestampDuration,
        lost_answers: TimestampDuration,
        not_seen_consecutively: TimestampDuration,
        in_unreliable_ping_span: TimestampDuration,
    ):
        self.can_not_send = can_not_send
        self.too_many_lost_answers = too_many_lost_answers
        self.no_ping_response = no_ping_response
        self.failed_to_send = failed_to_send
        self.lost_answers = lost_answers
        self.not_seen_consecutively = not_seen_consecutively
        self.in_unreliable_ping_span = in_unreliable_ping_span

    @classmethod
    def from_json(cls, j: dict) -> Self:
        """JSON object hook"""
        return cls(
            j["can_not_send"],
            j["too_many_lost_answers"],
            j["no_ping_response"],
            j["failed_to_send"],
            j["lost_answers"],
            j["not_seen_consecutively"],
            j["in_unreliable_ping_span"],
        )

class StateStats:
    span: TimestampDuration
    reliable: TimestampDuration
    unreliable: TimestampDuration
    dead: TimestampDuration
    punished: TimestampDuration
    reason: StateReasonStats

    def __init__(
        self,
        span: TimestampDuration,
        reliable: TimestampDuration,
        unreliable: TimestampDuration,
        dead: TimestampDuration,
        punished: TimestampDuration,
        reason: StateReasonStats,
    ):
        self.span = span
        self.reliable = reliable
        self.unreliable = unreliable
        self.dead = dead
        self.punished = punished
        self.reason = reason

    @classmethod
    def from_json(cls, j: dict) -> Self:
        """JSON object hook"""
        return cls(
            j["span"],
            j["reliable"],
            j["unreliable"],
            j["dead"],
            j["punished"],
            StateReasonStats.from_json(j["reason"]),
        )


class PeerStats:
    time_added: Timestamp
    rpc_stats: RPCStats
    latency: Optional[LatencyStats]
    transfer: TransferStatsDownUp
    state: StateStats

    def __init__(
        self,
        time_added: Timestamp,
        rpc_stats: RPCStats,
        latency: Optional[LatencyStats],
        transfer: TransferStatsDownUp,
        state: StateStats,
    ):
        self.time_added = time_added
        self.rpc_stats = rpc_stats
        self.latency = latency
        self.transfer = transfer
        self.state = state

    @classmethod
    def from_json(cls, j: dict) -> Self:
        """JSON object hook"""
        return cls(
            j["time_added"],
            RPCStats.from_json(j["rpc_stats"]),
            None if j["latency"] is None else LatencyStats.from_json(j["latency"]),
            TransferStatsDownUp.from_json(j["transfer"]),
            StateStats.from_json(j["state"]),
        )

    def to_json(self) -> dict:
        return self.__dict__


class PeerTableData:
    node_ids: list[str]
    peer_address: str
    peer_stats: PeerStats

    def __init__(self, node_ids: list[str], peer_address: str, peer_stats: PeerStats):
        self.node_ids = node_ids
        self.peer_address = peer_address
        self.peer_stats = peer_stats

    @classmethod
    def from_json(cls, j: dict) -> Self:
        """JSON object hook"""
        return cls(j["node_ids"], j["peer_address"], PeerStats.from_json(j["peer_stats"]))

    def to_json(self) -> dict:
        return self.__dict__


class VeilidStateNetwork:
    started: bool
    bps_down: ByteCount
    bps_up: ByteCount
    peers: list[PeerTableData]

    def __init__(
        self,
        started: bool,
        bps_down: ByteCount,
        bps_up: ByteCount,
        peers: list[PeerTableData],
    ):
        self.started = started
        self.bps_down = bps_down
        self.bps_up = bps_up
        self.peers = peers

    @classmethod
    def from_json(cls, j: dict) -> Self:
        """JSON object hook"""
        return cls(
            j["started"],
            ByteCount(j["bps_down"]),
            ByteCount(j["bps_up"]),
            [PeerTableData.from_json(peer) for peer in j["peers"]],
        )

    def to_json(self) -> dict:
        return self.__dict__


class VeilidStateConfig:
    config: VeilidConfig

    def __init__(self, config: VeilidConfig):
        self.config = config

    @classmethod
    def from_json(cls, j: dict) -> Self:
        """JSON object hook"""
        return cls(VeilidConfig.from_json(j["config"]))

    def to_json(self) -> dict:
        return self.__dict__


class VeilidState:
    attachment: VeilidStateAttachment
    network: VeilidStateNetwork
    config: VeilidStateConfig

    def __init__(
        self,
        attachment: VeilidStateAttachment,
        network: VeilidStateNetwork,
        config: VeilidStateConfig,
    ):
        self.attachment = attachment
        self.network = network
        self.config = config

    @classmethod
    def from_json(cls, j: dict) -> Self:
        """JSON object hook"""
        return cls(
            VeilidStateAttachment.from_json(j["attachment"]),
            VeilidStateNetwork.from_json(j["network"]),
            VeilidStateConfig.from_json(j["config"]),
        )

    def to_json(self) -> dict:
        return self.__dict__


class VeilidLog:
    log_level: VeilidLogLevel
    message: str
    backtrace: Optional[str]

    def __init__(self, log_level: VeilidLogLevel, message: str, backtrace: Optional[str]):
        self.log_level = log_level
        self.message = message
        self.backtrace = backtrace

    @classmethod
    def from_json(cls, j: dict) -> Self:
        """JSON object hook"""
        return cls(VeilidLogLevel(j["log_level"]), j["message"], j["backtrace"])

    def to_json(self) -> dict:
        return self.__dict__


class VeilidAppMessage:
    sender: Optional[TypedKey]
    route_id: Optional[RouteId]
    message: bytes

    def __init__(self, sender: Optional[TypedKey], route_id: Optional[RouteId], message: bytes):
        self.sender = sender
        self.route_id = route_id
        self.message = message

    @classmethod
    def from_json(cls, j: dict) -> Self:
        """JSON object hook"""
        return cls(
            None if j["sender"] is None else TypedKey(j["sender"]),
            None if j["route_id"] is None else RouteId(j["route_id"]),
            urlsafe_b64decode_no_pad(j["message"]),
        )

    def to_json(self) -> dict:
        return self.__dict__


class VeilidAppCall:
    sender: Optional[TypedKey]
    route_id: Optional[RouteId]
    message: bytes
    call_id: OperationId

    def __init__(self, sender: Optional[TypedKey], route_id: Optional[TypedKey], message: bytes, call_id: OperationId):
        self.sender = sender
        self.route_id = route_id
        self.message = message
        self.call_id = call_id

    @classmethod
    def from_json(cls, j: dict) -> Self:
        """JSON object hook"""
        return cls(
            None if j["sender"] is None else TypedKey(j["sender"]),
            None if j["route_id"] is None else RouteId(j["route_id"]),
            urlsafe_b64decode_no_pad(j["message"]),
            OperationId(j["call_id"]),
        )

    def to_json(self) -> dict:
        return self.__dict__


class VeilidRouteChange:
    dead_routes: list[RouteId]
    dead_remote_routes: list[RouteId]

    def __init__(self, dead_routes: list[RouteId], dead_remote_routes: list[RouteId]):
        self.dead_routes = dead_routes
        self.dead_remote_routes = dead_remote_routes

    @classmethod
    def from_json(cls, j: dict) -> Self:
        """JSON object hook"""
        return cls(
            [RouteId(route) for route in j["dead_routes"]],
            [RouteId(route) for route in j["dead_remote_routes"]],
        )

    def to_json(self) -> dict:
        return self.__dict__


class VeilidValueChange:
    key: TypedKey
    subkeys: list[tuple[ValueSubkey, ValueSubkey]]
    count: int
    value: Optional[ValueData]

    def __init__(self, key: TypedKey, subkeys: list[tuple[ValueSubkey, ValueSubkey]], count: int, value: Optional[ValueData]):
        self.key = key
        self.subkeys = subkeys
        self.count = count
        self.value = value

    @classmethod
    def from_json(cls, j: dict) -> Self:
        """JSON object hook"""
        return cls(
            TypedKey(j["key"]),
            [(p[0], p[1]) for p in j["subkeys"]],
            j["count"],
            None if j["value"] is None else ValueData.from_json(j["value"]),
        )

    def to_json(self) -> dict:
        return self.__dict__


class VeilidUpdateKind(StrEnum):
    LOG = "Log"
    APP_MESSAGE = "AppMessage"
    APP_CALL = "AppCall"
    ATTACHMENT = "Attachment"
    NETWORK = "Network"
    CONFIG = "Config"
    ROUTE_CHANGE = "RouteChange"
    VALUE_CHANGE = "ValueChange"
    SHUTDOWN = "Shutdown"


VeilidUpdateDetailType = Optional[
    VeilidLog
    | VeilidAppMessage
    | VeilidAppCall
    | VeilidStateAttachment
    | VeilidStateNetwork
    | VeilidStateConfig
    | VeilidRouteChange
    | VeilidValueChange
]


class VeilidUpdate:
    kind: VeilidUpdateKind
    detail: VeilidUpdateDetailType

    def __init__(
        self,
        kind: VeilidUpdateKind,
        detail: VeilidUpdateDetailType,
    ):
        self.kind = kind
        self.detail = detail

    @classmethod
    def from_json(cls, j: dict) -> Self:
        """JSON object hook"""
        kind = VeilidUpdateKind(j["kind"])
        detail: VeilidUpdateDetailType = None
        match kind:
            case VeilidUpdateKind.LOG:
                detail = VeilidLog.from_json(j)
            case VeilidUpdateKind.APP_MESSAGE:
                detail = VeilidAppMessage.from_json(j)
            case VeilidUpdateKind.APP_CALL:
                detail = VeilidAppCall.from_json(j)
            case VeilidUpdateKind.ATTACHMENT:
                detail = VeilidStateAttachment.from_json(j)
            case VeilidUpdateKind.NETWORK:
                detail = VeilidStateNetwork.from_json(j)
            case VeilidUpdateKind.CONFIG:
                detail = VeilidStateConfig.from_json(j)
            case VeilidUpdateKind.ROUTE_CHANGE:
                detail = VeilidRouteChange.from_json(j)
            case VeilidUpdateKind.VALUE_CHANGE:
                detail = VeilidValueChange.from_json(j)
            case VeilidUpdateKind.SHUTDOWN:
                detail = None
            case _:
                raise ValueError("Unknown VeilidUpdateKind")
        return cls(kind, detail)

    def to_json(self) -> dict:
        return self.__dict__
