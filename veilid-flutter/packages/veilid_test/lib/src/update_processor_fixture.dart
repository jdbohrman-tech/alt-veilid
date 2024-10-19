import 'dart:async';

import 'package:async_tools/async_tools.dart';
import 'package:veilid/veilid.dart';

import 'processor_connection_state.dart';
import 'veilid_fixture.dart';

class UpdateProcessorFixture {
  UpdateProcessorFixture({required this.veilidFixture});

  static final _fixtureMutex = Mutex();
  VeilidFixture veilidFixture;

  ProcessorConnectionState processorConnectionState =
      ProcessorConnectionState();

  Future<void> setUp() async {
    await _fixtureMutex.acquire();
    veilidFixture.updateStream.listen((update) {
      if (update is VeilidUpdateNetwork) {
        processorConnectionState.network = processorConnectionState.network =
            VeilidStateNetwork(
                started: update.started,
                bpsDown: update.bpsDown,
                bpsUp: update.bpsUp,
                peers: update.peers);
      } else if (update is VeilidUpdateAttachment) {
        processorConnectionState.attachment = VeilidStateAttachment(
            state: update.state,
            publicInternetReady: update.publicInternetReady,
            localNetworkReady: update.localNetworkReady,
            uptime: update.uptime,
            attachedUptime: update.attachedUptime);
      }
    });
  }

  Future<void> tearDown() async {
    assert(_fixtureMutex.isLocked, 'should not tearDown without setUp');

    _fixtureMutex.release();
  }
}
