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

_$VeilidLog _$$VeilidLogFromJson(Map<String, dynamic> json) => _$VeilidLog(
      logLevel: VeilidLogLevel.fromJson(json['log_level'] as String),
      message: json['message'] as String,
      backtrace: json['backtrace'] as String?,
      $type: json['kind'] as String?,
    );

Map<String, dynamic> _$$VeilidLogToJson(_$VeilidLog instance) =>
    <String, dynamic>{
      'log_level': instance.logLevel.toJson(),
      'message': instance.message,
      'backtrace': instance.backtrace,
      'kind': instance.$type,
    };

_$VeilidAppMessage _$$VeilidAppMessageFromJson(Map<String, dynamic> json) =>
    _$VeilidAppMessage(
      sender: json['sender'] == null
          ? null
          : Typed<FixedEncodedString43>.fromJson(json['sender']),
      message:
          const Uint8ListJsonConverter().fromJson(json['message'] as String),
      $type: json['kind'] as String?,
    );

Map<String, dynamic> _$$VeilidAppMessageToJson(_$VeilidAppMessage instance) =>
    <String, dynamic>{
      'sender': instance.sender?.toJson(),
      'message': const Uint8ListJsonConverter().toJson(instance.message),
      'kind': instance.$type,
    };

_$VeilidAppCall _$$VeilidAppCallFromJson(Map<String, dynamic> json) =>
    _$VeilidAppCall(
      sender: json['sender'] == null
          ? null
          : Typed<FixedEncodedString43>.fromJson(json['sender']),
      message:
          const Uint8ListJsonConverter().fromJson(json['message'] as String),
      callId: json['call_id'] as String,
      $type: json['kind'] as String?,
    );

Map<String, dynamic> _$$VeilidAppCallToJson(_$VeilidAppCall instance) =>
    <String, dynamic>{
      'sender': instance.sender?.toJson(),
      'message': const Uint8ListJsonConverter().toJson(instance.message),
      'call_id': instance.callId,
      'kind': instance.$type,
    };

_$VeilidUpdateAttachment _$$VeilidUpdateAttachmentFromJson(
        Map<String, dynamic> json) =>
    _$VeilidUpdateAttachment(
      state: AttachmentState.fromJson(json['state'] as String),
      publicInternetReady: json['public_internet_ready'] as bool,
      localNetworkReady: json['local_network_ready'] as bool,
      $type: json['kind'] as String?,
    );

Map<String, dynamic> _$$VeilidUpdateAttachmentToJson(
        _$VeilidUpdateAttachment instance) =>
    <String, dynamic>{
      'state': instance.state.toJson(),
      'public_internet_ready': instance.publicInternetReady,
      'local_network_ready': instance.localNetworkReady,
      'kind': instance.$type,
    };

_$VeilidUpdateNetwork _$$VeilidUpdateNetworkFromJson(
        Map<String, dynamic> json) =>
    _$VeilidUpdateNetwork(
      started: json['started'] as bool,
      bpsDown: BigInt.parse(json['bps_down'] as String),
      bpsUp: BigInt.parse(json['bps_up'] as String),
      peers: (json['peers'] as List<dynamic>)
          .map((e) => PeerTableData.fromJson(e as Map<String, dynamic>))
          .toList(),
      $type: json['kind'] as String?,
    );

Map<String, dynamic> _$$VeilidUpdateNetworkToJson(
        _$VeilidUpdateNetwork instance) =>
    <String, dynamic>{
      'started': instance.started,
      'bps_down': instance.bpsDown.toString(),
      'bps_up': instance.bpsUp.toString(),
      'peers': instance.peers.map((e) => e.toJson()).toList(),
      'kind': instance.$type,
    };

_$VeilidUpdateConfig _$$VeilidUpdateConfigFromJson(Map<String, dynamic> json) =>
    _$VeilidUpdateConfig(
      config: VeilidConfig.fromJson(json['config'] as Map<String, dynamic>),
      $type: json['kind'] as String?,
    );

Map<String, dynamic> _$$VeilidUpdateConfigToJson(
        _$VeilidUpdateConfig instance) =>
    <String, dynamic>{
      'config': instance.config.toJson(),
      'kind': instance.$type,
    };

