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

_$StateStatsImpl _$$StateStatsImplFromJson(Map<String, dynamic> json) =>
    _$StateStatsImpl(
      span: TimestampDuration.fromJson(json['span']),
      reliable: TimestampDuration.fromJson(json['reliable']),
      unreliable: TimestampDuration.fromJson(json['unreliable']),
      dead: TimestampDuration.fromJson(json['dead']),
      punished: TimestampDuration.fromJson(json['punished']),
      reason: StateReasonStats.fromJson(json['reason']),
    );

Map<String, dynamic> _$$StateStatsImplToJson(_$StateStatsImpl instance) =>
    <String, dynamic>{
      'span': instance.span.toJson(),
      'reliable': instance.reliable.toJson(),
      'unreliable': instance.unreliable.toJson(),
      'dead': instance.dead.toJson(),
      'punished': instance.punished.toJson(),
      'reason': instance.reason.toJson(),
    };

_$StateReasonStatsImpl _$$StateReasonStatsImplFromJson(
        Map<String, dynamic> json) =>
    _$StateReasonStatsImpl(
      canNotSend: TimestampDuration.fromJson(json['can_not_send']),
      tooManyLostAnswers:
          TimestampDuration.fromJson(json['too_many_lost_answers']),
      noPingResponse: TimestampDuration.fromJson(json['no_ping_response']),
      failedToSend: TimestampDuration.fromJson(json['failed_to_send']),
      lostAnswers: TimestampDuration.fromJson(json['lost_answers']),
      notSeenConsecutively:
          TimestampDuration.fromJson(json['not_seen_consecutively']),
      inUnreliablePingSpan:
          TimestampDuration.fromJson(json['in_unreliable_ping_span']),
    );

Map<String, dynamic> _$$StateReasonStatsImplToJson(
        _$StateReasonStatsImpl instance) =>
    <String, dynamic>{
      'can_not_send': instance.canNotSend.toJson(),
      'too_many_lost_answers': instance.tooManyLostAnswers.toJson(),
      'no_ping_response': instance.noPingResponse.toJson(),
      'failed_to_send': instance.failedToSend.toJson(),
      'lost_answers': instance.lostAnswers.toJson(),
      'not_seen_consecutively': instance.notSeenConsecutively.toJson(),
      'in_unreliable_ping_span': instance.inUnreliablePingSpan.toJson(),
    };

_$AnswerStatsImpl _$$AnswerStatsImplFromJson(Map<String, dynamic> json) =>
    _$AnswerStatsImpl(
      span: TimestampDuration.fromJson(json['span']),
      questions: (json['questions'] as num).toInt(),
      answers: (json['answers'] as num).toInt(),
      lostAnswers: (json['lost_answers'] as num).toInt(),
      consecutiveAnswersMaximum:
          (json['consecutive_answers_maximum'] as num).toInt(),
      consecutiveAnswersAverage:
          (json['consecutive_answers_average'] as num).toInt(),
      consecutiveAnswersMinimum:
          (json['consecutive_answers_minimum'] as num).toInt(),
      consecutiveLostAnswersMaximum:
          (json['consecutive_lost_answers_maximum'] as num).toInt(),
      consecutiveLostAnswersAverage:
          (json['consecutive_lost_answers_average'] as num).toInt(),
      consecutiveLostAnswersMinimum:
          (json['consecutive_lost_answers_minimum'] as num).toInt(),
    );

Map<String, dynamic> _$$AnswerStatsImplToJson(_$AnswerStatsImpl instance) =>
    <String, dynamic>{
      'span': instance.span.toJson(),
      'questions': instance.questions,
      'answers': instance.answers,
      'lost_answers': instance.lostAnswers,
      'consecutive_answers_maximum': instance.consecutiveAnswersMaximum,
      'consecutive_answers_average': instance.consecutiveAnswersAverage,
      'consecutive_answers_minimum': instance.consecutiveAnswersMinimum,
      'consecutive_lost_answers_maximum':
          instance.consecutiveLostAnswersMaximum,
      'consecutive_lost_answers_average':
          instance.consecutiveLostAnswersAverage,
      'consecutive_lost_answers_minimum':
          instance.consecutiveLostAnswersMinimum,
    };

