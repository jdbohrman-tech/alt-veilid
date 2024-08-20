import 'dart:async';
import 'dart:convert';

import 'package:flutter_test/flutter_test.dart';
import 'package:veilid/veilid.dart';

final bogusKey =
    TypedKey.fromString('VLD0:qD10lHHPD1_Qr23_Qy-1JnxTht12eaWwENVG_m2v7II');

Future<void> testGetDHTValueUnopened() async {
  final rc = await Veilid.instance.routingContext();
  try {
    await expectLater(() async => rc.getDHTValue(bogusKey, 0),
        throwsA(isA<VeilidAPIException>()));
  } finally {
    rc.close();
  }
}

Future<void> testOpenDHTRecordNonexistentNoWriter() async {
  final rc = await Veilid.instance.routingContext();
  try {
    await expectLater(() async => rc.openDHTRecord(bogusKey),
        throwsA(isA<VeilidAPIException>()));
  } finally {
    rc.close();
  }
}

Future<void> testCloseDHTRecordNonexistent() async {
  final rc = await Veilid.instance.routingContext();
  try {
    await expectLater(() async => rc.closeDHTRecord(bogusKey),
        throwsA(isA<VeilidAPIException>()));
  } finally {
    rc.close();
  }
}

Future<void> testDeleteDHTRecordNonexistent() async {
  final rc = await Veilid.instance.routingContext();
  try {
    await expectLater(() async => rc.deleteDHTRecord(bogusKey),
        throwsA(isA<VeilidAPIException>()));
  } finally {
    rc.close();
  }
}

Future<void> testCreateDeleteDHTRecordSimple() async {
  final rc = await Veilid.instance.routingContext();
  try {
    final rec = await rc.createDHTRecord(const DHTSchema.dflt(oCnt: 1));
    await rc.closeDHTRecord(rec.key);
    await rc.deleteDHTRecord(rec.key);
  } finally {
    rc.close();
  }
}

Future<void> testCreateDeleteDHTRecordNoClose() async {
  final rc = await Veilid.instance.routingContext();
  try {
    final rec = await rc.createDHTRecord(const DHTSchema.dflt(oCnt: 1));
    await rc.deleteDHTRecord(rec.key);
  } finally {
    rc.close();
  }
}

Future<void> testGetDHTValueNonexistent() async {
  final rc = await Veilid.instance.routingContext();
  try {
    final rec = await rc.createDHTRecord(const DHTSchema.dflt(oCnt: 1));
    expect(await rc.getDHTValue(rec.key, 0), isNull);
    await rc.deleteDHTRecord(rec.key);
  } finally {
    rc.close();
  }
}

Future<void> testSetGetDHTValue() async {
  final rc = await Veilid.instance.routingContext();
  try {
    final rec = await rc.createDHTRecord(const DHTSchema.dflt(oCnt: 2));
    expect(await rc.setDHTValue(rec.key, 0, utf8.encode('BLAH BLAH BLAH')),
        isNull);
    final vd2 = await rc.getDHTValue(rec.key, 0);
    expect(vd2, isNotNull);

    final vd3 = await rc.getDHTValue(rec.key, 0, forceRefresh: true);
    expect(vd3, isNotNull);

    final vd4 = await rc.getDHTValue(rec.key, 1);
    expect(vd4, isNull);

    expect(vd2, equals(vd3));

    await rc.deleteDHTRecord(rec.key);
  } finally {
    rc.close();
  }
}

