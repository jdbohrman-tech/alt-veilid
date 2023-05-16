import 'veilid.dart';

import 'dart:html' as html;
import 'dart:js' as js;
import 'dart:js_util' as js_util;
import 'dart:async';
import 'dart:convert';
import 'dart:typed_data';

import 'veilid_encoding.dart';

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
        wasm,
        "routing_context_with_custom_privacy",
        [_ctx.id, jsonEncode(stability)]);

    return VeilidRoutingContextJS._(_Ctx(newId, _ctx.js));
  }

  @override
  VeilidRoutingContextJS withSequencing(Sequencing sequencing) {
    final newId = js_util.callMethod(wasm, "routing_context_with_sequencing",
        [_ctx.id, jsonEncode(sequencing)]);
    return VeilidRoutingContextJS._(_Ctx(newId, _ctx.js));
  }

  @override
  Future<Uint8List> appCall(String target, Uint8List request) async {
    var encodedRequest = base64UrlNoPadEncode(request);

    return base64UrlNoPadDecode(await _wrapApiPromise(js_util.callMethod(
        wasm, "routing_context_app_call", [_ctx.id, target, encodedRequest])));
  }

  @override
  Future<void> appMessage(String target, Uint8List message) {
    var encodedMessage = base64UrlNoPadEncode(message);

    return _wrapApiPromise(js_util.callMethod(wasm,
        "routing_context_app_message", [_ctx.id, target, encodedMessage]));
  }
}

class _TDBT {
  final int id;
  VeilidTableDBJS tdbjs;
  VeilidJS js;

  _TDBT(this.id, this.tdbjs, this.js);
}

// JS implementation of VeilidTableDBTransaction
class VeilidTableDBTransactionJS extends VeilidTableDBTransaction {
  final _TDBT _tdbt;
  static final Finalizer<_TDBT> _finalizer = Finalizer((tdbt) => {
        js_util.callMethod(wasm, "release_table_db_transaction", [tdbt.id])
      });

  VeilidTableDBTransactionJS._(this._tdbt) {
    _finalizer.attach(this, _tdbt, detach: this);
  }

  @override
  Future<void> commit() {
    return _wrapApiPromise(
        js_util.callMethod(wasm, "table_db_transaction_commit", [_tdbt.id]));
  }

  @override
  Future<void> rollback() {
    return _wrapApiPromise(
        js_util.callMethod(wasm, "table_db_transaction_rollback", [_tdbt.id]));
  }

  @override
  Future<void> store(int col, Uint8List key, Uint8List value) {
    final encodedKey = base64UrlNoPadEncode(key);
    final encodedValue = base64UrlNoPadEncode(value);

    return _wrapApiPromise(js_util.callMethod(
        wasm,
        "table_db_transaction_store",
        [_tdbt.id, col, encodedKey, encodedValue]));
  }

  @override
  Future<bool> delete(int col, Uint8List key) {
    final encodedKey = base64UrlNoPadEncode(key);

    return _wrapApiPromise(js_util.callMethod(
        wasm, "table_db_transaction_delete", [_tdbt.id, col, encodedKey]));
  }
}

class _TDB {
  final int id;
  VeilidJS js;

  _TDB(this.id, this.js);
}

// JS implementation of VeilidTableDB
class VeilidTableDBJS extends VeilidTableDB {
  final _TDB _tdb;
  static final Finalizer<_TDB> _finalizer = Finalizer((tdb) => {
        js_util.callMethod(wasm, "release_table_db", [tdb.id])
      });

  VeilidTableDBJS._(this._tdb) {
    _finalizer.attach(this, _tdb, detach: this);
  }

  @override
  int getColumnCount() {
    return js_util.callMethod(wasm, "table_db_get_column_count", [_tdb.id]);
  }

  @override
  List<Uint8List> getKeys(int col) {
    String? s = js_util.callMethod(wasm, "table_db_get_keys", [_tdb.id, col]);
    if (s == null) {
      throw VeilidAPIExceptionInternal("No db for id");
    }
    List<dynamic> jarr = jsonDecode(s);
    return jarr.map((e) => base64UrlNoPadDecode(e)).toList();
  }

  @override
  VeilidTableDBTransaction transact() {
    final id = js_util.callMethod(wasm, "table_db_transact", [_tdb.id]);

    return VeilidTableDBTransactionJS._(_TDBT(id, this, _tdb.js));
  }

