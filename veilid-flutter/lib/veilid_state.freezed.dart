// dart format width=80
// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'veilid_state.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$LatencyStats {
  TimestampDuration get fastest;
  TimestampDuration get average;
  TimestampDuration get slowest;
  TimestampDuration get tm90;
  TimestampDuration get tm75;
  TimestampDuration get p90;
  TimestampDuration get p75;

  /// Create a copy of LatencyStats
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $LatencyStatsCopyWith<LatencyStats> get copyWith =>
      _$LatencyStatsCopyWithImpl<LatencyStats>(
          this as LatencyStats, _$identity);

  /// Serializes this LatencyStats to a JSON map.
  Map<String, dynamic> toJson();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is LatencyStats &&
            (identical(other.fastest, fastest) || other.fastest == fastest) &&
            (identical(other.average, average) || other.average == average) &&
            (identical(other.slowest, slowest) || other.slowest == slowest) &&
            (identical(other.tm90, tm90) || other.tm90 == tm90) &&
            (identical(other.tm75, tm75) || other.tm75 == tm75) &&
            (identical(other.p90, p90) || other.p90 == p90) &&
            (identical(other.p75, p75) || other.p75 == p75));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode =>
      Object.hash(runtimeType, fastest, average, slowest, tm90, tm75, p90, p75);

  @override
  String toString() {
    return 'LatencyStats(fastest: $fastest, average: $average, slowest: $slowest, tm90: $tm90, tm75: $tm75, p90: $p90, p75: $p75)';
  }
}

/// @nodoc
abstract mixin class $LatencyStatsCopyWith<$Res> {
  factory $LatencyStatsCopyWith(
          LatencyStats value, $Res Function(LatencyStats) _then) =
      _$LatencyStatsCopyWithImpl;
  @useResult
  $Res call(
      {TimestampDuration fastest,
      TimestampDuration average,
      TimestampDuration slowest,
      TimestampDuration tm90,
      TimestampDuration tm75,
      TimestampDuration p90,
      TimestampDuration p75});
}

/// @nodoc
class _$LatencyStatsCopyWithImpl<$Res> implements $LatencyStatsCopyWith<$Res> {
  _$LatencyStatsCopyWithImpl(this._self, this._then);

  final LatencyStats _self;
  final $Res Function(LatencyStats) _then;

  /// Create a copy of LatencyStats
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? fastest = null,
    Object? average = null,
    Object? slowest = null,
    Object? tm90 = null,
    Object? tm75 = null,
    Object? p90 = null,
    Object? p75 = null,
  }) {
    return _then(_self.copyWith(
      fastest: null == fastest
          ? _self.fastest
          : fastest // ignore: cast_nullable_to_non_nullable
              as TimestampDuration,
      average: null == average
          ? _self.average
          : average // ignore: cast_nullable_to_non_nullable
              as TimestampDuration,
      slowest: null == slowest
          ? _self.slowest
          : slowest // ignore: cast_nullable_to_non_nullable
              as TimestampDuration,
      tm90: null == tm90
          ? _self.tm90
          : tm90 // ignore: cast_nullable_to_non_nullable
              as TimestampDuration,
      tm75: null == tm75
          ? _self.tm75
          : tm75 // ignore: cast_nullable_to_non_nullable
              as TimestampDuration,
      p90: null == p90
          ? _self.p90
          : p90 // ignore: cast_nullable_to_non_nullable
              as TimestampDuration,
      p75: null == p75
          ? _self.p75
          : p75 // ignore: cast_nullable_to_non_nullable
              as TimestampDuration,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _LatencyStats implements LatencyStats {
  const _LatencyStats(
      {required this.fastest,
      required this.average,
      required this.slowest,
      required this.tm90,
      required this.tm75,
      required this.p90,
      required this.p75});
  factory _LatencyStats.fromJson(Map<String, dynamic> json) =>
      _$LatencyStatsFromJson(json);

  @override
  final TimestampDuration fastest;
  @override
  final TimestampDuration average;
  @override
  final TimestampDuration slowest;
  @override
  final TimestampDuration tm90;
  @override
  final TimestampDuration tm75;
  @override
  final TimestampDuration p90;
  @override
  final TimestampDuration p75;

  /// Create a copy of LatencyStats
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  _$LatencyStatsCopyWith<_LatencyStats> get copyWith =>
      __$LatencyStatsCopyWithImpl<_LatencyStats>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$LatencyStatsToJson(
      this,
    );
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _LatencyStats &&
            (identical(other.fastest, fastest) || other.fastest == fastest) &&
            (identical(other.average, average) || other.average == average) &&
            (identical(other.slowest, slowest) || other.slowest == slowest) &&
            (identical(other.tm90, tm90) || other.tm90 == tm90) &&
            (identical(other.tm75, tm75) || other.tm75 == tm75) &&
            (identical(other.p90, p90) || other.p90 == p90) &&
            (identical(other.p75, p75) || other.p75 == p75));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode =>
      Object.hash(runtimeType, fastest, average, slowest, tm90, tm75, p90, p75);

  @override
  String toString() {
    return 'LatencyStats(fastest: $fastest, average: $average, slowest: $slowest, tm90: $tm90, tm75: $tm75, p90: $p90, p75: $p75)';
  }
}

/// @nodoc
abstract mixin class _$LatencyStatsCopyWith<$Res>
    implements $LatencyStatsCopyWith<$Res> {
  factory _$LatencyStatsCopyWith(
          _LatencyStats value, $Res Function(_LatencyStats) _then) =
      __$LatencyStatsCopyWithImpl;
  @override
  @useResult
  $Res call(
      {TimestampDuration fastest,
      TimestampDuration average,
      TimestampDuration slowest,
      TimestampDuration tm90,
      TimestampDuration tm75,
      TimestampDuration p90,
      TimestampDuration p75});
}

/// @nodoc
class __$LatencyStatsCopyWithImpl<$Res>
    implements _$LatencyStatsCopyWith<$Res> {
  __$LatencyStatsCopyWithImpl(this._self, this._then);

  final _LatencyStats _self;
  final $Res Function(_LatencyStats) _then;

  /// Create a copy of LatencyStats
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? fastest = null,
    Object? average = null,
    Object? slowest = null,
    Object? tm90 = null,
    Object? tm75 = null,
    Object? p90 = null,
    Object? p75 = null,
  }) {
    return _then(_LatencyStats(
      fastest: null == fastest
          ? _self.fastest
          : fastest // ignore: cast_nullable_to_non_nullable
              as TimestampDuration,
      average: null == average
          ? _self.average
          : average // ignore: cast_nullable_to_non_nullable
              as TimestampDuration,
      slowest: null == slowest
          ? _self.slowest
          : slowest // ignore: cast_nullable_to_non_nullable
              as TimestampDuration,
      tm90: null == tm90
          ? _self.tm90
          : tm90 // ignore: cast_nullable_to_non_nullable
              as TimestampDuration,
      tm75: null == tm75
          ? _self.tm75
          : tm75 // ignore: cast_nullable_to_non_nullable
              as TimestampDuration,
      p90: null == p90
          ? _self.p90
          : p90 // ignore: cast_nullable_to_non_nullable
              as TimestampDuration,
      p75: null == p75
          ? _self.p75
          : p75 // ignore: cast_nullable_to_non_nullable
              as TimestampDuration,
    ));
  }
}

/// @nodoc
mixin _$TransferStats {
  BigInt get total;
  BigInt get maximum;
  BigInt get average;
  BigInt get minimum;

  /// Create a copy of TransferStats
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $TransferStatsCopyWith<TransferStats> get copyWith =>
      _$TransferStatsCopyWithImpl<TransferStats>(
          this as TransferStats, _$identity);

  /// Serializes this TransferStats to a JSON map.
  Map<String, dynamic> toJson();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is TransferStats &&
            (identical(other.total, total) || other.total == total) &&
            (identical(other.maximum, maximum) || other.maximum == maximum) &&
            (identical(other.average, average) || other.average == average) &&
            (identical(other.minimum, minimum) || other.minimum == minimum));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode =>
      Object.hash(runtimeType, total, maximum, average, minimum);

  @override
  String toString() {
    return 'TransferStats(total: $total, maximum: $maximum, average: $average, minimum: $minimum)';
  }
}

/// @nodoc
abstract mixin class $TransferStatsCopyWith<$Res> {
  factory $TransferStatsCopyWith(
          TransferStats value, $Res Function(TransferStats) _then) =
      _$TransferStatsCopyWithImpl;
  @useResult
  $Res call({BigInt total, BigInt maximum, BigInt average, BigInt minimum});
}

/// @nodoc
class _$TransferStatsCopyWithImpl<$Res>
    implements $TransferStatsCopyWith<$Res> {
  _$TransferStatsCopyWithImpl(this._self, this._then);

  final TransferStats _self;
  final $Res Function(TransferStats) _then;

  /// Create a copy of TransferStats
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? total = null,
    Object? maximum = null,
    Object? average = null,
    Object? minimum = null,
  }) {
    return _then(_self.copyWith(
      total: null == total
          ? _self.total
          : total // ignore: cast_nullable_to_non_nullable
              as BigInt,
      maximum: null == maximum
          ? _self.maximum
          : maximum // ignore: cast_nullable_to_non_nullable
              as BigInt,
      average: null == average
          ? _self.average
          : average // ignore: cast_nullable_to_non_nullable
              as BigInt,
      minimum: null == minimum
          ? _self.minimum
          : minimum // ignore: cast_nullable_to_non_nullable
              as BigInt,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _TransferStats implements TransferStats {
  const _TransferStats(
      {required this.total,
      required this.maximum,
      required this.average,
      required this.minimum});
  factory _TransferStats.fromJson(Map<String, dynamic> json) =>
      _$TransferStatsFromJson(json);

  @override
  final BigInt total;
  @override
  final BigInt maximum;
  @override
  final BigInt average;
  @override
  final BigInt minimum;

  /// Create a copy of TransferStats
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  _$TransferStatsCopyWith<_TransferStats> get copyWith =>
      __$TransferStatsCopyWithImpl<_TransferStats>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$TransferStatsToJson(
      this,
    );
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _TransferStats &&
            (identical(other.total, total) || other.total == total) &&
            (identical(other.maximum, maximum) || other.maximum == maximum) &&
            (identical(other.average, average) || other.average == average) &&
            (identical(other.minimum, minimum) || other.minimum == minimum));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode =>
      Object.hash(runtimeType, total, maximum, average, minimum);

  @override
  String toString() {
    return 'TransferStats(total: $total, maximum: $maximum, average: $average, minimum: $minimum)';
  }
}

/// @nodoc
abstract mixin class _$TransferStatsCopyWith<$Res>
    implements $TransferStatsCopyWith<$Res> {
  factory _$TransferStatsCopyWith(
          _TransferStats value, $Res Function(_TransferStats) _then) =
      __$TransferStatsCopyWithImpl;
  @override
  @useResult
  $Res call({BigInt total, BigInt maximum, BigInt average, BigInt minimum});
}

/// @nodoc
class __$TransferStatsCopyWithImpl<$Res>
    implements _$TransferStatsCopyWith<$Res> {
  __$TransferStatsCopyWithImpl(this._self, this._then);

  final _TransferStats _self;
  final $Res Function(_TransferStats) _then;

  /// Create a copy of TransferStats
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? total = null,
    Object? maximum = null,
    Object? average = null,
    Object? minimum = null,
  }) {
    return _then(_TransferStats(
      total: null == total
          ? _self.total
          : total // ignore: cast_nullable_to_non_nullable
              as BigInt,
      maximum: null == maximum
          ? _self.maximum
          : maximum // ignore: cast_nullable_to_non_nullable
              as BigInt,
      average: null == average
          ? _self.average
          : average // ignore: cast_nullable_to_non_nullable
              as BigInt,
      minimum: null == minimum
          ? _self.minimum
          : minimum // ignore: cast_nullable_to_non_nullable
              as BigInt,
    ));
  }
}

/// @nodoc
mixin _$TransferStatsDownUp {
  TransferStats get down;
  TransferStats get up;

  /// Create a copy of TransferStatsDownUp
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $TransferStatsDownUpCopyWith<TransferStatsDownUp> get copyWith =>
      _$TransferStatsDownUpCopyWithImpl<TransferStatsDownUp>(
          this as TransferStatsDownUp, _$identity);

  /// Serializes this TransferStatsDownUp to a JSON map.
  Map<String, dynamic> toJson();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is TransferStatsDownUp &&
            (identical(other.down, down) || other.down == down) &&
            (identical(other.up, up) || other.up == up));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, down, up);

  @override
  String toString() {
    return 'TransferStatsDownUp(down: $down, up: $up)';
  }
}

/// @nodoc
abstract mixin class $TransferStatsDownUpCopyWith<$Res> {
  factory $TransferStatsDownUpCopyWith(
          TransferStatsDownUp value, $Res Function(TransferStatsDownUp) _then) =
      _$TransferStatsDownUpCopyWithImpl;
  @useResult
  $Res call({TransferStats down, TransferStats up});

  $TransferStatsCopyWith<$Res> get down;
  $TransferStatsCopyWith<$Res> get up;
}

/// @nodoc
class _$TransferStatsDownUpCopyWithImpl<$Res>
    implements $TransferStatsDownUpCopyWith<$Res> {
  _$TransferStatsDownUpCopyWithImpl(this._self, this._then);

  final TransferStatsDownUp _self;
  final $Res Function(TransferStatsDownUp) _then;

  /// Create a copy of TransferStatsDownUp
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? down = null,
    Object? up = null,
  }) {
    return _then(_self.copyWith(
      down: null == down
          ? _self.down
          : down // ignore: cast_nullable_to_non_nullable
              as TransferStats,
      up: null == up
          ? _self.up
          : up // ignore: cast_nullable_to_non_nullable
              as TransferStats,
    ));
  }

  /// Create a copy of TransferStatsDownUp
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $TransferStatsCopyWith<$Res> get down {
    return $TransferStatsCopyWith<$Res>(_self.down, (value) {
      return _then(_self.copyWith(down: value));
    });
  }

  /// Create a copy of TransferStatsDownUp
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $TransferStatsCopyWith<$Res> get up {
    return $TransferStatsCopyWith<$Res>(_self.up, (value) {
      return _then(_self.copyWith(up: value));
    });
  }
}

/// @nodoc
@JsonSerializable()
class _TransferStatsDownUp implements TransferStatsDownUp {
  const _TransferStatsDownUp({required this.down, required this.up});
  factory _TransferStatsDownUp.fromJson(Map<String, dynamic> json) =>
      _$TransferStatsDownUpFromJson(json);

  @override
  final TransferStats down;
  @override
  final TransferStats up;

  /// Create a copy of TransferStatsDownUp
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  _$TransferStatsDownUpCopyWith<_TransferStatsDownUp> get copyWith =>
      __$TransferStatsDownUpCopyWithImpl<_TransferStatsDownUp>(
          this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$TransferStatsDownUpToJson(
      this,
    );
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _TransferStatsDownUp &&
            (identical(other.down, down) || other.down == down) &&
            (identical(other.up, up) || other.up == up));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, down, up);

  @override
  String toString() {
    return 'TransferStatsDownUp(down: $down, up: $up)';
  }
}

/// @nodoc
abstract mixin class _$TransferStatsDownUpCopyWith<$Res>
    implements $TransferStatsDownUpCopyWith<$Res> {
  factory _$TransferStatsDownUpCopyWith(_TransferStatsDownUp value,
          $Res Function(_TransferStatsDownUp) _then) =
      __$TransferStatsDownUpCopyWithImpl;
  @override
  @useResult
  $Res call({TransferStats down, TransferStats up});

  @override
  $TransferStatsCopyWith<$Res> get down;
  @override
  $TransferStatsCopyWith<$Res> get up;
}

