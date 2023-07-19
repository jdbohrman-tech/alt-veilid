import 'dart:async';
import 'dart:typed_data';
import 'dart:convert';

/////////////////////////////////////
/// VeilidTableDB
abstract class VeilidTableDBTransaction {
  bool isDone();
  Future<void> commit();
  Future<void> rollback();
  Future<void> store(int col, Uint8List key, Uint8List value);
  Future<void> delete(int col, Uint8List key);

  Future<void> storeJson(int col, Uint8List key, Object? object,
      {Object? Function(Object? nonEncodable)? toEncodable}) async {
    return store(col, key,
        utf8.encoder.convert(jsonEncode(object, toEncodable: toEncodable)));
  }

  Future<void> storeStringJson(int col, String key, Object? object,
      {Object? Function(Object? nonEncodable)? toEncodable}) {
    return storeJson(col, utf8.encoder.convert(key), object,
        toEncodable: toEncodable);
  }
}

abstract class VeilidTableDB {
  void close();
  int getColumnCount();
  Future<List<Uint8List>> getKeys(int col);
  VeilidTableDBTransaction transact();
  Future<void> store(int col, Uint8List key, Uint8List value);
  Future<Uint8List?> load(int col, Uint8List key);
  Future<Uint8List?> delete(int col, Uint8List key);

  Future<void> storeJson(int col, Uint8List key, Object? object,
      {Object? Function(Object? nonEncodable)? toEncodable}) {
    return store(col, key,
        utf8.encoder.convert(jsonEncode(object, toEncodable: toEncodable)));
  }

  Future<void> storeStringJson(int col, String key, Object? object,
      {Object? Function(Object? nonEncodable)? toEncodable}) {
    return storeJson(col, utf8.encoder.convert(key), object,
        toEncodable: toEncodable);
  }

  Future<Object?> loadJson(int col, Uint8List key,
      {Object? Function(Object? key, Object? value)? reviver}) async {
    var s = await load(col, key);
    if (s == null) {
      return null;
    }
    return jsonDecode(utf8.decode(s, allowMalformed: false), reviver: reviver);
  }

  Future<Object?> loadStringJson(int col, String key,
      {Object? Function(Object? key, Object? value)? reviver}) {
    return loadJson(col, utf8.encoder.convert(key), reviver: reviver);
  }

  Future<Object?> deleteJson(int col, Uint8List key,
      {Object? Function(Object? key, Object? value)? reviver}) async {
    var s = await delete(col, key);
    if (s == null) {
      return null;
    }
    return jsonDecode(utf8.decode(s, allowMalformed: false), reviver: reviver);
  }

  Future<Object?> deleteStringJson(int col, String key,
      {Object? Function(Object? key, Object? value)? reviver}) {
    return deleteJson(col, utf8.encoder.convert(key), reviver: reviver);
  }
}