_$RPCStatsImpl _$$RPCStatsImplFromJson(Map<String, dynamic> json) =>
    _$RPCStatsImpl(
      messagesSent: (json['messages_sent'] as num).toInt(),
      messagesRcvd: (json['messages_rcvd'] as num).toInt(),
      questionsInFlight: (json['questions_in_flight'] as num).toInt(),
      lastQuestionTs: json['last_question_ts'] == null
          ? null
          : Timestamp.fromJson(json['last_question_ts']),
      lastSeenTs: json['last_seen_ts'] == null
          ? null
          : Timestamp.fromJson(json['last_seen_ts']),
      firstConsecutiveSeenTs: json['first_consecutive_seen_ts'] == null
          ? null
          : Timestamp.fromJson(json['first_consecutive_seen_ts']),
      recentLostAnswers: (json['recent_lost_answers'] as num).toInt(),
      failedToSend: (json['failed_to_send'] as num).toInt(),
      answer: AnswerStats.fromJson(json['answer']),
    );

Map<String, dynamic> _$$RPCStatsImplToJson(_$RPCStatsImpl instance) =>
    <String, dynamic>{
      'messages_sent': instance.messagesSent,
      'messages_rcvd': instance.messagesRcvd,
      'questions_in_flight': instance.questionsInFlight,
      'last_question_ts': instance.lastQuestionTs?.toJson(),
      'last_seen_ts': instance.lastSeenTs?.toJson(),
      'first_consecutive_seen_ts': instance.firstConsecutiveSeenTs?.toJson(),
      'recent_lost_answers': instance.recentLostAnswers,
      'failed_to_send': instance.failedToSend,
      'answer': instance.answer.toJson(),
    };

_$PeerStatsImpl _$$PeerStatsImplFromJson(Map<String, dynamic> json) =>
    _$PeerStatsImpl(
      timeAdded: Timestamp.fromJson(json['time_added']),
      rpcStats: RPCStats.fromJson(json['rpc_stats']),
      transfer: TransferStatsDownUp.fromJson(json['transfer']),
      state: StateStats.fromJson(json['state']),
      latency: json['latency'] == null
          ? null
          : LatencyStats.fromJson(json['latency']),
    );

Map<String, dynamic> _$$PeerStatsImplToJson(_$PeerStatsImpl instance) =>
    <String, dynamic>{
      'time_added': instance.timeAdded.toJson(),
      'rpc_stats': instance.rpcStats.toJson(),
      'transfer': instance.transfer.toJson(),
      'state': instance.state.toJson(),
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
      message:
          const Uint8ListJsonConverter.jsIsArray().fromJson(json['message']),
      sender: json['sender'] == null
          ? null
          : Typed<FixedEncodedString43>.fromJson(json['sender']),
      routeId: json['route_id'] as String?,
      $type: json['kind'] as String?,
    );

Map<String, dynamic> _$$VeilidAppMessageImplToJson(
        _$VeilidAppMessageImpl instance) =>
    <String, dynamic>{
      'message':
          const Uint8ListJsonConverter.jsIsArray().toJson(instance.message),
      'sender': instance.sender?.toJson(),
      'route_id': instance.routeId,
      'kind': instance.$type,
    };

_$VeilidAppCallImpl _$$VeilidAppCallImplFromJson(Map<String, dynamic> json) =>
    _$VeilidAppCallImpl(
      message:
          const Uint8ListJsonConverter.jsIsArray().fromJson(json['message']),
      callId: json['call_id'] as String,
      sender: json['sender'] == null
          ? null
          : Typed<FixedEncodedString43>.fromJson(json['sender']),
      routeId: json['route_id'] as String?,
      $type: json['kind'] as String?,
    );

Map<String, dynamic> _$$VeilidAppCallImplToJson(_$VeilidAppCallImpl instance) =>
    <String, dynamic>{
      'message':
          const Uint8ListJsonConverter.jsIsArray().toJson(instance.message),
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
      count: (json['count'] as num).toInt(),
      value: json['value'] == null ? null : ValueData.fromJson(json['value']),
      $type: json['kind'] as String?,
    );

Map<String, dynamic> _$$VeilidUpdateValueChangeImplToJson(
        _$VeilidUpdateValueChangeImpl instance) =>
    <String, dynamic>{
      'key': instance.key.toJson(),
      'subkeys': instance.subkeys.map((e) => e.toJson()).toList(),
      'count': instance.count,
      'value': instance.value?.toJson(),
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
