import 'package:flutter/material.dart';
import 'package:flutter/foundation.dart';
import 'package:veilid/veilid.dart';
import 'package:loggy/loggy.dart';
import 'package:ansicolor/ansicolor.dart';

// Loggy tools
const LogLevel traceLevel = LogLevel('Trace', 1);

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

String wrapWithLogColor(LogLevel? level, String text) {
  if (level == null) {
    return text;
  }
  final pen = AnsiPen();
  ansiColorDisabled = false;
  switch (level) {
    case LogLevel.error:
      pen
        ..reset()
        ..red(bold: true);
      return pen(text);
    case LogLevel.warning:
      pen
        ..reset()
        ..yellow(bold: true);
      return pen(text);
    case LogLevel.info:
      pen
        ..reset()
        ..white(bold: true);
      return pen(text);
    case LogLevel.debug:
      pen
        ..reset()
        ..green(bold: true);
      return pen(text);
    case traceLevel:
      pen
        ..reset()
        ..blue(bold: true);
      return pen(text);
  }
  return text;
}

void setRootLogLevel(LogLevel? level) {
  Loggy('').level = getLogOptions(level);
  Veilid.instance.changeLogLevel("all", convertToVeilidConfigLogLevel(level));
}

extension PrettyPrintLogRecord on LogRecord {
  String pretty() {
    final lstr =
        wrapWithLogColor(level, '[${level.toString().substring(0, 1)}]');
    return '$lstr $message';
  }
}

class CallbackPrinter extends LoggyPrinter {
  CallbackPrinter() : super();

  void Function(LogRecord)? callback;

  @override
  void onLog(LogRecord record) {
    debugPrint(record.pretty());
    callback?.call(record);
  }

  void setCallback(Function(LogRecord)? cb) {
    callback = cb;
  }
}

var globalTerminalPrinter = CallbackPrinter();

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

void initLoggy() {
  Loggy.initLoggy(
    logPrinter: globalTerminalPrinter,
    logOptions: getLogOptions(null),
  );

  setRootLogLevel(LogLevel.info);
}