/// @nodoc
class __$TransferStatsDownUpCopyWithImpl<$Res>
    implements _$TransferStatsDownUpCopyWith<$Res> {
  __$TransferStatsDownUpCopyWithImpl(this._self, this._then);

  final _TransferStatsDownUp _self;
  final $Res Function(_TransferStatsDownUp) _then;

  /// Create a copy of TransferStatsDownUp
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? down = null,
    Object? up = null,
  }) {
    return _then(_TransferStatsDownUp(
      down: null == down
          ? _self.down
          : down // ignore: cast_nullable_to_non_nullable
              as TransferStats,
      up: null == up
          ? _self.up
          : up // ignore: cast_nullable_to_non_nullable
              as TransferStats,
    ));
  }

  /// Create a copy of TransferStatsDownUp
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $TransferStatsCopyWith<$Res> get down {
    return $TransferStatsCopyWith<$Res>(_self.down, (value) {
      return _then(_self.copyWith(down: value));
    });
  }

  /// Create a copy of TransferStatsDownUp
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $TransferStatsCopyWith<$Res> get up {
    return $TransferStatsCopyWith<$Res>(_self.up, (value) {
      return _then(_self.copyWith(up: value));
    });
  }
}

/// @nodoc
mixin _$StateStats {
  TimestampDuration get span;
  TimestampDuration get reliable;
  TimestampDuration get unreliable;
  TimestampDuration get dead;
  TimestampDuration get punished;
  StateReasonStats get reason;

  /// Create a copy of StateStats
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $StateStatsCopyWith<StateStats> get copyWith =>
      _$StateStatsCopyWithImpl<StateStats>(this as StateStats, _$identity);

  /// Serializes this StateStats to a JSON map.
  Map<String, dynamic> toJson();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is StateStats &&
            (identical(other.span, span) || other.span == span) &&
            (identical(other.reliable, reliable) ||
                other.reliable == reliable) &&
            (identical(other.unreliable, unreliable) ||
                other.unreliable == unreliable) &&
            (identical(other.dead, dead) || other.dead == dead) &&
            (identical(other.punished, punished) ||
                other.punished == punished) &&
            (identical(other.reason, reason) || other.reason == reason));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(
      runtimeType, span, reliable, unreliable, dead, punished, reason);

  @override
  String toString() {
    return 'StateStats(span: $span, reliable: $reliable, unreliable: $unreliable, dead: $dead, punished: $punished, reason: $reason)';
  }
}

/// @nodoc
abstract mixin class $StateStatsCopyWith<$Res> {
  factory $StateStatsCopyWith(
          StateStats value, $Res Function(StateStats) _then) =
      _$StateStatsCopyWithImpl;
  @useResult
  $Res call(
      {TimestampDuration span,
      TimestampDuration reliable,
      TimestampDuration unreliable,
      TimestampDuration dead,
      TimestampDuration punished,
      StateReasonStats reason});

  $StateReasonStatsCopyWith<$Res> get reason;
}

/// @nodoc
class _$StateStatsCopyWithImpl<$Res> implements $StateStatsCopyWith<$Res> {
  _$StateStatsCopyWithImpl(this._self, this._then);

  final StateStats _self;
  final $Res Function(StateStats) _then;

  /// Create a copy of StateStats
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? span = null,
    Object? reliable = null,
    Object? unreliable = null,
    Object? dead = null,
    Object? punished = null,
    Object? reason = null,
  }) {
    return _then(_self.copyWith(
      span: null == span
          ? _self.span
          : span // ignore: cast_nullable_to_non_nullable
              as TimestampDuration,
      reliable: null == reliable
          ? _self.reliable
          : reliable // ignore: cast_nullable_to_non_nullable
              as TimestampDuration,
      unreliable: null == unreliable
          ? _self.unreliable
          : unreliable // ignore: cast_nullable_to_non_nullable
              as TimestampDuration,
      dead: null == dead
          ? _self.dead
          : dead // ignore: cast_nullable_to_non_nullable
              as TimestampDuration,
      punished: null == punished
          ? _self.punished
          : punished // ignore: cast_nullable_to_non_nullable
              as TimestampDuration,
      reason: null == reason
          ? _self.reason
          : reason // ignore: cast_nullable_to_non_nullable
              as StateReasonStats,
    ));
  }

  /// Create a copy of StateStats
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $StateReasonStatsCopyWith<$Res> get reason {
    return $StateReasonStatsCopyWith<$Res>(_self.reason, (value) {
      return _then(_self.copyWith(reason: value));
    });
  }
}

/// @nodoc
@JsonSerializable()
class _StateStats implements StateStats {
  const _StateStats(
      {required this.span,
      required this.reliable,
      required this.unreliable,
      required this.dead,
      required this.punished,
      required this.reason});
  factory _StateStats.fromJson(Map<String, dynamic> json) =>
      _$StateStatsFromJson(json);

  @override
  final TimestampDuration span;
  @override
  final TimestampDuration reliable;
  @override
  final TimestampDuration unreliable;
  @override
  final TimestampDuration dead;
  @override
  final TimestampDuration punished;
  @override
  final StateReasonStats reason;

  /// Create a copy of StateStats
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  _$StateStatsCopyWith<_StateStats> get copyWith =>
      __$StateStatsCopyWithImpl<_StateStats>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$StateStatsToJson(
      this,
    );
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _StateStats &&
            (identical(other.span, span) || other.span == span) &&
            (identical(other.reliable, reliable) ||
                other.reliable == reliable) &&
            (identical(other.unreliable, unreliable) ||
                other.unreliable == unreliable) &&
            (identical(other.dead, dead) || other.dead == dead) &&
            (identical(other.punished, punished) ||
                other.punished == punished) &&
            (identical(other.reason, reason) || other.reason == reason));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(
      runtimeType, span, reliable, unreliable, dead, punished, reason);

  @override
  String toString() {
    return 'StateStats(span: $span, reliable: $reliable, unreliable: $unreliable, dead: $dead, punished: $punished, reason: $reason)';
  }
}

/// @nodoc
abstract mixin class _$StateStatsCopyWith<$Res>
    implements $StateStatsCopyWith<$Res> {
  factory _$StateStatsCopyWith(
          _StateStats value, $Res Function(_StateStats) _then) =
      __$StateStatsCopyWithImpl;
  @override
  @useResult
  $Res call(
      {TimestampDuration span,
      TimestampDuration reliable,
      TimestampDuration unreliable,
      TimestampDuration dead,
      TimestampDuration punished,
      StateReasonStats reason});

  @override
  $StateReasonStatsCopyWith<$Res> get reason;
}

/// @nodoc
class __$StateStatsCopyWithImpl<$Res> implements _$StateStatsCopyWith<$Res> {
  __$StateStatsCopyWithImpl(this._self, this._then);

  final _StateStats _self;
  final $Res Function(_StateStats) _then;

  /// Create a copy of StateStats
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? span = null,
    Object? reliable = null,
    Object? unreliable = null,
    Object? dead = null,
    Object? punished = null,
    Object? reason = null,
  }) {
    return _then(_StateStats(
      span: null == span
          ? _self.span
          : span // ignore: cast_nullable_to_non_nullable
              as TimestampDuration,
      reliable: null == reliable
          ? _self.reliable
          : reliable // ignore: cast_nullable_to_non_nullable
              as TimestampDuration,
      unreliable: null == unreliable
          ? _self.unreliable
          : unreliable // ignore: cast_nullable_to_non_nullable
              as TimestampDuration,
      dead: null == dead
          ? _self.dead
          : dead // ignore: cast_nullable_to_non_nullable
              as TimestampDuration,
      punished: null == punished
          ? _self.punished
          : punished // ignore: cast_nullable_to_non_nullable
              as TimestampDuration,
      reason: null == reason
          ? _self.reason
          : reason // ignore: cast_nullable_to_non_nullable
              as StateReasonStats,
    ));
  }

  /// Create a copy of StateStats
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $StateReasonStatsCopyWith<$Res> get reason {
    return $StateReasonStatsCopyWith<$Res>(_self.reason, (value) {
      return _then(_self.copyWith(reason: value));
    });
  }
}

/// @nodoc
mixin _$StateReasonStats {
  TimestampDuration get canNotSend;
  TimestampDuration get tooManyLostAnswers;
  TimestampDuration get noPingResponse;
  TimestampDuration get failedToSend;
  TimestampDuration get lostAnswers;
  TimestampDuration get notSeenConsecutively;
  TimestampDuration get inUnreliablePingSpan;

  /// Create a copy of StateReasonStats
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $StateReasonStatsCopyWith<StateReasonStats> get copyWith =>
      _$StateReasonStatsCopyWithImpl<StateReasonStats>(
          this as StateReasonStats, _$identity);

  /// Serializes this StateReasonStats to a JSON map.
  Map<String, dynamic> toJson();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is StateReasonStats &&
            (identical(other.canNotSend, canNotSend) ||
                other.canNotSend == canNotSend) &&
            (identical(other.tooManyLostAnswers, tooManyLostAnswers) ||
                other.tooManyLostAnswers == tooManyLostAnswers) &&
            (identical(other.noPingResponse, noPingResponse) ||
                other.noPingResponse == noPingResponse) &&
            (identical(other.failedToSend, failedToSend) ||
                other.failedToSend == failedToSend) &&
            (identical(other.lostAnswers, lostAnswers) ||
                other.lostAnswers == lostAnswers) &&
            (identical(other.notSeenConsecutively, notSeenConsecutively) ||
                other.notSeenConsecutively == notSeenConsecutively) &&
            (identical(other.inUnreliablePingSpan, inUnreliablePingSpan) ||
                other.inUnreliablePingSpan == inUnreliablePingSpan));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(
      runtimeType,
      canNotSend,
      tooManyLostAnswers,
      noPingResponse,
      failedToSend,
      lostAnswers,
      notSeenConsecutively,
      inUnreliablePingSpan);

  @override
  String toString() {
    return 'StateReasonStats(canNotSend: $canNotSend, tooManyLostAnswers: $tooManyLostAnswers, noPingResponse: $noPingResponse, failedToSend: $failedToSend, lostAnswers: $lostAnswers, notSeenConsecutively: $notSeenConsecutively, inUnreliablePingSpan: $inUnreliablePingSpan)';
  }
}

/// @nodoc
abstract mixin class $StateReasonStatsCopyWith<$Res> {
  factory $StateReasonStatsCopyWith(
          StateReasonStats value, $Res Function(StateReasonStats) _then) =
      _$StateReasonStatsCopyWithImpl;
  @useResult
  $Res call(
      {TimestampDuration canNotSend,
      TimestampDuration tooManyLostAnswers,
      TimestampDuration noPingResponse,
      TimestampDuration failedToSend,
      TimestampDuration lostAnswers,
      TimestampDuration notSeenConsecutively,
      TimestampDuration inUnreliablePingSpan});
}

/// @nodoc
class _$StateReasonStatsCopyWithImpl<$Res>
    implements $StateReasonStatsCopyWith<$Res> {
  _$StateReasonStatsCopyWithImpl(this._self, this._then);

  final StateReasonStats _self;
  final $Res Function(StateReasonStats) _then;

  /// Create a copy of StateReasonStats
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? canNotSend = null,
    Object? tooManyLostAnswers = null,
    Object? noPingResponse = null,
    Object? failedToSend = null,
    Object? lostAnswers = null,
    Object? notSeenConsecutively = null,
    Object? inUnreliablePingSpan = null,
  }) {
    return _then(_self.copyWith(
      canNotSend: null == canNotSend
          ? _self.canNotSend
          : canNotSend // ignore: cast_nullable_to_non_nullable
              as TimestampDuration,
      tooManyLostAnswers: null == tooManyLostAnswers
          ? _self.tooManyLostAnswers
          : tooManyLostAnswers // ignore: cast_nullable_to_non_nullable
              as TimestampDuration,
      noPingResponse: null == noPingResponse
          ? _self.noPingResponse
          : noPingResponse // ignore: cast_nullable_to_non_nullable
              as TimestampDuration,
      failedToSend: null == failedToSend
          ? _self.failedToSend
          : failedToSend // ignore: cast_nullable_to_non_nullable
              as TimestampDuration,
      lostAnswers: null == lostAnswers
          ? _self.lostAnswers
          : lostAnswers // ignore: cast_nullable_to_non_nullable
              as TimestampDuration,
      notSeenConsecutively: null == notSeenConsecutively
          ? _self.notSeenConsecutively
          : notSeenConsecutively // ignore: cast_nullable_to_non_nullable
              as TimestampDuration,
      inUnreliablePingSpan: null == inUnreliablePingSpan
          ? _self.inUnreliablePingSpan
          : inUnreliablePingSpan // ignore: cast_nullable_to_non_nullable
              as TimestampDuration,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _StateReasonStats implements StateReasonStats {
  const _StateReasonStats(
      {required this.canNotSend,
      required this.tooManyLostAnswers,
      required this.noPingResponse,
      required this.failedToSend,
      required this.lostAnswers,
      required this.notSeenConsecutively,
      required this.inUnreliablePingSpan});
  factory _StateReasonStats.fromJson(Map<String, dynamic> json) =>
      _$StateReasonStatsFromJson(json);

  @override
  final TimestampDuration canNotSend;
  @override
  final TimestampDuration tooManyLostAnswers;
  @override
  final TimestampDuration noPingResponse;
  @override
  final TimestampDuration failedToSend;
  @override
  final TimestampDuration lostAnswers;
  @override
  final TimestampDuration notSeenConsecutively;
  @override
  final TimestampDuration inUnreliablePingSpan;

  /// Create a copy of StateReasonStats
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  _$StateReasonStatsCopyWith<_StateReasonStats> get copyWith =>
      __$StateReasonStatsCopyWithImpl<_StateReasonStats>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$StateReasonStatsToJson(
      this,
    );
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _StateReasonStats &&
            (identical(other.canNotSend, canNotSend) ||
                other.canNotSend == canNotSend) &&
            (identical(other.tooManyLostAnswers, tooManyLostAnswers) ||
                other.tooManyLostAnswers == tooManyLostAnswers) &&
            (identical(other.noPingResponse, noPingResponse) ||
                other.noPingResponse == noPingResponse) &&
            (identical(other.failedToSend, failedToSend) ||
                other.failedToSend == failedToSend) &&
            (identical(other.lostAnswers, lostAnswers) ||
                other.lostAnswers == lostAnswers) &&
            (identical(other.notSeenConsecutively, notSeenConsecutively) ||
                other.notSeenConsecutively == notSeenConsecutively) &&
            (identical(other.inUnreliablePingSpan, inUnreliablePingSpan) ||
                other.inUnreliablePingSpan == inUnreliablePingSpan));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(
      runtimeType,
      canNotSend,
      tooManyLostAnswers,
      noPingResponse,
      failedToSend,
      lostAnswers,
      notSeenConsecutively,
      inUnreliablePingSpan);

  @override
  String toString() {
    return 'StateReasonStats(canNotSend: $canNotSend, tooManyLostAnswers: $tooManyLostAnswers, noPingResponse: $noPingResponse, failedToSend: $failedToSend, lostAnswers: $lostAnswers, notSeenConsecutively: $notSeenConsecutively, inUnreliablePingSpan: $inUnreliablePingSpan)';
  }
}

/// @nodoc
abstract mixin class _$StateReasonStatsCopyWith<$Res>
    implements $StateReasonStatsCopyWith<$Res> {
  factory _$StateReasonStatsCopyWith(
          _StateReasonStats value, $Res Function(_StateReasonStats) _then) =
      __$StateReasonStatsCopyWithImpl;
  @override
  @useResult
  $Res call(
      {TimestampDuration canNotSend,
      TimestampDuration tooManyLostAnswers,
      TimestampDuration noPingResponse,
      TimestampDuration failedToSend,
      TimestampDuration lostAnswers,
      TimestampDuration notSeenConsecutively,
      TimestampDuration inUnreliablePingSpan});
}

/// @nodoc
class __$StateReasonStatsCopyWithImpl<$Res>
    implements _$StateReasonStatsCopyWith<$Res> {
  __$StateReasonStatsCopyWithImpl(this._self, this._then);

  final _StateReasonStats _self;
  final $Res Function(_StateReasonStats) _then;

  /// Create a copy of StateReasonStats
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? canNotSend = null,
    Object? tooManyLostAnswers = null,
    Object? noPingResponse = null,
    Object? failedToSend = null,
    Object? lostAnswers = null,
    Object? notSeenConsecutively = null,
    Object? inUnreliablePingSpan = null,
  }) {
    return _then(_StateReasonStats(
      canNotSend: null == canNotSend
          ? _self.canNotSend
          : canNotSend // ignore: cast_nullable_to_non_nullable
              as TimestampDuration,
      tooManyLostAnswers: null == tooManyLostAnswers
          ? _self.tooManyLostAnswers
          : tooManyLostAnswers // ignore: cast_nullable_to_non_nullable
              as TimestampDuration,
      noPingResponse: null == noPingResponse
          ? _self.noPingResponse
          : noPingResponse // ignore: cast_nullable_to_non_nullable
              as TimestampDuration,
      failedToSend: null == failedToSend
          ? _self.failedToSend
          : failedToSend // ignore: cast_nullable_to_non_nullable
              as TimestampDuration,
      lostAnswers: null == lostAnswers
          ? _self.lostAnswers
          : lostAnswers // ignore: cast_nullable_to_non_nullable
              as TimestampDuration,
      notSeenConsecutively: null == notSeenConsecutively
          ? _self.notSeenConsecutively
          : notSeenConsecutively // ignore: cast_nullable_to_non_nullable
              as TimestampDuration,
      inUnreliablePingSpan: null == inUnreliablePingSpan
          ? _self.inUnreliablePingSpan
          : inUnreliablePingSpan // ignore: cast_nullable_to_non_nullable
              as TimestampDuration,
    ));
  }
}

