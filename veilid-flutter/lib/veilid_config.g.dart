// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'veilid_config.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

_VeilidFFIConfigLoggingTerminal _$VeilidFFIConfigLoggingTerminalFromJson(
        Map<String, dynamic> json) =>
    _VeilidFFIConfigLoggingTerminal(
      enabled: json['enabled'] as bool,
      level: VeilidConfigLogLevel.fromJson(json['level']),
      ignoreLogTargets: (json['ignore_log_targets'] as List<dynamic>?)
              ?.map((e) => e as String)
              .toList() ??
          const [],
    );

Map<String, dynamic> _$VeilidFFIConfigLoggingTerminalToJson(
        _VeilidFFIConfigLoggingTerminal instance) =>
    <String, dynamic>{
      'enabled': instance.enabled,
      'level': instance.level.toJson(),
      'ignore_log_targets': instance.ignoreLogTargets,
    };

_VeilidFFIConfigLoggingOtlp _$VeilidFFIConfigLoggingOtlpFromJson(
        Map<String, dynamic> json) =>
    _VeilidFFIConfigLoggingOtlp(
      enabled: json['enabled'] as bool,
      level: VeilidConfigLogLevel.fromJson(json['level']),
      grpcEndpoint: json['grpc_endpoint'] as String,
      serviceName: json['service_name'] as String,
      ignoreLogTargets: (json['ignore_log_targets'] as List<dynamic>?)
              ?.map((e) => e as String)
              .toList() ??
          const [],
    );

Map<String, dynamic> _$VeilidFFIConfigLoggingOtlpToJson(
        _VeilidFFIConfigLoggingOtlp instance) =>
    <String, dynamic>{
      'enabled': instance.enabled,
      'level': instance.level.toJson(),
      'grpc_endpoint': instance.grpcEndpoint,
      'service_name': instance.serviceName,
      'ignore_log_targets': instance.ignoreLogTargets,
    };

_VeilidFFIConfigLoggingApi _$VeilidFFIConfigLoggingApiFromJson(
        Map<String, dynamic> json) =>
    _VeilidFFIConfigLoggingApi(
      enabled: json['enabled'] as bool,
      level: VeilidConfigLogLevel.fromJson(json['level']),
      ignoreLogTargets: (json['ignore_log_targets'] as List<dynamic>?)
              ?.map((e) => e as String)
              .toList() ??
          const [],
    );

Map<String, dynamic> _$VeilidFFIConfigLoggingApiToJson(
        _VeilidFFIConfigLoggingApi instance) =>
    <String, dynamic>{
      'enabled': instance.enabled,
      'level': instance.level.toJson(),
      'ignore_log_targets': instance.ignoreLogTargets,
    };

_VeilidFFIConfigLoggingFlame _$VeilidFFIConfigLoggingFlameFromJson(
        Map<String, dynamic> json) =>
    _VeilidFFIConfigLoggingFlame(
      enabled: json['enabled'] as bool,
      path: json['path'] as String,
    );

Map<String, dynamic> _$VeilidFFIConfigLoggingFlameToJson(
        _VeilidFFIConfigLoggingFlame instance) =>
    <String, dynamic>{
      'enabled': instance.enabled,
      'path': instance.path,
    };

_VeilidFFIConfigLogging _$VeilidFFIConfigLoggingFromJson(
        Map<String, dynamic> json) =>
    _VeilidFFIConfigLogging(
      terminal: VeilidFFIConfigLoggingTerminal.fromJson(json['terminal']),
      otlp: VeilidFFIConfigLoggingOtlp.fromJson(json['otlp']),
      api: VeilidFFIConfigLoggingApi.fromJson(json['api']),
      flame: VeilidFFIConfigLoggingFlame.fromJson(json['flame']),
    );

Map<String, dynamic> _$VeilidFFIConfigLoggingToJson(
        _VeilidFFIConfigLogging instance) =>
    <String, dynamic>{
      'terminal': instance.terminal.toJson(),
      'otlp': instance.otlp.toJson(),
      'api': instance.api.toJson(),
      'flame': instance.flame.toJson(),
    };

