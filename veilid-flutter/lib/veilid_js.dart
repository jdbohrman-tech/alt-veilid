import 'veilid.dart';

import 'dart:html' as html;
import 'dart:js' as js;
import 'dart:js_util' as js_util;
import 'dart:async';
import 'dart:convert';
import 'dart:typed_data';

//////////////////////////////////////////////////////////

Veilid getVeilid() => VeilidJS();

Object wasm = js_util.getProperty(html.window, "veilid_wasm");

Future<T> _wrapApiPromise<T>(Object p) {
  return js_util.promiseToFuture(p).then((value) => value as T).catchError(
      (error) => Future<T>.error(
          VeilidAPIException.fromJson(jsonDecode(error as String))));
}

// JS implementation of VeilidRoutingContext
class VeilidRoutingContextJS implements VeilidRoutingContext {
  final int _id;
  final VeilidFFI _ffi;

  VeilidRoutingContextFFI._(this._id, this._ffi);
  @override
  VeilidRoutingContextFFI withPrivacy() {
    final newId = _ffi._routingContextWithPrivacy(_id);
    return VeilidRoutingContextFFI._(newId, _ffi);
  }

  @override
  VeilidRoutingContextFFI withCustomPrivacy(Stability stability) {
    final newId = _ffi._routingContextWithCustomPrivacy(
        _id, stability.json.toNativeUtf8());
    return VeilidRoutingContextFFI._(newId, _ffi);
  }

  @override
  VeilidRoutingContextFFI withSequencing(Sequencing sequencing) {
    final newId =
        _ffi._routingContextWithSequencing(_id, sequencing.json.toNativeUtf8());
    return VeilidRoutingContextFFI._(newId, _ffi);
  }

  @override
  Future<Uint8List> appCall(String target, Uint8List request) async {
    var nativeEncodedTarget = target.toNativeUtf8();
    var nativeEncodedRequest = base64UrlEncode(request).toNativeUtf8();

    final recvPort = ReceivePort("routing_context_app_call");
    final sendPort = recvPort.sendPort;
    _ffi._routingContextAppCall(
        sendPort.nativePort, _id, nativeEncodedTarget, nativeEncodedRequest);
    final out = await processFuturePlain(recvPort.first);
    return base64Decode(out);
  }

  @override
  Future<void> appMessage(String target, Uint8List message) async {
    var nativeEncodedTarget = target.toNativeUtf8();
    var nativeEncodedMessage = base64UrlEncode(message).toNativeUtf8();

    final recvPort = ReceivePort("routing_context_app_call");
    final sendPort = recvPort.sendPort;
    _ffi._routingContextAppCall(
        sendPort.nativePort, _id, nativeEncodedTarget, nativeEncodedMessage);
    return processFutureVoid(recvPort.first);
  }
}


// JS implementation of high level Veilid API

class VeilidJS implements Veilid {
  @override
  void initializeVeilidCore(Map<String, dynamic> platformConfigJson) {
    var platformConfigJsonString =
        jsonEncode(platformConfigJson, toEncodable: veilidApiToEncodable);
    js_util
        .callMethod(wasm, "initialize_veilid_core", [platformConfigJsonString]);
  }

  @override
  void changeLogLevel(String layer, VeilidConfigLogLevel logLevel) {
    var logLevelJsonString =
        jsonEncode(logLevel.json, toEncodable: veilidApiToEncodable);
    js_util.callMethod(wasm, "change_log_level", [layer, logLevelJsonString]);
  }

  @override
  Future<Stream<VeilidUpdate>> startupVeilidCore(VeilidConfig config) async {
    var streamController = StreamController<VeilidUpdate>();
    updateCallback(String update) {
      var updateJson = jsonDecode(update);
      if (updateJson["kind"] == "Shutdown") {
        streamController.close();
      } else {
        var update = VeilidUpdate.fromJson(updateJson);
        streamController.add(update);
      }
    }

    await _wrapApiPromise(js_util.callMethod(wasm, "startup_veilid_core", [
      js.allowInterop(updateCallback),
      jsonEncode(config.json, toEncodable: veilidApiToEncodable)
    ]));

    return streamController.stream;
  }

  @override
  Future<VeilidState> getVeilidState() async {
    return VeilidState.fromJson(jsonDecode(await _wrapApiPromise(
        js_util.callMethod(wasm, "get_veilid_state", []))));
  }

  @override
  Future<void> attach() async {
    return _wrapApiPromise(js_util.callMethod(wasm, "attach", []));
  }

  @override
  Future<void> detach() async {
    return _wrapApiPromise(js_util.callMethod(wasm, "detach", []));
  }

  @override
  Future<void> shutdownVeilidCore() {
    return _wrapApiPromise(
        js_util.callMethod(wasm, "shutdown_veilid_core", []));
  }


  @override
  Future<VeilidRoutingContext> routingContext() async {
    final recvPort = ReceivePort("routing_context");
    final sendPort = recvPort.sendPort;
    _routingContext(sendPort.nativePort);
    final id = await processFuturePlain(recvPort.first);
    return VeilidRoutingContextFFI._(id, this);
  }

  @override
  Future<KeyBlob> newPrivateRoute() async {
    final recvPort = ReceivePort("new_private_route");
    final sendPort = recvPort.sendPort;
    _newPrivateRoute(sendPort.nativePort);
    return processFutureJson(KeyBlob.fromJson, recvPort.first);
  }

  @override
  Future<KeyBlob> newCustomPrivateRoute(
      Stability stability, Sequencing sequencing) async {
    return _wrapApiPromise(
        js_util.callMethod(wasm, "new_custom_private_route", [stability, sequencing]));

  }

  @override
  Future<String> importRemotePrivateRoute(Uint8List blob) async {
    var encodedBlob = base64UrlEncode(blob);
    return _wrapApiPromise(
        js_util.callMethod(wasm, "import_remote_private_route", [encodedBlob]));
  }

  @override
  Future<void> releasePrivateRoute(String key) async {
    return _wrapApiPromise(
        js_util.callMethod(wasm, "release_private_route", [key]));
  }

  @override
  Future<void> appCallReply(String id, Uint8List message) {
    var encodedMessage = base64UrlEncode(message);
    return _wrapApiPromise(
        js_util.callMethod(wasm, "app_call_reply", [id, encodedMessage]));
  }
  
  @override
  Future<String> debug(String command) {
    return _wrapApiPromise(js_util.callMethod(wasm, "debug", [command]));
  }

  @override
  String veilidVersionString() {
    return js_util.callMethod(wasm, "veilid_version_string", []);
  }

  @override
  VeilidVersion veilidVersion() {
    var jsonVersion =
        jsonDecode(js_util.callMethod(wasm, "veilid_version", []));
    return VeilidVersion(
        jsonVersion["major"], jsonVersion["minor"], jsonVersion["patch"]);
  }
}
