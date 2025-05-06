// dart format width=80
// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'routing_context.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;
DHTSchema _$DHTSchemaFromJson(Map<String, dynamic> json) {
  switch (json['kind']) {
    case 'DFLT':
      return DHTSchemaDFLT.fromJson(json);
    case 'SMPL':
      return DHTSchemaSMPL.fromJson(json);

    default:
      throw CheckedFromJsonException(
          json, 'kind', 'DHTSchema', 'Invalid union type "${json['kind']}"!');
  }
}

/// @nodoc
mixin _$DHTSchema {
  int get oCnt;

  /// Create a copy of DHTSchema
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $DHTSchemaCopyWith<DHTSchema> get copyWith =>
      _$DHTSchemaCopyWithImpl<DHTSchema>(this as DHTSchema, _$identity);

  /// Serializes this DHTSchema to a JSON map.
  Map<String, dynamic> toJson();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is DHTSchema &&
            (identical(other.oCnt, oCnt) || other.oCnt == oCnt));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, oCnt);

  @override
  String toString() {
    return 'DHTSchema(oCnt: $oCnt)';
  }
}

/// @nodoc
abstract mixin class $DHTSchemaCopyWith<$Res> {
  factory $DHTSchemaCopyWith(DHTSchema value, $Res Function(DHTSchema) _then) =
      _$DHTSchemaCopyWithImpl;
  @useResult
  $Res call({int oCnt});
}

/// @nodoc
class _$DHTSchemaCopyWithImpl<$Res> implements $DHTSchemaCopyWith<$Res> {
  _$DHTSchemaCopyWithImpl(this._self, this._then);

  final DHTSchema _self;
  final $Res Function(DHTSchema) _then;

  /// Create a copy of DHTSchema
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? oCnt = null,
  }) {
    return _then(_self.copyWith(
      oCnt: null == oCnt
          ? _self.oCnt
          : oCnt // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class DHTSchemaDFLT implements DHTSchema {
  const DHTSchemaDFLT({required this.oCnt, final String? $type})
      : $type = $type ?? 'DFLT';
  factory DHTSchemaDFLT.fromJson(Map<String, dynamic> json) =>
      _$DHTSchemaDFLTFromJson(json);

  @override
  final int oCnt;

  @JsonKey(name: 'kind')
  final String $type;

  /// Create a copy of DHTSchema
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $DHTSchemaDFLTCopyWith<DHTSchemaDFLT> get copyWith =>
      _$DHTSchemaDFLTCopyWithImpl<DHTSchemaDFLT>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$DHTSchemaDFLTToJson(
      this,
    );
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is DHTSchemaDFLT &&
            (identical(other.oCnt, oCnt) || other.oCnt == oCnt));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, oCnt);

  @override
  String toString() {
    return 'DHTSchema.dflt(oCnt: $oCnt)';
  }
}

/// @nodoc
abstract mixin class $DHTSchemaDFLTCopyWith<$Res>
    implements $DHTSchemaCopyWith<$Res> {
  factory $DHTSchemaDFLTCopyWith(
          DHTSchemaDFLT value, $Res Function(DHTSchemaDFLT) _then) =
      _$DHTSchemaDFLTCopyWithImpl;
  @override
  @useResult
  $Res call({int oCnt});
}

/// @nodoc
class _$DHTSchemaDFLTCopyWithImpl<$Res>
    implements $DHTSchemaDFLTCopyWith<$Res> {
  _$DHTSchemaDFLTCopyWithImpl(this._self, this._then);

  final DHTSchemaDFLT _self;
  final $Res Function(DHTSchemaDFLT) _then;

  /// Create a copy of DHTSchema
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? oCnt = null,
  }) {
    return _then(DHTSchemaDFLT(
      oCnt: null == oCnt
          ? _self.oCnt
          : oCnt // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class DHTSchemaSMPL implements DHTSchema {
  const DHTSchemaSMPL(
      {required this.oCnt,
      required final List<DHTSchemaMember> members,
      final String? $type})
      : _members = members,
        $type = $type ?? 'SMPL';
  factory DHTSchemaSMPL.fromJson(Map<String, dynamic> json) =>
      _$DHTSchemaSMPLFromJson(json);

  @override
  final int oCnt;
  final List<DHTSchemaMember> _members;
  List<DHTSchemaMember> get members {
    if (_members is EqualUnmodifiableListView) return _members;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_members);
  }

  @JsonKey(name: 'kind')
  final String $type;

  /// Create a copy of DHTSchema
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $DHTSchemaSMPLCopyWith<DHTSchemaSMPL> get copyWith =>
      _$DHTSchemaSMPLCopyWithImpl<DHTSchemaSMPL>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$DHTSchemaSMPLToJson(
      this,
    );
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is DHTSchemaSMPL &&
            (identical(other.oCnt, oCnt) || other.oCnt == oCnt) &&
            const DeepCollectionEquality().equals(other._members, _members));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(
      runtimeType, oCnt, const DeepCollectionEquality().hash(_members));

  @override
  String toString() {
    return 'DHTSchema.smpl(oCnt: $oCnt, members: $members)';
  }
}

