// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'veilid_config.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

_$_VeilidFFIConfigLoggingTerminal _$$_VeilidFFIConfigLoggingTerminalFromJson(
        Map<String, dynamic> json) =>
    _$_VeilidFFIConfigLoggingTerminal(
      enabled: json['enabled'] as bool,
      level: VeilidConfigLogLevel.fromJson(json['level']),
    );

Map<String, dynamic> _$$_VeilidFFIConfigLoggingTerminalToJson(
        _$_VeilidFFIConfigLoggingTerminal instance) =>
    <String, dynamic>{
      'enabled': instance.enabled,
      'level': instance.level.toJson(),
    };

_$_VeilidFFIConfigLoggingOtlp _$$_VeilidFFIConfigLoggingOtlpFromJson(
        Map<String, dynamic> json) =>
    _$_VeilidFFIConfigLoggingOtlp(
      enabled: json['enabled'] as bool,
      level: VeilidConfigLogLevel.fromJson(json['level']),
      grpcEndpoint: json['grpc_endpoint'] as String,
      serviceName: json['service_name'] as String,
    );

Map<String, dynamic> _$$_VeilidFFIConfigLoggingOtlpToJson(
        _$_VeilidFFIConfigLoggingOtlp instance) =>
    <String, dynamic>{
      'enabled': instance.enabled,
      'level': instance.level.toJson(),
      'grpc_endpoint': instance.grpcEndpoint,
      'service_name': instance.serviceName,
    };

_$_VeilidFFIConfigLoggingApi _$$_VeilidFFIConfigLoggingApiFromJson(
        Map<String, dynamic> json) =>
    _$_VeilidFFIConfigLoggingApi(
      enabled: json['enabled'] as bool,
      level: VeilidConfigLogLevel.fromJson(json['level']),
    );

Map<String, dynamic> _$$_VeilidFFIConfigLoggingApiToJson(
        _$_VeilidFFIConfigLoggingApi instance) =>
    <String, dynamic>{
      'enabled': instance.enabled,
      'level': instance.level.toJson(),
    };

_$_VeilidFFIConfigLogging _$$_VeilidFFIConfigLoggingFromJson(
        Map<String, dynamic> json) =>
    _$_VeilidFFIConfigLogging(
      terminal: VeilidFFIConfigLoggingTerminal.fromJson(json['terminal']),
      otlp: VeilidFFIConfigLoggingOtlp.fromJson(json['otlp']),
      api: VeilidFFIConfigLoggingApi.fromJson(json['api']),
    );

Map<String, dynamic> _$$_VeilidFFIConfigLoggingToJson(
        _$_VeilidFFIConfigLogging instance) =>
    <String, dynamic>{
      'terminal': instance.terminal.toJson(),
      'otlp': instance.otlp.toJson(),
      'api': instance.api.toJson(),
    };

_$_VeilidFFIConfig _$$_VeilidFFIConfigFromJson(Map<String, dynamic> json) =>
    _$_VeilidFFIConfig(
      logging: VeilidFFIConfigLogging.fromJson(json['logging']),
    );

Map<String, dynamic> _$$_VeilidFFIConfigToJson(_$_VeilidFFIConfig instance) =>
    <String, dynamic>{
      'logging': instance.logging.toJson(),
    };

_$_VeilidWASMConfigLoggingPerformance
    _$$_VeilidWASMConfigLoggingPerformanceFromJson(Map<String, dynamic> json) =>
        _$_VeilidWASMConfigLoggingPerformance(
          enabled: json['enabled'] as bool,
          level: VeilidConfigLogLevel.fromJson(json['level']),
          logsInTimings: json['logs_in_timings'] as bool,
          logsInConsole: json['logs_in_console'] as bool,
        );

Map<String, dynamic> _$$_VeilidWASMConfigLoggingPerformanceToJson(
        _$_VeilidWASMConfigLoggingPerformance instance) =>
    <String, dynamic>{
      'enabled': instance.enabled,
      'level': instance.level.toJson(),
      'logs_in_timings': instance.logsInTimings,
      'logs_in_console': instance.logsInConsole,
    };

