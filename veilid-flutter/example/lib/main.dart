import 'dart:async';
import 'dart:typed_data';
import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter/foundation.dart' show kIsWeb;
import 'package:veilid/veilid.dart';
import 'package:flutter_loggy/flutter_loggy.dart';
import 'package:loggy/loggy.dart';

import 'config.dart';

// Loggy tools
const LogLevel traceLevel = LogLevel('Trace', 1);

class ConsolePrinter extends LoggyPrinter {
  ConsolePrinter(this.childPrinter) : super();

  final LoggyPrinter childPrinter;

  @override
  void onLog(LogRecord record) {
    debugPrint(record.toString());
    childPrinter.onLog(record);
  }
}

extension TraceLoggy on Loggy {
  void trace(dynamic message, [Object? error, StackTrace? stackTrace]) =>
      log(traceLevel, message, error, stackTrace);
}

LogOptions getLogOptions(LogLevel? level) {
  return LogOptions(
    level ?? LogLevel.all,
    stackTraceLevel: LogLevel.error,
  );
}

VeilidConfigLogLevel convertToVeilidConfigLogLevel(LogLevel? level) {
  if (level == null) {
    return VeilidConfigLogLevel.off;
  }
  switch (level) {
    case LogLevel.error:
      return VeilidConfigLogLevel.error;
    case LogLevel.warning:
      return VeilidConfigLogLevel.warn;
    case LogLevel.info:
      return VeilidConfigLogLevel.info;
    case LogLevel.debug:
      return VeilidConfigLogLevel.debug;
    case traceLevel:
      return VeilidConfigLogLevel.trace;
  }
  return VeilidConfigLogLevel.off;
}

void setRootLogLevel(LogLevel? level) {
  Loggy('').level = getLogOptions(level);
  Veilid.instance.changeLogLevel("all", convertToVeilidConfigLogLevel(level));
}

void initLoggy() {
  Loggy.initLoggy(
    logPrinter: StreamPrinter(ConsolePrinter(
      const PrettyDeveloperPrinter(),
    )),
    logOptions: getLogOptions(null),
  );
}

// Entrypoint
void main() {
  WidgetsFlutterBinding.ensureInitialized();

  initLoggy();
  if (kIsWeb) {
    var platformConfig = VeilidWASMConfig(
        logging: VeilidWASMConfigLogging(
            performance: VeilidWASMConfigLoggingPerformance(
                enabled: true,
                level: VeilidConfigLogLevel.debug,
                logsInTimings: true,
                logsInConsole: true),
            api: VeilidWASMConfigLoggingApi(
                enabled: true, level: VeilidConfigLogLevel.info)));
    Veilid.instance.initializeVeilidCore(platformConfig.json);
  } else {
    var platformConfig = VeilidFFIConfig(
        logging: VeilidFFIConfigLogging(
            terminal: VeilidFFIConfigLoggingTerminal(
              enabled: false,
              level: VeilidConfigLogLevel.debug,
            ),
            otlp: VeilidFFIConfigLoggingOtlp(
                enabled: false,
                level: VeilidConfigLogLevel.trace,
                grpcEndpoint: "localhost:4317",
                serviceName: "VeilidExample"),
            api: VeilidFFIConfigLoggingApi(
                enabled: true, level: VeilidConfigLogLevel.info)));
    Veilid.instance.initializeVeilidCore(platformConfig.json);
  }

  runApp(MaterialApp(
      title: 'Veilid Plugin Demo',
      theme: ThemeData(
        primarySwatch: Colors.blue,
        visualDensity: VisualDensity.adaptivePlatformDensity,
      ),
      home: const MyApp()));
}

// Main App
class MyApp extends StatefulWidget {
  const MyApp({Key? key}) : super(key: key);

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> with UiLoggy {
  String _veilidVersion = 'Unknown';
  Stream<VeilidUpdate>? _updateStream;
  Future<void>? _updateProcessor;

  @override
  void initState() {
    super.initState();
    setRootLogLevel(LogLevel.info);
    initPlatformState();
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
    if (!mounted) return;

    setState(() {
      _veilidVersion = veilidVersion;
    });
  }

  Future<void> processLog(VeilidLog log) async {
    StackTrace? stackTrace;
    Object? error;
    final backtrace = log.backtrace;
    if (backtrace != null) {
      stackTrace =
          StackTrace.fromString("$backtrace\n${StackTrace.current.toString()}");
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
    var stream = _updateStream;
    if (stream != null) {
      await for (final update in stream) {
        if (update is VeilidLog) {
          await processLog(update);
        } else if (update is VeilidAppMessage) {
          loggy.info("AppMessage: ${update.json}");
        } else if (update is VeilidAppCall) {
          loggy.info("AppCall: ${update.json}");
        } else {
          loggy.trace("Update: ${update.json}");
        }
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    final ButtonStyle buttonStyle =
        ElevatedButton.styleFrom(textStyle: const TextStyle(fontSize: 20));

    return Scaffold(
        appBar: AppBar(
          title: Text('Veilid Plugin Version $_veilidVersion'),
        ),
        body: Column(children: [
          Expanded(
              child: Container(
            color: ThemeData.dark().scaffoldBackgroundColor,
            height: MediaQuery.of(context).size.height * 0.4,
            child: LoggyStreamWidget(logLevel: loggy.level.logLevel),
          )),
          Container(
              padding: const EdgeInsets.fromLTRB(8, 8, 8, 12),
              child: Row(children: [
                ElevatedButton(
                  style: buttonStyle,
                  onPressed: _updateStream != null
                      ? null
                      : () async {
                          var updateStream = await Veilid.instance
                              .startupVeilidCore(
                                  await getDefaultVeilidConfig());
                          setState(() {
                            _updateStream = updateStream;
                            _updateProcessor = processUpdates();
                          });
                          await Veilid.instance.attach();
                        },
                  child: const Text('Startup'),
                ),
                ElevatedButton(
                  style: buttonStyle,
                  onPressed: _updateStream == null
                      ? null
                      : () async {
                          await Veilid.instance.shutdownVeilidCore();
                          if (_updateProcessor != null) {
                            await _updateProcessor;
                          }
                          setState(() {
                            _updateProcessor = null;
                            _updateStream = null;
                          });
                        },
                  child: const Text('Shutdown'),
                ),
              ])),
          Row(children: [
            Expanded(
                child: TextField(
                    decoration: const InputDecoration(
                        border: OutlineInputBorder(),
                        labelText: 'Debug Command'),
                    textInputAction: TextInputAction.send,
                    onSubmitted: (String v) async {
                      loggy.info(await Veilid.instance.debug(v));
                    })),
            DropdownButton<LogLevel>(
                value: loggy.level.logLevel,
                onChanged: (LogLevel? newLevel) {
                  setState(() {
                    setRootLogLevel(newLevel);
                  });
                },
                items: const [
                  DropdownMenuItem<LogLevel>(
                      value: LogLevel.error, child: Text("Error")),
                  DropdownMenuItem<LogLevel>(
                      value: LogLevel.warning, child: Text("Warning")),
                  DropdownMenuItem<LogLevel>(
                      value: LogLevel.info, child: Text("Info")),
                  DropdownMenuItem<LogLevel>(
                      value: LogLevel.debug, child: Text("Debug")),
                  DropdownMenuItem<LogLevel>(
                      value: traceLevel, child: Text("Trace")),
                  DropdownMenuItem<LogLevel>(
                      value: LogLevel.all, child: Text("All")),
                ])
          ]),
        ]));
  }
}