/// @nodoc
abstract mixin class $DHTSchemaSMPLCopyWith<$Res>
    implements $DHTSchemaCopyWith<$Res> {
  factory $DHTSchemaSMPLCopyWith(
          DHTSchemaSMPL value, $Res Function(DHTSchemaSMPL) _then) =
      _$DHTSchemaSMPLCopyWithImpl;
  @override
  @useResult
  $Res call({int oCnt, List<DHTSchemaMember> members});
}

/// @nodoc
class _$DHTSchemaSMPLCopyWithImpl<$Res>
    implements $DHTSchemaSMPLCopyWith<$Res> {
  _$DHTSchemaSMPLCopyWithImpl(this._self, this._then);

  final DHTSchemaSMPL _self;
  final $Res Function(DHTSchemaSMPL) _then;

  /// Create a copy of DHTSchema
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? oCnt = null,
    Object? members = null,
  }) {
    return _then(DHTSchemaSMPL(
      oCnt: null == oCnt
          ? _self.oCnt
          : oCnt // ignore: cast_nullable_to_non_nullable
              as int,
      members: null == members
          ? _self._members
          : members // ignore: cast_nullable_to_non_nullable
              as List<DHTSchemaMember>,
    ));
  }
}

/// @nodoc
mixin _$DHTSchemaMember {
  PublicKey get mKey;
  int get mCnt;

  /// Create a copy of DHTSchemaMember
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $DHTSchemaMemberCopyWith<DHTSchemaMember> get copyWith =>
      _$DHTSchemaMemberCopyWithImpl<DHTSchemaMember>(
          this as DHTSchemaMember, _$identity);

  /// Serializes this DHTSchemaMember to a JSON map.
  Map<String, dynamic> toJson();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is DHTSchemaMember &&
            (identical(other.mKey, mKey) || other.mKey == mKey) &&
            (identical(other.mCnt, mCnt) || other.mCnt == mCnt));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, mKey, mCnt);

  @override
  String toString() {
    return 'DHTSchemaMember(mKey: $mKey, mCnt: $mCnt)';
  }
}

/// @nodoc
abstract mixin class $DHTSchemaMemberCopyWith<$Res> {
  factory $DHTSchemaMemberCopyWith(
          DHTSchemaMember value, $Res Function(DHTSchemaMember) _then) =
      _$DHTSchemaMemberCopyWithImpl;
  @useResult
  $Res call({PublicKey mKey, int mCnt});
}

/// @nodoc
class _$DHTSchemaMemberCopyWithImpl<$Res>
    implements $DHTSchemaMemberCopyWith<$Res> {
  _$DHTSchemaMemberCopyWithImpl(this._self, this._then);

  final DHTSchemaMember _self;
  final $Res Function(DHTSchemaMember) _then;

  /// Create a copy of DHTSchemaMember
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? mKey = null,
    Object? mCnt = null,
  }) {
    return _then(_self.copyWith(
      mKey: null == mKey
          ? _self.mKey
          : mKey // ignore: cast_nullable_to_non_nullable
              as PublicKey,
      mCnt: null == mCnt
          ? _self.mCnt
          : mCnt // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _DHTSchemaMember implements DHTSchemaMember {
  const _DHTSchemaMember({required this.mKey, required this.mCnt})
      : assert(mCnt > 0 && mCnt <= 65535, 'value out of range');
  factory _DHTSchemaMember.fromJson(Map<String, dynamic> json) =>
      _$DHTSchemaMemberFromJson(json);

  @override
  final PublicKey mKey;
  @override
  final int mCnt;

  /// Create a copy of DHTSchemaMember
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  _$DHTSchemaMemberCopyWith<_DHTSchemaMember> get copyWith =>
      __$DHTSchemaMemberCopyWithImpl<_DHTSchemaMember>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$DHTSchemaMemberToJson(
      this,
    );
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _DHTSchemaMember &&
            (identical(other.mKey, mKey) || other.mKey == mKey) &&
            (identical(other.mCnt, mCnt) || other.mCnt == mCnt));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, mKey, mCnt);

  @override
  String toString() {
    return 'DHTSchemaMember(mKey: $mKey, mCnt: $mCnt)';
  }
}

/// @nodoc
abstract mixin class _$DHTSchemaMemberCopyWith<$Res>
    implements $DHTSchemaMemberCopyWith<$Res> {
  factory _$DHTSchemaMemberCopyWith(
          _DHTSchemaMember value, $Res Function(_DHTSchemaMember) _then) =
      __$DHTSchemaMemberCopyWithImpl;
  @override
  @useResult
  $Res call({PublicKey mKey, int mCnt});
}

