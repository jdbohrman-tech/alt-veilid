import 'dart:async';
import 'dart:convert';
import 'dart:ffi';
import 'dart:io';
import 'dart:isolate';
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
typedef _FreeStringDart = void Function(Pointer<Utf8>);
// fn initialize_veilid_flutter(
//    dart_post_c_object_ptr: ffi::DartPostCObjectFnType)
// fn initialize_veilid_core(platform_config: FfiStr)
typedef _InitializeVeilidCoreDart = void Function(Pointer<Utf8>);
// fn change_log_level(layer: FfiStr, log_level: FfiStr)
typedef _ChangeLogLevelDart = void Function(Pointer<Utf8>, Pointer<Utf8>);
// fn startup_veilid_core(port: i64, config: FfiStr)
typedef _StartupVeilidCoreDart = void Function(int, int, Pointer<Utf8>);
// fn get_veilid_state(port: i64)
typedef _GetVeilidStateDart = void Function(int);
// fn attach(port: i64)
typedef _AttachDart = void Function(int);
// fn detach(port: i64)
typedef _DetachDart = void Function(int);

// fn routing_context(port: i64)
typedef _RoutingContextDart = void Function(int);
// fn release_routing_context(id: u32)
typedef _ReleaseRoutingContextDart = int Function(int);
// fn routing_context_with_default_safety(id: u32) -> u32
typedef _RoutingContextWithDefaultSafetyDart = int Function(int);
// fn routing_context_with_safety(id: u32, stability: FfiStr)
typedef _RoutingContextWithSafetyDart = int Function(int, Pointer<Utf8>);
// fn routing_context_with_sequencing(id: u32, sequencing: FfiStr)
typedef _RoutingContextWithSequencingDart = int Function(int, Pointer<Utf8>);
// fn routing_context_safety(port: i64,
//    id: u32)
typedef _RoutingContextSafetyDart = void Function(int, int);
// fn routing_context_app_call(port: i64,
//    id: u32, target: FfiStr, request: FfiStr)
typedef _RoutingContextAppCallDart = void Function(
    int, int, Pointer<Utf8>, Pointer<Utf8>);
// fn routing_context_app_message(port: i64,
//    id: u32, target: FfiStr, request: FfiStr)
typedef _RoutingContextAppMessageDart = void Function(
    int, int, Pointer<Utf8>, Pointer<Utf8>);
// fn routing_context_create_dht_record(port: i64,
//    id: u32, kind: u32, schema: FfiStr)
typedef _RoutingContextCreateDHTRecordDart = void Function(
    int, int, Pointer<Utf8>, int);
// fn routing_context_open_dht_record(port: i64,
//    id: u32, key: FfiStr, writer: FfiStr)
typedef _RoutingContextOpenDHTRecordDart = void Function(
    int, int, Pointer<Utf8>, Pointer<Utf8>);
// fn routing_context_close_dht_record(port: i64, id: u32, key: FfiStr)
typedef _RoutingContextCloseDHTRecordDart = void Function(
    int, int, Pointer<Utf8>);
// fn routing_context_delete_dht_record(port: i64, id: u32, key: FfiStr)
typedef _RoutingContextDeleteDHTRecordDart = void Function(
    int, int, Pointer<Utf8>);
// fn routing_context_get_dht_value(port: i64,
//    id: u32, key: FfiStr, subkey: u32, force_refresh: bool)
typedef _RoutingContextGetDHTValueDart = void Function(
    int, int, Pointer<Utf8>, int, bool);
// fn routing_context_set_dht_value(port: i64,
//    id: u32, key: FfiStr, subkey: u32, data: FfiStr)
typedef _RoutingContextSetDHTValueDart = void Function(
    int, int, Pointer<Utf8>, int, Pointer<Utf8>);
// fn routing_context_watch_dht_values(port: i64,
//     id: u32, key: FfiStr, subkeys: FfiStr, expiration: FfiStr, count: u32)
typedef _RoutingContextWatchDHTValuesDart = void Function(
    int, int, Pointer<Utf8>, Pointer<Utf8>, int, int);
// fn routing_context_cancel_dht_watch(port: i64,
//     id: u32, key: FfiStr, subkeys: FfiStr)
typedef _RoutingContextCancelDHTWatchDart = void Function(
    int, int, Pointer<Utf8>, Pointer<Utf8>);

// fn new_private_route(port: i64)
typedef _NewPrivateRouteDart = void Function(int);
// fn new_custom_private_route(port: i64, stability: FfiStr, sequencing: FfiStr)
typedef _NewCustomPrivateRouteDart = void Function(
    int, Pointer<Utf8>, Pointer<Utf8>);
// fn import_remote_private_route(port: i64, blob: FfiStr)
typedef _ImportRemotePrivateRouteDart = void Function(int, Pointer<Utf8>);
// fn release_private_route(port:i64, key: FfiStr)
typedef _ReleasePrivateRouteDart = void Function(int, Pointer<Utf8>);

// fn app_call_reply(port: i64, id: FfiStr, message: FfiStr)
typedef _AppCallReplyDart = void Function(int, Pointer<Utf8>, Pointer<Utf8>);

// fn open_table_db(port: i64, name: FfiStr, column_count: u32)
typedef _OpenTableDbDart = void Function(int, Pointer<Utf8>, int);
// fn release_table_db(id: u32) -> i32
typedef _ReleaseTableDbDart = int Function(int);
// fn delete_table_db(port: i64, name: FfiStr)
typedef _DeleteTableDbDart = void Function(int, Pointer<Utf8>);
// fn table_db_get_column_count(id: u32) -> u32
typedef _TableDbGetColumnCountDart = int Function(int);
// fn table_db_get_keys(port: i64, id: u32, col: u32)
typedef _TableDbGetKeysDart = Pointer<Utf8> Function(int, int, int);
// fn table_db_store(port: i64, id: u32, col: u32, key: FfiStr, value: FfiStr)
typedef _TableDbStoreDart = void Function(
    int, int, int, Pointer<Utf8>, Pointer<Utf8>);
// fn table_db_load(port: i64, id: u32, col: u32, key: FfiStr)
typedef _TableDbLoadDart = void Function(int, int, int, Pointer<Utf8>);
// fn table_db_delete(port: i64, id: u32, col: u32, key: FfiStr)
typedef _TableDbDeleteDart = void Function(int, int, int, Pointer<Utf8>);
// fn table_db_transact(id: u32) -> u32
typedef _TableDbTransactDart = int Function(int);
// fn release_table_db_transaction(id: u32) -> i32
typedef _ReleaseTableDbTransactionDart = int Function(int);
// fn table_db_transaction_commit(port: i64, id: u32)
typedef _TableDbTransactionCommitDart = void Function(int, int);
// fn table_db_transaction_rollback(port: i64, id: u32)
typedef _TableDbTransactionRollbackDart = void Function(int, int);
// fn table_db_transaction_store(port: i64,
//  id: u32, col: u32, key: FfiStr, value: FfiStr)
typedef _TableDbTransactionStoreDart = void Function(
    int, int, int, Pointer<Utf8>, Pointer<Utf8>);
// fn table_db_transaction_delete(port: i64, id: u32, col: u32, key: FfiStr)
typedef _TableDbTransactionDeleteDart = void Function(
    int, int, int, Pointer<Utf8>);
// fn valid_crypto_kinds() -> *mut c_char
typedef _ValidCryptoKindsDart = Pointer<Utf8> Function();
// fn best_crypto_kind() -> u32
typedef _BestCryptoKindDart = int Function();
// fn verify_signatures(port: i64,
//  node_ids: FfiStr, data: FfiStr, signatures: FfiStr)
typedef _VerifySignaturesDart = void Function(
    int, Pointer<Utf8>, Pointer<Utf8>, Pointer<Utf8>);
// fn generate_signatures(port: i64, data: FfiStr, key_pairs: FfiStr)
typedef _GenerateSignaturesDart = void Function(
    int, Pointer<Utf8>, Pointer<Utf8>);
// fn generate_key_pair(port: i64, kind: u32) {
typedef _GenerateKeyPairDart = void Function(int, int);
// fn crypto_cached_dh(port: i64, kind: u32, key: FfiStr, secret: FfiStr)
typedef _CryptoCachedDHDart = void Function(
    int, int, Pointer<Utf8>, Pointer<Utf8>);
// fn crypto_compute_dh(port: i64, kind: u32, key: FfiStr, secret: FfiStr)
typedef _CryptoComputeDHDart = void Function(
    int, int, Pointer<Utf8>, Pointer<Utf8>);
// fn crypto_random_bytes(port: i64, kind: u32, len: u32)
typedef _CryptoRandomBytesDart = void Function(int, int, int);
// fn crypto_default_salt_length(port: i64, kind: u32)
typedef _CryptoDefaultSaltLengthDart = void Function(int, int);
// fn crypto_hash_password(port: i64, kind: u32, password: FfiStr, salt: FfiStr)
typedef _CryptoHashPasswordDart = void Function(
    int, int, Pointer<Utf8>, Pointer<Utf8>);
