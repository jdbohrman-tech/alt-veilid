import 'dart:convert';

import 'package:path/path.dart' as p;
import 'package:path_provider/path_provider.dart';

import 'veilid.dart';

Future<VeilidConfig> getDefaultVeilidConfig({
  required bool isWeb,
  required String programName,
  String bootstrap = '',
  String namespace = '',
  String deviceEncryptionKeyPassword = '',
  String? newDeviceEncryptionKeyPassword,
  String networkKeyPassword = '',
}) async {
  final defaultConfigStr = Veilid.instance.defaultVeilidConfig();
  final defaultConfig = VeilidConfig.fromJson(jsonDecode(defaultConfigStr));
  return defaultConfig.copyWith(
      programName: programName,
      namespace: namespace,
      tableStore: defaultConfig.tableStore.copyWith(
        directory: isWeb
            ? ''
            : p.join((await getApplicationSupportDirectory()).absolute.path,
                'table_store'),
      ),
      blockStore: defaultConfig.blockStore.copyWith(
        directory: isWeb
            ? ''
            : p.join((await getApplicationSupportDirectory()).absolute.path,
                'block_store'),
      ),
      protectedStore: defaultConfig.protectedStore.copyWith(
        directory: isWeb
            ? ''
            : p.join((await getApplicationSupportDirectory()).absolute.path,
                'protected_store'),
        deviceEncryptionKeyPassword: deviceEncryptionKeyPassword,
        newDeviceEncryptionKeyPassword: newDeviceEncryptionKeyPassword,
      ),
      network: defaultConfig.network.copyWith(
          networkKeyPassword: networkKeyPassword,
          routingTable: defaultConfig.network.routingTable.copyWith(
              bootstrap: bootstrap.isNotEmpty
                  ? bootstrap.split(',')
                  : defaultConfig.network.routingTable.bootstrap),
          dht: defaultConfig.network.dht.copyWith()));
}
