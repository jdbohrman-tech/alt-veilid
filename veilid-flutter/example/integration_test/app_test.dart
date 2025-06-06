@Timeout(Duration(seconds: 120))

library;

import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:veilid_test/veilid_test.dart';

import 'test_crypto.dart';
import 'test_dht.dart';
import 'test_routing_context.dart';
import 'test_table_db.dart';
import 'test_veilid_config.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  final fixture =
      DefaultVeilidFixture(programName: 'veilid_flutter integration test');

  group('Unstarted Tests', () {
    test('veilid config defaults', testVeilidConfigDefaults);
  });

  group('Started Tests', () {
    setUpAll(fixture.setUp);
    tearDownAll(fixture.tearDown);

    group('Crypto Tests', () {
      test('best cryptosystem', testBestCryptoSystem);
      test('get cryptosystem', testGetCryptoSystem);
      test('get cryptosystem invalid', testGetCryptoSystemInvalid);
      test('hash and verify password', testHashAndVerifyPassword);
      test('sign and verify signature', testSignAndVerifySignature);
      test('sign and verify signatures', testSignAndVerifySignatures);
    });

    group('Table DB Tests', () {
      test('delete table db nonexistent', testDeleteTableDbNonExistent);
      test('open delete table db', testOpenDeleteTableDb);
      test('open twice table db', testOpenTwiceTableDb);
      test('open twice table db store load', testOpenTwiceTableDbStoreLoad);
      test('open twice table db store delete load',
          testOpenTwiceTableDbStoreDeleteLoad);
      test('resize table db', testResizeTableDb);
    });

    group('Attached Tests', () {
      setUpAll(fixture.attach);
      tearDownAll(fixture.detach);

      group('Routing Contexts', () {
        test('routing contexts', testRoutingContexts);
        test('app message loopback',
            () async => testAppMessageLoopback(fixture.updateStream));
        test('app call loopback',
            () async => testAppCallLoopback(fixture.updateStream));
        test('app message loopback big packets',
            () async => testAppMessageLoopbackBigPackets(fixture.updateStream));
        test('app call loopback big packets',
            () async => testAppCallLoopbackBigPackets(fixture.updateStream));
      });

      group('Veilid DHT', () {
        test('get dht value unopened', testGetDHTValueUnopened);
        test('open dht record nonexistent no writer',
            testOpenDHTRecordNonexistentNoWriter);
        test('close dht record nonexistent', testCloseDHTRecordNonexistent);
        test('delete dht record nonexistent', testDeleteDHTRecordNonexistent);
        test(
            'create delete dht record simple', testCreateDeleteDHTRecordSimple);
        test('create delete dht record no close',
            testCreateDeleteDHTRecordNoClose);
        test('get dht value nonexistent', testGetDHTValueNonexistent);
        test('set get dht value', testSetGetDHTValue);
        test('set get dht value with owner', testSetGetDHTValueWithOwner);
        test('open writer dht value', testOpenWriterDHTValue);
        // xxx: needs to be a multi-server integration test
        // test('watch dht values',
        //     () async => testWatchDHTValues(fixture.updateStream));
        test('inspect dht record', testInspectDHTRecord);
      });
    });
  });
}
