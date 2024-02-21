// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'routing_context.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

_$DHTSchemaDFLT _$$DHTSchemaDFLTFromJson(Map<String, dynamic> json) =>
    _$DHTSchemaDFLT(
      oCnt: json['o_cnt'] as int,
      $type: json['kind'] as String?,
    );

Map<String, dynamic> _$$DHTSchemaDFLTToJson(_$DHTSchemaDFLT instance) =>
    <String, dynamic>{
      'o_cnt': instance.oCnt,
      'kind': instance.$type,
    };

_$DHTSchemaSMPL _$$DHTSchemaSMPLFromJson(Map<String, dynamic> json) =>
    _$DHTSchemaSMPL(
      oCnt: json['o_cnt'] as int,
      members: (json['members'] as List<dynamic>)
          .map(DHTSchemaMember.fromJson)
          .toList(),
      $type: json['kind'] as String?,
    );

Map<String, dynamic> _$$DHTSchemaSMPLToJson(_$DHTSchemaSMPL instance) =>
    <String, dynamic>{
      'o_cnt': instance.oCnt,
      'members': instance.members.map((e) => e.toJson()).toList(),
      'kind': instance.$type,
    };

_$_DHTSchemaMember _$$_DHTSchemaMemberFromJson(Map<String, dynamic> json) =>
    _$_DHTSchemaMember(
      mKey: FixedEncodedString43.fromJson(json['m_key']),
      mCnt: json['m_cnt'] as int,
    );

Map<String, dynamic> _$$_DHTSchemaMemberToJson(_$_DHTSchemaMember instance) =>
    <String, dynamic>{
      'm_key': instance.mKey.toJson(),
      'm_cnt': instance.mCnt,
    };

_$_DHTRecordDescriptor _$$_DHTRecordDescriptorFromJson(
        Map<String, dynamic> json) =>
    _$_DHTRecordDescriptor(
      key: Typed<FixedEncodedString43>.fromJson(json['key']),
      owner: FixedEncodedString43.fromJson(json['owner']),
      schema: DHTSchema.fromJson(json['schema']),
      ownerSecret: json['owner_secret'] == null
          ? null
          : FixedEncodedString43.fromJson(json['owner_secret']),
    );

Map<String, dynamic> _$$_DHTRecordDescriptorToJson(
        _$_DHTRecordDescriptor instance) =>
    <String, dynamic>{
      'key': instance.key.toJson(),
      'owner': instance.owner.toJson(),
      'schema': instance.schema.toJson(),
      'owner_secret': instance.ownerSecret?.toJson(),
    };

_$_ValueSubkeyRange _$$_ValueSubkeyRangeFromJson(Map<String, dynamic> json) =>
    _$_ValueSubkeyRange(
      low: json['low'] as int,
      high: json['high'] as int,
    );

Map<String, dynamic> _$$_ValueSubkeyRangeToJson(_$_ValueSubkeyRange instance) =>
    <String, dynamic>{
      'low': instance.low,
      'high': instance.high,
    };

_$_ValueData _$$_ValueDataFromJson(Map<String, dynamic> json) => _$_ValueData(
      seq: json['seq'] as int,
      data: const Uint8ListJsonConverter.jsIsArray().fromJson(json['data']),
      writer: FixedEncodedString43.fromJson(json['writer']),
    );

Map<String, dynamic> _$$_ValueDataToJson(_$_ValueData instance) =>
    <String, dynamic>{
      'seq': instance.seq,
      'data': const Uint8ListJsonConverter.jsIsArray().toJson(instance.data),
      'writer': instance.writer.toJson(),
    };

_$_SafetySpec _$$_SafetySpecFromJson(Map<String, dynamic> json) =>
    _$_SafetySpec(
      hopCount: json['hop_count'] as int,
      stability: Stability.fromJson(json['stability']),
      sequencing: Sequencing.fromJson(json['sequencing']),
      preferredRoute: json['preferred_route'] as String?,
    );

Map<String, dynamic> _$$_SafetySpecToJson(_$_SafetySpec instance) =>
    <String, dynamic>{
      'hop_count': instance.hopCount,
      'stability': instance.stability.toJson(),
      'sequencing': instance.sequencing.toJson(),
      'preferred_route': instance.preferredRoute,
    };

_$_RouteBlob _$$_RouteBlobFromJson(Map<String, dynamic> json) => _$_RouteBlob(
      routeId: json['route_id'] as String,
      blob: const Uint8ListJsonConverter().fromJson(json['blob']),
    );

Map<String, dynamic> _$$_RouteBlobToJson(_$_RouteBlob instance) =>
    <String, dynamic>{
      'route_id': instance.routeId,
      'blob': const Uint8ListJsonConverter().toJson(instance.blob),
    };