_$_VeilidWASMConfigLoggingApi _$$_VeilidWASMConfigLoggingApiFromJson(
        Map<String, dynamic> json) =>
    _$_VeilidWASMConfigLoggingApi(
      enabled: json['enabled'] as bool,
      level: VeilidConfigLogLevel.fromJson(json['level']),
    );

Map<String, dynamic> _$$_VeilidWASMConfigLoggingApiToJson(
        _$_VeilidWASMConfigLoggingApi instance) =>
    <String, dynamic>{
      'enabled': instance.enabled,
      'level': instance.level.toJson(),
    };

_$_VeilidWASMConfigLogging _$$_VeilidWASMConfigLoggingFromJson(
        Map<String, dynamic> json) =>
    _$_VeilidWASMConfigLogging(
      performance:
          VeilidWASMConfigLoggingPerformance.fromJson(json['performance']),
      api: VeilidWASMConfigLoggingApi.fromJson(json['api']),
    );

Map<String, dynamic> _$$_VeilidWASMConfigLoggingToJson(
        _$_VeilidWASMConfigLogging instance) =>
    <String, dynamic>{
      'performance': instance.performance.toJson(),
      'api': instance.api.toJson(),
    };

_$_VeilidWASMConfig _$$_VeilidWASMConfigFromJson(Map<String, dynamic> json) =>
    _$_VeilidWASMConfig(
      logging: VeilidWASMConfigLogging.fromJson(json['logging']),
    );

Map<String, dynamic> _$$_VeilidWASMConfigToJson(_$_VeilidWASMConfig instance) =>
    <String, dynamic>{
      'logging': instance.logging.toJson(),
    };

_$_VeilidConfigHTTPS _$$_VeilidConfigHTTPSFromJson(Map<String, dynamic> json) =>
    _$_VeilidConfigHTTPS(
      enabled: json['enabled'] as bool,
      listenAddress: json['listen_address'] as String,
      path: json['path'] as String,
      url: json['url'] as String?,
    );

Map<String, dynamic> _$$_VeilidConfigHTTPSToJson(
        _$_VeilidConfigHTTPS instance) =>
    <String, dynamic>{
      'enabled': instance.enabled,
      'listen_address': instance.listenAddress,
      'path': instance.path,
      'url': instance.url,
    };

_$_VeilidConfigHTTP _$$_VeilidConfigHTTPFromJson(Map<String, dynamic> json) =>
    _$_VeilidConfigHTTP(
      enabled: json['enabled'] as bool,
      listenAddress: json['listen_address'] as String,
      path: json['path'] as String,
      url: json['url'] as String?,
    );

Map<String, dynamic> _$$_VeilidConfigHTTPToJson(_$_VeilidConfigHTTP instance) =>
    <String, dynamic>{
      'enabled': instance.enabled,
      'listen_address': instance.listenAddress,
      'path': instance.path,
      'url': instance.url,
    };

_$_VeilidConfigApplication _$$_VeilidConfigApplicationFromJson(
        Map<String, dynamic> json) =>
    _$_VeilidConfigApplication(
      https: VeilidConfigHTTPS.fromJson(json['https']),
      http: VeilidConfigHTTP.fromJson(json['http']),
    );

Map<String, dynamic> _$$_VeilidConfigApplicationToJson(
        _$_VeilidConfigApplication instance) =>
    <String, dynamic>{
      'https': instance.https.toJson(),
      'http': instance.http.toJson(),
    };

_$_VeilidConfigUDP _$$_VeilidConfigUDPFromJson(Map<String, dynamic> json) =>
    _$_VeilidConfigUDP(
      enabled: json['enabled'] as bool,
      socketPoolSize: json['socket_pool_size'] as int,
      listenAddress: json['listen_address'] as String,
      publicAddress: json['public_address'] as String?,
    );

Map<String, dynamic> _$$_VeilidConfigUDPToJson(_$_VeilidConfigUDP instance) =>
    <String, dynamic>{
      'enabled': instance.enabled,
      'socket_pool_size': instance.socketPoolSize,
      'listen_address': instance.listenAddress,
      'public_address': instance.publicAddress,
    };

