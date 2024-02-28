import 'dart:async';
import 'dart:typed_data';

import 'package:equatable/equatable.dart';
import 'package:fixnum/fixnum.dart';
import 'package:freezed_annotation/freezed_annotation.dart';

//////////////////////////////////////////////////////////

import 'routing_context.dart';
import 'veilid_config.dart';
import 'veilid_crypto.dart';
import 'veilid_state.dart';
import 'veilid_stub.dart'
    if (dart.library.io) 'veilid_ffi.dart'
    if (dart.library.js) 'veilid_js.dart';
import 'veilid_table_db.dart';

export 'default_config.dart';
export 'routing_context.dart';
export 'value_subkey_range.dart';
export 'veilid.dart';
export 'veilid_api_exception.dart';
export 'veilid_config.dart';
export 'veilid_crypto.dart';
export 'veilid_encoding.dart';
export 'veilid_state.dart';
export 'veilid_table_db.dart';

//////////////////////////////////////
/// JSON Encode Helper

Object? veilidApiToEncodable(Object? value) {
  if (value == null) {
    return value;
  }
  switch (value.runtimeType) {
    // case KeyPair:
    //   return (value as KeyPair).json;
  }
  throw UnsupportedError('Cannot convert to JSON: $value');
}

List<T> Function(dynamic) jsonListConstructor<T>(
        T Function(dynamic) jsonConstructor) =>
    (dynamic j) => (j as List<dynamic>).map(jsonConstructor).toList();

//////////////////////////////////////
/// VeilidVersion

@immutable
class VeilidVersion extends Equatable {
  const VeilidVersion(this.major, this.minor, this.patch);
  final int major;
  final int minor;
  final int patch;
  @override
  List<Object> get props => [major, minor, patch];
}

//////////////////////////////////////
/// Timestamp
@immutable
class Timestamp extends Equatable implements Comparable<Timestamp> {
  const Timestamp({required this.value});
  factory Timestamp.fromInt64(Int64 i64) => Timestamp(
      value: (BigInt.from((i64 >> 32).toUnsigned(32).toInt()) << 32) |
          BigInt.from(i64.toUnsigned(32).toInt()));
  factory Timestamp.fromString(String s) => Timestamp(value: BigInt.parse(s));
  factory Timestamp.fromJson(dynamic json) =>
      Timestamp.fromString(json as String);
  final BigInt value;
  @override
  List<Object> get props => [value];

  @override
  int compareTo(Timestamp other) => value.compareTo(other.value);

  @override
  String toString() => value.toString();
  String toJson() => toString();
  Int64 toInt64() => Int64.fromInts(
      (value >> 32).toUnsigned(32).toInt(), value.toUnsigned(32).toInt());

  TimestampDuration diff(Timestamp other) =>
      TimestampDuration(value: value - other.value);

  Timestamp offset(TimestampDuration dur) =>
      Timestamp(value: value + dur.value);
}

@immutable
class TimestampDuration extends Equatable
    implements Comparable<TimestampDuration> {
  const TimestampDuration({required this.value});
  factory TimestampDuration.fromInt64(Int64 i64) => TimestampDuration(
      value: (BigInt.from((i64 >> 32).toUnsigned(32).toInt()) << 32) |
          BigInt.from(i64.toUnsigned(32).toInt()));
  factory TimestampDuration.fromString(String s) =>
      TimestampDuration(value: BigInt.parse(s));
  factory TimestampDuration.fromJson(dynamic json) =>
      TimestampDuration.fromString(json as String);
  final BigInt value;
  @override
  List<Object> get props => [value];

  @override
  int compareTo(TimestampDuration other) => value.compareTo(other.value);

  @override
  String toString() => value.toString();
  String toJson() => toString();
  Int64 toInt64() => Int64.fromInts(
      (value >> 32).toUnsigned(32).toInt(), value.toUnsigned(32).toInt());

  int toMillis() => (value ~/ BigInt.from(1000)).toInt();
  BigInt toMicros() => value;
}

//////////////////////////////////////
/// Veilid singleton factory

abstract class Veilid {
  static Veilid instance = getVeilid();

  void initializeVeilidCore(Map<String, dynamic> platformConfigJson);
  void changeLogLevel(String layer, VeilidConfigLogLevel logLevel);
  Future<Stream<VeilidUpdate>> startupVeilidCore(VeilidConfig config);
  Future<VeilidState> getVeilidState();
  Future<void> attach();
  Future<void> detach();
  Future<void> shutdownVeilidCore();

  // Crypto
  List<CryptoKind> validCryptoKinds();
  Future<VeilidCryptoSystem> getCryptoSystem(CryptoKind kind);
  Future<VeilidCryptoSystem> bestCryptoSystem();
  Future<List<TypedKey>> verifySignatures(
      List<TypedKey> nodeIds, Uint8List data, List<TypedSignature> signatures);
  Future<List<TypedSignature>> generateSignatures(
      Uint8List data, List<TypedKeyPair> keyPairs);
  Future<TypedKeyPair> generateKeyPair(CryptoKind kind);

  // Routing context
  Future<VeilidRoutingContext> routingContext();

  // Private route allocation
  Future<RouteBlob> newPrivateRoute();
  Future<RouteBlob> newCustomPrivateRoute(
      Stability stability, Sequencing sequencing);
  Future<String> importRemotePrivateRoute(Uint8List blob);
  Future<void> releasePrivateRoute(String key);

  // App calls
  Future<void> appCallReply(String callId, Uint8List message);

  // TableStore
  Future<VeilidTableDB> openTableDB(String name, int columnCount);
  Future<bool> deleteTableDB(String name);

  // Misc
  Timestamp now();
  String veilidVersionString();
  VeilidVersion veilidVersion();
  Future<String> debug(String command);
}
