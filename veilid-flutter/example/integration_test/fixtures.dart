import 'dart:async';

import 'package:flutter/foundation.dart';
import 'package:mutex/mutex.dart';
import 'package:veilid/veilid.dart';

class DefaultFixture {
  DefaultFixture();

  StreamSubscription<VeilidUpdate>? _veilidUpdateSubscription;
  Stream<VeilidUpdate>? _veilidUpdateStream;
  final StreamController<VeilidUpdate> _updateStreamController =
      StreamController.broadcast();

  static final _fixtureMutex = Mutex();

  Future<void> setUp() async {
    await _fixtureMutex.acquire();

    assert(_veilidUpdateStream == null, 'should not set up fixture twice');

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

    var config = await getDefaultVeilidConfig(
      isWeb: kIsWeb,
      programName: 'Veilid Tests',
      // ignore: avoid_redundant_argument_values, do_not_use_environment
      bootstrap: const String.fromEnvironment('BOOTSTRAP'),
      // ignore: avoid_redundant_argument_values, do_not_use_environment
      networkKeyPassword: const String.fromEnvironment('NETWORK_KEY'),
    );

    config =
        config.copyWith(tableStore: config.tableStore.copyWith(delete: true));
    config = config.copyWith(
        protectedStore: config.protectedStore.copyWith(delete: true));
    config =
        config.copyWith(blockStore: config.blockStore.copyWith(delete: true));

    final us =
        _veilidUpdateStream = await Veilid.instance.startupVeilidCore(config);

    _veilidUpdateSubscription = us.listen((update) {
      if (update is VeilidLog) {
      } else if (update is VeilidUpdateAttachment) {
      } else if (update is VeilidUpdateConfig) {
      } else if (update is VeilidUpdateNetwork) {
      } else if (update is VeilidAppMessage) {
      } else if (update is VeilidAppCall) {
      } else if (update is VeilidUpdateValueChange) {
      } else if (update is VeilidUpdateRouteChange) {
      } else {
        throw Exception('unexpected update: $update');
      }
      _updateStreamController.sink.add(update);
    });
  }

  Stream<VeilidUpdate> get updateStream => _updateStreamController.stream;

  Future<void> attach() async {
    await Veilid.instance.attach();

    // Wait for attached state
    while (true) {
      final state = await Veilid.instance.getVeilidState();
      var done = false;
      if (state.attachment.publicInternetReady) {
        switch (state.attachment.state) {
          case AttachmentState.detached:
            break;
          case AttachmentState.attaching:
            break;
          case AttachmentState.detaching:
            break;
          default:
            done = true;
            break;
        }
      }
      if (done) {
        break;
      }
      await Future.delayed(const Duration(seconds: 1));
    }
  }

  Future<void> detach() async {
    await Veilid.instance.detach();
  }

  Future<void> tearDown() async {
    assert(_veilidUpdateStream != null, 'should not tearDown without setUp');

    final cancelFut = _veilidUpdateSubscription?.cancel();
    await Veilid.instance.shutdownVeilidCore();
    await cancelFut;

    _veilidUpdateSubscription = null;
    _veilidUpdateStream = null;

    _fixtureMutex.release();
  }
}
