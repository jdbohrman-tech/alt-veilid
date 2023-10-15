import 'dart:async';
import 'dart:convert';
import 'dart:html' as html;
import 'dart:js' as js;
import 'dart:js_interop' as js_interop;
import 'dart:js_util' as js_util;
import 'dart:typed_data';

import 'veilid.dart';

//////////////////////////////////////////////////////////

Veilid getVeilid() => VeilidJS();

Object wasm = js_util.getProperty(html.window, 'veilid_wasm');

Uint8List convertUint8ListFromJson(dynamic json) => Uint8List.fromList(
    ((json as js_interop.JSArray).dartify()! as List<Object?>)
        .map((e) => e! as int)
        .toList());

dynamic convertUint8ListToJson(Uint8List data) => data.toList().jsify();

Future<T> _wrapApiPromise<T>(Object p) => js_util
        .promiseToFuture<T>(p)
        .then((value) => value)
        // ignore: inference_failure_on_untyped_parameter
        .catchError((e) {
      try {
        final ex = VeilidAPIException.fromJson(jsonDecode(e as String));
        throw ex;
      } on Exception catch (_) {
        // Wrap all other errors in VeilidAPIExceptionInternal
        throw VeilidAPIExceptionInternal(e.toString());
      }
    });

class _Ctx {
  _Ctx(int id, this.js) : _id = id;
  int? _id;
  final VeilidJS js;
  int requireId() {
    if (_id == null) {
      throw VeilidAPIExceptionNotInitialized();
    }
    return _id!;
  }

  void close() {
    if (_id != null) {
      js_util.callMethod<void>(wasm, 'release_routing_context', [_id]);
      _id = null;
    }
  }
}

// JS implementation of VeilidRoutingContext
class VeilidRoutingContextJS extends VeilidRoutingContext {
  VeilidRoutingContextJS._(this._ctx) {
    _finalizer.attach(this, _ctx, detach: this);
  }
  final _Ctx _ctx;
  static final Finalizer<_Ctx> _finalizer = Finalizer((ctx) => ctx.close());

  @override
  void close() {
    _ctx.close();
  }

  @override
  VeilidRoutingContextJS withPrivacy() {
    final id = _ctx.requireId();
    final int newId =
        js_util.callMethod(wasm, 'routing_context_with_privacy', [id]);
    return VeilidRoutingContextJS._(_Ctx(newId, _ctx.js));
  }

  @override
  VeilidRoutingContextJS withCustomPrivacy(SafetySelection safetySelection) {
    final id = _ctx.requireId();
    final newId = js_util.callMethod<int>(
        wasm,
        'routing_context_with_custom_privacy',
        [id, jsonEncode(safetySelection)]);

    return VeilidRoutingContextJS._(_Ctx(newId, _ctx.js));
  }

  @override
  VeilidRoutingContextJS withSequencing(Sequencing sequencing) {
    final id = _ctx.requireId();
    final newId = js_util.callMethod<int>(
        wasm, 'routing_context_with_sequencing', [id, jsonEncode(sequencing)]);
    return VeilidRoutingContextJS._(_Ctx(newId, _ctx.js));
  }

  @override
  Future<Uint8List> appCall(String target, Uint8List request) async {
    final id = _ctx.requireId();
    final encodedRequest = base64UrlNoPadEncode(request);

    return base64UrlNoPadDecode(await _wrapApiPromise(js_util.callMethod(
        wasm, 'routing_context_app_call', [id, target, encodedRequest])));
  }

  @override
  Future<void> appMessage(String target, Uint8List message) {
    final id = _ctx.requireId();
    final encodedMessage = base64UrlNoPadEncode(message);

    return _wrapApiPromise(js_util.callMethod(
        wasm, 'routing_context_app_message', [id, target, encodedMessage]));
  }

  @override
  Future<DHTRecordDescriptor> createDHTRecord(DHTSchema schema,
      {CryptoKind kind = 0}) async {
    final id = _ctx.requireId();
    return DHTRecordDescriptor.fromJson(jsonDecode(await _wrapApiPromise(js_util
        .callMethod(wasm, 'routing_context_create_dht_record',
            [id, jsonEncode(schema), kind]))));
  }

