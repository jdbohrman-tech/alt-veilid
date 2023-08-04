import 'dart:async';
import 'dart:convert';
import 'dart:typed_data';

import 'package:charcode/charcode.dart';
import 'package:equatable/equatable.dart';
import 'package:freezed_annotation/freezed_annotation.dart';

import 'veilid.dart';

//////////////////////////////////////
/// CryptoKind

typedef CryptoKind = int;
const CryptoKind cryptoKindVLD0 =
    $V << 0 | $L << 8 | $D << 16 | $0 << 24; // "VLD0"
const CryptoKind cryptoKindNONE =
    $N << 0 | $O << 8 | $N << 16 | $E << 24; // "NONE"

String cryptoKindToString(CryptoKind kind) =>
    cryptoKindToBytes(kind).map(String.fromCharCode).join();

const CryptoKind bestCryptoKind = cryptoKindVLD0;

Uint8List cryptoKindToBytes(CryptoKind kind) {
  final b = Uint8List(4);
  ByteData.sublistView(b).setUint32(0, kind);
  return b;
}

CryptoKind cryptoKindFromString(String s) {
  if (s.codeUnits.length != 4) {
    throw const FormatException('malformed string');
  }
  final kind =
      ByteData.sublistView(Uint8List.fromList(s.codeUnits)).getUint32(0);
  return kind;
}

//////////////////////////////////////
/// Types

@immutable
class Typed<V extends EncodedString> extends Equatable {
  const Typed({required this.kind, required this.value});

  factory Typed.fromString(String s) {
    final parts = s.split(':');
    if (parts.length < 2 || parts[0].codeUnits.length != 4) {
      throw const FormatException('malformed string');
    }
    final kind = cryptoKindFromString(parts[0]);
    final value = EncodedString.fromString<V>(parts.sublist(1).join(':'));
    return Typed(kind: kind, value: value);
  }
  factory Typed.fromJson(dynamic json) => Typed.fromString(json as String);
  final CryptoKind kind;
  final V value;
  @override
  List<Object> get props => [kind, value];

  @override
  String toString() => '${cryptoKindToString(kind)}:$value';

  Uint8List decode() {
    final b = BytesBuilder()
      ..add(cryptoKindToBytes(kind))
      ..add(value.decode());
    return b.toBytes();
  }

  String toJson() => toString();
}

@immutable
class KeyPair extends Equatable {
  const KeyPair({required this.key, required this.secret});

  factory KeyPair.fromString(String s) {
    final parts = s.split(':');
    if (parts.length != 2 ||
        parts[0].codeUnits.length != 43 ||
        parts[1].codeUnits.length != 43) {
      throw const FormatException('malformed string');
    }
    final key = PublicKey.fromString(parts[0]);
    final secret = PublicKey.fromString(parts[1]);
    return KeyPair(key: key, secret: secret);
  }
  factory KeyPair.fromJson(dynamic json) => KeyPair.fromString(json as String);
  final PublicKey key;
  final PublicKey secret;
  @override
  List<Object> get props => [key, secret];

  @override
  String toString() => '$key:$secret';

  String toJson() => toString();
}

@immutable
class TypedKeyPair extends Equatable {
  const TypedKeyPair(
      {required this.kind, required this.key, required this.secret});

  factory TypedKeyPair.fromString(String s) {
    final parts = s.split(':');
    if (parts.length != 3 ||
        parts[0].codeUnits.length != 4 ||
        parts[1].codeUnits.length != 43 ||
        parts[2].codeUnits.length != 43) {
      throw VeilidAPIExceptionInvalidArgument('malformed string', 's', s);
    }
    final kind = cryptoKindFromString(parts[0]);
    final key = PublicKey.fromString(parts[1]);
    final secret = PublicKey.fromString(parts[2]);
    return TypedKeyPair(kind: kind, key: key, secret: secret);
  }
  factory TypedKeyPair.fromJson(dynamic json) =>
      TypedKeyPair.fromString(json as String);
  factory TypedKeyPair.fromKeyPair(CryptoKind kind, KeyPair keyPair) =>
      TypedKeyPair(kind: kind, key: keyPair.key, secret: keyPair.secret);
  final CryptoKind kind;
  final PublicKey key;
  final PublicKey secret;
  @override
  List<Object> get props => [kind, key, secret];

  @override
  String toString() => '${cryptoKindToString(kind)}:$key:$secret';

  String toJson() => toString();
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
typedef TypedHashDigest = Typed<HashDigest>;

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
  Future<bool> validateKeyPairWithKeyPair(KeyPair keyPair) =>
      validateKeyPair(keyPair.key, keyPair.secret);

  Future<bool> validateHash(Uint8List data, HashDigest hash);
  //Future<bool> validateHashReader(Stream<List<int>> reader, HashDigest hash);
  Future<CryptoKeyDistance> distance(CryptoKey key1, CryptoKey key2);
  Future<Signature> sign(PublicKey key, SecretKey secret, Uint8List data);
  Future<Signature> signWithKeyPair(KeyPair keyPair, Uint8List data) =>
      sign(keyPair.key, keyPair.secret, data);

  Future<void> verify(PublicKey key, Uint8List data, Signature signature);
  Future<int> aeadOverhead();
  Future<Uint8List> decryptAead(Uint8List body, Nonce nonce,
      SharedSecret sharedSecret, Uint8List? associatedData);
  Future<Uint8List> encryptAead(Uint8List body, Nonce nonce,
      SharedSecret sharedSecret, Uint8List? associatedData);
  Future<Uint8List> cryptNoAuth(
      Uint8List body, Nonce nonce, SharedSecret sharedSecret);

  Future<Uint8List> encryptNoAuthWithNonce(
      Uint8List body, SharedSecret secret) async {
    // generate nonce
    final nonce = await randomNonce();
    // crypt and append nonce
    final b = BytesBuilder()
      ..add(await cryptNoAuth(body, nonce, secret))
      ..add(nonce.decode());
    return b.toBytes();
  }

  Future<Uint8List> decryptNoAuthWithNonce(
      Uint8List body, SharedSecret secret) async {
    if (body.length < Nonce.decodedLength()) {
      throw const FormatException('not enough data to decrypt');
    }
    final nonce =
        Nonce.fromBytes(body.sublist(body.length - Nonce.decodedLength()));
    final encryptedData = body.sublist(0, body.length - Nonce.decodedLength());
    // decrypt
    return cryptNoAuth(encryptedData, nonce, secret);
  }

  Future<Uint8List> encryptNoAuthWithPassword(
      Uint8List body, String password) async {
    final ekbytes = Uint8List.fromList(utf8.encode(password));
    final nonce = await randomNonce();
    final saltBytes = nonce.decode();
    final sharedSecret = await deriveSharedSecret(ekbytes, saltBytes);
    return (await cryptNoAuth(body, nonce, sharedSecret))..addAll(saltBytes);
  }

  Future<Uint8List> decryptNoAuthWithPassword(
      Uint8List body, String password) async {
    if (body.length < Nonce.decodedLength()) {
      throw const FormatException('not enough data to decrypt');
    }
    final ekbytes = Uint8List.fromList(utf8.encode(password));
    final bodyBytes = body.sublist(0, body.length - Nonce.decodedLength());
    final saltBytes = body.sublist(body.length - Nonce.decodedLength());
    final nonce = Nonce.fromBytes(saltBytes);
    final sharedSecret = await deriveSharedSecret(ekbytes, saltBytes);
    return cryptNoAuth(bodyBytes, nonce, sharedSecret);
  }
}
