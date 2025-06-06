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

List<T> Function(dynamic) jsonListConstructor<T>(
        T Function(dynamic) jsonConstructor) =>
    (dynamic j) => (j as List<dynamic>).map(jsonConstructor).toList();
List<T>? Function(dynamic) optJsonListConstructor<T>(
        T Function(dynamic) jsonConstructor) =>
    (dynamic j) =>
        j == null ? null : (j as List<dynamic>).map(jsonConstructor).toList();

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
  factory Timestamp.zero() => Timestamp(value: BigInt.zero);
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

  bool operator <(Timestamp other) => compareTo(other) < 0;
  bool operator <=(Timestamp other) => compareTo(other) <= 0;
  bool operator >(Timestamp other) => compareTo(other) > 0;
  bool operator >=(Timestamp other) => compareTo(other) >= 0;

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
  factory TimestampDuration.fromMillis(int millis) =>
      TimestampDuration(value: BigInt.from(millis) * BigInt.from(1000));
  factory TimestampDuration.fromDuration(Duration d) => TimestampDuration(
      value: BigInt.from(d.inSeconds) * BigInt.from(1000000) +
          BigInt.from(d.inMicroseconds % 1000000));
  factory TimestampDuration.fromString(String s) =>
      TimestampDuration(value: BigInt.parse(s));
  factory TimestampDuration.fromJson(dynamic json) =>
      TimestampDuration.fromString(json as String);
  final BigInt value;
  @override
  List<Object> get props => [value];

  @override
  int compareTo(TimestampDuration other) => value.compareTo(other.value);

  bool operator <(TimestampDuration other) => compareTo(other) < 0;
  bool operator <=(TimestampDuration other) => compareTo(other) <= 0;
  bool operator >(TimestampDuration other) => compareTo(other) > 0;
  bool operator >=(TimestampDuration other) => compareTo(other) >= 0;

  @override
  String toString() {
    final biDay = BigInt.from(1000000) * BigInt.from(60 * 60 * 24);
    final biHour = BigInt.from(1000000) * BigInt.from(60 * 60);
    final biMin = BigInt.from(1000000) * BigInt.from(60);
    final biSec = BigInt.from(1000000);
    final biMsec = BigInt.from(1000);

    final days = (value ~/ biDay).toInt();
    final dvalue = value % biDay;
    final hours = (dvalue ~/ biHour).toInt();
    final hvalue = dvalue % biHour;
    final mins = (hvalue ~/ biMin).toInt();
    final mvalue = hvalue % biMin;
    final secs = (mvalue ~/ biSec).toInt();
    final svalue = mvalue % biSec;
    final msecs = (svalue ~/ biMsec).toInt();
    final uvalue = svalue % biMsec;

    if (days == 0 && hours == 0 && mins == 0 && secs == 0) {
      // microseconds format
      return '$msecs.${uvalue.toString().padLeft(3, '0')}ms';
    }
    var out = '';
    if (days != 0) {
      out += '${days}d';
    }
    if (hours != 0) {
      out += '${hours}h';
    }
    if (mins != 0) {
      out += '${mins}m';
    }
    return '$out$secs.${msecs.toString().padLeft(3, '0')}s';
  }

  String toJson() => value.toString();
  Int64 toInt64() => Int64.fromInts(
      (value >> 32).toUnsigned(32).toInt(), value.toUnsigned(32).toInt());

  double toMillis() => value / BigInt.from(1000);
  double toSecs() => value / BigInt.from(1000000);
  BigInt toMicros() => value;
}

//////////////////////////////////////
/// Veilid singleton factory

abstract class Veilid {
  static Veilid instance = getVeilid();

  void initializeVeilidCore(Map<String, dynamic> platformConfigJson);
  void changeLogLevel(String layer, VeilidConfigLogLevel logLevel);
  void changeLogIgnore(String layer, List<String> changes);
  Future<Stream<VeilidUpdate>> startupVeilidCore(VeilidConfig config);
  Future<VeilidState> getVeilidState();
  Future<bool> isShutdown();
  Future<void> attach();
  Future<void> detach();
  Future<void> shutdownVeilidCore();

  // Crypto
  List<CryptoKind> validCryptoKinds();
  Future<VeilidCryptoSystem> getCryptoSystem(CryptoKind kind);
  Future<VeilidCryptoSystem> bestCryptoSystem();
  Future<List<TypedKey>?> verifySignatures(List<TypedKey> publicKeys,
      Uint8List data, List<TypedSignature> signatures);
  Future<List<TypedSignature>> generateSignatures(
      Uint8List data, List<TypedKeyPair> keyPairs);
  Future<TypedKeyPair> generateKeyPair(CryptoKind kind);

  // Routing context
  Future<VeilidRoutingContext> routingContext();
  Future<VeilidRoutingContext> safeRoutingContext(
      {Stability stability = Stability.lowLatency,
      Sequencing sequencing = Sequencing.preferOrdered}) async {
    final rc = await routingContext();
    final originalSafety = await rc.safety() as SafetySelectionSafe;
    final safetySpec = originalSafety.safetySpec
        .copyWith(stability: stability, sequencing: sequencing);
    return rc.withSafety(SafetySelectionSafe(safetySpec: safetySpec),
        closeSelf: true);
  }

  Future<VeilidRoutingContext> unsafeRoutingContext(
          {Sequencing sequencing = Sequencing.preferOrdered}) async =>
      (await routingContext())
          .withSafety(SafetySelectionUnsafe(sequencing: sequencing));

  // Private route allocation
  Future<RouteBlob> newPrivateRoute();
  Future<RouteBlob> newCustomPrivateRoute(
      Stability stability, Sequencing sequencing);
  Future<String> importRemotePrivateRoute(Uint8List blob);
  Future<void> releasePrivateRoute(String routeId);

  // App calls
  Future<void> appCallReply(String callId, Uint8List message);

  // TableStore
  Future<VeilidTableDB> openTableDB(String name, int columnCount);
  Future<bool> deleteTableDB(String name);

  // Misc
  Timestamp now();
  String veilidVersionString();
  VeilidVersion veilidVersion();
  String defaultVeilidConfig();
  Future<String> debug(String command);

  /// Return the features that were enabled when veilid-core was built.
  List<String> veilidFeatures();
}
