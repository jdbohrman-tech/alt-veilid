import 'dart:convert';
import 'dart:typed_data';

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

abstract class EncodedString {
  late String contents;
  EncodedString(String s) {
    validate(s);
    contents = s;
  }
  EncodedString.encode(List<int> b) {
    var s = base64UrlNoPadEncode(b);
    validate(s);
    contents = s;
  }

  int encodedLength();
  int decodedLength();
  void validate(String s) {
    var d = base64UrlNoPadDecode(s);
    if (d.length != decodedLength()) {
      throw Exception("length ${s.length} should be ${encodedLength()}");
    }
  }

  Uint8List decode() {
    return base64UrlNoPadDecode(contents);
  }

  @override
  String toString() {
    return contents;
  }

  static T fromString<T extends EncodedString>(String s) {
    switch (T) {
      case FixedEncodedString32:
        return FixedEncodedString32(s) as T;
      case FixedEncodedString43:
        return FixedEncodedString43(s) as T;
      case FixedEncodedString86:
        return FixedEncodedString86(s) as T;
      default:
        throw UnimplementedError();
    }
  }
}

class FixedEncodedString32 extends EncodedString {
  FixedEncodedString32(String s) : super(s);
  @override
  int encodedLength() {
    return 32;
  }

  @override
  int decodedLength() {
    return 24;
  }

  String get json {
    return toString();
  }

  FixedEncodedString32.fromJson(dynamic json) : this(json as String);
}

class FixedEncodedString43 extends EncodedString {
  FixedEncodedString43(String s) : super(s);
  @override
  int encodedLength() {
    return 43;
  }

  @override
  int decodedLength() {
    return 32;
  }

  String get json {
    return toString();
  }

  FixedEncodedString43.fromJson(dynamic json) : this(json as String);
}

class FixedEncodedString86 extends EncodedString {
  FixedEncodedString86(String s) : super(s);
  @override
  int encodedLength() {
    return 86;
  }

  @override
  int decodedLength() {
    return 64;
  }

  String get json {
    return toString();
  }

  FixedEncodedString86.fromJson(dynamic json) : this(json as String);
}
