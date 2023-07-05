import 'package:change_case/change_case.dart';

import 'veilid.dart';

//////////////////////////////////////////////////////////
// FFI Platform-specific config

class VeilidFFIConfigLoggingTerminal {
  bool enabled;
  VeilidConfigLogLevel level;

  VeilidFFIConfigLoggingTerminal({
    required this.enabled,
    required this.level,
  });

  Map<String, dynamic> toJson() {
    return {
      'enabled': enabled,
      'level': level.toJson(),
    };
  }

  VeilidFFIConfigLoggingTerminal.fromJson(dynamic json)
      : enabled = json['enabled'],
        level = VeilidConfigLogLevel.fromJson(json['level']);
}

class VeilidFFIConfigLoggingOtlp {
  bool enabled;
  VeilidConfigLogLevel level;
  String grpcEndpoint;
  String serviceName;

  VeilidFFIConfigLoggingOtlp({
    required this.enabled,
    required this.level,
    required this.grpcEndpoint,
    required this.serviceName,
  });

  Map<String, dynamic> toJson() {
    return {
      'enabled': enabled,
      'level': level.toJson(),
      'grpc_endpoint': grpcEndpoint,
      'service_name': serviceName,
    };
  }

  VeilidFFIConfigLoggingOtlp.fromJson(dynamic json)
      : enabled = json['enabled'],
        level = VeilidConfigLogLevel.fromJson(json['level']),
        grpcEndpoint = json['grpc_endpoint'],
        serviceName = json['service_name'];
}

class VeilidFFIConfigLoggingApi {
  bool enabled;
  VeilidConfigLogLevel level;

  VeilidFFIConfigLoggingApi({
    required this.enabled,
    required this.level,
  });

  Map<String, dynamic> toJson() {
    return {
      'enabled': enabled,
      'level': level.toJson(),
    };
  }

  VeilidFFIConfigLoggingApi.fromJson(dynamic json)
      : enabled = json['enabled'],
        level = VeilidConfigLogLevel.fromJson(json['level']);
}

class VeilidFFIConfigLogging {
  VeilidFFIConfigLoggingTerminal terminal;
  VeilidFFIConfigLoggingOtlp otlp;
  VeilidFFIConfigLoggingApi api;

  VeilidFFIConfigLogging(
      {required this.terminal, required this.otlp, required this.api});

  Map<String, dynamic> toJson() {
    return {
      'terminal': terminal.toJson(),
      'otlp': otlp.toJson(),
      'api': api.toJson(),
    };
  }

  VeilidFFIConfigLogging.fromJson(dynamic json)
      : terminal = VeilidFFIConfigLoggingTerminal.fromJson(json['terminal']),
        otlp = VeilidFFIConfigLoggingOtlp.fromJson(json['otlp']),
        api = VeilidFFIConfigLoggingApi.fromJson(json['api']);
}

class VeilidFFIConfig {
  VeilidFFIConfigLogging logging;

  VeilidFFIConfig({
    required this.logging,
  });

  Map<String, dynamic> toJson() {
    return {
      'logging': logging.toJson(),
    };
  }

  VeilidFFIConfig.fromJson(Map<String, dynamic> json)
      : logging = VeilidFFIConfigLogging.fromJson(json['logging']);
}

//////////////////////////////////////
/// VeilidConfigLogLevel

enum VeilidConfigLogLevel {
  off,
  error,
  warn,
  info,
  debug,
  trace;

  String toJson() {
    return name.toPascalCase();
  }

  factory VeilidConfigLogLevel.fromJson(dynamic j) {
    return VeilidConfigLogLevel.values.byName((j as String).toCamelCase());
  }
}

//////////////////////////////////////////////////////////
// WASM Platform-specific config

class VeilidWASMConfigLoggingPerformance {
  bool enabled;
  VeilidConfigLogLevel level;
  bool logsInTimings;
  bool logsInConsole;

  VeilidWASMConfigLoggingPerformance({
    required this.enabled,
    required this.level,
    required this.logsInTimings,
    required this.logsInConsole,
  });