/// @nodoc
mixin _$AnswerStats {
  TimestampDuration get span;
  int get questions;
  int get answers;
  int get lostAnswers;
  int get consecutiveAnswersMaximum;
  int get consecutiveAnswersAverage;
  int get consecutiveAnswersMinimum;
  int get consecutiveLostAnswersMaximum;
  int get consecutiveLostAnswersAverage;
  int get consecutiveLostAnswersMinimum;

  /// Create a copy of AnswerStats
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $AnswerStatsCopyWith<AnswerStats> get copyWith =>
      _$AnswerStatsCopyWithImpl<AnswerStats>(this as AnswerStats, _$identity);

  /// Serializes this AnswerStats to a JSON map.
  Map<String, dynamic> toJson();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is AnswerStats &&
            (identical(other.span, span) || other.span == span) &&
            (identical(other.questions, questions) ||
                other.questions == questions) &&
            (identical(other.answers, answers) || other.answers == answers) &&
            (identical(other.lostAnswers, lostAnswers) ||
                other.lostAnswers == lostAnswers) &&
            (identical(other.consecutiveAnswersMaximum,
                    consecutiveAnswersMaximum) ||
                other.consecutiveAnswersMaximum == consecutiveAnswersMaximum) &&
            (identical(other.consecutiveAnswersAverage,
                    consecutiveAnswersAverage) ||
                other.consecutiveAnswersAverage == consecutiveAnswersAverage) &&
            (identical(other.consecutiveAnswersMinimum,
                    consecutiveAnswersMinimum) ||
                other.consecutiveAnswersMinimum == consecutiveAnswersMinimum) &&
            (identical(other.consecutiveLostAnswersMaximum,
                    consecutiveLostAnswersMaximum) ||
                other.consecutiveLostAnswersMaximum ==
                    consecutiveLostAnswersMaximum) &&
            (identical(other.consecutiveLostAnswersAverage,
                    consecutiveLostAnswersAverage) ||
                other.consecutiveLostAnswersAverage ==
                    consecutiveLostAnswersAverage) &&
            (identical(other.consecutiveLostAnswersMinimum,
                    consecutiveLostAnswersMinimum) ||
                other.consecutiveLostAnswersMinimum ==
                    consecutiveLostAnswersMinimum));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(
      runtimeType,
      span,
      questions,
      answers,
      lostAnswers,
      consecutiveAnswersMaximum,
      consecutiveAnswersAverage,
      consecutiveAnswersMinimum,
      consecutiveLostAnswersMaximum,
      consecutiveLostAnswersAverage,
      consecutiveLostAnswersMinimum);

  @override
  String toString() {
    return 'AnswerStats(span: $span, questions: $questions, answers: $answers, lostAnswers: $lostAnswers, consecutiveAnswersMaximum: $consecutiveAnswersMaximum, consecutiveAnswersAverage: $consecutiveAnswersAverage, consecutiveAnswersMinimum: $consecutiveAnswersMinimum, consecutiveLostAnswersMaximum: $consecutiveLostAnswersMaximum, consecutiveLostAnswersAverage: $consecutiveLostAnswersAverage, consecutiveLostAnswersMinimum: $consecutiveLostAnswersMinimum)';
  }
}

/// @nodoc
abstract mixin class $AnswerStatsCopyWith<$Res> {
  factory $AnswerStatsCopyWith(
          AnswerStats value, $Res Function(AnswerStats) _then) =
      _$AnswerStatsCopyWithImpl;
  @useResult
  $Res call(
      {TimestampDuration span,
      int questions,
      int answers,
      int lostAnswers,
      int consecutiveAnswersMaximum,
      int consecutiveAnswersAverage,
      int consecutiveAnswersMinimum,
      int consecutiveLostAnswersMaximum,
      int consecutiveLostAnswersAverage,
      int consecutiveLostAnswersMinimum});
}

/// @nodoc
class _$AnswerStatsCopyWithImpl<$Res> implements $AnswerStatsCopyWith<$Res> {
  _$AnswerStatsCopyWithImpl(this._self, this._then);

  final AnswerStats _self;
  final $Res Function(AnswerStats) _then;

  /// Create a copy of AnswerStats
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? span = null,
    Object? questions = null,
    Object? answers = null,
    Object? lostAnswers = null,
    Object? consecutiveAnswersMaximum = null,
    Object? consecutiveAnswersAverage = null,
    Object? consecutiveAnswersMinimum = null,
    Object? consecutiveLostAnswersMaximum = null,
    Object? consecutiveLostAnswersAverage = null,
    Object? consecutiveLostAnswersMinimum = null,
  }) {
    return _then(_self.copyWith(
      span: null == span
          ? _self.span
          : span // ignore: cast_nullable_to_non_nullable
              as TimestampDuration,
      questions: null == questions
          ? _self.questions
          : questions // ignore: cast_nullable_to_non_nullable
              as int,
      answers: null == answers
          ? _self.answers
          : answers // ignore: cast_nullable_to_non_nullable
              as int,
      lostAnswers: null == lostAnswers
          ? _self.lostAnswers
          : lostAnswers // ignore: cast_nullable_to_non_nullable
              as int,
      consecutiveAnswersMaximum: null == consecutiveAnswersMaximum
          ? _self.consecutiveAnswersMaximum
          : consecutiveAnswersMaximum // ignore: cast_nullable_to_non_nullable
              as int,
      consecutiveAnswersAverage: null == consecutiveAnswersAverage
          ? _self.consecutiveAnswersAverage
          : consecutiveAnswersAverage // ignore: cast_nullable_to_non_nullable
              as int,
      consecutiveAnswersMinimum: null == consecutiveAnswersMinimum
          ? _self.consecutiveAnswersMinimum
          : consecutiveAnswersMinimum // ignore: cast_nullable_to_non_nullable
              as int,
      consecutiveLostAnswersMaximum: null == consecutiveLostAnswersMaximum
          ? _self.consecutiveLostAnswersMaximum
          : consecutiveLostAnswersMaximum // ignore: cast_nullable_to_non_nullable
              as int,
      consecutiveLostAnswersAverage: null == consecutiveLostAnswersAverage
          ? _self.consecutiveLostAnswersAverage
          : consecutiveLostAnswersAverage // ignore: cast_nullable_to_non_nullable
              as int,
      consecutiveLostAnswersMinimum: null == consecutiveLostAnswersMinimum
          ? _self.consecutiveLostAnswersMinimum
          : consecutiveLostAnswersMinimum // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _AnswerStats implements AnswerStats {
  const _AnswerStats(
      {required this.span,
      required this.questions,
      required this.answers,
      required this.lostAnswers,
      required this.consecutiveAnswersMaximum,
      required this.consecutiveAnswersAverage,
      required this.consecutiveAnswersMinimum,
      required this.consecutiveLostAnswersMaximum,
      required this.consecutiveLostAnswersAverage,
      required this.consecutiveLostAnswersMinimum});
  factory _AnswerStats.fromJson(Map<String, dynamic> json) =>
      _$AnswerStatsFromJson(json);

  @override
  final TimestampDuration span;
  @override
  final int questions;
  @override
  final int answers;
  @override
  final int lostAnswers;
  @override
  final int consecutiveAnswersMaximum;
  @override
  final int consecutiveAnswersAverage;
  @override
  final int consecutiveAnswersMinimum;
  @override
  final int consecutiveLostAnswersMaximum;
  @override
  final int consecutiveLostAnswersAverage;
  @override
  final int consecutiveLostAnswersMinimum;

  /// Create a copy of AnswerStats
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  _$AnswerStatsCopyWith<_AnswerStats> get copyWith =>
      __$AnswerStatsCopyWithImpl<_AnswerStats>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$AnswerStatsToJson(
      this,
    );
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _AnswerStats &&
            (identical(other.span, span) || other.span == span) &&
            (identical(other.questions, questions) ||
                other.questions == questions) &&
            (identical(other.answers, answers) || other.answers == answers) &&
            (identical(other.lostAnswers, lostAnswers) ||
                other.lostAnswers == lostAnswers) &&
            (identical(other.consecutiveAnswersMaximum,
                    consecutiveAnswersMaximum) ||
                other.consecutiveAnswersMaximum == consecutiveAnswersMaximum) &&
            (identical(other.consecutiveAnswersAverage,
                    consecutiveAnswersAverage) ||
                other.consecutiveAnswersAverage == consecutiveAnswersAverage) &&
            (identical(other.consecutiveAnswersMinimum,
                    consecutiveAnswersMinimum) ||
                other.consecutiveAnswersMinimum == consecutiveAnswersMinimum) &&
            (identical(other.consecutiveLostAnswersMaximum,
                    consecutiveLostAnswersMaximum) ||
                other.consecutiveLostAnswersMaximum ==
                    consecutiveLostAnswersMaximum) &&
            (identical(other.consecutiveLostAnswersAverage,
                    consecutiveLostAnswersAverage) ||
                other.consecutiveLostAnswersAverage ==
                    consecutiveLostAnswersAverage) &&
            (identical(other.consecutiveLostAnswersMinimum,
                    consecutiveLostAnswersMinimum) ||
                other.consecutiveLostAnswersMinimum ==
                    consecutiveLostAnswersMinimum));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(
      runtimeType,
      span,
      questions,
      answers,
      lostAnswers,
      consecutiveAnswersMaximum,
      consecutiveAnswersAverage,
      consecutiveAnswersMinimum,
      consecutiveLostAnswersMaximum,
      consecutiveLostAnswersAverage,
      consecutiveLostAnswersMinimum);

  @override
  String toString() {
    return 'AnswerStats(span: $span, questions: $questions, answers: $answers, lostAnswers: $lostAnswers, consecutiveAnswersMaximum: $consecutiveAnswersMaximum, consecutiveAnswersAverage: $consecutiveAnswersAverage, consecutiveAnswersMinimum: $consecutiveAnswersMinimum, consecutiveLostAnswersMaximum: $consecutiveLostAnswersMaximum, consecutiveLostAnswersAverage: $consecutiveLostAnswersAverage, consecutiveLostAnswersMinimum: $consecutiveLostAnswersMinimum)';
  }
}

/// @nodoc
abstract mixin class _$AnswerStatsCopyWith<$Res>
    implements $AnswerStatsCopyWith<$Res> {
  factory _$AnswerStatsCopyWith(
          _AnswerStats value, $Res Function(_AnswerStats) _then) =
      __$AnswerStatsCopyWithImpl;
  @override
  @useResult
  $Res call(
      {TimestampDuration span,
      int questions,
      int answers,
      int lostAnswers,
      int consecutiveAnswersMaximum,
      int consecutiveAnswersAverage,
      int consecutiveAnswersMinimum,
      int consecutiveLostAnswersMaximum,
      int consecutiveLostAnswersAverage,
      int consecutiveLostAnswersMinimum});
}

/// @nodoc
class __$AnswerStatsCopyWithImpl<$Res> implements _$AnswerStatsCopyWith<$Res> {
  __$AnswerStatsCopyWithImpl(this._self, this._then);

  final _AnswerStats _self;
  final $Res Function(_AnswerStats) _then;

  /// Create a copy of AnswerStats
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? span = null,
    Object? questions = null,
    Object? answers = null,
    Object? lostAnswers = null,
    Object? consecutiveAnswersMaximum = null,
    Object? consecutiveAnswersAverage = null,
    Object? consecutiveAnswersMinimum = null,
    Object? consecutiveLostAnswersMaximum = null,
    Object? consecutiveLostAnswersAverage = null,
    Object? consecutiveLostAnswersMinimum = null,
  }) {
    return _then(_AnswerStats(
      span: null == span
          ? _self.span
          : span // ignore: cast_nullable_to_non_nullable
              as TimestampDuration,
      questions: null == questions
          ? _self.questions
          : questions // ignore: cast_nullable_to_non_nullable
              as int,
      answers: null == answers
          ? _self.answers
          : answers // ignore: cast_nullable_to_non_nullable
              as int,
      lostAnswers: null == lostAnswers
          ? _self.lostAnswers
          : lostAnswers // ignore: cast_nullable_to_non_nullable
              as int,
      consecutiveAnswersMaximum: null == consecutiveAnswersMaximum
          ? _self.consecutiveAnswersMaximum
          : consecutiveAnswersMaximum // ignore: cast_nullable_to_non_nullable
              as int,
      consecutiveAnswersAverage: null == consecutiveAnswersAverage
          ? _self.consecutiveAnswersAverage
          : consecutiveAnswersAverage // ignore: cast_nullable_to_non_nullable
              as int,
      consecutiveAnswersMinimum: null == consecutiveAnswersMinimum
          ? _self.consecutiveAnswersMinimum
          : consecutiveAnswersMinimum // ignore: cast_nullable_to_non_nullable
              as int,
      consecutiveLostAnswersMaximum: null == consecutiveLostAnswersMaximum
          ? _self.consecutiveLostAnswersMaximum
          : consecutiveLostAnswersMaximum // ignore: cast_nullable_to_non_nullable
              as int,
      consecutiveLostAnswersAverage: null == consecutiveLostAnswersAverage
          ? _self.consecutiveLostAnswersAverage
          : consecutiveLostAnswersAverage // ignore: cast_nullable_to_non_nullable
              as int,
      consecutiveLostAnswersMinimum: null == consecutiveLostAnswersMinimum
          ? _self.consecutiveLostAnswersMinimum
          : consecutiveLostAnswersMinimum // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc
mixin _$RPCStats {
  int get messagesSent;
  int get messagesRcvd;
  int get questionsInFlight;
  Timestamp? get lastQuestionTs;
  Timestamp? get lastSeenTs;
  Timestamp? get firstConsecutiveSeenTs;
  int get recentLostAnswersUnordered;
  int get recentLostAnswersOrdered;
  int get failedToSend;
  AnswerStats get answerUnordered;
  AnswerStats get answerOrdered;

  /// Create a copy of RPCStats
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $RPCStatsCopyWith<RPCStats> get copyWith =>
      _$RPCStatsCopyWithImpl<RPCStats>(this as RPCStats, _$identity);

  /// Serializes this RPCStats to a JSON map.
  Map<String, dynamic> toJson();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is RPCStats &&
            (identical(other.messagesSent, messagesSent) ||
                other.messagesSent == messagesSent) &&
            (identical(other.messagesRcvd, messagesRcvd) ||
                other.messagesRcvd == messagesRcvd) &&
            (identical(other.questionsInFlight, questionsInFlight) ||
                other.questionsInFlight == questionsInFlight) &&
            (identical(other.lastQuestionTs, lastQuestionTs) ||
                other.lastQuestionTs == lastQuestionTs) &&
            (identical(other.lastSeenTs, lastSeenTs) ||
                other.lastSeenTs == lastSeenTs) &&
            (identical(other.firstConsecutiveSeenTs, firstConsecutiveSeenTs) ||
                other.firstConsecutiveSeenTs == firstConsecutiveSeenTs) &&
            (identical(other.recentLostAnswersUnordered,
                    recentLostAnswersUnordered) ||
                other.recentLostAnswersUnordered ==
                    recentLostAnswersUnordered) &&
            (identical(
                    other.recentLostAnswersOrdered, recentLostAnswersOrdered) ||
                other.recentLostAnswersOrdered == recentLostAnswersOrdered) &&
            (identical(other.failedToSend, failedToSend) ||
                other.failedToSend == failedToSend) &&
            (identical(other.answerUnordered, answerUnordered) ||
                other.answerUnordered == answerUnordered) &&
            (identical(other.answerOrdered, answerOrdered) ||
                other.answerOrdered == answerOrdered));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(
      runtimeType,
      messagesSent,
      messagesRcvd,
      questionsInFlight,
      lastQuestionTs,
      lastSeenTs,
      firstConsecutiveSeenTs,
      recentLostAnswersUnordered,
      recentLostAnswersOrdered,
      failedToSend,
      answerUnordered,
      answerOrdered);

  @override
  String toString() {
    return 'RPCStats(messagesSent: $messagesSent, messagesRcvd: $messagesRcvd, questionsInFlight: $questionsInFlight, lastQuestionTs: $lastQuestionTs, lastSeenTs: $lastSeenTs, firstConsecutiveSeenTs: $firstConsecutiveSeenTs, recentLostAnswersUnordered: $recentLostAnswersUnordered, recentLostAnswersOrdered: $recentLostAnswersOrdered, failedToSend: $failedToSend, answerUnordered: $answerUnordered, answerOrdered: $answerOrdered)';
  }
}

/// @nodoc
abstract mixin class $RPCStatsCopyWith<$Res> {
  factory $RPCStatsCopyWith(RPCStats value, $Res Function(RPCStats) _then) =
      _$RPCStatsCopyWithImpl;
  @useResult
  $Res call(
      {int messagesSent,
      int messagesRcvd,
      int questionsInFlight,
      Timestamp? lastQuestionTs,
      Timestamp? lastSeenTs,
      Timestamp? firstConsecutiveSeenTs,
      int recentLostAnswersUnordered,
      int recentLostAnswersOrdered,
      int failedToSend,
      AnswerStats answerUnordered,
      AnswerStats answerOrdered});

  $AnswerStatsCopyWith<$Res> get answerUnordered;
  $AnswerStatsCopyWith<$Res> get answerOrdered;
}

/// @nodoc
class _$RPCStatsCopyWithImpl<$Res> implements $RPCStatsCopyWith<$Res> {
  _$RPCStatsCopyWithImpl(this._self, this._then);

  final RPCStats _self;
  final $Res Function(RPCStats) _then;

  /// Create a copy of RPCStats
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? messagesSent = null,
    Object? messagesRcvd = null,
    Object? questionsInFlight = null,
    Object? lastQuestionTs = freezed,
    Object? lastSeenTs = freezed,
    Object? firstConsecutiveSeenTs = freezed,
    Object? recentLostAnswersUnordered = null,
    Object? recentLostAnswersOrdered = null,
    Object? failedToSend = null,
    Object? answerUnordered = null,
    Object? answerOrdered = null,
  }) {
    return _then(_self.copyWith(
      messagesSent: null == messagesSent
          ? _self.messagesSent
          : messagesSent // ignore: cast_nullable_to_non_nullable
              as int,
      messagesRcvd: null == messagesRcvd
          ? _self.messagesRcvd
          : messagesRcvd // ignore: cast_nullable_to_non_nullable
              as int,
      questionsInFlight: null == questionsInFlight
          ? _self.questionsInFlight
          : questionsInFlight // ignore: cast_nullable_to_non_nullable
              as int,
      lastQuestionTs: freezed == lastQuestionTs
          ? _self.lastQuestionTs
          : lastQuestionTs // ignore: cast_nullable_to_non_nullable
              as Timestamp?,
      lastSeenTs: freezed == lastSeenTs
          ? _self.lastSeenTs
          : lastSeenTs // ignore: cast_nullable_to_non_nullable
              as Timestamp?,
      firstConsecutiveSeenTs: freezed == firstConsecutiveSeenTs
          ? _self.firstConsecutiveSeenTs
          : firstConsecutiveSeenTs // ignore: cast_nullable_to_non_nullable
              as Timestamp?,
      recentLostAnswersUnordered: null == recentLostAnswersUnordered
          ? _self.recentLostAnswersUnordered
          : recentLostAnswersUnordered // ignore: cast_nullable_to_non_nullable
              as int,
      recentLostAnswersOrdered: null == recentLostAnswersOrdered
          ? _self.recentLostAnswersOrdered
          : recentLostAnswersOrdered // ignore: cast_nullable_to_non_nullable
              as int,
      failedToSend: null == failedToSend
          ? _self.failedToSend
          : failedToSend // ignore: cast_nullable_to_non_nullable
              as int,
      answerUnordered: null == answerUnordered
          ? _self.answerUnordered
          : answerUnordered // ignore: cast_nullable_to_non_nullable
              as AnswerStats,
      answerOrdered: null == answerOrdered
          ? _self.answerOrdered
          : answerOrdered // ignore: cast_nullable_to_non_nullable
              as AnswerStats,
    ));
  }

  /// Create a copy of RPCStats
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $AnswerStatsCopyWith<$Res> get answerUnordered {
    return $AnswerStatsCopyWith<$Res>(_self.answerUnordered, (value) {
      return _then(_self.copyWith(answerUnordered: value));
    });
  }

  /// Create a copy of RPCStats
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $AnswerStatsCopyWith<$Res> get answerOrdered {
    return $AnswerStatsCopyWith<$Res>(_self.answerOrdered, (value) {
      return _then(_self.copyWith(answerOrdered: value));
    });
  }
}