_$_VeilidConfigTCP _$$_VeilidConfigTCPFromJson(Map<String, dynamic> json) =>
    _$_VeilidConfigTCP(
      connect: json['connect'] as bool,
      listen: json['listen'] as bool,
      maxConnections: json['max_connections'] as int,
      listenAddress: json['listen_address'] as String,
      publicAddress: json['public_address'] as String?,
    );

Map<String, dynamic> _$$_VeilidConfigTCPToJson(_$_VeilidConfigTCP instance) =>
    <String, dynamic>{
      'connect': instance.connect,
      'listen': instance.listen,
      'max_connections': instance.maxConnections,
      'listen_address': instance.listenAddress,
      'public_address': instance.publicAddress,
    };

_$_VeilidConfigWS _$$_VeilidConfigWSFromJson(Map<String, dynamic> json) =>
    _$_VeilidConfigWS(
      connect: json['connect'] as bool,
      listen: json['listen'] as bool,
      maxConnections: json['max_connections'] as int,
      listenAddress: json['listen_address'] as String,
      path: json['path'] as String,
      url: json['url'] as String?,
    );

Map<String, dynamic> _$$_VeilidConfigWSToJson(_$_VeilidConfigWS instance) =>
    <String, dynamic>{
      'connect': instance.connect,
      'listen': instance.listen,
      'max_connections': instance.maxConnections,
      'listen_address': instance.listenAddress,
      'path': instance.path,
      'url': instance.url,
    };

_$_VeilidConfigWSS _$$_VeilidConfigWSSFromJson(Map<String, dynamic> json) =>
    _$_VeilidConfigWSS(
      connect: json['connect'] as bool,
      listen: json['listen'] as bool,
      maxConnections: json['max_connections'] as int,
      listenAddress: json['listen_address'] as String,
      path: json['path'] as String,
      url: json['url'] as String?,
    );

Map<String, dynamic> _$$_VeilidConfigWSSToJson(_$_VeilidConfigWSS instance) =>
    <String, dynamic>{
      'connect': instance.connect,
      'listen': instance.listen,
      'max_connections': instance.maxConnections,
      'listen_address': instance.listenAddress,
      'path': instance.path,
      'url': instance.url,
    };

_$_VeilidConfigProtocol _$$_VeilidConfigProtocolFromJson(
        Map<String, dynamic> json) =>
    _$_VeilidConfigProtocol(
      udp: VeilidConfigUDP.fromJson(json['udp']),
      tcp: VeilidConfigTCP.fromJson(json['tcp']),
      ws: VeilidConfigWS.fromJson(json['ws']),
      wss: VeilidConfigWSS.fromJson(json['wss']),
    );

Map<String, dynamic> _$$_VeilidConfigProtocolToJson(
        _$_VeilidConfigProtocol instance) =>
    <String, dynamic>{
      'udp': instance.udp.toJson(),
      'tcp': instance.tcp.toJson(),
      'ws': instance.ws.toJson(),
      'wss': instance.wss.toJson(),
    };

_$_VeilidConfigTLS _$$_VeilidConfigTLSFromJson(Map<String, dynamic> json) =>
    _$_VeilidConfigTLS(
      certificatePath: json['certificate_path'] as String,
      privateKeyPath: json['private_key_path'] as String,
      connectionInitialTimeoutMs: json['connection_initial_timeout_ms'] as int,
    );

Map<String, dynamic> _$$_VeilidConfigTLSToJson(_$_VeilidConfigTLS instance) =>
    <String, dynamic>{
      'certificate_path': instance.certificatePath,
      'private_key_path': instance.privateKeyPath,
      'connection_initial_timeout_ms': instance.connectionInitialTimeoutMs,
    };