Future<void> testOpenWriterDHTValue() async {
  final rc = await Veilid.instance.routingContext();
  try {
    var rec = await rc.createDHTRecord(const DHTSchema.dflt(oCnt: 2));
    final key = rec.key;
    final owner = rec.owner;
    final secret = rec.ownerSecret!;

    final cs = await Veilid.instance.getCryptoSystem(rec.key.kind);
    expect(await cs.validateKeyPair(owner, secret), isTrue);
    final otherKeyPair = await cs.generateKeyPair();

    final va = utf8.encode('Qwertyuiop Asdfghjkl Zxcvbnm');
    final vb = utf8.encode('1234567890');
    final vc = utf8.encode(r'!@#$%^&*()');

    // Test subkey writes
    expect(await rc.setDHTValue(key, 1, va), isNull);

    var vdtemp = await rc.getDHTValue(key, 1);
    expect(vdtemp, isNotNull);
    expect(vdtemp!.data, equals(va));
    expect(vdtemp.seq, equals(0));
    expect(vdtemp.writer, equals(owner));

    expect(await rc.getDHTValue(key, 0), isNull);

    expect(await rc.setDHTValue(key, 0, vb), isNull);

    expect(
        await rc.getDHTValue(key, 0, forceRefresh: true),
        equals(ValueData(
          data: vb,
          seq: 0,
          writer: owner,
        )));

    expect(
        await rc.getDHTValue(key, 1, forceRefresh: true),
        equals(ValueData(
          data: va,
          seq: 0,
          writer: owner,
        )));

    // Equal value should not trigger sequence number update
    expect(await rc.setDHTValue(key, 1, va), isNull);

    // Different value should trigger sequence number update
    expect(await rc.setDHTValue(key, 1, vb), isNull);

    await settle(rc, key, 0);
    await settle(rc, key, 1);

    // Now that we initialized some subkeys
    // and verified they stored correctly
    // Delete things locally and reopen and see if we can write
    // with the same writer key
    //

    await rc.closeDHTRecord(key);
    await rc.deleteDHTRecord(key);

    rec = await rc.openDHTRecord(key,
        writer: KeyPair(key: owner, secret: secret));
    expect(rec, isNotNull);
    expect(rec.key, equals(key));
    expect(rec.owner, equals(owner));
    expect(rec.ownerSecret, equals(secret));
    expect(rec.schema, isA<DHTSchemaDFLT>());
    expect(rec.schema.oCnt, equals(2));

    // Verify subkey 1 can be set before it is get but newer is available online
    vdtemp = await rc.setDHTValue(key, 1, vc);
    expect(vdtemp, isNotNull);
    expect(vdtemp!.data, equals(vb));
    expect(vdtemp.seq, equals(1));
    expect(vdtemp.writer, equals(owner));

    // Verify subkey 1 can be set a second time
    // and it updates because seq is newer
    expect(await rc.setDHTValue(key, 1, vc), isNull);

    // Verify the network got the subkey update with a refresh check
    vdtemp = await rc.getDHTValue(key, 1, forceRefresh: true);
    expect(vdtemp, isNotNull);
    expect(vdtemp!.data, equals(vc));
    expect(vdtemp.seq, equals(2));
    expect(vdtemp.writer, equals(owner));

    // Delete things locally and reopen and see if we can write
    // with a different writer key (should fail)
    await rc.closeDHTRecord(key);
    await rc.deleteDHTRecord(key);

    rec = await rc.openDHTRecord(key, writer: otherKeyPair);
    expect(rec, isNotNull);
    expect(rec.key, equals(key));
    expect(rec.owner, equals(owner));
    expect(rec.ownerSecret, isNull);
    expect(rec.schema, isA<DHTSchemaDFLT>());
    expect(rec.schema.oCnt, equals(2));

    // Verify subkey 1 can NOT be set because we have the wrong writer
    await expectLater(() async => rc.setDHTValue(key, 1, va),
        throwsA(isA<VeilidAPIException>()));

    // Verify subkey 0 can NOT be set because we have the wrong writer
    await expectLater(() async => rc.setDHTValue(key, 0, va),
        throwsA(isA<VeilidAPIException>()));

    // Verify subkey 0 can be set because override with the right writer
    expect(
        await rc.setDHTValue(key, 0, va,
            writer: KeyPair(key: owner, secret: secret)),
        isNull);

    // Clean up
    await rc.closeDHTRecord(key);
    await rc.deleteDHTRecord(key);
  } finally {
    rc.close();
  }
}