  @override
  Future<DHTRecordDescriptor> openDHTRecord(
      TypedKey key, KeyPair? writer) async {
    final id = _ctx.requireId();
    return DHTRecordDescriptor.fromJson(jsonDecode(await _wrapApiPromise(js_util
        .callMethod(wasm, 'routing_context_open_dht_record', [
      id,
      jsonEncode(key),
      if (writer != null) jsonEncode(writer) else null
    ]))));
  }

  @override
  Future<void> closeDHTRecord(TypedKey key) {
    final id = _ctx.requireId();
    return _wrapApiPromise(js_util.callMethod(
        wasm, 'routing_context_close_dht_record', [id, jsonEncode(key)]));
  }

  @override
  Future<void> deleteDHTRecord(TypedKey key) {
    final id = _ctx.requireId();
    return _wrapApiPromise(js_util.callMethod(
        wasm, 'routing_context_delete_dht_record', [id, jsonEncode(key)]));
  }

  @override
  Future<ValueData?> getDHTValue(
      TypedKey key, int subkey, bool forceRefresh) async {
    final id = _ctx.requireId();
    final opt = await _wrapApiPromise<String?>(js_util.callMethod(
        wasm,
        'routing_context_get_dht_value',
        [id, jsonEncode(key), subkey, forceRefresh]));
    if (opt == null) {
      return null;
    }
    final jsonOpt = jsonDecode(opt);
    return jsonOpt == null ? null : ValueData.fromJson(jsonOpt);
  }

  @override
  Future<ValueData?> setDHTValue(
      TypedKey key, int subkey, Uint8List data) async {
    final id = _ctx.requireId();
    final opt = await _wrapApiPromise<String?>(js_util.callMethod(
        wasm,
        'routing_context_set_dht_value',
        [id, jsonEncode(key), subkey, base64UrlNoPadEncode(data)]));
    if (opt == null) {
      return null;
    }
    final jsonOpt = jsonDecode(opt);
    return jsonOpt == null ? null : ValueData.fromJson(jsonOpt);
  }

  @override
  Future<Timestamp> watchDHTValues(TypedKey key, List<ValueSubkeyRange> subkeys,
      Timestamp expiration, int count) async {
    final id = _ctx.requireId();
    final ts = await _wrapApiPromise<String>(js_util.callMethod(
        wasm, 'routing_context_watch_dht_values', [
      id,
      jsonEncode(key),
      jsonEncode(subkeys),
      expiration.toString(),
      count
    ]));
    return Timestamp.fromString(ts);
  }

  @override
  Future<bool> cancelDHTWatch(TypedKey key, List<ValueSubkeyRange> subkeys) {
    final id = _ctx.requireId();
    return _wrapApiPromise(js_util.callMethod(
        wasm,
        'routing_context_cancel_dht_watch',
        [id, jsonEncode(key), jsonEncode(subkeys)]));
  }
}

// JS implementation of VeilidCryptoSystem
class VeilidCryptoSystemJS extends VeilidCryptoSystem {
  VeilidCryptoSystemJS._(this._js, this._kind);

  final CryptoKind _kind;
  // Keep the reference
  // ignore: unused_field
  final VeilidJS _js;

  @override
  CryptoKind kind() => _kind;

  @override
  Future<SharedSecret> cachedDH(PublicKey key, SecretKey secret) async =>
      SharedSecret.fromJson(jsonDecode(await _wrapApiPromise(js_util.callMethod(
          wasm,
          'crypto_cached_dh',
          [_kind, jsonEncode(key), jsonEncode(secret)]))));

  @override
  Future<SharedSecret> computeDH(PublicKey key, SecretKey secret) async =>
      SharedSecret.fromJson(jsonDecode(await _wrapApiPromise(js_util.callMethod(
          wasm,
          'crypto_compute_dh',
          [_kind, jsonEncode(key), jsonEncode(secret)]))));

  @override
  Future<Uint8List> randomBytes(int len) async =>
      base64UrlNoPadDecode(await _wrapApiPromise(
          js_util.callMethod(wasm, 'crypto_random_bytes', [_kind, len])));

