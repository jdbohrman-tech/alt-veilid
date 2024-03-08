// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'veilid_state.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

_$LatencyStatsImpl _$$LatencyStatsImplFromJson(Map<String, dynamic> json) =>
    _$LatencyStatsImpl(
      fastest: TimestampDuration.fromJson(json['fastest']),
      average: TimestampDuration.fromJson(json['average']),
      slowest: TimestampDuration.fromJson(json['slowest']),
    );

Map<String, dynamic> _$$LatencyStatsImplToJson(_$LatencyStatsImpl instance) =>
    <String, dynamic>{
      'fastest': instance.fastest.toJson(),
      'average': instance.average.toJson(),
      'slowest': instance.slowest.toJson(),
    };

_$TransferStatsImpl _$$TransferStatsImplFromJson(Map<String, dynamic> json) =>
    _$TransferStatsImpl(
      total: BigInt.parse(json['total'] as String),
      maximum: BigInt.parse(json['maximum'] as String),
      average: BigInt.parse(json['average'] as String),
      minimum: BigInt.parse(json['minimum'] as String),
    );

Map<String, dynamic> _$$TransferStatsImplToJson(_$TransferStatsImpl instance) =>
    <String, dynamic>{
      'total': instance.total.toString(),
      'maximum': instance.maximum.toString(),
      'average': instance.average.toString(),
      'minimum': instance.minimum.toString(),
    };

_$TransferStatsDownUpImpl _$$TransferStatsDownUpImplFromJson(
        Map<String, dynamic> json) =>
    _$TransferStatsDownUpImpl(
      down: TransferStats.fromJson(json['down']),
      up: TransferStats.fromJson(json['up']),
    );

Map<String, dynamic> _$$TransferStatsDownUpImplToJson(
        _$TransferStatsDownUpImpl instance) =>
    <String, dynamic>{
      'down': instance.down.toJson(),
      'up': instance.up.toJson(),
    };

