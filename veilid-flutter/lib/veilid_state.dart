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

  factory LatencyStats.fromJson(Map<String, Object?> json) =>
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

  factory TransferStats.fromJson(Map<String, Object?> json) =>
      _$TransferStatsFromJson(json);
}

////////////

@freezed
class TransferStatsDownUp with _$TransferStatsDownUp {
  const factory TransferStatsDownUp({
    required TransferStats down,
    required TransferStats up,
  }) = _TransferStatsDownUp;

  factory TransferStatsDownUp.fromJson(Map<String, Object?> json) =>
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

  factory RPCStats.fromJson(Map<String, Object?> json) =>
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

  factory PeerStats.fromJson(Map<String, Object?> json) =>
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

  factory PeerTableData.fromJson(Map<String, Object?> json) =>
      _$PeerTableDataFromJson(json);
}

//////////////////////////////////////
/// VeilidUpdate

abstract class VeilidUpdate {
  factory VeilidUpdate.fromJson(dynamic json) {
    switch (json["kind"]) {
      case "Log":
        {
          return VeilidLog(
              logLevel: VeilidLogLevel.fromJson(json["log_level"]),
              message: json["message"],
              backtrace: json["backtrace"]);
        }
      case "AppMessage":
        {
          return VeilidAppMessage(
              sender: json["sender"], message: json["message"]);
        }
      case "AppCall":
        {
          return VeilidAppCall(
              sender: json["sender"],
              message: json["message"],
              callId: json["call_id"]);
        }
      case "Attachment":
        {
          return VeilidUpdateAttachment(
              state: VeilidStateAttachment.fromJson(json));
        }
      case "Network":
        {
          return VeilidUpdateNetwork(state: VeilidStateNetwork.fromJson(json));
        }
      case "Config":
        {
          return VeilidUpdateConfig(state: VeilidStateConfig.fromJson(json));
        }
      case "RouteChange":
        {
          return VeilidUpdateRouteChange(
              deadRoutes: List<String>.from(json['dead_routes'].map((j) => j)),
              deadRemoteRoutes:
                  List<String>.from(json['dead_remote_routes'].map((j) => j)));
        }
      case "ValueChange":
        {
          return VeilidUpdateValueChange(
              key: TypedKey.fromJson(json['key']),
              subkeys: List<ValueSubkeyRange>.from(
                  json['subkeys'].map((j) => ValueSubkeyRange.fromJson(j))),
              count: json['count'],
              valueData: ValueData.fromJson(json['value_data']));
        }
      default:
        {
          throw VeilidAPIExceptionInternal(
              "Invalid VeilidAPIException type: ${json['kind']}");
        }
    }
  }
  Map<String, dynamic> toJson();
}

class VeilidLog implements VeilidUpdate {
  final VeilidLogLevel logLevel;
  final String message;
  final String? backtrace;
  //
  VeilidLog({
    required this.logLevel,
    required this.message,
    required this.backtrace,
  });

  @override
  Map<String, dynamic> toJson() {
    return {
      'kind': "Log",
      'log_level': logLevel.toJson(),
      'message': message,
      'backtrace': backtrace
    };
  }
}

class VeilidAppMessage implements VeilidUpdate {
  final TypedKey? sender;
  final Uint8List message;

  //
  VeilidAppMessage({
    required this.sender,
    required this.message,
  });

  @override
  Map<String, dynamic> toJson() {
    return {
      'kind': "AppMessage",
      'sender': sender,
      'message': base64UrlNoPadEncode(message)
    };
  }
}

class VeilidAppCall implements VeilidUpdate {
  final String? sender;
  final Uint8List message;
  final String callId;

  //
  VeilidAppCall({
    required this.sender,
    required this.message,
    required this.callId,
  });

  @override
  Map<String, dynamic> toJson() {
    return {
      'kind': "AppCall",
      'sender': sender,
      'message': base64UrlNoPadEncode(message),
      'call_id': callId,
    };
  }
}

class VeilidUpdateAttachment implements VeilidUpdate {
  final VeilidStateAttachment state;
  //
  VeilidUpdateAttachment({required this.state});

  @override
  Map<String, dynamic> toJson() {
    var jsonRep = state.toJson();
    jsonRep['kind'] = "Attachment";
    return jsonRep;
  }
}