// fn crypto_verify_password(port: i64,
//    kind: u32, password: FfiStr, password_hash: FfiStr )
typedef _CryptoVerifyPasswordDart = void Function(
    int, int, Pointer<Utf8>, Pointer<Utf8>);
// fn crypto_derive_shared_secret(port: i64,
//    kind: u32, password: FfiStr, salt: FfiStr )

// fn crypto_random_nonce(port: i64, kind: u32)
typedef _CryptoRandomNonceDart = void Function(int, int);
// fn crypto_random_shared_secret(port: i64, kind: u32)
typedef _CryptoRandomSharedSecretDart = void Function(int, int);
// fn crypto_generate_key_pair(port: i64, kind: u32)
typedef _CryptoGenerateKeyPairDart = void Function(int, int);
// fn crypto_generate_hash(port: i64, kind: u32, data: FfiStr)
typedef _CryptoGenerateHashDart = void Function(int, int, Pointer<Utf8>);
// fn crypto_validate_key_pair(port: i64,
//    kind: u32, key: FfiStr, secret: FfiStr)
typedef _CryptoValidateKeyPairDart = void Function(
    int, int, Pointer<Utf8>, Pointer<Utf8>);
// fn crypto_validate_hash(port: i64, kind: u32, data: FfiStr, hash: FfiStr)
typedef _CryptoValidateHashDart = void Function(
    int, int, Pointer<Utf8>, Pointer<Utf8>);
// fn crypto_distance(port: i64, kind: u32, key1: FfiStr, key2: FfiStr)
typedef _CryptoDistanceDart = void Function(
    int, int, Pointer<Utf8>, Pointer<Utf8>);
// fn crypto_sign(port: i64,
//    kind: u32, key: FfiStr, secret: FfiStr, data: FfiStr)
typedef _CryptoSignDart = void Function(
    int, int, Pointer<Utf8>, Pointer<Utf8>, Pointer<Utf8>);
// fn crypto_verify(port: i64,
//    kind: u32, key: FfiStr, data: FfiStr, signature: FfiStr)
typedef _CryptoVerifyDart = void Function(
    int, int, Pointer<Utf8>, Pointer<Utf8>, Pointer<Utf8>);
// fn crypto_aead_overhead(port: i64, kind: u32)
typedef _CryptoAeadOverheadDart = void Function(int, int);
// fn crypto_decrypt_aead(port: i64,
//    kind: u32, body: FfiStr, nonce: FfiStr,
//    shared_secret: FfiStr, associated_data: FfiStr)
typedef _CryptoDecryptAeadDart = void Function(
    int, int, Pointer<Utf8>, Pointer<Utf8>, Pointer<Utf8>, Pointer<Utf8>);
// fn crypto_encrypt_aead(port: i64,
//    kind: u32, body: FfiStr, nonce: FfiStr,
//    shared_secret: FfiStr, associated_data: FfiStr)
typedef _CryptoEncryptAeadDart = void Function(
    int, int, Pointer<Utf8>, Pointer<Utf8>, Pointer<Utf8>, Pointer<Utf8>);
// fn crypto_crypt_no_auth(port: i64,
//    kind: u32, body: FfiStr, nonce: FfiStr, shared_secret: FfiStr)
typedef _CryptoCryptNoAuthDart = void Function(
    int, int, Pointer<Utf8>, Pointer<Utf8>, Pointer<Utf8>);

// fn now() -> u64
typedef _NowDart = int Function();
// fn debug(port: i64, log_level: FfiStr)
typedef _DebugDart = void Function(int, Pointer<Utf8>);
// fn shutdown_veilid_core(port: i64)
typedef _ShutdownVeilidCoreDart = void Function(int);
// fn veilid_version_string() -> *mut c_char
typedef _VeilidVersionStringDart = Pointer<Utf8> Function();

// fn veilid_version() -> VeilidVersion
final class VeilidVersionFFI extends Struct {
  @Uint32()
  external int major;
  @Uint32()
  external int minor;
  @Uint32()
  external int patch;
}

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

// Uint8List marshaling
Uint8List convertUint8ListFromJson(dynamic json) =>
    base64UrlNoPadDecode(json as String);
dynamic convertUint8ListToJson(Uint8List data) => base64UrlNoPadEncode(data);

// Parse handle async returns
Future<T> processFuturePlain<T>(Future<dynamic> future) async =>
    future.then((value) {
      final list = value as List<dynamic>;
      switch (list[0] as int) {
        case messageOk:
          {
            if (list[1] == null && null is! T) {
              throw const VeilidAPIExceptionInternal(
                  'Null MESSAGE_OK value on non-nullable type');
            }
            return list[1] as T;
          }
        case messageErr:
          {
            throw VeilidAPIExceptionInternal('Internal API Error: ${list[1]}');
          }
        case messageErrJson:
          {
            throw VeilidAPIException.fromJson(jsonDecode(list[1] as String));
          }
        default:
          {
            throw VeilidAPIExceptionInternal(
                'Unexpected async return message type: ${list[0]}');
          }
      }
      // ignore: inference_failure_on_untyped_parameter
    }).catchError((e) {
      // Wrap all other errors in VeilidAPIExceptionInternal
      throw VeilidAPIExceptionInternal(e.toString());
    }, test: (e) => e is! VeilidAPIException);

Future<T> processFutureJson<T>(
        T Function(dynamic) jsonConstructor, Future<dynamic> future) async =>
    future.then((value) {
      final list = value as List<dynamic>;
      switch (list[0] as int) {
        case messageErr:
          {
            throw VeilidAPIExceptionInternal('Internal API Error: ${list[1]}');
          }
        case messageOkJson:
          {
            if (list[1] is! String) {
              throw const VeilidAPIExceptionInternal(
                  'Non-string MESSAGE_OK_JSON value');
            }
            final ret = jsonDecode(list[1] as String);
            if (ret == null) {
              throw const VeilidAPIExceptionInternal(
                  'Null JSON object on non nullable type');
            }
            return jsonConstructor(ret);
          }
        case messageErrJson:
          {
            throw VeilidAPIException.fromJson(jsonDecode(list[1] as String));
          }
        default:
          {
            throw VeilidAPIExceptionInternal(
                'Unexpected async return message type: ${list[0]}');
          }
      }
      // ignore: inference_failure_on_untyped_parameter
    }).catchError((e) {
      // Wrap all other errors in VeilidAPIExceptionInternal
      throw VeilidAPIExceptionInternal(e.toString());
    }, test: (e) => e is! VeilidAPIException);

Future<T?> processFutureOptJson<T>(
        T Function(dynamic) jsonConstructor, Future<dynamic> future) async =>
    future.then((value) {
      final list = value as List<dynamic>;
      switch (list[0] as int) {
        case messageErr:
          {
            throw VeilidAPIExceptionInternal('Internal API Error: ${list[1]}');
          }
        case messageOkJson:
          {
            if (list[1] == null) {
              return null;
            }
            if (list[1] is! String) {
              throw const VeilidAPIExceptionInternal(
                  'Non-string MESSAGE_OK_JSON optional value');
            }
            final ret = jsonDecode(list[1] as String);
            if (ret == null) {
              return null;
            }
            return jsonConstructor(ret);
          }
        case messageErrJson:
          {
            throw VeilidAPIException.fromJson(jsonDecode(list[1] as String));
          }
        default:
          {
            throw VeilidAPIExceptionInternal(
                'Unexpected async return message type: ${list[0]}');
          }
      }
      // ignore: inference_failure_on_untyped_parameter
    }).catchError((e) {
      // Wrap all other errors in VeilidAPIExceptionInternal
      throw VeilidAPIExceptionInternal(e.toString());
    }, test: (e) => e is! VeilidAPIException);

Future<void> processFutureVoid(Future<dynamic> future) async =>
    future.then((value) {
      final list = value as List<dynamic>;
      switch (list[0] as int) {
        case messageOk:
          {
            if (list[1] != null) {
              throw VeilidAPIExceptionInternal('Unexpected MESSAGE_OK value'
                  ' "${list[1]}" where null expected');
            }
            return;
          }
        case messageErr:
          {
            throw VeilidAPIExceptionInternal('Internal API Error: ${list[1]}');
          }
        case messageOkJson:
          {
            final ret = jsonDecode(list[1] as String);
            if (ret != null) {
              throw VeilidAPIExceptionInternal(
                  'Unexpected MESSAGE_OK_JSON value'
                  ' "$ret" where null expected');
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
                'Unexpected async return message type: ${list[0]}');
          }
      }
      // ignore: inference_failure_on_untyped_parameter
    }).catchError((e) {
      // Wrap all other errors in VeilidAPIExceptionInternal
      throw VeilidAPIExceptionInternal(e.toString());
    }, test: (e) => e is! VeilidAPIException);

