// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'veilid_state.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

_LatencyStats _$LatencyStatsFromJson(Map<String, dynamic> json) =>
    _LatencyStats(
      fastest: TimestampDuration.fromJson(json['fastest']),
      average: TimestampDuration.fromJson(json['average']),
      slowest: TimestampDuration.fromJson(json['slowest']),
      tm90: TimestampDuration.fromJson(json['tm90']),
      tm75: TimestampDuration.fromJson(json['tm75']),
      p90: TimestampDuration.fromJson(json['p90']),
      p75: TimestampDuration.fromJson(json['p75']),
    );

Map<String, dynamic> _$LatencyStatsToJson(_LatencyStats instance) =>
    <String, dynamic>{
      'fastest': instance.fastest.toJson(),
      'average': instance.average.toJson(),
      'slowest': instance.slowest.toJson(),
      'tm90': instance.tm90.toJson(),
      'tm75': instance.tm75.toJson(),
      'p90': instance.p90.toJson(),
      'p75': instance.p75.toJson(),
    };

_TransferStats _$TransferStatsFromJson(Map<String, dynamic> json) =>
    _TransferStats(
      total: BigInt.parse(json['total'] as String),
      maximum: BigInt.parse(json['maximum'] as String),
      average: BigInt.parse(json['average'] as String),
      minimum: BigInt.parse(json['minimum'] as String),
    );

Map<String, dynamic> _$TransferStatsToJson(_TransferStats instance) =>
    <String, dynamic>{
      'total': instance.total.toString(),
      'maximum': instance.maximum.toString(),
      'average': instance.average.toString(),
      'minimum': instance.minimum.toString(),
    };

_TransferStatsDownUp _$TransferStatsDownUpFromJson(Map<String, dynamic> json) =>
    _TransferStatsDownUp(
      down: TransferStats.fromJson(json['down']),
      up: TransferStats.fromJson(json['up']),
    );

Map<String, dynamic> _$TransferStatsDownUpToJson(
        _TransferStatsDownUp instance) =>
    <String, dynamic>{
      'down': instance.down.toJson(),
      'up': instance.up.toJson(),
    };

_StateStats _$StateStatsFromJson(Map<String, dynamic> json) => _StateStats(
      span: TimestampDuration.fromJson(json['span']),
      reliable: TimestampDuration.fromJson(json['reliable']),
      unreliable: TimestampDuration.fromJson(json['unreliable']),
      dead: TimestampDuration.fromJson(json['dead']),
      punished: TimestampDuration.fromJson(json['punished']),
      reason: StateReasonStats.fromJson(json['reason']),
    );

Map<String, dynamic> _$StateStatsToJson(_StateStats instance) =>
    <String, dynamic>{
      'span': instance.span.toJson(),
      'reliable': instance.reliable.toJson(),
      'unreliable': instance.unreliable.toJson(),
      'dead': instance.dead.toJson(),
      'punished': instance.punished.toJson(),
      'reason': instance.reason.toJson(),
    };

