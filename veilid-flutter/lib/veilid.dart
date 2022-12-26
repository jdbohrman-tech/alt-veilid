import 'dart:async';
import 'dart:typed_data';
import 'dart:convert';

import 'package:change_case/change_case.dart';

import 'veilid_stub.dart'
    if (dart.library.io) 'veilid_ffi.dart'
    if (dart.library.js) 'veilid_js.dart';

//////////////////////////////////////////////////////////

export 'default_config.dart';

//////////////////////////////////////////////////////////
// FFI Platform-specific config

class VeilidFFIConfigLoggingTerminal {
  bool enabled;
  VeilidConfigLogLevel level;

  VeilidFFIConfigLoggingTerminal({
    required this.enabled,
    required this.level,
  });

  Map<String, dynamic> get json {
    return {
      'enabled': enabled,
      'level': level.json,
    };
  }

  VeilidFFIConfigLoggingTerminal.fromJson(Map<String, dynamic> json)
      : enabled = json['enabled'],
        level = veilidConfigLogLevelFromJson(json['level']);
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

  Map<String, dynamic> get json {
    return {
      'enabled': enabled,
      'level': level.json,
      'grpc_endpoint': grpcEndpoint,
      'service_name': serviceName,
    };
  }

  VeilidFFIConfigLoggingOtlp.fromJson(Map<String, dynamic> json)
      : enabled = json['enabled'],
        level = veilidConfigLogLevelFromJson(json['level']),
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

  Map<String, dynamic> get json {
    return {
      'enabled': enabled,
      'level': level.json,
    };
  }

  VeilidFFIConfigLoggingApi.fromJson(Map<String, dynamic> json)
      : enabled = json['enabled'],
        level = veilidConfigLogLevelFromJson(json['level']);
}

class VeilidFFIConfigLogging {
  VeilidFFIConfigLoggingTerminal terminal;
  VeilidFFIConfigLoggingOtlp otlp;
  VeilidFFIConfigLoggingApi api;

  VeilidFFIConfigLogging(
      {required this.terminal, required this.otlp, required this.api});

  Map<String, dynamic> get json {
    return {
      'terminal': terminal.json,
      'otlp': otlp.json,
      'api': api.json,
    };
  }

  VeilidFFIConfigLogging.fromJson(Map<String, dynamic> json)
      : terminal = VeilidFFIConfigLoggingTerminal.fromJson(json['terminal']),
        otlp = VeilidFFIConfigLoggingOtlp.fromJson(json['otlp']),
        api = VeilidFFIConfigLoggingApi.fromJson(json['api']);
}

class VeilidFFIConfig {
  VeilidFFIConfigLogging logging;

  VeilidFFIConfig({
    required this.logging,
  });

  Map<String, dynamic> get json {
    return {
      'logging': logging.json,
    };
  }

  VeilidFFIConfig.fromJson(Map<String, dynamic> json)
      : logging = VeilidFFIConfigLogging.fromJson(json['logging']);
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

  Map<String, dynamic> get json {
    return {
      'enabled': enabled,
      'level': level.json,
      'logs_in_timings': logsInTimings,
      'logs_in_console': logsInConsole,
    };
  }

  VeilidWASMConfigLoggingPerformance.fromJson(Map<String, dynamic> json)
      : enabled = json['enabled'],
        level = veilidConfigLogLevelFromJson(json['level']),
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

  Map<String, dynamic> get json {
    return {
      'enabled': enabled,
      'level': level.json,
    };
  }

  VeilidWASMConfigLoggingApi.fromJson(Map<String, dynamic> json)
      : enabled = json['enabled'],
        level = veilidConfigLogLevelFromJson(json['level']);
}

class VeilidWASMConfigLogging {
  VeilidWASMConfigLoggingPerformance performance;
  VeilidWASMConfigLoggingApi api;

  VeilidWASMConfigLogging({required this.performance, required this.api});

  Map<String, dynamic> get json {
    return {
      'performance': performance.json,
      'api': api.json,
    };
  }

  VeilidWASMConfigLogging.fromJson(Map<String, dynamic> json)
      : performance =
            VeilidWASMConfigLoggingPerformance.fromJson(json['performance']),
        api = VeilidWASMConfigLoggingApi.fromJson(json['api']);
}

class VeilidWASMConfig {
  VeilidWASMConfigLogging logging;

  VeilidWASMConfig({
    required this.logging,
  });

  Map<String, dynamic> get json {
    return {
      'logging': logging.json,
    };
  }

  VeilidWASMConfig.fromJson(Map<String, dynamic> json)
      : logging = VeilidWASMConfigLogging.fromJson(json['logging']);
}

//////////////////////////////////////
/// JSON Encode Helper

