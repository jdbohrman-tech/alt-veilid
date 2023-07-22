import 'dart:async';
import 'dart:ffi';
import 'dart:io';
import 'dart:isolate';
import 'dart:convert';
import 'dart:typed_data';

import 'package:ffi/ffi.dart';

import 'veilid.dart';
import 'veilid_encoding.dart';

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
// fn routing_context_create_dht_record(port: i64, id: u32, kind: u32, schema: FfiStr)
typedef _RoutingContextCreateDHTRecordC = Void Function(
    Int64, Uint32, Pointer<Utf8>, Uint32);
typedef _RoutingContextCreateDHTRecordDart = void Function(
    int, int, Pointer<Utf8>, int);
// fn routing_context_open_dht_record(port: i64, id: u32, key: FfiStr, writer: FfiStr)
typedef _RoutingContextOpenDHTRecordC = Void Function(
    Int64, Uint32, Pointer<Utf8>, Pointer<Utf8>);
typedef _RoutingContextOpenDHTRecordDart = void Function(
    int, int, Pointer<Utf8>, Pointer<Utf8>);
// fn routing_context_close_dht_record(port: i64, id: u32, key: FfiStr)
typedef _RoutingContextCloseDHTRecordC = Void Function(
    Int64, Uint32, Pointer<Utf8>);
typedef _RoutingContextCloseDHTRecordDart = void Function(
    int, int, Pointer<Utf8>);
// fn routing_context_delete_dht_record(port: i64, id: u32, key: FfiStr)
typedef _RoutingContextDeleteDHTRecordC = Void Function(
    Int64, Uint32, Pointer<Utf8>);
typedef _RoutingContextDeleteDHTRecordDart = void Function(
    int, int, Pointer<Utf8>);
// fn routing_context_get_dht_value(port: i64, id: u32, key: FfiStr, subkey: u32, force_refresh: bool)
typedef _RoutingContextGetDHTValueC = Void Function(
    Int64, Uint32, Pointer<Utf8>, Uint32, Bool);
typedef _RoutingContextGetDHTValueDart = void Function(
    int, int, Pointer<Utf8>, int, bool);
// fn routing_context_set_dht_value(port: i64, id: u32, key: FfiStr, subkey: u32, data: FfiStr)
typedef _RoutingContextSetDHTValueC = Void Function(
    Int64, Uint32, Pointer<Utf8>, Uint32, Pointer<Utf8>);
typedef _RoutingContextSetDHTValueDart = void Function(
    int, int, Pointer<Utf8>, int, Pointer<Utf8>);
// fn routing_context_watch_dht_values(port: i64, id: u32, key: FfiStr, subkeys: FfiStr, expiration: FfiStr, count: u32)
typedef _RoutingContextWatchDHTValuesC = Void Function(
    Int64, Uint32, Pointer<Utf8>, Pointer<Utf8>, Uint64, Uint32);
typedef _RoutingContextWatchDHTValuesDart = void Function(
    int, int, Pointer<Utf8>, Pointer<Utf8>, int, int);
// fn routing_context_cancel_dht_watch(port: i64, id: u32, key: FfiStr, subkeys: FfiStr)
typedef _RoutingContextCancelDHTWatchC = Void Function(
    Int64, Uint32, Pointer<Utf8>, Pointer<Utf8>);