/// @nodoc
@JsonSerializable()
class _RPCStats implements RPCStats {
  const _RPCStats(
      {required this.messagesSent,
      required this.messagesRcvd,
      required this.questionsInFlight,
      required this.lastQuestionTs,
      required this.lastSeenTs,
      required this.firstConsecutiveSeenTs,
      required this.recentLostAnswersUnordered,
      required this.recentLostAnswersOrdered,
      required this.failedToSend,
      required this.answerUnordered,
      required this.answerOrdered});
  factory _RPCStats.fromJson(Map<String, dynamic> json) =>
      _$RPCStatsFromJson(json);

  @override
  final int messagesSent;
  @override
  final int messagesRcvd;
  @override
  final int questionsInFlight;
  @override
  final Timestamp? lastQuestionTs;
  @override
  final Timestamp? lastSeenTs;
  @override
  final Timestamp? firstConsecutiveSeenTs;
  @override
  final int recentLostAnswersUnordered;
  @override
  final int recentLostAnswersOrdered;
  @override
  final int failedToSend;
  @override
  final AnswerStats answerUnordered;
  @override
  final AnswerStats answerOrdered;

  /// Create a copy of RPCStats
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  _$RPCStatsCopyWith<_RPCStats> get copyWith =>
      __$RPCStatsCopyWithImpl<_RPCStats>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$RPCStatsToJson(
      this,
    );
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _RPCStats &&
            (identical(other.messagesSent, messagesSent) ||
                other.messagesSent == messagesSent) &&
            (identical(other.messagesRcvd, messagesRcvd) ||
                other.messagesRcvd == messagesRcvd) &&
            (identical(other.questionsInFlight, questionsInFlight) ||
                other.questionsInFlight == questionsInFlight) &&
            (identical(other.lastQuestionTs, lastQuestionTs) ||
                other.lastQuestionTs == lastQuestionTs) &&
            (identical(other.lastSeenTs, lastSeenTs) ||
                other.lastSeenTs == lastSeenTs) &&
            (identical(other.firstConsecutiveSeenTs, firstConsecutiveSeenTs) ||
                other.firstConsecutiveSeenTs == firstConsecutiveSeenTs) &&
            (identical(other.recentLostAnswersUnordered,
                    recentLostAnswersUnordered) ||
                other.recentLostAnswersUnordered ==
                    recentLostAnswersUnordered) &&
            (identical(
                    other.recentLostAnswersOrdered, recentLostAnswersOrdered) ||
                other.recentLostAnswersOrdered == recentLostAnswersOrdered) &&
            (identical(other.failedToSend, failedToSend) ||
                other.failedToSend == failedToSend) &&
            (identical(other.answerUnordered, answerUnordered) ||
                other.answerUnordered == answerUnordered) &&
            (identical(other.answerOrdered, answerOrdered) ||
                other.answerOrdered == answerOrdered));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(
      runtimeType,
      messagesSent,
      messagesRcvd,
      questionsInFlight,
      lastQuestionTs,
      lastSeenTs,
      firstConsecutiveSeenTs,
      recentLostAnswersUnordered,
      recentLostAnswersOrdered,
      failedToSend,
      answerUnordered,
      answerOrdered);

  @override
  String toString() {
    return 'RPCStats(messagesSent: $messagesSent, messagesRcvd: $messagesRcvd, questionsInFlight: $questionsInFlight, lastQuestionTs: $lastQuestionTs, lastSeenTs: $lastSeenTs, firstConsecutiveSeenTs: $firstConsecutiveSeenTs, recentLostAnswersUnordered: $recentLostAnswersUnordered, recentLostAnswersOrdered: $recentLostAnswersOrdered, failedToSend: $failedToSend, answerUnordered: $answerUnordered, answerOrdered: $answerOrdered)';
  }
}

/// @nodoc
abstract mixin class _$RPCStatsCopyWith<$Res>
    implements $RPCStatsCopyWith<$Res> {
  factory _$RPCStatsCopyWith(_RPCStats value, $Res Function(_RPCStats) _then) =
      __$RPCStatsCopyWithImpl;
  @override
  @useResult
  $Res call(
      {int messagesSent,
      int messagesRcvd,
      int questionsInFlight,
      Timestamp? lastQuestionTs,
      Timestamp? lastSeenTs,
      Timestamp? firstConsecutiveSeenTs,
      int recentLostAnswersUnordered,
      int recentLostAnswersOrdered,
      int failedToSend,
      AnswerStats answerUnordered,
      AnswerStats answerOrdered});

  @override
  $AnswerStatsCopyWith<$Res> get answerUnordered;
  @override
  $AnswerStatsCopyWith<$Res> get answerOrdered;
}

/// @nodoc
class __$RPCStatsCopyWithImpl<$Res> implements _$RPCStatsCopyWith<$Res> {
  __$RPCStatsCopyWithImpl(this._self, this._then);

  final _RPCStats _self;
  final $Res Function(_RPCStats) _then;

  /// Create a copy of RPCStats
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? messagesSent = null,
    Object? messagesRcvd = null,
    Object? questionsInFlight = null,
    Object? lastQuestionTs = freezed,
    Object? lastSeenTs = freezed,
    Object? firstConsecutiveSeenTs = freezed,
    Object? recentLostAnswersUnordered = null,
    Object? recentLostAnswersOrdered = null,
    Object? failedToSend = null,
    Object? answerUnordered = null,
    Object? answerOrdered = null,
  }) {
    return _then(_RPCStats(
      messagesSent: null == messagesSent
          ? _self.messagesSent
          : messagesSent // ignore: cast_nullable_to_non_nullable
              as int,
      messagesRcvd: null == messagesRcvd
          ? _self.messagesRcvd
          : messagesRcvd // ignore: cast_nullable_to_non_nullable
              as int,
      questionsInFlight: null == questionsInFlight
          ? _self.questionsInFlight
          : questionsInFlight // ignore: cast_nullable_to_non_nullable
              as int,
      lastQuestionTs: freezed == lastQuestionTs
          ? _self.lastQuestionTs
          : lastQuestionTs // ignore: cast_nullable_to_non_nullable
              as Timestamp?,
      lastSeenTs: freezed == lastSeenTs
          ? _self.lastSeenTs
          : lastSeenTs // ignore: cast_nullable_to_non_nullable
              as Timestamp?,
      firstConsecutiveSeenTs: freezed == firstConsecutiveSeenTs
          ? _self.firstConsecutiveSeenTs
          : firstConsecutiveSeenTs // ignore: cast_nullable_to_non_nullable
              as Timestamp?,
      recentLostAnswersUnordered: null == recentLostAnswersUnordered
          ? _self.recentLostAnswersUnordered
          : recentLostAnswersUnordered // ignore: cast_nullable_to_non_nullable
              as int,
      recentLostAnswersOrdered: null == recentLostAnswersOrdered
          ? _self.recentLostAnswersOrdered
          : recentLostAnswersOrdered // ignore: cast_nullable_to_non_nullable
              as int,
      failedToSend: null == failedToSend
          ? _self.failedToSend
          : failedToSend // ignore: cast_nullable_to_non_nullable
              as int,
      answerUnordered: null == answerUnordered
          ? _self.answerUnordered
          : answerUnordered // ignore: cast_nullable_to_non_nullable
              as AnswerStats,
      answerOrdered: null == answerOrdered
          ? _self.answerOrdered
          : answerOrdered // ignore: cast_nullable_to_non_nullable
              as AnswerStats,
    ));
  }

  /// Create a copy of RPCStats
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $AnswerStatsCopyWith<$Res> get answerUnordered {
    return $AnswerStatsCopyWith<$Res>(_self.answerUnordered, (value) {
      return _then(_self.copyWith(answerUnordered: value));
    });
  }

  /// Create a copy of RPCStats
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $AnswerStatsCopyWith<$Res> get answerOrdered {
    return $AnswerStatsCopyWith<$Res>(_self.answerOrdered, (value) {
      return _then(_self.copyWith(answerOrdered: value));
    });
  }
}