_$_VeilidConfigDHT _$$_VeilidConfigDHTFromJson(Map<String, dynamic> json) =>
    _$_VeilidConfigDHT(
      resolveNodeTimeoutMs: json['resolve_node_timeout_ms'] as int,
      resolveNodeCount: json['resolve_node_count'] as int,
      resolveNodeFanout: json['resolve_node_fanout'] as int,
      maxFindNodeCount: json['max_find_node_count'] as int,
      getValueTimeoutMs: json['get_value_timeout_ms'] as int,
      getValueCount: json['get_value_count'] as int,
      getValueFanout: json['get_value_fanout'] as int,
      setValueTimeoutMs: json['set_value_timeout_ms'] as int,
      setValueCount: json['set_value_count'] as int,
      setValueFanout: json['set_value_fanout'] as int,
      minPeerCount: json['min_peer_count'] as int,
      minPeerRefreshTimeMs: json['min_peer_refresh_time_ms'] as int,
      validateDialInfoReceiptTimeMs:
          json['validate_dial_info_receipt_time_ms'] as int,
      localSubkeyCacheSize: json['local_subkey_cache_size'] as int,
      localMaxSubkeyCacheMemoryMb:
          json['local_max_subkey_cache_memory_mb'] as int,
      remoteSubkeyCacheSize: json['remote_subkey_cache_size'] as int,
      remoteMaxRecords: json['remote_max_records'] as int,
      remoteMaxSubkeyCacheMemoryMb:
          json['remote_max_subkey_cache_memory_mb'] as int,
      remoteMaxStorageSpaceMb: json['remote_max_storage_space_mb'] as int,
      publicWatchLimit: json['public_watch_limit'] as int,
      memberWatchLimit: json['member_watch_limit'] as int,
      maxWatchExpirationMs: json['max_watch_expiration_ms'] as int,
    );

Map<String, dynamic> _$$_VeilidConfigDHTToJson(_$_VeilidConfigDHT instance) =>
    <String, dynamic>{
      'resolve_node_timeout_ms': instance.resolveNodeTimeoutMs,
      'resolve_node_count': instance.resolveNodeCount,
      'resolve_node_fanout': instance.resolveNodeFanout,
      'max_find_node_count': instance.maxFindNodeCount,
      'get_value_timeout_ms': instance.getValueTimeoutMs,
      'get_value_count': instance.getValueCount,
      'get_value_fanout': instance.getValueFanout,
      'set_value_timeout_ms': instance.setValueTimeoutMs,
      'set_value_count': instance.setValueCount,
      'set_value_fanout': instance.setValueFanout,
      'min_peer_count': instance.minPeerCount,
      'min_peer_refresh_time_ms': instance.minPeerRefreshTimeMs,
      'validate_dial_info_receipt_time_ms':
          instance.validateDialInfoReceiptTimeMs,
      'local_subkey_cache_size': instance.localSubkeyCacheSize,
      'local_max_subkey_cache_memory_mb': instance.localMaxSubkeyCacheMemoryMb,
      'remote_subkey_cache_size': instance.remoteSubkeyCacheSize,
      'remote_max_records': instance.remoteMaxRecords,
      'remote_max_subkey_cache_memory_mb':
          instance.remoteMaxSubkeyCacheMemoryMb,
      'remote_max_storage_space_mb': instance.remoteMaxStorageSpaceMb,
      'public_watch_limit': instance.publicWatchLimit,
      'member_watch_limit': instance.memberWatchLimit,
      'max_watch_expiration_ms': instance.maxWatchExpirationMs,
    };

_$_VeilidConfigRPC _$$_VeilidConfigRPCFromJson(Map<String, dynamic> json) =>
    _$_VeilidConfigRPC(
      concurrency: json['concurrency'] as int,
      queueSize: json['queue_size'] as int,
      timeoutMs: json['timeout_ms'] as int,
      maxRouteHopCount: json['max_route_hop_count'] as int,
      defaultRouteHopCount: json['default_route_hop_count'] as int,
      maxTimestampBehindMs: json['max_timestamp_behind_ms'] as int?,
      maxTimestampAheadMs: json['max_timestamp_ahead_ms'] as int?,
    );

Map<String, dynamic> _$$_VeilidConfigRPCToJson(_$_VeilidConfigRPC instance) =>
    <String, dynamic>{
      'concurrency': instance.concurrency,
      'queue_size': instance.queueSize,
      'timeout_ms': instance.timeoutMs,
      'max_route_hop_count': instance.maxRouteHopCount,
      'default_route_hop_count': instance.defaultRouteHopCount,
      'max_timestamp_behind_ms': instance.maxTimestampBehindMs,
      'max_timestamp_ahead_ms': instance.maxTimestampAheadMs,
    };