_$RPCStatsImpl _$$RPCStatsImplFromJson(Map<String, dynamic> json) =>
    _$RPCStatsImpl(
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

Map<String, dynamic> _$$RPCStatsImplToJson(_$RPCStatsImpl instance) =>
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

_$PeerStatsImpl _$$PeerStatsImplFromJson(Map<String, dynamic> json) =>
    _$PeerStatsImpl(
      timeAdded: Timestamp.fromJson(json['time_added']),
      rpcStats: RPCStats.fromJson(json['rpc_stats']),
      transfer: TransferStatsDownUp.fromJson(json['transfer']),
      latency: json['latency'] == null
          ? null
          : LatencyStats.fromJson(json['latency']),
    );

Map<String, dynamic> _$$PeerStatsImplToJson(_$PeerStatsImpl instance) =>
    <String, dynamic>{
      'time_added': instance.timeAdded.toJson(),
      'rpc_stats': instance.rpcStats.toJson(),
      'transfer': instance.transfer.toJson(),
      'latency': instance.latency?.toJson(),
    };

_$PeerTableDataImpl _$$PeerTableDataImplFromJson(Map<String, dynamic> json) =>
    _$PeerTableDataImpl(
      nodeIds: (json['node_ids'] as List<dynamic>)
          .map(Typed<FixedEncodedString43>.fromJson)
          .toList(),
      peerAddress: json['peer_address'] as String,
      peerStats: PeerStats.fromJson(json['peer_stats']),
    );

Map<String, dynamic> _$$PeerTableDataImplToJson(_$PeerTableDataImpl instance) =>
    <String, dynamic>{
      'node_ids': instance.nodeIds.map((e) => e.toJson()).toList(),
      'peer_address': instance.peerAddress,
      'peer_stats': instance.peerStats.toJson(),
    };

_$VeilidLogImpl _$$VeilidLogImplFromJson(Map<String, dynamic> json) =>
    _$VeilidLogImpl(
      logLevel: VeilidLogLevel.fromJson(json['log_level']),
      message: json['message'] as String,
      backtrace: json['backtrace'] as String?,
      $type: json['kind'] as String?,
    );

Map<String, dynamic> _$$VeilidLogImplToJson(_$VeilidLogImpl instance) =>
    <String, dynamic>{
      'log_level': instance.logLevel.toJson(),
      'message': instance.message,
      'backtrace': instance.backtrace,
      'kind': instance.$type,
    };

_$VeilidAppMessageImpl _$$VeilidAppMessageImplFromJson(
        Map<String, dynamic> json) =>
    _$VeilidAppMessageImpl(
      message: const Uint8ListJsonConverter().fromJson(json['message']),
      sender: json['sender'] == null
          ? null
          : Typed<FixedEncodedString43>.fromJson(json['sender']),
      routeId: json['route_id'] as String?,
      $type: json['kind'] as String?,
    );

Map<String, dynamic> _$$VeilidAppMessageImplToJson(
        _$VeilidAppMessageImpl instance) =>
    <String, dynamic>{
      'message': const Uint8ListJsonConverter().toJson(instance.message),
      'sender': instance.sender?.toJson(),
      'route_id': instance.routeId,
      'kind': instance.$type,
    };

_$VeilidAppCallImpl _$$VeilidAppCallImplFromJson(Map<String, dynamic> json) =>
    _$VeilidAppCallImpl(
      message: const Uint8ListJsonConverter().fromJson(json['message']),
      callId: json['call_id'] as String,
      sender: json['sender'] == null
          ? null
          : Typed<FixedEncodedString43>.fromJson(json['sender']),
      routeId: json['route_id'] as String?,
      $type: json['kind'] as String?,
    );

Map<String, dynamic> _$$VeilidAppCallImplToJson(_$VeilidAppCallImpl instance) =>
    <String, dynamic>{
      'message': const Uint8ListJsonConverter().toJson(instance.message),
      'call_id': instance.callId,
      'sender': instance.sender?.toJson(),
      'route_id': instance.routeId,
      'kind': instance.$type,
    };

_$VeilidUpdateAttachmentImpl _$$VeilidUpdateAttachmentImplFromJson(
        Map<String, dynamic> json) =>
    _$VeilidUpdateAttachmentImpl(
      state: AttachmentState.fromJson(json['state']),
      publicInternetReady: json['public_internet_ready'] as bool,
      localNetworkReady: json['local_network_ready'] as bool,
      $type: json['kind'] as String?,
    );

Map<String, dynamic> _$$VeilidUpdateAttachmentImplToJson(
        _$VeilidUpdateAttachmentImpl instance) =>
    <String, dynamic>{
      'state': instance.state.toJson(),
      'public_internet_ready': instance.publicInternetReady,
      'local_network_ready': instance.localNetworkReady,
      'kind': instance.$type,
    };

_$VeilidUpdateNetworkImpl _$$VeilidUpdateNetworkImplFromJson(
        Map<String, dynamic> json) =>
    _$VeilidUpdateNetworkImpl(
      started: json['started'] as bool,
      bpsDown: BigInt.parse(json['bps_down'] as String),
      bpsUp: BigInt.parse(json['bps_up'] as String),
      peers:
          (json['peers'] as List<dynamic>).map(PeerTableData.fromJson).toList(),
      $type: json['kind'] as String?,
    );

Map<String, dynamic> _$$VeilidUpdateNetworkImplToJson(
        _$VeilidUpdateNetworkImpl instance) =>
    <String, dynamic>{
      'started': instance.started,
      'bps_down': instance.bpsDown.toString(),
      'bps_up': instance.bpsUp.toString(),
      'peers': instance.peers.map((e) => e.toJson()).toList(),
      'kind': instance.$type,
    };

_$VeilidUpdateConfigImpl _$$VeilidUpdateConfigImplFromJson(
        Map<String, dynamic> json) =>
    _$VeilidUpdateConfigImpl(
      config: VeilidConfig.fromJson(json['config']),
      $type: json['kind'] as String?,
    );

Map<String, dynamic> _$$VeilidUpdateConfigImplToJson(
        _$VeilidUpdateConfigImpl instance) =>
    <String, dynamic>{
      'config': instance.config.toJson(),
      'kind': instance.$type,
    };

_$VeilidUpdateRouteChangeImpl _$$VeilidUpdateRouteChangeImplFromJson(
        Map<String, dynamic> json) =>
    _$VeilidUpdateRouteChangeImpl(
      deadRoutes: (json['dead_routes'] as List<dynamic>)
          .map((e) => e as String)
          .toList(),
      deadRemoteRoutes: (json['dead_remote_routes'] as List<dynamic>)
          .map((e) => e as String)
          .toList(),
      $type: json['kind'] as String?,
    );

Map<String, dynamic> _$$VeilidUpdateRouteChangeImplToJson(
        _$VeilidUpdateRouteChangeImpl instance) =>
    <String, dynamic>{
      'dead_routes': instance.deadRoutes,
      'dead_remote_routes': instance.deadRemoteRoutes,
      'kind': instance.$type,
    };

_$VeilidUpdateValueChangeImpl _$$VeilidUpdateValueChangeImplFromJson(
        Map<String, dynamic> json) =>
    _$VeilidUpdateValueChangeImpl(
      key: Typed<FixedEncodedString43>.fromJson(json['key']),
      subkeys: (json['subkeys'] as List<dynamic>)
          .map(ValueSubkeyRange.fromJson)
          .toList(),
      count: json['count'] as int,
      value: ValueData.fromJson(json['value']),
      $type: json['kind'] as String?,
    );

Map<String, dynamic> _$$VeilidUpdateValueChangeImplToJson(
        _$VeilidUpdateValueChangeImpl instance) =>
    <String, dynamic>{
      'key': instance.key.toJson(),
      'subkeys': instance.subkeys.map((e) => e.toJson()).toList(),
      'count': instance.count,
      'value': instance.value.toJson(),
      'kind': instance.$type,
    };

_$VeilidStateAttachmentImpl _$$VeilidStateAttachmentImplFromJson(
        Map<String, dynamic> json) =>
    _$VeilidStateAttachmentImpl(
      state: AttachmentState.fromJson(json['state']),
      publicInternetReady: json['public_internet_ready'] as bool,
      localNetworkReady: json['local_network_ready'] as bool,
    );

Map<String, dynamic> _$$VeilidStateAttachmentImplToJson(
        _$VeilidStateAttachmentImpl instance) =>
    <String, dynamic>{
      'state': instance.state.toJson(),
      'public_internet_ready': instance.publicInternetReady,
      'local_network_ready': instance.localNetworkReady,
    };

_$VeilidStateNetworkImpl _$$VeilidStateNetworkImplFromJson(
        Map<String, dynamic> json) =>
    _$VeilidStateNetworkImpl(
      started: json['started'] as bool,
      bpsDown: BigInt.parse(json['bps_down'] as String),
      bpsUp: BigInt.parse(json['bps_up'] as String),
      peers:
          (json['peers'] as List<dynamic>).map(PeerTableData.fromJson).toList(),
    );

Map<String, dynamic> _$$VeilidStateNetworkImplToJson(
        _$VeilidStateNetworkImpl instance) =>
    <String, dynamic>{
      'started': instance.started,
      'bps_down': instance.bpsDown.toString(),
      'bps_up': instance.bpsUp.toString(),
      'peers': instance.peers.map((e) => e.toJson()).toList(),
    };

_$VeilidStateConfigImpl _$$VeilidStateConfigImplFromJson(
        Map<String, dynamic> json) =>
    _$VeilidStateConfigImpl(
      config: VeilidConfig.fromJson(json['config']),
    );

Map<String, dynamic> _$$VeilidStateConfigImplToJson(
        _$VeilidStateConfigImpl instance) =>
    <String, dynamic>{
      'config': instance.config.toJson(),
    };

_$VeilidStateImpl _$$VeilidStateImplFromJson(Map<String, dynamic> json) =>
    _$VeilidStateImpl(
      attachment: VeilidStateAttachment.fromJson(json['attachment']),
      network: VeilidStateNetwork.fromJson(json['network']),
      config: VeilidStateConfig.fromJson(json['config']),
    );

Map<String, dynamic> _$$VeilidStateImplToJson(_$VeilidStateImpl instance) =>
    <String, dynamic>{
      'attachment': instance.attachment.toJson(),
      'network': instance.network.toJson(),
      'config': instance.config.toJson(),
    };