/// @nodoc
class __$DHTSchemaMemberCopyWithImpl<$Res>
    implements _$DHTSchemaMemberCopyWith<$Res> {
  __$DHTSchemaMemberCopyWithImpl(this._self, this._then);

  final _DHTSchemaMember _self;
  final $Res Function(_DHTSchemaMember) _then;

  /// Create a copy of DHTSchemaMember
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? mKey = null,
    Object? mCnt = null,
  }) {
    return _then(_DHTSchemaMember(
      mKey: null == mKey
          ? _self.mKey
          : mKey // ignore: cast_nullable_to_non_nullable
              as PublicKey,
      mCnt: null == mCnt
          ? _self.mCnt
          : mCnt // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc
mixin _$DHTRecordDescriptor {
  TypedKey get key;
  PublicKey get owner;
  DHTSchema get schema;
  PublicKey? get ownerSecret;

  /// Create a copy of DHTRecordDescriptor
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $DHTRecordDescriptorCopyWith<DHTRecordDescriptor> get copyWith =>
      _$DHTRecordDescriptorCopyWithImpl<DHTRecordDescriptor>(
          this as DHTRecordDescriptor, _$identity);

  /// Serializes this DHTRecordDescriptor to a JSON map.
  Map<String, dynamic> toJson();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is DHTRecordDescriptor &&
            (identical(other.key, key) || other.key == key) &&
            (identical(other.owner, owner) || other.owner == owner) &&
            (identical(other.schema, schema) || other.schema == schema) &&
            (identical(other.ownerSecret, ownerSecret) ||
                other.ownerSecret == ownerSecret));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, key, owner, schema, ownerSecret);

  @override
  String toString() {
    return 'DHTRecordDescriptor(key: $key, owner: $owner, schema: $schema, ownerSecret: $ownerSecret)';
  }
}

/// @nodoc
abstract mixin class $DHTRecordDescriptorCopyWith<$Res> {
  factory $DHTRecordDescriptorCopyWith(
          DHTRecordDescriptor value, $Res Function(DHTRecordDescriptor) _then) =
      _$DHTRecordDescriptorCopyWithImpl;
  @useResult
  $Res call(
      {TypedKey key,
      PublicKey owner,
      DHTSchema schema,
      PublicKey? ownerSecret});

  $DHTSchemaCopyWith<$Res> get schema;
}

/// @nodoc
class _$DHTRecordDescriptorCopyWithImpl<$Res>
    implements $DHTRecordDescriptorCopyWith<$Res> {
  _$DHTRecordDescriptorCopyWithImpl(this._self, this._then);

  final DHTRecordDescriptor _self;
  final $Res Function(DHTRecordDescriptor) _then;

  /// Create a copy of DHTRecordDescriptor
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? key = null,
    Object? owner = null,
    Object? schema = null,
    Object? ownerSecret = freezed,
  }) {
    return _then(_self.copyWith(
      key: null == key
          ? _self.key
          : key // ignore: cast_nullable_to_non_nullable
              as TypedKey,
      owner: null == owner
          ? _self.owner
          : owner // ignore: cast_nullable_to_non_nullable
              as PublicKey,
      schema: null == schema
          ? _self.schema
          : schema // ignore: cast_nullable_to_non_nullable
              as DHTSchema,
      ownerSecret: freezed == ownerSecret
          ? _self.ownerSecret
          : ownerSecret // ignore: cast_nullable_to_non_nullable
              as PublicKey?,
    ));
  }

  /// Create a copy of DHTRecordDescriptor
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $DHTSchemaCopyWith<$Res> get schema {
    return $DHTSchemaCopyWith<$Res>(_self.schema, (value) {
      return _then(_self.copyWith(schema: value));
    });
  }
}

/// @nodoc
@JsonSerializable()
class _DHTRecordDescriptor implements DHTRecordDescriptor {
  const _DHTRecordDescriptor(
      {required this.key,
      required this.owner,
      required this.schema,
      this.ownerSecret});
  factory _DHTRecordDescriptor.fromJson(Map<String, dynamic> json) =>
      _$DHTRecordDescriptorFromJson(json);

  @override
  final TypedKey key;
  @override
  final PublicKey owner;
  @override
  final DHTSchema schema;
  @override
  final PublicKey? ownerSecret;