Object? veilidApiToEncodable(Object? value) {
  if (value == null) {
    return value;
  }
  switch (value.runtimeType) {
    case AttachmentState:
      return (value as AttachmentState).json;
    case VeilidLogLevel:
      return (value as VeilidLogLevel).json;
    case VeilidConfigLogLevel:
      return (value as VeilidConfigLogLevel).json;
  }
  throw UnsupportedError('Cannot convert to JSON: $value');
}

//////////////////////////////////////
/// AttachmentState

enum AttachmentState {
  detached,
  attaching,
  attachedWeak,
  attachedGood,
  attachedStrong,
  fullyAttached,
  overAttached,
  detaching,
}

extension AttachmentStateExt on AttachmentState {
  String get json {
    return name.toPascalCase();
  }
}

AttachmentState attachmentStateFromJson(String j) {
  return AttachmentState.values.byName(j.toCamelCase());
}

//////////////////////////////////////
/// VeilidLogLevel

enum VeilidLogLevel {
  error,
  warn,
  info,
  debug,
  trace,
}

extension VeilidLogLevelExt on VeilidLogLevel {
  String get json {
    return name.toPascalCase();
  }
}

VeilidLogLevel veilidLogLevelFromJson(String j) {
  return VeilidLogLevel.values.byName(j.toCamelCase());
}

//////////////////////////////////////
/// VeilidConfigLogLevel

enum VeilidConfigLogLevel {
  off,
  error,
  warn,
  info,
  debug,
  trace,
}

extension VeilidConfigLogLevelExt on VeilidConfigLogLevel {
  String get json {
    return name.toPascalCase();
  }
}

VeilidConfigLogLevel veilidConfigLogLevelFromJson(String j) {
  return VeilidConfigLogLevel.values.byName(j.toCamelCase());
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

  Map<String, dynamic> get json {
    return {
      'enabled': enabled,
      'listen_address': listenAddress,
      'path': path,
      'url': url
    };
  }

  VeilidConfigHTTPS.fromJson(Map<String, dynamic> json)
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

  Map<String, dynamic> get json {
    return {
      'enabled': enabled,
      'listen_address': listenAddress,
      'path': path,
      'url': url
    };
  }

  VeilidConfigHTTP.fromJson(Map<String, dynamic> json)
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

  Map<String, dynamic> get json {
    return {
      'https': https.json,
      'http': http.json,
    };
  }

  VeilidConfigApplication.fromJson(Map<String, dynamic> json)
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

  Map<String, dynamic> get json {
    return {
      'enabled': enabled,
      'socket_pool_size': socketPoolSize,
      'listen_address': listenAddress,
      'public_address': publicAddress,
    };
  }

  VeilidConfigUDP.fromJson(Map<String, dynamic> json)
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

  Map<String, dynamic> get json {
    return {
      'connect': connect,
      'listen': listen,
      'max_connections': maxConnections,
      'listen_address': listenAddress,
      'public_address': publicAddress,
    };
  }

  VeilidConfigTCP.fromJson(Map<String, dynamic> json)
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

  Map<String, dynamic> get json {
    return {
      'connect': connect,
      'listen': listen,
      'max_connections': maxConnections,
      'listen_address': listenAddress,
      'path': path,
      'url': url,
    };
  }

  VeilidConfigWS.fromJson(Map<String, dynamic> json)
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

  Map<String, dynamic> get json {
    return {
      'connect': connect,
      'listen': listen,
      'max_connections': maxConnections,
      'listen_address': listenAddress,
      'path': path,
      'url': url,
    };
  }

  VeilidConfigWSS.fromJson(Map<String, dynamic> json)
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

  Map<String, dynamic> get json {
    return {
      'udp': udp.json,
      'tcp': tcp.json,
      'ws': ws.json,
      'wss': wss.json,
    };
  }

  VeilidConfigProtocol.fromJson(Map<String, dynamic> json)
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

  Map<String, dynamic> get json {
    return {
      'certificate_path': certificatePath,
      'private_key_path': privateKeyPath,
      'connection_initial_timeout_ms': connectionInitialTimeoutMs,
    };
  }

  VeilidConfigTLS.fromJson(Map<String, dynamic> json)
      : certificatePath = json['certificate_path'],
        privateKeyPath = json['private_key_path'],
        connectionInitialTimeoutMs = json['connection_initial_timeout_ms'];
}

////////////

class VeilidConfigDHT {
  int? resolveNodeTimeoutMs;
  int resolveNodeCount;
  int resolveNodeFanout;
  int maxFindNodeCount;
  int? getValueTimeoutMs;
  int getValueCount;
  int getValueFanout;
  int? setValueTimeoutMs;
  int setValueCount;
  int setValueFanout;
  int minPeerCount;
  int minPeerRefreshTimeMs;
  int validateDialInfoReceiptTimeMs;