_StateReasonStats _$StateReasonStatsFromJson(Map<String, dynamic> json) =>
    _StateReasonStats(
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

Map<String, dynamic> _$StateReasonStatsToJson(_StateReasonStats instance) =>
    <String, dynamic>{
      'can_not_send': instance.canNotSend.toJson(),
      'too_many_lost_answers': instance.tooManyLostAnswers.toJson(),
      'no_ping_response': instance.noPingResponse.toJson(),
      'failed_to_send': instance.failedToSend.toJson(),
      'lost_answers': instance.lostAnswers.toJson(),
      'not_seen_consecutively': instance.notSeenConsecutively.toJson(),
      'in_unreliable_ping_span': instance.inUnreliablePingSpan.toJson(),
    };

_AnswerStats _$AnswerStatsFromJson(Map<String, dynamic> json) => _AnswerStats(
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

Map<String, dynamic> _$AnswerStatsToJson(_AnswerStats instance) =>
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

_RPCStats _$RPCStatsFromJson(Map<String, dynamic> json) => _RPCStats(
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
      recentLostAnswersUnordered:
          (json['recent_lost_answers_unordered'] as num).toInt(),
      recentLostAnswersOrdered:
          (json['recent_lost_answers_ordered'] as num).toInt(),
      failedToSend: (json['failed_to_send'] as num).toInt(),
      answerUnordered: AnswerStats.fromJson(json['answer_unordered']),
      answerOrdered: AnswerStats.fromJson(json['answer_ordered']),
    );

Map<String, dynamic> _$RPCStatsToJson(_RPCStats instance) => <String, dynamic>{
      'messages_sent': instance.messagesSent,
      'messages_rcvd': instance.messagesRcvd,
      'questions_in_flight': instance.questionsInFlight,
      'last_question_ts': instance.lastQuestionTs?.toJson(),
      'last_seen_ts': instance.lastSeenTs?.toJson(),
      'first_consecutive_seen_ts': instance.firstConsecutiveSeenTs?.toJson(),
      'recent_lost_answers_unordered': instance.recentLostAnswersUnordered,
      'recent_lost_answers_ordered': instance.recentLostAnswersOrdered,
      'failed_to_send': instance.failedToSend,
      'answer_unordered': instance.answerUnordered.toJson(),
      'answer_ordered': instance.answerOrdered.toJson(),
    };

_PeerStats _$PeerStatsFromJson(Map<String, dynamic> json) => _PeerStats(
      timeAdded: Timestamp.fromJson(json['time_added']),
      rpcStats: RPCStats.fromJson(json['rpc_stats']),
      transfer: TransferStatsDownUp.fromJson(json['transfer']),
      state: StateStats.fromJson(json['state']),
      latency: json['latency'] == null
          ? null
          : LatencyStats.fromJson(json['latency']),
    );

Map<String, dynamic> _$PeerStatsToJson(_PeerStats instance) =>
    <String, dynamic>{
      'time_added': instance.timeAdded.toJson(),
      'rpc_stats': instance.rpcStats.toJson(),
      'transfer': instance.transfer.toJson(),
      'state': instance.state.toJson(),
      'latency': instance.latency?.toJson(),
    };

_PeerTableData _$PeerTableDataFromJson(Map<String, dynamic> json) =>
    _PeerTableData(
      nodeIds: (json['node_ids'] as List<dynamic>)
          .map(Typed<FixedEncodedString43>.fromJson)
          .toList(),
      peerAddress: json['peer_address'] as String,
      peerStats: PeerStats.fromJson(json['peer_stats']),
    );

Map<String, dynamic> _$PeerTableDataToJson(_PeerTableData instance) =>
    <String, dynamic>{
      'node_ids': instance.nodeIds.map((e) => e.toJson()).toList(),
      'peer_address': instance.peerAddress,
      'peer_stats': instance.peerStats.toJson(),
    };

VeilidLog _$VeilidLogFromJson(Map<String, dynamic> json) => VeilidLog(
      logLevel: VeilidLogLevel.fromJson(json['log_level']),
      message: json['message'] as String,
      backtrace: json['backtrace'] as String?,
      $type: json['kind'] as String?,
    );

Map<String, dynamic> _$VeilidLogToJson(VeilidLog instance) => <String, dynamic>{
      'log_level': instance.logLevel.toJson(),
      'message': instance.message,
      'backtrace': instance.backtrace,
      'kind': instance.$type,
    };

VeilidAppMessage _$VeilidAppMessageFromJson(Map<String, dynamic> json) =>
    VeilidAppMessage(
      message:
          const Uint8ListJsonConverter.jsIsArray().fromJson(json['message']),
      sender: json['sender'] == null
          ? null
          : Typed<FixedEncodedString43>.fromJson(json['sender']),
      routeId: json['route_id'] as String?,
      $type: json['kind'] as String?,
    );

Map<String, dynamic> _$VeilidAppMessageToJson(VeilidAppMessage instance) =>
    <String, dynamic>{
      'message':
          const Uint8ListJsonConverter.jsIsArray().toJson(instance.message),
      'sender': instance.sender?.toJson(),
      'route_id': instance.routeId,
      'kind': instance.$type,
    };

VeilidAppCall _$VeilidAppCallFromJson(Map<String, dynamic> json) =>
    VeilidAppCall(
      message:
          const Uint8ListJsonConverter.jsIsArray().fromJson(json['message']),
      callId: json['call_id'] as String,
      sender: json['sender'] == null
          ? null
          : Typed<FixedEncodedString43>.fromJson(json['sender']),
      routeId: json['route_id'] as String?,
      $type: json['kind'] as String?,
    );

Map<String, dynamic> _$VeilidAppCallToJson(VeilidAppCall instance) =>
    <String, dynamic>{
      'message':
          const Uint8ListJsonConverter.jsIsArray().toJson(instance.message),
      'call_id': instance.callId,
      'sender': instance.sender?.toJson(),
      'route_id': instance.routeId,
      'kind': instance.$type,
    };

VeilidUpdateAttachment _$VeilidUpdateAttachmentFromJson(
        Map<String, dynamic> json) =>
    VeilidUpdateAttachment(
      state: AttachmentState.fromJson(json['state']),
      publicInternetReady: json['public_internet_ready'] as bool,
      localNetworkReady: json['local_network_ready'] as bool,
      uptime: TimestampDuration.fromJson(json['uptime']),
      attachedUptime: json['attached_uptime'] == null
          ? null
          : TimestampDuration.fromJson(json['attached_uptime']),
      $type: json['kind'] as String?,
    );

Map<String, dynamic> _$VeilidUpdateAttachmentToJson(
        VeilidUpdateAttachment instance) =>
    <String, dynamic>{
      'state': instance.state.toJson(),
      'public_internet_ready': instance.publicInternetReady,
      'local_network_ready': instance.localNetworkReady,
      'uptime': instance.uptime.toJson(),
      'attached_uptime': instance.attachedUptime?.toJson(),
      'kind': instance.$type,
    };

VeilidUpdateNetwork _$VeilidUpdateNetworkFromJson(Map<String, dynamic> json) =>
    VeilidUpdateNetwork(
      started: json['started'] as bool,
      bpsDown: BigInt.parse(json['bps_down'] as String),
      bpsUp: BigInt.parse(json['bps_up'] as String),
      peers:
          (json['peers'] as List<dynamic>).map(PeerTableData.fromJson).toList(),
      $type: json['kind'] as String?,
    );

Map<String, dynamic> _$VeilidUpdateNetworkToJson(
        VeilidUpdateNetwork instance) =>
    <String, dynamic>{
      'started': instance.started,
      'bps_down': instance.bpsDown.toString(),
      'bps_up': instance.bpsUp.toString(),
      'peers': instance.peers.map((e) => e.toJson()).toList(),
      'kind': instance.$type,
    };

VeilidUpdateConfig _$VeilidUpdateConfigFromJson(Map<String, dynamic> json) =>
    VeilidUpdateConfig(
      config: VeilidConfig.fromJson(json['config']),
      $type: json['kind'] as String?,
    );

Map<String, dynamic> _$VeilidUpdateConfigToJson(VeilidUpdateConfig instance) =>
    <String, dynamic>{
      'config': instance.config.toJson(),
      'kind': instance.$type,
    };

VeilidUpdateRouteChange _$VeilidUpdateRouteChangeFromJson(
        Map<String, dynamic> json) =>
    VeilidUpdateRouteChange(
      deadRoutes: (json['dead_routes'] as List<dynamic>)
          .map((e) => e as String)
          .toList(),
      deadRemoteRoutes: (json['dead_remote_routes'] as List<dynamic>)
          .map((e) => e as String)
          .toList(),
      $type: json['kind'] as String?,
    );

Map<String, dynamic> _$VeilidUpdateRouteChangeToJson(
        VeilidUpdateRouteChange instance) =>
    <String, dynamic>{
      'dead_routes': instance.deadRoutes,
      'dead_remote_routes': instance.deadRemoteRoutes,
      'kind': instance.$type,
    };

VeilidUpdateValueChange _$VeilidUpdateValueChangeFromJson(
        Map<String, dynamic> json) =>
    VeilidUpdateValueChange(
      key: Typed<FixedEncodedString43>.fromJson(json['key']),
      subkeys: (json['subkeys'] as List<dynamic>)
          .map(ValueSubkeyRange.fromJson)
          .toList(),
      count: (json['count'] as num).toInt(),
      value: json['value'] == null ? null : ValueData.fromJson(json['value']),
      $type: json['kind'] as String?,
    );

Map<String, dynamic> _$VeilidUpdateValueChangeToJson(
        VeilidUpdateValueChange instance) =>
    <String, dynamic>{
      'key': instance.key.toJson(),
      'subkeys': instance.subkeys.map((e) => e.toJson()).toList(),
      'count': instance.count,
      'value': instance.value?.toJson(),
      'kind': instance.$type,
    };

_VeilidStateAttachment _$VeilidStateAttachmentFromJson(
        Map<String, dynamic> json) =>
    _VeilidStateAttachment(
      state: AttachmentState.fromJson(json['state']),
      publicInternetReady: json['public_internet_ready'] as bool,
      localNetworkReady: json['local_network_ready'] as bool,
      uptime: TimestampDuration.fromJson(json['uptime']),
      attachedUptime: json['attached_uptime'] == null
          ? null
          : TimestampDuration.fromJson(json['attached_uptime']),
    );

Map<String, dynamic> _$VeilidStateAttachmentToJson(
        _VeilidStateAttachment instance) =>
    <String, dynamic>{
      'state': instance.state.toJson(),
      'public_internet_ready': instance.publicInternetReady,
      'local_network_ready': instance.localNetworkReady,
      'uptime': instance.uptime.toJson(),
      'attached_uptime': instance.attachedUptime?.toJson(),
    };

_VeilidStateNetwork _$VeilidStateNetworkFromJson(Map<String, dynamic> json) =>
    _VeilidStateNetwork(
      started: json['started'] as bool,
      bpsDown: BigInt.parse(json['bps_down'] as String),
      bpsUp: BigInt.parse(json['bps_up'] as String),
      peers:
          (json['peers'] as List<dynamic>).map(PeerTableData.fromJson).toList(),
    );

Map<String, dynamic> _$VeilidStateNetworkToJson(_VeilidStateNetwork instance) =>
    <String, dynamic>{
      'started': instance.started,
      'bps_down': instance.bpsDown.toString(),
      'bps_up': instance.bpsUp.toString(),
      'peers': instance.peers.map((e) => e.toJson()).toList(),
    };

_VeilidStateConfig _$VeilidStateConfigFromJson(Map<String, dynamic> json) =>
    _VeilidStateConfig(
      config: VeilidConfig.fromJson(json['config']),
    );

Map<String, dynamic> _$VeilidStateConfigToJson(_VeilidStateConfig instance) =>
    <String, dynamic>{
      'config': instance.config.toJson(),
    };

_VeilidState _$VeilidStateFromJson(Map<String, dynamic> json) => _VeilidState(
      attachment: VeilidStateAttachment.fromJson(json['attachment']),
      network: VeilidStateNetwork.fromJson(json['network']),
      config: VeilidStateConfig.fromJson(json['config']),
    );

Map<String, dynamic> _$VeilidStateToJson(_VeilidState instance) =>
    <String, dynamic>{
      'attachment': instance.attachment.toJson(),
      'network': instance.network.toJson(),
      'config': instance.config.toJson(),
    };
