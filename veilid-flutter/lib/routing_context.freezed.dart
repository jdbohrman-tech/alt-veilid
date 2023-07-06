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
      : assert(mCnt >= 0 && mCnt <= 65535, 'value out of range');

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