class VeilidUpdateNetwork implements VeilidUpdate {
  final VeilidStateNetwork state;
  //
  VeilidUpdateNetwork({required this.state});

  @override
  Map<String, dynamic> toJson() {
    var jsonRep = state.toJson();
    jsonRep['kind'] = "Network";
    return jsonRep;
  }
}

class VeilidUpdateConfig implements VeilidUpdate {
  final VeilidStateConfig state;
  //
  VeilidUpdateConfig({required this.state});

  @override
  Map<String, dynamic> toJson() {
    var jsonRep = state.toJson();
    jsonRep['kind'] = "Config";
    return jsonRep;
  }
}

class VeilidUpdateRouteChange implements VeilidUpdate {
  final List<String> deadRoutes;
  final List<String> deadRemoteRoutes;
  //
  VeilidUpdateRouteChange({
    required this.deadRoutes,
    required this.deadRemoteRoutes,
  });

  @override
  Map<String, dynamic> toJson() {
    return {
      'dead_routes': deadRoutes.map((p) => p).toList(),
      'dead_remote_routes': deadRemoteRoutes.map((p) => p).toList()
    };
  }
}

class VeilidUpdateValueChange implements VeilidUpdate {
  final TypedKey key;
  final List<ValueSubkeyRange> subkeys;
  final int count;
  final ValueData valueData;

  //
  VeilidUpdateValueChange({
    required this.key,
    required this.subkeys,
    required this.count,
    required this.valueData,
  });

  @override
  Map<String, dynamic> toJson() {
    return {
      'key': key.toJson(),
      'subkeys': subkeys.map((p) => p.toJson()).toList(),
      'count': count,
      'value_data': valueData.toJson(),
    };
  }
}

//////////////////////////////////////
/// VeilidStateAttachment

class VeilidStateAttachment {
  final AttachmentState state;
  final bool publicInternetReady;
  final bool localNetworkReady;

  VeilidStateAttachment(
      this.state, this.publicInternetReady, this.localNetworkReady);

  VeilidStateAttachment.fromJson(dynamic json)
      : state = AttachmentState.fromJson(json['state']),
        publicInternetReady = json['public_internet_ready'],
        localNetworkReady = json['local_network_ready'];

  Map<String, dynamic> toJson() {
    return {
      'state': state.toJson(),
      'public_internet_ready': publicInternetReady,
      'local_network_ready': localNetworkReady,
    };
  }
}

//////////////////////////////////////
/// VeilidStateNetwork

class VeilidStateNetwork {
  final bool started;
  final BigInt bpsDown;
  final BigInt bpsUp;
  final List<PeerTableData> peers;

  VeilidStateNetwork(
      {required this.started,
      required this.bpsDown,
      required this.bpsUp,
      required this.peers});

  VeilidStateNetwork.fromJson(dynamic json)
      : started = json['started'],
        bpsDown = BigInt.parse(json['bps_down']),
        bpsUp = BigInt.parse(json['bps_up']),
        peers = List<PeerTableData>.from(
            json['peers'].map((j) => PeerTableData.fromJson(j)));

  Map<String, dynamic> toJson() {
    return {
      'started': started,
      'bps_down': bpsDown.toString(),
      'bps_up': bpsUp.toString(),
      'peers': peers.map((p) => p.toJson()).toList(),
    };
  }
}

//////////////////////////////////////
/// VeilidStateConfig

class VeilidStateConfig {
  final VeilidConfig config;

  VeilidStateConfig({
    required this.config,
  });

  VeilidStateConfig.fromJson(dynamic json)
      : config = VeilidConfig.fromJson(json['config']);

  Map<String, dynamic> toJson() {
    return {'config': config.toJson()};
  }
}

//////////////////////////////////////
/// VeilidState

class VeilidState {
  final VeilidStateAttachment attachment;
  final VeilidStateNetwork network;
  final VeilidStateConfig config;

  VeilidState.fromJson(dynamic json)
      : attachment = VeilidStateAttachment.fromJson(json['attachment']),
        network = VeilidStateNetwork.fromJson(json['network']),
        config = VeilidStateConfig.fromJson(json['config']);

  Map<String, dynamic> toJson() {
    return {
      'attachment': attachment.toJson(),
      'network': network.toJson(),
      'config': config.toJson()
    };
  }
}
