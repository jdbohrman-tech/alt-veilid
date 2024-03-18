import 'dart:async';
import 'dart:convert';

import 'package:flutter_test/flutter_test.dart';
import 'package:veilid/veilid.dart';

final bogusKey =
    TypedKey.fromString("VLD0:qD10lHHPD1_Qr23_Qy-1JnxTht12eaWwENVG_m2v7II");

Future<void> testGetDHTValueUnopened() async {
  final rc = await Veilid.instance
      .safeRoutingContext(sequencing: Sequencing.ensureOrdered);
  try {
    await expectLater(
        () async => await rc.getDHTValue(bogusKey, 0, forceRefresh: false),
        throwsA(isA<VeilidAPIException>()));
  } finally {
    rc.close();
  }
}

Future<void> testOpenDHTRecordNonexistentNoWriter() async {
  final rc = await Veilid.instance
      .safeRoutingContext(sequencing: Sequencing.ensureOrdered);
  try {
    await expectLater(() async => await rc.openDHTRecord(bogusKey),
        throwsA(isA<VeilidAPIException>()));
  } finally {
    rc.close();
  }
}

Future<void> testCloseDHTRecordNonexistent() async {
  final rc = await Veilid.instance
      .safeRoutingContext(sequencing: Sequencing.ensureOrdered);
  try {
    await expectLater(() async => await rc.closeDHTRecord(bogusKey),
        throwsA(isA<VeilidAPIException>()));
  } finally {
    rc.close();
  }
}

Future<void> testDeleteDHTRecordNonexistent() async {
  final rc = await Veilid.instance
      .safeRoutingContext(sequencing: Sequencing.ensureOrdered);
  try {
    await expectLater(() async => await rc.deleteDHTRecord(bogusKey),
        throwsA(isA<VeilidAPIException>()));
  } finally {
    rc.close();
  }
}

Future<void> testCreateDeleteDHTRecordSimple() async {
  final rc = await Veilid.instance
      .safeRoutingContext(sequencing: Sequencing.ensureOrdered);
  try {
    final rec = await rc.createDHTRecord(const DHTSchema.dflt(oCnt: 1));
    await rc.closeDHTRecord(rec.key);
    await rc.deleteDHTRecord(rec.key);
  } finally {
    rc.close();
  }
}

Future<void> testCreateDeleteDHTRecordNoClose() async {
  final rc = await Veilid.instance
      .safeRoutingContext(sequencing: Sequencing.ensureOrdered);
  try {
    final rec = await rc.createDHTRecord(const DHTSchema.dflt(oCnt: 1));
    await rc.deleteDHTRecord(rec.key);
  } finally {
    rc.close();
  }
}

Future<void> testGetDHTValueNonexistent() async {
  final rc = await Veilid.instance
      .safeRoutingContext(sequencing: Sequencing.ensureOrdered);
  try {
    final rec = await rc.createDHTRecord(const DHTSchema.dflt(oCnt: 1));
    expect(await rc.getDHTValue(rec.key, 0), isNull);
    await rc.deleteDHTRecord(rec.key);
  } finally {
    rc.close();
  }
}