  VeilidConfigDHT(
      {this.resolveNodeTimeoutMs,
      required this.resolveNodeCount,
      required this.resolveNodeFanout,
      required this.maxFindNodeCount,
      this.getValueTimeoutMs,
      required this.getValueCount,
      required this.getValueFanout,
      this.setValueTimeoutMs,
      required this.setValueCount,
      required this.setValueFanout,
      required this.minPeerCount,
      required this.minPeerRefreshTimeMs,
      required this.validateDialInfoReceiptTimeMs});

  Map<String, dynamic> get json {
    return {
      'resolve_node_timeout_ms': resolveNodeTimeoutMs,
      'resolve_node_count': resolveNodeCount,
      'resolve_node_fanout': resolveNodeFanout,
      'max_find_node_count': maxFindNodeCount,
      'get_value_timeout_ms': getValueTimeoutMs,
      'get_value_count': getValueCount,
      'get_value_fanout': getValueFanout,
      'set_value_timeout_ms': setValueTimeoutMs,
      'set_value_count': setValueCount,
      'set_value_fanout': setValueFanout,
      'min_peer_count': minPeerCount,
      'min_peer_refresh_time_ms': minPeerRefreshTimeMs,
      'validate_dial_info_receipt_time_ms': validateDialInfoReceiptTimeMs
    };
  }

  VeilidConfigDHT.fromJson(Map<String, dynamic> json)
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
            json['validate_dial_info_receipt_time_ms'];
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

  Map<String, dynamic> get json {
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

  VeilidConfigRPC.fromJson(Map<String, dynamic> json)
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
  int limitOverAttached;
  int limitFullyAttached;
  int limitAttachedStrong;
  int limitAttachedGood;
  int limitAttachedWeak;

  VeilidConfigRoutingTable({
    required this.limitOverAttached,
    required this.limitFullyAttached,
    required this.limitAttachedStrong,
    required this.limitAttachedGood,
    required this.limitAttachedWeak,
  });

  Map<String, dynamic> get json {
    return {
      'limit_over_attached': limitOverAttached,
      'limit_fully_attached': limitFullyAttached,
      'limit_attached_strong': limitAttachedStrong,
      'limit_attached_good': limitAttachedGood,
      'limit_attached_weak': limitAttachedWeak,
    };
  }

  VeilidConfigRoutingTable.fromJson(Map<String, dynamic> json)
      : limitOverAttached = json['limit_over_attached'],
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
  String? nodeId;
  String? nodeIdSecret;
  List<String> bootstrap;
  List<String> bootstrapNodes;
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
    required this.nodeId,
    required this.nodeIdSecret,
    required this.bootstrap,
    required this.bootstrapNodes,
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

  Map<String, dynamic> get json {
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
      'node_id': nodeId,
      'node_id_secret': nodeIdSecret,
      'bootstrap': bootstrap,
      'bootstrap_nodes': bootstrapNodes,
      'routing_table': routingTable.json,
      'rpc': rpc.json,
      'dht': dht.json,
      'upnp': upnp,
      'detect_address_changes': detectAddressChanges,
      'restricted_nat_retries': restrictedNatRetries,
      'tls': tls.json,
      'application': application.json,
      'protocol': protocol.json,
    };
  }

  VeilidConfigNetwork.fromJson(Map<String, dynamic> json)
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
        nodeId = json['node_id'],
        nodeIdSecret = json['node_id_secret'],
        bootstrap = json['bootstrap'],
        bootstrapNodes = json['bootstrap_nodes'],
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

  Map<String, dynamic> get json {
    return {'directory': directory, 'delete': delete};
  }

  VeilidConfigTableStore.fromJson(Map<String, dynamic> json)
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

  Map<String, dynamic> get json {
    return {'directory': directory, 'delete': delete};
  }

  VeilidConfigBlockStore.fromJson(Map<String, dynamic> json)
      : directory = json['directory'],
        delete = json['delete'];
}

////////////

class VeilidConfigProtectedStore {
  bool allowInsecureFallback;
  bool alwaysUseInsecureStorage;
  String insecureFallbackDirectory;
  bool delete;

  VeilidConfigProtectedStore({
    required this.allowInsecureFallback,
    required this.alwaysUseInsecureStorage,
    required this.insecureFallbackDirectory,
    required this.delete,
  });

  Map<String, dynamic> get json {
    return {
      'allow_insecure_fallback': allowInsecureFallback,
      'always_use_insecure_storage': alwaysUseInsecureStorage,
      'insecure_fallback_directory': insecureFallbackDirectory,
      'delete': delete,
    };
  }

