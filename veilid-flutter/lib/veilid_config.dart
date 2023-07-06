import 'package:freezed_annotation/freezed_annotation.dart';
import 'package:flutter/foundation.dart';
import 'package:change_case/change_case.dart';
import 'veilid.dart';
import 'veilid_encoding.dart';
import 'veilid_crypto.dart';

part 'veilid_config.freezed.dart';
part 'veilid_config.g.dart';

//////////////////////////////////////////////////////////
// FFI Platform-specific config
@freezed
class VeilidFFIConfigLoggingTerminal with _$VeilidFFIConfigLoggingTerminal {
  const factory VeilidFFIConfigLoggingTerminal({
    required bool enabled,
    required VeilidConfigLogLevel level,
  }) = _VeilidFFIConfigLoggingTerminal;

  factory VeilidFFIConfigLoggingTerminal.fromJson(Map<String, dynamic> json) =>
      _$VeilidFFIConfigLoggingTerminalFromJson(json);
}

@freezed
class VeilidFFIConfigLoggingOtlp with _$VeilidFFIConfigLoggingOtlp {
  const factory VeilidFFIConfigLoggingOtlp({
    required bool enabled,
    required VeilidConfigLogLevel level,
    required String grpcEndpoint,
    required String serviceName,
  }) = _VeilidFFIConfigLoggingOtlp;

  factory VeilidFFIConfigLoggingOtlp.fromJson(Map<String, dynamic> json) =>
      _$VeilidFFIConfigLoggingOtlpFromJson(json);
}

@freezed
class VeilidFFIConfigLoggingApi with _$VeilidFFIConfigLoggingApi {
  const factory VeilidFFIConfigLoggingApi({
    required bool enabled,
    required VeilidConfigLogLevel level,
  }) = _VeilidFFIConfigLoggingApi;

  factory VeilidFFIConfigLoggingApi.fromJson(Map<String, dynamic> json) =>
      _$VeilidFFIConfigLoggingApiFromJson(json);
}

@freezed
class VeilidFFIConfigLogging with _$VeilidFFIConfigLogging {
  const factory VeilidFFIConfigLogging(
      {required VeilidFFIConfigLoggingTerminal terminal,
      required VeilidFFIConfigLoggingOtlp otlp,
      required VeilidFFIConfigLoggingApi api}) = _VeilidFFIConfigLogging;

  factory VeilidFFIConfigLogging.fromJson(Map<String, dynamic> json) =>
      _$VeilidFFIConfigLoggingFromJson(json);
}

@freezed
class VeilidFFIConfig with _$VeilidFFIConfig {
  const factory VeilidFFIConfig({
    required VeilidFFIConfigLogging logging,
  }) = _VeilidFFIConfig;

  factory VeilidFFIConfig.fromJson(Map<String, dynamic> json) =>
      _$VeilidFFIConfigFromJson(json);
}

//////////////////////////////////////////////////////////
// WASM Platform-specific config

@freezed
class VeilidWASMConfigLoggingPerformance
    with _$VeilidWASMConfigLoggingPerformance {
  const factory VeilidWASMConfigLoggingPerformance({
    required bool enabled,
    required VeilidConfigLogLevel level,
    required bool logsInTimings,
    required bool logsInConsole,
  }) = _VeilidWASMConfigLoggingPerformance;

  factory VeilidWASMConfigLoggingPerformance.fromJson(
          Map<String, dynamic> json) =>
      _$VeilidWASMConfigLoggingPerformanceFromJson(json);
}

@freezed
class VeilidWASMConfigLoggingApi with _$VeilidWASMConfigLoggingApi {
  const factory VeilidWASMConfigLoggingApi({
    required bool enabled,
    required VeilidConfigLogLevel level,
  }) = _VeilidWASMConfigLoggingApi;

  factory VeilidWASMConfigLoggingApi.fromJson(Map<String, dynamic> json) =>
      _$VeilidWASMConfigLoggingApiFromJson(json);
}

@freezed
class VeilidWASMConfigLogging with _$VeilidWASMConfigLogging {
  const factory VeilidWASMConfigLogging(
      {required VeilidWASMConfigLoggingPerformance performance,
      required VeilidWASMConfigLoggingApi api}) = _VeilidWASMConfigLogging;

  factory VeilidWASMConfigLogging.fromJson(Map<String, dynamic> json) =>
      _$VeilidWASMConfigLoggingFromJson(json);
}

@freezed
class VeilidWASMConfig with _$VeilidWASMConfig {
  const factory VeilidWASMConfig({
    required VeilidWASMConfigLogging logging,
  }) = _VeilidWASMConfig;

