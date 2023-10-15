import 'dart:convert';

import 'package:equatable/equatable.dart';
import 'package:flutter/foundation.dart';
import 'package:freezed_annotation/freezed_annotation.dart';

import 'veilid_stub.dart'
    if (dart.library.io) 'veilid_ffi.dart'
    if (dart.library.js) 'veilid_js.dart';

String base64UrlNoPadEncode(List<int> bytes) {
  var x = base64Url.encode(bytes);
  while (x.endsWith('=')) {
    x = x.substring(0, x.length - 1);
  }
  return x;
}

Uint8List base64UrlNoPadDecode(String source) {
  source = base64Url.normalize(source);
  return base64Url.decode(source);
}

Uint8List base64UrlNoPadDecodeDynamic(dynamic source) =>
    base64UrlNoPadDecode(source as String);

class Uint8ListJsonConverter implements JsonConverter<Uint8List, dynamic> {
  const Uint8ListJsonConverter() : _jsIsArray = false;
  const Uint8ListJsonConverter.jsIsArray() : _jsIsArray = true;

  final bool _jsIsArray;

  @override
  Uint8List fromJson(dynamic json) => kIsWeb && _jsIsArray
      ? convertUint8ListFromJson(json)
      : base64UrlNoPadDecode(json as String);
  @override
  dynamic toJson(Uint8List data) => kIsWeb && _jsIsArray
      ? convertUint8ListToJson(data)
      : base64UrlNoPadEncode(data);
}

@immutable
abstract class EncodedString extends Equatable {
  const EncodedString(String s) : contents = s;
  final String contents;
  @override
  List<Object> get props => [contents];

  Uint8List decode() => base64UrlNoPadDecode(contents);

  @override
  String toString() => contents;

  static T fromBytes<T extends EncodedString>(Uint8List bytes) {
    switch (T) {
      case FixedEncodedString32:
        return FixedEncodedString32.fromBytes(bytes) as T;
      case FixedEncodedString43:
        return FixedEncodedString43.fromBytes(bytes) as T;
      case FixedEncodedString86:
        return FixedEncodedString86.fromBytes(bytes) as T;
      default:
        throw UnimplementedError();
    }
  }

  static T fromString<T extends EncodedString>(String s) {
    switch (T) {
      case FixedEncodedString32:
        return FixedEncodedString32.fromString(s) as T;
      case FixedEncodedString43:
        return FixedEncodedString43.fromString(s) as T;
      case FixedEncodedString86:
        return FixedEncodedString86.fromString(s) as T;
      default:
        throw UnimplementedError();
    }
  }
}

@immutable
class FixedEncodedString32 extends EncodedString {
  factory FixedEncodedString32.fromBytes(Uint8List bytes) {
    if (bytes.length != decodedLength()) {
      throw Exception('length ${bytes.length} should be ${decodedLength()}');
    }
    return FixedEncodedString32._(base64UrlNoPadEncode(bytes));
  }

  factory FixedEncodedString32.fromString(String s) {
    final d = base64UrlNoPadDecode(s);
    if (d.length != decodedLength()) {
      throw Exception('length ${s.length} should be ${encodedLength()}');
    }
    return FixedEncodedString32._(s);
  }
  factory FixedEncodedString32.fromJson(dynamic json) =>
      FixedEncodedString32.fromString(json as String);
  const FixedEncodedString32._(super.s);
  static int encodedLength() => 32;

  static int decodedLength() => 24;

  String toJson() => toString();
}

@immutable
class FixedEncodedString43 extends EncodedString {
  factory FixedEncodedString43.fromBytes(Uint8List bytes) {
    if (bytes.length != decodedLength()) {
      throw Exception('length ${bytes.length} should be ${decodedLength()}');
    }
    return FixedEncodedString43._(base64UrlNoPadEncode(bytes));
  }

  factory FixedEncodedString43.fromString(String s) {
    final d = base64UrlNoPadDecode(s);
    if (d.length != decodedLength()) {
      throw Exception('length ${s.length} should be ${encodedLength()}');
    }
    return FixedEncodedString43._(s);
  }
  factory FixedEncodedString43.fromJson(dynamic json) =>
      FixedEncodedString43.fromString(json as String);
  const FixedEncodedString43._(super.s);
  static int encodedLength() => 43;

  static int decodedLength() => 32;

  String toJson() => toString();
}

@immutable
class FixedEncodedString86 extends EncodedString {
  factory FixedEncodedString86.fromBytes(Uint8List bytes) {
    if (bytes.length != decodedLength()) {
      throw Exception('length ${bytes.length} should be ${decodedLength()}');
    }
    return FixedEncodedString86._(base64UrlNoPadEncode(bytes));
  }

  factory FixedEncodedString86.fromString(String s) {
    final d = base64UrlNoPadDecode(s);
    if (d.length != decodedLength()) {
      throw Exception('length ${s.length} should be ${encodedLength()}');
    }
    return FixedEncodedString86._(s);
  }

  factory FixedEncodedString86.fromJson(dynamic json) =>
      FixedEncodedString86.fromString(json as String);
  const FixedEncodedString86._(super.s);
  static int encodedLength() => 86;

  static int decodedLength() => 64;

  String toJson() => toString();
}
