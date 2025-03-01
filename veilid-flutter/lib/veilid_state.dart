import 'dart:typed_data';

import 'package:change_case/change_case.dart';
import 'package:freezed_annotation/freezed_annotation.dart';

import 'veilid.dart';

part 'veilid_state.freezed.dart';
part 'veilid_state.g.dart';

//////////////////////////////////////
/// AttachmentState

enum AttachmentState {
  detached,
  attaching,
  attachedWeak,
  attachedGood,
  attachedStrong,
  fullyAttached,
  overAttached,
  detaching;

  factory AttachmentState.fromJson(dynamic j) =>
      AttachmentState.values.byName((j as String).toCamelCase());

  String toJson() => name.toPascalCase();
}

//////////////////////////////////////
/// VeilidLogLevel

enum VeilidLogLevel {
  error,
  warn,
  info,
  debug,
  trace;

  factory VeilidLogLevel.fromJson(dynamic j) =>
      VeilidLogLevel.values.byName((j as String).toCamelCase());

  String toJson() => name.toPascalCase();
}

////////////

@freezed
class LatencyStats with _$LatencyStats {
  const factory LatencyStats({
    required TimestampDuration fastest,
    required TimestampDuration average,
    required TimestampDuration slowest,
    required TimestampDuration tm90,
    required TimestampDuration tm75,
    required TimestampDuration p90,
    required TimestampDuration p75,
  }) = _LatencyStats;

  factory LatencyStats.fromJson(dynamic json) =>
      _$LatencyStatsFromJson(json as Map<String, dynamic>);
}

////////////

@freezed
class TransferStats with _$TransferStats {
  const factory TransferStats({
    required BigInt total,
    required BigInt maximum,
    required BigInt average,
    required BigInt minimum,
  }) = _TransferStats;

  factory TransferStats.fromJson(dynamic json) =>
      _$TransferStatsFromJson(json as Map<String, dynamic>);
}

////////////

@freezed
class TransferStatsDownUp with _$TransferStatsDownUp {
  const factory TransferStatsDownUp({
    required TransferStats down,
    required TransferStats up,
  }) = _TransferStatsDownUp;

  factory TransferStatsDownUp.fromJson(dynamic json) =>
      _$TransferStatsDownUpFromJson(json as Map<String, dynamic>);
}

////////////

@freezed
class StateStats with _$StateStats {
  const factory StateStats({
    required TimestampDuration span,
    required TimestampDuration reliable,
    required TimestampDuration unreliable,
    required TimestampDuration dead,
    required TimestampDuration punished,
    required StateReasonStats reason,
  }) = _StateStats;

  factory StateStats.fromJson(dynamic json) =>
      _$StateStatsFromJson(json as Map<String, dynamic>);
}

////////////

@freezed
class StateReasonStats with _$StateReasonStats {
  const factory StateReasonStats({
    required TimestampDuration canNotSend,
    required TimestampDuration tooManyLostAnswers,
    required TimestampDuration noPingResponse,
    required TimestampDuration failedToSend,
    required TimestampDuration lostAnswers,
    required TimestampDuration notSeenConsecutively,
    required TimestampDuration inUnreliablePingSpan,
  }) = _StateReasonStats;

  factory StateReasonStats.fromJson(dynamic json) =>
      _$StateReasonStatsFromJson(json as Map<String, dynamic>);
}

////////////

@freezed
class AnswerStats with _$AnswerStats {
  const factory AnswerStats({
    required TimestampDuration span,
    required int questions,
    required int answers,
    required int lostAnswers,
    required int consecutiveAnswersMaximum,
    required int consecutiveAnswersAverage,
    required int consecutiveAnswersMinimum,
    required int consecutiveLostAnswersMaximum,
    required int consecutiveLostAnswersAverage,
    required int consecutiveLostAnswersMinimum,
  }) = _AnswerStats;

  factory AnswerStats.fromJson(dynamic json) =>
      _$AnswerStatsFromJson(json as Map<String, dynamic>);
}

////////////

@freezed
class RPCStats with _$RPCStats {
  const factory RPCStats({
    required int messagesSent,
    required int messagesRcvd,
    required int questionsInFlight,
    required Timestamp? lastQuestionTs,
    required Timestamp? lastSeenTs,
    required Timestamp? firstConsecutiveSeenTs,
    required int recentLostAnswersUnordered,
    required int recentLostAnswersOrdered,
    required int failedToSend,
    required AnswerStats answerUnordered,
    required AnswerStats answerOrdered,
  }) = _RPCStats;

  factory RPCStats.fromJson(dynamic json) =>
      _$RPCStatsFromJson(json as Map<String, dynamic>);
}

