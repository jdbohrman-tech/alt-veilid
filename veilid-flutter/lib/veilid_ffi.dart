import 'dart:async';
import 'dart:ffi';
import 'dart:io';
import 'dart:isolate';
import 'dart:convert';

import 'package:ffi/ffi.dart';

import 'veilid.dart';

//////////////////////////////////////////////////////////

// Load the veilid_flutter library once
const _base = 'veilid_flutter';
final _path = Platform.isWindows
    ? '$_base.dll'
    : Platform.isMacOS
        ? 'lib$_base.dylib'
        : 'lib$_base.so';
late final _dylib =
    Platform.isIOS ? DynamicLibrary.process() : DynamicLibrary.open(_path);

// Linkage for initialization
typedef _DartPostCObject
    = NativeFunction<Int8 Function(Int64, Pointer<Dart_CObject>)>;
// fn free_string(s: *mut std::os::raw::c_char)
typedef _FreeStringC = Void Function(Pointer<Utf8>);
typedef _FreeStringDart = void Function(Pointer<Utf8>);
// fn initialize_veilid_flutter(dart_post_c_object_ptr: ffi::DartPostCObjectFnType)
typedef _InitializeVeilidFlutterC = Void Function(Pointer<_DartPostCObject>);
typedef _InitializeVeilidFlutterDart = void Function(Pointer<_DartPostCObject>);
// fn initialize_veilid_core(platform_config: FfiStr)
typedef _InitializeVeilidCoreC = Void Function(Pointer<Utf8>);
typedef _InitializeVeilidCoreDart = void Function(Pointer<Utf8>);
// fn change_log_level(layer: FfiStr, log_level: FfiStr)
typedef _ChangeLogLevelC = Void Function(Pointer<Utf8>, Pointer<Utf8>);
typedef _ChangeLogLevelDart = void Function(Pointer<Utf8>, Pointer<Utf8>);
// fn startup_veilid_core(port: i64, config: FfiStr)
typedef _StartupVeilidCoreC = Void Function(Int64, Pointer<Utf8>);
typedef _StartupVeilidCoreDart = void Function(int, Pointer<Utf8>);
// fn get_veilid_state(port: i64)
typedef _GetVeilidStateC = Void Function(Int64);
typedef _GetVeilidStateDart = void Function(int);
// fn attach(port: i64)
typedef _AttachC = Void Function(Int64);
typedef _AttachDart = void Function(int);
// fn detach(port: i64)
typedef _DetachC = Void Function(Int64);
typedef _DetachDart = void Function(int);
// fn debug(port: i64, log_level: FfiStr)
typedef _DebugC = Void Function(Int64, Pointer<Utf8>);
typedef _DebugDart = void Function(int, Pointer<Utf8>);
// fn shutdown_veilid_core(port: i64)
typedef _ShutdownVeilidCoreC = Void Function(Int64);
typedef _ShutdownVeilidCoreDart = void Function(int);
// fn veilid_version_string() -> *mut c_char
typedef _VeilidVersionStringC = Pointer<Utf8> Function();
typedef _VeilidVersionStringDart = Pointer<Utf8> Function();

// fn veilid_version() -> VeilidVersion
class VeilidVersionFFI extends Struct {
  @Uint32()
  external int major;
  @Uint32()
  external int minor;
  @Uint32()
  external int patch;
}

typedef _VeilidVersionC = VeilidVersionFFI Function();
typedef _VeilidVersionDart = VeilidVersionFFI Function();

// Async message types
const int messageOk = 0;
const int messageErr = 1;
const int messageOkJson = 2;
const int messageErrJson = 3;
const int messageStreamItem = 4;
const int messageStreamItemJson = 5;
const int messageStreamAbort = 6;
const int messageStreamAbortJson = 7;
const int messageStreamClose = 8;

// Interface factory for high level Veilid API
Veilid getVeilid() => VeilidFFI(_dylib);

// Parse handle async returns
Future<T> processFuturePlain<T>(Future<dynamic> future) {
  return future.then((value) {
    final list = value as List<dynamic>;
    switch (list[0] as int) {
      case messageOk:
        {
          if (list[1] == null) {
            throw VeilidAPIExceptionInternal("Null MESSAGE_OK value");
          }
          return list[1] as T;
        }
      case messageErr:
        {
          throw VeilidAPIExceptionInternal("Internal API Error: ${list[1]}");
        }
      case messageErrJson:
        {
          throw VeilidAPIException.fromJson(jsonDecode(list[1]));
        }
      default:
        {
          throw VeilidAPIExceptionInternal(
              "Unexpected async return message type: ${list[0]}");
        }
    }
  }).catchError((e) {
    // Wrap all other errors in VeilidAPIExceptionInternal
    throw VeilidAPIExceptionInternal(e.toString());
  }, test: (e) {
    // Pass errors that are already VeilidAPIException through without wrapping
    return e is! VeilidAPIException;
  });
}