  VeilidConfigProtectedStore.fromJson(Map<String, dynamic> json)
      : allowInsecureFallback = json['allow_insecure_fallback'],
        alwaysUseInsecureStorage = json['always_use_insecure_storage'],
        insecureFallbackDirectory = json['insecure_fallback_directory'],
        delete = json['delete'];
}

////////////

class VeilidConfigCapabilities {
  bool protocolUDP;
  bool protocolConnectTCP;
  bool protocolAcceptTCP;
  bool protocolConnectWS;
  bool protocolAcceptWS;
  bool protocolConnectWSS;
  bool protocolAcceptWSS;

  VeilidConfigCapabilities({
    required this.protocolUDP,
    required this.protocolConnectTCP,
    required this.protocolAcceptTCP,
    required this.protocolConnectWS,
    required this.protocolAcceptWS,
    required this.protocolConnectWSS,
    required this.protocolAcceptWSS,
  });

  Map<String, dynamic> get json {
    return {
      'protocol_udp': protocolUDP,
      'protocol_connect_tcp': protocolConnectTCP,
      'protocol_accept_tcp': protocolAcceptTCP,
      'protocol_connect_ws': protocolConnectWS,
      'protocol_accept_ws': protocolAcceptWS,
      'protocol_connect_wss': protocolConnectWSS,
      'protocol_accept_wss': protocolAcceptWSS,
    };
  }

  VeilidConfigCapabilities.fromJson(Map<String, dynamic> json)
      : protocolUDP = json['protocol_udp'],
        protocolConnectTCP = json['protocol_connect_tcp'],
        protocolAcceptTCP = json['protocol_accept_tcp'],
        protocolConnectWS = json['protocol_connect_ws'],
        protocolAcceptWS = json['protocol_accept_ws'],
        protocolConnectWSS = json['protocol_connect_wss'],
        protocolAcceptWSS = json['protocol_accept_wss'];
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

  Map<String, dynamic> get json {
    return {
      'program_name': programName,
      'namespace': namespace,
      'capabilities': capabilities.json,
      'protected_store': protectedStore.json,
      'table_store': tableStore.json,
      'block_store': blockStore.json,
      'network': network.json
    };
  }

  VeilidConfig.fromJson(Map<String, dynamic> json)
      : programName = json['program_name'],
        namespace = json['namespace'],
        capabilities = VeilidConfigCapabilities.fromJson(json['capabilities']),
        protectedStore =
            VeilidConfigProtectedStore.fromJson(json['protected_store']),
        tableStore = VeilidConfigTableStore.fromJson(json['table_store']),
        blockStore = VeilidConfigBlockStore.fromJson(json['block_store']),
        network = VeilidConfigNetwork.fromJson(json['network']);
}

////////////

class LatencyStats {
  BigInt fastest;
  BigInt average;
  BigInt slowest;

  LatencyStats({
    required this.fastest,
    required this.average,
    required this.slowest,
  });

  Map<String, dynamic> get json {
    return {
      'fastest': fastest.toString(),
      'average': average.toString(),
      'slowest': slowest.toString(),
    };
  }

  LatencyStats.fromJson(Map<String, dynamic> json)
      : fastest = BigInt.parse(json['fastest']),
        average = BigInt.parse(json['average']),
        slowest = BigInt.parse(json['slowest']);
}

////////////

class TransferStats {
  BigInt total;
  BigInt maximum;
  BigInt average;
  BigInt minimum;

  TransferStats({
    required this.total,
    required this.maximum,
    required this.average,
    required this.minimum,
  });

  Map<String, dynamic> get json {
    return {
      'total': total.toString(),
      'maximum': maximum.toString(),
      'average': average.toString(),
      'minimum': minimum.toString(),
    };
  }

  TransferStats.fromJson(Map<String, dynamic> json)
      : total = BigInt.parse(json['total']),
        maximum = BigInt.parse(json['maximum']),
        average = BigInt.parse(json['average']),
        minimum = BigInt.parse(json['minimum']);
}

////////////

class TransferStatsDownUp {
  TransferStats down;
  TransferStats up;

  TransferStatsDownUp({
    required this.down,
    required this.up,
  });

  Map<String, dynamic> get json {
    return {
      'down': down.json,
      'up': up.json,
    };
  }

  TransferStatsDownUp.fromJson(Map<String, dynamic> json)
      : down = TransferStats.fromJson(json['down']),
        up = TransferStats.fromJson(json['up']);
}

////////////

class RPCStats {
  int messagesSent;
  int messagesRcvd;
  int questionsInFlight;
  BigInt? lastQuestion;
  BigInt? lastSeenTs;
  BigInt? firstConsecutiveSeenTs;
  int recentLostAnswers;
  int failedToSend;

