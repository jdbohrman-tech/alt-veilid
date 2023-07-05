// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'veilid_state.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

_$_LatencyStats _$$_LatencyStatsFromJson(Map<String, dynamic> json) =>
    _$_LatencyStats(
      fastest: TimestampDuration.fromJson(json['fastest']),
      average: TimestampDuration.fromJson(json['average']),
      slowest: TimestampDuration.fromJson(json['slowest']),
    );

Map<String, dynamic> _$$_LatencyStatsToJson(_$_LatencyStats instance) =>
    <String, dynamic>{
      'fastest': instance.fastest.toJson(),
      'average': instance.average.toJson(),
      'slowest': instance.slowest.toJson(),
    };

_$_TransferStats _$$_TransferStatsFromJson(Map<String, dynamic> json) =>
    _$_TransferStats(
      total: BigInt.parse(json['total'] as String),
      maximum: BigInt.parse(json['maximum'] as String),
      average: BigInt.parse(json['average'] as String),
      minimum: BigInt.parse(json['minimum'] as String),
    );

Map<String, dynamic> _$$_TransferStatsToJson(_$_TransferStats instance) =>
    <String, dynamic>{
      'total': instance.total.toString(),
      'maximum': instance.maximum.toString(),
      'average': instance.average.toString(),
      'minimum': instance.minimum.toString(),
    };

_$_TransferStatsDownUp _$$_TransferStatsDownUpFromJson(
        Map<String, dynamic> json) =>
    _$_TransferStatsDownUp(
      down: TransferStats.fromJson(json['down'] as Map<String, dynamic>),
      up: TransferStats.fromJson(json['up'] as Map<String, dynamic>),
    );

Map<String, dynamic> _$$_TransferStatsDownUpToJson(
        _$_TransferStatsDownUp instance) =>
    <String, dynamic>{
      'down': instance.down.toJson(),
      'up': instance.up.toJson(),
    };

_$_RPCStats _$$_RPCStatsFromJson(Map<String, dynamic> json) => _$_RPCStats(
      messagesSent: json['messages_sent'] as int,
      messagesRcvd: json['messages_rcvd'] as int,
      questionsInFlight: json['questions_in_flight'] as int,
      lastQuestion: json['last_question'] == null
          ? null
          : Timestamp.fromJson(json['last_question']),
      lastSeenTs: json['last_seen_ts'] == null
          ? null
          : Timestamp.fromJson(json['last_seen_ts']),
      firstConsecutiveSeenTs: json['first_consecutive_seen_ts'] == null
          ? null
          : Timestamp.fromJson(json['first_consecutive_seen_ts']),
      recentLostAnswers: json['recent_lost_answers'] as int,
      failedToSend: json['failed_to_send'] as int,
    );

Map<String, dynamic> _$$_RPCStatsToJson(_$_RPCStats instance) =>
    <String, dynamic>{
      'messages_sent': instance.messagesSent,
      'messages_rcvd': instance.messagesRcvd,
      'questions_in_flight': instance.questionsInFlight,
      'last_question': instance.lastQuestion?.toJson(),
      'last_seen_ts': instance.lastSeenTs?.toJson(),
      'first_consecutive_seen_ts': instance.firstConsecutiveSeenTs?.toJson(),
      'recent_lost_answers': instance.recentLostAnswers,
      'failed_to_send': instance.failedToSend,
    };

_$_PeerStats _$$_PeerStatsFromJson(Map<String, dynamic> json) => _$_PeerStats(
      timeAdded: Timestamp.fromJson(json['time_added']),
      rpcStats: RPCStats.fromJson(json['rpc_stats'] as Map<String, dynamic>),
      latency: json['latency'] == null
          ? null
          : LatencyStats.fromJson(json['latency'] as Map<String, dynamic>),
      transfer: TransferStatsDownUp.fromJson(
          json['transfer'] as Map<String, dynamic>),
    );

Map<String, dynamic> _$$_PeerStatsToJson(_$_PeerStats instance) =>
    <String, dynamic>{
      'time_added': instance.timeAdded.toJson(),
      'rpc_stats': instance.rpcStats.toJson(),
      'latency': instance.latency?.toJson(),
      'transfer': instance.transfer.toJson(),
    };

_$_PeerTableData _$$_PeerTableDataFromJson(Map<String, dynamic> json) =>
    _$_PeerTableData(
      nodeIds: (json['node_ids'] as List<dynamic>)
          .map(Typed<FixedEncodedString43>.fromJson)
          .toList(),
      peerAddress: json['peer_address'] as String,
      peerStats: PeerStats.fromJson(json['peer_stats'] as Map<String, dynamic>),
    );

Map<String, dynamic> _$$_PeerTableDataToJson(_$_PeerTableData instance) =>
    <String, dynamic>{
      'node_ids': instance.nodeIds.map((e) => e.toJson()).toList(),
      'peer_address': instance.peerAddress,
      'peer_stats': instance.peerStats.toJson(),
    };
