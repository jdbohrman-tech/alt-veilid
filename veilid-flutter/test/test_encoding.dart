import 'dart:convert';
import 'dart:math';
import 'dart:typed_data';

import 'package:flutter_test/flutter_test.dart';
import 'package:veilid/veilid_encoding.dart';

const knownVectors = [
  ['', ''],
  ['f', 'Zg'],
  ['fo', 'Zm8'],
  ['foo', 'Zm9v'],
  ['foob', 'Zm9vYg'],
  ['fooba', 'Zm9vYmE'],
  ['foobar', 'Zm9vYmFy']
];

Future<void> _testEncodingKnownVector(Uint8List k, String v) async {
  final e = base64UrlNoPadEncode(k);
  expect(e, v, reason: 'encode mismatch');

  final d = base64UrlNoPadDecode(v);
  expect(d, k, reason: 'decode mismatch');

  final r = base64UrlNoPadDecode(e);
  expect(r, k, reason: 'round trip mismatch');
}

Future<void> testEncodingKnownVectors() async {
  for (final kv in knownVectors) {
    final k = Uint8List.fromList(kv[0].codeUnits);
    final v = kv[1];

    await _testEncodingKnownVector(k, v);
  }
}

Future<void> testEncodeDecodeGarbage() async {
  final random = Random(0);
  for (var n = 0; n < 8192; n++) {
    final kl = List<int>.empty(growable: true);
    for (var p = 0; p < n; p++) {
      final v = random.nextInt(256);
      kl.add(v);
    }
    final k = Uint8List.fromList(kl);

    final e = base64UrlNoPadEncode(k);
    final r = base64UrlNoPadDecode(e);

    expect(r, k, reason: 'garbage round trip mismatch');
  }
}

Future<void> testEncodeDecodeGarbagePad() async {
  final random = Random(0);
  for (var n = 0; n < 8192; n++) {
    final kl = List<int>.empty(growable: true);
    for (var p = 0; p < n; p++) {
      final v = random.nextInt(256);
      kl.add(v);
    }
    final k = Uint8List.fromList(kl);

    final e = base64Url.encode(k);
    final r = base64UrlNoPadDecode(e);

    expect(r, k, reason: 'garbage w/pad round trip mismatch');
  }
}
