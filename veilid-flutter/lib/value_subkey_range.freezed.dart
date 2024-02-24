// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'value_subkey_range.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#custom-getters-and-methods');

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
      : assert(low >= 0 && low <= high, 'range is invalid');

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