_$_VeilidConfigRoutingTable _$$_VeilidConfigRoutingTableFromJson(
        Map<String, dynamic> json) =>
    _$_VeilidConfigRoutingTable(
      nodeId: (json['node_id'] as List<dynamic>)
          .map(Typed<FixedEncodedString43>.fromJson)
          .toList(),
      nodeIdSecret: (json['node_id_secret'] as List<dynamic>)
          .map(Typed<FixedEncodedString43>.fromJson)
          .toList(),
      bootstrap:
          (json['bootstrap'] as List<dynamic>).map((e) => e as String).toList(),
      limitOverAttached: json['limit_over_attached'] as int,
      limitFullyAttached: json['limit_fully_attached'] as int,
      limitAttachedStrong: json['limit_attached_strong'] as int,
      limitAttachedGood: json['limit_attached_good'] as int,
      limitAttachedWeak: json['limit_attached_weak'] as int,
    );

Map<String, dynamic> _$$_VeilidConfigRoutingTableToJson(
        _$_VeilidConfigRoutingTable instance) =>
    <String, dynamic>{
      'node_id': instance.nodeId.map((e) => e.toJson()).toList(),
      'node_id_secret': instance.nodeIdSecret.map((e) => e.toJson()).toList(),
      'bootstrap': instance.bootstrap,
      'limit_over_attached': instance.limitOverAttached,
      'limit_fully_attached': instance.limitFullyAttached,
      'limit_attached_strong': instance.limitAttachedStrong,
      'limit_attached_good': instance.limitAttachedGood,
      'limit_attached_weak': instance.limitAttachedWeak,
    };

_$_VeilidConfigNetwork _$$_VeilidConfigNetworkFromJson(
        Map<String, dynamic> json) =>
    _$_VeilidConfigNetwork(
      connectionInitialTimeoutMs: json['connection_initial_timeout_ms'] as int,
      connectionInactivityTimeoutMs:
          json['connection_inactivity_timeout_ms'] as int,
      maxConnectionsPerIp4: json['max_connections_per_ip4'] as int,
      maxConnectionsPerIp6Prefix: json['max_connections_per_ip6_prefix'] as int,
      maxConnectionsPerIp6PrefixSize:
          json['max_connections_per_ip6_prefix_size'] as int,
      maxConnectionFrequencyPerMin:
          json['max_connection_frequency_per_min'] as int,
      clientAllowlistTimeoutMs: json['client_allowlist_timeout_ms'] as int,
      reverseConnectionReceiptTimeMs:
          json['reverse_connection_receipt_time_ms'] as int,
      holePunchReceiptTimeMs: json['hole_punch_receipt_time_ms'] as int,
      routingTable: VeilidConfigRoutingTable.fromJson(json['routing_table']),
      rpc: VeilidConfigRPC.fromJson(json['rpc']),
      dht: VeilidConfigDHT.fromJson(json['dht']),
      upnp: json['upnp'] as bool,
      detectAddressChanges: json['detect_address_changes'] as bool,
      restrictedNatRetries: json['restricted_nat_retries'] as int,
      tls: VeilidConfigTLS.fromJson(json['tls']),
      application: VeilidConfigApplication.fromJson(json['application']),
      protocol: VeilidConfigProtocol.fromJson(json['protocol']),
      networkKeyPassword: json['network_key_password'] as String?,
    );

Map<String, dynamic> _$$_VeilidConfigNetworkToJson(
        _$_VeilidConfigNetwork instance) =>
    <String, dynamic>{
      'connection_initial_timeout_ms': instance.connectionInitialTimeoutMs,
      'connection_inactivity_timeout_ms':
          instance.connectionInactivityTimeoutMs,
      'max_connections_per_ip4': instance.maxConnectionsPerIp4,
      'max_connections_per_ip6_prefix': instance.maxConnectionsPerIp6Prefix,
      'max_connections_per_ip6_prefix_size':
          instance.maxConnectionsPerIp6PrefixSize,
      'max_connection_frequency_per_min': instance.maxConnectionFrequencyPerMin,
      'client_allowlist_timeout_ms': instance.clientAllowlistTimeoutMs,
      'reverse_connection_receipt_time_ms':
          instance.reverseConnectionReceiptTimeMs,
      'hole_punch_receipt_time_ms': instance.holePunchReceiptTimeMs,
      'routing_table': instance.routingTable.toJson(),
      'rpc': instance.rpc.toJson(),
      'dht': instance.dht.toJson(),
      'upnp': instance.upnp,
      'detect_address_changes': instance.detectAddressChanges,
      'restricted_nat_retries': instance.restrictedNatRetries,
      'tls': instance.tls.toJson(),
      'application': instance.application.toJson(),
      'protocol': instance.protocol.toJson(),
      'network_key_password': instance.networkKeyPassword,
    };