Future<Stream<T>> processFutureStream<T>(
        Stream<T> returnStream, Future<dynamic> future) async =>
    future.then((value) {
      final list = value as List<dynamic>;
      switch (list[0] as int) {
        case messageOk:
          {
            if (list[1] != null) {
              throw VeilidAPIExceptionInternal('Unexpected MESSAGE_OK value'
                  ' "${list[1]}" where null expected');
            }
            return returnStream;
          }
        case messageErr:
          {
            throw VeilidAPIExceptionInternal('Internal API Error: ${list[1]}');
          }
        case messageOkJson:
          {
            final ret = jsonDecode(list[1] as String);
            if (ret != null) {
              throw VeilidAPIExceptionInternal(
                  'Unexpected MESSAGE_OK_JSON value'
                  ' "$ret" where null expected');
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
                'Unexpected async return message type: ${list[0]}');
          }
      }
      // ignore: inference_failure_on_untyped_parameter
    }).catchError((e) {
      // Wrap all other errors in VeilidAPIExceptionInternal
      throw VeilidAPIExceptionInternal(e.toString());
    }, test: (e) => e is! VeilidAPIException);

Stream<T> processStreamJson<T>(
    T Function(dynamic) jsonConstructor, ReceivePort port) async* {
  try {
    await for (final value in port) {
      final list = value as List<dynamic>;
      switch (list[0] as int) {
        case messageStreamItemJson:
          {
            if (list[1] == null) {
              throw const VeilidAPIExceptionInternal(
                  'Null MESSAGE_STREAM_ITEM_JSON value');
            }
            final ret = jsonDecode(list[1] as String);
            yield jsonConstructor(ret);
            break;
          }
        case messageStreamAbort:
          {
            throw VeilidAPIExceptionInternal('Internal API Error: ${list[1]}');
          }
        case messageStreamAbortJson:
          {
            throw VeilidAPIException.fromJson(jsonDecode(list[1] as String));
          }
        case messageStreamClose:
          {
            break;
          }
        default:
          {
            throw VeilidAPIExceptionInternal(
                'Unexpected async return message type: ${list[0]}');
          }
      }
    }
  } on VeilidAPIException catch (_) {
    rethrow;
  } on Exception catch (e, s) {
    // Wrap all other errors in VeilidAPIExceptionInternal
    throw VeilidAPIExceptionInternal('$e\nStack Trace:\n$s');
  } finally {
    port.close();
  }
}

class _Ctx {
  _Ctx(int this.id, this.ffi);
  int? id;
  final VeilidFFI ffi;

  void ensureValid() {
    if (id == null) {
      throw VeilidAPIExceptionNotInitialized();
    }
  }

  void close() {
    if (id != null) {
      ffi._releaseRoutingContext(id!);
      id = null;
    }
  }
}

// FFI implementation of VeilidRoutingContext
class VeilidRoutingContextFFI extends VeilidRoutingContext {
  VeilidRoutingContextFFI._(this._ctx) {
    _finalizer.attach(this, _ctx, detach: this);
  }
  final _Ctx _ctx;
  static final Finalizer<_Ctx> _finalizer = Finalizer((ctx) => ctx.close());

  @override
  void close() {
    _ctx.close();
  }

  @override
  VeilidRoutingContextFFI withDefaultSafety() {
    _ctx.ensureValid();
    final newId = _ctx.ffi._routingContextWithDefaultSafety(_ctx.id!);
    return VeilidRoutingContextFFI._(_Ctx(newId, _ctx.ffi));
  }

  @override
  VeilidRoutingContextFFI withSafety(SafetySelection safetySelection) {
    _ctx.ensureValid();
    final newId = _ctx.ffi._routingContextWithSafety(
        _ctx.id!, jsonEncode(safetySelection).toNativeUtf8());
    return VeilidRoutingContextFFI._(_Ctx(newId, _ctx.ffi));
  }

  @override
  VeilidRoutingContextFFI withSequencing(Sequencing sequencing) {
    _ctx.ensureValid();
    final newId = _ctx.ffi._routingContextWithSequencing(
        _ctx.id!, jsonEncode(sequencing).toNativeUtf8());
    return VeilidRoutingContextFFI._(_Ctx(newId, _ctx.ffi));
  }

  @override
  Future<SafetySelection> safety() async {
    _ctx.ensureValid();
    final recvPort = ReceivePort('routing_context_safety');
    final sendPort = recvPort.sendPort;
    _ctx.ffi._routingContextSafety(sendPort.nativePort, _ctx.id!);
    final out = await processFutureJson<SafetySelection>(
        SafetySelection.fromJson, recvPort.first);
    return out;
  }

  @override
  Future<Uint8List> appCall(String target, Uint8List request) async {
    _ctx.ensureValid();
    final nativeEncodedTarget = target.toNativeUtf8();
    final nativeEncodedRequest = base64UrlNoPadEncode(request).toNativeUtf8();

    final recvPort = ReceivePort('routing_context_app_call');
    final sendPort = recvPort.sendPort;
    _ctx.ffi._routingContextAppCall(sendPort.nativePort, _ctx.id!,
        nativeEncodedTarget, nativeEncodedRequest);
    final out = await processFuturePlain<String>(recvPort.first);
    return base64UrlNoPadDecode(out);
  }

  @override
  Future<void> appMessage(String target, Uint8List message) async {
    _ctx.ensureValid();
    final nativeEncodedTarget = target.toNativeUtf8();
    final nativeEncodedMessage = base64UrlNoPadEncode(message).toNativeUtf8();

    final recvPort = ReceivePort('routing_context_app_message');
    final sendPort = recvPort.sendPort;
    _ctx.ffi._routingContextAppMessage(sendPort.nativePort, _ctx.id!,
        nativeEncodedTarget, nativeEncodedMessage);
    return await processFutureVoid(recvPort.first);
  }

  @override
  Future<DHTRecordDescriptor> createDHTRecord(DHTSchema schema,
      {CryptoKind kind = 0}) async {
    _ctx.ensureValid();
    final nativeSchema = jsonEncode(schema).toNativeUtf8();
    final recvPort = ReceivePort('routing_context_create_dht_record');
    final sendPort = recvPort.sendPort;
    _ctx.ffi._routingContextCreateDHTRecord(
        sendPort.nativePort, _ctx.id!, nativeSchema, kind);
    final dhtRecordDescriptor =
        await processFutureJson(DHTRecordDescriptor.fromJson, recvPort.first);
    return dhtRecordDescriptor;
  }

  @override
  Future<DHTRecordDescriptor> openDHTRecord(
      TypedKey key, KeyPair? writer) async {
    _ctx.ensureValid();
    final nativeKey = jsonEncode(key).toNativeUtf8();
    final nativeWriter =
        writer != null ? jsonEncode(writer).toNativeUtf8() : nullptr;
    final recvPort = ReceivePort('routing_context_open_dht_record');
    final sendPort = recvPort.sendPort;
    _ctx.ffi._routingContextOpenDHTRecord(
        sendPort.nativePort, _ctx.id!, nativeKey, nativeWriter);
    final dhtRecordDescriptor =
        await processFutureJson(DHTRecordDescriptor.fromJson, recvPort.first);
    return dhtRecordDescriptor;
  }

  @override
  Future<void> closeDHTRecord(TypedKey key) async {
    _ctx.ensureValid();
    final nativeKey = jsonEncode(key).toNativeUtf8();
    final recvPort = ReceivePort('routing_context_close_dht_record');
    final sendPort = recvPort.sendPort;
    _ctx.ffi._routingContextCloseDHTRecord(
        sendPort.nativePort, _ctx.id!, nativeKey);
    return await processFutureVoid(recvPort.first);
  }

  @override
  Future<void> deleteDHTRecord(TypedKey key) async {
    _ctx.ensureValid();
    final nativeKey = jsonEncode(key).toNativeUtf8();
    final recvPort = ReceivePort('routing_context_delete_dht_record');
    final sendPort = recvPort.sendPort;
    _ctx.ffi._routingContextDeleteDHTRecord(
        sendPort.nativePort, _ctx.id!, nativeKey);
    return await processFutureVoid(recvPort.first);
  }

  @override
  Future<ValueData?> getDHTValue(
      TypedKey key, int subkey, bool forceRefresh) async {
    _ctx.ensureValid();
    final nativeKey = jsonEncode(key).toNativeUtf8();
    final recvPort = ReceivePort('routing_context_get_dht_value');
    final sendPort = recvPort.sendPort;
    _ctx.ffi._routingContextGetDHTValue(
        sendPort.nativePort, _ctx.id!, nativeKey, subkey, forceRefresh);
    final valueData =
        await processFutureOptJson(ValueData.fromJson, recvPort.first);
    return valueData;
  }

  @override
  Future<ValueData?> setDHTValue(
      TypedKey key, int subkey, Uint8List data) async {
    _ctx.ensureValid();
    final nativeKey = jsonEncode(key).toNativeUtf8();
    final nativeData = base64UrlNoPadEncode(data).toNativeUtf8();

    final recvPort = ReceivePort('routing_context_set_dht_value');
    final sendPort = recvPort.sendPort;
    _ctx.ffi._routingContextSetDHTValue(
        sendPort.nativePort, _ctx.id!, nativeKey, subkey, nativeData);
    final valueData =
        await processFutureOptJson(ValueData.fromJson, recvPort.first);
    return valueData;
  }