Future<T> processFutureJson<T>(
    T Function(Map<String, dynamic>) jsonConstructor, Future<dynamic> future) {
  return future.then((value) {
    final list = value as List<dynamic>;
    switch (list[0] as int) {
      case messageErr:
        {
          throw VeilidAPIExceptionInternal("Internal API Error: ${list[1]}");
        }
      case messageOkJson:
        {
          if (list[1] == null) {
            throw VeilidAPIExceptionInternal("Null MESSAGE_OK_JSON value");
          }
          var ret = jsonDecode(list[1] as String);
          return jsonConstructor(ret);
        }
      case messageErrJson:
        {
          throw VeilidAPIException.fromJson(jsonDecode(list[1]));
        }
      default:
        {
          throw VeilidAPIExceptionInternal(
              "Unexpected async return message type: ${list[0]}");
        }
    }
  }).catchError((e) {
    // Wrap all other errors in VeilidAPIExceptionInternal
    throw VeilidAPIExceptionInternal(e.toString());
  }, test: (e) {
    // Pass errors that are already VeilidAPIException through without wrapping
    return e is! VeilidAPIException;
  });
}

Future<void> processFutureVoid(Future<dynamic> future) {
  return future.then((value) {
    final list = value as List<dynamic>;
    switch (list[0] as int) {
      case messageOk:
        {
          if (list[1] != null) {
            throw VeilidAPIExceptionInternal(
                "Unexpected MESSAGE_OK value '${list[1]}' where null expected");
          }
          return;
        }
      case messageErr:
        {
          throw VeilidAPIExceptionInternal("Internal API Error: ${list[1]}");
        }
      case messageOkJson:
        {
          var ret = jsonDecode(list[1] as String);
          if (ret != null) {
            throw VeilidAPIExceptionInternal(
                "Unexpected MESSAGE_OK_JSON value '$ret' where null expected");
          }
          return;
        }
      case messageErrJson:
        {
          throw VeilidAPIException.fromJson(jsonDecode(list[1] as String));
        }
      default:
        {
          throw VeilidAPIExceptionInternal(
              "Unexpected async return message type: ${list[0]}");
        }
    }
  }).catchError((e) {
    // Wrap all other errors in VeilidAPIExceptionInternal
    throw VeilidAPIExceptionInternal(e.toString());
  }, test: (e) {
    // Pass errors that are already VeilidAPIException through without wrapping
    return e is! VeilidAPIException;
  });
}

Stream<T> processStreamJson<T>(
    T Function(Map<String, dynamic>) jsonConstructor, ReceivePort port) async* {
  try {
    await for (var value in port) {
      final list = value as List<dynamic>;
      switch (list[0] as int) {
        case messageStreamItemJson:
          {
            if (list[1] == null) {
              throw VeilidAPIExceptionInternal(
                  "Null MESSAGE_STREAM_ITEM_JSON value");
            }
            var ret = jsonDecode(list[1] as String);
            yield jsonConstructor(ret);
            break;
          }
        case messageStreamAbort:
          {
            port.close();
            throw VeilidAPIExceptionInternal("Internal API Error: ${list[1]}");
          }
        case messageStreamAbortJson:
          {
            port.close();
            throw VeilidAPIException.fromJson(jsonDecode(list[1]));
          }
        case messageStreamClose:
          {
            port.close();
            break;
          }
        default:
          {
            throw VeilidAPIExceptionInternal(
                "Unexpected async return message type: ${list[0]}");
          }
      }
    }
  } catch (e, s) {
    // Wrap all other errors in VeilidAPIExceptionInternal
    throw VeilidAPIExceptionInternal(
        "${e.toString()}\nStack Trace:\n${s.toString()}");
  }
}

// FFI implementation of high level Veilid API
class VeilidFFI implements Veilid {
  // veilid_core shared library
  final DynamicLibrary _dylib;

  // Shared library functions
  final _FreeStringDart _freeString;
  final _InitializeVeilidCoreDart _initializeVeilidCore;
  final _ChangeLogLevelDart _changeLogLevel;
  final _StartupVeilidCoreDart _startupVeilidCore;
  final _GetVeilidStateDart _getVeilidState;
  final _AttachDart _attach;
  final _DetachDart _detach;
  final _ShutdownVeilidCoreDart _shutdownVeilidCore;
  final _DebugDart _debug;
  final _VeilidVersionStringDart _veilidVersionString;
  final _VeilidVersionDart _veilidVersion;