  @override
  Future<void> store(int col, Uint8List key, Uint8List value) {
    final encodedKey = base64UrlNoPadEncode(key);
    final encodedValue = base64UrlNoPadEncode(value);

    return _wrapApiPromise(js_util.callMethod(
        wasm, "table_db_store", [_tdb.id, col, encodedKey, encodedValue]));
  }

  @override
  Future<Uint8List?> load(int col, Uint8List key) async {
    final encodedKey = base64UrlNoPadEncode(key);

    String? out = await _wrapApiPromise(
        js_util.callMethod(wasm, "table_db_load", [_tdb.id, col, encodedKey]));
    if (out == null) {
      return null;
    }
    return base64UrlNoPadDecode(out);
  }

  @override
  Future<bool> delete(int col, Uint8List key) {
    final encodedKey = base64UrlNoPadEncode(key);

    return _wrapApiPromise(js_util
        .callMethod(wasm, "table_db_delete", [_tdb.id, col, encodedKey]));
  }
}

// JS implementation of high level Veilid API

class VeilidJS implements Veilid {
  @override
  void initializeVeilidCore(Map<String, dynamic> platformConfigJson) {
    var platformConfigJsonString = jsonEncode(platformConfigJson);
    js_util
        .callMethod(wasm, "initialize_veilid_core", [platformConfigJsonString]);
  }

  @override
  void changeLogLevel(String layer, VeilidConfigLogLevel logLevel) {
    var logLevelJsonString = jsonEncode(logLevel);
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

    await _wrapApiPromise(js_util.callMethod(wasm, "startup_veilid_core",
        [js.allowInterop(updateCallback), jsonEncode(config)]));

    return streamController.stream;
  }

  @override
  Future<VeilidState> getVeilidState() async {
    return VeilidState.fromJson(jsonDecode(await _wrapApiPromise(
        js_util.callMethod(wasm, "get_veilid_state", []))));
  }

  @override
  Future<void> attach() {
    return _wrapApiPromise(js_util.callMethod(wasm, "attach", []));
  }

  @override
  Future<void> detach() {
    return _wrapApiPromise(js_util.callMethod(wasm, "detach", []));
  }

  @override
  Future<void> shutdownVeilidCore() {
    return _wrapApiPromise(
        js_util.callMethod(wasm, "shutdown_veilid_core", []));
  }

  @override
  Future<VeilidRoutingContext> routingContext() async {
    int id =
        await _wrapApiPromise(js_util.callMethod(wasm, "routing_context", []));
    return VeilidRoutingContextJS._(_Ctx(id, this));
  }

  @override
  Future<RouteBlob> newPrivateRoute() async {
    Map<String, dynamic> blobJson = jsonDecode(await _wrapApiPromise(
        js_util.callMethod(wasm, "new_private_route", [])));
    return RouteBlob.fromJson(blobJson);
  }

  @override
  Future<RouteBlob> newCustomPrivateRoute(
      Stability stability, Sequencing sequencing) async {
    var stabilityString = jsonEncode(stability);
    var sequencingString = jsonEncode(sequencing);

    Map<String, dynamic> blobJson = jsonDecode(await _wrapApiPromise(js_util
        .callMethod(
            wasm, "new_private_route", [stabilityString, sequencingString])));
    return RouteBlob.fromJson(blobJson);
  }

  @override
  Future<String> importRemotePrivateRoute(Uint8List blob) {
    var encodedBlob = base64UrlNoPadEncode(blob);
    return _wrapApiPromise(
        js_util.callMethod(wasm, "import_remote_private_route", [encodedBlob]));
  }

  @override
  Future<void> releasePrivateRoute(String key) {
    return _wrapApiPromise(
        js_util.callMethod(wasm, "release_private_route", [key]));
  }

  @override
  Future<void> appCallReply(String id, Uint8List message) {
    var encodedMessage = base64UrlNoPadEncode(message);
    return _wrapApiPromise(
        js_util.callMethod(wasm, "app_call_reply", [id, encodedMessage]));
  }

  @override
  Future<VeilidTableDB> openTableDB(String name, int columnCount) async {
    int id = await _wrapApiPromise(
        js_util.callMethod(wasm, "open_table_db", [name, columnCount]));
    return VeilidTableDBJS._(_TDB(id, this));
  }

  @override
  Future<bool> deleteTableDB(String name) {
    return _wrapApiPromise(js_util.callMethod(wasm, "delete_table_db", [name]));
  }

  @override
  Future<String> debug(String command) async {
    return await _wrapApiPromise(js_util.callMethod(wasm, "debug", [command]));
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
