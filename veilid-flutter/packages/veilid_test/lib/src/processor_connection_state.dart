import 'package:veilid/veilid.dart';

class ProcessorConnectionState {
  ProcessorConnectionState();

  VeilidStateAttachment attachment = VeilidStateAttachment(
      localNetworkReady: false,
      publicInternetReady: false,
      state: AttachmentState.detached,
      uptime: TimestampDuration(
        value: BigInt.zero,
      ),
      attachedUptime: null);
  VeilidStateNetwork network = VeilidStateNetwork(
      bpsDown: BigInt.zero, bpsUp: BigInt.zero, started: false, peers: []);

  bool get isAttached => !(attachment.state == AttachmentState.detached ||
      attachment.state == AttachmentState.detaching ||
      attachment.state == AttachmentState.attaching);

  bool get isPublicInternetReady => attachment.publicInternetReady;
}
