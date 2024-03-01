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
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#adding-getters-and-methods-to-our-models');

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
abstract class _$$DHTSchemaDFLTImplCopyWith<$Res>
    implements $DHTSchemaCopyWith<$Res> {
  factory _$$DHTSchemaDFLTImplCopyWith(
          _$DHTSchemaDFLTImpl value, $Res Function(_$DHTSchemaDFLTImpl) then) =
      __$$DHTSchemaDFLTImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({int oCnt});
}

/// @nodoc
class __$$DHTSchemaDFLTImplCopyWithImpl<$Res>
    extends _$DHTSchemaCopyWithImpl<$Res, _$DHTSchemaDFLTImpl>
    implements _$$DHTSchemaDFLTImplCopyWith<$Res> {
  __$$DHTSchemaDFLTImplCopyWithImpl(
      _$DHTSchemaDFLTImpl _value, $Res Function(_$DHTSchemaDFLTImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? oCnt = null,
  }) {
    return _then(_$DHTSchemaDFLTImpl(
      oCnt: null == oCnt
          ? _value.oCnt
          : oCnt // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$DHTSchemaDFLTImpl implements DHTSchemaDFLT {
  const _$DHTSchemaDFLTImpl({required this.oCnt, final String? $type})
      : $type = $type ?? 'DFLT';

  factory _$DHTSchemaDFLTImpl.fromJson(Map<String, dynamic> json) =>
      _$$DHTSchemaDFLTImplFromJson(json);

  @override
  final int oCnt;

  @JsonKey(name: 'kind')
  final String $type;

  @override
  String toString() {
    return 'DHTSchema.dflt(oCnt: $oCnt)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$DHTSchemaDFLTImpl &&
            (identical(other.oCnt, oCnt) || other.oCnt == oCnt));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, oCnt);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$DHTSchemaDFLTImplCopyWith<_$DHTSchemaDFLTImpl> get copyWith =>
      __$$DHTSchemaDFLTImplCopyWithImpl<_$DHTSchemaDFLTImpl>(this, _$identity);

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
    return _$$DHTSchemaDFLTImplToJson(
      this,
    );
  }
}

abstract class DHTSchemaDFLT implements DHTSchema {
  const factory DHTSchemaDFLT({required final int oCnt}) = _$DHTSchemaDFLTImpl;

  factory DHTSchemaDFLT.fromJson(Map<String, dynamic> json) =
      _$DHTSchemaDFLTImpl.fromJson;

  @override
  int get oCnt;
  @override
  @JsonKey(ignore: true)
  _$$DHTSchemaDFLTImplCopyWith<_$DHTSchemaDFLTImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$DHTSchemaSMPLImplCopyWith<$Res>
    implements $DHTSchemaCopyWith<$Res> {
  factory _$$DHTSchemaSMPLImplCopyWith(
          _$DHTSchemaSMPLImpl value, $Res Function(_$DHTSchemaSMPLImpl) then) =
      __$$DHTSchemaSMPLImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({int oCnt, List<DHTSchemaMember> members});
}

/// @nodoc
class __$$DHTSchemaSMPLImplCopyWithImpl<$Res>
    extends _$DHTSchemaCopyWithImpl<$Res, _$DHTSchemaSMPLImpl>
    implements _$$DHTSchemaSMPLImplCopyWith<$Res> {
  __$$DHTSchemaSMPLImplCopyWithImpl(
      _$DHTSchemaSMPLImpl _value, $Res Function(_$DHTSchemaSMPLImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? oCnt = null,
    Object? members = null,
  }) {
    return _then(_$DHTSchemaSMPLImpl(
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
class _$DHTSchemaSMPLImpl implements DHTSchemaSMPL {
  const _$DHTSchemaSMPLImpl(
      {required this.oCnt,
      required final List<DHTSchemaMember> members,
      final String? $type})
      : _members = members,
        $type = $type ?? 'SMPL';

  factory _$DHTSchemaSMPLImpl.fromJson(Map<String, dynamic> json) =>
      _$$DHTSchemaSMPLImplFromJson(json);

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
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$DHTSchemaSMPLImpl &&
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
  _$$DHTSchemaSMPLImplCopyWith<_$DHTSchemaSMPLImpl> get copyWith =>
      __$$DHTSchemaSMPLImplCopyWithImpl<_$DHTSchemaSMPLImpl>(this, _$identity);

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
    return _$$DHTSchemaSMPLImplToJson(
      this,
    );
  }
}

abstract class DHTSchemaSMPL implements DHTSchema {
  const factory DHTSchemaSMPL(
      {required final int oCnt,
      required final List<DHTSchemaMember> members}) = _$DHTSchemaSMPLImpl;

  factory DHTSchemaSMPL.fromJson(Map<String, dynamic> json) =
      _$DHTSchemaSMPLImpl.fromJson;

  @override
  int get oCnt;
  List<DHTSchemaMember> get members;
  @override
  @JsonKey(ignore: true)
  _$$DHTSchemaSMPLImplCopyWith<_$DHTSchemaSMPLImpl> get copyWith =>
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
abstract class _$$DHTSchemaMemberImplCopyWith<$Res>
    implements $DHTSchemaMemberCopyWith<$Res> {
  factory _$$DHTSchemaMemberImplCopyWith(_$DHTSchemaMemberImpl value,
          $Res Function(_$DHTSchemaMemberImpl) then) =
      __$$DHTSchemaMemberImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({FixedEncodedString43 mKey, int mCnt});
}

/// @nodoc
class __$$DHTSchemaMemberImplCopyWithImpl<$Res>
    extends _$DHTSchemaMemberCopyWithImpl<$Res, _$DHTSchemaMemberImpl>
    implements _$$DHTSchemaMemberImplCopyWith<$Res> {
  __$$DHTSchemaMemberImplCopyWithImpl(
      _$DHTSchemaMemberImpl _value, $Res Function(_$DHTSchemaMemberImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? mKey = null,
    Object? mCnt = null,
  }) {
    return _then(_$DHTSchemaMemberImpl(
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
class _$DHTSchemaMemberImpl implements _DHTSchemaMember {
  const _$DHTSchemaMemberImpl({required this.mKey, required this.mCnt})
      : assert(mCnt > 0 && mCnt <= 65535, 'value out of range');

  factory _$DHTSchemaMemberImpl.fromJson(Map<String, dynamic> json) =>
      _$$DHTSchemaMemberImplFromJson(json);

  @override
  final FixedEncodedString43 mKey;
  @override
  final int mCnt;

  @override
  String toString() {
    return 'DHTSchemaMember(mKey: $mKey, mCnt: $mCnt)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$DHTSchemaMemberImpl &&
            (identical(other.mKey, mKey) || other.mKey == mKey) &&
            (identical(other.mCnt, mCnt) || other.mCnt == mCnt));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, mKey, mCnt);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$DHTSchemaMemberImplCopyWith<_$DHTSchemaMemberImpl> get copyWith =>
      __$$DHTSchemaMemberImplCopyWithImpl<_$DHTSchemaMemberImpl>(
          this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$DHTSchemaMemberImplToJson(
      this,
    );
  }
}

abstract class _DHTSchemaMember implements DHTSchemaMember {
  const factory _DHTSchemaMember(
      {required final FixedEncodedString43 mKey,
      required final int mCnt}) = _$DHTSchemaMemberImpl;

  factory _DHTSchemaMember.fromJson(Map<String, dynamic> json) =
      _$DHTSchemaMemberImpl.fromJson;

  @override
  FixedEncodedString43 get mKey;
  @override
  int get mCnt;
  @override
  @JsonKey(ignore: true)
  _$$DHTSchemaMemberImplCopyWith<_$DHTSchemaMemberImpl> get copyWith =>
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
abstract class _$$DHTRecordDescriptorImplCopyWith<$Res>
    implements $DHTRecordDescriptorCopyWith<$Res> {
  factory _$$DHTRecordDescriptorImplCopyWith(_$DHTRecordDescriptorImpl value,
          $Res Function(_$DHTRecordDescriptorImpl) then) =
      __$$DHTRecordDescriptorImplCopyWithImpl<$Res>;
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
class __$$DHTRecordDescriptorImplCopyWithImpl<$Res>
    extends _$DHTRecordDescriptorCopyWithImpl<$Res, _$DHTRecordDescriptorImpl>
    implements _$$DHTRecordDescriptorImplCopyWith<$Res> {
  __$$DHTRecordDescriptorImplCopyWithImpl(_$DHTRecordDescriptorImpl _value,
      $Res Function(_$DHTRecordDescriptorImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? key = null,
    Object? owner = null,
    Object? schema = null,
    Object? ownerSecret = freezed,
  }) {
    return _then(_$DHTRecordDescriptorImpl(
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
class _$DHTRecordDescriptorImpl implements _DHTRecordDescriptor {
  const _$DHTRecordDescriptorImpl(
      {required this.key,
      required this.owner,
      required this.schema,
      this.ownerSecret});

  factory _$DHTRecordDescriptorImpl.fromJson(Map<String, dynamic> json) =>
      _$$DHTRecordDescriptorImplFromJson(json);

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
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$DHTRecordDescriptorImpl &&
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
  _$$DHTRecordDescriptorImplCopyWith<_$DHTRecordDescriptorImpl> get copyWith =>
      __$$DHTRecordDescriptorImplCopyWithImpl<_$DHTRecordDescriptorImpl>(
          this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$DHTRecordDescriptorImplToJson(
      this,
    );
  }
}

abstract class _DHTRecordDescriptor implements DHTRecordDescriptor {
  const factory _DHTRecordDescriptor(
      {required final Typed<FixedEncodedString43> key,
      required final FixedEncodedString43 owner,
      required final DHTSchema schema,
      final FixedEncodedString43? ownerSecret}) = _$DHTRecordDescriptorImpl;

  factory _DHTRecordDescriptor.fromJson(Map<String, dynamic> json) =
      _$DHTRecordDescriptorImpl.fromJson;

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
  _$$DHTRecordDescriptorImplCopyWith<_$DHTRecordDescriptorImpl> get copyWith =>
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
abstract class _$$ValueDataImplCopyWith<$Res>
    implements $ValueDataCopyWith<$Res> {
  factory _$$ValueDataImplCopyWith(
          _$ValueDataImpl value, $Res Function(_$ValueDataImpl) then) =
      __$$ValueDataImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {int seq,
      @Uint8ListJsonConverter.jsIsArray() Uint8List data,
      FixedEncodedString43 writer});
}

/// @nodoc
class __$$ValueDataImplCopyWithImpl<$Res>
    extends _$ValueDataCopyWithImpl<$Res, _$ValueDataImpl>
    implements _$$ValueDataImplCopyWith<$Res> {
  __$$ValueDataImplCopyWithImpl(
      _$ValueDataImpl _value, $Res Function(_$ValueDataImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? seq = null,
    Object? data = null,
    Object? writer = null,
  }) {
    return _then(_$ValueDataImpl(
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
class _$ValueDataImpl implements _ValueData {
  const _$ValueDataImpl(
      {required this.seq,
      @Uint8ListJsonConverter.jsIsArray() required this.data,
      required this.writer})
      : assert(seq >= 0, 'seq out of range');

  factory _$ValueDataImpl.fromJson(Map<String, dynamic> json) =>
      _$$ValueDataImplFromJson(json);

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
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ValueDataImpl &&
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
  _$$ValueDataImplCopyWith<_$ValueDataImpl> get copyWith =>
      __$$ValueDataImplCopyWithImpl<_$ValueDataImpl>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$ValueDataImplToJson(
      this,
    );
  }
}

abstract class _ValueData implements ValueData {
  const factory _ValueData(
      {required final int seq,
      @Uint8ListJsonConverter.jsIsArray() required final Uint8List data,
      required final FixedEncodedString43 writer}) = _$ValueDataImpl;

  factory _ValueData.fromJson(Map<String, dynamic> json) =
      _$ValueDataImpl.fromJson;

  @override
  int get seq;
  @override
  @Uint8ListJsonConverter.jsIsArray()
  Uint8List get data;
  @override
  FixedEncodedString43 get writer;
  @override
  @JsonKey(ignore: true)
  _$$ValueDataImplCopyWith<_$ValueDataImpl> get copyWith =>
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
abstract class _$$SafetySpecImplCopyWith<$Res>
    implements $SafetySpecCopyWith<$Res> {
  factory _$$SafetySpecImplCopyWith(
          _$SafetySpecImpl value, $Res Function(_$SafetySpecImpl) then) =
      __$$SafetySpecImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {int hopCount,
      Stability stability,
      Sequencing sequencing,
      String? preferredRoute});
}

/// @nodoc
class __$$SafetySpecImplCopyWithImpl<$Res>
    extends _$SafetySpecCopyWithImpl<$Res, _$SafetySpecImpl>
    implements _$$SafetySpecImplCopyWith<$Res> {
  __$$SafetySpecImplCopyWithImpl(
      _$SafetySpecImpl _value, $Res Function(_$SafetySpecImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? hopCount = null,
    Object? stability = null,
    Object? sequencing = null,
    Object? preferredRoute = freezed,
  }) {
    return _then(_$SafetySpecImpl(
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
class _$SafetySpecImpl implements _SafetySpec {
  const _$SafetySpecImpl(
      {required this.hopCount,
      required this.stability,
      required this.sequencing,
      this.preferredRoute});

  factory _$SafetySpecImpl.fromJson(Map<String, dynamic> json) =>
      _$$SafetySpecImplFromJson(json);

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
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$SafetySpecImpl &&
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
  _$$SafetySpecImplCopyWith<_$SafetySpecImpl> get copyWith =>
      __$$SafetySpecImplCopyWithImpl<_$SafetySpecImpl>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$SafetySpecImplToJson(
      this,
    );
  }
}

abstract class _SafetySpec implements SafetySpec {
  const factory _SafetySpec(
      {required final int hopCount,
      required final Stability stability,
      required final Sequencing sequencing,
      final String? preferredRoute}) = _$SafetySpecImpl;

  factory _SafetySpec.fromJson(Map<String, dynamic> json) =
      _$SafetySpecImpl.fromJson;

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
  _$$SafetySpecImplCopyWith<_$SafetySpecImpl> get copyWith =>
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
abstract class _$$RouteBlobImplCopyWith<$Res>
    implements $RouteBlobCopyWith<$Res> {
  factory _$$RouteBlobImplCopyWith(
          _$RouteBlobImpl value, $Res Function(_$RouteBlobImpl) then) =
      __$$RouteBlobImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String routeId, @Uint8ListJsonConverter() Uint8List blob});
}

/// @nodoc
class __$$RouteBlobImplCopyWithImpl<$Res>
    extends _$RouteBlobCopyWithImpl<$Res, _$RouteBlobImpl>
    implements _$$RouteBlobImplCopyWith<$Res> {
  __$$RouteBlobImplCopyWithImpl(
      _$RouteBlobImpl _value, $Res Function(_$RouteBlobImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? routeId = null,
    Object? blob = null,
  }) {
    return _then(_$RouteBlobImpl(
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
class _$RouteBlobImpl implements _RouteBlob {
  const _$RouteBlobImpl(
      {required this.routeId, @Uint8ListJsonConverter() required this.blob});

  factory _$RouteBlobImpl.fromJson(Map<String, dynamic> json) =>
      _$$RouteBlobImplFromJson(json);

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
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$RouteBlobImpl &&
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
  _$$RouteBlobImplCopyWith<_$RouteBlobImpl> get copyWith =>
      __$$RouteBlobImplCopyWithImpl<_$RouteBlobImpl>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$RouteBlobImplToJson(
      this,
    );
  }
}

abstract class _RouteBlob implements RouteBlob {
  const factory _RouteBlob(
          {required final String routeId,
          @Uint8ListJsonConverter() required final Uint8List blob}) =
      _$RouteBlobImpl;

  factory _RouteBlob.fromJson(Map<String, dynamic> json) =
      _$RouteBlobImpl.fromJson;

  @override
  String get routeId;
  @override
  @Uint8ListJsonConverter()
  Uint8List get blob;
  @override
  @JsonKey(ignore: true)
  _$$RouteBlobImplCopyWith<_$RouteBlobImpl> get copyWith =>
      throw _privateConstructorUsedError;
}
