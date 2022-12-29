import 'dart:async';
import 'dart:ffi';
import 'dart:io';
import 'dart:isolate';
import 'dart:convert';
import 'dart:typed_data';

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
final _dylib =
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
typedef _StartupVeilidCoreC = Void Function(Int64, Int64, Pointer<Utf8>);
typedef _StartupVeilidCoreDart = void Function(int, int, Pointer<Utf8>);
// fn get_veilid_state(port: i64)
typedef _GetVeilidStateC = Void Function(Int64);
typedef _GetVeilidStateDart = void Function(int);
// fn attach(port: i64)
typedef _AttachC = Void Function(Int64);
typedef _AttachDart = void Function(int);
// fn detach(port: i64)
typedef _DetachC = Void Function(Int64);
typedef _DetachDart = void Function(int);

// fn routing_context(port: i64)
typedef _RoutingContextC = Void Function(Int64);
typedef _RoutingContextDart = void Function(int);
// fn release_routing_context(id: u32)
typedef _ReleaseRoutingContextC = Int32 Function(Uint32);
typedef _ReleaseRoutingContextDart = int Function(int);
// fn routing_context_with_privacy(id: u32) -> u32
typedef _RoutingContextWithPrivacyC = Uint32 Function(Uint32);
typedef _RoutingContextWithPrivacyDart = int Function(int);
// fn routing_context_with_custom_privacy(id: u32, stability: FfiStr)
typedef _RoutingContextWithCustomPrivacyC = Uint32 Function(
    Uint32, Pointer<Utf8>);
typedef _RoutingContextWithCustomPrivacyDart = int Function(int, Pointer<Utf8>);
// fn routing_context_with_sequencing(id: u32, sequencing: FfiStr)
typedef _RoutingContextWithSequencingC = Uint32 Function(Uint32, Pointer<Utf8>);
typedef _RoutingContextWithSequencingDart = int Function(int, Pointer<Utf8>);
// fn routing_context_app_call(port: i64, id: u32, target: FfiStr, request: FfiStr)
typedef _RoutingContextAppCallC = Void Function(
    Int64, Uint32, Pointer<Utf8>, Pointer<Utf8>);
typedef _RoutingContextAppCallDart = void Function(
    int, int, Pointer<Utf8>, Pointer<Utf8>);
// fn routing_context_app_message(port: i64, id: u32, target: FfiStr, request: FfiStr)
typedef _RoutingContextAppMessageC = Void Function(
    Int64, Uint32, Pointer<Utf8>, Pointer<Utf8>);
typedef _RoutingContextAppMessageDart = void Function(
    int, int, Pointer<Utf8>, Pointer<Utf8>);

// fn new_private_route(port: i64)
typedef _NewPrivateRouteC = Void Function(Int64);
typedef _NewPrivateRouteDart = void Function(int);
// fn new_custom_private_route(port: i64, stability: FfiStr, sequencing: FfiStr)
typedef _NewCustomPrivateRouteC = Void Function(
    Int64, Pointer<Utf8>, Pointer<Utf8>);
typedef _NewCustomPrivateRouteDart = void Function(
    int, Pointer<Utf8>, Pointer<Utf8>);
// fn import_remote_private_route(port: i64, blob: FfiStr)
typedef _ImportRemotePrivateRouteC = Void Function(Int64, Pointer<Utf8>);
typedef _ImportRemotePrivateRouteDart = void Function(int, Pointer<Utf8>);
// fn release_private_route(port:i64, key: FfiStr)
typedef _ReleasePrivateRouteC = Void Function(Int64, Pointer<Utf8>);
typedef _ReleasePrivateRouteDart = void Function(int, Pointer<Utf8>);

// fn app_call_reply(port: i64, id: FfiStr, message: FfiStr)
typedef _AppCallReplyC = Void Function(Int64, Pointer<Utf8>, Pointer<Utf8>);
typedef _AppCallReplyDart = void Function(int, Pointer<Utf8>, Pointer<Utf8>);

