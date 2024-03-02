import 'dart:async';

import 'package:flutter/foundation.dart';
import 'package:mutex/mutex.dart';
import 'package:veilid/veilid.dart';

class DefaultFixture {
  DefaultFixture();

  StreamSubscription<VeilidUpdate>? _updateSubscription;
  Stream<VeilidUpdate>? _updateStream;

  static final _fixtureMutex = Mutex();

  Future<void> setUp() async {
    await _fixtureMutex.acquire();

    assert(_updateStream == null, 'should not set up fixture twice');

    final Map<String, dynamic> platformConfigJson;
    if (kIsWeb) {
      const platformConfig = VeilidWASMConfig(
          logging: VeilidWASMConfigLogging(
              performance: VeilidWASMConfigLoggingPerformance(
                enabled: true,
                level: VeilidConfigLogLevel.debug,
                logsInTimings: true,
                logsInConsole: false,
              ),
              api: VeilidWASMConfigLoggingApi(
                enabled: true,
                level: VeilidConfigLogLevel.info,
              )));
      platformConfigJson = platformConfig.toJson();
    } else {
      const platformConfig = VeilidFFIConfig(
          logging: VeilidFFIConfigLogging(
              terminal: VeilidFFIConfigLoggingTerminal(
                enabled: false,
                level: VeilidConfigLogLevel.debug,
              ),
              otlp: VeilidFFIConfigLoggingOtlp(
                enabled: false,
                level: VeilidConfigLogLevel.trace,
                grpcEndpoint: 'localhost:4317',
                serviceName: 'Veilid Tests',
              ),
              api: VeilidFFIConfigLoggingApi(
                enabled: true,
                level: VeilidConfigLogLevel.info,
              )));
      platformConfigJson = platformConfig.toJson();
    }
    Veilid.instance.initializeVeilidCore(platformConfigJson);

    final defaultConfig = await getDefaultVeilidConfig(
        isWeb: kIsWeb, programName: 'Veilid Tests');

    final updateStream =
        _updateStream = await Veilid.instance.startupVeilidCore(defaultConfig);
    if (_updateStream == null) {
      throw Exception('failed to start up veilid core');
    }

    _updateSubscription = updateStream.listen((update) {
      if (update is VeilidLog) {
      } else if (update is VeilidUpdateAttachment) {
      } else if (update is VeilidUpdateConfig) {
      } else if (update is VeilidUpdateNetwork) {
      } else if (update is VeilidAppMessage) {
      } else if (update is VeilidAppCall) {
      } else if (update is VeilidUpdateValueChange) {
      } else {
        throw Exception('unexpected update: $update');
      }
    });
  }

  Future<void> tearDown() async {
    assert(_updateStream != null, 'should not tearDown without setUp');

    final cancelFut = _updateSubscription?.cancel();
    await Veilid.instance.shutdownVeilidCore();
    await cancelFut;

    _updateSubscription = null;
    _updateStream = null;

    _fixtureMutex.release();
  }
}