  Map<String, dynamic> toJson() {
    return {
      'enabled': enabled,
      'level': level.toJson(),
      'logs_in_timings': logsInTimings,
      'logs_in_console': logsInConsole,
    };
  }

  VeilidWASMConfigLoggingPerformance.fromJson(dynamic json)
      : enabled = json['enabled'],
        level = VeilidConfigLogLevel.fromJson(json['level']),
        logsInTimings = json['logs_in_timings'],
        logsInConsole = json['logs_in_console'];
}

class VeilidWASMConfigLoggingApi {
  bool enabled;
  VeilidConfigLogLevel level;

  VeilidWASMConfigLoggingApi({
    required this.enabled,
    required this.level,
  });

  Map<String, dynamic> toJson() {
    return {
      'enabled': enabled,
      'level': level.toJson(),
    };
  }

  VeilidWASMConfigLoggingApi.fromJson(dynamic json)
      : enabled = json['enabled'],
        level = VeilidConfigLogLevel.fromJson(json['level']);
}

class VeilidWASMConfigLogging {
  VeilidWASMConfigLoggingPerformance performance;
  VeilidWASMConfigLoggingApi api;

  VeilidWASMConfigLogging({required this.performance, required this.api});

  Map<String, dynamic> toJson() {
    return {
      'performance': performance.toJson(),
      'api': api.toJson(),
    };
  }

  VeilidWASMConfigLogging.fromJson(dynamic json)
      : performance =
            VeilidWASMConfigLoggingPerformance.fromJson(json['performance']),
        api = VeilidWASMConfigLoggingApi.fromJson(json['api']);
}

class VeilidWASMConfig {
  VeilidWASMConfigLogging logging;

  VeilidWASMConfig({
    required this.logging,
  });

  Map<String, dynamic> toJson() {
    return {
      'logging': logging.toJson(),
    };
  }

  VeilidWASMConfig.fromJson(dynamic json)
      : logging = VeilidWASMConfigLogging.fromJson(json['logging']);
}

//////////////////////////////////////
/// VeilidConfig

class VeilidConfigHTTPS {
  bool enabled;
  String listenAddress;
  String path;
  String? url;

  VeilidConfigHTTPS({
    required this.enabled,
    required this.listenAddress,
    required this.path,
    this.url,
  });

  Map<String, dynamic> toJson() {
    return {
      'enabled': enabled,
      'listen_address': listenAddress,
      'path': path,
      'url': url
    };
  }

  VeilidConfigHTTPS.fromJson(dynamic json)
      : enabled = json['enabled'],
        listenAddress = json['listen_address'],
        path = json['path'],
        url = json['url'];
}

////////////

class VeilidConfigHTTP {
  bool enabled;
  String listenAddress;
  String path;
  String? url;

  VeilidConfigHTTP({
    required this.enabled,
    required this.listenAddress,
    required this.path,
    this.url,
  });

  Map<String, dynamic> toJson() {
    return {
      'enabled': enabled,
      'listen_address': listenAddress,
      'path': path,
      'url': url
    };
  }

  VeilidConfigHTTP.fromJson(dynamic json)
      : enabled = json['enabled'],
        listenAddress = json['listen_address'],
        path = json['path'],
        url = json['url'];
}

////////////

class VeilidConfigApplication {
  VeilidConfigHTTPS https;
  VeilidConfigHTTP http;

  VeilidConfigApplication({
    required this.https,
    required this.http,
  });

  Map<String, dynamic> toJson() {
    return {
      'https': https.toJson(),
      'http': http.toJson(),
    };
  }

  VeilidConfigApplication.fromJson(dynamic json)
      : https = VeilidConfigHTTPS.fromJson(json['https']),
        http = VeilidConfigHTTP.fromJson(json['http']);
}

////////////

class VeilidConfigUDP {
  bool enabled;
  int socketPoolSize;
  String listenAddress;
  String? publicAddress;

  VeilidConfigUDP(
      {required this.enabled,
      required this.socketPoolSize,
      required this.listenAddress,
      this.publicAddress});