// fn open_table_db(port: i64, name: FfiStr, column_count: u32)
typedef _OpenTableDbC = Void Function(Int64, Pointer<Utf8>, Uint32);
typedef _OpenTableDbDart = void Function(int, Pointer<Utf8>, int);
// fn release_table_db(id: u32) -> i32
typedef _ReleaseTableDbC = Int32 Function(Uint32);
typedef _ReleaseTableDbDart = int Function(int);
// fn delete_table_db(port: i64, name: FfiStr)
typedef _DeleteTableDbC = Void Function(Int64, Pointer<Utf8>);
typedef _DeleteTableDbDart = void Function(int, Pointer<Utf8>);
// fn table_db_get_column_count(id: u32) -> u32
typedef _TableDbGetColumnCountC = Uint32 Function(Uint32);
typedef _TableDbGetColumnCountDart = int Function(int);
// fn table_db_get_keys(id: u32, col: u32) -> *mut c_char
typedef _TableDbGetKeysC = Pointer<Utf8> Function(Uint32, Uint32);
typedef _TableDbGetKeysDart = Pointer<Utf8> Function(int, int);
// fn table_db_store(port: i64, id: u32, col: u32, key: FfiStr, value: FfiStr)
typedef _TableDbStoreC = Void Function(
    Int64, Uint32, Uint32, Pointer<Utf8>, Pointer<Utf8>);
typedef _TableDbStoreDart = void Function(
    int, int, int, Pointer<Utf8>, Pointer<Utf8>);
// fn table_db_load(port: i64, id: u32, col: u32, key: FfiStr)
typedef _TableDbLoadC = Void Function(Int64, Uint32, Uint32, Pointer<Utf8>);
typedef _TableDbLoadDart = void Function(int, int, int, Pointer<Utf8>);
// fn table_db_delete(port: i64, id: u32, col: u32, key: FfiStr)
typedef _TableDbDeleteC = Void Function(Int64, Uint32, Uint32, Pointer<Utf8>);
typedef _TableDbDeleteDart = void Function(int, int, int, Pointer<Utf8>);
// fn table_db_transact(id: u32) -> u32
typedef _TableDbTransactC = Uint32 Function(Uint32);
typedef _TableDbTransactDart = int Function(int);
// fn release_table_db_transaction(id: u32) -> i32
typedef _ReleaseTableDbTransactionC = Int32 Function(Uint32);
typedef _ReleaseTableDbTransactionDart = int Function(int);
// fn table_db_transaction_commit(port: i64, id: u32)
typedef _TableDbTransactionCommitC = Void Function(Uint64, Uint32);
typedef _TableDbTransactionCommitDart = void Function(int, int);
// fn table_db_transaction_rollback(port: i64, id: u32)
typedef _TableDbTransactionRollbackC = Void Function(Uint64, Uint32);
typedef _TableDbTransactionRollbackDart = void Function(int, int);
// fn table_db_transaction_store(port: i64, id: u32, col: u32, key: FfiStr, value: FfiStr)
typedef _TableDbTransactionStoreC = Void Function(
    Int64, Uint32, Uint32, Pointer<Utf8>, Pointer<Utf8>);
typedef _TableDbTransactionStoreDart = void Function(
    int, int, int, Pointer<Utf8>, Pointer<Utf8>);
// fn table_db_transaction_delete(port: i64, id: u32, col: u32, key: FfiStr)
typedef _TableDbTransactionDeleteC = Void Function(
    Int64, Uint32, Uint32, Pointer<Utf8>);
