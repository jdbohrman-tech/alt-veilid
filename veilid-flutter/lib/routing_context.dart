import 'dart:async';
import 'dart:typed_data';
import 'dart:convert';

import 'package:change_case/change_case.dart';

import 'base64url_no_pad.dart';
import 'veilid.dart';

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
              "Invalid VeilidAPIException type: ${json['kind']}");
        }
    }
  }
  Map<String, dynamic> get json;
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
  Map<String, dynamic> get json {
    return {
      'kind': "DFLT",
      'o_cnt': oCnt,
    };
  }
}

class DHTSchemaMember {
  Key mKey;
  int mCnt;

  DHTSchemaMember({
    required this.mKey,
    required this.mCnt,
  }) {
    if (mCnt < 0 || mCnt > 65535) {
      throw VeilidAPIExceptionInvalidArgument(
          "value out of range", "mCnt", mCnt.toString());
    }
  }

  Map<String, dynamic> get json {
    return {
      'm_key': mKey,
      'm_cnt': mCnt,
    };
  }

  DHTSchemaMember.fromJson(dynamic json)
      : mKey = json['m_key'],
        mCnt = json['m_cnt'];
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
  Map<String, dynamic> get json {
    return {
      'kind': "SMPL",
      'o_cnt': oCnt,
      'members': members.map((p) => p.json).toList(),
    };
  }
}

//////////////////////////////////////
/// DHTRecordDescriptor

class DHTRecordDescriptor {
  TypedKey key;
  Key owner;
  Key? ownerSecret;
  DHTSchema schema;

  DHTRecordDescriptor({
    required this.key,
    required this.owner,
    this.ownerSecret,
    required this.schema,
  });

  Map<String, dynamic> get json {
    return {
      'key': key.toString(),
      'owner': owner,
      'owner_secret': ownerSecret,
      'schema': schema.json,
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

  List<dynamic> get json {
    return [low, high];
  }
}

//////////////////////////////////////
/// ValueData

class ValueData {
  final int seq;
  final Uint8List data;
  final Key writer;

  ValueData({
    required this.seq,
    required this.data,
    required this.writer,
  });

  ValueData.fromJson(dynamic json)
      : seq = json['seq'],
        data = base64UrlNoPadDecode(json['data']),
        writer = json['writer'];

  Map<String, dynamic> get json {
    return {'seq': seq, 'data': base64UrlNoPadEncode(data), 'writer': writer};
  }
}

/// Stability

enum Stability {
  lowLatency,
  reliable,
}

extension StabilityExt on Stability {
  String get json {
    return name.toPascalCase();
  }
}

Stability stabilityFromJson(String j) {
  return Stability.values.byName(j.toCamelCase());
}

//////////////////////////////////////
/// Sequencing

enum Sequencing {
  noPreference,
  preferOrdered,
  ensureOrdered,
}

extension SequencingExt on Sequencing {
  String get json {
    return name.toPascalCase();
  }
}

Sequencing sequencingFromJson(String j) {
  return Sequencing.values.byName(j.toCamelCase());
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

  Map<String, dynamic> get json {
    return {'route_id': routeId, 'blob': base64UrlNoPadEncode(blob)};
  }
}

//////////////////////////////////////
/// VeilidRoutingContext

abstract class VeilidRoutingContext {
  // Modifiers
  VeilidRoutingContext withPrivacy();
  VeilidRoutingContext withCustomPrivacy(Stability stability);
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
  Future<Timestamp> watchDHTValues(
      TypedKey key, ValueSubkeyRange subkeys, Timestamp expiration, int count);
  Future<bool> cancelDHTWatch(TypedKey key, ValueSubkeyRange subkeys);
}
