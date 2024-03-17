import 'dart:async';
import 'dart:convert';
import 'dart:math';

import 'package:flutter_test/flutter_test.dart';
import 'package:veilid/veilid.dart';

Future<void> testRoutingContexts() async {
  {
    final rc = await Veilid.instance.routingContext();
    rc.close();
  }

  {
    final rc = await Veilid.instance.routingContext();
    final rcp = rc.withDefaultSafety();
    rcp.close();
    rc.close();
  }

  {
    final rc = await Veilid.instance.routingContext();
    final rcp = rc.withSequencing(Sequencing.ensureOrdered);
    rcp.close();
    rc.close();
  }

  {
    final rc = await Veilid.instance.routingContext();
    final rcp = rc.withSafety(const SafetySelectionSafe(
        safetySpec: SafetySpec(
            hopCount: 2,
            stability: Stability.lowLatency,
            sequencing: Sequencing.noPreference)));
    rcp.close();
    rc.close();
  }
  {
    final rc = await Veilid.instance.routingContext();
    final rcp = rc.withSafety(
        const SafetySelectionUnsafe(sequencing: Sequencing.preferOrdered));
    rcp.close();
    rc.close();
  }
}

Future<void> testAppMessageLoopback(Stream<VeilidUpdate> updateStream) async {
  final appMessageQueue = StreamController<VeilidAppMessage>();
  final appMessageSubscription = updateStream.listen((update) {
    if (update is VeilidAppMessage) {
      appMessageQueue.sink.add(update);
    }
  });
  try {
    await Veilid.instance.debug("purge routes");

    // make a routing context that uses a safety route
    final rc = await Veilid.instance
        .safeRoutingContext(sequencing: Sequencing.ensureOrdered);
    try {
      // make a new local private route
      final prl = await Veilid.instance.newPrivateRoute();
      try {
        // import it as a remote route as well so we can send to it
        final prr = await Veilid.instance.importRemotePrivateRoute(prl.blob);
        try {
          // send an app message to our own private route
          final message = utf8.encode("abcd1234");
          await rc.appMessage(prr, message);

          // we should get the same message back
          final update = await appMessageQueue.stream.first;
          expect(update.message, equals(message));
          expect(update.routeId, isNotNull);
        } finally {
          await Veilid.instance.releasePrivateRoute(prr);
        }
      } finally {
        await Veilid.instance.releasePrivateRoute(prl.routeId);
      }
    } finally {
      rc.close();
    }
  } finally {
    await appMessageSubscription.cancel();
  }
}

Future<void> testAppCallLoopback(Stream<VeilidUpdate> updateStream) async {
  final appCallQueue = StreamController<VeilidAppCall>();
  final appMessageSubscription = updateStream.listen((update) {
    if (update is VeilidAppCall) {
      appCallQueue.sink.add(update);
    }
  });
  try {
    await Veilid.instance.debug("purge routes");

    // make a routing context that uses a safety route
    final rc = await Veilid.instance
        .safeRoutingContext(sequencing: Sequencing.ensureOrdered);
    try {
      // make a new local private route
      final prl = await Veilid.instance.newPrivateRoute();
      try {
        // import it as a remote route as well so we can send to it
        final prr = await Veilid.instance.importRemotePrivateRoute(prl.blob);
        try {
          // send an app call to our own private route
          final message = utf8.encode("abcd1234");
          final appCallFuture = rc.appCall(prr, message);

          // we should get the same call back
          final update = await appCallQueue.stream.first;
          final appcallid = update.callId;

          expect(update.message, equals(message));
          expect(update.routeId, isNotNull);

          // now we reply to the request
          final reply = utf8.encode("qwer5678");
          await Veilid.instance.appCallReply(appcallid, reply);

          // now we should get the reply from the call
          final result = await appCallFuture;
          expect(result, equals(reply));
        } finally {
          await Veilid.instance.releasePrivateRoute(prr);
        }
      } finally {
        await Veilid.instance.releasePrivateRoute(prl.routeId);
      }
    } finally {
      rc.close();
    }
  } finally {
    await appMessageSubscription.cancel();
  }
}