Future<void> settle(VeilidRoutingContext rc, TypedKey key, int subkey) async {
  // Wait for set to settle
  do {
    await Future<void>.delayed(const Duration(milliseconds: 100));
  } while (
      (await rc.inspectDHTRecord(key)).offlineSubkeys.containsSubkey(subkey));
}

Future<VeilidUpdateValueChange?> waitForValueChange(
    Stream<VeilidUpdateValueChange> stream,
    Duration duration,
    Future<void> Function() closure) async {
  final valueChangeQueueIterator = StreamIterator(stream);

  try {
    // Subscribe before call
    final iterfut = valueChangeQueueIterator.moveNext();

    // Call thing that might generate a value change
    await closure();

    // Wait for the first change
    final hasChange =
        await iterfut.timeout(duration, onTimeout: () async => false);

    if (!hasChange) {
      return null;
    }
    return valueChangeQueueIterator.current;
  } finally {
    // Stop waiting for changes
    await valueChangeQueueIterator.cancel();
  }
}

// XXX: Currently does not work because ValueChanged updates are suppressed
// for records that are the same sequence number locally as they are in the
// update. To properly test, you need two servers. Revisit this when we can
// make multiple veilid-core instantiations in a single process.
Future<void> testWatchDHTValues(Stream<VeilidUpdate> updateStream) async {
  final valueChangeQueue =
      StreamController<VeilidUpdateValueChange>.broadcast();
  final valueChangeSubscription = updateStream.listen((update) {
    if (update is VeilidUpdateValueChange) {
      // print("valuechange: " + update.toString());
      valueChangeQueue.sink.add(update);
    }
  });

  try {
    // Make two routing contexts, one with and one without safety
    // So we can pretend to be a different node and get the watch updates
    // Normally they would not get sent if the set comes from the same target
    // as the watch's target

    final rcSet = await Veilid.instance.routingContext();
    final rcWatch = await Veilid.instance.unsafeRoutingContext();
    try {
      // Make a DHT record
      var rec = await rcWatch.createDHTRecord(const DHTSchema.dflt(oCnt: 10));

      // Set some subkey we care about
      expect(
          await rcWatch.setDHTValue(rec.key, 3, utf8.encode('BLAH BLAH BLAH')),
          isNull);

      // Wait for set to settle
      await settle(rcWatch, rec.key, 3);

      // Make a watch on that subkey
      expect(await rcWatch.watchDHTValues(rec.key),
          isNot(equals(Timestamp.zero())));

      // Reopen without closing to change routing context and not lose watch
      rec = await rcSet.openDHTRecord(rec.key, writer: rec.ownerKeyPair());

      // Now we should NOT get an update because the update
      // is the same as our local copy
      final update1 = await waitForValueChange(
          valueChangeQueue.stream, const Duration(seconds: 10), () async {
        // Now set the subkey and trigger an update
        expect(await rcSet.setDHTValue(rec.key, 3, utf8.encode('BLAH BLAH')),
            isNull);

        // Wait for set to settle
        await settle(rcSet, rec.key, 3);
      });
      if (update1 != null) {
        fail('should not have a change');
      }

      // Wait for the update
      final update2 = await waitForValueChange(
          valueChangeQueue.stream, const Duration(seconds: 10), () async {
        // Now set a subkey and trigger an update
        expect(
            await rcSet.setDHTValue(rec.key, 3, utf8.encode('BLAH')), isNull);

        await settle(rcSet, rec.key, 3);
      });
      if (update2 == null) {
        fail('should have a change');
      }

      // Verify the update
      expect(update2.key, equals(rec.key));
      expect(update2.count, equals(0xFFFFFFFD));
      expect(update2.subkeys, equals([ValueSubkeyRange.single(3)]));
      expect(update2.value, isNull);

      // Reopen without closing to change routing context and not lose watch
      rec = await rcWatch.openDHTRecord(rec.key, writer: rec.ownerKeyPair());

      // Cancel some subkeys we don't care about
      expect(
          await rcWatch
              .cancelDHTWatch(rec.key, subkeys: [ValueSubkeyRange.make(0, 2)]),
          isTrue);

      // Reopen without closing to change routing context and not lose watch
      rec = await rcSet.openDHTRecord(rec.key, writer: rec.ownerKeyPair());

      // Wait for the update
      final update3 = await waitForValueChange(
          valueChangeQueue.stream, const Duration(seconds: 10), () async {
        // Now set multiple subkeys and trigger an update on one of them
        expect(
            await [
              rcSet.setDHTValue(rec.key, 3, utf8.encode('BLART')),
              rcSet.setDHTValue(rec.key, 1, utf8.encode('BZORT BZORT'))
            ].wait,
            equals([null, null]));

        await settle(rcSet, rec.key, 3);
        await settle(rcSet, rec.key, 1);
      });
      if (update3 == null) {
        fail('should have a change');
      }

      // Verify the update came back but we don't get a new value because the
      // sequence number is the same
      expect(update3.key, equals(rec.key));
      expect(update3.count, equals(0xFFFFFFFC));
      expect(update3.subkeys, equals([ValueSubkeyRange.single(3)]));
      expect(update3.value, isNull);

      // Reopen without closing to change routing context and not lose watch
      rec = await rcWatch.openDHTRecord(rec.key, writer: rec.ownerKeyPair());

      // Now cancel the update
      expect(
          await rcWatch
              .cancelDHTWatch(rec.key, subkeys: [ValueSubkeyRange.make(3, 9)]),
          isFalse);

      // Reopen without closing to change routing context and not lose watch
      rec = await rcSet.openDHTRecord(rec.key, writer: rec.ownerKeyPair());

      // Wait for the update
      final update4 = await waitForValueChange(
          valueChangeQueue.stream, const Duration(seconds: 10), () async {
        // Now set multiple subkeys that should not trigger an update
        expect(
            await [
              rcSet.setDHTValue(rec.key, 3, utf8.encode('BLAH BLAH BLAH BLAH')),
              rcSet.setDHTValue(rec.key, 5, utf8.encode('BZORT BZORT BZORT'))
            ].wait,
            equals([null, null]));

        await settle(rcSet, rec.key, 3);
        await settle(rcSet, rec.key, 5);
      });
      if (update4 != null) {
        fail('should not have a change');
      }

      // Clean up
      await rcSet.closeDHTRecord(rec.key);
      await rcSet.deleteDHTRecord(rec.key);
    } finally {
      rcWatch.close();
      rcSet.close();
    }
  } finally {
    await valueChangeSubscription.cancel();
    await valueChangeQueue.close();
  }
}

Future<void> testInspectDHTRecord() async {
  final rc = await Veilid.instance.routingContext();
  try {
    final rec = await rc.createDHTRecord(const DHTSchema.dflt(oCnt: 2));

    expect(await rc.setDHTValue(rec.key, 0, utf8.encode('BLAH BLAH BLAH')),
        isNull);

    final rr = await rc.inspectDHTRecord(rec.key);
    expect(rr.subkeys, equals([ValueSubkeyRange.make(0, 1)]));
    expect(rr.localSeqs, equals([0, 0xFFFFFFFF]));
    expect(rr.networkSeqs, equals([]));

    final rr2 =
        await rc.inspectDHTRecord(rec.key, scope: DHTReportScope.syncGet);
    expect(rr2.subkeys, equals([ValueSubkeyRange.make(0, 1)]));
    expect(rr2.localSeqs, equals([0, 0xFFFFFFFF]));
    expect(rr2.networkSeqs, equals([0, 0xFFFFFFFF]));

    await rc.closeDHTRecord(rec.key);
    await rc.deleteDHTRecord(rec.key);
  } finally {
    rc.close();
  }
}
