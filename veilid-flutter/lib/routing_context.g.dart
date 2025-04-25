// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'routing_context.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

DHTSchemaDFLT _$DHTSchemaDFLTFromJson(Map<String, dynamic> json) =>
    DHTSchemaDFLT(
      oCnt: (json['o_cnt'] as num).toInt(),
      $type: json['kind'] as String?,
    );

Map<String, dynamic> _$DHTSchemaDFLTToJson(DHTSchemaDFLT instance) =>
    <String, dynamic>{
      'o_cnt': instance.oCnt,
      'kind': instance.$type,
    };

DHTSchemaSMPL _$DHTSchemaSMPLFromJson(Map<String, dynamic> json) =>
    DHTSchemaSMPL(
      oCnt: (json['o_cnt'] as num).toInt(),
      members: (json['members'] as List<dynamic>)
          .map(DHTSchemaMember.fromJson)
          .toList(),
      $type: json['kind'] as String?,
    );

Map<String, dynamic> _$DHTSchemaSMPLToJson(DHTSchemaSMPL instance) =>
    <String, dynamic>{
      'o_cnt': instance.oCnt,
      'members': instance.members.map((e) => e.toJson()).toList(),
      'kind': instance.$type,
    };

_DHTSchemaMember _$DHTSchemaMemberFromJson(Map<String, dynamic> json) =>
    _DHTSchemaMember(
      mKey: FixedEncodedString43.fromJson(json['m_key']),
      mCnt: (json['m_cnt'] as num).toInt(),
    );

Map<String, dynamic> _$DHTSchemaMemberToJson(_DHTSchemaMember instance) =>
    <String, dynamic>{
      'm_key': instance.mKey.toJson(),
      'm_cnt': instance.mCnt,
    };

_DHTRecordDescriptor _$DHTRecordDescriptorFromJson(Map<String, dynamic> json) =>
    _DHTRecordDescriptor(
      key: Typed<FixedEncodedString43>.fromJson(json['key']),
      owner: FixedEncodedString43.fromJson(json['owner']),
      schema: DHTSchema.fromJson(json['schema']),
      ownerSecret: json['owner_secret'] == null
          ? null
          : FixedEncodedString43.fromJson(json['owner_secret']),
    );

Map<String, dynamic> _$DHTRecordDescriptorToJson(
        _DHTRecordDescriptor instance) =>
    <String, dynamic>{
      'key': instance.key.toJson(),
      'owner': instance.owner.toJson(),
      'schema': instance.schema.toJson(),
      'owner_secret': instance.ownerSecret?.toJson(),
    };

_ValueData _$ValueDataFromJson(Map<String, dynamic> json) => _ValueData(
      seq: (json['seq'] as num).toInt(),
      data: const Uint8ListJsonConverter.jsIsArray().fromJson(json['data']),
      writer: FixedEncodedString43.fromJson(json['writer']),
    );

Map<String, dynamic> _$ValueDataToJson(_ValueData instance) =>
    <String, dynamic>{
      'seq': instance.seq,
      'data': const Uint8ListJsonConverter.jsIsArray().toJson(instance.data),
      'writer': instance.writer.toJson(),
    };

_SafetySpec _$SafetySpecFromJson(Map<String, dynamic> json) => _SafetySpec(
      hopCount: (json['hop_count'] as num).toInt(),
      stability: Stability.fromJson(json['stability']),
      sequencing: Sequencing.fromJson(json['sequencing']),
      preferredRoute: json['preferred_route'] as String?,
    );

Map<String, dynamic> _$SafetySpecToJson(_SafetySpec instance) =>
    <String, dynamic>{
      'hop_count': instance.hopCount,
      'stability': instance.stability.toJson(),
      'sequencing': instance.sequencing.toJson(),
      'preferred_route': instance.preferredRoute,
    };

_RouteBlob _$RouteBlobFromJson(Map<String, dynamic> json) => _RouteBlob(
      routeId: json['route_id'] as String,
      blob: const Uint8ListJsonConverter().fromJson(json['blob']),
    );

Map<String, dynamic> _$RouteBlobToJson(_RouteBlob instance) =>
    <String, dynamic>{
      'route_id': instance.routeId,
      'blob': const Uint8ListJsonConverter().toJson(instance.blob),
    };

_DHTRecordReport _$DHTRecordReportFromJson(Map<String, dynamic> json) =>
    _DHTRecordReport(
      subkeys: (json['subkeys'] as List<dynamic>)
          .map(ValueSubkeyRange.fromJson)
          .toList(),
      offlineSubkeys: (json['offline_subkeys'] as List<dynamic>)
          .map(ValueSubkeyRange.fromJson)
          .toList(),
      localSeqs: (json['local_seqs'] as List<dynamic>)
          .map((e) => (e as num?)?.toInt())
          .toList(),
      networkSeqs: (json['network_seqs'] as List<dynamic>)
          .map((e) => (e as num?)?.toInt())
          .toList(),
    );

Map<String, dynamic> _$DHTRecordReportToJson(_DHTRecordReport instance) =>
    <String, dynamic>{
      'subkeys': instance.subkeys.map((e) => e.toJson()).toList(),
      'offline_subkeys':
          instance.offlineSubkeys.map((e) => e.toJson()).toList(),
      'local_seqs': instance.localSeqs,
      'network_seqs': instance.networkSeqs,
    };