Future<void> testAppMessageLoopbackBigPackets(
    Stream<VeilidUpdate> updateStream) async {
  final appMessageQueue = StreamController<VeilidAppMessage>();
  final appMessageSubscription = updateStream.listen((update) {
    if (update is VeilidAppMessage) {
      appMessageQueue.sink.add(update);
    }
  });

  final sentMessages = <String>{};
  final random = Random.secure();
  final cs = await Veilid.instance.bestCryptoSystem();

  try {
    await Veilid.instance.debug("purge routes");

    // make a routing context that uses a safety route
    final rc = await Veilid.instance
        .safeRoutingContext(sequencing: Sequencing.ensureOrdered);
    try {
      // make a new local private route
      final prl = await Veilid.instance.newPrivateRoute();
      try {
        // import it as a remote route as well so we can send to it
        final prr = await Veilid.instance.importRemotePrivateRoute(prl.blob);
        try {
          for (var i = 0; i < 5; i++) {
            // send an app message to our own private route
            final message = await cs.randomBytes(random.nextInt(32768));
            await rc.appMessage(prr, message);
            sentMessages.add(base64Url.encode(message));
          }

          final appMessageQueueIterator =
              StreamIterator(appMessageQueue.stream);

          // we should get the same messages back
          for (var i = 0; i < sentMessages.length; i++) {
            if (await appMessageQueueIterator.moveNext()) {
              final update = appMessageQueueIterator.current;
              expect(sentMessages.contains(base64Url.encode(update.message)),
                  isTrue);
            } else {
              fail("not enough messages in the queue");
            }
          }
        } finally {
          await Veilid.instance.releasePrivateRoute(prr);
        }
      } finally {
        await Veilid.instance.releasePrivateRoute(prl.routeId);
      }
    } finally {
      rc.close();
    }
  } finally {
    await appMessageSubscription.cancel();
  }
}

Future<void> testAppCallLoopbackBigPackets(
    Stream<VeilidUpdate> updateStream) async {
  final appCallQueue = StreamController<VeilidAppCall>();
  final appMessageSubscription = updateStream.listen((update) {
    if (update is VeilidAppCall) {
      appCallQueue.sink.add(update);
    }
  });
  final appCallQueueHandler = () async {
    await for (final update in appCallQueue.stream) {
      await Veilid.instance.appCallReply(update.callId, update.message);
    }
  }();

  final sentMessages = <String>{};
  final random = Random.secure();
  final cs = await Veilid.instance.bestCryptoSystem();

  try {
    await Veilid.instance.debug("purge routes");

    // make a routing context that uses a safety route
    final rc = (await Veilid.instance.routingContext())
        .withSequencing(Sequencing.ensureOrdered, closeSelf: true);
    try {
      // make a new local private route
      final prl = await Veilid.instance
          .newCustomPrivateRoute(Stability.reliable, Sequencing.ensureOrdered);
      try {
        // import it as a remote route as well so we can send to it
        final prr = await Veilid.instance.importRemotePrivateRoute(prl.blob);
        try {
          for (var i = 0; i < 5; i++) {
            // send an app message to our own private route
            final message = await cs.randomBytes(random.nextInt(32768));
            final outmessage = await rc.appCall(prr, message);
            expect(message, equals(outmessage));
          }

          final appMessageQueueIterator = StreamIterator(appCallQueue.stream);

          // we should get the same messages back
          for (var i = 0; i < sentMessages.length; i++) {
            if (await appMessageQueueIterator.moveNext()) {
              final update = appMessageQueueIterator.current;
              expect(sentMessages.contains(base64Url.encode(update.message)),
                  isTrue);
            } else {
              fail("not enough messages in the queue");
            }
          }
        } finally {
          await Veilid.instance.releasePrivateRoute(prr);
        }
      } finally {
        await Veilid.instance.releasePrivateRoute(prl.routeId);
      }
    } finally {
      rc.close();
    }
  } finally {
    await appMessageSubscription.cancel();
  }
  await appCallQueue.close();
  await appCallQueueHandler;
}