  Map<String, dynamic> toJson() {
    return {
      'enabled': enabled,
      'socket_pool_size': socketPoolSize,
      'listen_address': listenAddress,
      'public_address': publicAddress,
    };
  }

  VeilidConfigUDP.fromJson(dynamic json)
      : enabled = json['enabled'],
        socketPoolSize = json['socket_pool_size'],
        listenAddress = json['listen_address'],
        publicAddress = json['publicAddress'];
}

////////////

class VeilidConfigTCP {
  bool connect;
  bool listen;
  int maxConnections;
  String listenAddress;
  String? publicAddress;

  VeilidConfigTCP(
      {required this.connect,
      required this.listen,
      required this.maxConnections,
      required this.listenAddress,
      this.publicAddress});

  Map<String, dynamic> toJson() {
    return {
      'connect': connect,
      'listen': listen,
      'max_connections': maxConnections,
      'listen_address': listenAddress,
      'public_address': publicAddress,
    };
  }

  VeilidConfigTCP.fromJson(dynamic json)
      : connect = json['connect'],
        listen = json['listen'],
        maxConnections = json['max_connections'],
        listenAddress = json['listen_address'],
        publicAddress = json['publicAddress'];
}

////////////

class VeilidConfigWS {
  bool connect;
  bool listen;
  int maxConnections;
  String listenAddress;
  String path;
  String? url;

  VeilidConfigWS(
      {required this.connect,
      required this.listen,
      required this.maxConnections,
      required this.listenAddress,
      required this.path,
      this.url});

  Map<String, dynamic> toJson() {
    return {
      'connect': connect,
      'listen': listen,
      'max_connections': maxConnections,
      'listen_address': listenAddress,
      'path': path,
      'url': url,
    };
  }

  VeilidConfigWS.fromJson(dynamic json)
      : connect = json['connect'],
        listen = json['listen'],
        maxConnections = json['max_connections'],
        listenAddress = json['listen_address'],
        path = json['path'],
        url = json['url'];
}

////////////

class VeilidConfigWSS {
  bool connect;
  bool listen;
  int maxConnections;
  String listenAddress;
  String path;
  String? url;

  VeilidConfigWSS(
      {required this.connect,
      required this.listen,
      required this.maxConnections,
      required this.listenAddress,
      required this.path,
      this.url});

  Map<String, dynamic> toJson() {
    return {
      'connect': connect,
      'listen': listen,
      'max_connections': maxConnections,
      'listen_address': listenAddress,
      'path': path,
      'url': url,
    };
  }

  VeilidConfigWSS.fromJson(dynamic json)
      : connect = json['connect'],
        listen = json['listen'],
        maxConnections = json['max_connections'],
        listenAddress = json['listen_address'],
        path = json['path'],
        url = json['url'];
}

////////////

class VeilidConfigProtocol {
  VeilidConfigUDP udp;
  VeilidConfigTCP tcp;
  VeilidConfigWS ws;
  VeilidConfigWSS wss;

  VeilidConfigProtocol({
    required this.udp,
    required this.tcp,
    required this.ws,
    required this.wss,
  });

  Map<String, dynamic> toJson() {
    return {
      'udp': udp.toJson(),
      'tcp': tcp.toJson(),
      'ws': ws.toJson(),
      'wss': wss.toJson(),
    };
  }

  VeilidConfigProtocol.fromJson(dynamic json)
      : udp = VeilidConfigUDP.fromJson(json['udp']),
        tcp = VeilidConfigTCP.fromJson(json['tcp']),
        ws = VeilidConfigWS.fromJson(json['ws']),
        wss = VeilidConfigWSS.fromJson(json['wss']);
}

////////////

class VeilidConfigTLS {
  String certificatePath;
  String privateKeyPath;
  int connectionInitialTimeoutMs;

  VeilidConfigTLS({
    required this.certificatePath,
    required this.privateKeyPath,
    required this.connectionInitialTimeoutMs,
  });

