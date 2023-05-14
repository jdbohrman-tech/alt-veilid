import 'dart:async';
import 'dart:typed_data';

import 'package:charcode/charcode.dart';

import 'veilid_encoding.dart';
import 'veilid.dart';

//////////////////////////////////////
/// CryptoKind

typedef CryptoKind = int;
const CryptoKind cryptoKindVLD0 =
    $V << 0 | $L << 8 | $D << 16 | $0 << 24; // "VLD0"
const CryptoKind cryptoKindNONE =
    $N << 0 | $O << 8 | $N << 16 | $E << 24; // "NONE"

String cryptoKindToString(CryptoKind kind) {
  return "${String.fromCharCode(kind & 0xFF)}${String.fromCharCode((kind >> 8) & 0xFF)}${String.fromCharCode((kind >> 16) & 0xFF)}${String.fromCharCode((kind >> 24) & 0xFF)}";
}

CryptoKind cryptoKindFromString(String s) {
  if (s.codeUnits.length != 4) {
    throw const FormatException("malformed string");
  }
  CryptoKind kind = s.codeUnits[0] |
      s.codeUnits[1] << 8 |
      s.codeUnits[2] << 16 |
      s.codeUnits[3] << 24;
  return kind;
}

//////////////////////////////////////
/// Types

class Typed<V extends EncodedString> {
  late CryptoKind kind;
  late V value;
  Typed({required this.kind, required this.value});

  @override
  String toString() {
    return "${cryptoKindToString(kind)}:$value";
  }

  Typed.fromString(String s) {
    var parts = s.split(":");
    if (parts.length < 2 || parts[0].codeUnits.length != 4) {
      throw const FormatException("malformed string");
    }
    kind = parts[0].codeUnits[0] |
        parts[0].codeUnits[1] << 8 |
        parts[0].codeUnits[2] << 16 |
        parts[0].codeUnits[3] << 24;
    value = EncodedString.fromString<V>(parts.sublist(1).join(":"));
  }

  String get json {
    return toString();
  }

  Typed.fromJson(dynamic json) : this.fromString(json as String);
}

class KeyPair {
  late Key key;
  late Key secret;
  KeyPair({required this.key, required this.secret});

  @override
  String toString() {
    return "${key.toString()}:${secret.toString()}";
  }

  KeyPair.fromString(String s) {
    var parts = s.split(":");
    if (parts.length != 2 ||
        parts[0].codeUnits.length != 43 ||
        parts[1].codeUnits.length != 43) {
      throw const FormatException("malformed string");
    }
    key = Key(parts[0]);
    secret = Key(parts[1]);
  }

  String get json {
    return toString();
  }

  KeyPair.fromJson(dynamic json) : this.fromString(json as String);
}

class TypedKeyPair {
  late CryptoKind kind;
  late Key key;
  late Key secret;
  TypedKeyPair({required this.kind, required this.key, required this.secret});

  @override
  String toString() {
    return "${cryptoKindToString(kind)}:${key.toString()}:${secret.toString()}";
  }

  TypedKeyPair.fromString(String s) {
    var parts = s.split(":");
    if (parts.length != 3 ||
        parts[0].codeUnits.length != 4 ||
        parts[1].codeUnits.length != 43 ||
        parts[2].codeUnits.length != 43) {
      throw VeilidAPIExceptionInvalidArgument("malformed string", "s", s);
    }
    kind = cryptoKindFromString(parts[0]);
    key = Key(parts[1]);
    secret = Key(parts[2]);
  }

  String get json {
    return toString();
  }

  TypedKeyPair.fromJson(dynamic json) : this.fromString(json as String);
}

typedef Key = FixedEncodedString43;
typedef Signature = FixedEncodedString86;
typedef Nonce = FixedEncodedString32;

typedef TypedKey = Typed<Key>;
typedef TypedSignature = Typed<Signature>;

//////////////////////////////////////
/// VeilidCryptoSystem

abstract class VeilidCryptoSystem {
  CryptoKind kind();
  Key cachedDH(Key key, Key secret);
  Key computeDH(Key key, Key secret);
  Nonce randomNonce();
  Key randomSharedSecret();
  KeyPair generateKeyPair();
  Key generateHash(Uint8List data);
  Key generateHashReader(Stream<List<int>> reader);
  bool validateKeyPair(Key key, Key secret);
  bool validateHash(Uint8List data, Key hash);
  bool validateHashReader(Stream<List<int>> reader, Key hash);
  Key distance(Key key1, Key key2);
  Signature sign(Key key, Key secret, Uint8List data);
  void verify(Key key, Uint8List data, Signature signature);
  BigInt aeadOverhead();
  Uint8List decryptAead(
      Uint8List body, Nonce nonce, Key sharedSecret, Uint8List? associatedData);
  Uint8List encryptAead(
      Uint8List body, Nonce nonce, Key sharedSecret, Uint8List? associatedData);
  Uint8List cryptNoAuth(Uint8List body, Nonce nonce, Key sharedSecret);
}