  @override
  Future<int> defaultSaltLength() => _wrapApiPromise(
      js_util.callMethod(wasm, 'crypto_default_salt_length', [_kind]));

  @override
  Future<String> hashPassword(Uint8List password, Uint8List salt) =>
      _wrapApiPromise(js_util.callMethod(wasm, 'crypto_hash_password',
          [_kind, base64UrlNoPadEncode(password), base64UrlNoPadEncode(salt)]));

  @override
  Future<bool> verifyPassword(Uint8List password, String passwordHash) =>
      _wrapApiPromise(js_util.callMethod(wasm, 'crypto_verify_password',
          [_kind, base64UrlNoPadEncode(password), passwordHash]));

  @override
  Future<SharedSecret> deriveSharedSecret(
          Uint8List password, Uint8List salt) async =>
      SharedSecret.fromJson(jsonDecode(await _wrapApiPromise(js_util.callMethod(
          wasm, 'crypto_derive_shared_secret', [
        _kind,
        base64UrlNoPadEncode(password),
        base64UrlNoPadEncode(salt)
      ]))));

  @override
  Future<Nonce> randomNonce() async =>
      Nonce.fromJson(jsonDecode(await _wrapApiPromise(
          js_util.callMethod(wasm, 'crypto_random_nonce', [_kind]))));

  @override
  Future<SharedSecret> randomSharedSecret() async =>
      SharedSecret.fromJson(jsonDecode(await _wrapApiPromise(
          js_util.callMethod(wasm, 'crypto_random_shared_secret', [_kind]))));

  @override
  Future<KeyPair> generateKeyPair() async =>
      KeyPair.fromJson(jsonDecode(await _wrapApiPromise(
          js_util.callMethod(wasm, 'crypto_generate_key_pair', [_kind]))));

  @override
  Future<HashDigest> generateHash(Uint8List data) async =>
      HashDigest.fromJson(jsonDecode(await _wrapApiPromise(js_util.callMethod(
          wasm, 'crypto_generate_hash', [_kind, base64UrlNoPadEncode(data)]))));

  @override
  Future<bool> validateKeyPair(PublicKey key, SecretKey secret) =>
      _wrapApiPromise(js_util.callMethod(wasm, 'crypto_validate_key_pair',
          [_kind, jsonEncode(key), jsonEncode(secret)]));

  @override
  Future<bool> validateHash(Uint8List data, HashDigest hash) =>
      _wrapApiPromise(js_util.callMethod(wasm, 'crypto_validate_hash',
          [_kind, base64UrlNoPadEncode(data), jsonEncode(hash)]));

  @override
  Future<CryptoKeyDistance> distance(CryptoKey key1, CryptoKey key2) async =>
      CryptoKeyDistance.fromJson(jsonDecode(await _wrapApiPromise(js_util
          .callMethod(wasm, 'crypto_distance',
              [_kind, jsonEncode(key1), jsonEncode(key2)]))));

  @override
  Future<Signature> sign(
          PublicKey key, SecretKey secret, Uint8List data) async =>
      Signature.fromJson(jsonDecode(await _wrapApiPromise(js_util.callMethod(
          wasm, 'crypto_sign', [
        _kind,
        jsonEncode(key),
        jsonEncode(secret),
        base64UrlNoPadEncode(data)
      ]))));

  @override
  Future<void> verify(PublicKey key, Uint8List data, Signature signature) =>
      _wrapApiPromise(js_util.callMethod(wasm, 'crypto_verify', [
        _kind,
        jsonEncode(key),
        base64UrlNoPadEncode(data),
        jsonEncode(signature),
      ]));

  @override
  Future<int> aeadOverhead() => _wrapApiPromise(
      js_util.callMethod(wasm, 'crypto_aead_overhead', [_kind]));

  @override
  Future<Uint8List> decryptAead(Uint8List body, Nonce nonce,
          SharedSecret sharedSecret, Uint8List? associatedData) async =>
      base64UrlNoPadDecode(await _wrapApiPromise(
          js_util.callMethod(wasm, 'crypto_decrypt_aead', [
        _kind,
        base64UrlNoPadEncode(body),
        jsonEncode(nonce),
        jsonEncode(sharedSecret),
        if (associatedData != null)
          base64UrlNoPadEncode(associatedData)
        else
          null
      ])));

