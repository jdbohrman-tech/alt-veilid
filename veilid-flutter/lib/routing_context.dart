import 'dart:async';
import 'dart:typed_data';

import 'package:change_case/change_case.dart';
import 'package:freezed_annotation/freezed_annotation.dart';

import 'veilid_encoding.dart';
import 'veilid.dart';

part 'routing_context.freezed.dart';
part 'routing_context.g.dart';

//////////////////////////////////////

//////////////////////////////////////
/// DHT Schema

abstract class DHTSchema {
  factory DHTSchema.fromJson(dynamic json) {
    switch (json["kind"]) {
      case "DFLT":
        {
          return DHTSchemaDFLT(oCnt: json["o_cnt"]);
        }
      case "SMPL":
        {
          return DHTSchemaSMPL(
              oCnt: json["o_cnt"],
              members: List<DHTSchemaMember>.from(
                  json['members'].map((j) => DHTSchemaMember.fromJson(j))));
        }
      default:
        {
          throw VeilidAPIExceptionInternal(
              "Invalid DHTSchema type: ${json['kind']}");
        }
    }
  }
  Map<String, dynamic> toJson();
}

class DHTSchemaDFLT implements DHTSchema {
  final int oCnt;
  //
  DHTSchemaDFLT({
    required this.oCnt,
  }) {
    if (oCnt < 0 || oCnt > 65535) {
      throw VeilidAPIExceptionInvalidArgument(
          "value out of range", "oCnt", oCnt.toString());
    }
  }

  @override
  Map<String, dynamic> toJson() {
    return {
      'kind': "DFLT",
      'o_cnt': oCnt,
    };
  }
}

@freezed
class DHTSchemaMember with _$DHTSchemaMember {
  @Assert('mCnt >= 0 && mCnt <= 65535', 'value out of range')
  const factory DHTSchemaMember({
    required PublicKey mKey,
    required int mCnt,
  }) = _DHTSchemaMember;

  factory DHTSchemaMember.fromJson(Map<String, dynamic> json) =>
      _$DHTSchemaMemberFromJson(json);
}

class DHTSchemaSMPL implements DHTSchema {
  final int oCnt;
  final List<DHTSchemaMember> members;
  //
  DHTSchemaSMPL({
    required this.oCnt,
    required this.members,
  }) {
    if (oCnt < 0 || oCnt > 65535) {
      throw VeilidAPIExceptionInvalidArgument(
          "value out of range", "oCnt", oCnt.toString());
    }
  }
  @override
  Map<String, dynamic> toJson() {
    return {
      'kind': "SMPL",
      'o_cnt': oCnt,
      'members': members.map((p) => p.toJson()).toList(),
    };
  }
}

//////////////////////////////////////
/// DHTRecordDescriptor

class DHTRecordDescriptor {
  TypedKey key;
  PublicKey owner;
  PublicKey? ownerSecret;
  DHTSchema schema;

  DHTRecordDescriptor({
    required this.key,
    required this.owner,
    this.ownerSecret,
    required this.schema,
  });

  Map<String, dynamic> toJson() {
    return {
      'key': key.toString(),
      'owner': owner,
      'owner_secret': ownerSecret,
      'schema': schema.toJson(),
    };
  }

  DHTRecordDescriptor.fromJson(dynamic json)
      : key = TypedKey.fromString(json['key']),
        owner = json['owner'],
        ownerSecret = json['owner_secret'],
        schema = DHTSchema.fromJson(json['schema']);
}

//////////////////////////////////////
/// ValueSubkeyRange

class ValueSubkeyRange {
  final int low;
  final int high;

  ValueSubkeyRange({
    required this.low,
    required this.high,
  }) {
    if (low < 0 || low > high) {
      throw VeilidAPIExceptionInvalidArgument(
          "invalid range", "low", low.toString());
    }
    if (high < 0) {
      throw VeilidAPIExceptionInvalidArgument(
          "invalid range", "high", high.toString());
    }
  }

  ValueSubkeyRange.fromJson(dynamic json)
      : low = json[0],
        high = json[1] {
    if ((json as List<int>).length != 2) {
      throw VeilidAPIExceptionInvalidArgument(
          "not a pair of integers", "json", json.toString());
    }
  }

  List<dynamic> toJson() {
    return [low, high];
  }
}

//////////////////////////////////////
/// ValueData

class ValueData {
  final int seq;
  final Uint8List data;
  final PublicKey writer;

  ValueData({
    required this.seq,
    required this.data,
    required this.writer,
  });