_$VeilidUpdateRouteChange _$$VeilidUpdateRouteChangeFromJson(
        Map<String, dynamic> json) =>
    _$VeilidUpdateRouteChange(
      deadRoutes: (json['dead_routes'] as List<dynamic>)
          .map((e) => e as String)
          .toList(),
      deadRemoteRoutes: (json['dead_remote_routes'] as List<dynamic>)
          .map((e) => e as String)
          .toList(),
      $type: json['kind'] as String?,
    );

Map<String, dynamic> _$$VeilidUpdateRouteChangeToJson(
        _$VeilidUpdateRouteChange instance) =>
    <String, dynamic>{
      'dead_routes': instance.deadRoutes,
      'dead_remote_routes': instance.deadRemoteRoutes,
      'kind': instance.$type,
    };

_$VeilidUpdateValueChange _$$VeilidUpdateValueChangeFromJson(
        Map<String, dynamic> json) =>
    _$VeilidUpdateValueChange(
      key: Typed<FixedEncodedString43>.fromJson(json['key']),
      subkeys: (json['subkeys'] as List<dynamic>)
          .map((e) => ValueSubkeyRange.fromJson(e as Map<String, dynamic>))
          .toList(),
      count: json['count'] as int,
      valueData: ValueData.fromJson(json['value_data'] as Map<String, dynamic>),
      $type: json['kind'] as String?,
    );

Map<String, dynamic> _$$VeilidUpdateValueChangeToJson(
        _$VeilidUpdateValueChange instance) =>
    <String, dynamic>{
      'key': instance.key.toJson(),
      'subkeys': instance.subkeys.map((e) => e.toJson()).toList(),
      'count': instance.count,
      'value_data': instance.valueData.toJson(),
      'kind': instance.$type,
    };

_$_VeilidStateAttachment _$$_VeilidStateAttachmentFromJson(
        Map<String, dynamic> json) =>
    _$_VeilidStateAttachment(
      state: AttachmentState.fromJson(json['state'] as String),
      publicInternetReady: json['public_internet_ready'] as bool,
      localNetworkReady: json['local_network_ready'] as bool,
    );

Map<String, dynamic> _$$_VeilidStateAttachmentToJson(
        _$_VeilidStateAttachment instance) =>
    <String, dynamic>{
      'state': instance.state.toJson(),
      'public_internet_ready': instance.publicInternetReady,
      'local_network_ready': instance.localNetworkReady,
    };

_$_VeilidStateNetwork _$$_VeilidStateNetworkFromJson(
        Map<String, dynamic> json) =>
    _$_VeilidStateNetwork(
      started: json['started'] as bool,
      bpsDown: BigInt.parse(json['bps_down'] as String),
      bpsUp: BigInt.parse(json['bps_up'] as String),
      peers: (json['peers'] as List<dynamic>)
          .map((e) => PeerTableData.fromJson(e as Map<String, dynamic>))
          .toList(),
    );

Map<String, dynamic> _$$_VeilidStateNetworkToJson(
        _$_VeilidStateNetwork instance) =>
    <String, dynamic>{
      'started': instance.started,
      'bps_down': instance.bpsDown.toString(),
      'bps_up': instance.bpsUp.toString(),
      'peers': instance.peers.map((e) => e.toJson()).toList(),
    };

_$_VeilidStateConfig _$$_VeilidStateConfigFromJson(Map<String, dynamic> json) =>
    _$_VeilidStateConfig(
      config: VeilidConfig.fromJson(json['config'] as Map<String, dynamic>),
    );

Map<String, dynamic> _$$_VeilidStateConfigToJson(
        _$_VeilidStateConfig instance) =>
    <String, dynamic>{
      'config': instance.config.toJson(),
    };

_$_VeilidState _$$_VeilidStateFromJson(Map<String, dynamic> json) =>
    _$_VeilidState(
      attachment: VeilidStateAttachment.fromJson(
          json['attachment'] as Map<String, dynamic>),
      network:
          VeilidStateNetwork.fromJson(json['network'] as Map<String, dynamic>),
      config:
          VeilidStateConfig.fromJson(json['config'] as Map<String, dynamic>),
    );

Map<String, dynamic> _$$_VeilidStateToJson(_$_VeilidState instance) =>
    <String, dynamic>{
      'attachment': instance.attachment.toJson(),
      'network': instance.network.toJson(),
      'config': instance.config.toJson(),
    };
