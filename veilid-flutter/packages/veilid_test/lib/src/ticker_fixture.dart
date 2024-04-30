import 'dart:async';

import 'package:async_tools/async_tools.dart';

import 'update_processor_fixture.dart';

abstract class TickerFixtureTickable {
  Future<void> onTick();
}

class TickerFixture {
  TickerFixture({required this.updateProcessorFixture});

  static final _fixtureMutex = Mutex();

  UpdateProcessorFixture updateProcessorFixture;
  Timer? _tickTimer;
  final List<TickerFixtureTickable> _tickables = [];

  Future<void> setUp() async {
    await _fixtureMutex.acquire();
    _tickTimer = Timer.periodic(const Duration(seconds: 1), (timer) {
      singleFuture(this, _onTick);
    });
  }

  Future<void> tearDown() async {
    assert(_fixtureMutex.isLocked, 'should not tearDown without setUp');
    final tickTimer = _tickTimer;
    if (tickTimer != null) {
      tickTimer.cancel();
    }
    _fixtureMutex.release();
  }

  void register(TickerFixtureTickable tickable) {
    _tickables.add(tickable);
  }

  void unregister(TickerFixtureTickable tickable) {
    _tickables.remove(tickable);
  }

  Future<void> _onTick() async {
    await _tickables.map((t) => t.onTick()).wait;
  }
}