typedef _TableDbTransactionDeleteDart = void Function(
    int, int, int, Pointer<Utf8>);

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
    T Function(dynamic) jsonConstructor, Future<dynamic> future) {
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

Future<Stream<T>> processFutureStream<T>(
    Stream<T> returnStream, Future<dynamic> future) {
  return future.then((value) {
    final list = value as List<dynamic>;
    switch (list[0] as int) {
      case messageOk:
        {
          if (list[1] != null) {
            throw VeilidAPIExceptionInternal(
                "Unexpected MESSAGE_OK value '${list[1]}' where null expected");
          }
          return returnStream;
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
          return returnStream;
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

class _Ctx {
  final int id;
  final VeilidFFI ffi;
  _Ctx(this.id, this.ffi);
}

// FFI implementation of VeilidRoutingContext
class VeilidRoutingContextFFI implements VeilidRoutingContext {
  final _Ctx _ctx;
  static final Finalizer<_Ctx> _finalizer =
      Finalizer((ctx) => {ctx.ffi._releaseRoutingContext(ctx.id)});

  VeilidRoutingContextFFI._(this._ctx) {
    _finalizer.attach(this, _ctx, detach: this);
  }

  @override
  VeilidRoutingContextFFI withPrivacy() {
    final newId = _ctx.ffi._routingContextWithPrivacy(_ctx.id);
    return VeilidRoutingContextFFI._(_Ctx(newId, _ctx.ffi));
  }

  @override
  VeilidRoutingContextFFI withCustomPrivacy(Stability stability) {
    final newId = _ctx.ffi._routingContextWithCustomPrivacy(
        _ctx.id, stability.json.toNativeUtf8());
    return VeilidRoutingContextFFI._(_Ctx(newId, _ctx.ffi));
  }

  @override
  VeilidRoutingContextFFI withSequencing(Sequencing sequencing) {
    final newId = _ctx.ffi
        ._routingContextWithSequencing(_ctx.id, sequencing.json.toNativeUtf8());
    return VeilidRoutingContextFFI._(_Ctx(newId, _ctx.ffi));
  }

  @override
  Future<Uint8List> appCall(String target, Uint8List request) async {
    var nativeEncodedTarget = target.toNativeUtf8();
    var nativeEncodedRequest = base64UrlEncode(request).toNativeUtf8();

    final recvPort = ReceivePort("routing_context_app_call");
    final sendPort = recvPort.sendPort;
    _ctx.ffi._routingContextAppCall(sendPort.nativePort, _ctx.id,
        nativeEncodedTarget, nativeEncodedRequest);
    final out = await processFuturePlain(recvPort.first);
    return base64Decode(out);
  }

  @override
  Future<void> appMessage(String target, Uint8List message) async {
    final nativeEncodedTarget = target.toNativeUtf8();
    final nativeEncodedMessage = base64UrlEncode(message).toNativeUtf8();

    final recvPort = ReceivePort("routing_context_app_message");
    final sendPort = recvPort.sendPort;
    _ctx.ffi._routingContextAppMessage(sendPort.nativePort, _ctx.id,
        nativeEncodedTarget, nativeEncodedMessage);
    return processFutureVoid(recvPort.first);
  }
}

class _TDBT {
  final int id;
  VeilidTableDBFFI tdbffi;
  VeilidFFI ffi;

  _TDBT(this.id, this.tdbffi, this.ffi);
}

// FFI implementation of VeilidTableDBTransaction
class VeilidTableDBTransactionFFI extends VeilidTableDBTransaction {
  final _TDBT _tdbt;
  static final Finalizer<_TDBT> _finalizer =
      Finalizer((tdbt) => {tdbt.ffi._releaseTableDbTransaction(tdbt.id)});

  VeilidTableDBTransactionFFI._(this._tdbt) {
    _finalizer.attach(this, _tdbt, detach: this);
  }

  @override
  Future<void> commit() {
    final recvPort = ReceivePort("veilid_table_db_transaction_commit");
    final sendPort = recvPort.sendPort;
    _tdbt.ffi._tableDbTransactionCommit(
      sendPort.nativePort,
      _tdbt.id,
    );
    return processFutureVoid(recvPort.first);
  }

  @override
  Future<void> rollback() {
    final recvPort = ReceivePort("veilid_table_db_transaction_rollback");
    final sendPort = recvPort.sendPort;
    _tdbt.ffi._tableDbTransactionRollback(
      sendPort.nativePort,
      _tdbt.id,
    );
    return processFutureVoid(recvPort.first);
  }

  @override
  Future<void> store(int col, Uint8List key, Uint8List value) {
    final nativeEncodedKey = base64UrlEncode(key).toNativeUtf8();
    final nativeEncodedValue = base64UrlEncode(value).toNativeUtf8();

    final recvPort = ReceivePort("veilid_table_db_transaction_store");
    final sendPort = recvPort.sendPort;
    _tdbt.ffi._tableDbTransactionStore(
      sendPort.nativePort,
      _tdbt.id,
      col,
      nativeEncodedKey,
      nativeEncodedValue,
    );
    return processFutureVoid(recvPort.first);
  }

  @override
  Future<bool> delete(int col, Uint8List key) {
    final nativeEncodedKey = base64UrlEncode(key).toNativeUtf8();

    final recvPort = ReceivePort("veilid_table_db_transaction_delete");
    final sendPort = recvPort.sendPort;
    _tdbt.ffi._tableDbTransactionDelete(
      sendPort.nativePort,
      _tdbt.id,
      col,
      nativeEncodedKey,
    );
    return processFuturePlain(recvPort.first);
  }
}

class _TDB {
  final int id;
  VeilidFFI ffi;
  _TDB(this.id, this.ffi);
}

// FFI implementation of VeilidTableDB
class VeilidTableDBFFI extends VeilidTableDB {
  final _TDB _tdb;
  static final Finalizer<_TDB> _finalizer =
      Finalizer((tdb) => {tdb.ffi._releaseTableDb(tdb.id)});

  VeilidTableDBFFI._(this._tdb) {
    _finalizer.attach(this, _tdb, detach: this);
  }

  @override
  int getColumnCount() {
    return _tdb.ffi._tableDbGetColumnCount(_tdb.id);
  }

  @override
  List<Uint8List> getKeys(int col) {
    final s = _tdb.ffi._tableDbGetKeys(_tdb.id, col);
    if (s.address == nullptr.address) {
      throw VeilidAPIExceptionInternal("No db for id");
    }
    String ja = s.toDartString();
    _tdb.ffi._freeString(s);
    List<dynamic> jarr = jsonDecode(ja);
    return jarr.map((e) => base64Decode(e)).toList();
  }

  @override
  VeilidTableDBTransaction transact() {
    final id = _tdb.ffi._tableDbTransact(_tdb.id);
    return VeilidTableDBTransactionFFI._(_TDBT(id, this, _tdb.ffi));
  }

  @override
  Future<void> store(int col, Uint8List key, Uint8List value) {
    final nativeEncodedKey = base64UrlEncode(key).toNativeUtf8();
    final nativeEncodedValue = base64UrlEncode(value).toNativeUtf8();

    final recvPort = ReceivePort("veilid_table_db_store");
    final sendPort = recvPort.sendPort;
    _tdb.ffi._tableDbStore(
      sendPort.nativePort,
      _tdb.id,
      col,
      nativeEncodedKey,
      nativeEncodedValue,
    );
    return processFutureVoid(recvPort.first);
  }

  @override
  Future<Uint8List?> load(int col, Uint8List key) async {
    final nativeEncodedKey = base64UrlEncode(key).toNativeUtf8();

    final recvPort = ReceivePort("veilid_table_db_load");
    final sendPort = recvPort.sendPort;
    _tdb.ffi._tableDbLoad(
      sendPort.nativePort,
      _tdb.id,
      col,
      nativeEncodedKey,
    );
    String? out = await processFuturePlain(recvPort.first);
    if (out == null) {
      return null;
    }
    return base64Decode(out);
  }

  @override
  Future<bool> delete(int col, Uint8List key) {
    final nativeEncodedKey = base64UrlEncode(key).toNativeUtf8();

    final recvPort = ReceivePort("veilid_table_db_delete");
    final sendPort = recvPort.sendPort;
    _tdb.ffi._tableDbLoad(
      sendPort.nativePort,
      _tdb.id,
      col,
      nativeEncodedKey,
    );
    return processFuturePlain(recvPort.first);
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

  final _RoutingContextDart _routingContext;
  final _ReleaseRoutingContextDart _releaseRoutingContext;
  final _RoutingContextWithPrivacyDart _routingContextWithPrivacy;
  final _RoutingContextWithCustomPrivacyDart _routingContextWithCustomPrivacy;
  final _RoutingContextWithSequencingDart _routingContextWithSequencing;
  final _RoutingContextAppCallDart _routingContextAppCall;
  final _RoutingContextAppMessageDart _routingContextAppMessage;

  final _NewPrivateRouteDart _newPrivateRoute;
  final _NewCustomPrivateRouteDart _newCustomPrivateRoute;
  final _ImportRemotePrivateRouteDart _importRemotePrivateRoute;
  final _ReleasePrivateRouteDart _releasePrivateRoute;

  final _AppCallReplyDart _appCallReply;

  final _OpenTableDbDart _openTableDb;
  final _ReleaseTableDbDart _releaseTableDb;
  final _DeleteTableDbDart _deleteTableDb;
  final _TableDbGetColumnCountDart _tableDbGetColumnCount;
  final _TableDbGetKeysDart _tableDbGetKeys;
  final _TableDbStoreDart _tableDbStore;
  final _TableDbLoadDart _tableDbLoad;
  final _TableDbDeleteDart _tableDbDelete;
  final _TableDbTransactDart _tableDbTransact;
  final _ReleaseTableDbTransactionDart _releaseTableDbTransaction;
  final _TableDbTransactionCommitDart _tableDbTransactionCommit;
  final _TableDbTransactionRollbackDart _tableDbTransactionRollback;
  final _TableDbTransactionStoreDart _tableDbTransactionStore;
  final _TableDbTransactionDeleteDart _tableDbTransactionDelete;

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
        _routingContext =
            dylib.lookupFunction<_RoutingContextC, _RoutingContextDart>(
                'routing_context'),
        _releaseRoutingContext = dylib.lookupFunction<_ReleaseRoutingContextC,
            _ReleaseRoutingContextDart>('release_routing_context'),
        _routingContextWithPrivacy = dylib.lookupFunction<
            _RoutingContextWithPrivacyC,
            _RoutingContextWithPrivacyDart>('routing_context_with_privacy'),
        _routingContextWithCustomPrivacy = dylib.lookupFunction<
                _RoutingContextWithCustomPrivacyC,
                _RoutingContextWithCustomPrivacyDart>(
            'routing_context_with_custom_privacy'),
        _routingContextWithSequencing = dylib.lookupFunction<
                _RoutingContextWithSequencingC,
                _RoutingContextWithSequencingDart>(
            'routing_context_with_sequencing'),
        _routingContextAppCall = dylib.lookupFunction<_RoutingContextAppCallC,
            _RoutingContextAppCallDart>('routing_context_app_call'),
        _routingContextAppMessage = dylib.lookupFunction<
            _RoutingContextAppMessageC,
            _RoutingContextAppMessageDart>('routing_context_app_message'),
        _newPrivateRoute =
            dylib.lookupFunction<_NewPrivateRouteC, _NewPrivateRouteDart>(
                'new_private_route'),
        _newCustomPrivateRoute = dylib.lookupFunction<_NewCustomPrivateRouteC,
            _NewCustomPrivateRouteDart>('new_custom_private_route'),
        _importRemotePrivateRoute = dylib.lookupFunction<
            _ImportRemotePrivateRouteC,
            _ImportRemotePrivateRouteDart>('import_remote_private_route'),
        _releasePrivateRoute = dylib.lookupFunction<_ReleasePrivateRouteC,
            _ReleasePrivateRouteDart>('release_private_route'),
        _appCallReply = dylib.lookupFunction<_AppCallReplyC, _AppCallReplyDart>(
            'app_call_reply'),
        _openTableDb = dylib
            .lookupFunction<_OpenTableDbC, _OpenTableDbDart>('open_table_db'),
        _releaseTableDb =
            dylib.lookupFunction<_ReleaseTableDbC, _ReleaseTableDbDart>(
                'release_table_db'),
        _deleteTableDb =
            dylib.lookupFunction<_DeleteTableDbC, _DeleteTableDbDart>(
                'delete_table_db'),
        _tableDbGetColumnCount = dylib.lookupFunction<_TableDbGetColumnCountC,
            _TableDbGetColumnCountDart>('table_db_get_column_count'),
        _tableDbGetKeys =
            dylib.lookupFunction<_TableDbGetKeysC, _TableDbGetKeysDart>(
                'table_db_get_keys'),
        _tableDbStore = dylib.lookupFunction<_TableDbStoreC, _TableDbStoreDart>(
            'table_db_store'),
        _tableDbLoad = dylib
            .lookupFunction<_TableDbLoadC, _TableDbLoadDart>('table_db_load'),
        _tableDbDelete =
            dylib.lookupFunction<_TableDbDeleteC, _TableDbDeleteDart>(
                'table_db_delete'),
        _tableDbTransact =
            dylib.lookupFunction<_TableDbTransactC, _TableDbTransactDart>(
                'table_db_transact'),
        _releaseTableDbTransaction = dylib.lookupFunction<
            _ReleaseTableDbTransactionC,
            _ReleaseTableDbTransactionDart>('release_table_db_transaction'),
        _tableDbTransactionCommit = dylib.lookupFunction<
            _TableDbTransactionCommitC,
            _TableDbTransactionCommitDart>('table_db_transaction_commit'),
        _tableDbTransactionRollback = dylib.lookupFunction<
            _TableDbTransactionRollbackC,
            _TableDbTransactionRollbackDart>('table_db_transaction_rollback'),
        _tableDbTransactionStore = dylib.lookupFunction<
            _TableDbTransactionStoreC,
            _TableDbTransactionStoreDart>('table_db_transaction_store'),
        _tableDbTransactionDelete = dylib.lookupFunction<
            _TableDbTransactionDeleteC,
            _TableDbTransactionDeleteDart>('table_db_transaction_delete'),
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
  Future<Stream<VeilidUpdate>> startupVeilidCore(VeilidConfig config) {
    var nativeConfig =
        jsonEncode(config.json, toEncodable: veilidApiToEncodable)
            .toNativeUtf8();
    final recvStreamPort = ReceivePort("veilid_api_stream");
    final sendStreamPort = recvStreamPort.sendPort;
    final recvPort = ReceivePort("startup_veilid_core");
    final sendPort = recvPort.sendPort;
    _startupVeilidCore(
        sendPort.nativePort, sendStreamPort.nativePort, nativeConfig);
    malloc.free(nativeConfig);
    return processFutureStream(
        processStreamJson(VeilidUpdate.fromJson, recvStreamPort),
        recvPort.first);
  }

  @override
  Future<VeilidState> getVeilidState() {
    final recvPort = ReceivePort("get_veilid_state");
    final sendPort = recvPort.sendPort;
    _getVeilidState(sendPort.nativePort);
    return processFutureJson(VeilidState.fromJson, recvPort.first);
  }

  @override
  Future<void> attach() {
    final recvPort = ReceivePort("attach");
    final sendPort = recvPort.sendPort;
    _attach(sendPort.nativePort);
    return processFutureVoid(recvPort.first);
  }

  @override
  Future<void> detach() {
    final recvPort = ReceivePort("detach");
    final sendPort = recvPort.sendPort;
    _detach(sendPort.nativePort);
    return processFutureVoid(recvPort.first);
  }

  @override
  Future<void> shutdownVeilidCore() {
    final recvPort = ReceivePort("shutdown_veilid_core");
    final sendPort = recvPort.sendPort;
    _shutdownVeilidCore(sendPort.nativePort);
    return processFutureVoid(recvPort.first);
  }

  @override
  Future<VeilidRoutingContext> routingContext() async {
    final recvPort = ReceivePort("routing_context");
    final sendPort = recvPort.sendPort;
    _routingContext(sendPort.nativePort);
    final id = await processFuturePlain(recvPort.first);
    return VeilidRoutingContextFFI._(_Ctx(id, this));
  }

  @override
  Future<KeyBlob> newPrivateRoute() {
    final recvPort = ReceivePort("new_private_route");
    final sendPort = recvPort.sendPort;
    _newPrivateRoute(sendPort.nativePort);
    return processFutureJson(KeyBlob.fromJson, recvPort.first);
  }

  @override
  Future<KeyBlob> newCustomPrivateRoute(
      Stability stability, Sequencing sequencing) async {
    final recvPort = ReceivePort("new_custom_private_route");
    final sendPort = recvPort.sendPort;
    _newCustomPrivateRoute(sendPort.nativePort, stability.json.toNativeUtf8(),
        sequencing.json.toNativeUtf8());
    final keyblob = await processFutureJson(KeyBlob.fromJson, recvPort.first);
    return keyblob;
  }

  @override
  Future<String> importRemotePrivateRoute(Uint8List blob) {
    final nativeEncodedBlob = base64UrlEncode(blob).toNativeUtf8();

    final recvPort = ReceivePort("import_remote_private_route");
    final sendPort = recvPort.sendPort;
    _importRemotePrivateRoute(sendPort.nativePort, nativeEncodedBlob);
    return processFuturePlain(recvPort.first);
  }

  @override
  Future<void> releasePrivateRoute(String key) {
    final nativeEncodedKey = key.toNativeUtf8();

    final recvPort = ReceivePort("release_private_route");
    final sendPort = recvPort.sendPort;
    _releasePrivateRoute(sendPort.nativePort, nativeEncodedKey);
    return processFutureVoid(recvPort.first);
  }

  @override
  Future<void> appCallReply(String id, Uint8List message) {
    final nativeId = id.toNativeUtf8();
    final nativeEncodedMessage = base64UrlEncode(message).toNativeUtf8();
    final recvPort = ReceivePort("app_call_reply");
    final sendPort = recvPort.sendPort;
    _appCallReply(sendPort.nativePort, nativeId, nativeEncodedMessage);
    return processFutureVoid(recvPort.first);
  }

  @override
  Future<VeilidTableDB> openTableDB(String name, int columnCount) async {
    final recvPort = ReceivePort("open_table_db");
    final sendPort = recvPort.sendPort;
    _openTableDb(sendPort.nativePort, name.toNativeUtf8(), columnCount);
    final id = await processFuturePlain(recvPort.first);
    return VeilidTableDBFFI._(_TDB(id, this));
  }

  @override
  Future<bool> deleteTableDB(String name) async {
    final recvPort = ReceivePort("delete_table_db");
    final sendPort = recvPort.sendPort;
    _deleteTableDb(sendPort.nativePort, name.toNativeUtf8());
    final deleted = await processFuturePlain(recvPort.first);
    return deleted;
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