  /// Create a copy of DHTRecordDescriptor
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  _$DHTRecordDescriptorCopyWith<_DHTRecordDescriptor> get copyWith =>
      __$DHTRecordDescriptorCopyWithImpl<_DHTRecordDescriptor>(
          this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$DHTRecordDescriptorToJson(
      this,
    );
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _DHTRecordDescriptor &&
            (identical(other.key, key) || other.key == key) &&
            (identical(other.owner, owner) || other.owner == owner) &&
            (identical(other.schema, schema) || other.schema == schema) &&
            (identical(other.ownerSecret, ownerSecret) ||
                other.ownerSecret == ownerSecret));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, key, owner, schema, ownerSecret);

  @override
  String toString() {
    return 'DHTRecordDescriptor(key: $key, owner: $owner, schema: $schema, ownerSecret: $ownerSecret)';
  }
}

/// @nodoc
abstract mixin class _$DHTRecordDescriptorCopyWith<$Res>
    implements $DHTRecordDescriptorCopyWith<$Res> {
  factory _$DHTRecordDescriptorCopyWith(_DHTRecordDescriptor value,
          $Res Function(_DHTRecordDescriptor) _then) =
      __$DHTRecordDescriptorCopyWithImpl;
  @override
  @useResult
  $Res call(
      {TypedKey key,
      PublicKey owner,
      DHTSchema schema,
      PublicKey? ownerSecret});

  @override
  $DHTSchemaCopyWith<$Res> get schema;
}

/// @nodoc
class __$DHTRecordDescriptorCopyWithImpl<$Res>
    implements _$DHTRecordDescriptorCopyWith<$Res> {
  __$DHTRecordDescriptorCopyWithImpl(this._self, this._then);

  final _DHTRecordDescriptor _self;
  final $Res Function(_DHTRecordDescriptor) _then;

  /// Create a copy of DHTRecordDescriptor
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? key = null,
    Object? owner = null,
    Object? schema = null,
    Object? ownerSecret = freezed,
  }) {
    return _then(_DHTRecordDescriptor(
      key: null == key
          ? _self.key
          : key // ignore: cast_nullable_to_non_nullable
              as TypedKey,
      owner: null == owner
          ? _self.owner
          : owner // ignore: cast_nullable_to_non_nullable
              as PublicKey,
      schema: null == schema
          ? _self.schema
          : schema // ignore: cast_nullable_to_non_nullable
              as DHTSchema,
      ownerSecret: freezed == ownerSecret
          ? _self.ownerSecret
          : ownerSecret // ignore: cast_nullable_to_non_nullable
              as PublicKey?,
    ));
  }

  /// Create a copy of DHTRecordDescriptor
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $DHTSchemaCopyWith<$Res> get schema {
    return $DHTSchemaCopyWith<$Res>(_self.schema, (value) {
      return _then(_self.copyWith(schema: value));
    });
  }
}

/// @nodoc
mixin _$ValueData {
  int get seq;
  @Uint8ListJsonConverter.jsIsArray()
  Uint8List get data;
  PublicKey get writer;

  /// Create a copy of ValueData
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $ValueDataCopyWith<ValueData> get copyWith =>
      _$ValueDataCopyWithImpl<ValueData>(this as ValueData, _$identity);

  /// Serializes this ValueData to a JSON map.
  Map<String, dynamic> toJson();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is ValueData &&
            (identical(other.seq, seq) || other.seq == seq) &&
            const DeepCollectionEquality().equals(other.data, data) &&
            (identical(other.writer, writer) || other.writer == writer));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(
      runtimeType, seq, const DeepCollectionEquality().hash(data), writer);

  @override
  String toString() {
    return 'ValueData(seq: $seq, data: $data, writer: $writer)';
  }
}

/// @nodoc
abstract mixin class $ValueDataCopyWith<$Res> {
  factory $ValueDataCopyWith(ValueData value, $Res Function(ValueData) _then) =
      _$ValueDataCopyWithImpl;
  @useResult
  $Res call(
      {int seq,
      @Uint8ListJsonConverter.jsIsArray() Uint8List data,
      PublicKey writer});
}

/// @nodoc
class _$ValueDataCopyWithImpl<$Res> implements $ValueDataCopyWith<$Res> {
  _$ValueDataCopyWithImpl(this._self, this._then);

  final ValueData _self;
  final $Res Function(ValueData) _then;