typedef _RoutingContextCancelDHTWatchDart = void Function(
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
// fn table_db_get_keys(port: i64, id: u32, col: u32)
typedef _TableDbGetKeysC = Pointer<Utf8> Function(Uint64, Uint32, Uint32);
typedef _TableDbGetKeysDart = Pointer<Utf8> Function(int, int, int);
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
// fn valid_crypto_kinds() -> *mut c_char
typedef _ValidCryptoKindsC = Pointer<Utf8> Function();
typedef _ValidCryptoKindsDart = Pointer<Utf8> Function();
// fn best_crypto_kind() -> u32
typedef _BestCryptoKindC = Uint32 Function();
typedef _BestCryptoKindDart = int Function();
// fn verify_signatures(port: i64, node_ids: FfiStr, data: FfiStr, signatures: FfiStr)
typedef _VerifySignaturesC = Void Function(
    Int64, Pointer<Utf8>, Pointer<Utf8>, Pointer<Utf8>);
typedef _VerifySignaturesDart = void Function(
    int, Pointer<Utf8>, Pointer<Utf8>, Pointer<Utf8>);
// fn generate_signatures(port: i64, data: FfiStr, key_pairs: FfiStr)
typedef _GenerateSignaturesC = Void Function(
    Int64, Pointer<Utf8>, Pointer<Utf8>);
typedef _GenerateSignaturesDart = void Function(
    int, Pointer<Utf8>, Pointer<Utf8>);
// fn generate_key_pair(port: i64, kind: u32) {
typedef _GenerateKeyPairC = Void Function(Int64, Uint32);
typedef _GenerateKeyPairDart = void Function(int, int);
// fn crypto_cached_dh(port: i64, kind: u32, key: FfiStr, secret: FfiStr)
typedef _CryptoCachedDHC = Void Function(
    Int64, Uint32, Pointer<Utf8>, Pointer<Utf8>);
typedef _CryptoCachedDHDart = void Function(
    int, int, Pointer<Utf8>, Pointer<Utf8>);
// fn crypto_compute_dh(port: i64, kind: u32, key: FfiStr, secret: FfiStr)
typedef _CryptoComputeDHC = Void Function(
    Int64, Uint32, Pointer<Utf8>, Pointer<Utf8>);
typedef _CryptoComputeDHDart = void Function(
    int, int, Pointer<Utf8>, Pointer<Utf8>);
// fn crypto_random_bytes(port: i64, kind: u32, len: u32)
typedef _CryptoRandomBytesC = Void Function(Int64, Uint32, Uint32);
typedef _CryptoRandomBytesDart = void Function(int, int, int);
// fn crypto_default_salt_length(port: i64, kind: u32)
typedef _CryptoDefaultSaltLengthC = Void Function(Int64, Uint32);
typedef _CryptoDefaultSaltLengthDart = void Function(int, int);
// fn crypto_hash_password(port: i64, kind: u32, password: FfiStr, salt: FfiStr )
typedef _CryptoHashPasswordC = Void Function(
    Int64, Uint32, Pointer<Utf8>, Pointer<Utf8>);
typedef _CryptoHashPasswordDart = void Function(
    int, int, Pointer<Utf8>, Pointer<Utf8>);
// fn crypto_verify_password(port: i64, kind: u32, password: FfiStr, password_hash: FfiStr )
typedef _CryptoVerifyPasswordC = Void Function(
    Int64, Uint32, Pointer<Utf8>, Pointer<Utf8>);
typedef _CryptoVerifyPasswordDart = void Function(
    int, int, Pointer<Utf8>, Pointer<Utf8>);
// fn crypto_derive_shared_secret(port: i64, kind: u32, password: FfiStr, salt: FfiStr )
typedef _CryptoDeriveSharedSecretC = Void Function(
    Int64, Uint32, Pointer<Utf8>, Pointer<Utf8>);
typedef _CryptoDeriveSharedSecretDart = void Function(
    int, int, Pointer<Utf8>, Pointer<Utf8>);

// fn crypto_random_nonce(port: i64, kind: u32)
typedef _CryptoRandomNonceC = Void Function(Int64, Uint32);
typedef _CryptoRandomNonceDart = void Function(int, int);
// fn crypto_random_shared_secret(port: i64, kind: u32)
typedef _CryptoRandomSharedSecretC = Void Function(Int64, Uint32);
typedef _CryptoRandomSharedSecretDart = void Function(int, int);
// fn crypto_generate_key_pair(port: i64, kind: u32)
typedef _CryptoGenerateKeyPairC = Void Function(Int64, Uint32);
typedef _CryptoGenerateKeyPairDart = void Function(int, int);
// fn crypto_generate_hash(port: i64, kind: u32, data: FfiStr)
typedef _CryptoGenerateHashC = Void Function(Int64, Uint32, Pointer<Utf8>);
typedef _CryptoGenerateHashDart = void Function(int, int, Pointer<Utf8>);
// fn crypto_validate_key_pair(port: i64, kind: u32, key: FfiStr, secret: FfiStr)
typedef _CryptoValidateKeyPairC = Void Function(
    Int64, Uint32, Pointer<Utf8>, Pointer<Utf8>);
typedef _CryptoValidateKeyPairDart = void Function(
    int, int, Pointer<Utf8>, Pointer<Utf8>);
// fn crypto_validate_hash(port: i64, kind: u32, data: FfiStr, hash: FfiStr)
typedef _CryptoValidateHashC = Void Function(
    Int64, Uint32, Pointer<Utf8>, Pointer<Utf8>);
typedef _CryptoValidateHashDart = void Function(
    int, int, Pointer<Utf8>, Pointer<Utf8>);
// fn crypto_distance(port: i64, kind: u32, key1: FfiStr, key2: FfiStr)
typedef _CryptoDistanceC = Void Function(
    Int64, Uint32, Pointer<Utf8>, Pointer<Utf8>);
typedef _CryptoDistanceDart = void Function(
    int, int, Pointer<Utf8>, Pointer<Utf8>);
// fn crypto_sign(port: i64, kind: u32, key: FfiStr, secret: FfiStr, data: FfiStr)
typedef _CryptoSignC = Void Function(
    Int64, Uint32, Pointer<Utf8>, Pointer<Utf8>, Pointer<Utf8>);
typedef _CryptoSignDart = void Function(
    int, int, Pointer<Utf8>, Pointer<Utf8>, Pointer<Utf8>);
// fn crypto_verify(port: i64, kind: u32, key: FfiStr, data: FfiStr, signature: FfiStr)
typedef _CryptoVerifyC = Void Function(
    Int64, Uint32, Pointer<Utf8>, Pointer<Utf8>, Pointer<Utf8>);
typedef _CryptoVerifyDart = void Function(
    int, int, Pointer<Utf8>, Pointer<Utf8>, Pointer<Utf8>);
// fn crypto_aead_overhead(port: i64, kind: u32)
typedef _CryptoAeadOverheadC = Void Function(Int64, Uint32);
typedef _CryptoAeadOverheadDart = void Function(int, int);
// fn crypto_decrypt_aead(port: i64, kind: u32, body: FfiStr, nonce: FfiStr, shared_secret: FfiStr, associated_data: FfiStr)
typedef _CryptoDecryptAeadC = Void Function(
    Int64, Uint32, Pointer<Utf8>, Pointer<Utf8>, Pointer<Utf8>, Pointer<Utf8>);
typedef _CryptoDecryptAeadDart = void Function(
    int, int, Pointer<Utf8>, Pointer<Utf8>, Pointer<Utf8>, Pointer<Utf8>);
// fn crypto_encrypt_aead(port: i64, kind: u32, body: FfiStr, nonce: FfiStr, shared_secret: FfiStr, associated_data: FfiStr)
typedef _CryptoEncryptAeadC = Void Function(
    Int64, Uint32, Pointer<Utf8>, Pointer<Utf8>, Pointer<Utf8>, Pointer<Utf8>);
typedef _CryptoEncryptAeadDart = void Function(
    int, int, Pointer<Utf8>, Pointer<Utf8>, Pointer<Utf8>, Pointer<Utf8>);
// fn crypto_crypt_no_auth(port: i64, kind: u32, body: FfiStr, nonce: FfiStr, shared_secret: FfiStr)
typedef _CryptoCryptNoAuthC = Void Function(
    Int64, Uint32, Pointer<Utf8>, Pointer<Utf8>, Pointer<Utf8>);
typedef _CryptoCryptNoAuthDart = void Function(
    int, int, Pointer<Utf8>, Pointer<Utf8>, Pointer<Utf8>);

// fn now() -> u64
typedef _NowC = Uint64 Function();
typedef _NowDart = int Function();
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
final class VeilidVersionFFI extends Struct {
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
          if (list[1] == null && null is! T) {
            throw const VeilidAPIExceptionInternal(
                "Null MESSAGE_OK value on non-nullable type");
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
          if (list[1] is! String) {
            throw const VeilidAPIExceptionInternal(
                "Non-string MESSAGE_OK_JSON value");
          }
          var ret = jsonDecode(list[1] as String);
          if (ret == null) {
            throw const VeilidAPIExceptionInternal(
                "Null JSON object on non nullable type");
          }
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

Future<T?> processFutureOptJson<T>(
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
            return null;
          }
          if (list[1] is! String) {
            throw const VeilidAPIExceptionInternal(
                "Non-string MESSAGE_OK_JSON optional value");
          }
          var ret = jsonDecode(list[1] as String);
          if (ret == null) {
            return null;
          }
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
              throw const VeilidAPIExceptionInternal(
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
  int? id;
  final VeilidFFI ffi;
  _Ctx(int this.id, this.ffi);

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
  final _Ctx _ctx;
  static final Finalizer<_Ctx> _finalizer = Finalizer((ctx) => ctx.close());

  VeilidRoutingContextFFI._(this._ctx) {
    _finalizer.attach(this, _ctx, detach: this);
  }

  @override
  void close() {
    _ctx.close();
  }

  @override
  VeilidRoutingContextFFI withPrivacy() {
    _ctx.ensureValid();
    final newId = _ctx.ffi._routingContextWithPrivacy(_ctx.id!);
    return VeilidRoutingContextFFI._(_Ctx(newId, _ctx.ffi));
  }

  @override
  VeilidRoutingContextFFI withCustomPrivacy(SafetySelection safetySelection) {
    _ctx.ensureValid();
    final newId = _ctx.ffi._routingContextWithCustomPrivacy(
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
  Future<Uint8List> appCall(String target, Uint8List request) async {
    _ctx.ensureValid();
    var nativeEncodedTarget = target.toNativeUtf8();
    var nativeEncodedRequest = base64UrlNoPadEncode(request).toNativeUtf8();

    final recvPort = ReceivePort("routing_context_app_call");
    final sendPort = recvPort.sendPort;
    _ctx.ffi._routingContextAppCall(sendPort.nativePort, _ctx.id!,
        nativeEncodedTarget, nativeEncodedRequest);
    final out = await processFuturePlain(recvPort.first);
    return base64UrlNoPadDecode(out);
  }

  @override
  Future<void> appMessage(String target, Uint8List message) {
    _ctx.ensureValid();
    final nativeEncodedTarget = target.toNativeUtf8();
    final nativeEncodedMessage = base64UrlNoPadEncode(message).toNativeUtf8();

    final recvPort = ReceivePort("routing_context_app_message");
    final sendPort = recvPort.sendPort;
    _ctx.ffi._routingContextAppMessage(sendPort.nativePort, _ctx.id!,
        nativeEncodedTarget, nativeEncodedMessage);
    return processFutureVoid(recvPort.first);
  }

  @override
  Future<DHTRecordDescriptor> createDHTRecord(DHTSchema schema,
      {CryptoKind kind = 0}) async {
    _ctx.ensureValid();
    final nativeSchema = jsonEncode(schema).toNativeUtf8();
    final recvPort = ReceivePort("routing_context_create_dht_record");
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
        writer != null ? jsonEncode(key).toNativeUtf8() : nullptr;
    final recvPort = ReceivePort("routing_context_open_dht_record");
    final sendPort = recvPort.sendPort;
    _ctx.ffi._routingContextOpenDHTRecord(
        sendPort.nativePort, _ctx.id!, nativeKey, nativeWriter);
    final dhtRecordDescriptor =
        await processFutureJson(DHTRecordDescriptor.fromJson, recvPort.first);
    return dhtRecordDescriptor;
  }

  @override
  Future<void> closeDHTRecord(TypedKey key) {
    _ctx.ensureValid();
    final nativeKey = jsonEncode(key).toNativeUtf8();
    final recvPort = ReceivePort("routing_context_close_dht_record");
    final sendPort = recvPort.sendPort;
    _ctx.ffi._routingContextCloseDHTRecord(
        sendPort.nativePort, _ctx.id!, nativeKey);
    return processFutureVoid(recvPort.first);
  }

  @override
  Future<void> deleteDHTRecord(TypedKey key) {
    _ctx.ensureValid();
    final nativeKey = jsonEncode(key).toNativeUtf8();
    final recvPort = ReceivePort("routing_context_delete_dht_record");
    final sendPort = recvPort.sendPort;
    _ctx.ffi._routingContextDeleteDHTRecord(
        sendPort.nativePort, _ctx.id!, nativeKey);
    return processFutureVoid(recvPort.first);
  }

  @override
  Future<ValueData?> getDHTValue(
      TypedKey key, int subkey, bool forceRefresh) async {
    _ctx.ensureValid();
    final nativeKey = jsonEncode(key).toNativeUtf8();
    final recvPort = ReceivePort("routing_context_get_dht_value");
    final sendPort = recvPort.sendPort;
    _ctx.ffi._routingContextGetDHTValue(
        sendPort.nativePort, _ctx.id!, nativeKey, subkey, forceRefresh);
    final valueData = await processFutureJson(
        optFromJson(ValueData.fromJson), recvPort.first);
    return valueData;
  }

  @override
  Future<ValueData?> setDHTValue(
      TypedKey key, int subkey, Uint8List data) async {
    _ctx.ensureValid();
    final nativeKey = jsonEncode(key).toNativeUtf8();
    final nativeData = base64UrlNoPadEncode(data).toNativeUtf8();

    final recvPort = ReceivePort("routing_context_set_dht_value");
    final sendPort = recvPort.sendPort;
    _ctx.ffi._routingContextSetDHTValue(
        sendPort.nativePort, _ctx.id!, nativeKey, subkey, nativeData);
    final valueData = await processFutureJson(
        optFromJson(ValueData.fromJson), recvPort.first);
    return valueData;
  }

  @override
  Future<Timestamp> watchDHTValues(TypedKey key, List<ValueSubkeyRange> subkeys,
      Timestamp expiration, int count) async {
    _ctx.ensureValid();
    final nativeKey = jsonEncode(key).toNativeUtf8();
    final nativeSubkeys = jsonEncode(subkeys).toNativeUtf8();
    final nativeExpiration = expiration.value.toInt();

    final recvPort = ReceivePort("routing_context_watch_dht_values");
    final sendPort = recvPort.sendPort;
    _ctx.ffi._routingContextWatchDHTValues(sendPort.nativePort, _ctx.id!,
        nativeKey, nativeSubkeys, nativeExpiration, count);
    final actualExpiration = Timestamp(
        value: BigInt.from(await processFuturePlain<int>(recvPort.first)));
    return actualExpiration;
  }

  @override
  Future<bool> cancelDHTWatch(
      TypedKey key, List<ValueSubkeyRange> subkeys) async {
    _ctx.ensureValid();
    final nativeKey = jsonEncode(key).toNativeUtf8();
    final nativeSubkeys = jsonEncode(subkeys).toNativeUtf8();

    final recvPort = ReceivePort("routing_context_cancel_dht_watch");
    final sendPort = recvPort.sendPort;
    _ctx.ffi._routingContextCancelDHTWatch(
        sendPort.nativePort, _ctx.id!, nativeKey, nativeSubkeys);
    final cancelled = await processFuturePlain<bool>(recvPort.first);
    return cancelled;
  }
}

class _TDBT {
  int? id;
  final VeilidTableDBFFI tdbffi;
  final VeilidFFI ffi;

  _TDBT(int this.id, this.tdbffi, this.ffi);
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
  final _TDBT _tdbt;
  static final Finalizer<_TDBT> _finalizer = Finalizer((tdbt) => tdbt.close());

  VeilidTableDBTransactionFFI._(this._tdbt) {
    _finalizer.attach(this, _tdbt, detach: this);
  }

  @override
  bool isDone() {
    return _tdbt.id == null;
  }

  @override
  Future<void> commit() async {
    _tdbt.ensureValid();
    final recvPort = ReceivePort("veilid_table_db_transaction_commit");
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
    final recvPort = ReceivePort("veilid_table_db_transaction_rollback");
    final sendPort = recvPort.sendPort;
    _tdbt.ffi._tableDbTransactionRollback(
      sendPort.nativePort,
      _tdbt.id!,
    );
    await processFutureVoid(recvPort.first);
    _tdbt.close();
  }

  @override
  Future<void> store(int col, Uint8List key, Uint8List value) {
    _tdbt.ensureValid();
    final nativeEncodedKey = base64UrlNoPadEncode(key).toNativeUtf8();
    final nativeEncodedValue = base64UrlNoPadEncode(value).toNativeUtf8();

    final recvPort = ReceivePort("veilid_table_db_transaction_store");
    final sendPort = recvPort.sendPort;
    _tdbt.ffi._tableDbTransactionStore(
      sendPort.nativePort,
      _tdbt.id!,
      col,
      nativeEncodedKey,
      nativeEncodedValue,
    );
    return processFutureVoid(recvPort.first);
  }

  @override
  Future<void> delete(int col, Uint8List key) {
    _tdbt.ensureValid();
    final nativeEncodedKey = base64UrlNoPadEncode(key).toNativeUtf8();

    final recvPort = ReceivePort("veilid_table_db_transaction_delete");
    final sendPort = recvPort.sendPort;
    _tdbt.ffi._tableDbTransactionDelete(
      sendPort.nativePort,
      _tdbt.id!,
      col,
      nativeEncodedKey,
    );
    return processFuturePlain(recvPort.first);
  }
}

class _TDB {
  int? id;
  final VeilidFFI ffi;
  _TDB(int this.id, this.ffi);
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
  final _TDB _tdb;
  static final Finalizer<_TDB> _finalizer = Finalizer((tdb) => tdb.close());

  VeilidTableDBFFI._(this._tdb) {
    _finalizer.attach(this, _tdb, detach: this);
  }

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
  Future<List<Uint8List>> getKeys(int col) {
    _tdb.ensureValid();

    final recvPort = ReceivePort("veilid_table_db_get_keys");
    final sendPort = recvPort.sendPort;

    _tdb.ffi._tableDbGetKeys(sendPort.nativePort, _tdb.id!, col);

    return processFutureJson(
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
  Future<void> store(int col, Uint8List key, Uint8List value) {
    _tdb.ensureValid();

    final nativeEncodedKey = base64UrlNoPadEncode(key).toNativeUtf8();
    final nativeEncodedValue = base64UrlNoPadEncode(value).toNativeUtf8();

    final recvPort = ReceivePort("veilid_table_db_store");
    final sendPort = recvPort.sendPort;
    _tdb.ffi._tableDbStore(
      sendPort.nativePort,
      _tdb.id!,
      col,
      nativeEncodedKey,
      nativeEncodedValue,
    );
    return processFutureVoid(recvPort.first);
  }

  @override
  Future<Uint8List?> load(int col, Uint8List key) async {
    _tdb.ensureValid();
    final nativeEncodedKey = base64UrlNoPadEncode(key).toNativeUtf8();

    final recvPort = ReceivePort("veilid_table_db_load");
    final sendPort = recvPort.sendPort;
    _tdb.ffi._tableDbLoad(
      sendPort.nativePort,
      _tdb.id!,
      col,
      nativeEncodedKey,
    );
    String? out = await processFuturePlain(recvPort.first);
    if (out == null) {
      return null;
    }
    return base64UrlNoPadDecode(out);
  }

  @override
  Future<Uint8List?> delete(int col, Uint8List key) async {
    _tdb.ensureValid();
    final nativeEncodedKey = base64UrlNoPadEncode(key).toNativeUtf8();

    final recvPort = ReceivePort("veilid_table_db_delete");
    final sendPort = recvPort.sendPort;
    _tdb.ffi._tableDbDelete(
      sendPort.nativePort,
      _tdb.id!,
      col,
      nativeEncodedKey,
    );
    String? out = await processFuturePlain(recvPort.first);
    if (out == null) {
      return null;
    }
    return base64UrlNoPadDecode(out);
  }
}

// FFI implementation of VeilidCryptoSystem
class VeilidCryptoSystemFFI extends VeilidCryptoSystem {
  final CryptoKind _kind;
  final VeilidFFI _ffi;

  VeilidCryptoSystemFFI._(this._ffi, this._kind);

  @override
  CryptoKind kind() {
    return _kind;
  }

  @override
  Future<SharedSecret> cachedDH(PublicKey key, SecretKey secret) {
    final nativeKey = jsonEncode(key).toNativeUtf8();
    final nativeSecret = jsonEncode(secret).toNativeUtf8();

    final recvPort = ReceivePort("crypto_cached_dh");
    final sendPort = recvPort.sendPort;
    _ffi._cryptoCachedDH(sendPort.nativePort, _kind, nativeKey, nativeSecret);
    return processFutureJson(SharedSecret.fromJson, recvPort.first);
  }

  @override
  Future<SharedSecret> computeDH(PublicKey key, SecretKey secret) {
    final nativeKey = jsonEncode(key).toNativeUtf8();
    final nativeSecret = jsonEncode(secret).toNativeUtf8();

    final recvPort = ReceivePort("crypto_compute_dh");
    final sendPort = recvPort.sendPort;
    _ffi._cryptoComputeDH(sendPort.nativePort, _kind, nativeKey, nativeSecret);
    return processFutureJson(SharedSecret.fromJson, recvPort.first);
  }

  @override
  Future<Uint8List> randomBytes(int len) async {
    final recvPort = ReceivePort("crypto_random_bytes");
    final sendPort = recvPort.sendPort;
    _ffi._cryptoRandomBytes(sendPort.nativePort, _kind, len);
    final out = await processFuturePlain(recvPort.first);
    return base64UrlNoPadDecode(out);
  }

  @override
  Future<int> defaultSaltLength() {
    final recvPort = ReceivePort("crypto_default_salt_length");
    final sendPort = recvPort.sendPort;
    _ffi._cryptoDefaultSaltLength(sendPort.nativePort, _kind);
    return processFuturePlain(recvPort.first);
  }

  @override
  Future<String> hashPassword(Uint8List password, Uint8List salt) {
    final nativeEncodedPassword = base64UrlNoPadEncode(password).toNativeUtf8();
    final nativeEncodedSalt = base64UrlNoPadEncode(salt).toNativeUtf8();

    final recvPort = ReceivePort("crypto_hash_password");
    final sendPort = recvPort.sendPort;
    _ffi._cryptoHashPassword(
        sendPort.nativePort, _kind, nativeEncodedPassword, nativeEncodedSalt);
    return processFuturePlain(recvPort.first);
  }

  @override
  Future<bool> verifyPassword(Uint8List password, String passwordHash) {
    final nativeEncodedPassword = base64UrlNoPadEncode(password).toNativeUtf8();
    final nativeEncodedPasswordHash = passwordHash.toNativeUtf8();

    final recvPort = ReceivePort("crypto_verify_password");
    final sendPort = recvPort.sendPort;
    _ffi._cryptoVerifyPassword(sendPort.nativePort, _kind,
        nativeEncodedPassword, nativeEncodedPasswordHash);
    return processFuturePlain(recvPort.first);
  }

  @override
  Future<SharedSecret> deriveSharedSecret(Uint8List password, Uint8List salt) {
    final nativeEncodedPassword = base64UrlNoPadEncode(password).toNativeUtf8();
    final nativeEncodedSalt = base64UrlNoPadEncode(salt).toNativeUtf8();

    final recvPort = ReceivePort("crypto_derive_shared_secret");
    final sendPort = recvPort.sendPort;
    _ffi._cryptoDeriveSharedSecret(
        sendPort.nativePort, _kind, nativeEncodedPassword, nativeEncodedSalt);
    return processFutureJson(SharedSecret.fromJson, recvPort.first);
  }

  @override
  Future<Nonce> randomNonce() {
    final recvPort = ReceivePort("crypto_random_nonce");
    final sendPort = recvPort.sendPort;
    _ffi._cryptoRandomNonce(sendPort.nativePort, _kind);
    return processFutureJson(Nonce.fromJson, recvPort.first);
  }

  @override
  Future<SharedSecret> randomSharedSecret() {
    final recvPort = ReceivePort("crypto_random_shared_secret");
    final sendPort = recvPort.sendPort;
    _ffi._cryptoRandomSharedSecret(sendPort.nativePort, _kind);
    return processFutureJson(SharedSecret.fromJson, recvPort.first);
  }

  @override
  Future<KeyPair> generateKeyPair() {
    final recvPort = ReceivePort("crypto_generate_key_pair");
    final sendPort = recvPort.sendPort;
    _ffi._cryptoGenerateKeyPair(sendPort.nativePort, _kind);
    return processFutureJson(KeyPair.fromJson, recvPort.first);
  }

  @override
  Future<HashDigest> generateHash(Uint8List data) {
    final nativeEncodedData = base64UrlNoPadEncode(data).toNativeUtf8();

    final recvPort = ReceivePort("crypto_generate_hash");
    final sendPort = recvPort.sendPort;
    _ffi._cryptoGenerateHash(sendPort.nativePort, _kind, nativeEncodedData);
    return processFutureJson(HashDigest.fromJson, recvPort.first);
  }

  @override
  Future<bool> validateKeyPair(PublicKey key, SecretKey secret) {
    final nativeKey = jsonEncode(key).toNativeUtf8();
    final nativeSecret = jsonEncode(secret).toNativeUtf8();

    final recvPort = ReceivePort("crypto_validate_key_pair");
    final sendPort = recvPort.sendPort;
    _ffi._cryptoValidateKeyPair(
        sendPort.nativePort, _kind, nativeKey, nativeSecret);
    return processFuturePlain(recvPort.first);
  }

  @override
  Future<bool> validateHash(Uint8List data, HashDigest hash) {
    final nativeEncodedData = base64UrlNoPadEncode(data).toNativeUtf8();
    final nativeHash = jsonEncode(hash).toNativeUtf8();

    final recvPort = ReceivePort("crypto_validate_hash");
    final sendPort = recvPort.sendPort;
    _ffi._cryptoValidateHash(
        sendPort.nativePort, _kind, nativeEncodedData, nativeHash);
    return processFuturePlain(recvPort.first);
  }

  @override
  Future<CryptoKeyDistance> distance(CryptoKey key1, CryptoKey key2) {
    final nativeKey1 = jsonEncode(key1).toNativeUtf8();
    final nativeKey2 = jsonEncode(key2).toNativeUtf8();

    final recvPort = ReceivePort("crypto_distance");
    final sendPort = recvPort.sendPort;
    _ffi._cryptoDistance(sendPort.nativePort, _kind, nativeKey1, nativeKey2);
    return processFutureJson(CryptoKeyDistance.fromJson, recvPort.first);
  }

  @override
  Future<Signature> sign(PublicKey key, SecretKey secret, Uint8List data) {
    final nativeKey = jsonEncode(key).toNativeUtf8();
    final nativeSecret = jsonEncode(secret).toNativeUtf8();
    final nativeEncodedData = base64UrlNoPadEncode(data).toNativeUtf8();

    final recvPort = ReceivePort("crypto_sign");
    final sendPort = recvPort.sendPort;
    _ffi._cryptoSign(
        sendPort.nativePort, _kind, nativeKey, nativeSecret, nativeEncodedData);
    return processFutureJson(Signature.fromJson, recvPort.first);
  }

  @override
  Future<void> verify(PublicKey key, Uint8List data, Signature signature) {
    final nativeKey = jsonEncode(key).toNativeUtf8();
    final nativeEncodedData = base64UrlNoPadEncode(data).toNativeUtf8();
    final nativeSignature = jsonEncode(signature).toNativeUtf8();

    final recvPort = ReceivePort("crypto_verify");
    final sendPort = recvPort.sendPort;
    _ffi._cryptoVerify(sendPort.nativePort, _kind, nativeKey, nativeEncodedData,
        nativeSignature);
    return processFutureVoid(recvPort.first);
  }

  @override
  Future<int> aeadOverhead() {
    final recvPort = ReceivePort("crypto_aead_overhead");
    final sendPort = recvPort.sendPort;
    _ffi._cryptoAeadOverhead(
      sendPort.nativePort,
      _kind,
    );
    return processFuturePlain(recvPort.first);
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

    final recvPort = ReceivePort("crypto_decrypt_aead");
    final sendPort = recvPort.sendPort;
    _ffi._cryptoDecryptAead(sendPort.nativePort, _kind, nativeEncodedBody,
        nativeNonce, nativeSharedSecret, nativeSignature);
    final out = await processFuturePlain(recvPort.first);
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

    final recvPort = ReceivePort("crypto_encrypt_aead");
    final sendPort = recvPort.sendPort;
    _ffi._cryptoEncryptAead(sendPort.nativePort, _kind, nativeEncodedBody,
        nativeNonce, nativeSharedSecret, nativeSignature);
    final out = await processFuturePlain(recvPort.first);
    return base64UrlNoPadDecode(out);
  }

  @override
  Future<Uint8List> cryptNoAuth(
      Uint8List body, Nonce nonce, SharedSecret sharedSecret) async {
    final nativeEncodedBody = base64UrlNoPadEncode(body).toNativeUtf8();
    final nativeNonce = jsonEncode(nonce).toNativeUtf8();
    final nativeSharedSecret = jsonEncode(sharedSecret).toNativeUtf8();

    final recvPort = ReceivePort("crypto_crypt_no_auth");
    final sendPort = recvPort.sendPort;
    _ffi._cryptoCryptNoAuth(sendPort.nativePort, _kind, nativeEncodedBody,
        nativeNonce, nativeSharedSecret);
    final out = await processFuturePlain(recvPort.first);
    return base64UrlNoPadDecode(out);
  }
}

// FFI implementation of high level Veilid API
class VeilidFFI extends Veilid {
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
  final _CryptoDeriveSharedSecretDart _cryptoDeriveSharedSecret;

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
        _routingContextCreateDHTRecord = dylib.lookupFunction<
                _RoutingContextCreateDHTRecordC,
                _RoutingContextCreateDHTRecordDart>(
            'routing_context_create_dht_record'),
        _routingContextOpenDHTRecord = dylib.lookupFunction<
                _RoutingContextOpenDHTRecordC,
                _RoutingContextOpenDHTRecordDart>(
            'routing_context_open_dht_record'),
        _routingContextCloseDHTRecord = dylib.lookupFunction<
                _RoutingContextCloseDHTRecordC,
                _RoutingContextCloseDHTRecordDart>(
            'routing_context_close_dht_record'),
        _routingContextDeleteDHTRecord = dylib.lookupFunction<
                _RoutingContextDeleteDHTRecordC,
                _RoutingContextDeleteDHTRecordDart>(
            'routing_context_delete_dht_record'),
        _routingContextGetDHTValue = dylib.lookupFunction<
            _RoutingContextGetDHTValueC,
            _RoutingContextGetDHTValueDart>('routing_context_get_dht_value'),
        _routingContextSetDHTValue = dylib.lookupFunction<
            _RoutingContextSetDHTValueC,
            _RoutingContextSetDHTValueDart>('routing_context_set_dht_value'),
        _routingContextWatchDHTValues = dylib.lookupFunction<
                _RoutingContextWatchDHTValuesC,
                _RoutingContextWatchDHTValuesDart>(
            'routing_context_watch_dht_values'),
        _routingContextCancelDHTWatch = dylib.lookupFunction<
                _RoutingContextCancelDHTWatchC,
                _RoutingContextCancelDHTWatchDart>(
            'routing_context_cancel_dht_watch'),
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
        _validCryptoKinds =
            dylib.lookupFunction<_ValidCryptoKindsC, _ValidCryptoKindsDart>(
                'valid_crypto_kinds'),
        _bestCryptoKind =
            dylib.lookupFunction<_BestCryptoKindC, _BestCryptoKindDart>(
                'best_crypto_kind'),
        _verifySignatures =
            dylib.lookupFunction<_VerifySignaturesC, _VerifySignaturesDart>(
                'verify_signatures'),
        _generateSignatures =
            dylib.lookupFunction<_GenerateSignaturesC, _GenerateSignaturesDart>(
                'generate_signatures'),
        _generateKeyPair =
            dylib.lookupFunction<_GenerateKeyPairC, _GenerateKeyPairDart>(
                'generate_key_pair'),
        _cryptoCachedDH =
            dylib.lookupFunction<_CryptoCachedDHC, _CryptoCachedDHDart>(
                'crypto_cached_dh'),
        _cryptoComputeDH =
            dylib.lookupFunction<_CryptoComputeDHC, _CryptoComputeDHDart>(
                'crypto_compute_dh'),
        _cryptoRandomBytes =
            dylib.lookupFunction<_CryptoRandomBytesC, _CryptoRandomBytesDart>(
                'crypto_random_bytes'),
        _cryptoDefaultSaltLength = dylib.lookupFunction<
            _CryptoDefaultSaltLengthC,
            _CryptoDefaultSaltLengthDart>('crypto_default_salt_length'),
        _cryptoHashPassword =
            dylib.lookupFunction<_CryptoHashPasswordC, _CryptoHashPasswordDart>(
                'crypto_hash_password'),
        _cryptoVerifyPassword = dylib.lookupFunction<_CryptoVerifyPasswordC,
            _CryptoVerifyPasswordDart>('crypto_verify_password'),
        _cryptoDeriveSharedSecret = dylib.lookupFunction<
            _CryptoDeriveSharedSecretC,
            _CryptoVerifyPasswordDart>('crypto_derive_shared_secret'),
        _cryptoRandomNonce =
            dylib.lookupFunction<_CryptoRandomNonceC, _CryptoRandomNonceDart>(
                'crypto_random_nonce'),
        _cryptoRandomSharedSecret = dylib.lookupFunction<
            _CryptoRandomSharedSecretC,
            _CryptoRandomSharedSecretDart>('crypto_random_shared_secret'),
        _cryptoGenerateKeyPair = dylib.lookupFunction<_CryptoGenerateKeyPairC,
            _CryptoGenerateKeyPairDart>('crypto_generate_key_pair'),
        _cryptoGenerateHash =
            dylib.lookupFunction<_CryptoGenerateHashC, _CryptoGenerateHashDart>(
                'crypto_generate_hash'),
        _cryptoValidateKeyPair = dylib.lookupFunction<_CryptoValidateKeyPairC,
            _CryptoValidateKeyPairDart>('crypto_validate_key_pair'),
        _cryptoValidateHash =
            dylib.lookupFunction<_CryptoValidateHashC, _CryptoValidateHashDart>(
                'crypto_validate_hash'),
        _cryptoDistance =
            dylib.lookupFunction<_CryptoDistanceC, _CryptoDistanceDart>(
                'crypto_distance'),
        _cryptoSign =
            dylib.lookupFunction<_CryptoSignC, _CryptoSignDart>('crypto_sign'),
        _cryptoVerify = dylib
            .lookupFunction<_CryptoVerifyC, _CryptoVerifyDart>('crypto_verify'),
        _cryptoAeadOverhead =
            dylib.lookupFunction<_CryptoAeadOverheadC, _CryptoAeadOverheadDart>(
                'crypto_aead_overhead'),
        _cryptoDecryptAead =
            dylib.lookupFunction<_CryptoDecryptAeadC, _CryptoDecryptAeadDart>(
                'crypto_decrypt_aead'),
        _cryptoEncryptAead =
            dylib.lookupFunction<_CryptoEncryptAeadC, _CryptoEncryptAeadDart>(
                'crypto_encrypt_aead'),
        _cryptoCryptNoAuth =
            dylib.lookupFunction<_CryptoCryptNoAuthC, _CryptoCryptNoAuthDart>(
                'crypto_crypt_no_auth'),
        _now = dylib.lookupFunction<_NowC, _NowDart>('now'),
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
    var nativePlatformConfig = jsonEncode(platformConfigJson).toNativeUtf8();

    _initializeVeilidCore(nativePlatformConfig);

    malloc.free(nativePlatformConfig);
  }

  @override
  void changeLogLevel(String layer, VeilidConfigLogLevel logLevel) {
    var nativeLogLevel = jsonEncode(logLevel).toNativeUtf8();
    var nativeLayer = layer.toNativeUtf8();
    _changeLogLevel(nativeLayer, nativeLogLevel);
    malloc.free(nativeLayer);
    malloc.free(nativeLogLevel);
  }

  @override
  Future<Stream<VeilidUpdate>> startupVeilidCore(VeilidConfig config) {
    var nativeConfig = jsonEncode(config).toNativeUtf8();
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
  Future<RouteBlob> newPrivateRoute() {
    final recvPort = ReceivePort("new_private_route");
    final sendPort = recvPort.sendPort;
    _newPrivateRoute(sendPort.nativePort);
    return processFutureJson(RouteBlob.fromJson, recvPort.first);
  }

  @override
  Future<RouteBlob> newCustomPrivateRoute(
      Stability stability, Sequencing sequencing) {
    final recvPort = ReceivePort("new_custom_private_route");
    final sendPort = recvPort.sendPort;
    _newCustomPrivateRoute(
        sendPort.nativePort,
        jsonEncode(stability).toNativeUtf8(),
        jsonEncode(sequencing).toNativeUtf8());

    return processFutureJson(RouteBlob.fromJson, recvPort.first);
  }

  @override
  Future<String> importRemotePrivateRoute(Uint8List blob) {
    final nativeEncodedBlob = base64UrlNoPadEncode(blob).toNativeUtf8();

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
  Future<void> appCallReply(String call_id, Uint8List message) {
    final nativeCallId = call_id.toNativeUtf8();
    final nativeEncodedMessage = base64UrlNoPadEncode(message).toNativeUtf8();
    final recvPort = ReceivePort("app_call_reply");
    final sendPort = recvPort.sendPort;
    _appCallReply(sendPort.nativePort, nativeCallId, nativeEncodedMessage);
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
  List<CryptoKind> validCryptoKinds() {
    final vckString = _validCryptoKinds();
    final vck = jsonDecode(vckString.toDartString());
    _freeString(vckString);
    return vck;
  }

  @override
  Future<VeilidCryptoSystem> getCryptoSystem(CryptoKind kind) async {
    if (!validCryptoKinds().contains(kind)) {
      throw VeilidAPIExceptionGeneric("unsupported cryptosystem");
    }
    return VeilidCryptoSystemFFI._(this, kind);
  }

  @override
  Future<VeilidCryptoSystem> bestCryptoSystem() async {
    return VeilidCryptoSystemFFI._(this, _bestCryptoKind());
  }

  @override
  Future<List<TypedKey>> verifySignatures(
      List<TypedKey> nodeIds, Uint8List data, List<TypedSignature> signatures) {
    final nativeNodeIds = jsonEncode(nodeIds).toNativeUtf8();
    final nativeData = base64UrlNoPadEncode(data).toNativeUtf8();
    final nativeSignatures = jsonEncode(signatures).toNativeUtf8();

    final recvPort = ReceivePort("verify_signatures");
    final sendPort = recvPort.sendPort;
    _verifySignatures(
        sendPort.nativePort, nativeNodeIds, nativeData, nativeSignatures);
    return processFutureJson(
        jsonListConstructor<TypedKey>(TypedKey.fromJson), recvPort.first);
  }

  @override
  Future<List<TypedSignature>> generateSignatures(
      Uint8List data, List<TypedKeyPair> keyPairs) {
    final nativeData = base64UrlNoPadEncode(data).toNativeUtf8();
    final nativeKeyPairs = jsonEncode(keyPairs).toNativeUtf8();

    final recvPort = ReceivePort("generate_signatures");
    final sendPort = recvPort.sendPort;
    _generateSignatures(sendPort.nativePort, nativeData, nativeKeyPairs);
    return processFutureJson(
        jsonListConstructor<TypedSignature>(TypedSignature.fromJson),
        recvPort.first);
  }

  @override
  Timestamp now() {
    final ts = _now();
    return Timestamp(value: BigInt.from(ts));
  }

  @override
  Future<TypedKeyPair> generateKeyPair(CryptoKind kind) {
    final recvPort = ReceivePort("generate_key_pair");
    final sendPort = recvPort.sendPort;
    _generateKeyPair(sendPort.nativePort, kind);
    return processFutureJson(TypedKeyPair.fromJson, recvPort.first);
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
