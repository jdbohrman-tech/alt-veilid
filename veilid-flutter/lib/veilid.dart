import 'dart:async';
import 'dart:typed_data';
import 'dart:convert';

import 'package:change_case/change_case.dart';

import 'veilid_stub.dart'
    if (dart.library.io) 'veilid_ffi.dart'
    if (dart.library.js) 'veilid_js.dart';

import 'veilid_encoding.dart';

//////////////////////////////////////////////////////////

import 'routing_context.dart';
import 'veilid_config.dart';
import 'veilid_crypto.dart';
import 'veilid_table_db.dart';
import 'veilid_api_exception.dart';
import 'veilid_state.dart';

export 'default_config.dart';
export 'routing_context.dart';
export 'veilid_config.dart';
export 'veilid_crypto.dart';
export 'veilid_table_db.dart';
export 'veilid_api_exception.dart';
export 'veilid_state.dart';
export 'veilid.dart';

//////////////////////////////////////
/// JSON Encode Helper

Object? veilidApiToEncodable(Object? value) {
  if (value == null) {
    return value;
  }
  switch (value.runtimeType) {
    case AttachmentState:
      return (value as AttachmentState).json;
    case VeilidLogLevel:
      return (value as VeilidLogLevel).json;
    case VeilidConfigLogLevel:
      return (value as VeilidConfigLogLevel).json;
  }
  throw UnsupportedError('Cannot convert to JSON: $value');
}

//////////////////////////////////////
/// VeilidVersion

class VeilidVersion {
  final int major;
  final int minor;
  final int patch;

  VeilidVersion(this.major, this.minor, this.patch);
}

//////////////////////////////////////
/// Timestamp
class Timestamp {
  final BigInt value;
  Timestamp({required this.value});

  @override
  String toString() {
    return value.toString();
  }

  Timestamp.fromString(String s) : value = BigInt.parse(s);

  Timestamp.fromJson(dynamic json) : this.fromString(json as String);

  String get json {
    return toString();
  }

  TimestampDuration diff(Timestamp other) {
    return TimestampDuration(value: value - other.value);
  }

  Timestamp offset(TimestampDuration dur) {
    return Timestamp(value: value + dur.value);
  }
}

class TimestampDuration {
  final BigInt value;
  TimestampDuration({required this.value});

  @override
  String toString() {
    return value.toString();
  }

  TimestampDuration.fromString(String s) : value = BigInt.parse(s);

  TimestampDuration.fromJson(dynamic json) : this.fromString(json as String);

  String get json {
    return toString();
  }

  int toMillis() {
    return (value ~/ BigInt.from(1000)).toInt();
  }

  BigInt toMicros(Timestamp other) {
    return value;
  }
}

//////////////////////////////////////
/// Veilid singleton factory

abstract class Veilid {
  static late Veilid instance = getVeilid();

  void initializeVeilidCore(Map<String, dynamic> platformConfigJson);
  void changeLogLevel(String layer, VeilidConfigLogLevel logLevel);
  Future<Stream<VeilidUpdate>> startupVeilidCore(VeilidConfig config);
  Future<VeilidState> getVeilidState();
  Future<void> attach();
  Future<void> detach();
  Future<void> shutdownVeilidCore();

  // Crypto
  List<CryptoKind> validCryptoKinds();
  VeilidCryptoSystem getCryptoSystem(CryptoKind kind);
  VeilidCryptoSystem bestCryptoSystem();
  List<TypedKey> verifySignatures(
      List<TypedKey> nodeIds, Uint8List data, List<TypedSignature> signatures);
  List<TypedSignature> generateSignatures(
      Uint8List data, List<TypedKeyPair> keyPairs);
  TypedKeyPair generateKeyPair(CryptoKind kind);

  // Routing context
  Future<VeilidRoutingContext> routingContext();

  // Private route allocation
  Future<RouteBlob> newPrivateRoute();
  Future<RouteBlob> newCustomPrivateRoute(
      Stability stability, Sequencing sequencing);
  Future<String> importRemotePrivateRoute(Uint8List blob);
  Future<void> releasePrivateRoute(String key);

  // App calls
  Future<void> appCallReply(String id, Uint8List message);

  // TableStore
  Future<VeilidTableDB> openTableDB(String name, int columnCount);
  Future<bool> deleteTableDB(String name);

  // Misc
  Timestamp now();
  String veilidVersionString();
  VeilidVersion veilidVersion();
  Future<String> debug(String command);
}
