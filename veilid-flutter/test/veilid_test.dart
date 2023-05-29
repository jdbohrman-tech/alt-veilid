import 'package:flutter_test/flutter_test.dart';
import 'package:veilid/veilid.dart';

void main() {
  Veilid api = Veilid.instance;

  TestWidgetsFlutterBinding.ensureInitialized();

  setUp(() {});

  tearDown(() {});

  test('veilidVersionString', () async {
    expect(api.veilidVersionString(), '0.1.0');
  });
}