  VeilidFFI(DynamicLibrary dylib)
      : _dylib = dylib,
        _freeString =
            dylib.lookupFunction<_FreeStringC, _FreeStringDart>('free_string'),
        _initializeVeilidCore = dylib.lookupFunction<_InitializeVeilidCoreC,
            _InitializeVeilidCoreDart>('initialize_veilid_core'),
        _changeLogLevel =
            dylib.lookupFunction<_ChangeLogLevelC, _ChangeLogLevelDart>(
                'change_log_level'),
        _startupVeilidCore =
            dylib.lookupFunction<_StartupVeilidCoreC, _StartupVeilidCoreDart>(
                'startup_veilid_core'),
        _getVeilidState =
            dylib.lookupFunction<_GetVeilidStateC, _GetVeilidStateDart>(
                'get_veilid_state'),
        _attach = dylib.lookupFunction<_AttachC, _AttachDart>('attach'),
        _detach = dylib.lookupFunction<_DetachC, _DetachDart>('detach'),
        _shutdownVeilidCore =
            dylib.lookupFunction<_ShutdownVeilidCoreC, _ShutdownVeilidCoreDart>(
                'shutdown_veilid_core'),
        _debug = dylib.lookupFunction<_DebugC, _DebugDart>('debug'),
        _veilidVersionString = dylib.lookupFunction<_VeilidVersionStringC,
            _VeilidVersionStringDart>('veilid_version_string'),
        _veilidVersion =
            dylib.lookupFunction<_VeilidVersionC, _VeilidVersionDart>(
                'veilid_version') {
    // Get veilid_flutter initializer
    var initializeVeilidFlutter = _dylib.lookupFunction<
        _InitializeVeilidFlutterC,
        _InitializeVeilidFlutterDart>('initialize_veilid_flutter');
    initializeVeilidFlutter(NativeApi.postCObject);
  }

  @override
  void initializeVeilidCore(Map<String, dynamic> platformConfigJson) {
    var nativePlatformConfig =
        jsonEncode(platformConfigJson, toEncodable: veilidApiToEncodable)
            .toNativeUtf8();

    _initializeVeilidCore(nativePlatformConfig);

    malloc.free(nativePlatformConfig);
  }

  @override
  void changeLogLevel(String layer, VeilidConfigLogLevel logLevel) {
    var nativeLogLevel =
        jsonEncode(logLevel.json, toEncodable: veilidApiToEncodable)
            .toNativeUtf8();
    var nativeLayer = layer.toNativeUtf8();
    _changeLogLevel(nativeLayer, nativeLogLevel);
    malloc.free(nativeLayer);
    malloc.free(nativeLogLevel);
  }

  @override
  Stream<VeilidUpdate> startupVeilidCore(VeilidConfig config) {
    var nativeConfig =
        jsonEncode(config.json, toEncodable: veilidApiToEncodable)
            .toNativeUtf8();
    final recvPort = ReceivePort("startup_veilid_core");
    final sendPort = recvPort.sendPort;
    _startupVeilidCore(sendPort.nativePort, nativeConfig);
    malloc.free(nativeConfig);
    return processStreamJson(VeilidUpdate.fromJson, recvPort);
  }

  @override
  Future<VeilidState> getVeilidState() async {
    final recvPort = ReceivePort("get_veilid_state");
    final sendPort = recvPort.sendPort;
    _getVeilidState(sendPort.nativePort);
    return processFutureJson(VeilidState.fromJson, recvPort.first);
  }

  @override
  Future<void> attach() async {
    final recvPort = ReceivePort("attach");
    final sendPort = recvPort.sendPort;
    _attach(sendPort.nativePort);
    return processFutureVoid(recvPort.first);
  }

  @override
  Future<void> detach() async {
    final recvPort = ReceivePort("detach");
    final sendPort = recvPort.sendPort;
    _detach(sendPort.nativePort);
    return processFutureVoid(recvPort.first);
  }

  @override
  Future<void> shutdownVeilidCore() async {
    final recvPort = ReceivePort("shutdown_veilid_core");
    final sendPort = recvPort.sendPort;
    _shutdownVeilidCore(sendPort.nativePort);
    return processFutureVoid(recvPort.first);
  }

  @override
  Future<String> debug(String command) async {
    var nativeCommand = command.toNativeUtf8();
    final recvPort = ReceivePort("debug");
    final sendPort = recvPort.sendPort;
    _debug(sendPort.nativePort, nativeCommand);
    return processFuturePlain(recvPort.first);
  }

  @override
  String veilidVersionString() {
    final versionString = _veilidVersionString();
    String ret = versionString.toDartString();
    _freeString(versionString);
    return ret;
  }

  @override
  VeilidVersion veilidVersion() {
    final version = _veilidVersion();
    return VeilidVersion(
      version.major,
      version.minor,
      version.patch,
    );
  }
}