_VeilidFFIConfig _$VeilidFFIConfigFromJson(Map<String, dynamic> json) =>
    _VeilidFFIConfig(
      logging: VeilidFFIConfigLogging.fromJson(json['logging']),
    );

Map<String, dynamic> _$VeilidFFIConfigToJson(_VeilidFFIConfig instance) =>
    <String, dynamic>{
      'logging': instance.logging.toJson(),
    };

_VeilidWASMConfigLoggingPerformance
    _$VeilidWASMConfigLoggingPerformanceFromJson(Map<String, dynamic> json) =>
        _VeilidWASMConfigLoggingPerformance(
          enabled: json['enabled'] as bool,
          level: VeilidConfigLogLevel.fromJson(json['level']),
          logsInTimings: json['logs_in_timings'] as bool,
          logsInConsole: json['logs_in_console'] as bool,
          ignoreLogTargets: (json['ignore_log_targets'] as List<dynamic>?)
                  ?.map((e) => e as String)
                  .toList() ??
              const [],
        );

Map<String, dynamic> _$VeilidWASMConfigLoggingPerformanceToJson(
        _VeilidWASMConfigLoggingPerformance instance) =>
    <String, dynamic>{
      'enabled': instance.enabled,
      'level': instance.level.toJson(),
      'logs_in_timings': instance.logsInTimings,
      'logs_in_console': instance.logsInConsole,
      'ignore_log_targets': instance.ignoreLogTargets,
    };

_VeilidWASMConfigLoggingApi _$VeilidWASMConfigLoggingApiFromJson(
        Map<String, dynamic> json) =>
    _VeilidWASMConfigLoggingApi(
      enabled: json['enabled'] as bool,
      level: VeilidConfigLogLevel.fromJson(json['level']),
      ignoreLogTargets: (json['ignore_log_targets'] as List<dynamic>?)
              ?.map((e) => e as String)
              .toList() ??
          const [],
    );

Map<String, dynamic> _$VeilidWASMConfigLoggingApiToJson(
        _VeilidWASMConfigLoggingApi instance) =>
    <String, dynamic>{
      'enabled': instance.enabled,
      'level': instance.level.toJson(),
      'ignore_log_targets': instance.ignoreLogTargets,
    };

_VeilidWASMConfigLogging _$VeilidWASMConfigLoggingFromJson(
        Map<String, dynamic> json) =>
    _VeilidWASMConfigLogging(
      performance:
          VeilidWASMConfigLoggingPerformance.fromJson(json['performance']),
      api: VeilidWASMConfigLoggingApi.fromJson(json['api']),
    );

Map<String, dynamic> _$VeilidWASMConfigLoggingToJson(
        _VeilidWASMConfigLogging instance) =>
    <String, dynamic>{
      'performance': instance.performance.toJson(),
      'api': instance.api.toJson(),
    };

_VeilidWASMConfig _$VeilidWASMConfigFromJson(Map<String, dynamic> json) =>
    _VeilidWASMConfig(
      logging: VeilidWASMConfigLogging.fromJson(json['logging']),
    );

Map<String, dynamic> _$VeilidWASMConfigToJson(_VeilidWASMConfig instance) =>
    <String, dynamic>{
      'logging': instance.logging.toJson(),
    };

_VeilidConfigHTTPS _$VeilidConfigHTTPSFromJson(Map<String, dynamic> json) =>
    _VeilidConfigHTTPS(
      enabled: json['enabled'] as bool,
      listenAddress: json['listen_address'] as String,
      path: json['path'] as String,
      url: json['url'] as String?,
    );

Map<String, dynamic> _$VeilidConfigHTTPSToJson(_VeilidConfigHTTPS instance) =>
    <String, dynamic>{
      'enabled': instance.enabled,
      'listen_address': instance.listenAddress,
      'path': instance.path,
      'url': instance.url,
    };

_VeilidConfigHTTP _$VeilidConfigHTTPFromJson(Map<String, dynamic> json) =>
    _VeilidConfigHTTP(
      enabled: json['enabled'] as bool,
      listenAddress: json['listen_address'] as String,
      path: json['path'] as String,
      url: json['url'] as String?,
    );

