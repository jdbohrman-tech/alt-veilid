import 'package:flutter_test/flutter_test.dart';
import 'test_encoding.dart';
import 'test_value_subkey_range.dart';

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();

  setUp(() {});

  tearDown(() {});

  test('testEncodingKnownVectors', testEncodingKnownVectors);
  test('testEncodeDecodeGarbage', testEncodeDecodeGarbage);
  test('testEncodeDecodeGarbagePad', testEncodeDecodeGarbagePad);

  test('testVSR', testValueSubkeyRange);
  test('test List<ValueSubkeyRange>', testValueSubkeyRangeList);
}