  factory VeilidWASMConfig.fromJson(Map<String, dynamic> json) =>
      _$VeilidWASMConfigFromJson(json);
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

//////////////////////////////////////
/// VeilidConfig

@freezed
class VeilidConfigHTTPS with _$VeilidConfigHTTPS {
  const factory VeilidConfigHTTPS({
    required bool enabled,
    required String listenAddress,
    required String path,
    String? url,
  }) = _VeilidConfigHTTPS;

  factory VeilidConfigHTTPS.fromJson(Map<String, dynamic> json) =>
      _$VeilidConfigHTTPSFromJson(json);
}

////////////

@freezed
class VeilidConfigHTTP with _$VeilidConfigHTTP {
  const factory VeilidConfigHTTP({
    required bool enabled,
    required String listenAddress,
    required String path,
    String? url,
  }) = _VeilidConfigHTTP;

  factory VeilidConfigHTTP.fromJson(Map<String, dynamic> json) =>
      _$VeilidConfigHTTPFromJson(json);
}

////////////

@freezed
class VeilidConfigApplication with _$VeilidConfigApplication {
  const factory VeilidConfigApplication({
    required VeilidConfigHTTPS https,
    required VeilidConfigHTTP http,
  }) = _VeilidConfigApplication;

  factory VeilidConfigApplication.fromJson(Map<String, dynamic> json) =>
      _$VeilidConfigApplicationFromJson(json);
}

////////////
@freezed
class VeilidConfigUDP with _$VeilidConfigUDP {
  const factory VeilidConfigUDP(
      {required bool enabled,
      required int socketPoolSize,
      required String listenAddress,
      String? publicAddress}) = _VeilidConfigUDP;

  factory VeilidConfigUDP.fromJson(Map<String, dynamic> json) =>
      _$VeilidConfigUDPFromJson(json);
}

////////////
@freezed
class VeilidConfigTCP with _$VeilidConfigTCP {
  const factory VeilidConfigTCP(
      {required bool connect,
      required bool listen,
      required int maxConnections,
      required String listenAddress,
      String? publicAddress}) = _VeilidConfigTCP;

  factory VeilidConfigTCP.fromJson(Map<String, dynamic> json) =>
      _$VeilidConfigTCPFromJson(json);
}

////////////
@freezed
class VeilidConfigWS with _$VeilidConfigWS {
  const factory VeilidConfigWS(
      {required bool connect,
      required bool listen,
      required int maxConnections,
      required String listenAddress,
      required String path,
      String? url}) = _VeilidConfigWS;

  factory VeilidConfigWS.fromJson(Map<String, dynamic> json) =>
      _$VeilidConfigWSFromJson(json);
}

////////////
@freezed
class VeilidConfigWSS with _$VeilidConfigWSS {
  const factory VeilidConfigWSS(
      {required bool connect,
      required bool listen,
      required int maxConnections,
      required String listenAddress,
      required String path,
      String? url}) = _VeilidConfigWSS;

  factory VeilidConfigWSS.fromJson(Map<String, dynamic> json) =>
      _$VeilidConfigWSSFromJson(json);
}

////////////

@freezed
class VeilidConfigProtocol with _$VeilidConfigProtocol {
  const factory VeilidConfigProtocol({
    required VeilidConfigUDP udp,
    required VeilidConfigTCP tcp,
    required VeilidConfigWS ws,
    required VeilidConfigWSS wss,
  }) = _VeilidConfigProtocol;

  factory VeilidConfigProtocol.fromJson(Map<String, dynamic> json) =>
      _$VeilidConfigProtocolFromJson(json);
}

////////////

@freezed
class VeilidConfigTLS with _$VeilidConfigTLS {
  const factory VeilidConfigTLS({
    required String certificatePath,
    required String privateKeyPath,
    required int connectionInitialTimeoutMs,
  }) = _VeilidConfigTLS;

  factory VeilidConfigTLS.fromJson(Map<String, dynamic> json) =>
      _$VeilidConfigTLSFromJson(json);
}

////////////
@freezed
class VeilidConfigDHT with _$VeilidConfigDHT {
  const factory VeilidConfigDHT(
      {required int resolveNodeTimeoutMs,
      required int resolveNodeCount,
      required int resolveNodeFanout,
      required int maxFindNodeCount,
      required int getValueTimeoutMs,
      required int getValueCount,
      required int getValueFanout,
      required int setValueTimeoutMs,
      required int setValueCount,
      required int setValueFanout,
      required int minPeerCount,
      required int minPeerRefreshTimeMs,
      required int validateDialInfoReceiptTimeMs,
      required int localSubkeyCacheSize,
      required int localMaxSubkeyCacheMemoryMb,
      required int remoteSubkeyCacheSize,
      required int remoteMaxRecords,
      required int remoteMaxSubkeyCacheMemoryMb,
      required int remoteMaxStorageSpaceMb}) = _VeilidConfigDHT;