Map<String, dynamic> _$VeilidConfigHTTPToJson(_VeilidConfigHTTP instance) =>
    <String, dynamic>{
      'enabled': instance.enabled,
      'listen_address': instance.listenAddress,
      'path': instance.path,
      'url': instance.url,
    };

_VeilidConfigApplication _$VeilidConfigApplicationFromJson(
        Map<String, dynamic> json) =>
    _VeilidConfigApplication(
      https: VeilidConfigHTTPS.fromJson(json['https']),
      http: VeilidConfigHTTP.fromJson(json['http']),
    );

Map<String, dynamic> _$VeilidConfigApplicationToJson(
        _VeilidConfigApplication instance) =>
    <String, dynamic>{
      'https': instance.https.toJson(),
      'http': instance.http.toJson(),
    };

_VeilidConfigUDP _$VeilidConfigUDPFromJson(Map<String, dynamic> json) =>
    _VeilidConfigUDP(
      enabled: json['enabled'] as bool,
      socketPoolSize: (json['socket_pool_size'] as num).toInt(),
      listenAddress: json['listen_address'] as String,
      publicAddress: json['public_address'] as String?,
    );

Map<String, dynamic> _$VeilidConfigUDPToJson(_VeilidConfigUDP instance) =>
    <String, dynamic>{
      'enabled': instance.enabled,
      'socket_pool_size': instance.socketPoolSize,
      'listen_address': instance.listenAddress,
      'public_address': instance.publicAddress,
    };

_VeilidConfigTCP _$VeilidConfigTCPFromJson(Map<String, dynamic> json) =>
    _VeilidConfigTCP(
      connect: json['connect'] as bool,
      listen: json['listen'] as bool,
      maxConnections: (json['max_connections'] as num).toInt(),
      listenAddress: json['listen_address'] as String,
      publicAddress: json['public_address'] as String?,
    );

Map<String, dynamic> _$VeilidConfigTCPToJson(_VeilidConfigTCP instance) =>
    <String, dynamic>{
      'connect': instance.connect,
      'listen': instance.listen,
      'max_connections': instance.maxConnections,
      'listen_address': instance.listenAddress,
      'public_address': instance.publicAddress,
    };

_VeilidConfigWS _$VeilidConfigWSFromJson(Map<String, dynamic> json) =>
    _VeilidConfigWS(
      connect: json['connect'] as bool,
      listen: json['listen'] as bool,
      maxConnections: (json['max_connections'] as num).toInt(),
      listenAddress: json['listen_address'] as String,
      path: json['path'] as String,
      url: json['url'] as String?,
    );

Map<String, dynamic> _$VeilidConfigWSToJson(_VeilidConfigWS instance) =>
    <String, dynamic>{
      'connect': instance.connect,
      'listen': instance.listen,
      'max_connections': instance.maxConnections,
      'listen_address': instance.listenAddress,
      'path': instance.path,
      'url': instance.url,
    };

_VeilidConfigWSS _$VeilidConfigWSSFromJson(Map<String, dynamic> json) =>
    _VeilidConfigWSS(
      connect: json['connect'] as bool,
      listen: json['listen'] as bool,
      maxConnections: (json['max_connections'] as num).toInt(),
      listenAddress: json['listen_address'] as String,
      path: json['path'] as String,
      url: json['url'] as String?,
    );

Map<String, dynamic> _$VeilidConfigWSSToJson(_VeilidConfigWSS instance) =>
    <String, dynamic>{
      'connect': instance.connect,
      'listen': instance.listen,
      'max_connections': instance.maxConnections,
      'listen_address': instance.listenAddress,
      'path': instance.path,
      'url': instance.url,
    };

_VeilidConfigProtocol _$VeilidConfigProtocolFromJson(
        Map<String, dynamic> json) =>
    _VeilidConfigProtocol(
      udp: VeilidConfigUDP.fromJson(json['udp']),
      tcp: VeilidConfigTCP.fromJson(json['tcp']),
      ws: VeilidConfigWS.fromJson(json['ws']),
      wss: VeilidConfigWSS.fromJson(json['wss']),
    );

