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
  late PublicKey key;
  late PublicKey secret;
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
    key = PublicKey(parts[0]);
    secret = PublicKey(parts[1]);
  }

  String get json {
    return toString();
  }

  KeyPair.fromJson(dynamic json) : this.fromString(json as String);
}

class TypedKeyPair {
  late CryptoKind kind;
  late PublicKey key;
  late PublicKey secret;
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
    key = PublicKey(parts[1]);
    secret = PublicKey(parts[2]);
  }

  String get json {
    return toString();
  }

  TypedKeyPair.fromJson(dynamic json) : this.fromString(json as String);
}

typedef CryptoKey = FixedEncodedString43;
typedef Signature = FixedEncodedString86;
typedef Nonce = FixedEncodedString32;

typedef PublicKey = CryptoKey;
typedef SecretKey = CryptoKey;
typedef HashDigest = CryptoKey;
typedef SharedSecret = CryptoKey;
typedef CryptoKeyDistance = CryptoKey;

typedef TypedKey = Typed<CryptoKey>;
typedef TypedSignature = Typed<Signature>;

//////////////////////////////////////
/// VeilidCryptoSystem

abstract class VeilidCryptoSystem {
  CryptoKind kind();
  Future<SharedSecret> cachedDH(PublicKey key, SecretKey secret);
  Future<SharedSecret> computeDH(PublicKey key, SecretKey secret);
  Future<Nonce> randomNonce();
  Future<SharedSecret> randomSharedSecret();
  Future<KeyPair> generateKeyPair();
  Future<HashDigest> generateHash(Uint8List data);
  Future<HashDigest> generateHashReader(Stream<List<int>> reader);
  Future<bool> validateKeyPair(PublicKey key, SecretKey secret);
  Future<bool> validateHash(Uint8List data, HashDigest hash);
  Future<bool> validateHashReader(Stream<List<int>> reader, HashDigest hash);
  Future<CryptoKeyDistance> distance(CryptoKey key1, CryptoKey key2);
  Future<Signature> sign(PublicKey key, SecretKey secret, Uint8List data);
  Future<void> verify(PublicKey key, Uint8List data, Signature signature);
  Future<int> aeadOverhead();
  Future<Uint8List> decryptAead(Uint8List body, Nonce nonce,
      SharedSecret sharedSecret, Uint8List? associatedData);
  Future<Uint8List> encryptAead(Uint8List body, Nonce nonce,
      SharedSecret sharedSecret, Uint8List? associatedData);
  Future<Uint8List> cryptNoAuth(
      Uint8List body, Nonce nonce, SharedSecret sharedSecret);
}