  factory VeilidConfigDHT.fromJson(Map<String, dynamic> json) =>
      _$VeilidConfigDHTFromJson(json);
}

////////////

@freezed
class VeilidConfigRPC with _$VeilidConfigRPC {
  const factory VeilidConfigRPC(
      {required int concurrency,
      required int queueSize,
      int? maxTimestampBehindMs,
      int? maxTimestampAheadMs,
      required int timeoutMs,
      required int maxRouteHopCount,
      required int defaultRouteHopCount}) = _VeilidConfigRPC;

  factory VeilidConfigRPC.fromJson(Map<String, dynamic> json) =>
      _$VeilidConfigRPCFromJson(json);
}

////////////

@freezed
class VeilidConfigRoutingTable with _$VeilidConfigRoutingTable {
  const factory VeilidConfigRoutingTable({
    required List<TypedKey> nodeId,
    required List<TypedSecret> nodeIdSecret,
    required List<String> bootstrap,
    required int limitOverAttached,
    required int limitFullyAttached,
    required int limitAttachedStrong,
    required int limitAttachedGood,
    required int limitAttachedWeak,
  }) = _VeilidConfigRoutingTable;

  factory VeilidConfigRoutingTable.fromJson(Map<String, dynamic> json) =>
      _$VeilidConfigRoutingTableFromJson(json);
}

////////////

@freezed
class VeilidConfigNetwork with _$VeilidConfigNetwork {
  const factory VeilidConfigNetwork({
    required int connectionInitialTimeoutMs,
    required int connectionInactivityTimeoutMs,
    required int maxConnectionsPerIp4,
    required int maxConnectionsPerIp6Prefix,
    required int maxConnectionsPerIp6PrefixSize,
    required int maxConnectionFrequencyPerMin,
    required int clientWhitelistTimeoutMs,
    required int reverseConnectionReceiptTimeMs,
    required int holePunchReceiptTimeMs,
    String? networkKeyPassword,
    required VeilidConfigRoutingTable routingTable,
    required VeilidConfigRPC rpc,
    required VeilidConfigDHT dht,
    required bool upnp,
    required bool detectAddressChanges,
    required int restrictedNatRetries,
    required VeilidConfigTLS tls,
    required VeilidConfigApplication application,
    required VeilidConfigProtocol protocol,
  }) = _VeilidConfigNetwork;

  factory VeilidConfigNetwork.fromJson(Map<String, dynamic> json) =>
      _$VeilidConfigNetworkFromJson(json);
}

////////////

@freezed
class VeilidConfigTableStore with _$VeilidConfigTableStore {
  const factory VeilidConfigTableStore({
    required String directory,
    required bool delete,
  }) = _VeilidConfigTableStore;

  factory VeilidConfigTableStore.fromJson(Map<String, dynamic> json) =>
      _$VeilidConfigTableStoreFromJson(json);
}

////////////

@freezed
class VeilidConfigBlockStore with _$VeilidConfigBlockStore {
  const factory VeilidConfigBlockStore({
    required String directory,
    required bool delete,
  }) = _VeilidConfigBlockStore;

  factory VeilidConfigBlockStore.fromJson(Map<String, dynamic> json) =>
      _$VeilidConfigBlockStoreFromJson(json);
}

////////////

@freezed
class VeilidConfigProtectedStore with _$VeilidConfigProtectedStore {
  const factory VeilidConfigProtectedStore(
      {required bool allowInsecureFallback,
      required bool alwaysUseInsecureStorage,
      required String directory,
      required bool delete,
      required String deviceEncryptionKeyPassword,
      String? newDeviceEncryptionKeyPassword}) = _VeilidConfigProtectedStore;

  factory VeilidConfigProtectedStore.fromJson(Map<String, dynamic> json) =>
      _$VeilidConfigProtectedStoreFromJson(json);
}

////////////

@freezed
class VeilidConfigCapabilities with _$VeilidConfigCapabilities {
  const factory VeilidConfigCapabilities({
    required List<String> disable,
  }) = _VeilidConfigCapabilities;

  factory VeilidConfigCapabilities.fromJson(Map<String, dynamic> json) =>
      _$VeilidConfigCapabilitiesFromJson(json);
}

////////////

@freezed
class VeilidConfig with _$VeilidConfig {
  const factory VeilidConfig({
    required String programName,
    required String namespace,
    required VeilidConfigCapabilities capabilities,
    required VeilidConfigProtectedStore protectedStore,
    required VeilidConfigTableStore tableStore,
    required VeilidConfigBlockStore blockStore,
    required VeilidConfigNetwork network,
  }) = _VeilidConfig;

  factory VeilidConfig.fromJson(Map<String, dynamic> json) =>
      _$VeilidConfigFromJson(json);
}