  @override
  Future<Timestamp> watchDHTValues(TypedKey key,
      {List<ValueSubkeyRange>? subkeys,
      Timestamp? expiration,
      int? count}) async {
    subkeys ??= [];
    expiration ??= Timestamp(value: BigInt.zero);
    count ??= 0xFFFFFFFF;

    _ctx.ensureValid();
    final nativeKey = jsonEncode(key).toNativeUtf8();
    final nativeSubkeys = jsonEncode(subkeys).toNativeUtf8();
    final nativeExpiration = expiration.value.toInt();

    final recvPort = ReceivePort('routing_context_watch_dht_values');
    final sendPort = recvPort.sendPort;
    _ctx.ffi._routingContextWatchDHTValues(sendPort.nativePort, _ctx.id!,
        nativeKey, nativeSubkeys, nativeExpiration, count);
    final actualExpiration = Timestamp(
        value: BigInt.from(await processFuturePlain<int>(recvPort.first)));
    return actualExpiration;
  }

  @override
  Future<bool> cancelDHTWatch(TypedKey key,
      {List<ValueSubkeyRange>? subkeys}) async {
    subkeys ??= [];

    _ctx.ensureValid();
    final nativeKey = jsonEncode(key).toNativeUtf8();
    final nativeSubkeys = jsonEncode(subkeys).toNativeUtf8();

    final recvPort = ReceivePort('routing_context_cancel_dht_watch');
    final sendPort = recvPort.sendPort;
    _ctx.ffi._routingContextCancelDHTWatch(
        sendPort.nativePort, _ctx.id!, nativeKey, nativeSubkeys);
    final cancelled = await processFuturePlain<bool>(recvPort.first);
    return cancelled;
  }
}

class _TDBT {
  _TDBT(int this.id, this.tdbffi, this.ffi);
  int? id;
  final VeilidTableDBFFI tdbffi;
  final VeilidFFI ffi;
  void ensureValid() {
    if (id == null) {
      throw VeilidAPIExceptionNotInitialized();
    }
  }

  void close() {
    if (id != null) {
      ffi._releaseTableDbTransaction(id!);
      id = null;
    }
  }
}

// FFI implementation of VeilidTableDBTransaction
class VeilidTableDBTransactionFFI extends VeilidTableDBTransaction {
  VeilidTableDBTransactionFFI._(this._tdbt) {
    _finalizer.attach(this, _tdbt, detach: this);
  }
  final _TDBT _tdbt;
  static final Finalizer<_TDBT> _finalizer = Finalizer((tdbt) => tdbt.close());

  @override
  bool isDone() => _tdbt.id == null;

  @override
  Future<void> commit() async {
    _tdbt.ensureValid();
    final recvPort = ReceivePort('veilid_table_db_transaction_commit');
    final sendPort = recvPort.sendPort;
    _tdbt.ffi._tableDbTransactionCommit(
      sendPort.nativePort,
      _tdbt.id!,
    );
    await processFutureVoid(recvPort.first);
    _tdbt.close();
  }

  @override
  Future<void> rollback() async {
    _tdbt.ensureValid();
    final recvPort = ReceivePort('veilid_table_db_transaction_rollback');
    final sendPort = recvPort.sendPort;
    _tdbt.ffi._tableDbTransactionRollback(
      sendPort.nativePort,
      _tdbt.id!,
    );
    await processFutureVoid(recvPort.first);
    _tdbt.close();
  }

  @override
  Future<void> store(int col, Uint8List key, Uint8List value) async {
    _tdbt.ensureValid();
    final nativeEncodedKey = base64UrlNoPadEncode(key).toNativeUtf8();
    final nativeEncodedValue = base64UrlNoPadEncode(value).toNativeUtf8();

    final recvPort = ReceivePort('veilid_table_db_transaction_store');
    final sendPort = recvPort.sendPort;
    _tdbt.ffi._tableDbTransactionStore(
      sendPort.nativePort,
      _tdbt.id!,
      col,
      nativeEncodedKey,
      nativeEncodedValue,
    );
    return await processFutureVoid(recvPort.first);
  }

  @override
  Future<void> delete(int col, Uint8List key) async {
    _tdbt.ensureValid();
    final nativeEncodedKey = base64UrlNoPadEncode(key).toNativeUtf8();

    final recvPort = ReceivePort('veilid_table_db_transaction_delete');
    final sendPort = recvPort.sendPort;
    _tdbt.ffi._tableDbTransactionDelete(
      sendPort.nativePort,
      _tdbt.id!,
      col,
      nativeEncodedKey,
    );
    return await processFuturePlain(recvPort.first);
  }
}

class _TDB {
  _TDB(int this.id, this.ffi);
  int? id;
  final VeilidFFI ffi;
  void ensureValid() {
    if (id == null) {
      throw VeilidAPIExceptionNotInitialized();
    }
  }

  void close() {
    if (id != null) {
      ffi._releaseTableDb(id!);
      id = null;
    }
  }
}

// FFI implementation of VeilidTableDB
class VeilidTableDBFFI extends VeilidTableDB {
  VeilidTableDBFFI._(this._tdb) {
    _finalizer.attach(this, _tdb, detach: this);
  }
  final _TDB _tdb;
  static final Finalizer<_TDB> _finalizer = Finalizer((tdb) => tdb.close());

  @override
  void close() {
    _tdb.close();
  }

  @override
  int getColumnCount() {
    _tdb.ensureValid();
    return _tdb.ffi._tableDbGetColumnCount(_tdb.id!);
  }

  @override
  Future<List<Uint8List>> getKeys(int col) async {
    _tdb.ensureValid();

    final recvPort = ReceivePort('veilid_table_db_get_keys');
    final sendPort = recvPort.sendPort;

    _tdb.ffi._tableDbGetKeys(sendPort.nativePort, _tdb.id!, col);

    return await processFutureJson(
        jsonListConstructor<Uint8List>(base64UrlNoPadDecodeDynamic),
        recvPort.first);
  }

  @override
  VeilidTableDBTransaction transact() {
    _tdb.ensureValid();

    final id = _tdb.ffi._tableDbTransact(_tdb.id!);
    return VeilidTableDBTransactionFFI._(_TDBT(id, this, _tdb.ffi));
  }

  @override
  Future<void> store(int col, Uint8List key, Uint8List value) async {
    _tdb.ensureValid();

    final nativeEncodedKey = base64UrlNoPadEncode(key).toNativeUtf8();
    final nativeEncodedValue = base64UrlNoPadEncode(value).toNativeUtf8();

    final recvPort = ReceivePort('veilid_table_db_store');
    final sendPort = recvPort.sendPort;
    _tdb.ffi._tableDbStore(
      sendPort.nativePort,
      _tdb.id!,
      col,
      nativeEncodedKey,
      nativeEncodedValue,
    );
    return await processFutureVoid(recvPort.first);
  }

  @override
  Future<Uint8List?> load(int col, Uint8List key) async {
    _tdb.ensureValid();
    final nativeEncodedKey = base64UrlNoPadEncode(key).toNativeUtf8();

    final recvPort = ReceivePort('veilid_table_db_load');
    final sendPort = recvPort.sendPort;
    _tdb.ffi._tableDbLoad(
      sendPort.nativePort,
      _tdb.id!,
      col,
      nativeEncodedKey,
    );
    final out = await processFuturePlain<String?>(recvPort.first);
    if (out == null) {
      return null;
    }
    return base64UrlNoPadDecode(out);
  }

  @override
  Future<Uint8List?> delete(int col, Uint8List key) async {
    _tdb.ensureValid();
    final nativeEncodedKey = base64UrlNoPadEncode(key).toNativeUtf8();

    final recvPort = ReceivePort('veilid_table_db_delete');
    final sendPort = recvPort.sendPort;
    _tdb.ffi._tableDbDelete(
      sendPort.nativePort,
      _tdb.id!,
      col,
      nativeEncodedKey,
    );
    final out = await processFuturePlain<String?>(recvPort.first);
    if (out == null) {
      return null;
    }
    return base64UrlNoPadDecode(out);
  }
}

// FFI implementation of VeilidCryptoSystem
class VeilidCryptoSystemFFI extends VeilidCryptoSystem {
  VeilidCryptoSystemFFI._(this._ffi, this._kind);
  final CryptoKind _kind;
  final VeilidFFI _ffi;

  @override
  CryptoKind kind() => _kind;

  @override
  Future<SharedSecret> cachedDH(PublicKey key, SecretKey secret) async {
    final nativeKey = jsonEncode(key).toNativeUtf8();
    final nativeSecret = jsonEncode(secret).toNativeUtf8();

    final recvPort = ReceivePort('crypto_cached_dh');
    final sendPort = recvPort.sendPort;
    _ffi._cryptoCachedDH(sendPort.nativePort, _kind, nativeKey, nativeSecret);
    return await processFutureJson(SharedSecret.fromJson, recvPort.first);
  }