  @override
  Future<Uint8List> encryptAead(Uint8List body, Nonce nonce,
          SharedSecret sharedSecret, Uint8List? associatedData) async =>
      base64UrlNoPadDecode(await _wrapApiPromise(
          js_util.callMethod(wasm, 'crypto_encrypt_aead', [
        _kind,
        base64UrlNoPadEncode(body),
        jsonEncode(nonce),
        jsonEncode(sharedSecret),
        if (associatedData != null)
          base64UrlNoPadEncode(associatedData)
        else
          null
      ])));

  @override
  Future<Uint8List> cryptNoAuth(
          Uint8List body, Nonce nonce, SharedSecret sharedSecret) async =>
      base64UrlNoPadDecode(await _wrapApiPromise(js_util.callMethod(
          wasm, 'crypto_crypt_no_auth', [
        _kind,
        base64UrlNoPadEncode(body),
        jsonEncode(nonce),
        jsonEncode(sharedSecret)
      ])));
}

class _TDBT {
  _TDBT(this.id, this.tdbjs, this.js);
  int? id;
  final VeilidTableDBJS tdbjs;
  final VeilidJS js;
  void ensureValid() {
    if (id == null) {
      throw VeilidAPIExceptionNotInitialized();
    }
  }

  void close() {
    if (id != null) {
      js_util.callMethod<void>(wasm, 'release_table_db_transaction', [id]);
      id = null;
    }
  }
}

// JS implementation of VeilidTableDBTransaction
class VeilidTableDBTransactionJS extends VeilidTableDBTransaction {
  VeilidTableDBTransactionJS._(this._tdbt) {
    _finalizer.attach(this, _tdbt, detach: this);
  }
  final _TDBT _tdbt;
  static final Finalizer<_TDBT> _finalizer = Finalizer((tdbt) => tdbt.close());

  @override
  bool isDone() => _tdbt.id == null;

  @override
  Future<void> commit() async {
    _tdbt.ensureValid();
    final id = _tdbt.id!;
    await _wrapApiPromise<void>(
        js_util.callMethod(wasm, 'table_db_transaction_commit', [id]));
    _tdbt.close();
  }

  @override
  Future<void> rollback() async {
    _tdbt.ensureValid();
    final id = _tdbt.id!;
    await _wrapApiPromise<void>(
        js_util.callMethod(wasm, 'table_db_transaction_rollback', [id]));
    _tdbt.close();
  }

  @override
  Future<void> store(int col, Uint8List key, Uint8List value) async {
    _tdbt.ensureValid();
    final id = _tdbt.id!;
    final encodedKey = base64UrlNoPadEncode(key);
    final encodedValue = base64UrlNoPadEncode(value);

    await _wrapApiPromise<void>(js_util.callMethod(wasm,
        'table_db_transaction_store', [id, col, encodedKey, encodedValue]));
  }

  @override
  Future<void> delete(int col, Uint8List key) async {
    _tdbt.ensureValid();
    final id = _tdbt.id!;
    final encodedKey = base64UrlNoPadEncode(key);

    await _wrapApiPromise<void>(js_util.callMethod(
        wasm, 'table_db_transaction_delete', [id, col, encodedKey]));
  }
}

class _TDB {
  _TDB(int id, this.js) : _id = id;

  int? _id;

  final VeilidJS js;
  int requireId() {
    if (_id == null) {
      throw VeilidAPIExceptionNotInitialized();
    }
    return _id!;
  }

  void close() {
    if (_id != null) {
      js_util.callMethod<void>(wasm, 'release_table_db', [_id]);
      _id = null;
    }
  }
}

// JS implementation of VeilidTableDB
class VeilidTableDBJS extends VeilidTableDB {
  VeilidTableDBJS._(this._tdb) {
    _finalizer.attach(this, _tdb, detach: this);
  }
  final _TDB _tdb;
  static final Finalizer<_TDB> _finalizer = Finalizer((tdb) => tdb.close());

  @override
  void close() {
    _tdb.close();
  }

  @override
  int getColumnCount() {
    final id = _tdb.requireId();
    return js_util.callMethod(wasm, 'table_db_get_column_count', [id]);
  }