  /// Create a copy of ValueData
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? seq = null,
    Object? data = null,
    Object? writer = null,
  }) {
    return _then(_self.copyWith(
      seq: null == seq
          ? _self.seq
          : seq // ignore: cast_nullable_to_non_nullable
              as int,
      data: null == data
          ? _self.data
          : data // ignore: cast_nullable_to_non_nullable
              as Uint8List,
      writer: null == writer
          ? _self.writer
          : writer // ignore: cast_nullable_to_non_nullable
              as PublicKey,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _ValueData implements ValueData {
  const _ValueData(
      {required this.seq,
      @Uint8ListJsonConverter.jsIsArray() required this.data,
      required this.writer})
      : assert(seq >= 0, 'seq out of range');
  factory _ValueData.fromJson(Map<String, dynamic> json) =>
      _$ValueDataFromJson(json);

  @override
  final int seq;
  @override
  @Uint8ListJsonConverter.jsIsArray()
  final Uint8List data;
  @override
  final PublicKey writer;

  /// Create a copy of ValueData
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  _$ValueDataCopyWith<_ValueData> get copyWith =>
      __$ValueDataCopyWithImpl<_ValueData>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$ValueDataToJson(
      this,
    );
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _ValueData &&
            (identical(other.seq, seq) || other.seq == seq) &&
            const DeepCollectionEquality().equals(other.data, data) &&
            (identical(other.writer, writer) || other.writer == writer));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(
      runtimeType, seq, const DeepCollectionEquality().hash(data), writer);

  @override
  String toString() {
    return 'ValueData(seq: $seq, data: $data, writer: $writer)';
  }
}

/// @nodoc
abstract mixin class _$ValueDataCopyWith<$Res>
    implements $ValueDataCopyWith<$Res> {
  factory _$ValueDataCopyWith(
          _ValueData value, $Res Function(_ValueData) _then) =
      __$ValueDataCopyWithImpl;
  @override
  @useResult
  $Res call(
      {int seq,
      @Uint8ListJsonConverter.jsIsArray() Uint8List data,
      PublicKey writer});
}

/// @nodoc
class __$ValueDataCopyWithImpl<$Res> implements _$ValueDataCopyWith<$Res> {
  __$ValueDataCopyWithImpl(this._self, this._then);

  final _ValueData _self;
  final $Res Function(_ValueData) _then;

  /// Create a copy of ValueData
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? seq = null,
    Object? data = null,
    Object? writer = null,
  }) {
    return _then(_ValueData(
      seq: null == seq
          ? _self.seq
          : seq // ignore: cast_nullable_to_non_nullable
              as int,
      data: null == data
          ? _self.data
          : data // ignore: cast_nullable_to_non_nullable
              as Uint8List,
      writer: null == writer
          ? _self.writer
          : writer // ignore: cast_nullable_to_non_nullable
              as PublicKey,
    ));
  }
}

/// @nodoc
mixin _$SafetySpec {
  int get hopCount;
  Stability get stability;
  Sequencing get sequencing;
  String? get preferredRoute;

  /// Create a copy of SafetySpec
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $SafetySpecCopyWith<SafetySpec> get copyWith =>
      _$SafetySpecCopyWithImpl<SafetySpec>(this as SafetySpec, _$identity);

  /// Serializes this SafetySpec to a JSON map.
  Map<String, dynamic> toJson();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is SafetySpec &&
            (identical(other.hopCount, hopCount) ||
                other.hopCount == hopCount) &&
            (identical(other.stability, stability) ||
                other.stability == stability) &&
            (identical(other.sequencing, sequencing) ||
                other.sequencing == sequencing) &&
            (identical(other.preferredRoute, preferredRoute) ||
                other.preferredRoute == preferredRoute));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode =>
      Object.hash(runtimeType, hopCount, stability, sequencing, preferredRoute);

  @override
  String toString() {
    return 'SafetySpec(hopCount: $hopCount, stability: $stability, sequencing: $sequencing, preferredRoute: $preferredRoute)';
  }
}

/// @nodoc
abstract mixin class $SafetySpecCopyWith<$Res> {
  factory $SafetySpecCopyWith(
          SafetySpec value, $Res Function(SafetySpec) _then) =
      _$SafetySpecCopyWithImpl;
  @useResult
  $Res call(
      {int hopCount,
      Stability stability,
      Sequencing sequencing,
      String? preferredRoute});
}

/// @nodoc
class _$SafetySpecCopyWithImpl<$Res> implements $SafetySpecCopyWith<$Res> {
  _$SafetySpecCopyWithImpl(this._self, this._then);

  final SafetySpec _self;
  final $Res Function(SafetySpec) _then;