Map<String, dynamic> _$VeilidConfigProtocolToJson(
        _VeilidConfigProtocol instance) =>
    <String, dynamic>{
      'udp': instance.udp.toJson(),
      'tcp': instance.tcp.toJson(),
      'ws': instance.ws.toJson(),
      'wss': instance.wss.toJson(),
    };

_VeilidConfigTLS _$VeilidConfigTLSFromJson(Map<String, dynamic> json) =>
    _VeilidConfigTLS(
      certificatePath: json['certificate_path'] as String,
      privateKeyPath: json['private_key_path'] as String,
      connectionInitialTimeoutMs:
          (json['connection_initial_timeout_ms'] as num).toInt(),
    );

Map<String, dynamic> _$VeilidConfigTLSToJson(_VeilidConfigTLS instance) =>
    <String, dynamic>{
      'certificate_path': instance.certificatePath,
      'private_key_path': instance.privateKeyPath,
      'connection_initial_timeout_ms': instance.connectionInitialTimeoutMs,
    };

_VeilidConfigDHT _$VeilidConfigDHTFromJson(Map<String, dynamic> json) =>
    _VeilidConfigDHT(
      resolveNodeTimeoutMs: (json['resolve_node_timeout_ms'] as num).toInt(),
      resolveNodeCount: (json['resolve_node_count'] as num).toInt(),
      resolveNodeFanout: (json['resolve_node_fanout'] as num).toInt(),
      maxFindNodeCount: (json['max_find_node_count'] as num).toInt(),
      getValueTimeoutMs: (json['get_value_timeout_ms'] as num).toInt(),
      getValueCount: (json['get_value_count'] as num).toInt(),
      getValueFanout: (json['get_value_fanout'] as num).toInt(),
      setValueTimeoutMs: (json['set_value_timeout_ms'] as num).toInt(),
      setValueCount: (json['set_value_count'] as num).toInt(),
      setValueFanout: (json['set_value_fanout'] as num).toInt(),
      minPeerCount: (json['min_peer_count'] as num).toInt(),
      minPeerRefreshTimeMs: (json['min_peer_refresh_time_ms'] as num).toInt(),
      validateDialInfoReceiptTimeMs:
          (json['validate_dial_info_receipt_time_ms'] as num).toInt(),
      localSubkeyCacheSize: (json['local_subkey_cache_size'] as num).toInt(),
      localMaxSubkeyCacheMemoryMb:
          (json['local_max_subkey_cache_memory_mb'] as num).toInt(),
      remoteSubkeyCacheSize: (json['remote_subkey_cache_size'] as num).toInt(),
      remoteMaxRecords: (json['remote_max_records'] as num).toInt(),
      remoteMaxSubkeyCacheMemoryMb:
          (json['remote_max_subkey_cache_memory_mb'] as num).toInt(),
      remoteMaxStorageSpaceMb:
          (json['remote_max_storage_space_mb'] as num).toInt(),
      publicWatchLimit: (json['public_watch_limit'] as num).toInt(),
      memberWatchLimit: (json['member_watch_limit'] as num).toInt(),
      maxWatchExpirationMs: (json['max_watch_expiration_ms'] as num).toInt(),
    );

Map<String, dynamic> _$VeilidConfigDHTToJson(_VeilidConfigDHT instance) =>
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

_VeilidConfigRPC _$VeilidConfigRPCFromJson(Map<String, dynamic> json) =>
    _VeilidConfigRPC(
      concurrency: (json['concurrency'] as num).toInt(),
      queueSize: (json['queue_size'] as num).toInt(),
      timeoutMs: (json['timeout_ms'] as num).toInt(),
      maxRouteHopCount: (json['max_route_hop_count'] as num).toInt(),
      defaultRouteHopCount: (json['default_route_hop_count'] as num).toInt(),
      maxTimestampBehindMs: (json['max_timestamp_behind_ms'] as num?)?.toInt(),
      maxTimestampAheadMs: (json['max_timestamp_ahead_ms'] as num?)?.toInt(),
    );