  ValueData.fromJson(dynamic json)
      : seq = json['seq'],
        data = base64UrlNoPadDecode(json['data']),
        writer = json['writer'];

  Map<String, dynamic> toJson() {
    return {'seq': seq, 'data': base64UrlNoPadEncode(data), 'writer': writer};
  }
}

//////////////////////////////////////
/// Stability

enum Stability {
  lowLatency,
  reliable;

  String toJson() {
    return name.toPascalCase();
  }

  factory Stability.fromJson(String j) {
    return Stability.values.byName(j.toCamelCase());
  }
}

//////////////////////////////////////
/// Sequencing

enum Sequencing {
  noPreference,
  preferOrdered,
  ensureOrdered;

  String toJson() {
    return name.toPascalCase();
  }

  factory Sequencing.fromJson(String j) {
    return Sequencing.values.byName(j.toCamelCase());
  }
}

//////////////////////////////////////
/// SafetySelection

abstract class SafetySelection {
  factory SafetySelection.fromJson(dynamic json) {
    var m = json as Map<String, dynamic>;
    if (m.containsKey("Unsafe")) {
      return SafetySelectionUnsafe(
          sequencing: Sequencing.fromJson(m["Unsafe"]));
    } else if (m.containsKey("Safe")) {
      return SafetySelectionSafe(safetySpec: SafetySpec.fromJson(m["Safe"]));
    } else {
      throw VeilidAPIExceptionInternal("Invalid SafetySelection");
    }
  }
  Map<String, dynamic> toJson();
}

class SafetySelectionUnsafe implements SafetySelection {
  final Sequencing sequencing;
  //
  SafetySelectionUnsafe({
    required this.sequencing,
  });

  @override
  Map<String, dynamic> toJson() {
    return {'Unsafe': sequencing.toJson()};
  }
}

class SafetySelectionSafe implements SafetySelection {
  final SafetySpec safetySpec;
  //
  SafetySelectionSafe({
    required this.safetySpec,
  });

  @override
  Map<String, dynamic> toJson() {
    return {'Safe': safetySpec.toJson()};
  }
}

/// Options for safety routes (sender privacy)
class SafetySpec {
  final String? preferredRoute;
  final int hopCount;
  final Stability stability;
  final Sequencing sequencing;
  //
  SafetySpec({
    this.preferredRoute,
    required this.hopCount,
    required this.stability,
    required this.sequencing,
  });

  SafetySpec.fromJson(dynamic json)
      : preferredRoute = json['preferred_route'],
        hopCount = json['hop_count'],
        stability = Stability.fromJson(json['stability']),
        sequencing = Sequencing.fromJson(json['sequencing']);

  Map<String, dynamic> toJson() {
    return {
      'preferred_route': preferredRoute,
      'hop_count': hopCount,
      'stability': stability.toJson(),
      'sequencing': sequencing.toJson()
    };
  }
}

//////////////////////////////////////
/// RouteBlob
class RouteBlob {
  final String routeId;
  final Uint8List blob;

  RouteBlob(this.routeId, this.blob);

  RouteBlob.fromJson(dynamic json)
      : routeId = json['route_id'],
        blob = base64UrlNoPadDecode(json['blob']);

  Map<String, dynamic> toJson() {
    return {'route_id': routeId, 'blob': base64UrlNoPadEncode(blob)};
  }
}

//////////////////////////////////////
/// VeilidRoutingContext

abstract class VeilidRoutingContext {
  // Modifiers
  VeilidRoutingContext withPrivacy();
  VeilidRoutingContext withCustomPrivacy(SafetySelection safetySelection);
  VeilidRoutingContext withSequencing(Sequencing sequencing);

  // App call/message
  Future<Uint8List> appCall(String target, Uint8List request);
  Future<void> appMessage(String target, Uint8List message);

  // DHT Operations
  Future<DHTRecordDescriptor> createDHTRecord(
      CryptoKind kind, DHTSchema schema);
  Future<DHTRecordDescriptor> openDHTRecord(TypedKey key, KeyPair? writer);
  Future<void> closeDHTRecord(TypedKey key);
  Future<void> deleteDHTRecord(TypedKey key);
  Future<ValueData?> getDHTValue(TypedKey key, int subkey, bool forceRefresh);
  Future<ValueData?> setDHTValue(TypedKey key, int subkey, Uint8List data);
  Future<Timestamp> watchDHTValues(TypedKey key, List<ValueSubkeyRange> subkeys,
      Timestamp expiration, int count);
  Future<bool> cancelDHTWatch(TypedKey key, List<ValueSubkeyRange> subkeys);
}