  /// Create a copy of SafetySpec
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? hopCount = null,
    Object? stability = null,
    Object? sequencing = null,
    Object? preferredRoute = freezed,
  }) {
    return _then(_self.copyWith(
      hopCount: null == hopCount
          ? _self.hopCount
          : hopCount // ignore: cast_nullable_to_non_nullable
              as int,
      stability: null == stability
          ? _self.stability
          : stability // ignore: cast_nullable_to_non_nullable
              as Stability,
      sequencing: null == sequencing
          ? _self.sequencing
          : sequencing // ignore: cast_nullable_to_non_nullable
              as Sequencing,
      preferredRoute: freezed == preferredRoute
          ? _self.preferredRoute
          : preferredRoute // ignore: cast_nullable_to_non_nullable
              as String?,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _SafetySpec implements SafetySpec {
  const _SafetySpec(
      {required this.hopCount,
      required this.stability,
      required this.sequencing,
      this.preferredRoute});
  factory _SafetySpec.fromJson(Map<String, dynamic> json) =>
      _$SafetySpecFromJson(json);

  @override
  final int hopCount;
  @override
  final Stability stability;
  @override
  final Sequencing sequencing;
  @override
  final String? preferredRoute;

  /// Create a copy of SafetySpec
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  _$SafetySpecCopyWith<_SafetySpec> get copyWith =>
      __$SafetySpecCopyWithImpl<_SafetySpec>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$SafetySpecToJson(
      this,
    );
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _SafetySpec &&
            (identical(other.hopCount, hopCount) ||
                other.hopCount == hopCount) &&
            (identical(other.stability, stability) ||
                other.stability == stability) &&
            (identical(other.sequencing, sequencing) ||
                other.sequencing == sequencing) &&
            (identical(other.preferredRoute, preferredRoute) ||
                other.preferredRoute == preferredRoute));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode =>
      Object.hash(runtimeType, hopCount, stability, sequencing, preferredRoute);

  @override
  String toString() {
    return 'SafetySpec(hopCount: $hopCount, stability: $stability, sequencing: $sequencing, preferredRoute: $preferredRoute)';
  }
}

/// @nodoc
abstract mixin class _$SafetySpecCopyWith<$Res>
    implements $SafetySpecCopyWith<$Res> {
  factory _$SafetySpecCopyWith(
          _SafetySpec value, $Res Function(_SafetySpec) _then) =
      __$SafetySpecCopyWithImpl;
  @override
  @useResult
  $Res call(
      {int hopCount,
      Stability stability,
      Sequencing sequencing,
      String? preferredRoute});
}

/// @nodoc
class __$SafetySpecCopyWithImpl<$Res> implements _$SafetySpecCopyWith<$Res> {
  __$SafetySpecCopyWithImpl(this._self, this._then);

  final _SafetySpec _self;
  final $Res Function(_SafetySpec) _then;

  /// Create a copy of SafetySpec
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? hopCount = null,
    Object? stability = null,
    Object? sequencing = null,
    Object? preferredRoute = freezed,
  }) {
    return _then(_SafetySpec(
      hopCount: null == hopCount
          ? _self.hopCount
          : hopCount // ignore: cast_nullable_to_non_nullable
              as int,
      stability: null == stability
          ? _self.stability
          : stability // ignore: cast_nullable_to_non_nullable
              as Stability,
      sequencing: null == sequencing
          ? _self.sequencing
          : sequencing // ignore: cast_nullable_to_non_nullable
              as Sequencing,
      preferredRoute: freezed == preferredRoute
          ? _self.preferredRoute
          : preferredRoute // ignore: cast_nullable_to_non_nullable
              as String?,
    ));
  }
}

/// @nodoc
mixin _$RouteBlob {
  String get routeId;
  @Uint8ListJsonConverter()
  Uint8List get blob;

  /// Create a copy of RouteBlob
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $RouteBlobCopyWith<RouteBlob> get copyWith =>
      _$RouteBlobCopyWithImpl<RouteBlob>(this as RouteBlob, _$identity);

  /// Serializes this RouteBlob to a JSON map.
  Map<String, dynamic> toJson();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is RouteBlob &&
            (identical(other.routeId, routeId) || other.routeId == routeId) &&
            const DeepCollectionEquality().equals(other.blob, blob));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(
      runtimeType, routeId, const DeepCollectionEquality().hash(blob));

  @override
  String toString() {
    return 'RouteBlob(routeId: $routeId, blob: $blob)';
  }
}

/// @nodoc
abstract mixin class $RouteBlobCopyWith<$Res> {
  factory $RouteBlobCopyWith(RouteBlob value, $Res Function(RouteBlob) _then) =
      _$RouteBlobCopyWithImpl;
  @useResult
  $Res call({String routeId, @Uint8ListJsonConverter() Uint8List blob});
}

/// @nodoc
class _$RouteBlobCopyWithImpl<$Res> implements $RouteBlobCopyWith<$Res> {
  _$RouteBlobCopyWithImpl(this._self, this._then);

  final RouteBlob _self;
  final $Res Function(RouteBlob) _then;