  @override
  Future<SharedSecret> computeDH(PublicKey key, SecretKey secret) async {
    final nativeKey = jsonEncode(key).toNativeUtf8();
    final nativeSecret = jsonEncode(secret).toNativeUtf8();

    final recvPort = ReceivePort('crypto_compute_dh');
    final sendPort = recvPort.sendPort;
    _ffi._cryptoComputeDH(sendPort.nativePort, _kind, nativeKey, nativeSecret);
    return await processFutureJson(SharedSecret.fromJson, recvPort.first);
  }

  @override
  Future<Uint8List> randomBytes(int len) async {
    final recvPort = ReceivePort('crypto_random_bytes');
    final sendPort = recvPort.sendPort;
    _ffi._cryptoRandomBytes(sendPort.nativePort, _kind, len);
    final out = await processFuturePlain<String>(recvPort.first);
    return base64UrlNoPadDecode(out);
  }

  @override
  Future<int> defaultSaltLength() async {
    final recvPort = ReceivePort('crypto_default_salt_length');
    final sendPort = recvPort.sendPort;
    _ffi._cryptoDefaultSaltLength(sendPort.nativePort, _kind);
    return await processFuturePlain(recvPort.first);
  }

  @override
  Future<String> hashPassword(Uint8List password, Uint8List salt) async {
    final nativeEncodedPassword = base64UrlNoPadEncode(password).toNativeUtf8();
    final nativeEncodedSalt = base64UrlNoPadEncode(salt).toNativeUtf8();

    final recvPort = ReceivePort('crypto_hash_password');
    final sendPort = recvPort.sendPort;
    _ffi._cryptoHashPassword(
        sendPort.nativePort, _kind, nativeEncodedPassword, nativeEncodedSalt);
    return await processFuturePlain(recvPort.first);
  }

  @override
  Future<bool> verifyPassword(Uint8List password, String passwordHash) async {
    final nativeEncodedPassword = base64UrlNoPadEncode(password).toNativeUtf8();
    final nativeEncodedPasswordHash = passwordHash.toNativeUtf8();

    final recvPort = ReceivePort('crypto_verify_password');
    final sendPort = recvPort.sendPort;
    _ffi._cryptoVerifyPassword(sendPort.nativePort, _kind,
        nativeEncodedPassword, nativeEncodedPasswordHash);
    return await processFuturePlain(recvPort.first);
  }

  @override
  Future<SharedSecret> deriveSharedSecret(
      Uint8List password, Uint8List salt) async {
    final nativeEncodedPassword = base64UrlNoPadEncode(password).toNativeUtf8();
    final nativeEncodedSalt = base64UrlNoPadEncode(salt).toNativeUtf8();

    final recvPort = ReceivePort('crypto_derive_shared_secret');
    final sendPort = recvPort.sendPort;
    _ffi._cryptoDeriveSharedSecret(
        sendPort.nativePort, _kind, nativeEncodedPassword, nativeEncodedSalt);
    return await processFutureJson(SharedSecret.fromJson, recvPort.first);
  }

  @override
  Future<Nonce> randomNonce() async {
    final recvPort = ReceivePort('crypto_random_nonce');
    final sendPort = recvPort.sendPort;
    _ffi._cryptoRandomNonce(sendPort.nativePort, _kind);
    return await processFutureJson(Nonce.fromJson, recvPort.first);
  }

  @override
  Future<SharedSecret> randomSharedSecret() async {
    final recvPort = ReceivePort('crypto_random_shared_secret');
    final sendPort = recvPort.sendPort;
    _ffi._cryptoRandomSharedSecret(sendPort.nativePort, _kind);
    return await processFutureJson(SharedSecret.fromJson, recvPort.first);
  }

  @override
  Future<KeyPair> generateKeyPair() async {
    final recvPort = ReceivePort('crypto_generate_key_pair');
    final sendPort = recvPort.sendPort;
    _ffi._cryptoGenerateKeyPair(sendPort.nativePort, _kind);
    return await processFutureJson(KeyPair.fromJson, recvPort.first);
  }

  @override
  Future<HashDigest> generateHash(Uint8List data) async {
    final nativeEncodedData = base64UrlNoPadEncode(data).toNativeUtf8();

    final recvPort = ReceivePort('crypto_generate_hash');
    final sendPort = recvPort.sendPort;
    _ffi._cryptoGenerateHash(sendPort.nativePort, _kind, nativeEncodedData);
    return await processFutureJson(HashDigest.fromJson, recvPort.first);
  }

  @override
  Future<bool> validateKeyPair(PublicKey key, SecretKey secret) async {
    final nativeKey = jsonEncode(key).toNativeUtf8();
    final nativeSecret = jsonEncode(secret).toNativeUtf8();

    final recvPort = ReceivePort('crypto_validate_key_pair');
    final sendPort = recvPort.sendPort;
    _ffi._cryptoValidateKeyPair(
        sendPort.nativePort, _kind, nativeKey, nativeSecret);
    return await processFuturePlain(recvPort.first);
  }

  @override
  Future<bool> validateHash(Uint8List data, HashDigest hash) async {
    final nativeEncodedData = base64UrlNoPadEncode(data).toNativeUtf8();
    final nativeHash = jsonEncode(hash).toNativeUtf8();

    final recvPort = ReceivePort('crypto_validate_hash');
    final sendPort = recvPort.sendPort;
    _ffi._cryptoValidateHash(
        sendPort.nativePort, _kind, nativeEncodedData, nativeHash);
    return await processFuturePlain(recvPort.first);
  }

  @override
  Future<CryptoKeyDistance> distance(CryptoKey key1, CryptoKey key2) async {
    final nativeKey1 = jsonEncode(key1).toNativeUtf8();
    final nativeKey2 = jsonEncode(key2).toNativeUtf8();

    final recvPort = ReceivePort('crypto_distance');
    final sendPort = recvPort.sendPort;
    _ffi._cryptoDistance(sendPort.nativePort, _kind, nativeKey1, nativeKey2);
    return await processFutureJson(CryptoKeyDistance.fromJson, recvPort.first);
  }

  @override
  Future<Signature> sign(
      PublicKey key, SecretKey secret, Uint8List data) async {
    final nativeKey = jsonEncode(key).toNativeUtf8();
    final nativeSecret = jsonEncode(secret).toNativeUtf8();
    final nativeEncodedData = base64UrlNoPadEncode(data).toNativeUtf8();

    final recvPort = ReceivePort('crypto_sign');
    final sendPort = recvPort.sendPort;
    _ffi._cryptoSign(
        sendPort.nativePort, _kind, nativeKey, nativeSecret, nativeEncodedData);
    return await processFutureJson(Signature.fromJson, recvPort.first);
  }

  @override
  Future<void> verify(
      PublicKey key, Uint8List data, Signature signature) async {
    final nativeKey = jsonEncode(key).toNativeUtf8();
    final nativeEncodedData = base64UrlNoPadEncode(data).toNativeUtf8();
    final nativeSignature = jsonEncode(signature).toNativeUtf8();

    final recvPort = ReceivePort('crypto_verify');
    final sendPort = recvPort.sendPort;
    _ffi._cryptoVerify(sendPort.nativePort, _kind, nativeKey, nativeEncodedData,
        nativeSignature);
    return await processFutureVoid(recvPort.first);
  }

  @override
  Future<int> aeadOverhead() async {
    final recvPort = ReceivePort('crypto_aead_overhead');
    final sendPort = recvPort.sendPort;
    _ffi._cryptoAeadOverhead(
      sendPort.nativePort,
      _kind,
    );
    return await processFuturePlain(recvPort.first);
  }

  @override
  Future<Uint8List> decryptAead(Uint8List body, Nonce nonce,
      SharedSecret sharedSecret, Uint8List? associatedData) async {
    final nativeEncodedBody = base64UrlNoPadEncode(body).toNativeUtf8();
    final nativeNonce = jsonEncode(nonce).toNativeUtf8();
    final nativeSharedSecret = jsonEncode(sharedSecret).toNativeUtf8();
    final nativeSignature = (associatedData != null)
        ? jsonEncode(associatedData).toNativeUtf8()
        : nullptr;

    final recvPort = ReceivePort('crypto_decrypt_aead');
    final sendPort = recvPort.sendPort;
    _ffi._cryptoDecryptAead(sendPort.nativePort, _kind, nativeEncodedBody,
        nativeNonce, nativeSharedSecret, nativeSignature);
    final out = await processFuturePlain<String>(recvPort.first);
    return base64UrlNoPadDecode(out);
  }

  @override
  Future<Uint8List> encryptAead(Uint8List body, Nonce nonce,
      SharedSecret sharedSecret, Uint8List? associatedData) async {
    final nativeEncodedBody = base64UrlNoPadEncode(body).toNativeUtf8();
    final nativeNonce = jsonEncode(nonce).toNativeUtf8();
    final nativeSharedSecret = jsonEncode(sharedSecret).toNativeUtf8();
    final nativeSignature = (associatedData != null)
        ? jsonEncode(associatedData).toNativeUtf8()
        : nullptr;

    final recvPort = ReceivePort('crypto_encrypt_aead');
    final sendPort = recvPort.sendPort;
    _ffi._cryptoEncryptAead(sendPort.nativePort, _kind, nativeEncodedBody,
        nativeNonce, nativeSharedSecret, nativeSignature);
    final out = await processFuturePlain<String>(recvPort.first);
    return base64UrlNoPadDecode(out);
  }

