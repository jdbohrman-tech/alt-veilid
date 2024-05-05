import 'dart:async';
import 'dart:convert';

import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:loggy/loggy.dart';
import 'package:veilid/veilid.dart';

import 'history_wrapper.dart';
import 'log.dart';
import 'log_terminal.dart';
import 'veilid_theme.dart';

// Main App
class MyApp extends StatefulWidget {
  const MyApp({super.key});

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> with UiLoggy {
  String _veilidVersion = 'Unknown';
  bool _startedUp = false;
  Stream<VeilidUpdate>? _updateStream;
  Future<void>? _updateProcessor;
  final _debugHistoryWrapper = HistoryWrapper();
  String? _errorText;

  @override
  void initState() {
    super.initState();

    unawaited(initPlatformState());
  }

  // Platform messages are asynchronous, so we initialize in an async method.
  Future<void> initPlatformState() async {
    String veilidVersion;
    // Platform messages may fail, so we use a try/catch PlatformException.
    // We also handle the message potentially returning null.
    try {
      veilidVersion = Veilid.instance.veilidVersionString();
    } on Exception {
      veilidVersion = 'Failed to get veilid version.';
    }

    // In case of hot restart shut down first
    try {
      await Veilid.instance.shutdownVeilidCore();
    } on Exception {
      //
    }

    // If the widget was removed from the tree while the asynchronous platform
    // message was in flight, we want to discard the reply rather than calling
    // setState to update our non-existent appearance.
    if (!mounted) {
      return;
    }

    setState(() {
      _veilidVersion = veilidVersion;
    });
  }

  Future<void> processLog(VeilidLog log) async {
    StackTrace? stackTrace;
    Object? error;
    final backtrace = log.backtrace;
    if (backtrace != null) {
      stackTrace = StackTrace.fromString('$backtrace\n${StackTrace.current}');
      error = 'embedded stack trace for ${log.logLevel} ${log.message}';
    }

    switch (log.logLevel) {
      case VeilidLogLevel.error:
        loggy.error(log.message, error, stackTrace);
        break;
      case VeilidLogLevel.warn:
        loggy.warning(log.message, error, stackTrace);
        break;
      case VeilidLogLevel.info:
        loggy.info(log.message, error, stackTrace);
        break;
      case VeilidLogLevel.debug:
        loggy.debug(log.message, error, stackTrace);
        break;
      case VeilidLogLevel.trace:
        loggy.trace(log.message, error, stackTrace);
        break;
    }
  }

  Future<void> processUpdates() async {
    final stream = _updateStream;
    if (stream != null) {
      await for (final update in stream) {
        if (update is VeilidLog) {
          await processLog(update);
        } else if (update is VeilidAppMessage) {
          loggy.info('AppMessage: ${jsonEncode(update)}');
        } else if (update is VeilidAppCall) {
          loggy.info('AppCall: ${jsonEncode(update)}');
        } else {
          loggy.trace('Update: ${jsonEncode(update)}');
        }
      }
    }
  }

  Future<void> toggleStartup(bool startup) async {
    if (startup && !_startedUp) {
      var config = await getDefaultVeilidConfig(
          isWeb: kIsWeb,
          programName: 'Veilid Plugin Example',
          // ignore: avoid_redundant_argument_values, do_not_use_environment
          bootstrap: const String.fromEnvironment('BOOTSTRAP'),
          // ignore: avoid_redundant_argument_values, do_not_use_environment
          networkKeyPassword: const String.fromEnvironment('NETWORK_KEY'));
      // ignore: do_not_use_environment
      if (const String.fromEnvironment('DELETE_TABLE_STORE') == '1') {
        config = config.copyWith(
            tableStore: config.tableStore.copyWith(delete: true));
      }
      // ignore: do_not_use_environment
      if (const String.fromEnvironment('DELETE_PROTECTED_STORE') == '1') {
        config = config.copyWith(
            protectedStore: config.protectedStore.copyWith(delete: true));
      }
      // ignore: do_not_use_environment
      if (const String.fromEnvironment('DELETE_BLOCK_STORE') == '1') {
        config = config.copyWith(
            blockStore: config.blockStore.copyWith(delete: true));
      }

      final updateStream = await Veilid.instance.startupVeilidCore(config);
      setState(() {
        _updateStream = updateStream;
        _updateProcessor = processUpdates();
        _startedUp = true;
      });
      await Veilid.instance.attach();
    } else if (!startup && _startedUp) {
      try {
        await Veilid.instance.shutdownVeilidCore();
        if (_updateProcessor != null) {
          await _updateProcessor;
        }
      } finally {
        setState(() {
          _updateProcessor = null;
          _updateStream = null;
          _startedUp = false;
        });
      }
    }
  }

  @override
  Widget build(BuildContext context) => Scaffold(
      appBar: AppBar(
        title: Text('Veilid Plugin Version $_veilidVersion'),
      ),
      body: Column(children: [
        const Expanded(child: LogTerminal()),
        Container(
          decoration: BoxDecoration(
              color: materialBackgroundColor.shade100,
              boxShadow: [
                BoxShadow(
                  color: Colors.black.withOpacity(0.15),
                  spreadRadius: 4,
                  blurRadius: 4,
                )
              ]),
          padding: const EdgeInsets.all(5),
          child: Row(children: [
            Expanded(
                child: pad(_debugHistoryWrapper.wrap(
              setState,
              TextField(
                  controller: _debugHistoryWrapper.controller,
                  decoration: newInputDecoration(
                      'Debug Command', _errorText, _startedUp),
                  textInputAction: TextInputAction.unspecified,
                  enabled: _startedUp,
                  onChanged: (v) {
                    setState(() {
                      _errorText = null;
                    });
                  },
                  onSubmitted: (v) async {
                    try {
                      if (v.isEmpty) {
                        return;
                      }
                      final res = await Veilid.instance.debug(v);
                      loggy.info(res);
                      setState(() {
                        _debugHistoryWrapper.submit(v);
                      });
                    } on VeilidAPIException catch (e) {
                      setState(() {
                        _errorText = e.toDisplayError();
                      });
                    }
                  }),
            ))),
            pad(
              Column(children: [
                const Text('Startup'),
                Switch(
                    value: _startedUp,
                    onChanged: (value) async {
                      await toggleStartup(value);
                    }),
              ]),
            ),
            pad(Column(children: [
              const Text('Log Level'),
              DropdownButton<LogLevel>(
                  value: loggy.level.logLevel,
                  onChanged: (newLevel) {
                    setState(() {
                      setRootLogLevel(newLevel);
                    });
                  },
                  items: const [
                    DropdownMenuItem<LogLevel>(
                        value: LogLevel.error, child: Text('Error')),
                    DropdownMenuItem<LogLevel>(
                        value: LogLevel.warning, child: Text('Warning')),
                    DropdownMenuItem<LogLevel>(
                        value: LogLevel.info, child: Text('Info')),
                    DropdownMenuItem<LogLevel>(
                        value: LogLevel.debug, child: Text('Debug')),
                    DropdownMenuItem<LogLevel>(
                        value: traceLevel, child: Text('Trace')),
                    DropdownMenuItem<LogLevel>(
                        value: LogLevel.all, child: Text('All')),
                  ]),
            ])),
          ]),
        ),
      ]));
}
