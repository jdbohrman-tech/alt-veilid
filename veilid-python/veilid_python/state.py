from typing import Self, Optional
from enum import StrEnum
from .types import Timestamp, TimestampDuration, ByteCount
from .config import VeilidConfig

class AttachmentState(StrEnum):
    DETACHED = 'Detached'
    ATTACHING = 'Attaching'
    ATTACHED_WEAK = 'AttachedWeak'
    ATTACHED_GOOD = 'AttachedGood'
    ATTACHED_STRONG = 'AttachedStrong'
    FULLY_ATTACHED = 'FullyAttached'
    OVER_ATTACHED = 'OverAttached'
    DETACHING = 'Detaching'

class VeilidStateAttachment:
    state: AttachmentState
    public_internet_ready: bool
    local_network_ready: bool
    
    def __init__(self, state: AttachmentState, public_internet_ready: bool, local_network_ready: bool):
        self.state = state
        self.public_internet_ready = public_internet_ready
        self.local_network_ready = local_network_ready

    @staticmethod
    def from_json(j: dict) -> Self:
        return VeilidStateAttachment(
            AttachmentState(j['state']),
            j['public_internet_ready'],
            j['local_network_ready'])

class RPCStats:
    messages_sent: int
    messages_rcvd: int
    questions_in_flight: int
    last_question_ts: Optional[Timestamp]
    last_seen_ts: Optional[Timestamp]
    first_consecutive_seen_ts: Optional[Timestamp]
    recent_lost_answers: int
    failed_to_send: int

    def __init__(self, messages_sent: int, messages_rcvd: int, questions_in_flight: int, 
                last_question_ts: Optional[Timestamp], last_seen_ts: Optional[Timestamp],
                first_consecutive_seen_ts: Optional[Timestamp], recent_lost_answers: int, failed_to_send: int):
        self.messages_sent = messages_sent
        self.messages_rcvd = messages_rcvd
        self.questions_in_flight = questions_in_flight
        self.last_question_ts = last_question_ts
        self.last_seen_ts = last_seen_ts
        self.first_consecutive_seen_ts = first_consecutive_seen_ts
        self.recent_lost_answers = recent_lost_answers
        self.failed_to_send = failed_to_send
    
    @staticmethod
    def from_json(j: dict) -> Self:
        return RPCStats(
            j['messages_sent'],
            j['messages_rcvd'],
            j['questions_in_flight'],
            None if j['last_question_ts'] is None else Timestamp(j['last_question_ts']),
            None if j['last_seen_ts'] is None else Timestamp(j['last_seen_ts']),
            None if j['first_consecutive_seen_ts'] is None else Timestamp(j['first_consecutive_seen_ts']),
            j['recent_lost_answers'],
            j['failed_to_send'])

class LatencyStats:
    fastest: TimestampDuration
    average: TimestampDuration
    slowest: TimestampDuration

    def __init__(self, fastest: TimestampDuration, average: TimestampDuration, slowest: TimestampDuration):
        self.fastest = fastest
        self.average = average
        self.slowest = slowest
    
    @staticmethod
    def from_json(j: dict) -> Self:
        return LatencyStats(
            TimestampDuration(j['fastest']),
            TimestampDuration(j['average']),
            TimestampDuration(j['slowest']))


class TransferStats:
    total: ByteCount
    maximum: ByteCount
    average: ByteCount
    minimum: ByteCount

    def __init__(self, total: ByteCount, maximum: ByteCount, average: ByteCount, minimum: ByteCount):
        self.total = total
        self.maximum = maximum
        self.average = average
        self.minimum = minimum
    
    @staticmethod
    def from_json(j: dict) -> Self:
        return TransferStats(
            ByteCount(j['total']),
            ByteCount(j['maximum']),
            ByteCount(j['average']),
            ByteCount(j['minimum']))


class TransferStatsDownUp:
    down: TransferStats
    up: TransferStats

    def __init__(self, down: TransferStats, up: TransferStats):
        self.down = down
        self.up = up

    @staticmethod
    def from_json(j: dict) -> Self:
        return TransferStatsDownUp(
            TransferStats.from_json(j['down']),
            TransferStats.from_json(j['up']))

class PeerStats:
    time_added: Timestamp
    rpc_stats: RPCStats
    latency: Optional[LatencyStats]
    transfer: TransferStatsDownUp

    def __init__(self, time_added: Timestamp, rpc_stats: RPCStats, latency: Optional[LatencyStats], transfer: TransferStatsDownUp):
        self.time_added = time_added
        self.rpc_stats = rpc_stats
        self.latency = latency 
        self.transfer = transfer

    @staticmethod
    def from_json(j: dict) -> Self:
        return PeerStats(
            j['time_added'],
            RPCStats.from_json(j['rpc_stats']),
            None if j['latency'] is None else LatencyStats.from_json(j['latency']),
            TransferStatsDownUp.from_json(j['transfer']))

class PeerTableData:
    node_ids: list[str]
    peer_address: str
    peer_stats: PeerStats

    def __init__(self, node_ids: list[str], peer_address: str, peer_stats: PeerStats):
        self.node_ids = node_ids
        self.peer_address = peer_address
        self.peer_stats = peer_stats
    
    @staticmethod
    def from_json(j: dict) -> Self:
        return PeerTableData(
            j['node_ids'],
            j['peer_address'],
            PeerStats.from_json(j['peer_stats']))

class VeilidStateNetwork:
    started: bool
    bps_down: ByteCount
    bps_up: ByteCount
    peers: list[PeerTableData]

    def __init__(self, started: bool, bps_down: ByteCount, bps_up: ByteCount, peers: list[PeerTableData]):
        self.started = started
        self.bps_down = bps_down
        self.bps_up = bps_up
        self.peers = peers
    
    @staticmethod
    def from_json(j: dict) -> Self:
        return VeilidStateNetwork(
            j['started'],
            ByteCount(j['bps_down']),
            ByteCount(j['bps_up']),
            map(lambda x: PeerTableData.from_json(x), j['peers']))

class VeilidStateConfig:
    config: VeilidConfig

    def __init__(self, config: VeilidConfig):
        self.config = config
    
    @staticmethod
    def from_json(j: dict) -> Self:
        return VeilidStateConfig(
            j['config'])


class VeilidState:
    attachment: VeilidStateAttachment
    network: VeilidStateNetwork
    config: VeilidStateConfig
    
    def __init__(self, attachment: VeilidStateAttachment, network: VeilidStateNetwork, config: VeilidStateConfig):
        self.attachment = attachment
        self.network = network
        self.config = config
    
    @staticmethod
    def from_json(j: dict) -> Self:
        return VeilidState(
            VeilidStateAttachment.from_json(j['attachment']), 
            VeilidStateNetwork.from_json(j['network']), 
            VeilidStateConfig.from_json(j['config']))
        