  RPCStats({
    required this.messagesSent,
    required this.messagesRcvd,
    required this.questionsInFlight,
    required this.lastQuestion,
    required this.lastSeenTs,
    required this.firstConsecutiveSeenTs,
    required this.recentLostAnswers,
    required this.failedToSend,
  });

  Map<String, dynamic> get json {
    return {
      'messages_sent': messagesSent,
      'messages_rcvd': messagesRcvd,
      'questions_in_flight': questionsInFlight,
      'last_question': lastQuestion?.toString(),
      'last_seen_ts': lastSeenTs?.toString(),
      'first_consecutive_seen_ts': firstConsecutiveSeenTs?.toString(),
      'recent_lost_answers': recentLostAnswers,
      'failed_to_send': failedToSend,
    };
  }

  RPCStats.fromJson(Map<String, dynamic> json)
      : messagesSent = json['messages_sent'],
        messagesRcvd = json['messages_rcvd'],
        questionsInFlight = json['questions_in_flight'],
        lastQuestion = json['last_question'] != null
            ? BigInt.parse(json['last_question'])
            : null,
        lastSeenTs = json['last_seen_ts'] != null
            ? BigInt.parse(json['last_seen_ts'])
            : null,
        firstConsecutiveSeenTs = json['first_consecutive_seen_ts'] != null
            ? BigInt.parse(json['first_consecutive_seen_ts'])
            : null,
        recentLostAnswers = json['recent_lost_answers'],
        failedToSend = json['failed_to_send'];
}

////////////

class PeerStats {
  BigInt timeAdded;
  RPCStats rpcStats;
  LatencyStats? latency;
  TransferStatsDownUp transfer;

  PeerStats({
    required this.timeAdded,
    required this.rpcStats,
    required this.latency,
    required this.transfer,
  });

  Map<String, dynamic> get json {
    return {
      'time_added': timeAdded.toString(),
      'rpc_stats': rpcStats.json,
      'latency': latency?.json,
      'transfer': transfer.json,
    };
  }

  PeerStats.fromJson(Map<String, dynamic> json)
      : timeAdded = BigInt.parse(json['time_added']),
        rpcStats = RPCStats.fromJson(json['rpc_stats']),
        latency = json['latency'] != null
            ? LatencyStats.fromJson(json['latency'])
            : null,
        transfer = TransferStatsDownUp.fromJson(json['transfer']);
}

////////////

class PeerTableData {
  String nodeId;
  PeerAddress peerAddress;
  PeerStats peerStats;

  PeerTableData({
    required this.nodeId,
    required this.peerAddress,
    required this.peerStats,
  });

  Map<String, dynamic> get json {
    return {
      'node_id': nodeId,
      'peer_address': peerAddress.json,
      'peer_stats': peerStats.json,
    };
  }

  PeerTableData.fromJson(Map<String, dynamic> json)
      : nodeId = json['node_id'],
        peerAddress = PeerAddress.fromJson(json['peer_address']),
        peerStats = PeerStats.fromJson(json['peer_stats']);
}

//////////////////////////////////////
/// AttachmentState

enum ProtocolType {
  udp,
  tcp,
  ws,
  wss,
}

extension ProtocolTypeExt on ProtocolType {
  String get json {
    return name.toUpperCase();
  }
}

ProtocolType protocolTypeFromJson(String j) {
  return ProtocolType.values.byName(j.toLowerCase());
}

////////////

class PeerAddress {
  ProtocolType protocolType;
  String socketAddress;

  PeerAddress({
    required this.protocolType,
    required this.socketAddress,
  });

  Map<String, dynamic> get json {
    return {
      'protocol_type': protocolType.json,
      'socket_address': socketAddress,
    };
  }

  PeerAddress.fromJson(Map<String, dynamic> json)
      : protocolType = protocolTypeFromJson(json['protocol_type']),
        socketAddress = json['socket_address'];
}

//////////////////////////////////////
/// VeilidUpdate

abstract class VeilidUpdate {
  factory VeilidUpdate.fromJson(Map<String, dynamic> json) {
    switch (json["kind"]) {
      case "Log":
        {
          return VeilidLog(
              logLevel: veilidLogLevelFromJson(json["log_level"]),
              message: json["message"],
              backtrace: json["backtrace"]);
        }
      case "AppMessage":
        {
          return VeilidAppMessage(
              sender: json["sender"], message: json["message"]);
        }
      case "AppCall":
        {
          return VeilidAppCall(
              sender: json["sender"], message: json["message"], id: json["id"]);
        }
      case "Attachment":
        {
          return VeilidUpdateAttachment(
              state: VeilidStateAttachment.fromJson(json));
        }
      case "Network":
        {
          return VeilidUpdateNetwork(state: VeilidStateNetwork.fromJson(json));
        }
      case "Config":
        {
          return VeilidUpdateConfig(state: VeilidStateConfig.fromJson(json));
        }
      case "Route":
        {
          return VeilidUpdateRoute(state: VeilidStateRoute.fromJson(json));
        }
      default:
        {
          throw VeilidAPIExceptionInternal(
              "Invalid VeilidAPIException type: ${json['kind']}");
        }
    }
  }
  Map<String, dynamic> get json;
}