////////////

@freezed
class PeerStats with _$PeerStats {
  const factory PeerStats({
    required Timestamp timeAdded,
    required RPCStats rpcStats,
    required TransferStatsDownUp transfer,
    required StateStats state,
    LatencyStats? latency,
  }) = _PeerStats;

  factory PeerStats.fromJson(dynamic json) =>
      _$PeerStatsFromJson(json as Map<String, dynamic>);
}

////////////

@freezed
class PeerTableData with _$PeerTableData {
  const factory PeerTableData({
    required List<TypedKey> nodeIds,
    required String peerAddress,
    required PeerStats peerStats,
  }) = _PeerTableData;

  factory PeerTableData.fromJson(dynamic json) =>
      _$PeerTableDataFromJson(json as Map<String, dynamic>);
}

//////////////////////////////////////
/// VeilidUpdate

@Freezed(unionKey: 'kind', unionValueCase: FreezedUnionCase.pascal)
sealed class VeilidUpdate with _$VeilidUpdate {
  const factory VeilidUpdate.log({
    required VeilidLogLevel logLevel,
    required String message,
    String? backtrace,
  }) = VeilidLog;
  const factory VeilidUpdate.appMessage({
    @Uint8ListJsonConverter.jsIsArray() required Uint8List message,
    TypedKey? sender,
    String? routeId,
  }) = VeilidAppMessage;
  const factory VeilidUpdate.appCall({
    @Uint8ListJsonConverter.jsIsArray() required Uint8List message,
    required String callId,
    TypedKey? sender,
    String? routeId,
  }) = VeilidAppCall;
  const factory VeilidUpdate.attachment(
      {required AttachmentState state,
      required bool publicInternetReady,
      required bool localNetworkReady,
      required TimestampDuration uptime,
      required TimestampDuration? attachedUptime}) = VeilidUpdateAttachment;
  const factory VeilidUpdate.network(
      {required bool started,
      required BigInt bpsDown,
      required BigInt bpsUp,
      required List<PeerTableData> peers}) = VeilidUpdateNetwork;
  const factory VeilidUpdate.config({
    required VeilidConfig config,
  }) = VeilidUpdateConfig;
  const factory VeilidUpdate.routeChange({
    required List<String> deadRoutes,
    required List<String> deadRemoteRoutes,
  }) = VeilidUpdateRouteChange;
  const factory VeilidUpdate.valueChange({
    required TypedKey key,
    required List<ValueSubkeyRange> subkeys,
    required int count,
    required ValueData? value,
  }) = VeilidUpdateValueChange;

  factory VeilidUpdate.fromJson(dynamic json) =>
      _$VeilidUpdateFromJson(json as Map<String, dynamic>);
}

//////////////////////////////////////
/// VeilidStateAttachment

@freezed
class VeilidStateAttachment with _$VeilidStateAttachment {
  const factory VeilidStateAttachment(
      {required AttachmentState state,
      required bool publicInternetReady,
      required bool localNetworkReady,
      required TimestampDuration uptime,
      required TimestampDuration? attachedUptime}) = _VeilidStateAttachment;

  factory VeilidStateAttachment.fromJson(dynamic json) =>
      _$VeilidStateAttachmentFromJson(json as Map<String, dynamic>);
}

//////////////////////////////////////
/// VeilidStateNetwork

@freezed
class VeilidStateNetwork with _$VeilidStateNetwork {
  const factory VeilidStateNetwork(
      {required bool started,
      required BigInt bpsDown,
      required BigInt bpsUp,
      required List<PeerTableData> peers}) = _VeilidStateNetwork;

  factory VeilidStateNetwork.fromJson(dynamic json) =>
      _$VeilidStateNetworkFromJson(json as Map<String, dynamic>);
}

//////////////////////////////////////
/// VeilidStateConfig

@freezed
class VeilidStateConfig with _$VeilidStateConfig {
  const factory VeilidStateConfig({
    required VeilidConfig config,
  }) = _VeilidStateConfig;

  factory VeilidStateConfig.fromJson(dynamic json) =>
      _$VeilidStateConfigFromJson(json as Map<String, dynamic>);
}

//////////////////////////////////////
/// VeilidState

@freezed
class VeilidState with _$VeilidState {
  const factory VeilidState({
    required VeilidStateAttachment attachment,
    required VeilidStateNetwork network,
    required VeilidStateConfig config,
  }) = _VeilidState;

  factory VeilidState.fromJson(dynamic json) =>
      _$VeilidStateFromJson(json as Map<String, dynamic>);
}