  @override
  Future<Uint8List> cryptNoAuth(
      Uint8List body, Nonce nonce, SharedSecret sharedSecret) async {
    final nativeEncodedBody = base64UrlNoPadEncode(body).toNativeUtf8();
    final nativeNonce = jsonEncode(nonce).toNativeUtf8();
    final nativeSharedSecret = jsonEncode(sharedSecret).toNativeUtf8();

    final recvPort = ReceivePort('crypto_crypt_no_auth');
    final sendPort = recvPort.sendPort;
    _ffi._cryptoCryptNoAuth(sendPort.nativePort, _kind, nativeEncodedBody,
        nativeNonce, nativeSharedSecret);
    final out = await processFuturePlain<String>(recvPort.first);
    return base64UrlNoPadDecode(out);
  }
}

// FFI implementation of high level Veilid API
class VeilidFFI extends Veilid {
  VeilidFFI(DynamicLibrary dylib)
      : _dylib = dylib,
        _freeString =
            dylib.lookupFunction<Void Function(Pointer<Utf8>), _FreeStringDart>(
                'free_string'),
        _initializeVeilidCore = dylib.lookupFunction<
            Void Function(Pointer<Utf8>),
            _InitializeVeilidCoreDart>('initialize_veilid_core'),
        _changeLogLevel = dylib.lookupFunction<
            Void Function(Pointer<Utf8>, Pointer<Utf8>),
            _ChangeLogLevelDart>('change_log_level'),
        _startupVeilidCore = dylib.lookupFunction<
            Void Function(Int64, Int64, Pointer<Utf8>),
            _StartupVeilidCoreDart>('startup_veilid_core'),
        _getVeilidState =
            dylib.lookupFunction<Void Function(Int64), _GetVeilidStateDart>(
                'get_veilid_state'),
        _attach =
            dylib.lookupFunction<Void Function(Int64), _AttachDart>('attach'),
        _detach =
            dylib.lookupFunction<Void Function(Int64), _DetachDart>('detach'),
        _shutdownVeilidCore =
            dylib.lookupFunction<Void Function(Int64), _ShutdownVeilidCoreDart>(
                'shutdown_veilid_core'),
        _routingContext =
            dylib.lookupFunction<Void Function(Int64), _RoutingContextDart>(
                'routing_context'),
        _releaseRoutingContext = dylib.lookupFunction<Int32 Function(Uint32),
            _ReleaseRoutingContextDart>('release_routing_context'),
        _routingContextWithDefaultSafety = dylib.lookupFunction<
                Uint32 Function(Uint32), _RoutingContextWithDefaultSafetyDart>(
            'routing_context_with_default_safety'),
        _routingContextWithSafety = dylib.lookupFunction<
            Uint32 Function(Uint32, Pointer<Utf8>),
            _RoutingContextWithSafetyDart>('routing_context_with_safety'),
        _routingContextWithSequencing = dylib.lookupFunction<
                Uint32 Function(Uint32, Pointer<Utf8>),
                _RoutingContextWithSequencingDart>(
            'routing_context_with_sequencing'),
        _routingContextSafety = dylib.lookupFunction<
            Void Function(Int64, Uint32),
            _RoutingContextSafetyDart>('routing_context_safety'),
        _routingContextAppCall = dylib.lookupFunction<
            Void Function(Int64, Uint32, Pointer<Utf8>, Pointer<Utf8>),
            _RoutingContextAppCallDart>('routing_context_app_call'),
        _routingContextAppMessage = dylib.lookupFunction<
            Void Function(Int64, Uint32, Pointer<Utf8>, Pointer<Utf8>),
            _RoutingContextAppMessageDart>('routing_context_app_message'),
        _routingContextCreateDHTRecord = dylib.lookupFunction<
                Void Function(Int64, Uint32, Pointer<Utf8>, Uint32),
                _RoutingContextCreateDHTRecordDart>(
            'routing_context_create_dht_record'),
        _routingContextOpenDHTRecord = dylib.lookupFunction<
                Void Function(Int64, Uint32, Pointer<Utf8>, Pointer<Utf8>),
                _RoutingContextOpenDHTRecordDart>(
            'routing_context_open_dht_record'),
        _routingContextCloseDHTRecord = dylib.lookupFunction<
                Void Function(Int64, Uint32, Pointer<Utf8>),
                _RoutingContextCloseDHTRecordDart>(
            'routing_context_close_dht_record'),
        _routingContextDeleteDHTRecord = dylib.lookupFunction<
                Void Function(Int64, Uint32, Pointer<Utf8>),
                _RoutingContextDeleteDHTRecordDart>(
            'routing_context_delete_dht_record'),
        _routingContextGetDHTValue = dylib.lookupFunction<
            Void Function(Int64, Uint32, Pointer<Utf8>, Uint32, Bool),
            _RoutingContextGetDHTValueDart>('routing_context_get_dht_value'),
        _routingContextSetDHTValue = dylib.lookupFunction<
            Void Function(Int64, Uint32, Pointer<Utf8>, Uint32, Pointer<Utf8>),
            _RoutingContextSetDHTValueDart>('routing_context_set_dht_value'),
        _routingContextWatchDHTValues = dylib.lookupFunction<
                Void Function(Int64, Uint32, Pointer<Utf8>, Pointer<Utf8>,
                    Uint64, Uint32),
                _RoutingContextWatchDHTValuesDart>(
            'routing_context_watch_dht_values'),
        _routingContextCancelDHTWatch = dylib.lookupFunction<
                Void Function(Int64, Uint32, Pointer<Utf8>, Pointer<Utf8>),
                _RoutingContextCancelDHTWatchDart>(
            'routing_context_cancel_dht_watch'),
        _newPrivateRoute =
            dylib.lookupFunction<Void Function(Int64), _NewPrivateRouteDart>(
                'new_private_route'),
        _newCustomPrivateRoute = dylib.lookupFunction<
            Void Function(Int64, Pointer<Utf8>, Pointer<Utf8>),
            _NewCustomPrivateRouteDart>('new_custom_private_route'),
        _importRemotePrivateRoute = dylib.lookupFunction<
            Void Function(Int64, Pointer<Utf8>),
            _ImportRemotePrivateRouteDart>('import_remote_private_route'),
        _releasePrivateRoute = dylib.lookupFunction<
            Void Function(Int64, Pointer<Utf8>),
            _ReleasePrivateRouteDart>('release_private_route'),
        _appCallReply = dylib.lookupFunction<
            Void Function(Int64, Pointer<Utf8>, Pointer<Utf8>),
            _AppCallReplyDart>('app_call_reply'),
        _openTableDb = dylib.lookupFunction<
            Void Function(Int64, Pointer<Utf8>, Uint32),
            _OpenTableDbDart>('open_table_db'),
        _releaseTableDb =
            dylib.lookupFunction<Int32 Function(Uint32), _ReleaseTableDbDart>(
                'release_table_db'),
        _deleteTableDb = dylib.lookupFunction<
            Void Function(Int64, Pointer<Utf8>),
            _DeleteTableDbDart>('delete_table_db'),
        _tableDbGetColumnCount = dylib.lookupFunction<Uint32 Function(Uint32),
            _TableDbGetColumnCountDart>('table_db_get_column_count'),
        _tableDbGetKeys = dylib.lookupFunction<
            Pointer<Utf8> Function(Uint64, Uint32, Uint32),
            _TableDbGetKeysDart>('table_db_get_keys'),
        _tableDbStore = dylib.lookupFunction<
            Void Function(Int64, Uint32, Uint32, Pointer<Utf8>, Pointer<Utf8>),
            _TableDbStoreDart>('table_db_store'),
        _tableDbLoad = dylib.lookupFunction<
            Void Function(Int64, Uint32, Uint32, Pointer<Utf8>),
            _TableDbLoadDart>('table_db_load'),
        _tableDbDelete = dylib.lookupFunction<
            Void Function(Int64, Uint32, Uint32, Pointer<Utf8>),
            _TableDbDeleteDart>('table_db_delete'),
        _tableDbTransact =
            dylib.lookupFunction<Uint32 Function(Uint32), _TableDbTransactDart>(
                'table_db_transact'),
        _releaseTableDbTransaction = dylib.lookupFunction<
            Int32 Function(Uint32),
            _ReleaseTableDbTransactionDart>('release_table_db_transaction'),
        _tableDbTransactionCommit = dylib.lookupFunction<
            Void Function(Uint64, Uint32),
            _TableDbTransactionCommitDart>('table_db_transaction_commit'),
        _tableDbTransactionRollback = dylib.lookupFunction<
            Void Function(Uint64, Uint32),
            _TableDbTransactionRollbackDart>('table_db_transaction_rollback'),
        _tableDbTransactionStore = dylib.lookupFunction<
            Void Function(Int64, Uint32, Uint32, Pointer<Utf8>, Pointer<Utf8>),
            _TableDbTransactionStoreDart>('table_db_transaction_store'),
        _tableDbTransactionDelete = dylib.lookupFunction<
            Void Function(Int64, Uint32, Uint32, Pointer<Utf8>),
            _TableDbTransactionDeleteDart>('table_db_transaction_delete'),
        _validCryptoKinds = dylib.lookupFunction<Pointer<Utf8> Function(),
            _ValidCryptoKindsDart>('valid_crypto_kinds'),
        _bestCryptoKind =
            dylib.lookupFunction<Uint32 Function(), _BestCryptoKindDart>(
                'best_crypto_kind'),
        _verifySignatures = dylib.lookupFunction<
            Void Function(Int64, Pointer<Utf8>, Pointer<Utf8>, Pointer<Utf8>),
            _VerifySignaturesDart>('verify_signatures'),
        _generateSignatures = dylib.lookupFunction<
            Void Function(Int64, Pointer<Utf8>, Pointer<Utf8>),
            _GenerateSignaturesDart>('generate_signatures'),
        _generateKeyPair = dylib.lookupFunction<Void Function(Int64, Uint32),
            _GenerateKeyPairDart>('generate_key_pair'),
        _cryptoCachedDH = dylib.lookupFunction<
            Void Function(Int64, Uint32, Pointer<Utf8>, Pointer<Utf8>),
            _CryptoCachedDHDart>('crypto_cached_dh'),
        _cryptoComputeDH = dylib.lookupFunction<
            Void Function(Int64, Uint32, Pointer<Utf8>, Pointer<Utf8>),
            _CryptoComputeDHDart>('crypto_compute_dh'),
        _cryptoRandomBytes = dylib.lookupFunction<
            Void Function(Int64, Uint32, Uint32),
            _CryptoRandomBytesDart>('crypto_random_bytes'),
        _cryptoDefaultSaltLength = dylib.lookupFunction<
            Void Function(Int64, Uint32),
            _CryptoDefaultSaltLengthDart>('crypto_default_salt_length'),
        _cryptoHashPassword = dylib.lookupFunction<
            Void Function(Int64, Uint32, Pointer<Utf8>, Pointer<Utf8>),
            _CryptoHashPasswordDart>('crypto_hash_password'),
        _cryptoVerifyPassword = dylib.lookupFunction<
            Void Function(Int64, Uint32, Pointer<Utf8>, Pointer<Utf8>),
            _CryptoVerifyPasswordDart>('crypto_verify_password'),
        _cryptoDeriveSharedSecret = dylib.lookupFunction<
            Void Function(Int64, Uint32, Pointer<Utf8>, Pointer<Utf8>),
            _CryptoVerifyPasswordDart>('crypto_derive_shared_secret'),
        _cryptoRandomNonce = dylib.lookupFunction<Void Function(Int64, Uint32),
            _CryptoRandomNonceDart>('crypto_random_nonce'),
        _cryptoRandomSharedSecret = dylib.lookupFunction<
            Void Function(Int64, Uint32),
            _CryptoRandomSharedSecretDart>('crypto_random_shared_secret'),
        _cryptoGenerateKeyPair = dylib.lookupFunction<
            Void Function(Int64, Uint32),
            _CryptoGenerateKeyPairDart>('crypto_generate_key_pair'),
        _cryptoGenerateHash = dylib.lookupFunction<
            Void Function(Int64, Uint32, Pointer<Utf8>),
            _CryptoGenerateHashDart>('crypto_generate_hash'),
        _cryptoValidateKeyPair = dylib.lookupFunction<
            Void Function(Int64, Uint32, Pointer<Utf8>, Pointer<Utf8>),
            _CryptoValidateKeyPairDart>('crypto_validate_key_pair'),
        _cryptoValidateHash = dylib.lookupFunction<
            Void Function(Int64, Uint32, Pointer<Utf8>, Pointer<Utf8>),
            _CryptoValidateHashDart>('crypto_validate_hash'),
        _cryptoDistance = dylib.lookupFunction<
            Void Function(Int64, Uint32, Pointer<Utf8>, Pointer<Utf8>),
            _CryptoDistanceDart>('crypto_distance'),
        _cryptoSign = dylib.lookupFunction<
            Void Function(
                Int64, Uint32, Pointer<Utf8>, Pointer<Utf8>, Pointer<Utf8>),
            _CryptoSignDart>('crypto_sign'),
        _cryptoVerify = dylib.lookupFunction<
            Void Function(
                Int64, Uint32, Pointer<Utf8>, Pointer<Utf8>, Pointer<Utf8>),
            _CryptoVerifyDart>('crypto_verify'),
        _cryptoAeadOverhead = dylib.lookupFunction<Void Function(Int64, Uint32),
            _CryptoAeadOverheadDart>('crypto_aead_overhead'),
        _cryptoDecryptAead = dylib.lookupFunction<
            Void Function(Int64, Uint32, Pointer<Utf8>, Pointer<Utf8>,
                Pointer<Utf8>, Pointer<Utf8>),
            _CryptoDecryptAeadDart>('crypto_decrypt_aead'),
        _cryptoEncryptAead = dylib.lookupFunction<
            Void Function(Int64, Uint32, Pointer<Utf8>, Pointer<Utf8>,
                Pointer<Utf8>, Pointer<Utf8>),
            _CryptoEncryptAeadDart>('crypto_encrypt_aead'),
        _cryptoCryptNoAuth = dylib.lookupFunction<
            Void Function(
                Int64, Uint32, Pointer<Utf8>, Pointer<Utf8>, Pointer<Utf8>),
            _CryptoCryptNoAuthDart>('crypto_crypt_no_auth'),
        _now = dylib.lookupFunction<Uint64 Function(), _NowDart>('now'),
        _debug = dylib.lookupFunction<Void Function(Int64, Pointer<Utf8>),
            _DebugDart>('debug'),
        _veilidVersionString = dylib.lookupFunction<Pointer<Utf8> Function(),
            _VeilidVersionStringDart>('veilid_version_string'),
        _veilidVersion = dylib.lookupFunction<VeilidVersionFFI Function(),
            _VeilidVersionDart>('veilid_version') {
    // Get veilid_flutter initializer
    final initializeVeilidFlutter = _dylib.lookupFunction<
        Void Function(Pointer<_DartPostCObject>),
        void Function(Pointer<_DartPostCObject>)>('initialize_veilid_flutter');
    initializeVeilidFlutter(NativeApi.postCObject);
  }
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
  final _RoutingContextWithDefaultSafetyDart _routingContextWithDefaultSafety;
  final _RoutingContextWithSafetyDart _routingContextWithSafety;
  final _RoutingContextWithSequencingDart _routingContextWithSequencing;
  final _RoutingContextSafetyDart _routingContextSafety;
  final _RoutingContextAppCallDart _routingContextAppCall;
  final _RoutingContextAppMessageDart _routingContextAppMessage;
  final _RoutingContextCreateDHTRecordDart _routingContextCreateDHTRecord;
  final _RoutingContextOpenDHTRecordDart _routingContextOpenDHTRecord;
  final _RoutingContextCloseDHTRecordDart _routingContextCloseDHTRecord;
  final _RoutingContextDeleteDHTRecordDart _routingContextDeleteDHTRecord;
  final _RoutingContextGetDHTValueDart _routingContextGetDHTValue;
  final _RoutingContextSetDHTValueDart _routingContextSetDHTValue;
  final _RoutingContextWatchDHTValuesDart _routingContextWatchDHTValues;
  final _RoutingContextCancelDHTWatchDart _routingContextCancelDHTWatch;

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