  Map<String, dynamic> toJson() {
    return {
      'certificate_path': certificatePath,
      'private_key_path': privateKeyPath,
      'connection_initial_timeout_ms': connectionInitialTimeoutMs,
    };
  }

  VeilidConfigTLS.fromJson(dynamic json)
      : certificatePath = json['certificate_path'],
        privateKeyPath = json['private_key_path'],
        connectionInitialTimeoutMs = json['connection_initial_timeout_ms'];
}

////////////

class VeilidConfigDHT {
  int resolveNodeTimeoutMs;
  int resolveNodeCount;
  int resolveNodeFanout;
  int maxFindNodeCount;
  int getValueTimeoutMs;
  int getValueCount;
  int getValueFanout;
  int setValueTimeoutMs;
  int setValueCount;
  int setValueFanout;
  int minPeerCount;
  int minPeerRefreshTimeMs;
  int validateDialInfoReceiptTimeMs;
  int localSubkeyCacheSize;
  int localMaxSubkeyCacheMemoryMb;
  int remoteSubkeyCacheSize;
  int remoteMaxRecords;
  int remoteMaxSubkeyCacheMemoryMb;
  int remoteMaxStorageSpaceMb;

  VeilidConfigDHT(
      {required this.resolveNodeTimeoutMs,
      required this.resolveNodeCount,
      required this.resolveNodeFanout,
      required this.maxFindNodeCount,
      required this.getValueTimeoutMs,
      required this.getValueCount,
      required this.getValueFanout,
      required this.setValueTimeoutMs,
      required this.setValueCount,
      required this.setValueFanout,
      required this.minPeerCount,
      required this.minPeerRefreshTimeMs,
      required this.validateDialInfoReceiptTimeMs,
      required this.localSubkeyCacheSize,
      required this.localMaxSubkeyCacheMemoryMb,
      required this.remoteSubkeyCacheSize,
      required this.remoteMaxRecords,
      required this.remoteMaxSubkeyCacheMemoryMb,
      required this.remoteMaxStorageSpaceMb});

  Map<String, dynamic> toJson() {
    return {
      'max_find_node_count': maxFindNodeCount,
      'resolve_node_timeout_ms': resolveNodeTimeoutMs,
      'resolve_node_count': resolveNodeCount,
      'resolve_node_fanout': resolveNodeFanout,
      'get_value_timeout_ms': getValueTimeoutMs,
      'get_value_count': getValueCount,
      'get_value_fanout': getValueFanout,
      'set_value_timeout_ms': setValueTimeoutMs,
      'set_value_count': setValueCount,
      'set_value_fanout': setValueFanout,
      'min_peer_count': minPeerCount,
      'min_peer_refresh_time_ms': minPeerRefreshTimeMs,
      'validate_dial_info_receipt_time_ms': validateDialInfoReceiptTimeMs,
      'local_subkey_cache_size': localSubkeyCacheSize,
      'local_max_subkey_cache_memory_mb': localMaxSubkeyCacheMemoryMb,
      'remote_subkey_cache_size': remoteSubkeyCacheSize,
      'remote_max_records': remoteMaxRecords,
      'remote_max_subkey_cache_memory_mb': remoteMaxSubkeyCacheMemoryMb,
      'remote_max_storage_space_mb': remoteMaxStorageSpaceMb,
    };
  }

  VeilidConfigDHT.fromJson(dynamic json)
      : resolveNodeTimeoutMs = json['resolve_node_timeout_ms'],
        resolveNodeCount = json['resolve_node_count'],
        resolveNodeFanout = json['resolve_node_fanout'],
        maxFindNodeCount = json['max_find_node_count'],
        getValueTimeoutMs = json['get_value_timeout_ms'],
        getValueCount = json['get_value_count'],
        getValueFanout = json['get_value_fanout'],
        setValueTimeoutMs = json['set_value_timeout_ms'],
        setValueCount = json['set_value_count'],
        setValueFanout = json['set_value_fanout'],
        minPeerCount = json['min_peer_count'],
        minPeerRefreshTimeMs = json['min_peer_refresh_time_ms'],
        validateDialInfoReceiptTimeMs =
            json['validate_dial_info_receipt_time_ms'],
        localSubkeyCacheSize = json['local_subkey_cache_size'],
        localMaxSubkeyCacheMemoryMb = json['local_max_subkey_cache_memory_mb'],
        remoteSubkeyCacheSize = json['remote_subkey_cache_size'],
        remoteMaxRecords = json['remote_max_records'],
        remoteMaxSubkeyCacheMemoryMb =
            json['remote_max_subkey_cache_memory_mb'],
        remoteMaxStorageSpaceMb = json['remote_max_storage_space_mb'];
}