Map<String, dynamic> _$VeilidConfigRPCToJson(_VeilidConfigRPC instance) =>
    <String, dynamic>{
      'concurrency': instance.concurrency,
      'queue_size': instance.queueSize,
      'timeout_ms': instance.timeoutMs,
      'max_route_hop_count': instance.maxRouteHopCount,
      'default_route_hop_count': instance.defaultRouteHopCount,
      'max_timestamp_behind_ms': instance.maxTimestampBehindMs,
      'max_timestamp_ahead_ms': instance.maxTimestampAheadMs,
    };

_VeilidConfigRoutingTable _$VeilidConfigRoutingTableFromJson(
        Map<String, dynamic> json) =>
    _VeilidConfigRoutingTable(
      nodeId: (json['node_id'] as List<dynamic>)
          .map(Typed<FixedEncodedString43>.fromJson)
          .toList(),
      nodeIdSecret: (json['node_id_secret'] as List<dynamic>)
          .map(Typed<FixedEncodedString43>.fromJson)
          .toList(),
      bootstrap:
          (json['bootstrap'] as List<dynamic>).map((e) => e as String).toList(),
      bootstrapKeys: (json['bootstrap_keys'] as List<dynamic>)
          .map(Typed<FixedEncodedString43>.fromJson)
          .toList(),
      limitOverAttached: (json['limit_over_attached'] as num).toInt(),
      limitFullyAttached: (json['limit_fully_attached'] as num).toInt(),
      limitAttachedStrong: (json['limit_attached_strong'] as num).toInt(),
      limitAttachedGood: (json['limit_attached_good'] as num).toInt(),
      limitAttachedWeak: (json['limit_attached_weak'] as num).toInt(),
    );

Map<String, dynamic> _$VeilidConfigRoutingTableToJson(
        _VeilidConfigRoutingTable instance) =>
    <String, dynamic>{
      'node_id': instance.nodeId.map((e) => e.toJson()).toList(),
      'node_id_secret': instance.nodeIdSecret.map((e) => e.toJson()).toList(),
      'bootstrap': instance.bootstrap,
      'bootstrap_keys': instance.bootstrapKeys.map((e) => e.toJson()).toList(),
      'limit_over_attached': instance.limitOverAttached,
      'limit_fully_attached': instance.limitFullyAttached,
      'limit_attached_strong': instance.limitAttachedStrong,
      'limit_attached_good': instance.limitAttachedGood,
      'limit_attached_weak': instance.limitAttachedWeak,
    };

_VeilidConfigNetwork _$VeilidConfigNetworkFromJson(Map<String, dynamic> json) =>
    _VeilidConfigNetwork(
      connectionInitialTimeoutMs:
          (json['connection_initial_timeout_ms'] as num).toInt(),
      connectionInactivityTimeoutMs:
          (json['connection_inactivity_timeout_ms'] as num).toInt(),
      maxConnectionsPerIp4: (json['max_connections_per_ip4'] as num).toInt(),
      maxConnectionsPerIp6Prefix:
          (json['max_connections_per_ip6_prefix'] as num).toInt(),
      maxConnectionsPerIp6PrefixSize:
          (json['max_connections_per_ip6_prefix_size'] as num).toInt(),
      maxConnectionFrequencyPerMin:
          (json['max_connection_frequency_per_min'] as num).toInt(),
      clientAllowlistTimeoutMs:
          (json['client_allowlist_timeout_ms'] as num).toInt(),
      reverseConnectionReceiptTimeMs:
          (json['reverse_connection_receipt_time_ms'] as num).toInt(),
      holePunchReceiptTimeMs:
          (json['hole_punch_receipt_time_ms'] as num).toInt(),
      routingTable: VeilidConfigRoutingTable.fromJson(json['routing_table']),
      rpc: VeilidConfigRPC.fromJson(json['rpc']),
      dht: VeilidConfigDHT.fromJson(json['dht']),
      upnp: json['upnp'] as bool,
      detectAddressChanges: json['detect_address_changes'] as bool,
      restrictedNatRetries: (json['restricted_nat_retries'] as num).toInt(),
      tls: VeilidConfigTLS.fromJson(json['tls']),
      application: VeilidConfigApplication.fromJson(json['application']),
      protocol: VeilidConfigProtocol.fromJson(json['protocol']),
      networkKeyPassword: json['network_key_password'] as String?,
    );