class VeilidLog implements VeilidUpdate {
  final VeilidLogLevel logLevel;
  final String message;
  final String? backtrace;
  //
  VeilidLog({
    required this.logLevel,
    required this.message,
    required this.backtrace,
  });

  @override
  Map<String, dynamic> get json {
    return {
      'kind': "Log",
      'log_level': logLevel.json,
      'message': message,
      'backtrace': backtrace
    };
  }
}

class VeilidAppMessage implements VeilidUpdate {
  final String? sender;
  final Uint8List message;

  //
  VeilidAppMessage({
    required this.sender,
    required this.message,
  });

  @override
  Map<String, dynamic> get json {
    return {
      'kind': "AppMessage",
      'sender': sender,
      'message': base64UrlEncode(message)
    };
  }
}

class VeilidAppCall implements VeilidUpdate {
  final String? sender;
  final Uint8List message;
  final String id;

  //
  VeilidAppCall({
    required this.sender,
    required this.message,
    required this.id,
  });

  @override
  Map<String, dynamic> get json {
    return {
      'kind': "AppMessage",
      'sender': sender,
      'message': base64UrlEncode(message),
      'id': id,
    };
  }
}

class VeilidUpdateAttachment implements VeilidUpdate {
  final VeilidStateAttachment state;
  //
  VeilidUpdateAttachment({required this.state});

  @override
  Map<String, dynamic> get json {
    var jsonRep = state.json;
    jsonRep['kind'] = "Attachment";
    return jsonRep;
  }
}

class VeilidUpdateNetwork implements VeilidUpdate {
  final VeilidStateNetwork state;
  //
  VeilidUpdateNetwork({required this.state});

  @override
  Map<String, dynamic> get json {
    var jsonRep = state.json;
    jsonRep['kind'] = "Network";
    return jsonRep;
  }
}

class VeilidUpdateConfig implements VeilidUpdate {
  final VeilidStateConfig state;
  //
  VeilidUpdateConfig({required this.state});

  @override
  Map<String, dynamic> get json {
    var jsonRep = state.json;
    jsonRep['kind'] = "Config";
    return jsonRep;
  }
}

class VeilidUpdateRoute implements VeilidUpdate {
  final VeilidStateRoute state;
  //
  VeilidUpdateRoute({required this.state});

  @override
  Map<String, dynamic> get json {
    var jsonRep = state.json;
    jsonRep['kind'] = "Route";
    return jsonRep;
  }
}

//////////////////////////////////////
/// VeilidStateAttachment

class VeilidStateAttachment {
  final AttachmentState state;

  VeilidStateAttachment(this.state);

  VeilidStateAttachment.fromJson(Map<String, dynamic> json)
      : state = attachmentStateFromJson(json['state']);

  Map<String, dynamic> get json {
    return {
      'state': state.json,
    };
  }
}

//////////////////////////////////////
/// VeilidStateNetwork

class VeilidStateNetwork {
  final bool started;
  final BigInt bpsDown;
  final BigInt bpsUp;
  final List<PeerTableData> peers;

  VeilidStateNetwork(
      {required this.started,
      required this.bpsDown,
      required this.bpsUp,
      required this.peers});

  VeilidStateNetwork.fromJson(Map<String, dynamic> json)
      : started = json['started'],
        bpsDown = BigInt.parse(json['bps_down']),
        bpsUp = BigInt.parse(json['bps_up']),
        peers = List<PeerTableData>.from(
            json['peers'].map((j) => PeerTableData.fromJson(j)));

  Map<String, dynamic> get json {
    return {
      'started': started,
      'bps_down': bpsDown.toString(),
      'bps_up': bpsUp.toString(),
      'peers': peers.map((p) => p.json).toList(),
    };
  }
}

//////////////////////////////////////
/// VeilidStateConfig

class VeilidStateConfig {
  final Map<String, dynamic> config;

  VeilidStateConfig({
    required this.config,
  });

  VeilidStateConfig.fromJson(Map<String, dynamic> json)
      : config = json['config'];

  Map<String, dynamic> get json {
    return {'config': config};
  }
}

//////////////////////////////////////
/// VeilidStateRoute

class VeilidStateRoute {
  final List<String> deadRoutes;
  final List<String> deadRemoteRoutes;