////////////

class VeilidConfigRPC {
  int concurrency;
  int queueSize;
  int? maxTimestampBehindMs;
  int? maxTimestampAheadMs;
  int timeoutMs;
  int maxRouteHopCount;
  int defaultRouteHopCount;

  VeilidConfigRPC(
      {required this.concurrency,
      required this.queueSize,
      this.maxTimestampBehindMs,
      this.maxTimestampAheadMs,
      required this.timeoutMs,
      required this.maxRouteHopCount,
      required this.defaultRouteHopCount});

  Map<String, dynamic> toJson() {
    return {
      'concurrency': concurrency,
      'queue_size': queueSize,
      'max_timestamp_behind_ms': maxTimestampBehindMs,
      'max_timestamp_ahead_ms': maxTimestampAheadMs,
      'timeout_ms': timeoutMs,
      'max_route_hop_count': maxRouteHopCount,
      'default_route_hop_count': defaultRouteHopCount,
    };
  }

  VeilidConfigRPC.fromJson(dynamic json)
      : concurrency = json['concurrency'],
        queueSize = json['queue_size'],
        maxTimestampBehindMs = json['max_timestamp_behind_ms'],
        maxTimestampAheadMs = json['max_timestamp_ahead_ms'],
        timeoutMs = json['timeout_ms'],
        maxRouteHopCount = json['max_route_hop_count'],
        defaultRouteHopCount = json['default_route_hop_count'];
}

////////////

class VeilidConfigRoutingTable {
  List<TypedKey> nodeId;
  List<TypedSecret> nodeIdSecret;
  List<String> bootstrap;
  int limitOverAttached;
  int limitFullyAttached;
  int limitAttachedStrong;
  int limitAttachedGood;
  int limitAttachedWeak;

  VeilidConfigRoutingTable({
    required this.nodeId,
    required this.nodeIdSecret,
    required this.bootstrap,
    required this.limitOverAttached,
    required this.limitFullyAttached,
    required this.limitAttachedStrong,
    required this.limitAttachedGood,
    required this.limitAttachedWeak,
  });

  Map<String, dynamic> toJson() {
    return {
      'node_id': nodeId.map((p) => p.toJson()).toList(),
      'node_id_secret': nodeIdSecret.map((p) => p.toJson()).toList(),
      'bootstrap': bootstrap.map((p) => p).toList(),
      'limit_over_attached': limitOverAttached,
      'limit_fully_attached': limitFullyAttached,
      'limit_attached_strong': limitAttachedStrong,
      'limit_attached_good': limitAttachedGood,
      'limit_attached_weak': limitAttachedWeak,
    };
  }

  VeilidConfigRoutingTable.fromJson(dynamic json)
      : nodeId = List<TypedKey>.from(
            json['node_id'].map((j) => TypedKey.fromJson(j))),
        nodeIdSecret = List<TypedSecret>.from(
            json['node_id_secret'].map((j) => TypedSecret.fromJson(j))),
        bootstrap = List<String>.from(json['bootstrap'].map((j) => j)),
        limitOverAttached = json['limit_over_attached'],
        limitFullyAttached = json['limit_fully_attached'],
        limitAttachedStrong = json['limit_attached_strong'],
        limitAttachedGood = json['limit_attached_good'],
        limitAttachedWeak = json['limit_attached_weak'];
}

////////////

