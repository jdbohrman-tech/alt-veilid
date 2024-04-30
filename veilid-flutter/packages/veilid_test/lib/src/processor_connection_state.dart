import 'package:veilid/veilid.dart';

class ProcessorConnectionState {
  ProcessorConnectionState();

  VeilidStateAttachment attachment = const VeilidStateAttachment(
      localNetworkReady: false,
      publicInternetReady: false,
      state: AttachmentState.detached);
  VeilidStateNetwork network = VeilidStateNetwork(
      bpsDown: BigInt.from(0),
      bpsUp: BigInt.from(0),
      started: false,
      peers: []);

  bool get isAttached => !(attachment.state == AttachmentState.detached ||
      attachment.state == AttachmentState.detaching ||
      attachment.state == AttachmentState.attaching);

  bool get isPublicInternetReady => attachment.publicInternetReady;
}
