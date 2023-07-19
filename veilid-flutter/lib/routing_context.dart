import 'dart:async';
import 'dart:typed_data';

import 'package:change_case/change_case.dart';
import 'package:equatable/equatable.dart';
import 'package:freezed_annotation/freezed_annotation.dart';

import 'veilid_encoding.dart';
import 'veilid.dart';

part 'routing_context.freezed.dart';
part 'routing_context.g.dart';

//////////////////////////////////////

extension ValidateDFLT on DHTSchemaDFLT {
  bool validate() {
    if (oCnt > 65535) {
      return false;
    }
    if (oCnt <= 0) {
      return false;
    }
    return true;
  }
}

extension ValidateSMPL on DHTSchemaSMPL {
  bool validate() {
    final totalsv = members.fold(0, (acc, v) => (acc + v.mCnt)) + oCnt;
    if (totalsv > 65535) {
      return false;
    }
    if (totalsv <= 0) {
      return false;
    }
    return true;
  }
}

//////////////////////////////////////
/// DHT Schema

@Freezed(unionKey: 'kind', unionValueCase: FreezedUnionCase.pascal)
sealed class DHTSchema with _$DHTSchema {
  @FreezedUnionValue('DFLT')
  const factory DHTSchema.dflt({required int oCnt}) = DHTSchemaDFLT;

  @FreezedUnionValue('SMPL')
  const factory DHTSchema.smpl(
      {required int oCnt,
      required List<DHTSchemaMember> members}) = DHTSchemaSMPL;

  factory DHTSchema.fromJson(Map<String, dynamic> json) =>
      _$DHTSchemaFromJson(json);
}

const DHTSchema defaultDHTSchema = DHTSchema.dflt(oCnt: 1);

@freezed
class DHTSchemaMember with _$DHTSchemaMember {
  @Assert('mCnt > 0 && mCnt <= 65535', 'value out of range')
  const factory DHTSchemaMember({
    required PublicKey mKey,
    required int mCnt,
  }) = _DHTSchemaMember;

  factory DHTSchemaMember.fromJson(Map<String, dynamic> json) =>
      _$DHTSchemaMemberFromJson(json);
}

//////////////////////////////////////
/// DHTRecordDescriptor

@freezed
class DHTRecordDescriptor with _$DHTRecordDescriptor {
  const factory DHTRecordDescriptor({
    required TypedKey key,
    required PublicKey owner,
    PublicKey? ownerSecret,
    required DHTSchema schema,
  }) = _DHTRecordDescriptor;
  factory DHTRecordDescriptor.fromJson(Map<String, dynamic> json) =>
      _$DHTRecordDescriptorFromJson(json);
}

//////////////////////////////////////
/// ValueSubkeyRange

@freezed
class ValueSubkeyRange with _$ValueSubkeyRange {
  @Assert('low < 0 || low > high', 'low out of range')
  @Assert('high < 0', 'high out of range')
  const factory ValueSubkeyRange({
    required int low,
    required int high,
  }) = _ValueSubkeyRange;

  factory ValueSubkeyRange.fromJson(Map<String, dynamic> json) =>
      _$ValueSubkeyRangeFromJson(json);
}

//////////////////////////////////////
/// ValueData

@freezed
class ValueData with _$ValueData {
  @Assert('seq >= 0', 'seq out of range')
  const factory ValueData({
    required int seq,
    @Uint8ListJsonConverter() required Uint8List data,
    required PublicKey writer,
  }) = _ValueData;

  factory ValueData.fromJson(Map<String, dynamic> json) =>
      _$ValueDataFromJson(json);
}

//////////////////////////////////////
/// Stability

enum Stability {
  lowLatency,
  reliable;

  String toJson() => name.toPascalCase();
  factory Stability.fromJson(String j) =>
      Stability.values.byName(j.toCamelCase());
}

//////////////////////////////////////
/// Sequencing

enum Sequencing {
  noPreference,
  preferOrdered,
  ensureOrdered;

  String toJson() => name.toPascalCase();
  factory Sequencing.fromJson(String j) =>
      Sequencing.values.byName(j.toCamelCase());
}

//////////////////////////////////////
/// SafetySelection

@immutable
abstract class SafetySelection extends Equatable {
  factory SafetySelection.fromJson(Map<String, dynamic> json) {
    if (json.containsKey("Unsafe")) {
      return SafetySelectionUnsafe(
          sequencing: Sequencing.fromJson(json["Unsafe"]));
    } else if (json.containsKey("Safe")) {
      return SafetySelectionSafe(safetySpec: SafetySpec.fromJson(json["Safe"]));
    } else {
      throw const VeilidAPIExceptionInternal("Invalid SafetySelection");
    }
  }
  Map<String, dynamic> toJson();
}

@immutable
class SafetySelectionUnsafe implements SafetySelection {
  final Sequencing sequencing;
  @override
  List<Object> get props => [sequencing];
  @override
  bool? get stringify => null;

  //
  const SafetySelectionUnsafe({
    required this.sequencing,
  });

  @override
  Map<String, dynamic> toJson() {
    return {'Unsafe': sequencing.toJson()};
  }
}

@immutable
class SafetySelectionSafe implements SafetySelection {
  final SafetySpec safetySpec;
  @override
  List<Object> get props => [safetySpec];
  @override
  bool? get stringify => null;

  //
  const SafetySelectionSafe({
    required this.safetySpec,
  });

  @override
  Map<String, dynamic> toJson() {
    return {'Safe': safetySpec.toJson()};
  }
}

/// Options for safety routes (sender privacy)
@freezed
class SafetySpec with _$SafetySpec {
  const factory SafetySpec({
    String? preferredRoute,
    required int hopCount,
    required Stability stability,
    required Sequencing sequencing,
  }) = _SafetySpec;

  factory SafetySpec.fromJson(Map<String, dynamic> json) =>
      _$SafetySpecFromJson(json);
}

//////////////////////////////////////
/// RouteBlob
@freezed
class RouteBlob with _$RouteBlob {
  const factory RouteBlob(
      {required String routeId,
      @Uint8ListJsonConverter() required Uint8List blob}) = _RouteBlob;
  factory RouteBlob.fromJson(Map<String, dynamic> json) =>
      _$RouteBlobFromJson(json);
}

//////////////////////////////////////
/// VeilidRoutingContext

abstract class VeilidRoutingContext {
  void close();

  // Modifiers
  VeilidRoutingContext withPrivacy();
  VeilidRoutingContext withCustomPrivacy(SafetySelection safetySelection);
  VeilidRoutingContext withSequencing(Sequencing sequencing);

  // App call/message
  Future<Uint8List> appCall(String target, Uint8List request);
  Future<void> appMessage(String target, Uint8List message);

  // DHT Operations
  Future<DHTRecordDescriptor> createDHTRecord(DHTSchema schema,
      {CryptoKind kind = 0});
  Future<DHTRecordDescriptor> openDHTRecord(TypedKey key, KeyPair? writer);
  Future<void> closeDHTRecord(TypedKey key);
  Future<void> deleteDHTRecord(TypedKey key);
  Future<ValueData?> getDHTValue(TypedKey key, int subkey, bool forceRefresh);
  Future<ValueData?> setDHTValue(TypedKey key, int subkey, Uint8List data);
  Future<Timestamp> watchDHTValues(TypedKey key, List<ValueSubkeyRange> subkeys,
      Timestamp expiration, int count);
  Future<bool> cancelDHTWatch(TypedKey key, List<ValueSubkeyRange> subkeys);
}