/// @nodoc
mixin _$PeerStats {
  Timestamp get timeAdded;
  RPCStats get rpcStats;
  TransferStatsDownUp get transfer;
  StateStats get state;
  LatencyStats? get latency;

  /// Create a copy of PeerStats
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $PeerStatsCopyWith<PeerStats> get copyWith =>
      _$PeerStatsCopyWithImpl<PeerStats>(this as PeerStats, _$identity);

  /// Serializes this PeerStats to a JSON map.
  Map<String, dynamic> toJson();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is PeerStats &&
            (identical(other.timeAdded, timeAdded) ||
                other.timeAdded == timeAdded) &&
            (identical(other.rpcStats, rpcStats) ||
                other.rpcStats == rpcStats) &&
            (identical(other.transfer, transfer) ||
                other.transfer == transfer) &&
            (identical(other.state, state) || other.state == state) &&
            (identical(other.latency, latency) || other.latency == latency));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode =>
      Object.hash(runtimeType, timeAdded, rpcStats, transfer, state, latency);

  @override
  String toString() {
    return 'PeerStats(timeAdded: $timeAdded, rpcStats: $rpcStats, transfer: $transfer, state: $state, latency: $latency)';
  }
}

/// @nodoc
abstract mixin class $PeerStatsCopyWith<$Res> {
  factory $PeerStatsCopyWith(PeerStats value, $Res Function(PeerStats) _then) =
      _$PeerStatsCopyWithImpl;
  @useResult
  $Res call(
      {Timestamp timeAdded,
      RPCStats rpcStats,
      TransferStatsDownUp transfer,
      StateStats state,
      LatencyStats? latency});

  $RPCStatsCopyWith<$Res> get rpcStats;
  $TransferStatsDownUpCopyWith<$Res> get transfer;
  $StateStatsCopyWith<$Res> get state;
  $LatencyStatsCopyWith<$Res>? get latency;
}

/// @nodoc
class _$PeerStatsCopyWithImpl<$Res> implements $PeerStatsCopyWith<$Res> {
  _$PeerStatsCopyWithImpl(this._self, this._then);

  final PeerStats _self;
  final $Res Function(PeerStats) _then;

  /// Create a copy of PeerStats
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? timeAdded = null,
    Object? rpcStats = null,
    Object? transfer = null,
    Object? state = null,
    Object? latency = freezed,
  }) {
    return _then(_self.copyWith(
      timeAdded: null == timeAdded
          ? _self.timeAdded
          : timeAdded // ignore: cast_nullable_to_non_nullable
              as Timestamp,
      rpcStats: null == rpcStats
          ? _self.rpcStats
          : rpcStats // ignore: cast_nullable_to_non_nullable
              as RPCStats,
      transfer: null == transfer
          ? _self.transfer
          : transfer // ignore: cast_nullable_to_non_nullable
              as TransferStatsDownUp,
      state: null == state
          ? _self.state
          : state // ignore: cast_nullable_to_non_nullable
              as StateStats,
      latency: freezed == latency
          ? _self.latency
          : latency // ignore: cast_nullable_to_non_nullable
              as LatencyStats?,
    ));
  }

  /// Create a copy of PeerStats
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $RPCStatsCopyWith<$Res> get rpcStats {
    return $RPCStatsCopyWith<$Res>(_self.rpcStats, (value) {
      return _then(_self.copyWith(rpcStats: value));
    });
  }

  /// Create a copy of PeerStats
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $TransferStatsDownUpCopyWith<$Res> get transfer {
    return $TransferStatsDownUpCopyWith<$Res>(_self.transfer, (value) {
      return _then(_self.copyWith(transfer: value));
    });
  }

  /// Create a copy of PeerStats
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $StateStatsCopyWith<$Res> get state {
    return $StateStatsCopyWith<$Res>(_self.state, (value) {
      return _then(_self.copyWith(state: value));
    });
  }

  /// Create a copy of PeerStats
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $LatencyStatsCopyWith<$Res>? get latency {
    if (_self.latency == null) {
      return null;
    }

    return $LatencyStatsCopyWith<$Res>(_self.latency!, (value) {
      return _then(_self.copyWith(latency: value));
    });
  }
}

/// @nodoc
@JsonSerializable()
class _PeerStats implements PeerStats {
  const _PeerStats(
      {required this.timeAdded,
      required this.rpcStats,
      required this.transfer,
      required this.state,
      this.latency});
  factory _PeerStats.fromJson(Map<String, dynamic> json) =>
      _$PeerStatsFromJson(json);

  @override
  final Timestamp timeAdded;
  @override
  final RPCStats rpcStats;
  @override
  final TransferStatsDownUp transfer;
  @override
  final StateStats state;
  @override
  final LatencyStats? latency;

  /// Create a copy of PeerStats
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  _$PeerStatsCopyWith<_PeerStats> get copyWith =>
      __$PeerStatsCopyWithImpl<_PeerStats>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$PeerStatsToJson(
      this,
    );
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _PeerStats &&
            (identical(other.timeAdded, timeAdded) ||
                other.timeAdded == timeAdded) &&
            (identical(other.rpcStats, rpcStats) ||
                other.rpcStats == rpcStats) &&
            (identical(other.transfer, transfer) ||
                other.transfer == transfer) &&
            (identical(other.state, state) || other.state == state) &&
            (identical(other.latency, latency) || other.latency == latency));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode =>
      Object.hash(runtimeType, timeAdded, rpcStats, transfer, state, latency);

  @override
  String toString() {
    return 'PeerStats(timeAdded: $timeAdded, rpcStats: $rpcStats, transfer: $transfer, state: $state, latency: $latency)';
  }
}

/// @nodoc
abstract mixin class _$PeerStatsCopyWith<$Res>
    implements $PeerStatsCopyWith<$Res> {
  factory _$PeerStatsCopyWith(
          _PeerStats value, $Res Function(_PeerStats) _then) =
      __$PeerStatsCopyWithImpl;
  @override
  @useResult
  $Res call(
      {Timestamp timeAdded,
      RPCStats rpcStats,
      TransferStatsDownUp transfer,
      StateStats state,
      LatencyStats? latency});

  @override
  $RPCStatsCopyWith<$Res> get rpcStats;
  @override
  $TransferStatsDownUpCopyWith<$Res> get transfer;
  @override
  $StateStatsCopyWith<$Res> get state;
  @override
  $LatencyStatsCopyWith<$Res>? get latency;
}

/// @nodoc
class __$PeerStatsCopyWithImpl<$Res> implements _$PeerStatsCopyWith<$Res> {
  __$PeerStatsCopyWithImpl(this._self, this._then);

  final _PeerStats _self;
  final $Res Function(_PeerStats) _then;

  /// Create a copy of PeerStats
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? timeAdded = null,
    Object? rpcStats = null,
    Object? transfer = null,
    Object? state = null,
    Object? latency = freezed,
  }) {
    return _then(_PeerStats(
      timeAdded: null == timeAdded
          ? _self.timeAdded
          : timeAdded // ignore: cast_nullable_to_non_nullable
              as Timestamp,
      rpcStats: null == rpcStats
          ? _self.rpcStats
          : rpcStats // ignore: cast_nullable_to_non_nullable
              as RPCStats,
      transfer: null == transfer
          ? _self.transfer
          : transfer // ignore: cast_nullable_to_non_nullable
              as TransferStatsDownUp,
      state: null == state
          ? _self.state
          : state // ignore: cast_nullable_to_non_nullable
              as StateStats,
      latency: freezed == latency
          ? _self.latency
          : latency // ignore: cast_nullable_to_non_nullable
              as LatencyStats?,
    ));
  }

  /// Create a copy of PeerStats
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $RPCStatsCopyWith<$Res> get rpcStats {
    return $RPCStatsCopyWith<$Res>(_self.rpcStats, (value) {
      return _then(_self.copyWith(rpcStats: value));
    });
  }

  /// Create a copy of PeerStats
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $TransferStatsDownUpCopyWith<$Res> get transfer {
    return $TransferStatsDownUpCopyWith<$Res>(_self.transfer, (value) {
      return _then(_self.copyWith(transfer: value));
    });
  }

  /// Create a copy of PeerStats
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $StateStatsCopyWith<$Res> get state {
    return $StateStatsCopyWith<$Res>(_self.state, (value) {
      return _then(_self.copyWith(state: value));
    });
  }

  /// Create a copy of PeerStats
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $LatencyStatsCopyWith<$Res>? get latency {
    if (_self.latency == null) {
      return null;
    }

    return $LatencyStatsCopyWith<$Res>(_self.latency!, (value) {
      return _then(_self.copyWith(latency: value));
    });
  }
}

/// @nodoc
mixin _$PeerTableData {
  List<TypedKey> get nodeIds;
  String get peerAddress;
  PeerStats get peerStats;

  /// Create a copy of PeerTableData
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $PeerTableDataCopyWith<PeerTableData> get copyWith =>
      _$PeerTableDataCopyWithImpl<PeerTableData>(
          this as PeerTableData, _$identity);

  /// Serializes this PeerTableData to a JSON map.
  Map<String, dynamic> toJson();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is PeerTableData &&
            const DeepCollectionEquality().equals(other.nodeIds, nodeIds) &&
            (identical(other.peerAddress, peerAddress) ||
                other.peerAddress == peerAddress) &&
            (identical(other.peerStats, peerStats) ||
                other.peerStats == peerStats));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType,
      const DeepCollectionEquality().hash(nodeIds), peerAddress, peerStats);

  @override
  String toString() {
    return 'PeerTableData(nodeIds: $nodeIds, peerAddress: $peerAddress, peerStats: $peerStats)';
  }
}

/// @nodoc
abstract mixin class $PeerTableDataCopyWith<$Res> {
  factory $PeerTableDataCopyWith(
          PeerTableData value, $Res Function(PeerTableData) _then) =
      _$PeerTableDataCopyWithImpl;
  @useResult
  $Res call({List<TypedKey> nodeIds, String peerAddress, PeerStats peerStats});

  $PeerStatsCopyWith<$Res> get peerStats;
}

/// @nodoc
class _$PeerTableDataCopyWithImpl<$Res>
    implements $PeerTableDataCopyWith<$Res> {
  _$PeerTableDataCopyWithImpl(this._self, this._then);

  final PeerTableData _self;
  final $Res Function(PeerTableData) _then;

  /// Create a copy of PeerTableData
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? nodeIds = null,
    Object? peerAddress = null,
    Object? peerStats = null,
  }) {
    return _then(_self.copyWith(
      nodeIds: null == nodeIds
          ? _self.nodeIds
          : nodeIds // ignore: cast_nullable_to_non_nullable
              as List<TypedKey>,
      peerAddress: null == peerAddress
          ? _self.peerAddress
          : peerAddress // ignore: cast_nullable_to_non_nullable
              as String,
      peerStats: null == peerStats
          ? _self.peerStats
          : peerStats // ignore: cast_nullable_to_non_nullable
              as PeerStats,
    ));
  }

  /// Create a copy of PeerTableData
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $PeerStatsCopyWith<$Res> get peerStats {
    return $PeerStatsCopyWith<$Res>(_self.peerStats, (value) {
      return _then(_self.copyWith(peerStats: value));
    });
  }
}

/// @nodoc
@JsonSerializable()
class _PeerTableData implements PeerTableData {
  const _PeerTableData(
      {required final List<TypedKey> nodeIds,
      required this.peerAddress,
      required this.peerStats})
      : _nodeIds = nodeIds;
  factory _PeerTableData.fromJson(Map<String, dynamic> json) =>
      _$PeerTableDataFromJson(json);

  final List<TypedKey> _nodeIds;
  @override
  List<TypedKey> get nodeIds {
    if (_nodeIds is EqualUnmodifiableListView) return _nodeIds;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_nodeIds);
  }

  @override
  final String peerAddress;
  @override
  final PeerStats peerStats;

  /// Create a copy of PeerTableData
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  _$PeerTableDataCopyWith<_PeerTableData> get copyWith =>
      __$PeerTableDataCopyWithImpl<_PeerTableData>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$PeerTableDataToJson(
      this,
    );
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _PeerTableData &&
            const DeepCollectionEquality().equals(other._nodeIds, _nodeIds) &&
            (identical(other.peerAddress, peerAddress) ||
                other.peerAddress == peerAddress) &&
            (identical(other.peerStats, peerStats) ||
                other.peerStats == peerStats));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType,
      const DeepCollectionEquality().hash(_nodeIds), peerAddress, peerStats);

  @override
  String toString() {
    return 'PeerTableData(nodeIds: $nodeIds, peerAddress: $peerAddress, peerStats: $peerStats)';
  }
}

/// @nodoc
abstract mixin class _$PeerTableDataCopyWith<$Res>
    implements $PeerTableDataCopyWith<$Res> {
  factory _$PeerTableDataCopyWith(
          _PeerTableData value, $Res Function(_PeerTableData) _then) =
      __$PeerTableDataCopyWithImpl;
  @override
  @useResult
  $Res call({List<TypedKey> nodeIds, String peerAddress, PeerStats peerStats});

  @override
  $PeerStatsCopyWith<$Res> get peerStats;
}

/// @nodoc
class __$PeerTableDataCopyWithImpl<$Res>
    implements _$PeerTableDataCopyWith<$Res> {
  __$PeerTableDataCopyWithImpl(this._self, this._then);

  final _PeerTableData _self;
  final $Res Function(_PeerTableData) _then;

  /// Create a copy of PeerTableData
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? nodeIds = null,
    Object? peerAddress = null,
    Object? peerStats = null,
  }) {
    return _then(_PeerTableData(
      nodeIds: null == nodeIds
          ? _self._nodeIds
          : nodeIds // ignore: cast_nullable_to_non_nullable
              as List<TypedKey>,
      peerAddress: null == peerAddress
          ? _self.peerAddress
          : peerAddress // ignore: cast_nullable_to_non_nullable
              as String,
      peerStats: null == peerStats
          ? _self.peerStats
          : peerStats // ignore: cast_nullable_to_non_nullable
              as PeerStats,
    ));
  }

  /// Create a copy of PeerTableData
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $PeerStatsCopyWith<$Res> get peerStats {
    return $PeerStatsCopyWith<$Res>(_self.peerStats, (value) {
      return _then(_self.copyWith(peerStats: value));
    });
  }
}

VeilidUpdate _$VeilidUpdateFromJson(Map<String, dynamic> json) {
  switch (json['kind']) {
    case 'Log':
      return VeilidLog.fromJson(json);
    case 'AppMessage':
      return VeilidAppMessage.fromJson(json);
    case 'AppCall':
      return VeilidAppCall.fromJson(json);
    case 'Attachment':
      return VeilidUpdateAttachment.fromJson(json);
    case 'Network':
      return VeilidUpdateNetwork.fromJson(json);
    case 'Config':
      return VeilidUpdateConfig.fromJson(json);
    case 'RouteChange':
      return VeilidUpdateRouteChange.fromJson(json);
    case 'ValueChange':
      return VeilidUpdateValueChange.fromJson(json);

    default:
      throw CheckedFromJsonException(json, 'kind', 'VeilidUpdate',
          'Invalid union type "${json['kind']}"!');
  }
}

/// @nodoc
mixin _$VeilidUpdate {
  /// Serializes this VeilidUpdate to a JSON map.
  Map<String, dynamic> toJson();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is VeilidUpdate);
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'VeilidUpdate()';
  }
}

/// @nodoc
class $VeilidUpdateCopyWith<$Res> {
  $VeilidUpdateCopyWith(VeilidUpdate _, $Res Function(VeilidUpdate) __);
}