  final _ValidCryptoKindsDart _validCryptoKinds;
  final _BestCryptoKindDart _bestCryptoKind;
  final _VerifySignaturesDart _verifySignatures;
  final _GenerateSignaturesDart _generateSignatures;
  final _GenerateKeyPairDart _generateKeyPair;

  final _CryptoCachedDHDart _cryptoCachedDH;
  final _CryptoComputeDHDart _cryptoComputeDH;

  final _CryptoRandomBytesDart _cryptoRandomBytes;
  final _CryptoDefaultSaltLengthDart _cryptoDefaultSaltLength;
  final _CryptoHashPasswordDart _cryptoHashPassword;
  final _CryptoVerifyPasswordDart _cryptoVerifyPassword;
  final void Function(int, int, Pointer<Utf8>, Pointer<Utf8>)
      _cryptoDeriveSharedSecret;

  final _CryptoRandomNonceDart _cryptoRandomNonce;
  final _CryptoRandomSharedSecretDart _cryptoRandomSharedSecret;
  final _CryptoGenerateKeyPairDart _cryptoGenerateKeyPair;
  final _CryptoGenerateHashDart _cryptoGenerateHash;
  final _CryptoValidateKeyPairDart _cryptoValidateKeyPair;
  final _CryptoValidateHashDart _cryptoValidateHash;
  final _CryptoDistanceDart _cryptoDistance;
  final _CryptoSignDart _cryptoSign;
  final _CryptoVerifyDart _cryptoVerify;
  final _CryptoAeadOverheadDart _cryptoAeadOverhead;
  final _CryptoDecryptAeadDart _cryptoDecryptAead;
  final _CryptoEncryptAeadDart _cryptoEncryptAead;
  final _CryptoCryptNoAuthDart _cryptoCryptNoAuth;

