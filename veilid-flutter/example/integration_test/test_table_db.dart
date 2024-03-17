import 'dart:convert';

import 'package:flutter_test/flutter_test.dart';
import 'package:veilid/veilid.dart';

const testDb = "__dart_test_db";
const testNonexistentDb = "__dart_test_nonexistent_db";

Future<void> testDeleteTableDbNonExistent() async {
  expect(await Veilid.instance.deleteTableDB(testNonexistentDb), isFalse);
}

Future<void> testOpenDeleteTableDb() async {
  // delete test db if it exists
  await Veilid.instance.deleteTableDB(testDb);

  final tdb = await Veilid.instance.openTableDB(testDb, 1);
  try {
    expect(() async => await Veilid.instance.deleteTableDB(testDb),
        throwsA(isA<VeilidAPIException>()));
  } finally {
    tdb.close();
  }
  expect(await Veilid.instance.deleteTableDB(testDb), isTrue);
}

Future<void> testOpenTwiceTableDb() async {
  // delete test db if it exists
  await Veilid.instance.deleteTableDB(testDb);

  final tdb = await Veilid.instance.openTableDB(testDb, 1);
  final tdb2 = await Veilid.instance.openTableDB(testDb, 1);

  // delete should fail because open
  await expectLater(() async => await Veilid.instance.deleteTableDB(testDb),
      throwsA(isA<VeilidAPIException>()));
  tdb.close();
  // delete should fail because open
  await expectLater(() async => await Veilid.instance.deleteTableDB(testDb),
      throwsA(isA<VeilidAPIException>()));
  tdb2.close();

  // delete should now succeed
  expect(await Veilid.instance.deleteTableDB(testDb), isTrue);
}

Future<void> testOpenTwiceTableDbStoreLoad() async {
  // delete test db if it exists
  await Veilid.instance.deleteTableDB(testDb);

  final tdb = await Veilid.instance.openTableDB(testDb, 1);
  try {
    final tdb2 = await Veilid.instance.openTableDB(testDb, 1);
    try {
      // store into first db copy
      await tdb.store(0, utf8.encode("asdf"), utf8.encode("1234"));
      // load from second db copy
      expect(
          await tdb2.load(0, utf8.encode("asdf")), equals(utf8.encode("1234")));
    } finally {
      tdb2.close();
    }
  } finally {
    tdb.close();
  }

  // delete should now succeed
  expect(await Veilid.instance.deleteTableDB(testDb), isTrue);
}

Future<void> testOpenTwiceTableDbStoreDeleteLoad() async {
  // delete test db if it exists
  await Veilid.instance.deleteTableDB(testDb);

  final tdb = await Veilid.instance.openTableDB(testDb, 1);
  try {
    final tdb2 = await Veilid.instance.openTableDB(testDb, 1);
    try {
      // store into first db copy
      await tdb.store(0, utf8.encode("asdf"), utf8.encode("1234"));
      // delete from second db copy and clean up
      await tdb2.delete(0, utf8.encode("asdf"));
    } finally {
      tdb2.close();
    }
    // load from first db copy
    expect(await tdb.load(0, utf8.encode("asdf")), isNull);
  } finally {
    tdb.close();
  }

  // delete should now succeed
  expect(await Veilid.instance.deleteTableDB(testDb), isTrue);
}

Future<void> testResizeTableDb() async {
  // delete test db if it exists
  await Veilid.instance.deleteTableDB(testDb);

  final tdb = await Veilid.instance.openTableDB(testDb, 1);
  try {
    // reopen the db with more columns should fail if it is already open
    await expectLater(() async => await Veilid.instance.openTableDB(testDb, 2),
        throwsA(isA<VeilidAPIException>()));
  } finally {
    tdb.close();
  }

  final tdb2 = await Veilid.instance.openTableDB(testDb, 2);
  try {
    // write something to second column
    await tdb2.store(1, utf8.encode("qwer"), utf8.encode("5678"));

    // reopen the db with fewer columns
    final tdb3 = await Veilid.instance.openTableDB(testDb, 1);
    try {
      // Should fail access to second column
      await expectLater(() async => await tdb3.load(1, utf8.encode("qwer")),
          throwsA(isA<VeilidAPIException>()));

      // Should succeed with access to second column
      expect(
          await tdb2.load(1, utf8.encode("qwer")), equals(utf8.encode("5678")));
    } finally {
      tdb3.close();
    }
  } finally {
    tdb2.close();
  }

  // delete should now succeed
  expect(await Veilid.instance.deleteTableDB(testDb), isTrue);
}
