from typing import Self, Optional
from enum import StrEnum
from json import dumps

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
    def to_json(self) -> dict:
        return self.__dict__

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
    def to_json(self) -> dict:
        return self.__dict__

class VeilidConfigTableStore:
    directory: str
    delete: bool

    def __init__(self, directory: str, delete: bool):
        self.directory = directory
        self.delete = delete
    
    @staticmethod
    def from_json(j: dict) -> Self:
        return VeilidConfigTableStore(j['directory'], j['delete'])
    def to_json(self) -> dict:
        return self.__dict__

class VeilidConfigBlockStore:
    directory: str
    delete: bool

    def __init__(self, directory: str, delete: bool):
        self.directory = directory
        self.delete = delete
    
    @staticmethod
    def from_json(j: dict) -> Self:
        return VeilidConfigBlockStore(j['directory'], j['delete'])
    def to_json(self) -> dict:
        return self.__dict__

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
    def to_json(self) -> dict:
        return self.__dict__


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
    def to_json(self) -> dict:
        return self.__dict__

class VeilidConfigDHT:
    max_find_node_count: int
    resolve_node_timeout_ms: int
    resolve_node_count: int
    resolve_node_fanout: int
    get_value_timeout_ms: int
    get_value_count: int
    get_value_fanout: int
    set_value_timeout_ms: int
    set_value_count: int
    set_value_fanout: int
    min_peer_count: int
    min_peer_refresh_time_ms: int
    validate_dial_info_receipt_time_ms: int
    local_subkey_cache_size: int
    local_max_subkey_cache_memory_mb: int
    remote_subkey_cache_size: int
    remote_max_records: int
    remote_max_subkey_cache_memory_mb: int
    remote_max_storage_space_mb: int

    def __init__(self,  max_find_node_count: int, resolve_node_timeout_ms: int, resolve_node_count: int,
        resolve_node_fanout: int, get_value_timeout_ms: int, get_value_count: int, get_value_fanout: int,
        set_value_timeout_ms: int, set_value_count: int, set_value_fanout: int,
        min_peer_count: int, min_peer_refresh_time_ms: int, validate_dial_info_receipt_time_ms: int,
        local_subkey_cache_size: int, local_max_subkey_cache_memory_mb: int,
        remote_subkey_cache_size: int, remote_max_records: int, remote_max_subkey_cache_memory_mb: int, remote_max_storage_space_mb: int):

        self.max_find_node_count = max_find_node_count
        self.resolve_node_timeout_ms =resolve_node_timeout_ms
        self.resolve_node_count = resolve_node_count
        self.resolve_node_fanout = resolve_node_fanout
        self.get_value_timeout_ms = get_value_timeout_ms
        self.get_value_count = get_value_count
        self.get_value_fanout = get_value_fanout
        self.set_value_timeout_ms = set_value_timeout_ms
        self.set_value_count = set_value_count
        self.set_value_fanout = set_value_fanout
        self.min_peer_count = min_peer_count
        self.min_peer_refresh_time_ms = min_peer_refresh_time_ms
        self.validate_dial_info_receipt_time_ms = validate_dial_info_receipt_time_ms
        self.local_subkey_cache_size = local_subkey_cache_size
        self.local_max_subkey_cache_memory_mb = local_max_subkey_cache_memory_mb
        self.remote_subkey_cache_size = remote_subkey_cache_size
        self.remote_max_records = remote_max_records
        self.remote_max_subkey_cache_memory_mb = remote_max_subkey_cache_memory_mb
        self.remote_max_storage_space_mb = remote_max_storage_space_mb

    @staticmethod
    def from_json(j: dict) -> Self:
        return VeilidConfigDHT(
            j['max_find_node_count'],
            j['resolve_node_timeout_ms'],
            j['resolve_node_count'],
            j['resolve_node_fanout'],
            j['get_value_timeout_ms'],
            j['get_value_count'],
            j['get_value_fanout'],
            j['set_value_timeout_ms'],
            j['set_value_count'],
            j['set_value_fanout'],
            j['min_peer_count'],
            j['min_peer_refresh_time_ms'],
            j['validate_dial_info_receipt_time_ms'],
            j['local_subkey_cache_size'],
            j['local_max_subkey_cache_memory_mb'],
            j['remote_subkey_cache_size'],
            j['remote_max_records'],
            j['remote_max_subkey_cache_memory_mb'],
            j['remote_max_storage_space_mb'])
    def to_json(self) -> dict:
        return self.__dict__

class VeilidConfigTLS:
    certificate_path: str
    private_key_path: str
    connection_initial_timeout_ms: int

    def __init__(self, certificate_path: str, private_key_path: str, connection_initial_timeout_ms: int):
        self.certificate_path = certificate_path
        self.private_key_path = private_key_path
        self.connection_initial_timeout_ms = connection_initial_timeout_ms
    
    @staticmethod
    def from_json(j: dict) -> Self:
        return VeilidConfigTLS(
            j['certificate_path'],
            j['private_key_path'],
            j['connection_initial_timeout_ms'])
    def to_json(self) -> dict:
        return self.__dict__

class VeilidConfigHTTPS:
    enabled: bool
    listen_address: str
    path: str
    url: Optional[str]
    
    def __init__(self, enabled: bool, listen_address: str, path: str, url: Optional[str]):
        self.enabled = enabled
        self.listen_address = listen_address
        self.path = path
        self.url = url

    @staticmethod
    def from_json(j: dict) -> Self:
        return VeilidConfigHTTPS(
            j['enabled'],
            j['listen_address'],
            j['path'],
            j['url'])
    def to_json(self) -> dict:
        return self.__dict__