Map<String, dynamic> _$VeilidConfigNetworkToJson(
        _VeilidConfigNetwork instance) =>
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

_VeilidConfigTableStore _$VeilidConfigTableStoreFromJson(
        Map<String, dynamic> json) =>
    _VeilidConfigTableStore(
      directory: json['directory'] as String,
      delete: json['delete'] as bool,
    );

Map<String, dynamic> _$VeilidConfigTableStoreToJson(
        _VeilidConfigTableStore instance) =>
    <String, dynamic>{
      'directory': instance.directory,
      'delete': instance.delete,
    };

_VeilidConfigBlockStore _$VeilidConfigBlockStoreFromJson(
        Map<String, dynamic> json) =>
    _VeilidConfigBlockStore(
      directory: json['directory'] as String,
      delete: json['delete'] as bool,
    );

Map<String, dynamic> _$VeilidConfigBlockStoreToJson(
        _VeilidConfigBlockStore instance) =>
    <String, dynamic>{
      'directory': instance.directory,
      'delete': instance.delete,
    };

_VeilidConfigProtectedStore _$VeilidConfigProtectedStoreFromJson(
        Map<String, dynamic> json) =>
    _VeilidConfigProtectedStore(
      allowInsecureFallback: json['allow_insecure_fallback'] as bool,
      alwaysUseInsecureStorage: json['always_use_insecure_storage'] as bool,
      directory: json['directory'] as String,
      delete: json['delete'] as bool,
      deviceEncryptionKeyPassword:
          json['device_encryption_key_password'] as String,
      newDeviceEncryptionKeyPassword:
          json['new_device_encryption_key_password'] as String?,
    );

Map<String, dynamic> _$VeilidConfigProtectedStoreToJson(
        _VeilidConfigProtectedStore instance) =>
    <String, dynamic>{
      'allow_insecure_fallback': instance.allowInsecureFallback,
      'always_use_insecure_storage': instance.alwaysUseInsecureStorage,
      'directory': instance.directory,
      'delete': instance.delete,
      'device_encryption_key_password': instance.deviceEncryptionKeyPassword,
      'new_device_encryption_key_password':
          instance.newDeviceEncryptionKeyPassword,
    };

_VeilidConfigCapabilities _$VeilidConfigCapabilitiesFromJson(
        Map<String, dynamic> json) =>
    _VeilidConfigCapabilities(
      disable:
          (json['disable'] as List<dynamic>).map((e) => e as String).toList(),
    );

Map<String, dynamic> _$VeilidConfigCapabilitiesToJson(
        _VeilidConfigCapabilities instance) =>
    <String, dynamic>{
      'disable': instance.disable,
    };

_VeilidConfig _$VeilidConfigFromJson(Map<String, dynamic> json) =>
    _VeilidConfig(
      programName: json['program_name'] as String,
      namespace: json['namespace'] as String,
      capabilities: VeilidConfigCapabilities.fromJson(json['capabilities']),
      protectedStore:
          VeilidConfigProtectedStore.fromJson(json['protected_store']),
      tableStore: VeilidConfigTableStore.fromJson(json['table_store']),
      blockStore: VeilidConfigBlockStore.fromJson(json['block_store']),
      network: VeilidConfigNetwork.fromJson(json['network']),
    );

Map<String, dynamic> _$VeilidConfigToJson(_VeilidConfig instance) =>
    <String, dynamic>{
      'program_name': instance.programName,
      'namespace': instance.namespace,
      'capabilities': instance.capabilities.toJson(),
      'protected_store': instance.protectedStore.toJson(),
      'table_store': instance.tableStore.toJson(),
      'block_store': instance.blockStore.toJson(),
      'network': instance.network.toJson(),
    };
