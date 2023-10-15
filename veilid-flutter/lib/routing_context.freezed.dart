// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'routing_context.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#custom-getters-and-methods');

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
  int get oCnt => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(int oCnt) dflt,
    required TResult Function(int oCnt, List<DHTSchemaMember> members) smpl,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(int oCnt)? dflt,
    TResult? Function(int oCnt, List<DHTSchemaMember> members)? smpl,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(int oCnt)? dflt,
    TResult Function(int oCnt, List<DHTSchemaMember> members)? smpl,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(DHTSchemaDFLT value) dflt,
    required TResult Function(DHTSchemaSMPL value) smpl,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(DHTSchemaDFLT value)? dflt,
    TResult? Function(DHTSchemaSMPL value)? smpl,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(DHTSchemaDFLT value)? dflt,
    TResult Function(DHTSchemaSMPL value)? smpl,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
  @JsonKey(ignore: true)
  $DHTSchemaCopyWith<DHTSchema> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $DHTSchemaCopyWith<$Res> {
  factory $DHTSchemaCopyWith(DHTSchema value, $Res Function(DHTSchema) then) =
      _$DHTSchemaCopyWithImpl<$Res, DHTSchema>;
  @useResult
  $Res call({int oCnt});
}

/// @nodoc
class _$DHTSchemaCopyWithImpl<$Res, $Val extends DHTSchema>
    implements $DHTSchemaCopyWith<$Res> {
  _$DHTSchemaCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? oCnt = null,
  }) {
    return _then(_value.copyWith(
      oCnt: null == oCnt
          ? _value.oCnt
          : oCnt // ignore: cast_nullable_to_non_nullable
              as int,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$DHTSchemaDFLTCopyWith<$Res>
    implements $DHTSchemaCopyWith<$Res> {
  factory _$$DHTSchemaDFLTCopyWith(
          _$DHTSchemaDFLT value, $Res Function(_$DHTSchemaDFLT) then) =
      __$$DHTSchemaDFLTCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({int oCnt});
}

/// @nodoc
class __$$DHTSchemaDFLTCopyWithImpl<$Res>
    extends _$DHTSchemaCopyWithImpl<$Res, _$DHTSchemaDFLT>
    implements _$$DHTSchemaDFLTCopyWith<$Res> {
  __$$DHTSchemaDFLTCopyWithImpl(
      _$DHTSchemaDFLT _value, $Res Function(_$DHTSchemaDFLT) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? oCnt = null,
  }) {
    return _then(_$DHTSchemaDFLT(
      oCnt: null == oCnt
          ? _value.oCnt
          : oCnt // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$DHTSchemaDFLT implements DHTSchemaDFLT {
  const _$DHTSchemaDFLT({required this.oCnt, final String? $type})
      : $type = $type ?? 'DFLT';

  factory _$DHTSchemaDFLT.fromJson(Map<String, dynamic> json) =>
      _$$DHTSchemaDFLTFromJson(json);

  @override
  final int oCnt;

  @JsonKey(name: 'kind')
  final String $type;

  @override
  String toString() {
    return 'DHTSchema.dflt(oCnt: $oCnt)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$DHTSchemaDFLT &&
            (identical(other.oCnt, oCnt) || other.oCnt == oCnt));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, oCnt);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$DHTSchemaDFLTCopyWith<_$DHTSchemaDFLT> get copyWith =>
      __$$DHTSchemaDFLTCopyWithImpl<_$DHTSchemaDFLT>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(int oCnt) dflt,
    required TResult Function(int oCnt, List<DHTSchemaMember> members) smpl,
  }) {
    return dflt(oCnt);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(int oCnt)? dflt,
    TResult? Function(int oCnt, List<DHTSchemaMember> members)? smpl,
  }) {
    return dflt?.call(oCnt);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(int oCnt)? dflt,
    TResult Function(int oCnt, List<DHTSchemaMember> members)? smpl,
    required TResult orElse(),
  }) {
    if (dflt != null) {
      return dflt(oCnt);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(DHTSchemaDFLT value) dflt,
    required TResult Function(DHTSchemaSMPL value) smpl,
  }) {
    return dflt(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(DHTSchemaDFLT value)? dflt,
    TResult? Function(DHTSchemaSMPL value)? smpl,
  }) {
    return dflt?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(DHTSchemaDFLT value)? dflt,
    TResult Function(DHTSchemaSMPL value)? smpl,
    required TResult orElse(),
  }) {
    if (dflt != null) {
      return dflt(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$DHTSchemaDFLTToJson(
      this,
    );
  }
}

abstract class DHTSchemaDFLT implements DHTSchema {
  const factory DHTSchemaDFLT({required final int oCnt}) = _$DHTSchemaDFLT;

  factory DHTSchemaDFLT.fromJson(Map<String, dynamic> json) =
      _$DHTSchemaDFLT.fromJson;

  @override
  int get oCnt;
  @override
  @JsonKey(ignore: true)
  _$$DHTSchemaDFLTCopyWith<_$DHTSchemaDFLT> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$DHTSchemaSMPLCopyWith<$Res>
    implements $DHTSchemaCopyWith<$Res> {
  factory _$$DHTSchemaSMPLCopyWith(
          _$DHTSchemaSMPL value, $Res Function(_$DHTSchemaSMPL) then) =
      __$$DHTSchemaSMPLCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({int oCnt, List<DHTSchemaMember> members});
}

/// @nodoc
class __$$DHTSchemaSMPLCopyWithImpl<$Res>
    extends _$DHTSchemaCopyWithImpl<$Res, _$DHTSchemaSMPL>
    implements _$$DHTSchemaSMPLCopyWith<$Res> {
  __$$DHTSchemaSMPLCopyWithImpl(
      _$DHTSchemaSMPL _value, $Res Function(_$DHTSchemaSMPL) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? oCnt = null,
    Object? members = null,
  }) {
    return _then(_$DHTSchemaSMPL(
      oCnt: null == oCnt
          ? _value.oCnt
          : oCnt // ignore: cast_nullable_to_non_nullable
              as int,
      members: null == members
          ? _value._members
          : members // ignore: cast_nullable_to_non_nullable
              as List<DHTSchemaMember>,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$DHTSchemaSMPL implements DHTSchemaSMPL {
  const _$DHTSchemaSMPL(
      {required this.oCnt,
      required final List<DHTSchemaMember> members,
      final String? $type})
      : _members = members,
        $type = $type ?? 'SMPL';

  factory _$DHTSchemaSMPL.fromJson(Map<String, dynamic> json) =>
      _$$DHTSchemaSMPLFromJson(json);

  @override
  final int oCnt;
  final List<DHTSchemaMember> _members;
  @override
  List<DHTSchemaMember> get members {
    if (_members is EqualUnmodifiableListView) return _members;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_members);
  }

  @JsonKey(name: 'kind')
  final String $type;

  @override
  String toString() {
    return 'DHTSchema.smpl(oCnt: $oCnt, members: $members)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$DHTSchemaSMPL &&
            (identical(other.oCnt, oCnt) || other.oCnt == oCnt) &&
            const DeepCollectionEquality().equals(other._members, _members));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(
      runtimeType, oCnt, const DeepCollectionEquality().hash(_members));

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$DHTSchemaSMPLCopyWith<_$DHTSchemaSMPL> get copyWith =>
      __$$DHTSchemaSMPLCopyWithImpl<_$DHTSchemaSMPL>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(int oCnt) dflt,
    required TResult Function(int oCnt, List<DHTSchemaMember> members) smpl,
  }) {
    return smpl(oCnt, members);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(int oCnt)? dflt,
    TResult? Function(int oCnt, List<DHTSchemaMember> members)? smpl,
  }) {
    return smpl?.call(oCnt, members);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(int oCnt)? dflt,
    TResult Function(int oCnt, List<DHTSchemaMember> members)? smpl,
    required TResult orElse(),
  }) {
    if (smpl != null) {
      return smpl(oCnt, members);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(DHTSchemaDFLT value) dflt,
    required TResult Function(DHTSchemaSMPL value) smpl,
  }) {
    return smpl(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(DHTSchemaDFLT value)? dflt,
    TResult? Function(DHTSchemaSMPL value)? smpl,
  }) {
    return smpl?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(DHTSchemaDFLT value)? dflt,
    TResult Function(DHTSchemaSMPL value)? smpl,
    required TResult orElse(),
  }) {
    if (smpl != null) {
      return smpl(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$DHTSchemaSMPLToJson(
      this,
    );
  }
}

abstract class DHTSchemaSMPL implements DHTSchema {
  const factory DHTSchemaSMPL(
      {required final int oCnt,
      required final List<DHTSchemaMember> members}) = _$DHTSchemaSMPL;

  factory DHTSchemaSMPL.fromJson(Map<String, dynamic> json) =
      _$DHTSchemaSMPL.fromJson;

  @override
  int get oCnt;
  List<DHTSchemaMember> get members;
  @override
  @JsonKey(ignore: true)
  _$$DHTSchemaSMPLCopyWith<_$DHTSchemaSMPL> get copyWith =>
      throw _privateConstructorUsedError;
}

DHTSchemaMember _$DHTSchemaMemberFromJson(Map<String, dynamic> json) {
  return _DHTSchemaMember.fromJson(json);
}

/// @nodoc
mixin _$DHTSchemaMember {
  FixedEncodedString43 get mKey => throw _privateConstructorUsedError;
  int get mCnt => throw _privateConstructorUsedError;

  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
  @JsonKey(ignore: true)
  $DHTSchemaMemberCopyWith<DHTSchemaMember> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $DHTSchemaMemberCopyWith<$Res> {
  factory $DHTSchemaMemberCopyWith(
          DHTSchemaMember value, $Res Function(DHTSchemaMember) then) =
      _$DHTSchemaMemberCopyWithImpl<$Res, DHTSchemaMember>;
  @useResult
  $Res call({FixedEncodedString43 mKey, int mCnt});
}

/// @nodoc
class _$DHTSchemaMemberCopyWithImpl<$Res, $Val extends DHTSchemaMember>
    implements $DHTSchemaMemberCopyWith<$Res> {
  _$DHTSchemaMemberCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? mKey = null,
    Object? mCnt = null,
  }) {
    return _then(_value.copyWith(
      mKey: null == mKey
          ? _value.mKey
          : mKey // ignore: cast_nullable_to_non_nullable
              as FixedEncodedString43,
      mCnt: null == mCnt
          ? _value.mCnt
          : mCnt // ignore: cast_nullable_to_non_nullable
              as int,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$_DHTSchemaMemberCopyWith<$Res>
    implements $DHTSchemaMemberCopyWith<$Res> {
  factory _$$_DHTSchemaMemberCopyWith(
          _$_DHTSchemaMember value, $Res Function(_$_DHTSchemaMember) then) =
      __$$_DHTSchemaMemberCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({FixedEncodedString43 mKey, int mCnt});
}

/// @nodoc
class __$$_DHTSchemaMemberCopyWithImpl<$Res>
    extends _$DHTSchemaMemberCopyWithImpl<$Res, _$_DHTSchemaMember>
    implements _$$_DHTSchemaMemberCopyWith<$Res> {
  __$$_DHTSchemaMemberCopyWithImpl(
      _$_DHTSchemaMember _value, $Res Function(_$_DHTSchemaMember) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? mKey = null,
    Object? mCnt = null,
  }) {
    return _then(_$_DHTSchemaMember(
      mKey: null == mKey
          ? _value.mKey
          : mKey // ignore: cast_nullable_to_non_nullable
              as FixedEncodedString43,
      mCnt: null == mCnt
          ? _value.mCnt
          : mCnt // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$_DHTSchemaMember implements _DHTSchemaMember {
  const _$_DHTSchemaMember({required this.mKey, required this.mCnt})
      : assert(mCnt > 0 && mCnt <= 65535, 'value out of range');

  factory _$_DHTSchemaMember.fromJson(Map<String, dynamic> json) =>
      _$$_DHTSchemaMemberFromJson(json);

  @override
  final FixedEncodedString43 mKey;
  @override
  final int mCnt;

  @override
  String toString() {
    return 'DHTSchemaMember(mKey: $mKey, mCnt: $mCnt)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$_DHTSchemaMember &&
            (identical(other.mKey, mKey) || other.mKey == mKey) &&
            (identical(other.mCnt, mCnt) || other.mCnt == mCnt));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, mKey, mCnt);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$_DHTSchemaMemberCopyWith<_$_DHTSchemaMember> get copyWith =>
      __$$_DHTSchemaMemberCopyWithImpl<_$_DHTSchemaMember>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$_DHTSchemaMemberToJson(
      this,
    );
  }
}

abstract class _DHTSchemaMember implements DHTSchemaMember {
  const factory _DHTSchemaMember(
      {required final FixedEncodedString43 mKey,
      required final int mCnt}) = _$_DHTSchemaMember;

  factory _DHTSchemaMember.fromJson(Map<String, dynamic> json) =
      _$_DHTSchemaMember.fromJson;

  @override
  FixedEncodedString43 get mKey;
  @override
  int get mCnt;
  @override
  @JsonKey(ignore: true)
  _$$_DHTSchemaMemberCopyWith<_$_DHTSchemaMember> get copyWith =>
      throw _privateConstructorUsedError;
}

DHTRecordDescriptor _$DHTRecordDescriptorFromJson(Map<String, dynamic> json) {
  return _DHTRecordDescriptor.fromJson(json);
}

/// @nodoc
mixin _$DHTRecordDescriptor {
  Typed<FixedEncodedString43> get key => throw _privateConstructorUsedError;
  FixedEncodedString43 get owner => throw _privateConstructorUsedError;
  DHTSchema get schema => throw _privateConstructorUsedError;
  FixedEncodedString43? get ownerSecret => throw _privateConstructorUsedError;

  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
  @JsonKey(ignore: true)
  $DHTRecordDescriptorCopyWith<DHTRecordDescriptor> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $DHTRecordDescriptorCopyWith<$Res> {
  factory $DHTRecordDescriptorCopyWith(
          DHTRecordDescriptor value, $Res Function(DHTRecordDescriptor) then) =
      _$DHTRecordDescriptorCopyWithImpl<$Res, DHTRecordDescriptor>;
  @useResult
  $Res call(
      {Typed<FixedEncodedString43> key,
      FixedEncodedString43 owner,
      DHTSchema schema,
      FixedEncodedString43? ownerSecret});

  $DHTSchemaCopyWith<$Res> get schema;
}

/// @nodoc
class _$DHTRecordDescriptorCopyWithImpl<$Res, $Val extends DHTRecordDescriptor>
    implements $DHTRecordDescriptorCopyWith<$Res> {
  _$DHTRecordDescriptorCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? key = null,
    Object? owner = null,
    Object? schema = null,
    Object? ownerSecret = freezed,
  }) {
    return _then(_value.copyWith(
      key: null == key
          ? _value.key
          : key // ignore: cast_nullable_to_non_nullable
              as Typed<FixedEncodedString43>,
      owner: null == owner
          ? _value.owner
          : owner // ignore: cast_nullable_to_non_nullable
              as FixedEncodedString43,
      schema: null == schema
          ? _value.schema
          : schema // ignore: cast_nullable_to_non_nullable
              as DHTSchema,
      ownerSecret: freezed == ownerSecret
          ? _value.ownerSecret
          : ownerSecret // ignore: cast_nullable_to_non_nullable
              as FixedEncodedString43?,
    ) as $Val);
  }

  @override
  @pragma('vm:prefer-inline')
  $DHTSchemaCopyWith<$Res> get schema {
    return $DHTSchemaCopyWith<$Res>(_value.schema, (value) {
      return _then(_value.copyWith(schema: value) as $Val);
    });
  }
}

/// @nodoc
abstract class _$$_DHTRecordDescriptorCopyWith<$Res>
    implements $DHTRecordDescriptorCopyWith<$Res> {
  factory _$$_DHTRecordDescriptorCopyWith(_$_DHTRecordDescriptor value,
          $Res Function(_$_DHTRecordDescriptor) then) =
      __$$_DHTRecordDescriptorCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {Typed<FixedEncodedString43> key,
      FixedEncodedString43 owner,
      DHTSchema schema,
      FixedEncodedString43? ownerSecret});

  @override
  $DHTSchemaCopyWith<$Res> get schema;
}

/// @nodoc
class __$$_DHTRecordDescriptorCopyWithImpl<$Res>
    extends _$DHTRecordDescriptorCopyWithImpl<$Res, _$_DHTRecordDescriptor>
    implements _$$_DHTRecordDescriptorCopyWith<$Res> {
  __$$_DHTRecordDescriptorCopyWithImpl(_$_DHTRecordDescriptor _value,
      $Res Function(_$_DHTRecordDescriptor) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? key = null,
    Object? owner = null,
    Object? schema = null,
    Object? ownerSecret = freezed,
  }) {
    return _then(_$_DHTRecordDescriptor(
      key: null == key
          ? _value.key
          : key // ignore: cast_nullable_to_non_nullable
              as Typed<FixedEncodedString43>,
      owner: null == owner
          ? _value.owner
          : owner // ignore: cast_nullable_to_non_nullable
              as FixedEncodedString43,
      schema: null == schema
          ? _value.schema
          : schema // ignore: cast_nullable_to_non_nullable
              as DHTSchema,
      ownerSecret: freezed == ownerSecret
          ? _value.ownerSecret
          : ownerSecret // ignore: cast_nullable_to_non_nullable
              as FixedEncodedString43?,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$_DHTRecordDescriptor implements _DHTRecordDescriptor {
  const _$_DHTRecordDescriptor(
      {required this.key,
      required this.owner,
      required this.schema,
      this.ownerSecret});

  factory _$_DHTRecordDescriptor.fromJson(Map<String, dynamic> json) =>
      _$$_DHTRecordDescriptorFromJson(json);

  @override
  final Typed<FixedEncodedString43> key;
  @override
  final FixedEncodedString43 owner;
  @override
  final DHTSchema schema;
  @override
  final FixedEncodedString43? ownerSecret;

  @override
  String toString() {
    return 'DHTRecordDescriptor(key: $key, owner: $owner, schema: $schema, ownerSecret: $ownerSecret)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$_DHTRecordDescriptor &&
            (identical(other.key, key) || other.key == key) &&
            (identical(other.owner, owner) || other.owner == owner) &&
            (identical(other.schema, schema) || other.schema == schema) &&
            (identical(other.ownerSecret, ownerSecret) ||
                other.ownerSecret == ownerSecret));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, key, owner, schema, ownerSecret);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$_DHTRecordDescriptorCopyWith<_$_DHTRecordDescriptor> get copyWith =>
      __$$_DHTRecordDescriptorCopyWithImpl<_$_DHTRecordDescriptor>(
          this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$_DHTRecordDescriptorToJson(
      this,
    );
  }
}

abstract class _DHTRecordDescriptor implements DHTRecordDescriptor {
  const factory _DHTRecordDescriptor(
      {required final Typed<FixedEncodedString43> key,
      required final FixedEncodedString43 owner,
      required final DHTSchema schema,
      final FixedEncodedString43? ownerSecret}) = _$_DHTRecordDescriptor;

  factory _DHTRecordDescriptor.fromJson(Map<String, dynamic> json) =
      _$_DHTRecordDescriptor.fromJson;

  @override
  Typed<FixedEncodedString43> get key;
  @override
  FixedEncodedString43 get owner;
  @override
  DHTSchema get schema;
  @override
  FixedEncodedString43? get ownerSecret;
  @override
  @JsonKey(ignore: true)
  _$$_DHTRecordDescriptorCopyWith<_$_DHTRecordDescriptor> get copyWith =>
      throw _privateConstructorUsedError;
}

ValueSubkeyRange _$ValueSubkeyRangeFromJson(Map<String, dynamic> json) {
  return _ValueSubkeyRange.fromJson(json);
}

/// @nodoc
mixin _$ValueSubkeyRange {
  int get low => throw _privateConstructorUsedError;
  int get high => throw _privateConstructorUsedError;

  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
  @JsonKey(ignore: true)
  $ValueSubkeyRangeCopyWith<ValueSubkeyRange> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $ValueSubkeyRangeCopyWith<$Res> {
  factory $ValueSubkeyRangeCopyWith(
          ValueSubkeyRange value, $Res Function(ValueSubkeyRange) then) =
      _$ValueSubkeyRangeCopyWithImpl<$Res, ValueSubkeyRange>;
  @useResult
  $Res call({int low, int high});
}

/// @nodoc
class _$ValueSubkeyRangeCopyWithImpl<$Res, $Val extends ValueSubkeyRange>
    implements $ValueSubkeyRangeCopyWith<$Res> {
  _$ValueSubkeyRangeCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? low = null,
    Object? high = null,
  }) {
    return _then(_value.copyWith(
      low: null == low
          ? _value.low
          : low // ignore: cast_nullable_to_non_nullable
              as int,
      high: null == high
          ? _value.high
          : high // ignore: cast_nullable_to_non_nullable
              as int,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$_ValueSubkeyRangeCopyWith<$Res>
    implements $ValueSubkeyRangeCopyWith<$Res> {
  factory _$$_ValueSubkeyRangeCopyWith(
          _$_ValueSubkeyRange value, $Res Function(_$_ValueSubkeyRange) then) =
      __$$_ValueSubkeyRangeCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({int low, int high});
}

/// @nodoc
class __$$_ValueSubkeyRangeCopyWithImpl<$Res>
    extends _$ValueSubkeyRangeCopyWithImpl<$Res, _$_ValueSubkeyRange>
    implements _$$_ValueSubkeyRangeCopyWith<$Res> {
  __$$_ValueSubkeyRangeCopyWithImpl(
      _$_ValueSubkeyRange _value, $Res Function(_$_ValueSubkeyRange) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? low = null,
    Object? high = null,
  }) {
    return _then(_$_ValueSubkeyRange(
      low: null == low
          ? _value.low
          : low // ignore: cast_nullable_to_non_nullable
              as int,
      high: null == high
          ? _value.high
          : high // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$_ValueSubkeyRange implements _ValueSubkeyRange {
  const _$_ValueSubkeyRange({required this.low, required this.high})
      : assert(low < 0 || low > high, 'low out of range'),
        assert(high < 0, 'high out of range');

  factory _$_ValueSubkeyRange.fromJson(Map<String, dynamic> json) =>
      _$$_ValueSubkeyRangeFromJson(json);

  @override
  final int low;
  @override
  final int high;

  @override
  String toString() {
    return 'ValueSubkeyRange(low: $low, high: $high)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$_ValueSubkeyRange &&
            (identical(other.low, low) || other.low == low) &&
            (identical(other.high, high) || other.high == high));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, low, high);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$_ValueSubkeyRangeCopyWith<_$_ValueSubkeyRange> get copyWith =>
      __$$_ValueSubkeyRangeCopyWithImpl<_$_ValueSubkeyRange>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$_ValueSubkeyRangeToJson(
      this,
    );
  }
}

abstract class _ValueSubkeyRange implements ValueSubkeyRange {
  const factory _ValueSubkeyRange(
      {required final int low, required final int high}) = _$_ValueSubkeyRange;

  factory _ValueSubkeyRange.fromJson(Map<String, dynamic> json) =
      _$_ValueSubkeyRange.fromJson;

  @override
  int get low;
  @override
  int get high;
  @override
  @JsonKey(ignore: true)
  _$$_ValueSubkeyRangeCopyWith<_$_ValueSubkeyRange> get copyWith =>
      throw _privateConstructorUsedError;
}

ValueData _$ValueDataFromJson(Map<String, dynamic> json) {
  return _ValueData.fromJson(json);
}

/// @nodoc
mixin _$ValueData {
  int get seq => throw _privateConstructorUsedError;
  @Uint8ListJsonConverter.jsIsArray()
  Uint8List get data => throw _privateConstructorUsedError;
  FixedEncodedString43 get writer => throw _privateConstructorUsedError;

  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
  @JsonKey(ignore: true)
  $ValueDataCopyWith<ValueData> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $ValueDataCopyWith<$Res> {
  factory $ValueDataCopyWith(ValueData value, $Res Function(ValueData) then) =
      _$ValueDataCopyWithImpl<$Res, ValueData>;
  @useResult
  $Res call(
      {int seq,
      @Uint8ListJsonConverter.jsIsArray() Uint8List data,
      FixedEncodedString43 writer});
}

/// @nodoc
class _$ValueDataCopyWithImpl<$Res, $Val extends ValueData>
    implements $ValueDataCopyWith<$Res> {
  _$ValueDataCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? seq = null,
    Object? data = null,
    Object? writer = null,
  }) {
    return _then(_value.copyWith(
      seq: null == seq
          ? _value.seq
          : seq // ignore: cast_nullable_to_non_nullable
              as int,
      data: null == data
          ? _value.data
          : data // ignore: cast_nullable_to_non_nullable
              as Uint8List,
      writer: null == writer
          ? _value.writer
          : writer // ignore: cast_nullable_to_non_nullable
              as FixedEncodedString43,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$_ValueDataCopyWith<$Res> implements $ValueDataCopyWith<$Res> {
  factory _$$_ValueDataCopyWith(
          _$_ValueData value, $Res Function(_$_ValueData) then) =
      __$$_ValueDataCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {int seq,
      @Uint8ListJsonConverter.jsIsArray() Uint8List data,
      FixedEncodedString43 writer});
}

/// @nodoc
class __$$_ValueDataCopyWithImpl<$Res>
    extends _$ValueDataCopyWithImpl<$Res, _$_ValueData>
    implements _$$_ValueDataCopyWith<$Res> {
  __$$_ValueDataCopyWithImpl(
      _$_ValueData _value, $Res Function(_$_ValueData) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? seq = null,
    Object? data = null,
    Object? writer = null,
  }) {
    return _then(_$_ValueData(
      seq: null == seq
          ? _value.seq
          : seq // ignore: cast_nullable_to_non_nullable
              as int,
      data: null == data
          ? _value.data
          : data // ignore: cast_nullable_to_non_nullable
              as Uint8List,
      writer: null == writer
          ? _value.writer
          : writer // ignore: cast_nullable_to_non_nullable
              as FixedEncodedString43,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$_ValueData implements _ValueData {
  const _$_ValueData(
      {required this.seq,
      @Uint8ListJsonConverter.jsIsArray() required this.data,
      required this.writer})
      : assert(seq >= 0, 'seq out of range');

  factory _$_ValueData.fromJson(Map<String, dynamic> json) =>
      _$$_ValueDataFromJson(json);

  @override
  final int seq;
  @override
  @Uint8ListJsonConverter.jsIsArray()
  final Uint8List data;
  @override
  final FixedEncodedString43 writer;

  @override
  String toString() {
    return 'ValueData(seq: $seq, data: $data, writer: $writer)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$_ValueData &&
            (identical(other.seq, seq) || other.seq == seq) &&
            const DeepCollectionEquality().equals(other.data, data) &&
            (identical(other.writer, writer) || other.writer == writer));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(
      runtimeType, seq, const DeepCollectionEquality().hash(data), writer);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$_ValueDataCopyWith<_$_ValueData> get copyWith =>
      __$$_ValueDataCopyWithImpl<_$_ValueData>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$_ValueDataToJson(
      this,
    );
  }
}

abstract class _ValueData implements ValueData {
  const factory _ValueData(
      {required final int seq,
      @Uint8ListJsonConverter.jsIsArray() required final Uint8List data,
      required final FixedEncodedString43 writer}) = _$_ValueData;

  factory _ValueData.fromJson(Map<String, dynamic> json) =
      _$_ValueData.fromJson;

  @override
  int get seq;
  @override
  @Uint8ListJsonConverter.jsIsArray()
  Uint8List get data;
  @override
  FixedEncodedString43 get writer;
  @override
  @JsonKey(ignore: true)
  _$$_ValueDataCopyWith<_$_ValueData> get copyWith =>
      throw _privateConstructorUsedError;
}

SafetySpec _$SafetySpecFromJson(Map<String, dynamic> json) {
  return _SafetySpec.fromJson(json);
}

/// @nodoc
mixin _$SafetySpec {
  int get hopCount => throw _privateConstructorUsedError;
  Stability get stability => throw _privateConstructorUsedError;
  Sequencing get sequencing => throw _privateConstructorUsedError;
  String? get preferredRoute => throw _privateConstructorUsedError;

  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
  @JsonKey(ignore: true)
  $SafetySpecCopyWith<SafetySpec> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $SafetySpecCopyWith<$Res> {
  factory $SafetySpecCopyWith(
          SafetySpec value, $Res Function(SafetySpec) then) =
      _$SafetySpecCopyWithImpl<$Res, SafetySpec>;
  @useResult
  $Res call(
      {int hopCount,
      Stability stability,
      Sequencing sequencing,
      String? preferredRoute});
}

/// @nodoc
class _$SafetySpecCopyWithImpl<$Res, $Val extends SafetySpec>
    implements $SafetySpecCopyWith<$Res> {
  _$SafetySpecCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? hopCount = null,
    Object? stability = null,
    Object? sequencing = null,
    Object? preferredRoute = freezed,
  }) {
    return _then(_value.copyWith(
      hopCount: null == hopCount
          ? _value.hopCount
          : hopCount // ignore: cast_nullable_to_non_nullable
              as int,
      stability: null == stability
          ? _value.stability
          : stability // ignore: cast_nullable_to_non_nullable
              as Stability,
      sequencing: null == sequencing
          ? _value.sequencing
          : sequencing // ignore: cast_nullable_to_non_nullable
              as Sequencing,
      preferredRoute: freezed == preferredRoute
          ? _value.preferredRoute
          : preferredRoute // ignore: cast_nullable_to_non_nullable
              as String?,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$_SafetySpecCopyWith<$Res>
    implements $SafetySpecCopyWith<$Res> {
  factory _$$_SafetySpecCopyWith(
          _$_SafetySpec value, $Res Function(_$_SafetySpec) then) =
      __$$_SafetySpecCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {int hopCount,
      Stability stability,
      Sequencing sequencing,
      String? preferredRoute});
}

/// @nodoc
class __$$_SafetySpecCopyWithImpl<$Res>
    extends _$SafetySpecCopyWithImpl<$Res, _$_SafetySpec>
    implements _$$_SafetySpecCopyWith<$Res> {
  __$$_SafetySpecCopyWithImpl(
      _$_SafetySpec _value, $Res Function(_$_SafetySpec) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? hopCount = null,
    Object? stability = null,
    Object? sequencing = null,
    Object? preferredRoute = freezed,
  }) {
    return _then(_$_SafetySpec(
      hopCount: null == hopCount
          ? _value.hopCount
          : hopCount // ignore: cast_nullable_to_non_nullable
              as int,
      stability: null == stability
          ? _value.stability
          : stability // ignore: cast_nullable_to_non_nullable
              as Stability,
      sequencing: null == sequencing
          ? _value.sequencing
          : sequencing // ignore: cast_nullable_to_non_nullable
              as Sequencing,
      preferredRoute: freezed == preferredRoute
          ? _value.preferredRoute
          : preferredRoute // ignore: cast_nullable_to_non_nullable
              as String?,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$_SafetySpec implements _SafetySpec {
  const _$_SafetySpec(
      {required this.hopCount,
      required this.stability,
      required this.sequencing,
      this.preferredRoute});

  factory _$_SafetySpec.fromJson(Map<String, dynamic> json) =>
      _$$_SafetySpecFromJson(json);

  @override
  final int hopCount;
  @override
  final Stability stability;
  @override
  final Sequencing sequencing;
  @override
  final String? preferredRoute;

  @override
  String toString() {
    return 'SafetySpec(hopCount: $hopCount, stability: $stability, sequencing: $sequencing, preferredRoute: $preferredRoute)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$_SafetySpec &&
            (identical(other.hopCount, hopCount) ||
                other.hopCount == hopCount) &&
            (identical(other.stability, stability) ||
                other.stability == stability) &&
            (identical(other.sequencing, sequencing) ||
                other.sequencing == sequencing) &&
            (identical(other.preferredRoute, preferredRoute) ||
                other.preferredRoute == preferredRoute));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode =>
      Object.hash(runtimeType, hopCount, stability, sequencing, preferredRoute);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$_SafetySpecCopyWith<_$_SafetySpec> get copyWith =>
      __$$_SafetySpecCopyWithImpl<_$_SafetySpec>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$_SafetySpecToJson(
      this,
    );
  }
}

abstract class _SafetySpec implements SafetySpec {
  const factory _SafetySpec(
      {required final int hopCount,
      required final Stability stability,
      required final Sequencing sequencing,
      final String? preferredRoute}) = _$_SafetySpec;

  factory _SafetySpec.fromJson(Map<String, dynamic> json) =
      _$_SafetySpec.fromJson;

  @override
  int get hopCount;
  @override
  Stability get stability;
  @override
  Sequencing get sequencing;
  @override
  String? get preferredRoute;
  @override
  @JsonKey(ignore: true)
  _$$_SafetySpecCopyWith<_$_SafetySpec> get copyWith =>
      throw _privateConstructorUsedError;
}

RouteBlob _$RouteBlobFromJson(Map<String, dynamic> json) {
  return _RouteBlob.fromJson(json);
}

/// @nodoc
mixin _$RouteBlob {
  String get routeId => throw _privateConstructorUsedError;
  @Uint8ListJsonConverter()
  Uint8List get blob => throw _privateConstructorUsedError;

  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
  @JsonKey(ignore: true)
  $RouteBlobCopyWith<RouteBlob> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $RouteBlobCopyWith<$Res> {
  factory $RouteBlobCopyWith(RouteBlob value, $Res Function(RouteBlob) then) =
      _$RouteBlobCopyWithImpl<$Res, RouteBlob>;
  @useResult
  $Res call({String routeId, @Uint8ListJsonConverter() Uint8List blob});
}

/// @nodoc
class _$RouteBlobCopyWithImpl<$Res, $Val extends RouteBlob>
    implements $RouteBlobCopyWith<$Res> {
  _$RouteBlobCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? routeId = null,
    Object? blob = null,
  }) {
    return _then(_value.copyWith(
      routeId: null == routeId
          ? _value.routeId
          : routeId // ignore: cast_nullable_to_non_nullable
              as String,
      blob: null == blob
          ? _value.blob
          : blob // ignore: cast_nullable_to_non_nullable
              as Uint8List,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$_RouteBlobCopyWith<$Res> implements $RouteBlobCopyWith<$Res> {
  factory _$$_RouteBlobCopyWith(
          _$_RouteBlob value, $Res Function(_$_RouteBlob) then) =
      __$$_RouteBlobCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String routeId, @Uint8ListJsonConverter() Uint8List blob});
}

/// @nodoc
class __$$_RouteBlobCopyWithImpl<$Res>
    extends _$RouteBlobCopyWithImpl<$Res, _$_RouteBlob>
    implements _$$_RouteBlobCopyWith<$Res> {
  __$$_RouteBlobCopyWithImpl(
      _$_RouteBlob _value, $Res Function(_$_RouteBlob) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? routeId = null,
    Object? blob = null,
  }) {
    return _then(_$_RouteBlob(
      routeId: null == routeId
          ? _value.routeId
          : routeId // ignore: cast_nullable_to_non_nullable
              as String,
      blob: null == blob
          ? _value.blob
          : blob // ignore: cast_nullable_to_non_nullable
              as Uint8List,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$_RouteBlob implements _RouteBlob {
  const _$_RouteBlob(
      {required this.routeId, @Uint8ListJsonConverter() required this.blob});

  factory _$_RouteBlob.fromJson(Map<String, dynamic> json) =>
      _$$_RouteBlobFromJson(json);

  @override
  final String routeId;
  @override
  @Uint8ListJsonConverter()
  final Uint8List blob;

  @override
  String toString() {
    return 'RouteBlob(routeId: $routeId, blob: $blob)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$_RouteBlob &&
            (identical(other.routeId, routeId) || other.routeId == routeId) &&
            const DeepCollectionEquality().equals(other.blob, blob));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(
      runtimeType, routeId, const DeepCollectionEquality().hash(blob));

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$_RouteBlobCopyWith<_$_RouteBlob> get copyWith =>
      __$$_RouteBlobCopyWithImpl<_$_RouteBlob>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$_RouteBlobToJson(
      this,
    );
  }
}

abstract class _RouteBlob implements RouteBlob {
  const factory _RouteBlob(
      {required final String routeId,
      @Uint8ListJsonConverter() required final Uint8List blob}) = _$_RouteBlob;

  factory _RouteBlob.fromJson(Map<String, dynamic> json) =
      _$_RouteBlob.fromJson;

  @override
  String get routeId;
  @override
  @Uint8ListJsonConverter()
  Uint8List get blob;
  @override
  @JsonKey(ignore: true)
  _$$_RouteBlobCopyWith<_$_RouteBlob> get copyWith =>
      throw _privateConstructorUsedError;
}