/// @nodoc
@JsonSerializable()
class VeilidLog implements VeilidUpdate {
  const VeilidLog(
      {required this.logLevel,
      required this.message,
      this.backtrace,
      final String? $type})
      : $type = $type ?? 'Log';
  factory VeilidLog.fromJson(Map<String, dynamic> json) =>
      _$VeilidLogFromJson(json);

  final VeilidLogLevel logLevel;
  final String message;
  final String? backtrace;

  @JsonKey(name: 'kind')
  final String $type;

  /// Create a copy of VeilidUpdate
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $VeilidLogCopyWith<VeilidLog> get copyWith =>
      _$VeilidLogCopyWithImpl<VeilidLog>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$VeilidLogToJson(
      this,
    );
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is VeilidLog &&
            (identical(other.logLevel, logLevel) ||
                other.logLevel == logLevel) &&
            (identical(other.message, message) || other.message == message) &&
            (identical(other.backtrace, backtrace) ||
                other.backtrace == backtrace));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, logLevel, message, backtrace);

  @override
  String toString() {
    return 'VeilidUpdate.log(logLevel: $logLevel, message: $message, backtrace: $backtrace)';
  }
}

/// @nodoc
abstract mixin class $VeilidLogCopyWith<$Res>
    implements $VeilidUpdateCopyWith<$Res> {
  factory $VeilidLogCopyWith(VeilidLog value, $Res Function(VeilidLog) _then) =
      _$VeilidLogCopyWithImpl;
  @useResult
  $Res call({VeilidLogLevel logLevel, String message, String? backtrace});
}

/// @nodoc
class _$VeilidLogCopyWithImpl<$Res> implements $VeilidLogCopyWith<$Res> {
  _$VeilidLogCopyWithImpl(this._self, this._then);

  final VeilidLog _self;
  final $Res Function(VeilidLog) _then;

  /// Create a copy of VeilidUpdate
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? logLevel = null,
    Object? message = null,
    Object? backtrace = freezed,
  }) {
    return _then(VeilidLog(
      logLevel: null == logLevel
          ? _self.logLevel
          : logLevel // ignore: cast_nullable_to_non_nullable
              as VeilidLogLevel,
      message: null == message
          ? _self.message
          : message // ignore: cast_nullable_to_non_nullable
              as String,
      backtrace: freezed == backtrace
          ? _self.backtrace
          : backtrace // ignore: cast_nullable_to_non_nullable
              as String?,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class VeilidAppMessage implements VeilidUpdate {
  const VeilidAppMessage(
      {@Uint8ListJsonConverter.jsIsArray() required this.message,
      this.sender,
      this.routeId,
      final String? $type})
      : $type = $type ?? 'AppMessage';
  factory VeilidAppMessage.fromJson(Map<String, dynamic> json) =>
      _$VeilidAppMessageFromJson(json);

  @Uint8ListJsonConverter.jsIsArray()
  final Uint8List message;
  final TypedKey? sender;
  final String? routeId;

  @JsonKey(name: 'kind')
  final String $type;

  /// Create a copy of VeilidUpdate
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $VeilidAppMessageCopyWith<VeilidAppMessage> get copyWith =>
      _$VeilidAppMessageCopyWithImpl<VeilidAppMessage>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$VeilidAppMessageToJson(
      this,
    );
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is VeilidAppMessage &&
            const DeepCollectionEquality().equals(other.message, message) &&
            (identical(other.sender, sender) || other.sender == sender) &&
            (identical(other.routeId, routeId) || other.routeId == routeId));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType,
      const DeepCollectionEquality().hash(message), sender, routeId);

  @override
  String toString() {
    return 'VeilidUpdate.appMessage(message: $message, sender: $sender, routeId: $routeId)';
  }
}

/// @nodoc
abstract mixin class $VeilidAppMessageCopyWith<$Res>
    implements $VeilidUpdateCopyWith<$Res> {
  factory $VeilidAppMessageCopyWith(
          VeilidAppMessage value, $Res Function(VeilidAppMessage) _then) =
      _$VeilidAppMessageCopyWithImpl;
  @useResult
  $Res call(
      {@Uint8ListJsonConverter.jsIsArray() Uint8List message,
      TypedKey? sender,
      String? routeId});
}

/// @nodoc
class _$VeilidAppMessageCopyWithImpl<$Res>
    implements $VeilidAppMessageCopyWith<$Res> {
  _$VeilidAppMessageCopyWithImpl(this._self, this._then);

  final VeilidAppMessage _self;
  final $Res Function(VeilidAppMessage) _then;

  /// Create a copy of VeilidUpdate
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? message = null,
    Object? sender = freezed,
    Object? routeId = freezed,
  }) {
    return _then(VeilidAppMessage(
      message: null == message
          ? _self.message
          : message // ignore: cast_nullable_to_non_nullable
              as Uint8List,
      sender: freezed == sender
          ? _self.sender
          : sender // ignore: cast_nullable_to_non_nullable
              as TypedKey?,
      routeId: freezed == routeId
          ? _self.routeId
          : routeId // ignore: cast_nullable_to_non_nullable
              as String?,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class VeilidAppCall implements VeilidUpdate {
  const VeilidAppCall(
      {@Uint8ListJsonConverter.jsIsArray() required this.message,
      required this.callId,
      this.sender,
      this.routeId,
      final String? $type})
      : $type = $type ?? 'AppCall';
  factory VeilidAppCall.fromJson(Map<String, dynamic> json) =>
      _$VeilidAppCallFromJson(json);

  @Uint8ListJsonConverter.jsIsArray()
  final Uint8List message;
  final String callId;
  final TypedKey? sender;
  final String? routeId;

  @JsonKey(name: 'kind')
  final String $type;

  /// Create a copy of VeilidUpdate
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $VeilidAppCallCopyWith<VeilidAppCall> get copyWith =>
      _$VeilidAppCallCopyWithImpl<VeilidAppCall>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$VeilidAppCallToJson(
      this,
    );
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is VeilidAppCall &&
            const DeepCollectionEquality().equals(other.message, message) &&
            (identical(other.callId, callId) || other.callId == callId) &&
            (identical(other.sender, sender) || other.sender == sender) &&
            (identical(other.routeId, routeId) || other.routeId == routeId));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType,
      const DeepCollectionEquality().hash(message), callId, sender, routeId);

  @override
  String toString() {
    return 'VeilidUpdate.appCall(message: $message, callId: $callId, sender: $sender, routeId: $routeId)';
  }
}

/// @nodoc
abstract mixin class $VeilidAppCallCopyWith<$Res>
    implements $VeilidUpdateCopyWith<$Res> {
  factory $VeilidAppCallCopyWith(
          VeilidAppCall value, $Res Function(VeilidAppCall) _then) =
      _$VeilidAppCallCopyWithImpl;
  @useResult
  $Res call(
      {@Uint8ListJsonConverter.jsIsArray() Uint8List message,
      String callId,
      TypedKey? sender,
      String? routeId});
}

/// @nodoc
class _$VeilidAppCallCopyWithImpl<$Res>
    implements $VeilidAppCallCopyWith<$Res> {
  _$VeilidAppCallCopyWithImpl(this._self, this._then);

  final VeilidAppCall _self;
  final $Res Function(VeilidAppCall) _then;

  /// Create a copy of VeilidUpdate
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? message = null,
    Object? callId = null,
    Object? sender = freezed,
    Object? routeId = freezed,
  }) {
    return _then(VeilidAppCall(
      message: null == message
          ? _self.message
          : message // ignore: cast_nullable_to_non_nullable
              as Uint8List,
      callId: null == callId
          ? _self.callId
          : callId // ignore: cast_nullable_to_non_nullable
              as String,
      sender: freezed == sender
          ? _self.sender
          : sender // ignore: cast_nullable_to_non_nullable
              as TypedKey?,
      routeId: freezed == routeId
          ? _self.routeId
          : routeId // ignore: cast_nullable_to_non_nullable
              as String?,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class VeilidUpdateAttachment implements VeilidUpdate {
  const VeilidUpdateAttachment(
      {required this.state,
      required this.publicInternetReady,
      required this.localNetworkReady,
      required this.uptime,
      required this.attachedUptime,
      final String? $type})
      : $type = $type ?? 'Attachment';
  factory VeilidUpdateAttachment.fromJson(Map<String, dynamic> json) =>
      _$VeilidUpdateAttachmentFromJson(json);

  final AttachmentState state;
  final bool publicInternetReady;
  final bool localNetworkReady;
  final TimestampDuration uptime;
  final TimestampDuration? attachedUptime;

  @JsonKey(name: 'kind')
  final String $type;

  /// Create a copy of VeilidUpdate
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $VeilidUpdateAttachmentCopyWith<VeilidUpdateAttachment> get copyWith =>
      _$VeilidUpdateAttachmentCopyWithImpl<VeilidUpdateAttachment>(
          this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$VeilidUpdateAttachmentToJson(
      this,
    );
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is VeilidUpdateAttachment &&
            (identical(other.state, state) || other.state == state) &&
            (identical(other.publicInternetReady, publicInternetReady) ||
                other.publicInternetReady == publicInternetReady) &&
            (identical(other.localNetworkReady, localNetworkReady) ||
                other.localNetworkReady == localNetworkReady) &&
            (identical(other.uptime, uptime) || other.uptime == uptime) &&
            (identical(other.attachedUptime, attachedUptime) ||
                other.attachedUptime == attachedUptime));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, state, publicInternetReady,
      localNetworkReady, uptime, attachedUptime);

  @override
  String toString() {
    return 'VeilidUpdate.attachment(state: $state, publicInternetReady: $publicInternetReady, localNetworkReady: $localNetworkReady, uptime: $uptime, attachedUptime: $attachedUptime)';
  }
}

/// @nodoc
abstract mixin class $VeilidUpdateAttachmentCopyWith<$Res>
    implements $VeilidUpdateCopyWith<$Res> {
  factory $VeilidUpdateAttachmentCopyWith(VeilidUpdateAttachment value,
          $Res Function(VeilidUpdateAttachment) _then) =
      _$VeilidUpdateAttachmentCopyWithImpl;
  @useResult
  $Res call(
      {AttachmentState state,
      bool publicInternetReady,
      bool localNetworkReady,
      TimestampDuration uptime,
      TimestampDuration? attachedUptime});
}

/// @nodoc
class _$VeilidUpdateAttachmentCopyWithImpl<$Res>
    implements $VeilidUpdateAttachmentCopyWith<$Res> {
  _$VeilidUpdateAttachmentCopyWithImpl(this._self, this._then);

  final VeilidUpdateAttachment _self;
  final $Res Function(VeilidUpdateAttachment) _then;

  /// Create a copy of VeilidUpdate
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? state = null,
    Object? publicInternetReady = null,
    Object? localNetworkReady = null,
    Object? uptime = null,
    Object? attachedUptime = freezed,
  }) {
    return _then(VeilidUpdateAttachment(
      state: null == state
          ? _self.state
          : state // ignore: cast_nullable_to_non_nullable
              as AttachmentState,
      publicInternetReady: null == publicInternetReady
          ? _self.publicInternetReady
          : publicInternetReady // ignore: cast_nullable_to_non_nullable
              as bool,
      localNetworkReady: null == localNetworkReady
          ? _self.localNetworkReady
          : localNetworkReady // ignore: cast_nullable_to_non_nullable
              as bool,
      uptime: null == uptime
          ? _self.uptime
          : uptime // ignore: cast_nullable_to_non_nullable
              as TimestampDuration,
      attachedUptime: freezed == attachedUptime
          ? _self.attachedUptime
          : attachedUptime // ignore: cast_nullable_to_non_nullable
              as TimestampDuration?,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class VeilidUpdateNetwork implements VeilidUpdate {
  const VeilidUpdateNetwork(
      {required this.started,
      required this.bpsDown,
      required this.bpsUp,
      required final List<PeerTableData> peers,
      final String? $type})
      : _peers = peers,
        $type = $type ?? 'Network';
  factory VeilidUpdateNetwork.fromJson(Map<String, dynamic> json) =>
      _$VeilidUpdateNetworkFromJson(json);

  final bool started;
  final BigInt bpsDown;
  final BigInt bpsUp;
  final List<PeerTableData> _peers;
  List<PeerTableData> get peers {
    if (_peers is EqualUnmodifiableListView) return _peers;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_peers);
  }

  @JsonKey(name: 'kind')
  final String $type;

  /// Create a copy of VeilidUpdate
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $VeilidUpdateNetworkCopyWith<VeilidUpdateNetwork> get copyWith =>
      _$VeilidUpdateNetworkCopyWithImpl<VeilidUpdateNetwork>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$VeilidUpdateNetworkToJson(
      this,
    );
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is VeilidUpdateNetwork &&
            (identical(other.started, started) || other.started == started) &&
            (identical(other.bpsDown, bpsDown) || other.bpsDown == bpsDown) &&
            (identical(other.bpsUp, bpsUp) || other.bpsUp == bpsUp) &&
            const DeepCollectionEquality().equals(other._peers, _peers));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, started, bpsDown, bpsUp,
      const DeepCollectionEquality().hash(_peers));

  @override
  String toString() {
    return 'VeilidUpdate.network(started: $started, bpsDown: $bpsDown, bpsUp: $bpsUp, peers: $peers)';
  }
}

/// @nodoc
abstract mixin class $VeilidUpdateNetworkCopyWith<$Res>
    implements $VeilidUpdateCopyWith<$Res> {
  factory $VeilidUpdateNetworkCopyWith(
          VeilidUpdateNetwork value, $Res Function(VeilidUpdateNetwork) _then) =
      _$VeilidUpdateNetworkCopyWithImpl;
  @useResult
  $Res call(
      {bool started, BigInt bpsDown, BigInt bpsUp, List<PeerTableData> peers});
}

/// @nodoc
class _$VeilidUpdateNetworkCopyWithImpl<$Res>
    implements $VeilidUpdateNetworkCopyWith<$Res> {
  _$VeilidUpdateNetworkCopyWithImpl(this._self, this._then);

  final VeilidUpdateNetwork _self;
  final $Res Function(VeilidUpdateNetwork) _then;

  /// Create a copy of VeilidUpdate
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? started = null,
    Object? bpsDown = null,
    Object? bpsUp = null,
    Object? peers = null,
  }) {
    return _then(VeilidUpdateNetwork(
      started: null == started
          ? _self.started
          : started // ignore: cast_nullable_to_non_nullable
              as bool,
      bpsDown: null == bpsDown
          ? _self.bpsDown
          : bpsDown // ignore: cast_nullable_to_non_nullable
              as BigInt,
      bpsUp: null == bpsUp
          ? _self.bpsUp
          : bpsUp // ignore: cast_nullable_to_non_nullable
              as BigInt,
      peers: null == peers
          ? _self._peers
          : peers // ignore: cast_nullable_to_non_nullable
              as List<PeerTableData>,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class VeilidUpdateConfig implements VeilidUpdate {
  const VeilidUpdateConfig({required this.config, final String? $type})
      : $type = $type ?? 'Config';
  factory VeilidUpdateConfig.fromJson(Map<String, dynamic> json) =>
      _$VeilidUpdateConfigFromJson(json);

  final VeilidConfig config;

  @JsonKey(name: 'kind')
  final String $type;

  /// Create a copy of VeilidUpdate
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $VeilidUpdateConfigCopyWith<VeilidUpdateConfig> get copyWith =>
      _$VeilidUpdateConfigCopyWithImpl<VeilidUpdateConfig>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$VeilidUpdateConfigToJson(
      this,
    );
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is VeilidUpdateConfig &&
            (identical(other.config, config) || other.config == config));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, config);

  @override
  String toString() {
    return 'VeilidUpdate.config(config: $config)';
  }
}

/// @nodoc
abstract mixin class $VeilidUpdateConfigCopyWith<$Res>
    implements $VeilidUpdateCopyWith<$Res> {
  factory $VeilidUpdateConfigCopyWith(
          VeilidUpdateConfig value, $Res Function(VeilidUpdateConfig) _then) =
      _$VeilidUpdateConfigCopyWithImpl;
  @useResult
  $Res call({VeilidConfig config});

  $VeilidConfigCopyWith<$Res> get config;
}

/// @nodoc
class _$VeilidUpdateConfigCopyWithImpl<$Res>
    implements $VeilidUpdateConfigCopyWith<$Res> {
  _$VeilidUpdateConfigCopyWithImpl(this._self, this._then);

  final VeilidUpdateConfig _self;
  final $Res Function(VeilidUpdateConfig) _then;

  /// Create a copy of VeilidUpdate
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? config = null,
  }) {
    return _then(VeilidUpdateConfig(
      config: null == config
          ? _self.config
          : config // ignore: cast_nullable_to_non_nullable
              as VeilidConfig,
    ));
  }

  /// Create a copy of VeilidUpdate
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $VeilidConfigCopyWith<$Res> get config {
    return $VeilidConfigCopyWith<$Res>(_self.config, (value) {
      return _then(_self.copyWith(config: value));
    });
  }
}

