import 'dart:async';

import 'package:async_tools/async_tools.dart';
import 'package:veilid/veilid.dart';

// ignore: do_not_use_environment
bool kIsWeb = const bool.fromEnvironment('dart.library.js_util');

abstract class VeilidFixture {
  Future<void> setUp();
  Future<void> tearDown();
  Future<void> attach();
  Future<void> detach();
  Stream<VeilidUpdate> get updateStream;
}

class DefaultVeilidFixture implements VeilidFixture {
  DefaultVeilidFixture({required this.programName});

  StreamSubscription<VeilidUpdate>? _veilidUpdateSubscription;
  Stream<VeilidUpdate>? _veilidUpdateStream;
  late final StreamController<VeilidUpdate> _updateStreamController;

  static final _fixtureMutex = Mutex();
  final String programName;

  @override
  Future<void> setUp() async {
    await _fixtureMutex.acquire();
    assert(_veilidUpdateStream == null, 'should not set up fixture twice');

    _updateStreamController = StreamController.broadcast();

    final ignoreLogTargetsStr =
        // ignore: do_not_use_environment
        const String.fromEnvironment('IGNORE_LOG_TARGETS').trim();
    final ignoreLogTargets = ignoreLogTargetsStr.isEmpty
        ? <String>[]
        : ignoreLogTargetsStr.split(',').map((e) => e.trim()).toList();

    final logLevel = VeilidConfigLogLevel.fromJson(
        // ignore: do_not_use_environment
        const String.fromEnvironment('LOG_LEVEL', defaultValue: 'info'));

    final Map<String, dynamic> platformConfigJson;
    if (kIsWeb) {
      final platformConfig = VeilidWASMConfig(
          logging: VeilidWASMConfigLogging(
              performance: VeilidWASMConfigLoggingPerformance(
                enabled: true,
                level: logLevel,
                logsInTimings: true,
                logsInConsole: true,
                ignoreLogTargets: ignoreLogTargets,
              ),
              api: VeilidWASMConfigLoggingApi(
                enabled: true,
                level: logLevel,
                ignoreLogTargets: ignoreLogTargets,
              )));
      platformConfigJson = platformConfig.toJson();
    } else {
      final platformConfig = VeilidFFIConfig(
          logging: VeilidFFIConfigLogging(
              terminal: VeilidFFIConfigLoggingTerminal(
                enabled: false,
                level: VeilidConfigLogLevel.debug,
                ignoreLogTargets: ignoreLogTargets,
              ),
              otlp: VeilidFFIConfigLoggingOtlp(
                enabled: false,
                level: VeilidConfigLogLevel.trace,
                grpcEndpoint: 'localhost:4317',
                serviceName: 'Veilid Tests',
                ignoreLogTargets: ignoreLogTargets,
              ),
              api: VeilidFFIConfigLoggingApi(
                enabled: true,
                level: logLevel,
                ignoreLogTargets: ignoreLogTargets,
              )));
      platformConfigJson = platformConfig.toJson();
    }
    Veilid.instance.initializeVeilidCore(platformConfigJson);

    var config = await getDefaultVeilidConfig(
      isWeb: kIsWeb,
      programName: programName,
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
        // print(update.message);
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

  @override
  Stream<VeilidUpdate> get updateStream => _updateStreamController.stream;

  @override
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
          case AttachmentState.attachedGood:
          case AttachmentState.attachedStrong:
          case AttachmentState.attachedWeak:
          case AttachmentState.overAttached:
          case AttachmentState.fullyAttached:
            done = true;
            break;
        }
      }
      if (done) {
        break;
      }
      await Future<void>.delayed(const Duration(seconds: 1));
    }
  }

  @override
  Future<void> detach() async {
    await Veilid.instance.detach();
  }

  @override
  Future<void> tearDown() async {
    assert(_fixtureMutex.isLocked, 'should not tearDown without setUp');

    await Veilid.instance.shutdownVeilidCore();
    await _veilidUpdateSubscription?.cancel();
    await _updateStreamController.close();

    _veilidUpdateSubscription = null;
    _veilidUpdateStream = null;

    _fixtureMutex.release();
  }
}