  /// Create a copy of RouteBlob
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? routeId = null,
    Object? blob = null,
  }) {
    return _then(_self.copyWith(
      routeId: null == routeId
          ? _self.routeId
          : routeId // ignore: cast_nullable_to_non_nullable
              as String,
      blob: null == blob
          ? _self.blob
          : blob // ignore: cast_nullable_to_non_nullable
              as Uint8List,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _RouteBlob implements RouteBlob {
  const _RouteBlob(
      {required this.routeId, @Uint8ListJsonConverter() required this.blob});
  factory _RouteBlob.fromJson(Map<String, dynamic> json) =>
      _$RouteBlobFromJson(json);

  @override
  final String routeId;
  @override
  @Uint8ListJsonConverter()
  final Uint8List blob;

  /// Create a copy of RouteBlob
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  _$RouteBlobCopyWith<_RouteBlob> get copyWith =>
      __$RouteBlobCopyWithImpl<_RouteBlob>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$RouteBlobToJson(
      this,
    );
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _RouteBlob &&
            (identical(other.routeId, routeId) || other.routeId == routeId) &&
            const DeepCollectionEquality().equals(other.blob, blob));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(
      runtimeType, routeId, const DeepCollectionEquality().hash(blob));

  @override
  String toString() {
    return 'RouteBlob(routeId: $routeId, blob: $blob)';
  }
}

/// @nodoc
abstract mixin class _$RouteBlobCopyWith<$Res>
    implements $RouteBlobCopyWith<$Res> {
  factory _$RouteBlobCopyWith(
          _RouteBlob value, $Res Function(_RouteBlob) _then) =
      __$RouteBlobCopyWithImpl;
  @override
  @useResult
  $Res call({String routeId, @Uint8ListJsonConverter() Uint8List blob});
}

/// @nodoc
class __$RouteBlobCopyWithImpl<$Res> implements _$RouteBlobCopyWith<$Res> {
  __$RouteBlobCopyWithImpl(this._self, this._then);

  final _RouteBlob _self;
  final $Res Function(_RouteBlob) _then;

  /// Create a copy of RouteBlob
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? routeId = null,
    Object? blob = null,
  }) {
    return _then(_RouteBlob(
      routeId: null == routeId
          ? _self.routeId
          : routeId // ignore: cast_nullable_to_non_nullable
              as String,
      blob: null == blob
          ? _self.blob
          : blob // ignore: cast_nullable_to_non_nullable
              as Uint8List,
    ));
  }
}

/// @nodoc
mixin _$DHTRecordReport {
  List<ValueSubkeyRange> get subkeys;
  List<ValueSubkeyRange> get offlineSubkeys;
  List<int?> get localSeqs;
  List<int?> get networkSeqs;

  /// Create a copy of DHTRecordReport
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $DHTRecordReportCopyWith<DHTRecordReport> get copyWith =>
      _$DHTRecordReportCopyWithImpl<DHTRecordReport>(
          this as DHTRecordReport, _$identity);

  /// Serializes this DHTRecordReport to a JSON map.
  Map<String, dynamic> toJson();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is DHTRecordReport &&
            const DeepCollectionEquality().equals(other.subkeys, subkeys) &&
            const DeepCollectionEquality()
                .equals(other.offlineSubkeys, offlineSubkeys) &&
            const DeepCollectionEquality().equals(other.localSeqs, localSeqs) &&
            const DeepCollectionEquality()
                .equals(other.networkSeqs, networkSeqs));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(
      runtimeType,
      const DeepCollectionEquality().hash(subkeys),
      const DeepCollectionEquality().hash(offlineSubkeys),
      const DeepCollectionEquality().hash(localSeqs),
      const DeepCollectionEquality().hash(networkSeqs));

  @override
  String toString() {
    return 'DHTRecordReport(subkeys: $subkeys, offlineSubkeys: $offlineSubkeys, localSeqs: $localSeqs, networkSeqs: $networkSeqs)';
  }
}

/// @nodoc
abstract mixin class $DHTRecordReportCopyWith<$Res> {
  factory $DHTRecordReportCopyWith(
          DHTRecordReport value, $Res Function(DHTRecordReport) _then) =
      _$DHTRecordReportCopyWithImpl;
  @useResult
  $Res call(
      {List<ValueSubkeyRange> subkeys,
      List<ValueSubkeyRange> offlineSubkeys,
      List<int?> localSeqs,
      List<int?> networkSeqs});
}