  @override
  Future<List<Uint8List>> getKeys(int col) async {
    final id = _tdb.requireId();
    return jsonListConstructor(base64UrlNoPadDecodeDynamic)(jsonDecode(
        await js_util.callMethod(wasm, 'table_db_get_keys', [id, col])));
  }

  @override
  VeilidTableDBTransaction transact() {
    final id = _tdb.requireId();
    final xid = js_util.callMethod<int>(wasm, 'table_db_transact', [id]);

    return VeilidTableDBTransactionJS._(_TDBT(xid, this, _tdb.js));
  }

  @override
  Future<void> store(int col, Uint8List key, Uint8List value) {
    final id = _tdb.requireId();
    final encodedKey = base64UrlNoPadEncode(key);
    final encodedValue = base64UrlNoPadEncode(value);

    return _wrapApiPromise(js_util.callMethod(
        wasm, 'table_db_store', [id, col, encodedKey, encodedValue]));
  }

  @override
  Future<Uint8List?> load(int col, Uint8List key) async {
    final id = _tdb.requireId();
    final encodedKey = base64UrlNoPadEncode(key);

    final out = await _wrapApiPromise<String?>(
        js_util.callMethod(wasm, 'table_db_load', [id, col, encodedKey]));
    if (out == null) {
      return null;
    }
    return base64UrlNoPadDecode(out);
  }

  @override
  Future<Uint8List?> delete(int col, Uint8List key) async {
    final id = _tdb.requireId();
    final encodedKey = base64UrlNoPadEncode(key);

    final out = await _wrapApiPromise<String?>(
        js_util.callMethod(wasm, 'table_db_delete', [id, col, encodedKey]));
    if (out == null) {
      return null;
    }
    return base64UrlNoPadDecode(out);
  }
}

// JS implementation of high level Veilid API

class VeilidJS extends Veilid {
  @override
  void initializeVeilidCore(Map<String, dynamic> platformConfigJson) {
    final platformConfigJsonString = jsonEncode(platformConfigJson);
    js_util.callMethod<void>(
        wasm, 'initialize_veilid_core', [platformConfigJsonString]);
  }

  @override
  void changeLogLevel(String layer, VeilidConfigLogLevel logLevel) {
    final logLevelJsonString = jsonEncode(logLevel);
    js_util.callMethod<void>(
        wasm, 'change_log_level', [layer, logLevelJsonString]);
  }

  @override
  Future<Stream<VeilidUpdate>> startupVeilidCore(VeilidConfig config) async {
    final streamController = StreamController<VeilidUpdate>();
    void updateCallback(String update) {
      final updateJson = jsonDecode(update) as Map<String, dynamic>;
      if (updateJson['kind'] == 'Shutdown') {
        unawaited(streamController.close());
      } else {
        final update = VeilidUpdate.fromJson(updateJson);
        streamController.add(update);
      }
    }

    await _wrapApiPromise<void>(js_util.callMethod(wasm, 'startup_veilid_core',
        [js.allowInterop(updateCallback), jsonEncode(config)]));

    return streamController.stream;
  }

  @override
  Future<VeilidState> getVeilidState() async =>
      VeilidState.fromJson(jsonDecode(await _wrapApiPromise<String>(
          js_util.callMethod(wasm, 'get_veilid_state', []))));

  @override
  Future<void> attach() =>
      _wrapApiPromise(js_util.callMethod(wasm, 'attach', []));

  @override
  Future<void> detach() =>
      _wrapApiPromise(js_util.callMethod(wasm, 'detach', []));

  @override
  Future<void> shutdownVeilidCore() =>
      _wrapApiPromise(js_util.callMethod(wasm, 'shutdown_veilid_core', []));

  @override
  List<CryptoKind> validCryptoKinds() {
    final vck = jsonDecode(js_util.callMethod(wasm, 'valid_crypto_kinds', []))
        as List<dynamic>;
    return vck.map((v) => v as CryptoKind).toList();
  }

  @override
  Future<VeilidCryptoSystem> getCryptoSystem(CryptoKind kind) async {
    if (!validCryptoKinds().contains(kind)) {
      throw const VeilidAPIExceptionGeneric('unsupported cryptosystem');
    }
    return VeilidCryptoSystemJS._(this, kind);
  }