/// @nodoc
@JsonSerializable()
class VeilidUpdateRouteChange implements VeilidUpdate {
  const VeilidUpdateRouteChange(
      {required final List<String> deadRoutes,
      required final List<String> deadRemoteRoutes,
      final String? $type})
      : _deadRoutes = deadRoutes,
        _deadRemoteRoutes = deadRemoteRoutes,
        $type = $type ?? 'RouteChange';
  factory VeilidUpdateRouteChange.fromJson(Map<String, dynamic> json) =>
      _$VeilidUpdateRouteChangeFromJson(json);

  final List<String> _deadRoutes;
  List<String> get deadRoutes {
    if (_deadRoutes is EqualUnmodifiableListView) return _deadRoutes;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_deadRoutes);
  }

  final List<String> _deadRemoteRoutes;
  List<String> get deadRemoteRoutes {
    if (_deadRemoteRoutes is EqualUnmodifiableListView)
      return _deadRemoteRoutes;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_deadRemoteRoutes);
  }

  @JsonKey(name: 'kind')
  final String $type;

  /// Create a copy of VeilidUpdate
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $VeilidUpdateRouteChangeCopyWith<VeilidUpdateRouteChange> get copyWith =>
      _$VeilidUpdateRouteChangeCopyWithImpl<VeilidUpdateRouteChange>(
          this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$VeilidUpdateRouteChangeToJson(
      this,
    );
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is VeilidUpdateRouteChange &&
            const DeepCollectionEquality()
                .equals(other._deadRoutes, _deadRoutes) &&
            const DeepCollectionEquality()
                .equals(other._deadRemoteRoutes, _deadRemoteRoutes));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(
      runtimeType,
      const DeepCollectionEquality().hash(_deadRoutes),
      const DeepCollectionEquality().hash(_deadRemoteRoutes));

  @override
  String toString() {
    return 'VeilidUpdate.routeChange(deadRoutes: $deadRoutes, deadRemoteRoutes: $deadRemoteRoutes)';
  }
}

/// @nodoc
abstract mixin class $VeilidUpdateRouteChangeCopyWith<$Res>
    implements $VeilidUpdateCopyWith<$Res> {
  factory $VeilidUpdateRouteChangeCopyWith(VeilidUpdateRouteChange value,
          $Res Function(VeilidUpdateRouteChange) _then) =
      _$VeilidUpdateRouteChangeCopyWithImpl;
  @useResult
  $Res call({List<String> deadRoutes, List<String> deadRemoteRoutes});
}

/// @nodoc
class _$VeilidUpdateRouteChangeCopyWithImpl<$Res>
    implements $VeilidUpdateRouteChangeCopyWith<$Res> {
  _$VeilidUpdateRouteChangeCopyWithImpl(this._self, this._then);

  final VeilidUpdateRouteChange _self;
  final $Res Function(VeilidUpdateRouteChange) _then;

  /// Create a copy of VeilidUpdate
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? deadRoutes = null,
    Object? deadRemoteRoutes = null,
  }) {
    return _then(VeilidUpdateRouteChange(
      deadRoutes: null == deadRoutes
          ? _self._deadRoutes
          : deadRoutes // ignore: cast_nullable_to_non_nullable
              as List<String>,
      deadRemoteRoutes: null == deadRemoteRoutes
          ? _self._deadRemoteRoutes
          : deadRemoteRoutes // ignore: cast_nullable_to_non_nullable
              as List<String>,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class VeilidUpdateValueChange implements VeilidUpdate {
  const VeilidUpdateValueChange(
      {required this.key,
      required final List<ValueSubkeyRange> subkeys,
      required this.count,
      required this.value,
      final String? $type})
      : _subkeys = subkeys,
        $type = $type ?? 'ValueChange';
  factory VeilidUpdateValueChange.fromJson(Map<String, dynamic> json) =>
      _$VeilidUpdateValueChangeFromJson(json);

  final TypedKey key;
  final List<ValueSubkeyRange> _subkeys;
  List<ValueSubkeyRange> get subkeys {
    if (_subkeys is EqualUnmodifiableListView) return _subkeys;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_subkeys);
  }

  final int count;
  final ValueData? value;

  @JsonKey(name: 'kind')
  final String $type;

  /// Create a copy of VeilidUpdate
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $VeilidUpdateValueChangeCopyWith<VeilidUpdateValueChange> get copyWith =>
      _$VeilidUpdateValueChangeCopyWithImpl<VeilidUpdateValueChange>(
          this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$VeilidUpdateValueChangeToJson(
      this,
    );
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is VeilidUpdateValueChange &&
            (identical(other.key, key) || other.key == key) &&
            const DeepCollectionEquality().equals(other._subkeys, _subkeys) &&
            (identical(other.count, count) || other.count == count) &&
            (identical(other.value, value) || other.value == value));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, key,
      const DeepCollectionEquality().hash(_subkeys), count, value);

  @override
  String toString() {
    return 'VeilidUpdate.valueChange(key: $key, subkeys: $subkeys, count: $count, value: $value)';
  }
}

/// @nodoc
abstract mixin class $VeilidUpdateValueChangeCopyWith<$Res>
    implements $VeilidUpdateCopyWith<$Res> {
  factory $VeilidUpdateValueChangeCopyWith(VeilidUpdateValueChange value,
          $Res Function(VeilidUpdateValueChange) _then) =
      _$VeilidUpdateValueChangeCopyWithImpl;
  @useResult
  $Res call(
      {TypedKey key,
      List<ValueSubkeyRange> subkeys,
      int count,
      ValueData? value});

  $ValueDataCopyWith<$Res>? get value;
}

/// @nodoc
class _$VeilidUpdateValueChangeCopyWithImpl<$Res>
    implements $VeilidUpdateValueChangeCopyWith<$Res> {
  _$VeilidUpdateValueChangeCopyWithImpl(this._self, this._then);

  final VeilidUpdateValueChange _self;
  final $Res Function(VeilidUpdateValueChange) _then;

  /// Create a copy of VeilidUpdate
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? key = null,
    Object? subkeys = null,
    Object? count = null,
    Object? value = freezed,
  }) {
    return _then(VeilidUpdateValueChange(
      key: null == key
          ? _self.key
          : key // ignore: cast_nullable_to_non_nullable
              as TypedKey,
      subkeys: null == subkeys
          ? _self._subkeys
          : subkeys // ignore: cast_nullable_to_non_nullable
              as List<ValueSubkeyRange>,
      count: null == count
          ? _self.count
          : count // ignore: cast_nullable_to_non_nullable
              as int,
      value: freezed == value
          ? _self.value
          : value // ignore: cast_nullable_to_non_nullable
              as ValueData?,
    ));
  }

  /// Create a copy of VeilidUpdate
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $ValueDataCopyWith<$Res>? get value {
    if (_self.value == null) {
      return null;
    }

    return $ValueDataCopyWith<$Res>(_self.value!, (value) {
      return _then(_self.copyWith(value: value));
    });
  }
}

/// @nodoc
mixin _$VeilidStateAttachment {
  AttachmentState get state;
  bool get publicInternetReady;
  bool get localNetworkReady;
  TimestampDuration get uptime;
  TimestampDuration? get attachedUptime;

  /// Create a copy of VeilidStateAttachment
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $VeilidStateAttachmentCopyWith<VeilidStateAttachment> get copyWith =>
      _$VeilidStateAttachmentCopyWithImpl<VeilidStateAttachment>(
          this as VeilidStateAttachment, _$identity);

  /// Serializes this VeilidStateAttachment to a JSON map.
  Map<String, dynamic> toJson();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is VeilidStateAttachment &&
            (identical(other.state, state) || other.state == state) &&
            (identical(other.publicInternetReady, publicInternetReady) ||
                other.publicInternetReady == publicInternetReady) &&
            (identical(other.localNetworkReady, localNetworkReady) ||
                other.localNetworkReady == localNetworkReady) &&
            (identical(other.uptime, uptime) || other.uptime == uptime) &&
            (identical(other.attachedUptime, attachedUptime) ||
                other.attachedUptime == attachedUptime));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, state, publicInternetReady,
      localNetworkReady, uptime, attachedUptime);

  @override
  String toString() {
    return 'VeilidStateAttachment(state: $state, publicInternetReady: $publicInternetReady, localNetworkReady: $localNetworkReady, uptime: $uptime, attachedUptime: $attachedUptime)';
  }
}

/// @nodoc
abstract mixin class $VeilidStateAttachmentCopyWith<$Res> {
  factory $VeilidStateAttachmentCopyWith(VeilidStateAttachment value,
          $Res Function(VeilidStateAttachment) _then) =
      _$VeilidStateAttachmentCopyWithImpl;
  @useResult
  $Res call(
      {AttachmentState state,
      bool publicInternetReady,
      bool localNetworkReady,
      TimestampDuration uptime,
      TimestampDuration? attachedUptime});
}

/// @nodoc
class _$VeilidStateAttachmentCopyWithImpl<$Res>
    implements $VeilidStateAttachmentCopyWith<$Res> {
  _$VeilidStateAttachmentCopyWithImpl(this._self, this._then);

  final VeilidStateAttachment _self;
  final $Res Function(VeilidStateAttachment) _then;

  /// Create a copy of VeilidStateAttachment
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? state = null,
    Object? publicInternetReady = null,
    Object? localNetworkReady = null,
    Object? uptime = null,
    Object? attachedUptime = freezed,
  }) {
    return _then(_self.copyWith(
      state: null == state
          ? _self.state
          : state // ignore: cast_nullable_to_non_nullable
              as AttachmentState,
      publicInternetReady: null == publicInternetReady
          ? _self.publicInternetReady
          : publicInternetReady // ignore: cast_nullable_to_non_nullable
              as bool,
      localNetworkReady: null == localNetworkReady
          ? _self.localNetworkReady
          : localNetworkReady // ignore: cast_nullable_to_non_nullable
              as bool,
      uptime: null == uptime
          ? _self.uptime
          : uptime // ignore: cast_nullable_to_non_nullable
              as TimestampDuration,
      attachedUptime: freezed == attachedUptime
          ? _self.attachedUptime
          : attachedUptime // ignore: cast_nullable_to_non_nullable
              as TimestampDuration?,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _VeilidStateAttachment implements VeilidStateAttachment {
  const _VeilidStateAttachment(
      {required this.state,
      required this.publicInternetReady,
      required this.localNetworkReady,
      required this.uptime,
      required this.attachedUptime});
  factory _VeilidStateAttachment.fromJson(Map<String, dynamic> json) =>
      _$VeilidStateAttachmentFromJson(json);

  @override
  final AttachmentState state;
  @override
  final bool publicInternetReady;
  @override
  final bool localNetworkReady;
  @override
  final TimestampDuration uptime;
  @override
  final TimestampDuration? attachedUptime;

  /// Create a copy of VeilidStateAttachment
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  _$VeilidStateAttachmentCopyWith<_VeilidStateAttachment> get copyWith =>
      __$VeilidStateAttachmentCopyWithImpl<_VeilidStateAttachment>(
          this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$VeilidStateAttachmentToJson(
      this,
    );
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _VeilidStateAttachment &&
            (identical(other.state, state) || other.state == state) &&
            (identical(other.publicInternetReady, publicInternetReady) ||
                other.publicInternetReady == publicInternetReady) &&
            (identical(other.localNetworkReady, localNetworkReady) ||
                other.localNetworkReady == localNetworkReady) &&
            (identical(other.uptime, uptime) || other.uptime == uptime) &&
            (identical(other.attachedUptime, attachedUptime) ||
                other.attachedUptime == attachedUptime));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, state, publicInternetReady,
      localNetworkReady, uptime, attachedUptime);

  @override
  String toString() {
    return 'VeilidStateAttachment(state: $state, publicInternetReady: $publicInternetReady, localNetworkReady: $localNetworkReady, uptime: $uptime, attachedUptime: $attachedUptime)';
  }
}

/// @nodoc
abstract mixin class _$VeilidStateAttachmentCopyWith<$Res>
    implements $VeilidStateAttachmentCopyWith<$Res> {
  factory _$VeilidStateAttachmentCopyWith(_VeilidStateAttachment value,
          $Res Function(_VeilidStateAttachment) _then) =
      __$VeilidStateAttachmentCopyWithImpl;
  @override
  @useResult
  $Res call(
      {AttachmentState state,
      bool publicInternetReady,
      bool localNetworkReady,
      TimestampDuration uptime,
      TimestampDuration? attachedUptime});
}

/// @nodoc
class __$VeilidStateAttachmentCopyWithImpl<$Res>
    implements _$VeilidStateAttachmentCopyWith<$Res> {
  __$VeilidStateAttachmentCopyWithImpl(this._self, this._then);

  final _VeilidStateAttachment _self;
  final $Res Function(_VeilidStateAttachment) _then;

  /// Create a copy of VeilidStateAttachment
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? state = null,
    Object? publicInternetReady = null,
    Object? localNetworkReady = null,
    Object? uptime = null,
    Object? attachedUptime = freezed,
  }) {
    return _then(_VeilidStateAttachment(
      state: null == state
          ? _self.state
          : state // ignore: cast_nullable_to_non_nullable
              as AttachmentState,
      publicInternetReady: null == publicInternetReady
          ? _self.publicInternetReady
          : publicInternetReady // ignore: cast_nullable_to_non_nullable
              as bool,
      localNetworkReady: null == localNetworkReady
          ? _self.localNetworkReady
          : localNetworkReady // ignore: cast_nullable_to_non_nullable
              as bool,
      uptime: null == uptime
          ? _self.uptime
          : uptime // ignore: cast_nullable_to_non_nullable
              as TimestampDuration,
      attachedUptime: freezed == attachedUptime
          ? _self.attachedUptime
          : attachedUptime // ignore: cast_nullable_to_non_nullable
              as TimestampDuration?,
    ));
  }
}

