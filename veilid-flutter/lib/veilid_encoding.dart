import 'dart:convert';
import 'dart:typed_data';

import 'package:equatable/equatable.dart';
import 'package:freezed_annotation/freezed_annotation.dart';

String base64UrlNoPadEncode(List<int> bytes) {
  var x = base64Url.encode(bytes);
  while (x.endsWith('=')) {
    x = x.substring(0, x.length - 1);
  }
  return x;
}

Uint8List base64UrlNoPadDecode(String source) {
  source = base64.normalize(source);
  return base64.decode(source);
}

Uint8List base64UrlNoPadDecodeDynamic(dynamic source) {
  source = source as String;
  source = base64.normalize(source);
  return base64.decode(source);
}

@immutable
abstract class EncodedString extends Equatable {
  final String contents;
  @override
  List<Object> get props => [contents];

  const EncodedString(String s) : contents = s;

  Uint8List decode() {
    return base64UrlNoPadDecode(contents);
  }

  @override
  String toString() => contents;

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
  const FixedEncodedString32._(String s) : super(s);
  static int encodedLength() {
    return 32;
  }

  static int decodedLength() {
    return 24;
  }

  factory FixedEncodedString32.fromString(String s) {
    var d = base64UrlNoPadDecode(s);
    if (d.length != decodedLength()) {
      throw Exception("length ${s.length} should be ${encodedLength()}");
    }
    return FixedEncodedString32._(s);
  }

  String toJson() => toString();
  factory FixedEncodedString32.fromJson(dynamic json) =>
      FixedEncodedString32.fromString(json as String);
}

@immutable
class FixedEncodedString43 extends EncodedString {
  const FixedEncodedString43._(String s) : super(s);
  static int encodedLength() {
    return 43;
  }

  static int decodedLength() {
    return 32;
  }

  factory FixedEncodedString43.fromString(String s) {
    var d = base64UrlNoPadDecode(s);
    if (d.length != decodedLength()) {
      throw Exception("length ${s.length} should be ${encodedLength()}");
    }
    return FixedEncodedString43._(s);
  }

  String toJson() => toString();
  factory FixedEncodedString43.fromJson(dynamic json) =>
      FixedEncodedString43.fromString(json as String);
}

@immutable
class FixedEncodedString86 extends EncodedString {
  const FixedEncodedString86._(String s) : super(s);
  static int encodedLength() {
    return 86;
  }

  static int decodedLength() {
    return 64;
  }

  String toJson() {
    return toString();
  }

  factory FixedEncodedString86.fromString(String s) {
    var d = base64UrlNoPadDecode(s);
    if (d.length != decodedLength()) {
      throw Exception("length ${s.length} should be ${encodedLength()}");
    }
    return FixedEncodedString86._(s);
  }

  factory FixedEncodedString86.fromJson(dynamic json) =>
      FixedEncodedString86.fromString(json as String);
}
