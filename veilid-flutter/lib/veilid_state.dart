import 'dart:typed_data';

import 'package:change_case/change_case.dart';

import 'veilid_encoding.dart';
import 'veilid.dart';

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

  String toJson() {
    return name.toPascalCase();
  }

  factory AttachmentState.fromJson(String j) {
    return AttachmentState.values.byName(j.toCamelCase());
  }
}

//////////////////////////////////////
/// VeilidLogLevel

enum VeilidLogLevel {
  error,
  warn,
  info,
  debug,
  trace;

  String toJson() {
    return name.toPascalCase();
  }

  factory VeilidLogLevel.fromJson(String j) {
    return VeilidLogLevel.values.byName(j.toCamelCase());
  }
}

////////////

class LatencyStats {
  TimestampDuration fastest;
  TimestampDuration average;
  TimestampDuration slowest;

  LatencyStats({
    required this.fastest,
    required this.average,
    required this.slowest,
  });

  Map<String, dynamic> toJson() {
    return {
      'fastest': fastest.toJson(),
      'average': average.toJson(),
      'slowest': slowest.toJson(),
    };
  }

  LatencyStats.fromJson(dynamic json)
      : fastest = TimestampDuration.fromJson(json['fastest']),
        average = TimestampDuration.fromJson(json['average']),
        slowest = TimestampDuration.fromJson(json['slowest']);
}

////////////

class TransferStats {
  BigInt total;
  BigInt maximum;
  BigInt average;
  BigInt minimum;

  TransferStats({
    required this.total,
    required this.maximum,
    required this.average,
    required this.minimum,
  });

  Map<String, dynamic> toJson() {
    return {
      'total': total.toString(),
      'maximum': maximum.toString(),
      'average': average.toString(),
      'minimum': minimum.toString(),
    };
  }

  TransferStats.fromJson(dynamic json)
      : total = BigInt.parse(json['total']),
        maximum = BigInt.parse(json['maximum']),
        average = BigInt.parse(json['average']),
        minimum = BigInt.parse(json['minimum']);
}

////////////

class TransferStatsDownUp {
  TransferStats down;
  TransferStats up;

  TransferStatsDownUp({
    required this.down,
    required this.up,
  });

  Map<String, dynamic> toJson() {
    return {
      'down': down.toJson(),
      'up': up.toJson(),
    };
  }

  TransferStatsDownUp.fromJson(dynamic json)
      : down = TransferStats.fromJson(json['down']),
        up = TransferStats.fromJson(json['up']);
}

////////////

class RPCStats {
  int messagesSent;
  int messagesRcvd;
  int questionsInFlight;
  Timestamp? lastQuestion;
  Timestamp? lastSeenTs;
  Timestamp? firstConsecutiveSeenTs;
  int recentLostAnswers;
  int failedToSend;

  RPCStats({
    required this.messagesSent,
    required this.messagesRcvd,
    required this.questionsInFlight,
    required this.lastQuestion,
    required this.lastSeenTs,
    required this.firstConsecutiveSeenTs,
    required this.recentLostAnswers,
    required this.failedToSend,
  });

  Map<String, dynamic> toJson() {
    return {
      'messages_sent': messagesSent,
      'messages_rcvd': messagesRcvd,
      'questions_in_flight': questionsInFlight,
      'last_question': lastQuestion?.toJson(),
      'last_seen_ts': lastSeenTs?.toJson(),
      'first_consecutive_seen_ts': firstConsecutiveSeenTs?.toJson(),
      'recent_lost_answers': recentLostAnswers,
      'failed_to_send': failedToSend,
    };
  }

  RPCStats.fromJson(dynamic json)
      : messagesSent = json['messages_sent'],
        messagesRcvd = json['messages_rcvd'],
        questionsInFlight = json['questions_in_flight'],
        lastQuestion = json['last_question'] != null
            ? Timestamp.fromJson(json['last_question'])
            : null,
        lastSeenTs = json['last_seen_ts'] != null
            ? Timestamp.fromJson(json['last_seen_ts'])
            : null,
        firstConsecutiveSeenTs = json['first_consecutive_seen_ts'] != null
            ? Timestamp.fromJson(json['first_consecutive_seen_ts'])
            : null,
        recentLostAnswers = json['recent_lost_answers'],
        failedToSend = json['failed_to_send'];
}

////////////

class PeerStats {
  Timestamp timeAdded;
  RPCStats rpcStats;
  LatencyStats? latency;
  TransferStatsDownUp transfer;

  PeerStats({
    required this.timeAdded,
    required this.rpcStats,
    required this.latency,
    required this.transfer,
  });

  Map<String, dynamic> toJson() {
    return {
      'time_added': timeAdded.toJson(),
      'rpc_stats': rpcStats.toJson(),
      'latency': latency?.toJson(),
      'transfer': transfer.toJson(),
    };
  }

  PeerStats.fromJson(dynamic json)
      : timeAdded = Timestamp.fromJson(json['time_added']),
        rpcStats = RPCStats.fromJson(json['rpc_stats']),
        latency = json['latency'] != null
            ? LatencyStats.fromJson(json['latency'])
            : null,
        transfer = TransferStatsDownUp.fromJson(json['transfer']);
}

////////////

class PeerTableData {
  List<TypedKey> nodeIds;
  String peerAddress;
  PeerStats peerStats;

  PeerTableData({
    required this.nodeIds,
    required this.peerAddress,
    required this.peerStats,
  });

  Map<String, dynamic> toJson() {
    return {
      'node_ids': nodeIds.map((p) => p.toJson()).toList(),
      'peer_address': peerAddress,
      'peer_stats': peerStats.toJson(),
    };
  }

  PeerTableData.fromJson(dynamic json)
      : nodeIds = List<TypedKey>.from(
            json['node_ids'].map((j) => TypedKey.fromJson(j))),
        peerAddress = json['peer_address'],
        peerStats = PeerStats.fromJson(json['peer_stats']);
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
              sender: json["sender"], message: json["message"], id: json["id"]);
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
  final String id;

  //
  VeilidAppCall({
    required this.sender,
    required this.message,
    required this.id,
  });

  @override
  Map<String, dynamic> toJson() {
    return {
      'kind': "AppMessage",
      'sender': sender,
      'message': base64UrlNoPadEncode(message),
      'id': id,
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