  @override
  Future<VeilidCryptoSystem> bestCryptoSystem() async => VeilidCryptoSystemJS._(
      this, js_util.callMethod(wasm, 'best_crypto_kind', []));

  @override
  Future<List<TypedKey>> verifySignatures(List<TypedKey> nodeIds,
          Uint8List data, List<TypedSignature> signatures) async =>
      jsonListConstructor(TypedKey.fromJson)(jsonDecode(await _wrapApiPromise(
          js_util.callMethod(wasm, 'verify_signatures', [
        jsonEncode(nodeIds),
        base64UrlNoPadEncode(data),
        jsonEncode(signatures)
      ]))));

  @override
  Future<List<TypedSignature>> generateSignatures(
          Uint8List data, List<TypedKeyPair> keyPairs) async =>
      jsonListConstructor(TypedSignature.fromJson)(jsonDecode(
          await _wrapApiPromise(js_util.callMethod(wasm, 'generate_signatures',
              [base64UrlNoPadEncode(data), jsonEncode(keyPairs)]))));

  @override
  Future<TypedKeyPair> generateKeyPair(CryptoKind kind) async =>
      TypedKeyPair.fromJson(jsonDecode(await _wrapApiPromise(
          js_util.callMethod(wasm, 'generate_key_pair', [kind]))));

  @override
  Future<VeilidRoutingContext> routingContext() async {
    final rcid = await _wrapApiPromise<int>(
        js_util.callMethod(wasm, 'routing_context', []));
    return VeilidRoutingContextJS._(_Ctx(rcid, this));
  }

  @override
  Future<RouteBlob> newPrivateRoute() async =>
      RouteBlob.fromJson(jsonDecode(await _wrapApiPromise(
          js_util.callMethod(wasm, 'new_private_route', []))));

  @override
  Future<RouteBlob> newCustomPrivateRoute(
      Stability stability, Sequencing sequencing) async {
    final stabilityString = jsonEncode(stability);
    final sequencingString = jsonEncode(sequencing);

    return RouteBlob.fromJson(jsonDecode(await _wrapApiPromise(js_util
        .callMethod(
            wasm, 'new_private_route', [stabilityString, sequencingString]))));
  }

  @override
  Future<String> importRemotePrivateRoute(Uint8List blob) {
    final encodedBlob = base64UrlNoPadEncode(blob);
    return _wrapApiPromise(
        js_util.callMethod(wasm, 'import_remote_private_route', [encodedBlob]));
  }

  @override
  Future<void> releasePrivateRoute(String key) =>
      _wrapApiPromise(js_util.callMethod(wasm, 'release_private_route', [key]));

  @override
  Future<void> appCallReply(String callId, Uint8List message) {
    final encodedMessage = base64UrlNoPadEncode(message);
    return _wrapApiPromise(
        js_util.callMethod(wasm, 'app_call_reply', [callId, encodedMessage]));
  }

  @override
  Future<VeilidTableDB> openTableDB(String name, int columnCount) async {
    final dbid = await _wrapApiPromise<int>(
        js_util.callMethod(wasm, 'open_table_db', [name, columnCount]));
    return VeilidTableDBJS._(_TDB(dbid, this));
  }

  @override
  Future<bool> deleteTableDB(String name) =>
      _wrapApiPromise(js_util.callMethod(wasm, 'delete_table_db', [name]));

  @override
  Timestamp now() => Timestamp.fromString(js_util.callMethod(wasm, 'now', []));

  @override
  Future<String> debug(String command) async =>
      _wrapApiPromise(js_util.callMethod(wasm, 'debug', [command]));

  @override
  String veilidVersionString() =>
      js_util.callMethod(wasm, 'veilid_version_string', []);

  @override
  VeilidVersion veilidVersion() {
    final jsonVersion =
        jsonDecode(js_util.callMethod(wasm, 'veilid_version', []))
            as Map<String, dynamic>;
    return VeilidVersion(jsonVersion['major'] as int,
        jsonVersion['minor'] as int, jsonVersion['patch'] as int);
  }
}