  VeilidStateRoute({
    required this.deadRoutes,
    required this.deadRemoteRoutes,
  });

  VeilidStateRoute.fromJson(Map<String, dynamic> json)
      : deadRoutes = List<String>.from(json['dead_routes'].map((j) => j)),
        deadRemoteRoutes =
            List<String>.from(json['dead_remote_routes'].map((j) => j));

  Map<String, dynamic> get json {
    return {
      'dead_routes': deadRoutes.map((p) => p).toList(),
      'dead_remote_routes': deadRemoteRoutes.map((p) => p).toList()
    };
  }
}

//////////////////////////////////////
/// VeilidState

class VeilidState {
  final VeilidStateAttachment attachment;
  final VeilidStateNetwork network;
  final VeilidStateConfig config;

  VeilidState.fromJson(Map<String, dynamic> json)
      : attachment = VeilidStateAttachment.fromJson(json['attachment']),
        network = VeilidStateNetwork.fromJson(json['network']),
        config = VeilidStateConfig.fromJson(json['config']);

  Map<String, dynamic> get json {
    return {
      'attachment': attachment.json,
      'network': network.json,
      'config': config.json
    };
  }
}

//////////////////////////////////////
/// VeilidAPIException

abstract class VeilidAPIException implements Exception {
  factory VeilidAPIException.fromJson(Map<String, dynamic> json) {
    switch (json["kind"]) {
      case "NotInitialized":
        {
          return VeilidAPIExceptionNotInitialized();
        }
      case "AlreadyInitialized":
        {
          return VeilidAPIExceptionAlreadyInitialized();
        }
      case "Timeout":
        {
          return VeilidAPIExceptionTimeout();
        }
      case "Shutdown":
        {
          return VeilidAPIExceptionShutdown();
        }
      case "NodeNotFound":
        {
          return VeilidAPIExceptionNodeNotFound(json["node_id"]);
        }
      case "NoDialInfo":
        {
          return VeilidAPIExceptionNoDialInfo(json["node_id"]);
        }
      case "Internal":
        {
          return VeilidAPIExceptionInternal(json["message"]);
        }
      case "Unimplemented":
        {
          return VeilidAPIExceptionUnimplemented(json["unimplemented"]);
        }
      case "ParseError":
        {
          return VeilidAPIExceptionParseError(json["message"], json["value"]);
        }
      case "InvalidArgument":
        {
          return VeilidAPIExceptionInvalidArgument(
              json["context"], json["argument"], json["value"]);
        }
      case "MissingArgument":
        {
          return VeilidAPIExceptionMissingArgument(
              json["context"], json["argument"]);
        }
      case "Generic":
        {
          return VeilidAPIExceptionGeneric(json["message"]);
        }
      default:
        {
          throw VeilidAPIExceptionInternal(
              "Invalid VeilidAPIException type: ${json['kind']}");
        }
    }
  }

  String toDisplayError();
}

class VeilidAPIExceptionNotInitialized implements VeilidAPIException {
  @override
  String toString() {
    return "VeilidAPIException: NotInitialized";
  }

  @override
  String toDisplayError() {
    return "Not initialized";
  }
}

class VeilidAPIExceptionAlreadyInitialized implements VeilidAPIException {
  @override
  String toString() {
    return "VeilidAPIException: AlreadyInitialized";
  }

  @override
  String toDisplayError() {
    return "Already initialized";
  }
}

class VeilidAPIExceptionTimeout implements VeilidAPIException {
  @override
  String toString() {
    return "VeilidAPIException: Timeout";
  }

  @override
  String toDisplayError() {
    return "Timeout";
  }
}

class VeilidAPIExceptionShutdown implements VeilidAPIException {
  @override
  String toString() {
    return "VeilidAPIException: Shutdown";
  }

  @override
  String toDisplayError() {
    return "Currently shut down";
  }
}

class VeilidAPIExceptionNodeNotFound implements VeilidAPIException {
  final String nodeId;

  @override
  String toString() {
    return "VeilidAPIException: NodeNotFound (nodeId: $nodeId)";
  }

  @override
  String toDisplayError() {
    return "Node node found: $nodeId";
  }

  //
  VeilidAPIExceptionNodeNotFound(this.nodeId);
}

class VeilidAPIExceptionNoDialInfo implements VeilidAPIException {
  final String nodeId;

  @override
  String toString() {
    return "VeilidAPIException: NoDialInfo (nodeId: $nodeId)";
  }

  @override
  String toDisplayError() {
    return "No dial info: $nodeId";
  }

  //
  VeilidAPIExceptionNoDialInfo(this.nodeId);
}

class VeilidAPIExceptionInternal implements VeilidAPIException {
  final String message;

  @override
  String toString() {
    return "VeilidAPIException: Internal ($message)";
  }

