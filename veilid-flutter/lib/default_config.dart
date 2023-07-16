import 'package:flutter/foundation.dart' show kIsWeb;
import 'package:path_provider/path_provider.dart';
import 'package:path/path.dart' as p;
import 'package:system_info2/system_info2.dart' as sysinfo;
import 'veilid.dart';

const int megaByte = 1024 * 1024;

int getLocalSubkeyCacheSize() {
  if (kIsWeb) {
    return 128;
  }
  return 1024;
}

int getLocalMaxSubkeyCacheMemoryMb() {
  if (kIsWeb) {
    return 256;
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

int getRemoteMaxSubkeyCacheMemoryMb() {
  if (kIsWeb) {
    return 256;
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
  return VeilidConfig(
    programName: programName,
    namespace: "",
    capabilities: const VeilidConfigCapabilities(disable: []),
    protectedStore: const VeilidConfigProtectedStore(
      allowInsecureFallback: false,
      alwaysUseInsecureStorage: false,
      directory: "",
      delete: false,
      deviceEncryptionKeyPassword: "",
      newDeviceEncryptionKeyPassword: null,
    ),
    tableStore: VeilidConfigTableStore(
      directory: kIsWeb
          ? ""
          : p.join((await getApplicationSupportDirectory()).absolute.path,
              "table_store"),
      delete: false,
    ),
    blockStore: VeilidConfigBlockStore(
      directory: kIsWeb
          ? ""
          : p.join((await getApplicationSupportDirectory()).absolute.path,
              "block_store"),
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
      routingTable: const VeilidConfigRoutingTable(
        nodeId: [],
        nodeIdSecret: [],
        bootstrap: kIsWeb
            ? ["ws://bootstrap.veilid.net:5150/ws"]
            : ["bootstrap.veilid.net"],
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
          resolveNodeTimeoutMs: 10000,
          resolveNodeCount: 20,
          resolveNodeFanout: 3,
          maxFindNodeCount: 20,
          getValueTimeoutMs: 10000,
          getValueCount: 20,
          getValueFanout: 3,
          setValueTimeoutMs: 10000,
          setValueCount: 20,
          setValueFanout: 5,
          minPeerCount: 20,
          minPeerRefreshTimeMs: 2000,
          validateDialInfoReceiptTimeMs: 2000,
          localSubkeyCacheSize: getLocalSubkeyCacheSize(),
          localMaxSubkeyCacheMemoryMb: getLocalMaxSubkeyCacheMemoryMb(),
          remoteSubkeyCacheSize: getRemoteSubkeyCacheSize(),
          remoteMaxRecords: getRemoteMaxRecords(),
          remoteMaxSubkeyCacheMemoryMb: getRemoteMaxSubkeyCacheMemoryMb(),
          remoteMaxStorageSpaceMb: getRemoteMaxStorageSpaceMb()),
      upnp: true,
      detectAddressChanges: true,
      restrictedNatRetries: 0,
      tls: const VeilidConfigTLS(
        certificatePath: "",
        privateKeyPath: "",
        connectionInitialTimeoutMs: 2000,
      ),
      application: const VeilidConfigApplication(
          https: VeilidConfigHTTPS(
            enabled: false,
            listenAddress: "",
            path: "",
            url: null,
          ),
          http: VeilidConfigHTTP(
            enabled: false,
            listenAddress: "",
            path: "",
            url: null,
          )),
      protocol: const VeilidConfigProtocol(
        udp: VeilidConfigUDP(
          enabled: !kIsWeb,
          socketPoolSize: 0,
          listenAddress: "",
          publicAddress: null,
        ),
        tcp: VeilidConfigTCP(
          connect: !kIsWeb,
          listen: !kIsWeb,
          maxConnections: 32,
          listenAddress: "",
          publicAddress: null,
        ),
        ws: VeilidConfigWS(
          connect: true,
          listen: !kIsWeb,
          maxConnections: 16,
          listenAddress: "",
          path: "ws",
          url: null,
        ),
        wss: VeilidConfigWSS(
          connect: true,
          listen: false,
          maxConnections: 16,
          listenAddress: "",
          path: "ws",
          url: null,
        ),
      ),
    ),
  );
}
