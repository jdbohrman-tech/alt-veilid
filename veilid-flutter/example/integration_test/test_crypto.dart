import 'dart:convert';

import 'package:flutter_test/flutter_test.dart';
import 'package:veilid/veilid.dart';

Future<void> testBestCryptoSystem() async {
  final cs = await Veilid.instance.bestCryptoSystem();
  expect(await cs.defaultSaltLength(), equals(16));
}

Future<void> testGetCryptoSystem() async {
  final cs = await Veilid.instance.getCryptoSystem(cryptoKindVLD0);
  expect(await cs.defaultSaltLength(), equals(16));
}

Future<void> testGetCryptoSystemInvalid() async {
  await expectLater(
      () async => await Veilid.instance.getCryptoSystem(cryptoKindNONE),
      throwsA(isA<VeilidAPIException>()));
}

Future<void> testHashAndVerifyPassword() async {
  final cs = await Veilid.instance.bestCryptoSystem();
  final nonce = await cs.randomNonce();
  final salt = nonce.decode();

  // Password match
  final phash = await cs.hashPassword(utf8.encode("abc123"), salt);
  expect(await cs.verifyPassword(utf8.encode("abc123"), phash), isTrue);

  // Password mismatch
  await cs.hashPassword(utf8.encode("abc1234"), salt);
  expect(await cs.verifyPassword(utf8.encode("abc1235"), phash), isFalse);
}

Future<void> testGenerateSharedSecret() async {
  final cs = await Veilid.instance.bestCryptoSystem();

  final kp1 = await cs.generateKeyPair();
  final kp2 = await cs.generateKeyPair();
  final kp3 = await cs.generateKeyPair();

  final ssA =
      await cs.generateSharedSecret(kp1.key, kp2.secret, utf8.encode("abc123"));
  final ssB =
      await cs.generateSharedSecret(kp2.key, kp1.secret, utf8.encode("abc123"));

  expect(ssA, equals(ssB));

  final ssC = await cs.generateSharedSecret(
      kp2.key, kp1.secret, utf8.encode("abc1234"));

  expect(ssA, isNot(equals(ssC)));

  final ssD =
      await cs.generateSharedSecret(kp3.key, kp1.secret, utf8.encode("abc123"));

  expect(ssA, isNot(equals(ssD)));
}
