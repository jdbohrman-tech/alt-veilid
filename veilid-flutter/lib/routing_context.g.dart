// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'routing_context.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

_$DHTSchemaDFLTImpl _$$DHTSchemaDFLTImplFromJson(Map<String, dynamic> json) =>
    _$DHTSchemaDFLTImpl(
      oCnt: json['o_cnt'] as int,
      $type: json['kind'] as String?,
    );

Map<String, dynamic> _$$DHTSchemaDFLTImplToJson(_$DHTSchemaDFLTImpl instance) =>
    <String, dynamic>{
      'o_cnt': instance.oCnt,
      'kind': instance.$type,
    };

_$DHTSchemaSMPLImpl _$$DHTSchemaSMPLImplFromJson(Map<String, dynamic> json) =>
    _$DHTSchemaSMPLImpl(
      oCnt: json['o_cnt'] as int,
      members: (json['members'] as List<dynamic>)
          .map(DHTSchemaMember.fromJson)
          .toList(),
      $type: json['kind'] as String?,
    );

Map<String, dynamic> _$$DHTSchemaSMPLImplToJson(_$DHTSchemaSMPLImpl instance) =>
    <String, dynamic>{
      'o_cnt': instance.oCnt,
      'members': instance.members.map((e) => e.toJson()).toList(),
      'kind': instance.$type,
    };

_$DHTSchemaMemberImpl _$$DHTSchemaMemberImplFromJson(
        Map<String, dynamic> json) =>
    _$DHTSchemaMemberImpl(
      mKey: FixedEncodedString43.fromJson(json['m_key']),
      mCnt: json['m_cnt'] as int,
    );

Map<String, dynamic> _$$DHTSchemaMemberImplToJson(
        _$DHTSchemaMemberImpl instance) =>
    <String, dynamic>{
      'm_key': instance.mKey.toJson(),
      'm_cnt': instance.mCnt,
    };

_$DHTRecordDescriptorImpl _$$DHTRecordDescriptorImplFromJson(
        Map<String, dynamic> json) =>
    _$DHTRecordDescriptorImpl(
      key: Typed<FixedEncodedString43>.fromJson(json['key']),
      owner: FixedEncodedString43.fromJson(json['owner']),
      schema: DHTSchema.fromJson(json['schema']),
      ownerSecret: json['owner_secret'] == null
          ? null
          : FixedEncodedString43.fromJson(json['owner_secret']),
    );

Map<String, dynamic> _$$DHTRecordDescriptorImplToJson(
        _$DHTRecordDescriptorImpl instance) =>
    <String, dynamic>{
      'key': instance.key.toJson(),
      'owner': instance.owner.toJson(),
      'schema': instance.schema.toJson(),
      'owner_secret': instance.ownerSecret?.toJson(),
    };

_$ValueDataImpl _$$ValueDataImplFromJson(Map<String, dynamic> json) =>
    _$ValueDataImpl(
      seq: json['seq'] as int,
      data: const Uint8ListJsonConverter.jsIsArray().fromJson(json['data']),
      writer: FixedEncodedString43.fromJson(json['writer']),
    );

Map<String, dynamic> _$$ValueDataImplToJson(_$ValueDataImpl instance) =>
    <String, dynamic>{
      'seq': instance.seq,
      'data': const Uint8ListJsonConverter.jsIsArray().toJson(instance.data),
      'writer': instance.writer.toJson(),
    };

_$SafetySpecImpl _$$SafetySpecImplFromJson(Map<String, dynamic> json) =>
    _$SafetySpecImpl(
      hopCount: json['hop_count'] as int,
      stability: Stability.fromJson(json['stability']),
      sequencing: Sequencing.fromJson(json['sequencing']),
      preferredRoute: json['preferred_route'] as String?,
    );

Map<String, dynamic> _$$SafetySpecImplToJson(_$SafetySpecImpl instance) =>
    <String, dynamic>{
      'hop_count': instance.hopCount,
      'stability': instance.stability.toJson(),
      'sequencing': instance.sequencing.toJson(),
      'preferred_route': instance.preferredRoute,
    };

_$RouteBlobImpl _$$RouteBlobImplFromJson(Map<String, dynamic> json) =>
    _$RouteBlobImpl(
      routeId: json['route_id'] as String,
      blob: const Uint8ListJsonConverter().fromJson(json['blob']),
    );

Map<String, dynamic> _$$RouteBlobImplToJson(_$RouteBlobImpl instance) =>
    <String, dynamic>{
      'route_id': instance.routeId,
      'blob': const Uint8ListJsonConverter().toJson(instance.blob),
    };

_$DHTRecordReportImpl _$$DHTRecordReportImplFromJson(
        Map<String, dynamic> json) =>
    _$DHTRecordReportImpl(
      subkeys: (json['subkeys'] as List<dynamic>)
          .map(ValueSubkeyRange.fromJson)
          .toList(),
      localSeqs:
          (json['local_seqs'] as List<dynamic>).map((e) => e as int).toList(),
      networkSeqs:
          (json['network_seqs'] as List<dynamic>).map((e) => e as int).toList(),
    );

Map<String, dynamic> _$$DHTRecordReportImplToJson(
        _$DHTRecordReportImpl instance) =>
    <String, dynamic>{
      'subkeys': instance.subkeys.map((e) => e.toJson()).toList(),
      'local_seqs': instance.localSeqs,
      'network_seqs': instance.networkSeqs,
    };
