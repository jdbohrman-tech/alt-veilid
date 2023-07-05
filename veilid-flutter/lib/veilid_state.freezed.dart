// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'veilid_state.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#custom-getters-and-methods');

LatencyStats _$LatencyStatsFromJson(Map<String, dynamic> json) {
  return _LatencyStats.fromJson(json);
}

/// @nodoc
mixin _$LatencyStats {
  TimestampDuration get fastest => throw _privateConstructorUsedError;
  TimestampDuration get average => throw _privateConstructorUsedError;
  TimestampDuration get slowest => throw _privateConstructorUsedError;

  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
  @JsonKey(ignore: true)
  $LatencyStatsCopyWith<LatencyStats> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $LatencyStatsCopyWith<$Res> {
  factory $LatencyStatsCopyWith(
          LatencyStats value, $Res Function(LatencyStats) then) =
      _$LatencyStatsCopyWithImpl<$Res, LatencyStats>;
  @useResult
  $Res call(
      {TimestampDuration fastest,
      TimestampDuration average,
      TimestampDuration slowest});
}

/// @nodoc
class _$LatencyStatsCopyWithImpl<$Res, $Val extends LatencyStats>
    implements $LatencyStatsCopyWith<$Res> {
  _$LatencyStatsCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? fastest = null,
    Object? average = null,
    Object? slowest = null,
  }) {
    return _then(_value.copyWith(
      fastest: null == fastest
          ? _value.fastest
          : fastest // ignore: cast_nullable_to_non_nullable
              as TimestampDuration,
      average: null == average
          ? _value.average
          : average // ignore: cast_nullable_to_non_nullable
              as TimestampDuration,
      slowest: null == slowest
          ? _value.slowest
          : slowest // ignore: cast_nullable_to_non_nullable
              as TimestampDuration,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$_LatencyStatsCopyWith<$Res>
    implements $LatencyStatsCopyWith<$Res> {
  factory _$$_LatencyStatsCopyWith(
          _$_LatencyStats value, $Res Function(_$_LatencyStats) then) =
      __$$_LatencyStatsCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {TimestampDuration fastest,
      TimestampDuration average,
      TimestampDuration slowest});
}

/// @nodoc
class __$$_LatencyStatsCopyWithImpl<$Res>
    extends _$LatencyStatsCopyWithImpl<$Res, _$_LatencyStats>
    implements _$$_LatencyStatsCopyWith<$Res> {
  __$$_LatencyStatsCopyWithImpl(
      _$_LatencyStats _value, $Res Function(_$_LatencyStats) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? fastest = null,
    Object? average = null,
    Object? slowest = null,
  }) {
    return _then(_$_LatencyStats(
      fastest: null == fastest
          ? _value.fastest
          : fastest // ignore: cast_nullable_to_non_nullable
              as TimestampDuration,
      average: null == average
          ? _value.average
          : average // ignore: cast_nullable_to_non_nullable
              as TimestampDuration,
      slowest: null == slowest
          ? _value.slowest
          : slowest // ignore: cast_nullable_to_non_nullable
              as TimestampDuration,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$_LatencyStats implements _LatencyStats {
  const _$_LatencyStats(
      {required this.fastest, required this.average, required this.slowest});

  factory _$_LatencyStats.fromJson(Map<String, dynamic> json) =>
      _$$_LatencyStatsFromJson(json);

  @override
  final TimestampDuration fastest;
  @override
  final TimestampDuration average;
  @override
  final TimestampDuration slowest;

  @override
  String toString() {
    return 'LatencyStats(fastest: $fastest, average: $average, slowest: $slowest)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$_LatencyStats &&
            (identical(other.fastest, fastest) || other.fastest == fastest) &&
            (identical(other.average, average) || other.average == average) &&
            (identical(other.slowest, slowest) || other.slowest == slowest));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, fastest, average, slowest);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$_LatencyStatsCopyWith<_$_LatencyStats> get copyWith =>
      __$$_LatencyStatsCopyWithImpl<_$_LatencyStats>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$_LatencyStatsToJson(
      this,
    );
  }
}

abstract class _LatencyStats implements LatencyStats {
  const factory _LatencyStats(
      {required final TimestampDuration fastest,
      required final TimestampDuration average,
      required final TimestampDuration slowest}) = _$_LatencyStats;

  factory _LatencyStats.fromJson(Map<String, dynamic> json) =
      _$_LatencyStats.fromJson;

  @override
  TimestampDuration get fastest;
  @override
  TimestampDuration get average;
  @override
  TimestampDuration get slowest;
  @override
  @JsonKey(ignore: true)
  _$$_LatencyStatsCopyWith<_$_LatencyStats> get copyWith =>
      throw _privateConstructorUsedError;
}

TransferStats _$TransferStatsFromJson(Map<String, dynamic> json) {
  return _TransferStats.fromJson(json);
}

/// @nodoc
mixin _$TransferStats {
  BigInt get total => throw _privateConstructorUsedError;
  BigInt get maximum => throw _privateConstructorUsedError;
  BigInt get average => throw _privateConstructorUsedError;
  BigInt get minimum => throw _privateConstructorUsedError;

  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
  @JsonKey(ignore: true)
  $TransferStatsCopyWith<TransferStats> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $TransferStatsCopyWith<$Res> {
  factory $TransferStatsCopyWith(
          TransferStats value, $Res Function(TransferStats) then) =
      _$TransferStatsCopyWithImpl<$Res, TransferStats>;
  @useResult
  $Res call({BigInt total, BigInt maximum, BigInt average, BigInt minimum});
}

/// @nodoc
class _$TransferStatsCopyWithImpl<$Res, $Val extends TransferStats>
    implements $TransferStatsCopyWith<$Res> {
  _$TransferStatsCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? total = null,
    Object? maximum = null,
    Object? average = null,
    Object? minimum = null,
  }) {
    return _then(_value.copyWith(
      total: null == total
          ? _value.total
          : total // ignore: cast_nullable_to_non_nullable
              as BigInt,
      maximum: null == maximum
          ? _value.maximum
          : maximum // ignore: cast_nullable_to_non_nullable
              as BigInt,
      average: null == average
          ? _value.average
          : average // ignore: cast_nullable_to_non_nullable
              as BigInt,
      minimum: null == minimum
          ? _value.minimum
          : minimum // ignore: cast_nullable_to_non_nullable
              as BigInt,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$_TransferStatsCopyWith<$Res>
    implements $TransferStatsCopyWith<$Res> {
  factory _$$_TransferStatsCopyWith(
          _$_TransferStats value, $Res Function(_$_TransferStats) then) =
      __$$_TransferStatsCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({BigInt total, BigInt maximum, BigInt average, BigInt minimum});
}

/// @nodoc
class __$$_TransferStatsCopyWithImpl<$Res>
    extends _$TransferStatsCopyWithImpl<$Res, _$_TransferStats>
    implements _$$_TransferStatsCopyWith<$Res> {
  __$$_TransferStatsCopyWithImpl(
      _$_TransferStats _value, $Res Function(_$_TransferStats) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? total = null,
    Object? maximum = null,
    Object? average = null,
    Object? minimum = null,
  }) {
    return _then(_$_TransferStats(
      total: null == total
          ? _value.total
          : total // ignore: cast_nullable_to_non_nullable
              as BigInt,
      maximum: null == maximum
          ? _value.maximum
          : maximum // ignore: cast_nullable_to_non_nullable
              as BigInt,
      average: null == average
          ? _value.average
          : average // ignore: cast_nullable_to_non_nullable
              as BigInt,
      minimum: null == minimum
          ? _value.minimum
          : minimum // ignore: cast_nullable_to_non_nullable
              as BigInt,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$_TransferStats implements _TransferStats {
  const _$_TransferStats(
      {required this.total,
      required this.maximum,
      required this.average,
      required this.minimum});

  factory _$_TransferStats.fromJson(Map<String, dynamic> json) =>
      _$$_TransferStatsFromJson(json);

  @override
  final BigInt total;
  @override
  final BigInt maximum;
  @override
  final BigInt average;
  @override
  final BigInt minimum;

  @override
  String toString() {
    return 'TransferStats(total: $total, maximum: $maximum, average: $average, minimum: $minimum)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$_TransferStats &&
            (identical(other.total, total) || other.total == total) &&
            (identical(other.maximum, maximum) || other.maximum == maximum) &&
            (identical(other.average, average) || other.average == average) &&
            (identical(other.minimum, minimum) || other.minimum == minimum));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode =>
      Object.hash(runtimeType, total, maximum, average, minimum);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$_TransferStatsCopyWith<_$_TransferStats> get copyWith =>
      __$$_TransferStatsCopyWithImpl<_$_TransferStats>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$_TransferStatsToJson(
      this,
    );
  }
}

abstract class _TransferStats implements TransferStats {
  const factory _TransferStats(
      {required final BigInt total,
      required final BigInt maximum,
      required final BigInt average,
      required final BigInt minimum}) = _$_TransferStats;

  factory _TransferStats.fromJson(Map<String, dynamic> json) =
      _$_TransferStats.fromJson;

  @override
  BigInt get total;
  @override
  BigInt get maximum;
  @override
  BigInt get average;
  @override
  BigInt get minimum;
  @override
  @JsonKey(ignore: true)
  _$$_TransferStatsCopyWith<_$_TransferStats> get copyWith =>
      throw _privateConstructorUsedError;
}

TransferStatsDownUp _$TransferStatsDownUpFromJson(Map<String, dynamic> json) {
  return _TransferStatsDownUp.fromJson(json);
}

/// @nodoc
mixin _$TransferStatsDownUp {
  TransferStats get down => throw _privateConstructorUsedError;
  TransferStats get up => throw _privateConstructorUsedError;

  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
  @JsonKey(ignore: true)
  $TransferStatsDownUpCopyWith<TransferStatsDownUp> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $TransferStatsDownUpCopyWith<$Res> {
  factory $TransferStatsDownUpCopyWith(
          TransferStatsDownUp value, $Res Function(TransferStatsDownUp) then) =
      _$TransferStatsDownUpCopyWithImpl<$Res, TransferStatsDownUp>;
  @useResult
  $Res call({TransferStats down, TransferStats up});

  $TransferStatsCopyWith<$Res> get down;
  $TransferStatsCopyWith<$Res> get up;
}

/// @nodoc
class _$TransferStatsDownUpCopyWithImpl<$Res, $Val extends TransferStatsDownUp>
    implements $TransferStatsDownUpCopyWith<$Res> {
  _$TransferStatsDownUpCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? down = null,
    Object? up = null,
  }) {
    return _then(_value.copyWith(
      down: null == down
          ? _value.down
          : down // ignore: cast_nullable_to_non_nullable
              as TransferStats,
      up: null == up
          ? _value.up
          : up // ignore: cast_nullable_to_non_nullable
              as TransferStats,
    ) as $Val);
  }

  @override
  @pragma('vm:prefer-inline')
  $TransferStatsCopyWith<$Res> get down {
    return $TransferStatsCopyWith<$Res>(_value.down, (value) {
      return _then(_value.copyWith(down: value) as $Val);
    });
  }

  @override
  @pragma('vm:prefer-inline')
  $TransferStatsCopyWith<$Res> get up {
    return $TransferStatsCopyWith<$Res>(_value.up, (value) {
      return _then(_value.copyWith(up: value) as $Val);
    });
  }
}

/// @nodoc
abstract class _$$_TransferStatsDownUpCopyWith<$Res>
    implements $TransferStatsDownUpCopyWith<$Res> {
  factory _$$_TransferStatsDownUpCopyWith(_$_TransferStatsDownUp value,
          $Res Function(_$_TransferStatsDownUp) then) =
      __$$_TransferStatsDownUpCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({TransferStats down, TransferStats up});

  @override
  $TransferStatsCopyWith<$Res> get down;
  @override
  $TransferStatsCopyWith<$Res> get up;
}

/// @nodoc
class __$$_TransferStatsDownUpCopyWithImpl<$Res>
    extends _$TransferStatsDownUpCopyWithImpl<$Res, _$_TransferStatsDownUp>
    implements _$$_TransferStatsDownUpCopyWith<$Res> {
  __$$_TransferStatsDownUpCopyWithImpl(_$_TransferStatsDownUp _value,
      $Res Function(_$_TransferStatsDownUp) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? down = null,
    Object? up = null,
  }) {
    return _then(_$_TransferStatsDownUp(
      down: null == down
          ? _value.down
          : down // ignore: cast_nullable_to_non_nullable
              as TransferStats,
      up: null == up
          ? _value.up
          : up // ignore: cast_nullable_to_non_nullable
              as TransferStats,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$_TransferStatsDownUp implements _TransferStatsDownUp {
  const _$_TransferStatsDownUp({required this.down, required this.up});

  factory _$_TransferStatsDownUp.fromJson(Map<String, dynamic> json) =>
      _$$_TransferStatsDownUpFromJson(json);

  @override
  final TransferStats down;
  @override
  final TransferStats up;

  @override
  String toString() {
    return 'TransferStatsDownUp(down: $down, up: $up)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$_TransferStatsDownUp &&
            (identical(other.down, down) || other.down == down) &&
            (identical(other.up, up) || other.up == up));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, down, up);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$_TransferStatsDownUpCopyWith<_$_TransferStatsDownUp> get copyWith =>
      __$$_TransferStatsDownUpCopyWithImpl<_$_TransferStatsDownUp>(
          this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$_TransferStatsDownUpToJson(
      this,
    );
  }
}

abstract class _TransferStatsDownUp implements TransferStatsDownUp {
  const factory _TransferStatsDownUp(
      {required final TransferStats down,
      required final TransferStats up}) = _$_TransferStatsDownUp;

  factory _TransferStatsDownUp.fromJson(Map<String, dynamic> json) =
      _$_TransferStatsDownUp.fromJson;

  @override
  TransferStats get down;
  @override
  TransferStats get up;
  @override
  @JsonKey(ignore: true)
  _$$_TransferStatsDownUpCopyWith<_$_TransferStatsDownUp> get copyWith =>
      throw _privateConstructorUsedError;
}

RPCStats _$RPCStatsFromJson(Map<String, dynamic> json) {
  return _RPCStats.fromJson(json);
}

/// @nodoc
mixin _$RPCStats {
  int get messagesSent => throw _privateConstructorUsedError;
  int get messagesRcvd => throw _privateConstructorUsedError;
  int get questionsInFlight => throw _privateConstructorUsedError;
  Timestamp? get lastQuestion => throw _privateConstructorUsedError;
  Timestamp? get lastSeenTs => throw _privateConstructorUsedError;
  Timestamp? get firstConsecutiveSeenTs => throw _privateConstructorUsedError;
  int get recentLostAnswers => throw _privateConstructorUsedError;
  int get failedToSend => throw _privateConstructorUsedError;

  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
  @JsonKey(ignore: true)
  $RPCStatsCopyWith<RPCStats> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $RPCStatsCopyWith<$Res> {
  factory $RPCStatsCopyWith(RPCStats value, $Res Function(RPCStats) then) =
      _$RPCStatsCopyWithImpl<$Res, RPCStats>;
  @useResult
  $Res call(
      {int messagesSent,
      int messagesRcvd,
      int questionsInFlight,
      Timestamp? lastQuestion,
      Timestamp? lastSeenTs,
      Timestamp? firstConsecutiveSeenTs,
      int recentLostAnswers,
      int failedToSend});
}

/// @nodoc
class _$RPCStatsCopyWithImpl<$Res, $Val extends RPCStats>
    implements $RPCStatsCopyWith<$Res> {
  _$RPCStatsCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? messagesSent = null,
    Object? messagesRcvd = null,
    Object? questionsInFlight = null,
    Object? lastQuestion = freezed,
    Object? lastSeenTs = freezed,
    Object? firstConsecutiveSeenTs = freezed,
    Object? recentLostAnswers = null,
    Object? failedToSend = null,
  }) {
    return _then(_value.copyWith(
      messagesSent: null == messagesSent
          ? _value.messagesSent
          : messagesSent // ignore: cast_nullable_to_non_nullable
              as int,
      messagesRcvd: null == messagesRcvd
          ? _value.messagesRcvd
          : messagesRcvd // ignore: cast_nullable_to_non_nullable
              as int,
      questionsInFlight: null == questionsInFlight
          ? _value.questionsInFlight
          : questionsInFlight // ignore: cast_nullable_to_non_nullable
              as int,
      lastQuestion: freezed == lastQuestion
          ? _value.lastQuestion
          : lastQuestion // ignore: cast_nullable_to_non_nullable
              as Timestamp?,
      lastSeenTs: freezed == lastSeenTs
          ? _value.lastSeenTs
          : lastSeenTs // ignore: cast_nullable_to_non_nullable
              as Timestamp?,
      firstConsecutiveSeenTs: freezed == firstConsecutiveSeenTs
          ? _value.firstConsecutiveSeenTs
          : firstConsecutiveSeenTs // ignore: cast_nullable_to_non_nullable
              as Timestamp?,
      recentLostAnswers: null == recentLostAnswers
          ? _value.recentLostAnswers
          : recentLostAnswers // ignore: cast_nullable_to_non_nullable
              as int,
      failedToSend: null == failedToSend
          ? _value.failedToSend
          : failedToSend // ignore: cast_nullable_to_non_nullable
              as int,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$_RPCStatsCopyWith<$Res> implements $RPCStatsCopyWith<$Res> {
  factory _$$_RPCStatsCopyWith(
          _$_RPCStats value, $Res Function(_$_RPCStats) then) =
      __$$_RPCStatsCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {int messagesSent,
      int messagesRcvd,
      int questionsInFlight,
      Timestamp? lastQuestion,
      Timestamp? lastSeenTs,
      Timestamp? firstConsecutiveSeenTs,
      int recentLostAnswers,
      int failedToSend});
}

/// @nodoc
class __$$_RPCStatsCopyWithImpl<$Res>
    extends _$RPCStatsCopyWithImpl<$Res, _$_RPCStats>
    implements _$$_RPCStatsCopyWith<$Res> {
  __$$_RPCStatsCopyWithImpl(
      _$_RPCStats _value, $Res Function(_$_RPCStats) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? messagesSent = null,
    Object? messagesRcvd = null,
    Object? questionsInFlight = null,
    Object? lastQuestion = freezed,
    Object? lastSeenTs = freezed,
    Object? firstConsecutiveSeenTs = freezed,
    Object? recentLostAnswers = null,
    Object? failedToSend = null,
  }) {
    return _then(_$_RPCStats(
      messagesSent: null == messagesSent
          ? _value.messagesSent
          : messagesSent // ignore: cast_nullable_to_non_nullable
              as int,
      messagesRcvd: null == messagesRcvd
          ? _value.messagesRcvd
          : messagesRcvd // ignore: cast_nullable_to_non_nullable
              as int,
      questionsInFlight: null == questionsInFlight
          ? _value.questionsInFlight
          : questionsInFlight // ignore: cast_nullable_to_non_nullable
              as int,
      lastQuestion: freezed == lastQuestion
          ? _value.lastQuestion
          : lastQuestion // ignore: cast_nullable_to_non_nullable
              as Timestamp?,
      lastSeenTs: freezed == lastSeenTs
          ? _value.lastSeenTs
          : lastSeenTs // ignore: cast_nullable_to_non_nullable
              as Timestamp?,
      firstConsecutiveSeenTs: freezed == firstConsecutiveSeenTs
          ? _value.firstConsecutiveSeenTs
          : firstConsecutiveSeenTs // ignore: cast_nullable_to_non_nullable
              as Timestamp?,
      recentLostAnswers: null == recentLostAnswers
          ? _value.recentLostAnswers
          : recentLostAnswers // ignore: cast_nullable_to_non_nullable
              as int,
      failedToSend: null == failedToSend
          ? _value.failedToSend
          : failedToSend // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$_RPCStats implements _RPCStats {
  const _$_RPCStats(
      {required this.messagesSent,
      required this.messagesRcvd,
      required this.questionsInFlight,
      required this.lastQuestion,
      required this.lastSeenTs,
      required this.firstConsecutiveSeenTs,
      required this.recentLostAnswers,
      required this.failedToSend});

  factory _$_RPCStats.fromJson(Map<String, dynamic> json) =>
      _$$_RPCStatsFromJson(json);

  @override
  final int messagesSent;
  @override
  final int messagesRcvd;
  @override
  final int questionsInFlight;
  @override
  final Timestamp? lastQuestion;
  @override
  final Timestamp? lastSeenTs;
  @override
  final Timestamp? firstConsecutiveSeenTs;
  @override
  final int recentLostAnswers;
  @override
  final int failedToSend;

  @override
  String toString() {
    return 'RPCStats(messagesSent: $messagesSent, messagesRcvd: $messagesRcvd, questionsInFlight: $questionsInFlight, lastQuestion: $lastQuestion, lastSeenTs: $lastSeenTs, firstConsecutiveSeenTs: $firstConsecutiveSeenTs, recentLostAnswers: $recentLostAnswers, failedToSend: $failedToSend)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$_RPCStats &&
            (identical(other.messagesSent, messagesSent) ||
                other.messagesSent == messagesSent) &&
            (identical(other.messagesRcvd, messagesRcvd) ||
                other.messagesRcvd == messagesRcvd) &&
            (identical(other.questionsInFlight, questionsInFlight) ||
                other.questionsInFlight == questionsInFlight) &&
            (identical(other.lastQuestion, lastQuestion) ||
                other.lastQuestion == lastQuestion) &&
            (identical(other.lastSeenTs, lastSeenTs) ||
                other.lastSeenTs == lastSeenTs) &&
            (identical(other.firstConsecutiveSeenTs, firstConsecutiveSeenTs) ||
                other.firstConsecutiveSeenTs == firstConsecutiveSeenTs) &&
            (identical(other.recentLostAnswers, recentLostAnswers) ||
                other.recentLostAnswers == recentLostAnswers) &&
            (identical(other.failedToSend, failedToSend) ||
                other.failedToSend == failedToSend));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(
      runtimeType,
      messagesSent,
      messagesRcvd,
      questionsInFlight,
      lastQuestion,
      lastSeenTs,
      firstConsecutiveSeenTs,
      recentLostAnswers,
      failedToSend);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$_RPCStatsCopyWith<_$_RPCStats> get copyWith =>
      __$$_RPCStatsCopyWithImpl<_$_RPCStats>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$_RPCStatsToJson(
      this,
    );
  }
}

abstract class _RPCStats implements RPCStats {
  const factory _RPCStats(
      {required final int messagesSent,
      required final int messagesRcvd,
      required final int questionsInFlight,
      required final Timestamp? lastQuestion,
      required final Timestamp? lastSeenTs,
      required final Timestamp? firstConsecutiveSeenTs,
      required final int recentLostAnswers,
      required final int failedToSend}) = _$_RPCStats;

  factory _RPCStats.fromJson(Map<String, dynamic> json) = _$_RPCStats.fromJson;

  @override
  int get messagesSent;
  @override
  int get messagesRcvd;
  @override
  int get questionsInFlight;
  @override
  Timestamp? get lastQuestion;
  @override
  Timestamp? get lastSeenTs;
  @override
  Timestamp? get firstConsecutiveSeenTs;
  @override
  int get recentLostAnswers;
  @override
  int get failedToSend;
  @override
  @JsonKey(ignore: true)
  _$$_RPCStatsCopyWith<_$_RPCStats> get copyWith =>
      throw _privateConstructorUsedError;
}

PeerStats _$PeerStatsFromJson(Map<String, dynamic> json) {
  return _PeerStats.fromJson(json);
}

/// @nodoc
mixin _$PeerStats {
  Timestamp get timeAdded => throw _privateConstructorUsedError;
  RPCStats get rpcStats => throw _privateConstructorUsedError;
  LatencyStats? get latency => throw _privateConstructorUsedError;
  TransferStatsDownUp get transfer => throw _privateConstructorUsedError;

  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
  @JsonKey(ignore: true)
  $PeerStatsCopyWith<PeerStats> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $PeerStatsCopyWith<$Res> {
  factory $PeerStatsCopyWith(PeerStats value, $Res Function(PeerStats) then) =
      _$PeerStatsCopyWithImpl<$Res, PeerStats>;
  @useResult
  $Res call(
      {Timestamp timeAdded,
      RPCStats rpcStats,
      LatencyStats? latency,
      TransferStatsDownUp transfer});

  $RPCStatsCopyWith<$Res> get rpcStats;
  $LatencyStatsCopyWith<$Res>? get latency;
  $TransferStatsDownUpCopyWith<$Res> get transfer;
}

/// @nodoc
class _$PeerStatsCopyWithImpl<$Res, $Val extends PeerStats>
    implements $PeerStatsCopyWith<$Res> {
  _$PeerStatsCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? timeAdded = null,
    Object? rpcStats = null,
    Object? latency = freezed,
    Object? transfer = null,
  }) {
    return _then(_value.copyWith(
      timeAdded: null == timeAdded
          ? _value.timeAdded
          : timeAdded // ignore: cast_nullable_to_non_nullable
              as Timestamp,
      rpcStats: null == rpcStats
          ? _value.rpcStats
          : rpcStats // ignore: cast_nullable_to_non_nullable
              as RPCStats,
      latency: freezed == latency
          ? _value.latency
          : latency // ignore: cast_nullable_to_non_nullable
              as LatencyStats?,
      transfer: null == transfer
          ? _value.transfer
          : transfer // ignore: cast_nullable_to_non_nullable
              as TransferStatsDownUp,
    ) as $Val);
  }

  @override
  @pragma('vm:prefer-inline')
  $RPCStatsCopyWith<$Res> get rpcStats {
    return $RPCStatsCopyWith<$Res>(_value.rpcStats, (value) {
      return _then(_value.copyWith(rpcStats: value) as $Val);
    });
  }

  @override
  @pragma('vm:prefer-inline')
  $LatencyStatsCopyWith<$Res>? get latency {
    if (_value.latency == null) {
      return null;
    }

    return $LatencyStatsCopyWith<$Res>(_value.latency!, (value) {
      return _then(_value.copyWith(latency: value) as $Val);
    });
  }

  @override
  @pragma('vm:prefer-inline')
  $TransferStatsDownUpCopyWith<$Res> get transfer {
    return $TransferStatsDownUpCopyWith<$Res>(_value.transfer, (value) {
      return _then(_value.copyWith(transfer: value) as $Val);
    });
  }
}

/// @nodoc
abstract class _$$_PeerStatsCopyWith<$Res> implements $PeerStatsCopyWith<$Res> {
  factory _$$_PeerStatsCopyWith(
          _$_PeerStats value, $Res Function(_$_PeerStats) then) =
      __$$_PeerStatsCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {Timestamp timeAdded,
      RPCStats rpcStats,
      LatencyStats? latency,
      TransferStatsDownUp transfer});

  @override
  $RPCStatsCopyWith<$Res> get rpcStats;
  @override
  $LatencyStatsCopyWith<$Res>? get latency;
  @override
  $TransferStatsDownUpCopyWith<$Res> get transfer;
}

/// @nodoc
class __$$_PeerStatsCopyWithImpl<$Res>
    extends _$PeerStatsCopyWithImpl<$Res, _$_PeerStats>
    implements _$$_PeerStatsCopyWith<$Res> {
  __$$_PeerStatsCopyWithImpl(
      _$_PeerStats _value, $Res Function(_$_PeerStats) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? timeAdded = null,
    Object? rpcStats = null,
    Object? latency = freezed,
    Object? transfer = null,
  }) {
    return _then(_$_PeerStats(
      timeAdded: null == timeAdded
          ? _value.timeAdded
          : timeAdded // ignore: cast_nullable_to_non_nullable
              as Timestamp,
      rpcStats: null == rpcStats
          ? _value.rpcStats
          : rpcStats // ignore: cast_nullable_to_non_nullable
              as RPCStats,
      latency: freezed == latency
          ? _value.latency
          : latency // ignore: cast_nullable_to_non_nullable
              as LatencyStats?,
      transfer: null == transfer
          ? _value.transfer
          : transfer // ignore: cast_nullable_to_non_nullable
              as TransferStatsDownUp,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$_PeerStats implements _PeerStats {
  const _$_PeerStats(
      {required this.timeAdded,
      required this.rpcStats,
      this.latency,
      required this.transfer});

  factory _$_PeerStats.fromJson(Map<String, dynamic> json) =>
      _$$_PeerStatsFromJson(json);

  @override
  final Timestamp timeAdded;
  @override
  final RPCStats rpcStats;
  @override
  final LatencyStats? latency;
  @override
  final TransferStatsDownUp transfer;

  @override
  String toString() {
    return 'PeerStats(timeAdded: $timeAdded, rpcStats: $rpcStats, latency: $latency, transfer: $transfer)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$_PeerStats &&
            (identical(other.timeAdded, timeAdded) ||
                other.timeAdded == timeAdded) &&
            (identical(other.rpcStats, rpcStats) ||
                other.rpcStats == rpcStats) &&
            (identical(other.latency, latency) || other.latency == latency) &&
            (identical(other.transfer, transfer) ||
                other.transfer == transfer));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode =>
      Object.hash(runtimeType, timeAdded, rpcStats, latency, transfer);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$_PeerStatsCopyWith<_$_PeerStats> get copyWith =>
      __$$_PeerStatsCopyWithImpl<_$_PeerStats>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$_PeerStatsToJson(
      this,
    );
  }
}

abstract class _PeerStats implements PeerStats {
  const factory _PeerStats(
      {required final Timestamp timeAdded,
      required final RPCStats rpcStats,
      final LatencyStats? latency,
      required final TransferStatsDownUp transfer}) = _$_PeerStats;

  factory _PeerStats.fromJson(Map<String, dynamic> json) =
      _$_PeerStats.fromJson;

  @override
  Timestamp get timeAdded;
  @override
  RPCStats get rpcStats;
  @override
  LatencyStats? get latency;
  @override
  TransferStatsDownUp get transfer;
  @override
  @JsonKey(ignore: true)
  _$$_PeerStatsCopyWith<_$_PeerStats> get copyWith =>
      throw _privateConstructorUsedError;
}

PeerTableData _$PeerTableDataFromJson(Map<String, dynamic> json) {
  return _PeerTableData.fromJson(json);
}

/// @nodoc
mixin _$PeerTableData {
  List<Typed<FixedEncodedString43>> get nodeIds =>
      throw _privateConstructorUsedError;
  String get peerAddress => throw _privateConstructorUsedError;
  PeerStats get peerStats => throw _privateConstructorUsedError;

  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
  @JsonKey(ignore: true)
  $PeerTableDataCopyWith<PeerTableData> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $PeerTableDataCopyWith<$Res> {
  factory $PeerTableDataCopyWith(
          PeerTableData value, $Res Function(PeerTableData) then) =
      _$PeerTableDataCopyWithImpl<$Res, PeerTableData>;
  @useResult
  $Res call(
      {List<Typed<FixedEncodedString43>> nodeIds,
      String peerAddress,
      PeerStats peerStats});

  $PeerStatsCopyWith<$Res> get peerStats;
}

/// @nodoc
class _$PeerTableDataCopyWithImpl<$Res, $Val extends PeerTableData>
    implements $PeerTableDataCopyWith<$Res> {
  _$PeerTableDataCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? nodeIds = null,
    Object? peerAddress = null,
    Object? peerStats = null,
  }) {
    return _then(_value.copyWith(
      nodeIds: null == nodeIds
          ? _value.nodeIds
          : nodeIds // ignore: cast_nullable_to_non_nullable
              as List<Typed<FixedEncodedString43>>,
      peerAddress: null == peerAddress
          ? _value.peerAddress
          : peerAddress // ignore: cast_nullable_to_non_nullable
              as String,
      peerStats: null == peerStats
          ? _value.peerStats
          : peerStats // ignore: cast_nullable_to_non_nullable
              as PeerStats,
    ) as $Val);
  }

  @override
  @pragma('vm:prefer-inline')
  $PeerStatsCopyWith<$Res> get peerStats {
    return $PeerStatsCopyWith<$Res>(_value.peerStats, (value) {
      return _then(_value.copyWith(peerStats: value) as $Val);
    });
  }
}

/// @nodoc
abstract class _$$_PeerTableDataCopyWith<$Res>
    implements $PeerTableDataCopyWith<$Res> {
  factory _$$_PeerTableDataCopyWith(
          _$_PeerTableData value, $Res Function(_$_PeerTableData) then) =
      __$$_PeerTableDataCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {List<Typed<FixedEncodedString43>> nodeIds,
      String peerAddress,
      PeerStats peerStats});

  @override
  $PeerStatsCopyWith<$Res> get peerStats;
}

/// @nodoc
class __$$_PeerTableDataCopyWithImpl<$Res>
    extends _$PeerTableDataCopyWithImpl<$Res, _$_PeerTableData>
    implements _$$_PeerTableDataCopyWith<$Res> {
  __$$_PeerTableDataCopyWithImpl(
      _$_PeerTableData _value, $Res Function(_$_PeerTableData) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? nodeIds = null,
    Object? peerAddress = null,
    Object? peerStats = null,
  }) {
    return _then(_$_PeerTableData(
      nodeIds: null == nodeIds
          ? _value._nodeIds
          : nodeIds // ignore: cast_nullable_to_non_nullable
              as List<Typed<FixedEncodedString43>>,
      peerAddress: null == peerAddress
          ? _value.peerAddress
          : peerAddress // ignore: cast_nullable_to_non_nullable
              as String,
      peerStats: null == peerStats
          ? _value.peerStats
          : peerStats // ignore: cast_nullable_to_non_nullable
              as PeerStats,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$_PeerTableData implements _PeerTableData {
  const _$_PeerTableData(
      {required final List<Typed<FixedEncodedString43>> nodeIds,
      required this.peerAddress,
      required this.peerStats})
      : _nodeIds = nodeIds;

  factory _$_PeerTableData.fromJson(Map<String, dynamic> json) =>
      _$$_PeerTableDataFromJson(json);

  final List<Typed<FixedEncodedString43>> _nodeIds;
  @override
  List<Typed<FixedEncodedString43>> get nodeIds {
    if (_nodeIds is EqualUnmodifiableListView) return _nodeIds;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_nodeIds);
  }

  @override
  final String peerAddress;
  @override
  final PeerStats peerStats;

  @override
  String toString() {
    return 'PeerTableData(nodeIds: $nodeIds, peerAddress: $peerAddress, peerStats: $peerStats)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$_PeerTableData &&
            const DeepCollectionEquality().equals(other._nodeIds, _nodeIds) &&
            (identical(other.peerAddress, peerAddress) ||
                other.peerAddress == peerAddress) &&
            (identical(other.peerStats, peerStats) ||
                other.peerStats == peerStats));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType,
      const DeepCollectionEquality().hash(_nodeIds), peerAddress, peerStats);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$_PeerTableDataCopyWith<_$_PeerTableData> get copyWith =>
      __$$_PeerTableDataCopyWithImpl<_$_PeerTableData>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$_PeerTableDataToJson(
      this,
    );
  }
}

abstract class _PeerTableData implements PeerTableData {
  const factory _PeerTableData(
      {required final List<Typed<FixedEncodedString43>> nodeIds,
      required final String peerAddress,
      required final PeerStats peerStats}) = _$_PeerTableData;

  factory _PeerTableData.fromJson(Map<String, dynamic> json) =
      _$_PeerTableData.fromJson;

  @override
  List<Typed<FixedEncodedString43>> get nodeIds;
  @override
  String get peerAddress;
  @override
  PeerStats get peerStats;
  @override
  @JsonKey(ignore: true)
  _$$_PeerTableDataCopyWith<_$_PeerTableData> get copyWith =>
      throw _privateConstructorUsedError;
}