  final _NowDart _now;
  final _DebugDart _debug;
  final _VeilidVersionStringDart _veilidVersionString;
  final _VeilidVersionDart _veilidVersion;

  @override
  void initializeVeilidCore(Map<String, dynamic> platformConfigJson) {
    final nativePlatformConfig = jsonEncode(platformConfigJson).toNativeUtf8();

    _initializeVeilidCore(nativePlatformConfig);

    malloc.free(nativePlatformConfig);
  }

  @override
  void changeLogLevel(String layer, VeilidConfigLogLevel logLevel) {
    final nativeLogLevel = jsonEncode(logLevel).toNativeUtf8();
    final nativeLayer = layer.toNativeUtf8();
    _changeLogLevel(nativeLayer, nativeLogLevel);
    malloc
      ..free(nativeLayer)
      ..free(nativeLogLevel);
  }

  @override
  Future<Stream<VeilidUpdate>> startupVeilidCore(VeilidConfig config) async {
    final nativeConfig = jsonEncode(config).toNativeUtf8();
    final recvStreamPort = ReceivePort('veilid_api_stream');
    final sendStreamPort = recvStreamPort.sendPort;
    final recvPort = ReceivePort('startup_veilid_core');
    final sendPort = recvPort.sendPort;
    _startupVeilidCore(
        sendPort.nativePort, sendStreamPort.nativePort, nativeConfig);
    malloc.free(nativeConfig);
    return await processFutureStream(
        processStreamJson(VeilidUpdate.fromJson, recvStreamPort),
        recvPort.first);
  }

  @override
  Future<VeilidState> getVeilidState() async {
    final recvPort = ReceivePort('get_veilid_state');
    final sendPort = recvPort.sendPort;
    _getVeilidState(sendPort.nativePort);
    return await processFutureJson(VeilidState.fromJson, recvPort.first);
  }

  @override
  Future<void> attach() async {
    final recvPort = ReceivePort('attach');
    final sendPort = recvPort.sendPort;
    _attach(sendPort.nativePort);
    return await processFutureVoid(recvPort.first);
  }

  @override
  Future<void> detach() async {
    final recvPort = ReceivePort('detach');
    final sendPort = recvPort.sendPort;
    _detach(sendPort.nativePort);
    return await processFutureVoid(recvPort.first);
  }

  @override
  Future<void> shutdownVeilidCore() async {
    final recvPort = ReceivePort('shutdown_veilid_core');
    final sendPort = recvPort.sendPort;
    _shutdownVeilidCore(sendPort.nativePort);
    return await processFutureVoid(recvPort.first);
  }

  @override
  Future<VeilidRoutingContext> routingContext() async {
    final recvPort = ReceivePort('routing_context');
    final sendPort = recvPort.sendPort;
    _routingContext(sendPort.nativePort);
    final id = await processFuturePlain<int>(recvPort.first);
    return VeilidRoutingContextFFI._(_Ctx(id, this));
  }

  @override
  Future<RouteBlob> newPrivateRoute() async {
    final recvPort = ReceivePort('new_private_route');
    final sendPort = recvPort.sendPort;
    _newPrivateRoute(sendPort.nativePort);
    return await processFutureJson(RouteBlob.fromJson, recvPort.first);
  }

  @override
  Future<RouteBlob> newCustomPrivateRoute(
      Stability stability, Sequencing sequencing) async {
    final recvPort = ReceivePort('new_custom_private_route');
    final sendPort = recvPort.sendPort;
    _newCustomPrivateRoute(
        sendPort.nativePort,
        jsonEncode(stability).toNativeUtf8(),
        jsonEncode(sequencing).toNativeUtf8());

    return await processFutureJson(RouteBlob.fromJson, recvPort.first);
  }

  @override
  Future<String> importRemotePrivateRoute(Uint8List blob) async {
    final nativeEncodedBlob = base64UrlNoPadEncode(blob).toNativeUtf8();

    final recvPort = ReceivePort('import_remote_private_route');
    final sendPort = recvPort.sendPort;
    _importRemotePrivateRoute(sendPort.nativePort, nativeEncodedBlob);
    return await processFuturePlain(recvPort.first);
  }

  @override
  Future<void> releasePrivateRoute(String key) async {
    final nativeEncodedKey = key.toNativeUtf8();

    final recvPort = ReceivePort('release_private_route');
    final sendPort = recvPort.sendPort;
    _releasePrivateRoute(sendPort.nativePort, nativeEncodedKey);
    return await processFutureVoid(recvPort.first);
  }

  @override
  Future<void> appCallReply(String callId, Uint8List message) async {
    final nativeCallId = callId.toNativeUtf8();
    final nativeEncodedMessage = base64UrlNoPadEncode(message).toNativeUtf8();
    final recvPort = ReceivePort('app_call_reply');
    final sendPort = recvPort.sendPort;
    _appCallReply(sendPort.nativePort, nativeCallId, nativeEncodedMessage);
    return await processFutureVoid(recvPort.first);
  }

  @override
  Future<VeilidTableDB> openTableDB(String name, int columnCount) async {
    final recvPort = ReceivePort('open_table_db');
    final sendPort = recvPort.sendPort;
    _openTableDb(sendPort.nativePort, name.toNativeUtf8(), columnCount);
    final id = await processFuturePlain<int>(recvPort.first);
    return VeilidTableDBFFI._(_TDB(id, this));
  }

  @override
  Future<bool> deleteTableDB(String name) async {
    final recvPort = ReceivePort('delete_table_db');
    final sendPort = recvPort.sendPort;
    _deleteTableDb(sendPort.nativePort, name.toNativeUtf8());
    return await processFuturePlain(recvPort.first);
  }

  @override
  List<CryptoKind> validCryptoKinds() {
    final vckString = _validCryptoKinds();
    final vck = jsonDecode(vckString.toDartString()) as List<dynamic>;
    _freeString(vckString);
    return vck.map((v) => v as CryptoKind).toList();
  }

  @override
  Future<VeilidCryptoSystem> getCryptoSystem(CryptoKind kind) async {
    if (!validCryptoKinds().contains(kind)) {
      throw const VeilidAPIExceptionGeneric('unsupported cryptosystem');
    }
    return VeilidCryptoSystemFFI._(this, kind);
  }

  @override
  Future<VeilidCryptoSystem> bestCryptoSystem() async =>
      VeilidCryptoSystemFFI._(this, _bestCryptoKind());

  @override
  Future<List<TypedKey>> verifySignatures(List<TypedKey> nodeIds,
      Uint8List data, List<TypedSignature> signatures) async {
    final nativeNodeIds = jsonEncode(nodeIds).toNativeUtf8();
    final nativeData = base64UrlNoPadEncode(data).toNativeUtf8();
    final nativeSignatures = jsonEncode(signatures).toNativeUtf8();

    final recvPort = ReceivePort('verify_signatures');
    final sendPort = recvPort.sendPort;
    _verifySignatures(
        sendPort.nativePort, nativeNodeIds, nativeData, nativeSignatures);
    return await processFutureJson(
        jsonListConstructor<TypedKey>(TypedKey.fromJson), recvPort.first);
  }

  @override
  Future<List<TypedSignature>> generateSignatures(
      Uint8List data, List<TypedKeyPair> keyPairs) async {
    final nativeData = base64UrlNoPadEncode(data).toNativeUtf8();
    final nativeKeyPairs = jsonEncode(keyPairs).toNativeUtf8();

    final recvPort = ReceivePort('generate_signatures');
    final sendPort = recvPort.sendPort;
    _generateSignatures(sendPort.nativePort, nativeData, nativeKeyPairs);
    return await processFutureJson(
        jsonListConstructor<TypedSignature>(TypedSignature.fromJson),
        recvPort.first);
  }

  @override
  Timestamp now() {
    final ts = _now();
    return Timestamp(value: BigInt.from(ts));
  }

  @override
  Future<TypedKeyPair> generateKeyPair(CryptoKind kind) async {
    final recvPort = ReceivePort('generate_key_pair');
    final sendPort = recvPort.sendPort;
    _generateKeyPair(sendPort.nativePort, kind);
    return await processFutureJson(TypedKeyPair.fromJson, recvPort.first);
  }

  @override
  Future<String> debug(String command) async {
    final nativeCommand = command.toNativeUtf8();
    final recvPort = ReceivePort('debug');
    final sendPort = recvPort.sendPort;
    _debug(sendPort.nativePort, nativeCommand);
    return processFuturePlain(recvPort.first);
  }

  @override
  String veilidVersionString() {
    final versionString = _veilidVersionString();
    final ret = versionString.toDartString();
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
