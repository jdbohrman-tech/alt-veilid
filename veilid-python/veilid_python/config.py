from typing import Self, Optional
from enum import StrEnum

class VeilidConfigLogLevel(StrEnum):
    OFF = 'Off'
    ERROR = 'Error'
    WARN = 'Warn'
    INFO = 'Info'
    DEBUG = 'Debug'
    TRACE = 'Trace'

class VeilidConfigCapabilities:
    protocol_udp: bool
    protocol_connect_tcp: bool
    protocol_accept_tcp: bool
    protocol_connect_ws: bool
    protocol_accept_ws: bool
    protocol_connect_wss: bool
    protocol_accept_wss: bool

    def __init__(self, protocol_udp: bool, protocol_connect_tcp: bool, protocol_accept_tcp: bool,
        protocol_connect_ws: bool, protocol_accept_ws: bool, protocol_connect_wss: bool, protocol_accept_wss: bool):

        self.protocol_udp = protocol_udp
        self.protocol_connect_tcp = protocol_connect_tcp
        self.protocol_accept_tcp = protocol_accept_tcp
        self.protocol_connect_ws = protocol_connect_ws
        self.protocol_accept_ws = protocol_accept_ws
        self.protocol_connect_wss = protocol_connect_wss
        self.protocol_accept_wss = protocol_accept_wss

    @staticmethod
    def from_json(j: dict) -> Self:
        return VeilidConfigCapabilities(j['protocol_udp'],
            j['protocol_connect_tcp'],
            j['protocol_accept_tcp'],
            j['protocol_connect_ws'],
            j['protocol_accept_ws'],
            j['protocol_connect_wss'],
            j['protocol_accept_wss'])

class VeilidConfigProtectedStore:
    allow_insecure_fallback: bool
    always_use_insecure_storage: bool
    directory: str
    delete: bool
    device_encryption_key_password: str
    new_device_encryption_key_password: Optional[str]

    def __init__(self, allow_insecure_fallback: bool, always_use_insecure_storage: bool,
        directory: str, delete: bool, device_encryption_key_password: str, new_device_encryption_key_password: Optional[str]):

        self.allow_insecure_fallback = allow_insecure_fallback
        self.always_use_insecure_storage = always_use_insecure_storage
        self.directory = directory
        self.delete = delete
        self.device_encryption_key_password = device_encryption_key_password
        self.new_device_encryption_key_password = new_device_encryption_key_password
    
    @staticmethod
    def from_json(j: dict) -> Self:
        return VeilidConfigProtectedStore(j['allow_insecure_fallback'], j['always_use_insecure_storage'],
            j['directory'], j['delete'], j['device_encryption_key_password'], j['new_device_encryption_key_password'])

class VeilidConfigTableStore:
    directory: str
    delete: bool

    def __init__(self, directory: str, delete: bool):
        self.directory = directory
        self.delete = delete
    
    @staticmethod
    def from_json(j: dict) -> Self:
        return VeilidConfigTableStore(j['directory'], j['delete'])

class VeilidConfigBlockStore:
    directory: str
    delete: bool

    def __init__(self, directory: str, delete: bool):
        self.directory = directory
        self.delete = delete
    
    @staticmethod
    def from_json(j: dict) -> Self:
        return VeilidConfigBlockStore(j['directory'], j['delete'])

class VeilidConfigRoutingTable:
    node_id: list[str]
    node_id_secret: list[str]
    bootstrap: list[str]
    limit_over_attached: int
    limit_fully_attached: int
    limit_attached_strong: int
    limit_attached_good: int
    limit_attached_weak: int

    def __init__(self, node_id: list[str], node_id_secret: list[str], bootstrap: list[str], limit_over_attached: int,
        limit_fully_attached: int, limit_attached_strong: int, limit_attached_good: int, limit_attached_weak: int):

        self.node_id = node_id
        self.node_id_secret = node_id_secret
        self.bootstrap = bootstrap
        self.limit_over_attached = limit_over_attached
        self.limit_fully_attached = limit_fully_attached
        self.limit_attached_strong = limit_attached_strong
        self.limit_attached_good = limit_attached_good
        self.limit_attached_weak = limit_attached_weak
    
    @staticmethod
    def from_json(j: dict) -> Self:
        return VeilidConfigRoutingTable(
            j['node_id'],
            j['node_id_secret'],
            j['bootstrap'],
            j['limit_over_attached'],
            j['limit_fully_attached'],
            j['limit_attached_strong'],
            j['limit_attached_good'],
            j['limit_attached_weak'])


class VeilidConfigRPC:
    concurrency: int
    queue_size: int
    max_timestamp_behind_ms: Optional[int]
    max_timestamp_ahead_ms: Optional[int]
    timeout_ms: int
    max_route_hop_count: int
    default_route_hop_count: int

    def __init__(self, concurrency: int, queue_size: int, max_timestamp_behind_ms: Optional[int], max_timestamp_ahead_ms: Optional[int],
        timeout_ms: int, max_route_hop_count: int, default_route_hop_count: int):

        self.concurrency = concurrency
        self.queue_size = queue_size
        self.max_timestamp_behind_ms = max_timestamp_behind_ms
        self.max_timestamp_ahead_ms = max_timestamp_ahead_ms
        self.timeout_ms = timeout_ms
        self.max_route_hop_count = max_route_hop_count
        self.default_route_hop_count = default_route_hop_count
    
    @staticmethod
    def from_json(j: dict) -> Self:
        return VeilidConfigRPC(
            j['concurrency'],
            j['queue_size'],
            j['max_timestamp_behind_ms'],
            j['max_timestamp_ahead_ms'],
            j['timeout_ms'],
            j['max_route_hop_count'],
            j['default_route_hop_count'])


