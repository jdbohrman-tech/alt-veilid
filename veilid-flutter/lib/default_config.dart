import 'package:flutter/foundation.dart' show kIsWeb;
import 'package:path_provider/path_provider.dart';
import 'package:path/path.dart' as p;
import 'veilid.dart';

Future<VeilidConfig> getDefaultVeilidConfig(String programName) async {
  return VeilidConfig(
    programName: programName,
    namespace: "",
    capabilities: VeilidConfigCapabilities(
      protocolUDP: !kIsWeb,
      protocolConnectTCP: !kIsWeb,
      protocolAcceptTCP: !kIsWeb,
      protocolConnectWS: true,
      protocolAcceptWS: !kIsWeb,
      protocolConnectWSS: true,
      protocolAcceptWSS: false,
    ),
    protectedStore: VeilidConfigProtectedStore(
      allowInsecureFallback: false,
      alwaysUseInsecureStorage: false,
      insecureFallbackDirectory: "",
      delete: false,
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
      nodeId: null,
      nodeIdSecret: null,
      bootstrap: kIsWeb
          ? ["ws://bootstrap.dev.veilid.net:5150/ws"]
          : ["bootstrap.dev.veilid.net"],
      bootstrapNodes: [],
      routingTable: VeilidConfigRoutingTable(
        limitOverAttached: 64,
        limitFullyAttached: 32,
        limitAttachedStrong: 16,
        limitAttachedGood: 8,
        limitAttachedWeak: 4,
      ),
      rpc: VeilidConfigRPC(
        concurrency: 0,
        queueSize: 1024,
        maxTimestampBehindMs: 10000,
        maxTimestampAheadMs: 10000,
        timeoutMs: 10000,
        maxRouteHopCount: 4,
        defaultRouteHopCount: 1,
      ),
      dht: VeilidConfigDHT(
        resolveNodeTimeoutMs: null,
        resolveNodeCount: 20,
        resolveNodeFanout: 3,
        maxFindNodeCount: 20,
        getValueTimeoutMs: null,
        getValueCount: 20,
        getValueFanout: 3,
        setValueTimeoutMs: null,
        setValueCount: 20,
        setValueFanout: 5,
        minPeerCount: 20,
        minPeerRefreshTimeMs: 2000,
        validateDialInfoReceiptTimeMs: 2000,
      ),
      upnp: true,
      detectAddressChanges: true,
      restrictedNatRetries: 0,
      tls: VeilidConfigTLS(
        certificatePath: "",
        privateKeyPath: "",
        connectionInitialTimeoutMs: 2000,
      ),
      application: VeilidConfigApplication(
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
      protocol: VeilidConfigProtocol(
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
