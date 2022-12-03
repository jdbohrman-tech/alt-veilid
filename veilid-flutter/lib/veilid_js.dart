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

class _Ctx {
  final int id;
  final VeilidJS js;
  _Ctx(this.id, this.js);
}

// JS implementation of VeilidRoutingContext
class VeilidRoutingContextJS implements VeilidRoutingContext {
  final _Ctx _ctx;
  static final Finalizer<_Ctx> _finalizer = Finalizer((ctx) => {
        js_util.callMethod(wasm, "release_routing_context", [ctx.id])
      });

  VeilidRoutingContextJS._(this._ctx) {
    _finalizer.attach(this, _ctx, detach: this);
  }

  @override
  VeilidRoutingContextJS withPrivacy() {
    int newId =
        js_util.callMethod(wasm, "routing_context_with_privacy", [_ctx.id]);
    return VeilidRoutingContextJS._(_Ctx(newId, _ctx.js));
  }

  @override
  VeilidRoutingContextJS withCustomPrivacy(Stability stability) {
    final newId = js_util.callMethod(
        wasm, "routing_context_with_custom_privacy", [_ctx.id, stability.json]);

    return VeilidRoutingContextJS._(_Ctx(newId, _ctx.js));
  }

  @override
  VeilidRoutingContextJS withSequencing(Sequencing sequencing) {
    final newId = js_util.callMethod(
        wasm, "routing_context_with_sequencing", [_ctx.id, sequencing.json]);
    return VeilidRoutingContextJS._(_Ctx(newId, _ctx.js));
  }

  @override
  Future<Uint8List> appCall(String target, Uint8List request) async {
    var encodedRequest = base64UrlEncode(request);

    return base64Decode(await _wrapApiPromise(js_util.callMethod(
        wasm, "routing_context_app_call", [_ctx.id, encodedRequest])));
  }

  @override
  Future<void> appMessage(String target, Uint8List message) async {
    var encodedMessage = base64UrlEncode(message);

    return _wrapApiPromise(js_util.callMethod(
        wasm, "routing_context_app_message", [_ctx.id, encodedMessage]));
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
    int id = jsonDecode(
        await _wrapApiPromise(js_util.callMethod(wasm, "routing_context", [])));
    return VeilidRoutingContextJS._(_Ctx(id, this));
  }

  @override
  Future<KeyBlob> newPrivateRoute() async {
    Map<String, dynamic> blobJson = jsonDecode(await _wrapApiPromise(
        js_util.callMethod(wasm, "new_private_route", [])));
    return KeyBlob.fromJson(blobJson);
  }

  @override
  Future<KeyBlob> newCustomPrivateRoute(
      Stability stability, Sequencing sequencing) async {
    var stabilityString =
        jsonEncode(stability, toEncodable: veilidApiToEncodable);
    var sequencingString =
        jsonEncode(sequencing, toEncodable: veilidApiToEncodable);

    Map<String, dynamic> blobJson = jsonDecode(await _wrapApiPromise(js_util
        .callMethod(
            wasm, "new_private_route", [stabilityString, sequencingString])));
    return KeyBlob.fromJson(blobJson);
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
    Map<String, dynamic> jsonVersion =
        jsonDecode(js_util.callMethod(wasm, "veilid_version", []));
    return VeilidVersion(
        jsonVersion["major"], jsonVersion["minor"], jsonVersion["patch"]);
  }
}