class VeilidConfigNetwork:
    connection_initial_timeout_ms: int
    connection_inactivity_timeout_ms: int
    max_connections_per_ip4: int
    max_connections_per_ip6_prefix: int
    max_connections_per_ip6_prefix_size: int
    max_connection_frequency_per_min: int
    client_whitelist_timeout_ms: int
    reverse_connection_receipt_time_ms: int
    hole_punch_receipt_time_ms: int
    routing_table: VeilidConfigRoutingTable
    rpc: VeilidConfigRPC
    dht: VeilidConfigDHT
    upnp: bool
    detect_address_changes: bool
    restricted_nat_retries: int
    tls: VeilidConfigTLS
    application: VeilidConfigApplication
    protocol: VeilidConfigProtocol

    def __init__(self, connection_initial_timeout_ms: int, connection_inactivity_timeout_ms: int,
        max_connections_per_ip4: int, max_connections_per_ip6_prefix: int,
        max_connections_per_ip6_prefix_size: int, max_connection_frequency_per_min: int,
        client_whitelist_timeout_ms: int, reverse_connection_receipt_time_ms: int,
        hole_punch_receipt_time_ms: int, routing_table: VeilidConfigRoutingTable,
        rpc: VeilidConfigRPC, dht: VeilidConfigDHT, upnp: bool, detect_address_changes: bool,
        restricted_nat_retries: int, tls: VeilidConfigTLS, application: VeilidConfigApplication, protocol: VeilidConfigProtocol):

        self.connection_initial_timeout_ms = connection_initial_timeout_ms
        self.connection_inactivity_timeout_ms = connection_inactivity_timeout_ms
        self.max_connections_per_ip4 = max_connections_per_ip4
        self.max_connections_per_ip6_prefix = max_connections_per_ip6_prefix
        self.max_connections_per_ip6_prefix_size = max_connections_per_ip6_prefix_size
        self.max_connection_frequency_per_min = max_connection_frequency_per_min
        self.client_whitelist_timeout_ms = client_whitelist_timeout_ms
        self.reverse_connection_receipt_time_ms = reverse_connection_receipt_time_ms
        self.hole_punch_receipt_time_ms = hole_punch_receipt_time_ms
        self.routing_table = routing_table
        self.rpc = rpc
        self.dht = dht
        self.upnp = upnp
        self.detect_address_changes = detect_address_changes
        self.restricted_nat_retries = restricted_nat_retries
        self.tls = tls
        self.application = application
        self.protocol = protocol
    
    @staticmethod
    def from_json(j: dict) -> Self:
        return VeilidConfigNetwork(
            j['connection_initial_timeout_ms'],
            j['connection_inactivity_timeout_ms'],
            j['max_connections_per_ip4'],
            j['max_connections_per_ip6_prefix'],
            j['max_connections_per_ip6_prefix_size'],
            j['max_connection_frequency_per_min'],
            j['client_whitelist_timeout_ms'],
            j['reverse_connection_receipt_time_ms'],
            j['hole_punch_receipt_time_ms'],
            VeilidConfigRoutingTable.from_json(j['routing_table']),
            VeilidConfigRPC.from_json(j['rpc']),
            VeilidConfigDHT.from_json(j['dht']),
            j['upnp'],
            j['detect_address_changes'],
            j['restricted_nat_retries'],
            VeilidConfigTLS.from_json(j['tls']),
            VeilidConfigApplication.from_json(j['application']),
            VeilidConfigProtocol.from_json(j['protocol']))

class VeilidConfig:
    program_name: str
    namespace: str
    capabilities: VeilidConfigCapabilities
    protected_store: VeilidConfigProtectedStore
    table_store: VeilidConfigTableStore
    block_store: VeilidConfigBlockStore
    network: VeilidConfigNetwork
    
    def __init__(self, program_name: str, namespace: str, capabilities: VeilidConfigCapabilities,
        protected_store: VeilidConfigProtectedStore, table_store: VeilidConfigTableStore,
        block_store: VeilidConfigBlockStore, network: VeilidConfigNetwork):

        self.program_name = program_name
        self.namespace = namespace
        self.capabilities = capabilities
        self.protected_store = protected_store
        self.table_store = table_store
        self.block_store = block_store
        self.network = network
    
    @staticmethod
    def from_json(j: dict) -> Self:
        return VeilidConfig(j['program_name'], j['namespace'],
            VeilidConfigCapabilities.from_json(j['capabilities']), 
            VeilidConfigProtectedStore.from_json(j['protected_store']), 
            VeilidConfigTableStore.from_json(j['table_store']),
            VeilidConfigBlockStore.from_json(j['block_store']), 
            VeilidConfigNetwork.from_json(j['network']))
        
