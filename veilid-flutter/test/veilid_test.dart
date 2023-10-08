import 'package:flutter_test/flutter_test.dart';
import 'test_encoding.dart';

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();

  setUp(() {});

  tearDown(() {});

  test('testEncodingKnownVectors', testEncodingKnownVectors);
  test('testEncodeDecodeGarbage', testEncodeDecodeGarbage);
  test('testEncodeDecodeGarbagePad', testEncodeDecodeGarbagePad);
}