/// @nodoc
mixin _$VeilidStateNetwork {
  bool get started;
  BigInt get bpsDown;
  BigInt get bpsUp;
  List<PeerTableData> get peers;

  /// Create a copy of VeilidStateNetwork
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $VeilidStateNetworkCopyWith<VeilidStateNetwork> get copyWith =>
      _$VeilidStateNetworkCopyWithImpl<VeilidStateNetwork>(
          this as VeilidStateNetwork, _$identity);

  /// Serializes this VeilidStateNetwork to a JSON map.
  Map<String, dynamic> toJson();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is VeilidStateNetwork &&
            (identical(other.started, started) || other.started == started) &&
            (identical(other.bpsDown, bpsDown) || other.bpsDown == bpsDown) &&
            (identical(other.bpsUp, bpsUp) || other.bpsUp == bpsUp) &&
            const DeepCollectionEquality().equals(other.peers, peers));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, started, bpsDown, bpsUp,
      const DeepCollectionEquality().hash(peers));

  @override
  String toString() {
    return 'VeilidStateNetwork(started: $started, bpsDown: $bpsDown, bpsUp: $bpsUp, peers: $peers)';
  }
}

/// @nodoc
abstract mixin class $VeilidStateNetworkCopyWith<$Res> {
  factory $VeilidStateNetworkCopyWith(
          VeilidStateNetwork value, $Res Function(VeilidStateNetwork) _then) =
      _$VeilidStateNetworkCopyWithImpl;
  @useResult
  $Res call(
      {bool started, BigInt bpsDown, BigInt bpsUp, List<PeerTableData> peers});
}

/// @nodoc
class _$VeilidStateNetworkCopyWithImpl<$Res>
    implements $VeilidStateNetworkCopyWith<$Res> {
  _$VeilidStateNetworkCopyWithImpl(this._self, this._then);

  final VeilidStateNetwork _self;
  final $Res Function(VeilidStateNetwork) _then;

  /// Create a copy of VeilidStateNetwork
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? started = null,
    Object? bpsDown = null,
    Object? bpsUp = null,
    Object? peers = null,
  }) {
    return _then(_self.copyWith(
      started: null == started
          ? _self.started
          : started // ignore: cast_nullable_to_non_nullable
              as bool,
      bpsDown: null == bpsDown
          ? _self.bpsDown
          : bpsDown // ignore: cast_nullable_to_non_nullable
              as BigInt,
      bpsUp: null == bpsUp
          ? _self.bpsUp
          : bpsUp // ignore: cast_nullable_to_non_nullable
              as BigInt,
      peers: null == peers
          ? _self.peers
          : peers // ignore: cast_nullable_to_non_nullable
              as List<PeerTableData>,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _VeilidStateNetwork implements VeilidStateNetwork {
  const _VeilidStateNetwork(
      {required this.started,
      required this.bpsDown,
      required this.bpsUp,
      required final List<PeerTableData> peers})
      : _peers = peers;
  factory _VeilidStateNetwork.fromJson(Map<String, dynamic> json) =>
      _$VeilidStateNetworkFromJson(json);

  @override
  final bool started;
  @override
  final BigInt bpsDown;
  @override
  final BigInt bpsUp;
  final List<PeerTableData> _peers;
  @override
  List<PeerTableData> get peers {
    if (_peers is EqualUnmodifiableListView) return _peers;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_peers);
  }

  /// Create a copy of VeilidStateNetwork
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  _$VeilidStateNetworkCopyWith<_VeilidStateNetwork> get copyWith =>
      __$VeilidStateNetworkCopyWithImpl<_VeilidStateNetwork>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$VeilidStateNetworkToJson(
      this,
    );
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _VeilidStateNetwork &&
            (identical(other.started, started) || other.started == started) &&
            (identical(other.bpsDown, bpsDown) || other.bpsDown == bpsDown) &&
            (identical(other.bpsUp, bpsUp) || other.bpsUp == bpsUp) &&
            const DeepCollectionEquality().equals(other._peers, _peers));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, started, bpsDown, bpsUp,
      const DeepCollectionEquality().hash(_peers));

  @override
  String toString() {
    return 'VeilidStateNetwork(started: $started, bpsDown: $bpsDown, bpsUp: $bpsUp, peers: $peers)';
  }
}

/// @nodoc
abstract mixin class _$VeilidStateNetworkCopyWith<$Res>
    implements $VeilidStateNetworkCopyWith<$Res> {
  factory _$VeilidStateNetworkCopyWith(
          _VeilidStateNetwork value, $Res Function(_VeilidStateNetwork) _then) =
      __$VeilidStateNetworkCopyWithImpl;
  @override
  @useResult
  $Res call(
      {bool started, BigInt bpsDown, BigInt bpsUp, List<PeerTableData> peers});
}

/// @nodoc
class __$VeilidStateNetworkCopyWithImpl<$Res>
    implements _$VeilidStateNetworkCopyWith<$Res> {
  __$VeilidStateNetworkCopyWithImpl(this._self, this._then);

  final _VeilidStateNetwork _self;
  final $Res Function(_VeilidStateNetwork) _then;

  /// Create a copy of VeilidStateNetwork
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? started = null,
    Object? bpsDown = null,
    Object? bpsUp = null,
    Object? peers = null,
  }) {
    return _then(_VeilidStateNetwork(
      started: null == started
          ? _self.started
          : started // ignore: cast_nullable_to_non_nullable
              as bool,
      bpsDown: null == bpsDown
          ? _self.bpsDown
          : bpsDown // ignore: cast_nullable_to_non_nullable
              as BigInt,
      bpsUp: null == bpsUp
          ? _self.bpsUp
          : bpsUp // ignore: cast_nullable_to_non_nullable
              as BigInt,
      peers: null == peers
          ? _self._peers
          : peers // ignore: cast_nullable_to_non_nullable
              as List<PeerTableData>,
    ));
  }
}

/// @nodoc
mixin _$VeilidStateConfig {
  VeilidConfig get config;

  /// Create a copy of VeilidStateConfig
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $VeilidStateConfigCopyWith<VeilidStateConfig> get copyWith =>
      _$VeilidStateConfigCopyWithImpl<VeilidStateConfig>(
          this as VeilidStateConfig, _$identity);

  /// Serializes this VeilidStateConfig to a JSON map.
  Map<String, dynamic> toJson();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is VeilidStateConfig &&
            (identical(other.config, config) || other.config == config));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, config);

  @override
  String toString() {
    return 'VeilidStateConfig(config: $config)';
  }
}

/// @nodoc
abstract mixin class $VeilidStateConfigCopyWith<$Res> {
  factory $VeilidStateConfigCopyWith(
          VeilidStateConfig value, $Res Function(VeilidStateConfig) _then) =
      _$VeilidStateConfigCopyWithImpl;
  @useResult
  $Res call({VeilidConfig config});

  $VeilidConfigCopyWith<$Res> get config;
}

/// @nodoc
class _$VeilidStateConfigCopyWithImpl<$Res>
    implements $VeilidStateConfigCopyWith<$Res> {
  _$VeilidStateConfigCopyWithImpl(this._self, this._then);

  final VeilidStateConfig _self;
  final $Res Function(VeilidStateConfig) _then;

  /// Create a copy of VeilidStateConfig
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? config = null,
  }) {
    return _then(_self.copyWith(
      config: null == config
          ? _self.config
          : config // ignore: cast_nullable_to_non_nullable
              as VeilidConfig,
    ));
  }

  /// Create a copy of VeilidStateConfig
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $VeilidConfigCopyWith<$Res> get config {
    return $VeilidConfigCopyWith<$Res>(_self.config, (value) {
      return _then(_self.copyWith(config: value));
    });
  }
}

/// @nodoc
@JsonSerializable()
class _VeilidStateConfig implements VeilidStateConfig {
  const _VeilidStateConfig({required this.config});
  factory _VeilidStateConfig.fromJson(Map<String, dynamic> json) =>
      _$VeilidStateConfigFromJson(json);

  @override
  final VeilidConfig config;

  /// Create a copy of VeilidStateConfig
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  _$VeilidStateConfigCopyWith<_VeilidStateConfig> get copyWith =>
      __$VeilidStateConfigCopyWithImpl<_VeilidStateConfig>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$VeilidStateConfigToJson(
      this,
    );
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _VeilidStateConfig &&
            (identical(other.config, config) || other.config == config));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, config);

  @override
  String toString() {
    return 'VeilidStateConfig(config: $config)';
  }
}

/// @nodoc
abstract mixin class _$VeilidStateConfigCopyWith<$Res>
    implements $VeilidStateConfigCopyWith<$Res> {
  factory _$VeilidStateConfigCopyWith(
          _VeilidStateConfig value, $Res Function(_VeilidStateConfig) _then) =
      __$VeilidStateConfigCopyWithImpl;
  @override
  @useResult
  $Res call({VeilidConfig config});

  @override
  $VeilidConfigCopyWith<$Res> get config;
}

/// @nodoc
class __$VeilidStateConfigCopyWithImpl<$Res>
    implements _$VeilidStateConfigCopyWith<$Res> {
  __$VeilidStateConfigCopyWithImpl(this._self, this._then);

  final _VeilidStateConfig _self;
  final $Res Function(_VeilidStateConfig) _then;

  /// Create a copy of VeilidStateConfig
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? config = null,
  }) {
    return _then(_VeilidStateConfig(
      config: null == config
          ? _self.config
          : config // ignore: cast_nullable_to_non_nullable
              as VeilidConfig,
    ));
  }

  /// Create a copy of VeilidStateConfig
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $VeilidConfigCopyWith<$Res> get config {
    return $VeilidConfigCopyWith<$Res>(_self.config, (value) {
      return _then(_self.copyWith(config: value));
    });
  }
}

/// @nodoc
mixin _$VeilidState {
  VeilidStateAttachment get attachment;
  VeilidStateNetwork get network;
  VeilidStateConfig get config;

  /// Create a copy of VeilidState
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $VeilidStateCopyWith<VeilidState> get copyWith =>
      _$VeilidStateCopyWithImpl<VeilidState>(this as VeilidState, _$identity);

  /// Serializes this VeilidState to a JSON map.
  Map<String, dynamic> toJson();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is VeilidState &&
            (identical(other.attachment, attachment) ||
                other.attachment == attachment) &&
            (identical(other.network, network) || other.network == network) &&
            (identical(other.config, config) || other.config == config));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, attachment, network, config);

  @override
  String toString() {
    return 'VeilidState(attachment: $attachment, network: $network, config: $config)';
  }
}

/// @nodoc
abstract mixin class $VeilidStateCopyWith<$Res> {
  factory $VeilidStateCopyWith(
          VeilidState value, $Res Function(VeilidState) _then) =
      _$VeilidStateCopyWithImpl;
  @useResult
  $Res call(
      {VeilidStateAttachment attachment,
      VeilidStateNetwork network,
      VeilidStateConfig config});

  $VeilidStateAttachmentCopyWith<$Res> get attachment;
  $VeilidStateNetworkCopyWith<$Res> get network;
  $VeilidStateConfigCopyWith<$Res> get config;
}

/// @nodoc
class _$VeilidStateCopyWithImpl<$Res> implements $VeilidStateCopyWith<$Res> {
  _$VeilidStateCopyWithImpl(this._self, this._then);

  final VeilidState _self;
  final $Res Function(VeilidState) _then;

  /// Create a copy of VeilidState
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? attachment = null,
    Object? network = null,
    Object? config = null,
  }) {
    return _then(_self.copyWith(
      attachment: null == attachment
          ? _self.attachment
          : attachment // ignore: cast_nullable_to_non_nullable
              as VeilidStateAttachment,
      network: null == network
          ? _self.network
          : network // ignore: cast_nullable_to_non_nullable
              as VeilidStateNetwork,
      config: null == config
          ? _self.config
          : config // ignore: cast_nullable_to_non_nullable
              as VeilidStateConfig,
    ));
  }

  /// Create a copy of VeilidState
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $VeilidStateAttachmentCopyWith<$Res> get attachment {
    return $VeilidStateAttachmentCopyWith<$Res>(_self.attachment, (value) {
      return _then(_self.copyWith(attachment: value));
    });
  }

  /// Create a copy of VeilidState
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $VeilidStateNetworkCopyWith<$Res> get network {
    return $VeilidStateNetworkCopyWith<$Res>(_self.network, (value) {
      return _then(_self.copyWith(network: value));
    });
  }

  /// Create a copy of VeilidState
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $VeilidStateConfigCopyWith<$Res> get config {
    return $VeilidStateConfigCopyWith<$Res>(_self.config, (value) {
      return _then(_self.copyWith(config: value));
    });
  }
}

/// @nodoc
@JsonSerializable()
class _VeilidState implements VeilidState {
  const _VeilidState(
      {required this.attachment, required this.network, required this.config});
  factory _VeilidState.fromJson(Map<String, dynamic> json) =>
      _$VeilidStateFromJson(json);

  @override
  final VeilidStateAttachment attachment;
  @override
  final VeilidStateNetwork network;
  @override
  final VeilidStateConfig config;

  /// Create a copy of VeilidState
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  _$VeilidStateCopyWith<_VeilidState> get copyWith =>
      __$VeilidStateCopyWithImpl<_VeilidState>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$VeilidStateToJson(
      this,
    );
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _VeilidState &&
            (identical(other.attachment, attachment) ||
                other.attachment == attachment) &&
            (identical(other.network, network) || other.network == network) &&
            (identical(other.config, config) || other.config == config));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, attachment, network, config);

  @override
  String toString() {
    return 'VeilidState(attachment: $attachment, network: $network, config: $config)';
  }
}

/// @nodoc
abstract mixin class _$VeilidStateCopyWith<$Res>
    implements $VeilidStateCopyWith<$Res> {
  factory _$VeilidStateCopyWith(
          _VeilidState value, $Res Function(_VeilidState) _then) =
      __$VeilidStateCopyWithImpl;
  @override
  @useResult
  $Res call(
      {VeilidStateAttachment attachment,
      VeilidStateNetwork network,
      VeilidStateConfig config});

  @override
  $VeilidStateAttachmentCopyWith<$Res> get attachment;
  @override
  $VeilidStateNetworkCopyWith<$Res> get network;
  @override
  $VeilidStateConfigCopyWith<$Res> get config;
}

/// @nodoc
class __$VeilidStateCopyWithImpl<$Res> implements _$VeilidStateCopyWith<$Res> {
  __$VeilidStateCopyWithImpl(this._self, this._then);

  final _VeilidState _self;
  final $Res Function(_VeilidState) _then;

  /// Create a copy of VeilidState
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? attachment = null,
    Object? network = null,
    Object? config = null,
  }) {
    return _then(_VeilidState(
      attachment: null == attachment
          ? _self.attachment
          : attachment // ignore: cast_nullable_to_non_nullable
              as VeilidStateAttachment,
      network: null == network
          ? _self.network
          : network // ignore: cast_nullable_to_non_nullable
              as VeilidStateNetwork,
      config: null == config
          ? _self.config
          : config // ignore: cast_nullable_to_non_nullable
              as VeilidStateConfig,
    ));
  }

  /// Create a copy of VeilidState
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $VeilidStateAttachmentCopyWith<$Res> get attachment {
    return $VeilidStateAttachmentCopyWith<$Res>(_self.attachment, (value) {
      return _then(_self.copyWith(attachment: value));
    });
  }

  /// Create a copy of VeilidState
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $VeilidStateNetworkCopyWith<$Res> get network {
    return $VeilidStateNetworkCopyWith<$Res>(_self.network, (value) {
      return _then(_self.copyWith(network: value));
    });
  }

  /// Create a copy of VeilidState
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $VeilidStateConfigCopyWith<$Res> get config {
    return $VeilidStateConfigCopyWith<$Res>(_self.config, (value) {
      return _then(_self.copyWith(config: value));
    });
  }
}

// dart format on