class VeilidConfigNetwork {
  int connectionInitialTimeoutMs;
  int connectionInactivityTimeoutMs;
  int maxConnectionsPerIp4;
  int maxConnectionsPerIp6Prefix;
  int maxConnectionsPerIp6PrefixSize;
  int maxConnectionFrequencyPerMin;
  int clientWhitelistTimeoutMs;
  int reverseConnectionReceiptTimeMs;
  int holePunchReceiptTimeMs;
  String? networkKeyPassword;
  VeilidConfigRoutingTable routingTable;
  VeilidConfigRPC rpc;
  VeilidConfigDHT dht;
  bool upnp;
  bool detectAddressChanges;
  int restrictedNatRetries;
  VeilidConfigTLS tls;
  VeilidConfigApplication application;
  VeilidConfigProtocol protocol;

  VeilidConfigNetwork({
    required this.connectionInitialTimeoutMs,
    required this.connectionInactivityTimeoutMs,
    required this.maxConnectionsPerIp4,
    required this.maxConnectionsPerIp6Prefix,
    required this.maxConnectionsPerIp6PrefixSize,
    required this.maxConnectionFrequencyPerMin,
    required this.clientWhitelistTimeoutMs,
    required this.reverseConnectionReceiptTimeMs,
    required this.holePunchReceiptTimeMs,
    this.networkKeyPassword,
    required this.routingTable,
    required this.rpc,
    required this.dht,
    required this.upnp,
    required this.detectAddressChanges,
    required this.restrictedNatRetries,
    required this.tls,
    required this.application,
    required this.protocol,
  });

  Map<String, dynamic> toJson() {
    return {
      'connection_initial_timeout_ms': connectionInitialTimeoutMs,
      'connection_inactivity_timeout_ms': connectionInactivityTimeoutMs,
      'max_connections_per_ip4': maxConnectionsPerIp4,
      'max_connections_per_ip6_prefix': maxConnectionsPerIp6Prefix,
      'max_connections_per_ip6_prefix_size': maxConnectionsPerIp6PrefixSize,
      'max_connection_frequency_per_min': maxConnectionFrequencyPerMin,
      'client_whitelist_timeout_ms': clientWhitelistTimeoutMs,
      'reverse_connection_receipt_time_ms': reverseConnectionReceiptTimeMs,
      'hole_punch_receipt_time_ms': holePunchReceiptTimeMs,
      'network_key_password': networkKeyPassword,
      'routing_table': routingTable.toJson(),
      'rpc': rpc.toJson(),
      'dht': dht.toJson(),
      'upnp': upnp,
      'detect_address_changes': detectAddressChanges,
      'restricted_nat_retries': restrictedNatRetries,
      'tls': tls.toJson(),
      'application': application.toJson(),
      'protocol': protocol.toJson(),
    };
  }

  VeilidConfigNetwork.fromJson(dynamic json)
      : connectionInitialTimeoutMs = json['connection_initial_timeout_ms'],
        connectionInactivityTimeoutMs =
            json['connection_inactivity_timeout_ms'],
        maxConnectionsPerIp4 = json['max_connections_per_ip4'],
        maxConnectionsPerIp6Prefix = json['max_connections_per_ip6_prefix'],
        maxConnectionsPerIp6PrefixSize =
            json['max_connections_per_ip6_prefix_size'],
        maxConnectionFrequencyPerMin = json['max_connection_frequency_per_min'],
        clientWhitelistTimeoutMs = json['client_whitelist_timeout_ms'],
        reverseConnectionReceiptTimeMs =
            json['reverse_connection_receipt_time_ms'],
        holePunchReceiptTimeMs = json['hole_punch_receipt_time_ms'],
        networkKeyPassword = json['network_key_password'],
        routingTable = VeilidConfigRoutingTable.fromJson(json['routing_table']),
        rpc = VeilidConfigRPC.fromJson(json['rpc']),
        dht = VeilidConfigDHT.fromJson(json['dht']),
        upnp = json['upnp'],
        detectAddressChanges = json['detect_address_changes'],
        restrictedNatRetries = json['restricted_nat_retries'],
        tls = VeilidConfigTLS.fromJson(json['tls']),
        application = VeilidConfigApplication.fromJson(json['application']),
        protocol = VeilidConfigProtocol.fromJson(json['protocol']);
}