/// @nodoc
class _$DHTRecordReportCopyWithImpl<$Res>
    implements $DHTRecordReportCopyWith<$Res> {
  _$DHTRecordReportCopyWithImpl(this._self, this._then);

  final DHTRecordReport _self;
  final $Res Function(DHTRecordReport) _then;

  /// Create a copy of DHTRecordReport
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? subkeys = null,
    Object? offlineSubkeys = null,
    Object? localSeqs = null,
    Object? networkSeqs = null,
  }) {
    return _then(_self.copyWith(
      subkeys: null == subkeys
          ? _self.subkeys
          : subkeys // ignore: cast_nullable_to_non_nullable
              as List<ValueSubkeyRange>,
      offlineSubkeys: null == offlineSubkeys
          ? _self.offlineSubkeys
          : offlineSubkeys // ignore: cast_nullable_to_non_nullable
              as List<ValueSubkeyRange>,
      localSeqs: null == localSeqs
          ? _self.localSeqs
          : localSeqs // ignore: cast_nullable_to_non_nullable
              as List<int?>,
      networkSeqs: null == networkSeqs
          ? _self.networkSeqs
          : networkSeqs // ignore: cast_nullable_to_non_nullable
              as List<int?>,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _DHTRecordReport implements DHTRecordReport {
  const _DHTRecordReport(
      {required final List<ValueSubkeyRange> subkeys,
      required final List<ValueSubkeyRange> offlineSubkeys,
      required final List<int?> localSeqs,
      required final List<int?> networkSeqs})
      : _subkeys = subkeys,
        _offlineSubkeys = offlineSubkeys,
        _localSeqs = localSeqs,
        _networkSeqs = networkSeqs;
  factory _DHTRecordReport.fromJson(Map<String, dynamic> json) =>
      _$DHTRecordReportFromJson(json);

  final List<ValueSubkeyRange> _subkeys;
  @override
  List<ValueSubkeyRange> get subkeys {
    if (_subkeys is EqualUnmodifiableListView) return _subkeys;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_subkeys);
  }

  final List<ValueSubkeyRange> _offlineSubkeys;
  @override
  List<ValueSubkeyRange> get offlineSubkeys {
    if (_offlineSubkeys is EqualUnmodifiableListView) return _offlineSubkeys;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_offlineSubkeys);
  }

  final List<int?> _localSeqs;
  @override
  List<int?> get localSeqs {
    if (_localSeqs is EqualUnmodifiableListView) return _localSeqs;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_localSeqs);
  }

  final List<int?> _networkSeqs;
  @override
  List<int?> get networkSeqs {
    if (_networkSeqs is EqualUnmodifiableListView) return _networkSeqs;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_networkSeqs);
  }

  /// Create a copy of DHTRecordReport
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  _$DHTRecordReportCopyWith<_DHTRecordReport> get copyWith =>
      __$DHTRecordReportCopyWithImpl<_DHTRecordReport>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$DHTRecordReportToJson(
      this,
    );
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _DHTRecordReport &&
            const DeepCollectionEquality().equals(other._subkeys, _subkeys) &&
            const DeepCollectionEquality()
                .equals(other._offlineSubkeys, _offlineSubkeys) &&
            const DeepCollectionEquality()
                .equals(other._localSeqs, _localSeqs) &&
            const DeepCollectionEquality()
                .equals(other._networkSeqs, _networkSeqs));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(
      runtimeType,
      const DeepCollectionEquality().hash(_subkeys),
      const DeepCollectionEquality().hash(_offlineSubkeys),
      const DeepCollectionEquality().hash(_localSeqs),
      const DeepCollectionEquality().hash(_networkSeqs));

  @override
  String toString() {
    return 'DHTRecordReport(subkeys: $subkeys, offlineSubkeys: $offlineSubkeys, localSeqs: $localSeqs, networkSeqs: $networkSeqs)';
  }
}

/// @nodoc
abstract mixin class _$DHTRecordReportCopyWith<$Res>
    implements $DHTRecordReportCopyWith<$Res> {
  factory _$DHTRecordReportCopyWith(
          _DHTRecordReport value, $Res Function(_DHTRecordReport) _then) =
      __$DHTRecordReportCopyWithImpl;
  @override
  @useResult
  $Res call(
      {List<ValueSubkeyRange> subkeys,
      List<ValueSubkeyRange> offlineSubkeys,
      List<int?> localSeqs,
      List<int?> networkSeqs});
}

/// @nodoc
class __$DHTRecordReportCopyWithImpl<$Res>
    implements _$DHTRecordReportCopyWith<$Res> {
  __$DHTRecordReportCopyWithImpl(this._self, this._then);

  final _DHTRecordReport _self;
  final $Res Function(_DHTRecordReport) _then;

  /// Create a copy of DHTRecordReport
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? subkeys = null,
    Object? offlineSubkeys = null,
    Object? localSeqs = null,
    Object? networkSeqs = null,
  }) {
    return _then(_DHTRecordReport(
      subkeys: null == subkeys
          ? _self._subkeys
          : subkeys // ignore: cast_nullable_to_non_nullable
              as List<ValueSubkeyRange>,
      offlineSubkeys: null == offlineSubkeys
          ? _self._offlineSubkeys
          : offlineSubkeys // ignore: cast_nullable_to_non_nullable
              as List<ValueSubkeyRange>,
      localSeqs: null == localSeqs
          ? _self._localSeqs
          : localSeqs // ignore: cast_nullable_to_non_nullable
              as List<int?>,
      networkSeqs: null == networkSeqs
          ? _self._networkSeqs
          : networkSeqs // ignore: cast_nullable_to_non_nullable
              as List<int?>,
    ));
  }
}

// dart format on