  @override
  String toDisplayError() {
    return "Internal error: $message";
  }

  //
  VeilidAPIExceptionInternal(this.message);
}

class VeilidAPIExceptionUnimplemented implements VeilidAPIException {
  final String message;

  @override
  String toString() {
    return "VeilidAPIException: Unimplemented ($message)";
  }

  @override
  String toDisplayError() {
    return "Unimplemented: $message";
  }

  //
  VeilidAPIExceptionUnimplemented(this.message);
}

class VeilidAPIExceptionParseError implements VeilidAPIException {
  final String message;
  final String value;

  @override
  String toString() {
    return "VeilidAPIException: ParseError ($message)\n    value: $value";
  }

  @override
  String toDisplayError() {
    return "Parse error: $message";
  }

  //
  VeilidAPIExceptionParseError(this.message, this.value);
}

class VeilidAPIExceptionInvalidArgument implements VeilidAPIException {
  final String context;
  final String argument;
  final String value;

  @override
  String toString() {
    return "VeilidAPIException: InvalidArgument ($context:$argument)\n    value: $value";
  }

  @override
  String toDisplayError() {
    return "Invalid argument for $context: $argument";
  }

  //
  VeilidAPIExceptionInvalidArgument(this.context, this.argument, this.value);
}

class VeilidAPIExceptionMissingArgument implements VeilidAPIException {
  final String context;
  final String argument;

  @override
  String toString() {
    return "VeilidAPIException: MissingArgument ($context:$argument)";
  }

  @override
  String toDisplayError() {
    return "Missing argument for $context: $argument";
  }

  //
  VeilidAPIExceptionMissingArgument(this.context, this.argument);
}

class VeilidAPIExceptionGeneric implements VeilidAPIException {
  final String message;

  @override
  String toString() {
    return "VeilidAPIException: Generic (message: $message)";
  }

  @override
  String toDisplayError() {
    return message;
  }

  //
  VeilidAPIExceptionGeneric(this.message);
}

//////////////////////////////////////
/// VeilidVersion

class VeilidVersion {
  final int major;
  final int minor;
  final int patch;

  VeilidVersion(this.major, this.minor, this.patch);
}

//////////////////////////////////////
/// Stability

enum Stability {
  lowLatency,
  reliable,
}

extension StabilityExt on Stability {
  String get json {
    return name.toPascalCase();
  }
}

Stability stabilityFromJson(String j) {
  return Stability.values.byName(j.toCamelCase());
}

//////////////////////////////////////
/// Sequencing

enum Sequencing {
  noPreference,
  preferOrdered,
  ensureOrdered,
}

extension SequencingExt on Sequencing {
  String get json {
    return name.toPascalCase();
  }
}

Sequencing sequencingFromJson(String j) {
  return Sequencing.values.byName(j.toCamelCase());
}

//////////////////////////////////////
/// KeyBlob
class KeyBlob {
  final String key;
  final Uint8List blob;

  KeyBlob(this.key, this.blob);

  KeyBlob.fromJson(Map<String, dynamic> json)
      : key = json['key'],
        blob = base64Decode(json['blob']);

  Map<String, dynamic> get json {
    return {'key': key, 'blob': base64UrlEncode(blob)};
  }
}

//////////////////////////////////////
/// VeilidRoutingContext
abstract class VeilidRoutingContext {
  VeilidRoutingContext withPrivacy();
  VeilidRoutingContext withCustomPrivacy(Stability stability);
  VeilidRoutingContext withSequencing(Sequencing sequencing);
  Future<Uint8List> appCall(String target, Uint8List request);
  Future<void> appMessage(String target, Uint8List message);
}

//////////////////////////////////////
/// Veilid singleton factory

abstract class Veilid {
  static late Veilid instance = getVeilid();

  void initializeVeilidCore(Map<String, dynamic> platformConfigJson);
  void changeLogLevel(String layer, VeilidConfigLogLevel logLevel);
  Future<Stream<VeilidUpdate>> startupVeilidCore(VeilidConfig config);
  Future<VeilidState> getVeilidState();
  Future<void> attach();
  Future<void> detach();
  Future<void> shutdownVeilidCore();

  // Routing context
  Future<VeilidRoutingContext> routingContext();

  // Private route allocation
  Future<KeyBlob> newPrivateRoute();
  Future<KeyBlob> newCustomPrivateRoute(
      Stability stability, Sequencing sequencing);
  Future<String> importRemotePrivateRoute(Uint8List blob);
  Future<void> releasePrivateRoute(String key);

  // App calls
  Future<void> appCallReply(String id, Uint8List message);

  // Misc
  String veilidVersionString();
  VeilidVersion veilidVersion();
  Future<String> debug(String command);
}