Future<void> testSetGetDHTValue() async {
  final rc = await Veilid.instance
      .safeRoutingContext(sequencing: Sequencing.ensureOrdered);
  try {
    final rec = await rc.createDHTRecord(const DHTSchema.dflt(oCnt: 2));
    expect(await rc.setDHTValue(rec.key, 0, utf8.encode("BLAH BLAH BLAH")),
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
  final rc = await Veilid.instance
      .safeRoutingContext(sequencing: Sequencing.ensureOrdered);
  try {
    var rec = await rc.createDHTRecord(const DHTSchema.dflt(oCnt: 2));
    final key = rec.key;
    final owner = rec.owner;
    final secret = rec.ownerSecret!;

    final cs = await Veilid.instance.getCryptoSystem(rec.key.kind);
    expect(await cs.validateKeyPair(owner, secret), isTrue);
    final otherKeyPair = await cs.generateKeyPair();

    final va = utf8.encode("Qwertyuiop Asdfghjkl Zxcvbnm");
    final vb = utf8.encode("1234567890");
    final vc = utf8.encode("!@#\$%^&*()");

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

    // Verify subkey 1 can be set a second time and it updates because seq is newer
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
    await expectLater(() async => await rc.setDHTValue(key, 1, va),
        throwsA(isA<VeilidAPIException>()));

    // Verify subkey 0 can NOT be set because we have the wrong writer
    await expectLater(() async => await rc.setDHTValue(key, 0, va),
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

Future<void> testWatchDHTValues(Stream<VeilidUpdate> updateStream) async {
  final valueChangeQueue = StreamController<VeilidUpdateValueChange>();
  final valueChangeSubscription = updateStream.listen((update) {
    if (update is VeilidUpdateValueChange) {
      // print("valuechange: " + update.toString());
      valueChangeQueue.sink.add(update);
    }
  });
  final valueChangeQueueIterator = StreamIterator(valueChangeQueue.stream);

  try {
    // Make two routing contexts, one with and one without safety
    // So we can pretend to be a different node and get the watch updates
    // Normally they would not get sent if the set comes from the same target
    // as the watch's target

    final rcWatch = (await Veilid.instance.routingContext())
        .withSequencing(Sequencing.ensureOrdered, closeSelf: true);
    final rcSet = (await Veilid.instance.routingContext()).withSafety(
        const SafetySelectionUnsafe(sequencing: Sequencing.ensureOrdered),
        closeSelf: true);
    try {
      // Make a DHT record
      var rec = await rcWatch.createDHTRecord(const DHTSchema.dflt(oCnt: 10));

      // Set some subkey we care about
      expect(
          await rcWatch.setDHTValue(rec.key, 3, utf8.encode("BLAH BLAH BLAH")),
          isNull);

      // Make a watch on that subkey
      expect(await rcWatch.watchDHTValues(rec.key),
          isNot(equals(Timestamp.zero())));

      // Reopen without closing to change routing context and not lose watch
      rec = await rcSet.openDHTRecord(rec.key, writer: rec.ownerKeyPair());

      // Now set the subkey and trigger an update
      expect(await rcSet.setDHTValue(rec.key, 3, utf8.encode("BLAH")), isNull);

      // Wait for the update
      await valueChangeQueueIterator
          .moveNext()
          .timeout(const Duration(seconds: 5), onTimeout: () {
        fail("should have a change");
      });

      // Verify the update
      expect(valueChangeQueueIterator.current.key, equals(rec.key));
      expect(valueChangeQueueIterator.current.count, equals(0xFFFFFFFE));
      expect(valueChangeQueueIterator.current.subkeys,
          equals([ValueSubkeyRange.single(3)]));
      expect(valueChangeQueueIterator.current.value.seq, equals(1));
      expect(valueChangeQueueIterator.current.value.data,
          equals(utf8.encode("BLAH")));
      expect(valueChangeQueueIterator.current.value.writer, equals(rec.owner));

      // Reopen without closing to change routing context and not lose watch
      rec = await rcWatch.openDHTRecord(rec.key, writer: rec.ownerKeyPair());

      // Cancel some subkeys we don't care about
      expect(
          await rcWatch
              .cancelDHTWatch(rec.key, subkeys: [ValueSubkeyRange.make(0, 2)]),
          isTrue);

      // Reopen without closing to change routing context and not lose watch
      rec = await rcSet.openDHTRecord(rec.key, writer: rec.ownerKeyPair());

      // Change our subkey
      expect(await rcSet.setDHTValue(rec.key, 3, utf8.encode("BLAH BLAH BLAH")),
          isNull);

      // Wait for the update
      await valueChangeQueueIterator
          .moveNext()
          .timeout(const Duration(seconds: 5), onTimeout: () {
        fail("should have a change");
      });

      // Verify the update
      expect(valueChangeQueueIterator.current.key, equals(rec.key));
      expect(valueChangeQueueIterator.current.count, equals(0xFFFFFFFD));
      expect(valueChangeQueueIterator.current.subkeys,
          equals([ValueSubkeyRange.single(3)]));
      expect(valueChangeQueueIterator.current.value.seq, equals(2));
      expect(valueChangeQueueIterator.current.value.data,
          equals(utf8.encode("BLAH BLAH BLAH")));
      expect(valueChangeQueueIterator.current.value.writer, equals(rec.owner));

      // Reopen without closing to change routing context and not lose watch
      rec = await rcWatch.openDHTRecord(rec.key, writer: rec.ownerKeyPair());

      // Now cancel the update
      expect(
          await rcWatch
              .cancelDHTWatch(rec.key, subkeys: [ValueSubkeyRange.make(3, 9)]),
          isFalse);

      // Reopen without closing to change routing context and not lose watch
      rec = await rcSet.openDHTRecord(rec.key, writer: rec.ownerKeyPair());

      // Set the value without a watch
      expect(await rcSet.setDHTValue(rec.key, 3, utf8.encode("BLAH")), isNull);

      // Now we should NOT get an update
      if (await valueChangeQueueIterator
          .moveNext()
          .timeout(const Duration(seconds: 5), onTimeout: () {
        return false;
      })) {
        fail("should not have a change");
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
  }
}

Future<void> testInspectDHTRecord() async {
  final rc = await Veilid.instance
      .safeRoutingContext(sequencing: Sequencing.ensureOrdered);
  try {
    var rec = await rc.createDHTRecord(const DHTSchema.dflt(oCnt: 2));

    expect(await rc.setDHTValue(rec.key, 0, utf8.encode("BLAH BLAH BLAH")),
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
