import 'dart:async';
import 'dart:typed_data';

import 'package:charcode/charcode.dart';
import 'package:equatable/equatable.dart';
import 'package:freezed_annotation/freezed_annotation.dart';

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

@immutable
class Typed<V extends EncodedString> extends Equatable {
  final CryptoKind kind;
  final V value;
  @override
  List<Object> get props => [kind, value];

  const Typed({required this.kind, required this.value});

  @override
  String toString() {
    return "${cryptoKindToString(kind)}:$value";
  }

  factory Typed.fromString(String s) {
    final parts = s.split(":");
    if (parts.length < 2 || parts[0].codeUnits.length != 4) {
      throw const FormatException("malformed string");
    }
    final kind = cryptoKindFromString(parts[0]);
    final value = EncodedString.fromString<V>(parts.sublist(1).join(":"));
    return Typed(kind: kind, value: value);
  }

  String toJson() => toString();
  factory Typed.fromJson(dynamic json) => Typed.fromString(json as String);
}

@immutable
class KeyPair extends Equatable {
  final PublicKey key;
  final PublicKey secret;
  @override
  List<Object> get props => [key, secret];

  const KeyPair({required this.key, required this.secret});

  @override
  String toString() {
    return "${key.toString()}:${secret.toString()}";
  }

  factory KeyPair.fromString(String s) {
    final parts = s.split(":");
    if (parts.length != 2 ||
        parts[0].codeUnits.length != 43 ||
        parts[1].codeUnits.length != 43) {
      throw const FormatException("malformed string");
    }
    final key = PublicKey.fromString(parts[0]);
    final secret = PublicKey.fromString(parts[1]);
    return KeyPair(key: key, secret: secret);
  }

  String toJson() => toString();
  factory KeyPair.fromJson(dynamic json) => KeyPair.fromString(json as String);
}

@immutable
class TypedKeyPair extends Equatable {
  final CryptoKind kind;
  final PublicKey key;
  final PublicKey secret;
  @override
  List<Object> get props => [kind, key, secret];

  const TypedKeyPair(
      {required this.kind, required this.key, required this.secret});

  @override
  String toString() =>
      "${cryptoKindToString(kind)}:${key.toString()}:${secret.toString()}";

  factory TypedKeyPair.fromString(String s) {
    final parts = s.split(":");
    if (parts.length != 3 ||
        parts[0].codeUnits.length != 4 ||
        parts[1].codeUnits.length != 43 ||
        parts[2].codeUnits.length != 43) {
      throw VeilidAPIExceptionInvalidArgument("malformed string", "s", s);
    }
    final kind = cryptoKindFromString(parts[0]);
    final key = PublicKey.fromString(parts[1]);
    final secret = PublicKey.fromString(parts[2]);
    return TypedKeyPair(kind: kind, key: key, secret: secret);
  }

  String toJson() => toString();
  factory TypedKeyPair.fromJson(dynamic json) =>
      TypedKeyPair.fromString(json as String);
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
typedef TypedSecret = Typed<SecretKey>;

typedef TypedSignature = Typed<Signature>;

//////////////////////////////////////
/// VeilidCryptoSystem

abstract class VeilidCryptoSystem {
  CryptoKind kind();
  Future<SharedSecret> cachedDH(PublicKey key, SecretKey secret);
  Future<SharedSecret> computeDH(PublicKey key, SecretKey secret);
  Future<Uint8List> randomBytes(int len);
  Future<int> defaultSaltLength();
  Future<String> hashPassword(Uint8List password, Uint8List salt);
  Future<bool> verifyPassword(Uint8List password, String passwordHash);
  Future<SharedSecret> deriveSharedSecret(Uint8List password, Uint8List salt);
  Future<Nonce> randomNonce();
  Future<SharedSecret> randomSharedSecret();
  Future<KeyPair> generateKeyPair();
  Future<HashDigest> generateHash(Uint8List data);
  //Future<HashDigest> generateHashReader(Stream<List<int>> reader);
  Future<bool> validateKeyPair(PublicKey key, SecretKey secret);
  Future<bool> validateHash(Uint8List data, HashDigest hash);
  //Future<bool> validateHashReader(Stream<List<int>> reader, HashDigest hash);
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