_$_VeilidConfigTableStore _$$_VeilidConfigTableStoreFromJson(
        Map<String, dynamic> json) =>
    _$_VeilidConfigTableStore(
      directory: json['directory'] as String,
      delete: json['delete'] as bool,
    );

Map<String, dynamic> _$$_VeilidConfigTableStoreToJson(
        _$_VeilidConfigTableStore instance) =>
    <String, dynamic>{
      'directory': instance.directory,
      'delete': instance.delete,
    };

_$_VeilidConfigBlockStore _$$_VeilidConfigBlockStoreFromJson(
        Map<String, dynamic> json) =>
    _$_VeilidConfigBlockStore(
      directory: json['directory'] as String,
      delete: json['delete'] as bool,
    );

Map<String, dynamic> _$$_VeilidConfigBlockStoreToJson(
        _$_VeilidConfigBlockStore instance) =>
    <String, dynamic>{
      'directory': instance.directory,
      'delete': instance.delete,
    };

_$_VeilidConfigProtectedStore _$$_VeilidConfigProtectedStoreFromJson(
        Map<String, dynamic> json) =>
    _$_VeilidConfigProtectedStore(
      allowInsecureFallback: json['allow_insecure_fallback'] as bool,
      alwaysUseInsecureStorage: json['always_use_insecure_storage'] as bool,
      directory: json['directory'] as String,
      delete: json['delete'] as bool,
      deviceEncryptionKeyPassword:
          json['device_encryption_key_password'] as String,
      newDeviceEncryptionKeyPassword:
          json['new_device_encryption_key_password'] as String?,
    );

Map<String, dynamic> _$$_VeilidConfigProtectedStoreToJson(
        _$_VeilidConfigProtectedStore instance) =>
    <String, dynamic>{
      'allow_insecure_fallback': instance.allowInsecureFallback,
      'always_use_insecure_storage': instance.alwaysUseInsecureStorage,
      'directory': instance.directory,
      'delete': instance.delete,
      'device_encryption_key_password': instance.deviceEncryptionKeyPassword,
      'new_device_encryption_key_password':
          instance.newDeviceEncryptionKeyPassword,
    };

_$_VeilidConfigCapabilities _$$_VeilidConfigCapabilitiesFromJson(
        Map<String, dynamic> json) =>
    _$_VeilidConfigCapabilities(
      disable:
          (json['disable'] as List<dynamic>).map((e) => e as String).toList(),
    );

Map<String, dynamic> _$$_VeilidConfigCapabilitiesToJson(
        _$_VeilidConfigCapabilities instance) =>
    <String, dynamic>{
      'disable': instance.disable,
    };

_$_VeilidConfig _$$_VeilidConfigFromJson(Map<String, dynamic> json) =>
    _$_VeilidConfig(
      programName: json['program_name'] as String,
      namespace: json['namespace'] as String,
      capabilities: VeilidConfigCapabilities.fromJson(json['capabilities']),
      protectedStore:
          VeilidConfigProtectedStore.fromJson(json['protected_store']),
      tableStore: VeilidConfigTableStore.fromJson(json['table_store']),
      blockStore: VeilidConfigBlockStore.fromJson(json['block_store']),
      network: VeilidConfigNetwork.fromJson(json['network']),
    );

Map<String, dynamic> _$$_VeilidConfigToJson(_$_VeilidConfig instance) =>
    <String, dynamic>{
      'program_name': instance.programName,
      'namespace': instance.namespace,
      'capabilities': instance.capabilities.toJson(),
      'protected_store': instance.protectedStore.toJson(),
      'table_store': instance.tableStore.toJson(),
      'block_store': instance.blockStore.toJson(),
      'network': instance.network.toJson(),
    };