////////////

class VeilidConfigTableStore {
  String directory;
  bool delete;

  VeilidConfigTableStore({
    required this.directory,
    required this.delete,
  });

  Map<String, dynamic> toJson() {
    return {'directory': directory, 'delete': delete};
  }

  VeilidConfigTableStore.fromJson(dynamic json)
      : directory = json['directory'],
        delete = json['delete'];
}

////////////

class VeilidConfigBlockStore {
  String directory;
  bool delete;

  VeilidConfigBlockStore({
    required this.directory,
    required this.delete,
  });

  Map<String, dynamic> toJson() {
    return {'directory': directory, 'delete': delete};
  }

  VeilidConfigBlockStore.fromJson(dynamic json)
      : directory = json['directory'],
        delete = json['delete'];
}

////////////

class VeilidConfigProtectedStore {
  bool allowInsecureFallback;
  bool alwaysUseInsecureStorage;
  String directory;
  bool delete;
  String deviceEncryptionKeyPassword;
  String? newDeviceEncryptionKeyPassword;

  VeilidConfigProtectedStore(
      {required this.allowInsecureFallback,
      required this.alwaysUseInsecureStorage,
      required this.directory,
      required this.delete,
      required this.deviceEncryptionKeyPassword,
      String? newDeviceEncryptionKeyPassword});

  Map<String, dynamic> toJson() {
    return {
      'allow_insecure_fallback': allowInsecureFallback,
      'always_use_insecure_storage': alwaysUseInsecureStorage,
      'directory': directory,
      'delete': delete,
      'device_encryption_key_password': deviceEncryptionKeyPassword,
      'new_device_encryption_key': newDeviceEncryptionKeyPassword,
    };
  }

  VeilidConfigProtectedStore.fromJson(dynamic json)
      : allowInsecureFallback = json['allow_insecure_fallback'],
        alwaysUseInsecureStorage = json['always_use_insecure_storage'],
        directory = json['directory'],
        delete = json['delete'],
        deviceEncryptionKeyPassword = json['device_encryption_key_password'],
        newDeviceEncryptionKeyPassword =
            json['new_device_encryption_key_password'];
}

////////////

class VeilidConfigCapabilities {
  List<String> disable;

  VeilidConfigCapabilities({
    required this.disable,
  });

  Map<String, dynamic> toJson() {
    return {
      'disable': disable.map((p) => p).toList(),
    };
  }

  VeilidConfigCapabilities.fromJson(dynamic json)
      : disable = List<String>.from(json['disable'].map((j) => j));
}

////////////

class VeilidConfig {
  String programName;
  String namespace;
  VeilidConfigCapabilities capabilities;
  VeilidConfigProtectedStore protectedStore;
  VeilidConfigTableStore tableStore;
  VeilidConfigBlockStore blockStore;
  VeilidConfigNetwork network;

  VeilidConfig({
    required this.programName,
    required this.namespace,
    required this.capabilities,
    required this.protectedStore,
    required this.tableStore,
    required this.blockStore,
    required this.network,
  });

  Map<String, dynamic> toJson() {
    return {
      'program_name': programName,
      'namespace': namespace,
      'capabilities': capabilities.toJson(),
      'protected_store': protectedStore.toJson(),
      'table_store': tableStore.toJson(),
      'block_store': blockStore.toJson(),
      'network': network.toJson()
    };
  }

  VeilidConfig.fromJson(dynamic json)
      : programName = json['program_name'],
        namespace = json['namespace'],
        capabilities = VeilidConfigCapabilities.fromJson(json['capabilities']),
        protectedStore =
            VeilidConfigProtectedStore.fromJson(json['protected_store']),
        tableStore = VeilidConfigTableStore.fromJson(json['table_store']),
        blockStore = VeilidConfigBlockStore.fromJson(json['block_store']),
        network = VeilidConfigNetwork.fromJson(json['network']);
}
