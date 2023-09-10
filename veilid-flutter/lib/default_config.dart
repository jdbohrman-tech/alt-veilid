import 'dart:io';

import 'package:flutter/foundation.dart' show kIsWeb;
import 'package:path/path.dart' as p;
import 'package:path_provider/path_provider.dart';
import 'package:system_info2/system_info2.dart' as sysinfo;
import 'package:system_info_plus/system_info_plus.dart';

import 'veilid.dart';

const int megaByte = 1024 * 1024;

int getLocalSubkeyCacheSize() {
  if (kIsWeb) {
    return 128;
  }
  return 1024;
}

Future<int> getLocalMaxSubkeyCacheMemoryMb() async {
  if (kIsWeb) {
    return 256;
  }
  if (Platform.isIOS || Platform.isAndroid) {
    return (await SystemInfoPlus.physicalMemory ?? 2048) ~/ 32;
  }
  return sysinfo.SysInfo.getTotalPhysicalMemory() ~/ 32 ~/ megaByte;
}

int getRemoteSubkeyCacheSize() {
  if (kIsWeb) {
    return 64;
  }
  return 128;
}

int getRemoteMaxRecords() {
  if (kIsWeb) {
    return 64;
  }
  return 128;
}

Future<int> getRemoteMaxSubkeyCacheMemoryMb() async {
  if (kIsWeb) {
    return 256;
  }
  if (Platform.isIOS || Platform.isAndroid) {
    return (await SystemInfoPlus.physicalMemory ?? 2048) ~/ 32;
  }
  return sysinfo.SysInfo.getTotalPhysicalMemory() ~/ 32 ~/ megaByte;
}

int getRemoteMaxStorageSpaceMb() {
  if (kIsWeb) {
    return 128;
  }
  return 256;
}

Future<VeilidConfig> getDefaultVeilidConfig(String programName) async {
  // ignore: do_not_use_environment
  const bootstrap = String.fromEnvironment('BOOTSTRAP');
  return VeilidConfig(
    programName: programName,
    namespace: '',
    capabilities: const VeilidConfigCapabilities(disable: []),
    protectedStore: const VeilidConfigProtectedStore(
      allowInsecureFallback: false,
      alwaysUseInsecureStorage: false,
      directory: '',
      delete: false,
      deviceEncryptionKeyPassword: '',
    ),
    tableStore: VeilidConfigTableStore(
      directory: kIsWeb
          ? ''
          : p.join((await getApplicationSupportDirectory()).absolute.path,
              'table_store'),
      delete: false,
    ),
    blockStore: VeilidConfigBlockStore(
      directory: kIsWeb
          ? ''
          : p.join((await getApplicationSupportDirectory()).absolute.path,
              'block_store'),
      delete: false,
    ),
    network: VeilidConfigNetwork(
      connectionInitialTimeoutMs: 2000,
      connectionInactivityTimeoutMs: 60000,
      maxConnectionsPerIp4: 32,
      maxConnectionsPerIp6Prefix: 32,
      maxConnectionsPerIp6PrefixSize: 56,
      maxConnectionFrequencyPerMin: 128,
      clientWhitelistTimeoutMs: 300000,
      reverseConnectionReceiptTimeMs: 5000,
      holePunchReceiptTimeMs: 5000,
      routingTable: VeilidConfigRoutingTable(
        nodeId: [],
        nodeIdSecret: [],
        bootstrap: bootstrap.isNotEmpty
            ? bootstrap.split(',')
            : (kIsWeb
                ? ['ws://bootstrap.veilid.net:5150/ws']
                : ['bootstrap.veilid.net']),
        limitOverAttached: 64,
        limitFullyAttached: 32,
        limitAttachedStrong: 16,
        limitAttachedGood: 8,
        limitAttachedWeak: 4,
      ),
      rpc: const VeilidConfigRPC(
        concurrency: 0,
        queueSize: 1024,
        maxTimestampBehindMs: 10000,
        maxTimestampAheadMs: 10000,
        timeoutMs: 5000,
        maxRouteHopCount: 4,
        defaultRouteHopCount: 1,
      ),
      dht: VeilidConfigDHT(
          maxFindNodeCount: 20,
          resolveNodeTimeoutMs: 10000,
          resolveNodeCount: 1,
          resolveNodeFanout: 4,
          getValueTimeoutMs: 10000,
          getValueCount: 3,
          getValueFanout: 4,
          setValueTimeoutMs: 10000,
          setValueCount: 5,
          setValueFanout: 4,
          minPeerCount: 20,
          minPeerRefreshTimeMs: 60000,
          validateDialInfoReceiptTimeMs: 2000,
          localSubkeyCacheSize: getLocalSubkeyCacheSize(),
          localMaxSubkeyCacheMemoryMb: await getLocalMaxSubkeyCacheMemoryMb(),
          remoteSubkeyCacheSize: getRemoteSubkeyCacheSize(),
          remoteMaxRecords: getRemoteMaxRecords(),
          remoteMaxSubkeyCacheMemoryMb: await getRemoteMaxSubkeyCacheMemoryMb(),
          remoteMaxStorageSpaceMb: getRemoteMaxStorageSpaceMb()),
      upnp: true,
      detectAddressChanges: true,
      restrictedNatRetries: 0,
      tls: const VeilidConfigTLS(
        certificatePath: '',
        privateKeyPath: '',
        connectionInitialTimeoutMs: 2000,
      ),
      application: const VeilidConfigApplication(
          https: VeilidConfigHTTPS(
            enabled: false,
            listenAddress: '',
            path: '',
          ),
          http: VeilidConfigHTTP(
            enabled: false,
            listenAddress: '',
            path: '',
          )),
      protocol: const VeilidConfigProtocol(
        udp: VeilidConfigUDP(
          enabled: !kIsWeb,
          socketPoolSize: 0,
          listenAddress: '',
        ),
        tcp: VeilidConfigTCP(
          connect: !kIsWeb,
          listen: !kIsWeb,
          maxConnections: 32,
          listenAddress: '',
        ),
        ws: VeilidConfigWS(
          connect: true,
          listen: !kIsWeb,
          maxConnections: 32,
          listenAddress: '',
          path: 'ws',
        ),
        wss: VeilidConfigWSS(
          connect: true,
          listen: false,
          maxConnections: 32,
          listenAddress: '',
          path: 'ws',
        ),
      ),
    ),
  );
}
