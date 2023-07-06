import 'dart:typed_data';

import 'package:change_case/change_case.dart';
import 'package:freezed_annotation/freezed_annotation.dart';

import 'veilid_encoding.dart';
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

  String toJson() => name.toPascalCase();
  factory AttachmentState.fromJson(String j) =>
      AttachmentState.values.byName(j.toCamelCase());
}

//////////////////////////////////////
/// VeilidLogLevel

enum VeilidLogLevel {
  error,
  warn,
  info,
  debug,
  trace;

  String toJson() => name.toPascalCase();
  factory VeilidLogLevel.fromJson(String j) =>
      VeilidLogLevel.values.byName(j.toCamelCase());
}

////////////

@freezed
class LatencyStats with _$LatencyStats {
  const factory LatencyStats({
    required TimestampDuration fastest,
    required TimestampDuration average,
    required TimestampDuration slowest,
  }) = _LatencyStats;

  factory LatencyStats.fromJson(Map<String, dynamic> json) =>
      _$LatencyStatsFromJson(json);
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

  factory TransferStats.fromJson(Map<String, dynamic> json) =>
      _$TransferStatsFromJson(json);
}

////////////

@freezed
class TransferStatsDownUp with _$TransferStatsDownUp {
  const factory TransferStatsDownUp({
    required TransferStats down,
    required TransferStats up,
  }) = _TransferStatsDownUp;

  factory TransferStatsDownUp.fromJson(Map<String, dynamic> json) =>
      _$TransferStatsDownUpFromJson(json);
}

////////////

@freezed
class RPCStats with _$RPCStats {
  const factory RPCStats({
    required int messagesSent,
    required int messagesRcvd,
    required int questionsInFlight,
    required Timestamp? lastQuestion,
    required Timestamp? lastSeenTs,
    required Timestamp? firstConsecutiveSeenTs,
    required int recentLostAnswers,
    required int failedToSend,
  }) = _RPCStats;

  factory RPCStats.fromJson(Map<String, dynamic> json) =>
      _$RPCStatsFromJson(json);
}

////////////

@freezed
class PeerStats with _$PeerStats {
  const factory PeerStats({
    required Timestamp timeAdded,
    required RPCStats rpcStats,
    LatencyStats? latency,
    required TransferStatsDownUp transfer,
  }) = _PeerStats;

  factory PeerStats.fromJson(Map<String, dynamic> json) =>
      _$PeerStatsFromJson(json);
}

////////////

@freezed
class PeerTableData with _$PeerTableData {
  const factory PeerTableData({
    required List<TypedKey> nodeIds,
    required String peerAddress,
    required PeerStats peerStats,
  }) = _PeerTableData;

  factory PeerTableData.fromJson(Map<String, dynamic> json) =>
      _$PeerTableDataFromJson(json);
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
    TypedKey? sender,
    @Uint8ListJsonConverter() required Uint8List message,
  }) = VeilidAppMessage;
  const factory VeilidUpdate.appCall({
    TypedKey? sender,
    @Uint8ListJsonConverter() required Uint8List message,
    required String callId,
  }) = VeilidAppCall;
  const factory VeilidUpdate.attachment(
      {required AttachmentState state,
      required bool publicInternetReady,
      required bool localNetworkReady}) = VeilidUpdateAttachment;
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
    required ValueData valueData,
  }) = VeilidUpdateValueChange;

  factory VeilidUpdate.fromJson(Map<String, dynamic> json) =>
      _$VeilidUpdateFromJson(json);
}

//////////////////////////////////////
/// VeilidStateAttachment

@freezed
class VeilidStateAttachment with _$VeilidStateAttachment {
  const factory VeilidStateAttachment(
      {required AttachmentState state,
      required bool publicInternetReady,
      required bool localNetworkReady}) = _VeilidStateAttachment;

  factory VeilidStateAttachment.fromJson(Map<String, dynamic> json) =>
      _$VeilidStateAttachmentFromJson(json);
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

  factory VeilidStateNetwork.fromJson(Map<String, dynamic> json) =>
      _$VeilidStateNetworkFromJson(json);
}

//////////////////////////////////////
/// VeilidStateConfig

@freezed
class VeilidStateConfig with _$VeilidStateConfig {
  const factory VeilidStateConfig({
    required VeilidConfig config,
  }) = _VeilidStateConfig;

  factory VeilidStateConfig.fromJson(Map<String, dynamic> json) =>
      _$VeilidStateConfigFromJson(json);
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

  factory VeilidState.fromJson(Map<String, dynamic> json) =>
      _$VeilidStateFromJson(json);
}
