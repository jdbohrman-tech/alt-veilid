import 'package:flutter_test/flutter_test.dart';
import 'test_encoding.dart';
import 'test_value_subkey_range.dart';

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();

  setUp(() {});
  tearDown(() {});

  group('encoding', () {
    test('test encoding known vectors', testEncodingKnownVectors);
    test('test encode/decode garbage', testEncodeDecodeGarbage);
    test('test encode/decode garbage with pad', testEncodeDecodeGarbagePad);
  });

  group('ValueSubkeyRange', () {
    test('test ValueSubkeyRange', testValueSubkeyRange);
    test('test List<ValueSubkeyRange>', testValueSubkeyRangeList);
  });
}