class VeilidConfigHTTP:
    enabled: bool
    listen_address: str
    path: str
    url: Optional[str]
    
    def __init__(self, enabled: bool, listen_address: str, path: str, url: Optional[str]):
        self.enabled = enabled
        self.listen_address = listen_address
        self.path = path
        self.url = url

    @staticmethod
    def from_json(j: dict) -> Self:
        return VeilidConfigHTTP(
            j['enabled'],
            j['listen_address'],
            j['path'],
            j['url'])
    def to_json(self) -> dict:
        return self.__dict__

class VeilidConfigApplication:
    https: VeilidConfigHTTPS
    http: VeilidConfigHTTP

    def __init__(self, https: VeilidConfigHTTPS, http: VeilidConfigHTTP):
        self.https = https
        self.http = http

    @staticmethod
    def from_json(j: dict) -> Self:
        return VeilidConfigApplication(
            VeilidConfigHTTPS.from_json(j['https']),
            VeilidConfigHTTP.from_json(j['http']))
    def to_json(self) -> dict:
        return self.__dict__


class VeilidConfigUDP:
    enabled: bool
    socket_pool_size: int
    listen_address: str
    public_address: Optional[str]
    
    def __init__(self, enabled: bool, socket_pool_size: int, listen_address: str, public_address: Optional[str]):
        self.enabled = enabled
        self.socket_pool_size = socket_pool_size
        self.listen_address = listen_address
        self.public_address = public_address

    @staticmethod
    def from_json(j: dict) -> Self:
        return VeilidConfigUDP(
            j['enabled'], 
            j['socket_pool_size'], 
            j['listen_address'], 
            j['public_address'])
    def to_json(self) -> dict:
        return self.__dict__

class VeilidConfigTCP:
    connect: bool
    listen: bool
    max_connections: int
    listen_address: str
    public_address: Optional[str]
    
    def __init__(self, connect: bool, listen: bool, max_connections: int, listen_address: str, public_address: Optional[str]):
        self.connect = connect
        self.listen = listen
        self.max_connections = max_connections
        self.listen_address = listen_address
        self.public_address = public_address

    @staticmethod
    def from_json(j: dict) -> Self:
        return VeilidConfigTCP(
            j['connect'], 
            j['listen'], 
            j['max_connections'], 
            j['listen_address'], 
            j['public_address'])
    def to_json(self) -> dict:
        return self.__dict__

class VeilidConfigWS:
    connect: bool
    listen: bool
    max_connections: int
    listen_address: str
    path: str
    url: Optional[str]
    
    def __init__(self, connect: bool, listen: bool, max_connections: int, listen_address: str, path: str, url: Optional[str]):
        self.connect = connect
        self.listen = listen
        self.max_connections = max_connections
        self.listen_address = listen_address
        self.path = path
        self.url = url

    @staticmethod
    def from_json(j: dict) -> Self:
        return VeilidConfigWS(
            j['connect'], 
            j['listen'], 
            j['max_connections'], 
            j['listen_address'], 
            j['path'],
            j['url'])
    def to_json(self) -> dict:
        return self.__dict__

class VeilidConfigWSS:
    connect: bool
    listen: bool
    max_connections: int
    listen_address: str
    path: str
    url: Optional[str]
    
    def __init__(self, connect: bool, listen: bool, max_connections: int, listen_address: str, path: str, url: Optional[str]):
        self.connect = connect
        self.listen = listen
        self.max_connections = max_connections
        self.listen_address = listen_address
        self.path = path
        self.url = url

    @staticmethod
    def from_json(j: dict) -> Self:
        return VeilidConfigWSS(
            j['connect'], 
            j['listen'], 
            j['max_connections'], 
            j['listen_address'], 
            j['path'],
            j['url'])
    def to_json(self) -> dict:
        return self.__dict__

class VeilidConfigProtocol:
    udp: VeilidConfigUDP
    tcp: VeilidConfigTCP
    ws: VeilidConfigWS
    wss: VeilidConfigWSS

    def __init__(self, udp: VeilidConfigUDP, tcp: VeilidConfigTCP, ws: VeilidConfigWS, wss: VeilidConfigWSS):
        self.udp = udp
        self.tcp = tcp
        self.ws = ws
        self.wss = wss

    @staticmethod
    def from_json(j: dict) -> Self:
        return VeilidConfigProtocol(
            VeilidConfigUDP.from_json(j['udp']),
            VeilidConfigTCP.from_json(j['tcp']),
            VeilidConfigWS.from_json(j['ws']),
            VeilidConfigWSS.from_json(j['wss']))
    def to_json(self) -> dict:
        return self.__dict__


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
    def to_json(self) -> dict:
        return self.__dict__

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
        '''JSON object hook'''
        return VeilidConfig(j['program_name'], j['namespace'],
            VeilidConfigCapabilities.from_json(j['capabilities']), 
            VeilidConfigProtectedStore.from_json(j['protected_store']), 
            VeilidConfigTableStore.from_json(j['table_store']),
            VeilidConfigBlockStore.from_json(j['block_store']), 
            VeilidConfigNetwork.from_json(j['network']))
    def to_json(self) -> dict:
        return self.__dict__
    
    