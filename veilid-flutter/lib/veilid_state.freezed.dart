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
  TransferStatsDownUp get transfer => throw _privateConstructorUsedError;
  LatencyStats? get latency => throw _privateConstructorUsedError;

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
      TransferStatsDownUp transfer,
      LatencyStats? latency});

  $RPCStatsCopyWith<$Res> get rpcStats;
  $TransferStatsDownUpCopyWith<$Res> get transfer;
  $LatencyStatsCopyWith<$Res>? get latency;
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
    Object? transfer = null,
    Object? latency = freezed,
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
      transfer: null == transfer
          ? _value.transfer
          : transfer // ignore: cast_nullable_to_non_nullable
              as TransferStatsDownUp,
      latency: freezed == latency
          ? _value.latency
          : latency // ignore: cast_nullable_to_non_nullable
              as LatencyStats?,
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
  $TransferStatsDownUpCopyWith<$Res> get transfer {
    return $TransferStatsDownUpCopyWith<$Res>(_value.transfer, (value) {
      return _then(_value.copyWith(transfer: value) as $Val);
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
      TransferStatsDownUp transfer,
      LatencyStats? latency});

  @override
  $RPCStatsCopyWith<$Res> get rpcStats;
  @override
  $TransferStatsDownUpCopyWith<$Res> get transfer;
  @override
  $LatencyStatsCopyWith<$Res>? get latency;
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
    Object? transfer = null,
    Object? latency = freezed,
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
      transfer: null == transfer
          ? _value.transfer
          : transfer // ignore: cast_nullable_to_non_nullable
              as TransferStatsDownUp,
      latency: freezed == latency
          ? _value.latency
          : latency // ignore: cast_nullable_to_non_nullable
              as LatencyStats?,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$_PeerStats implements _PeerStats {
  const _$_PeerStats(
      {required this.timeAdded,
      required this.rpcStats,
      required this.transfer,
      this.latency});

  factory _$_PeerStats.fromJson(Map<String, dynamic> json) =>
      _$$_PeerStatsFromJson(json);

  @override
  final Timestamp timeAdded;
  @override
  final RPCStats rpcStats;
  @override
  final TransferStatsDownUp transfer;
  @override
  final LatencyStats? latency;

  @override
  String toString() {
    return 'PeerStats(timeAdded: $timeAdded, rpcStats: $rpcStats, transfer: $transfer, latency: $latency)';
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
            (identical(other.transfer, transfer) ||
                other.transfer == transfer) &&
            (identical(other.latency, latency) || other.latency == latency));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode =>
      Object.hash(runtimeType, timeAdded, rpcStats, transfer, latency);

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
      required final TransferStatsDownUp transfer,
      final LatencyStats? latency}) = _$_PeerStats;

  factory _PeerStats.fromJson(Map<String, dynamic> json) =
      _$_PeerStats.fromJson;

  @override
  Timestamp get timeAdded;
  @override
  RPCStats get rpcStats;
  @override
  TransferStatsDownUp get transfer;
  @override
  LatencyStats? get latency;
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
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(
            VeilidLogLevel logLevel, String message, String? backtrace)
        log,
    required TResult Function(@Uint8ListJsonConverter() Uint8List message,
            Typed<FixedEncodedString43>? sender)
        appMessage,
    required TResult Function(@Uint8ListJsonConverter() Uint8List message,
            String callId, Typed<FixedEncodedString43>? sender)
        appCall,
    required TResult Function(AttachmentState state, bool publicInternetReady,
            bool localNetworkReady)
        attachment,
    required TResult Function(bool started, BigInt bpsDown, BigInt bpsUp,
            List<PeerTableData> peers)
        network,
    required TResult Function(VeilidConfig config) config,
    required TResult Function(
            List<String> deadRoutes, List<String> deadRemoteRoutes)
        routeChange,
    required TResult Function(Typed<FixedEncodedString43> key,
            List<ValueSubkeyRange> subkeys, int count, ValueData value)
        valueChange,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(
            VeilidLogLevel logLevel, String message, String? backtrace)?
        log,
    TResult? Function(@Uint8ListJsonConverter() Uint8List message,
            Typed<FixedEncodedString43>? sender)?
        appMessage,
    TResult? Function(@Uint8ListJsonConverter() Uint8List message,
            String callId, Typed<FixedEncodedString43>? sender)?
        appCall,
    TResult? Function(AttachmentState state, bool publicInternetReady,
            bool localNetworkReady)?
        attachment,
    TResult? Function(bool started, BigInt bpsDown, BigInt bpsUp,
            List<PeerTableData> peers)?
        network,
    TResult? Function(VeilidConfig config)? config,
    TResult? Function(List<String> deadRoutes, List<String> deadRemoteRoutes)?
        routeChange,
    TResult? Function(Typed<FixedEncodedString43> key,
            List<ValueSubkeyRange> subkeys, int count, ValueData value)?
        valueChange,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(
            VeilidLogLevel logLevel, String message, String? backtrace)?
        log,
    TResult Function(@Uint8ListJsonConverter() Uint8List message,
            Typed<FixedEncodedString43>? sender)?
        appMessage,
    TResult Function(@Uint8ListJsonConverter() Uint8List message, String callId,
            Typed<FixedEncodedString43>? sender)?
        appCall,
    TResult Function(AttachmentState state, bool publicInternetReady,
            bool localNetworkReady)?
        attachment,
    TResult Function(bool started, BigInt bpsDown, BigInt bpsUp,
            List<PeerTableData> peers)?
        network,
    TResult Function(VeilidConfig config)? config,
    TResult Function(List<String> deadRoutes, List<String> deadRemoteRoutes)?
        routeChange,
    TResult Function(Typed<FixedEncodedString43> key,
            List<ValueSubkeyRange> subkeys, int count, ValueData value)?
        valueChange,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(VeilidLog value) log,
    required TResult Function(VeilidAppMessage value) appMessage,
    required TResult Function(VeilidAppCall value) appCall,
    required TResult Function(VeilidUpdateAttachment value) attachment,
    required TResult Function(VeilidUpdateNetwork value) network,
    required TResult Function(VeilidUpdateConfig value) config,
    required TResult Function(VeilidUpdateRouteChange value) routeChange,
    required TResult Function(VeilidUpdateValueChange value) valueChange,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(VeilidLog value)? log,
    TResult? Function(VeilidAppMessage value)? appMessage,
    TResult? Function(VeilidAppCall value)? appCall,
    TResult? Function(VeilidUpdateAttachment value)? attachment,
    TResult? Function(VeilidUpdateNetwork value)? network,
    TResult? Function(VeilidUpdateConfig value)? config,
    TResult? Function(VeilidUpdateRouteChange value)? routeChange,
    TResult? Function(VeilidUpdateValueChange value)? valueChange,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(VeilidLog value)? log,
    TResult Function(VeilidAppMessage value)? appMessage,
    TResult Function(VeilidAppCall value)? appCall,
    TResult Function(VeilidUpdateAttachment value)? attachment,
    TResult Function(VeilidUpdateNetwork value)? network,
    TResult Function(VeilidUpdateConfig value)? config,
    TResult Function(VeilidUpdateRouteChange value)? routeChange,
    TResult Function(VeilidUpdateValueChange value)? valueChange,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $VeilidUpdateCopyWith<$Res> {
  factory $VeilidUpdateCopyWith(
          VeilidUpdate value, $Res Function(VeilidUpdate) then) =
      _$VeilidUpdateCopyWithImpl<$Res, VeilidUpdate>;
}

/// @nodoc
class _$VeilidUpdateCopyWithImpl<$Res, $Val extends VeilidUpdate>
    implements $VeilidUpdateCopyWith<$Res> {
  _$VeilidUpdateCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$VeilidLogCopyWith<$Res> {
  factory _$$VeilidLogCopyWith(
          _$VeilidLog value, $Res Function(_$VeilidLog) then) =
      __$$VeilidLogCopyWithImpl<$Res>;
  @useResult
  $Res call({VeilidLogLevel logLevel, String message, String? backtrace});
}

/// @nodoc
class __$$VeilidLogCopyWithImpl<$Res>
    extends _$VeilidUpdateCopyWithImpl<$Res, _$VeilidLog>
    implements _$$VeilidLogCopyWith<$Res> {
  __$$VeilidLogCopyWithImpl(
      _$VeilidLog _value, $Res Function(_$VeilidLog) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? logLevel = null,
    Object? message = null,
    Object? backtrace = freezed,
  }) {
    return _then(_$VeilidLog(
      logLevel: null == logLevel
          ? _value.logLevel
          : logLevel // ignore: cast_nullable_to_non_nullable
              as VeilidLogLevel,
      message: null == message
          ? _value.message
          : message // ignore: cast_nullable_to_non_nullable
              as String,
      backtrace: freezed == backtrace
          ? _value.backtrace
          : backtrace // ignore: cast_nullable_to_non_nullable
              as String?,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$VeilidLog implements VeilidLog {
  const _$VeilidLog(
      {required this.logLevel,
      required this.message,
      this.backtrace,
      final String? $type})
      : $type = $type ?? 'Log';

  factory _$VeilidLog.fromJson(Map<String, dynamic> json) =>
      _$$VeilidLogFromJson(json);

  @override
  final VeilidLogLevel logLevel;
  @override
  final String message;
  @override
  final String? backtrace;

  @JsonKey(name: 'kind')
  final String $type;

  @override
  String toString() {
    return 'VeilidUpdate.log(logLevel: $logLevel, message: $message, backtrace: $backtrace)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$VeilidLog &&
            (identical(other.logLevel, logLevel) ||
                other.logLevel == logLevel) &&
            (identical(other.message, message) || other.message == message) &&
            (identical(other.backtrace, backtrace) ||
                other.backtrace == backtrace));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, logLevel, message, backtrace);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$VeilidLogCopyWith<_$VeilidLog> get copyWith =>
      __$$VeilidLogCopyWithImpl<_$VeilidLog>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(
            VeilidLogLevel logLevel, String message, String? backtrace)
        log,
    required TResult Function(@Uint8ListJsonConverter() Uint8List message,
            Typed<FixedEncodedString43>? sender)
        appMessage,
    required TResult Function(@Uint8ListJsonConverter() Uint8List message,
            String callId, Typed<FixedEncodedString43>? sender)
        appCall,
    required TResult Function(AttachmentState state, bool publicInternetReady,
            bool localNetworkReady)
        attachment,
    required TResult Function(bool started, BigInt bpsDown, BigInt bpsUp,
            List<PeerTableData> peers)
        network,
    required TResult Function(VeilidConfig config) config,
    required TResult Function(
            List<String> deadRoutes, List<String> deadRemoteRoutes)
        routeChange,
    required TResult Function(Typed<FixedEncodedString43> key,
            List<ValueSubkeyRange> subkeys, int count, ValueData value)
        valueChange,
  }) {
    return log(logLevel, message, backtrace);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(
            VeilidLogLevel logLevel, String message, String? backtrace)?
        log,
    TResult? Function(@Uint8ListJsonConverter() Uint8List message,
            Typed<FixedEncodedString43>? sender)?
        appMessage,
    TResult? Function(@Uint8ListJsonConverter() Uint8List message,
            String callId, Typed<FixedEncodedString43>? sender)?
        appCall,
    TResult? Function(AttachmentState state, bool publicInternetReady,
            bool localNetworkReady)?
        attachment,
    TResult? Function(bool started, BigInt bpsDown, BigInt bpsUp,
            List<PeerTableData> peers)?
        network,
    TResult? Function(VeilidConfig config)? config,
    TResult? Function(List<String> deadRoutes, List<String> deadRemoteRoutes)?
        routeChange,
    TResult? Function(Typed<FixedEncodedString43> key,
            List<ValueSubkeyRange> subkeys, int count, ValueData value)?
        valueChange,
  }) {
    return log?.call(logLevel, message, backtrace);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(
            VeilidLogLevel logLevel, String message, String? backtrace)?
        log,
    TResult Function(@Uint8ListJsonConverter() Uint8List message,
            Typed<FixedEncodedString43>? sender)?
        appMessage,
    TResult Function(@Uint8ListJsonConverter() Uint8List message, String callId,
            Typed<FixedEncodedString43>? sender)?
        appCall,
    TResult Function(AttachmentState state, bool publicInternetReady,
            bool localNetworkReady)?
        attachment,
    TResult Function(bool started, BigInt bpsDown, BigInt bpsUp,
            List<PeerTableData> peers)?
        network,
    TResult Function(VeilidConfig config)? config,
    TResult Function(List<String> deadRoutes, List<String> deadRemoteRoutes)?
        routeChange,
    TResult Function(Typed<FixedEncodedString43> key,
            List<ValueSubkeyRange> subkeys, int count, ValueData value)?
        valueChange,
    required TResult orElse(),
  }) {
    if (log != null) {
      return log(logLevel, message, backtrace);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(VeilidLog value) log,
    required TResult Function(VeilidAppMessage value) appMessage,
    required TResult Function(VeilidAppCall value) appCall,
    required TResult Function(VeilidUpdateAttachment value) attachment,
    required TResult Function(VeilidUpdateNetwork value) network,
    required TResult Function(VeilidUpdateConfig value) config,
    required TResult Function(VeilidUpdateRouteChange value) routeChange,
    required TResult Function(VeilidUpdateValueChange value) valueChange,
  }) {
    return log(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(VeilidLog value)? log,
    TResult? Function(VeilidAppMessage value)? appMessage,
    TResult? Function(VeilidAppCall value)? appCall,
    TResult? Function(VeilidUpdateAttachment value)? attachment,
    TResult? Function(VeilidUpdateNetwork value)? network,
    TResult? Function(VeilidUpdateConfig value)? config,
    TResult? Function(VeilidUpdateRouteChange value)? routeChange,
    TResult? Function(VeilidUpdateValueChange value)? valueChange,
  }) {
    return log?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(VeilidLog value)? log,
    TResult Function(VeilidAppMessage value)? appMessage,
    TResult Function(VeilidAppCall value)? appCall,
    TResult Function(VeilidUpdateAttachment value)? attachment,
    TResult Function(VeilidUpdateNetwork value)? network,
    TResult Function(VeilidUpdateConfig value)? config,
    TResult Function(VeilidUpdateRouteChange value)? routeChange,
    TResult Function(VeilidUpdateValueChange value)? valueChange,
    required TResult orElse(),
  }) {
    if (log != null) {
      return log(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$VeilidLogToJson(
      this,
    );
  }
}

abstract class VeilidLog implements VeilidUpdate {
  const factory VeilidLog(
      {required final VeilidLogLevel logLevel,
      required final String message,
      final String? backtrace}) = _$VeilidLog;

  factory VeilidLog.fromJson(Map<String, dynamic> json) = _$VeilidLog.fromJson;

  VeilidLogLevel get logLevel;
  String get message;
  String? get backtrace;
  @JsonKey(ignore: true)
  _$$VeilidLogCopyWith<_$VeilidLog> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$VeilidAppMessageCopyWith<$Res> {
  factory _$$VeilidAppMessageCopyWith(
          _$VeilidAppMessage value, $Res Function(_$VeilidAppMessage) then) =
      __$$VeilidAppMessageCopyWithImpl<$Res>;
  @useResult
  $Res call(
      {@Uint8ListJsonConverter() Uint8List message,
      Typed<FixedEncodedString43>? sender});
}

/// @nodoc
class __$$VeilidAppMessageCopyWithImpl<$Res>
    extends _$VeilidUpdateCopyWithImpl<$Res, _$VeilidAppMessage>
    implements _$$VeilidAppMessageCopyWith<$Res> {
  __$$VeilidAppMessageCopyWithImpl(
      _$VeilidAppMessage _value, $Res Function(_$VeilidAppMessage) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? message = null,
    Object? sender = freezed,
  }) {
    return _then(_$VeilidAppMessage(
      message: null == message
          ? _value.message
          : message // ignore: cast_nullable_to_non_nullable
              as Uint8List,
      sender: freezed == sender
          ? _value.sender
          : sender // ignore: cast_nullable_to_non_nullable
              as Typed<FixedEncodedString43>?,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$VeilidAppMessage implements VeilidAppMessage {
  const _$VeilidAppMessage(
      {@Uint8ListJsonConverter() required this.message,
      this.sender,
      final String? $type})
      : $type = $type ?? 'AppMessage';

  factory _$VeilidAppMessage.fromJson(Map<String, dynamic> json) =>
      _$$VeilidAppMessageFromJson(json);

  @override
  @Uint8ListJsonConverter()
  final Uint8List message;
  @override
  final Typed<FixedEncodedString43>? sender;

  @JsonKey(name: 'kind')
  final String $type;

  @override
  String toString() {
    return 'VeilidUpdate.appMessage(message: $message, sender: $sender)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$VeilidAppMessage &&
            const DeepCollectionEquality().equals(other.message, message) &&
            (identical(other.sender, sender) || other.sender == sender));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(
      runtimeType, const DeepCollectionEquality().hash(message), sender);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$VeilidAppMessageCopyWith<_$VeilidAppMessage> get copyWith =>
      __$$VeilidAppMessageCopyWithImpl<_$VeilidAppMessage>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(
            VeilidLogLevel logLevel, String message, String? backtrace)
        log,
    required TResult Function(@Uint8ListJsonConverter() Uint8List message,
            Typed<FixedEncodedString43>? sender)
        appMessage,
    required TResult Function(@Uint8ListJsonConverter() Uint8List message,
            String callId, Typed<FixedEncodedString43>? sender)
        appCall,
    required TResult Function(AttachmentState state, bool publicInternetReady,
            bool localNetworkReady)
        attachment,
    required TResult Function(bool started, BigInt bpsDown, BigInt bpsUp,
            List<PeerTableData> peers)
        network,
    required TResult Function(VeilidConfig config) config,
    required TResult Function(
            List<String> deadRoutes, List<String> deadRemoteRoutes)
        routeChange,
    required TResult Function(Typed<FixedEncodedString43> key,
            List<ValueSubkeyRange> subkeys, int count, ValueData value)
        valueChange,
  }) {
    return appMessage(message, sender);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(
            VeilidLogLevel logLevel, String message, String? backtrace)?
        log,
    TResult? Function(@Uint8ListJsonConverter() Uint8List message,
            Typed<FixedEncodedString43>? sender)?
        appMessage,
    TResult? Function(@Uint8ListJsonConverter() Uint8List message,
            String callId, Typed<FixedEncodedString43>? sender)?
        appCall,
    TResult? Function(AttachmentState state, bool publicInternetReady,
            bool localNetworkReady)?
        attachment,
    TResult? Function(bool started, BigInt bpsDown, BigInt bpsUp,
            List<PeerTableData> peers)?
        network,
    TResult? Function(VeilidConfig config)? config,
    TResult? Function(List<String> deadRoutes, List<String> deadRemoteRoutes)?
        routeChange,
    TResult? Function(Typed<FixedEncodedString43> key,
            List<ValueSubkeyRange> subkeys, int count, ValueData value)?
        valueChange,
  }) {
    return appMessage?.call(message, sender);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(
            VeilidLogLevel logLevel, String message, String? backtrace)?
        log,
    TResult Function(@Uint8ListJsonConverter() Uint8List message,
            Typed<FixedEncodedString43>? sender)?
        appMessage,
    TResult Function(@Uint8ListJsonConverter() Uint8List message, String callId,
            Typed<FixedEncodedString43>? sender)?
        appCall,
    TResult Function(AttachmentState state, bool publicInternetReady,
            bool localNetworkReady)?
        attachment,
    TResult Function(bool started, BigInt bpsDown, BigInt bpsUp,
            List<PeerTableData> peers)?
        network,
    TResult Function(VeilidConfig config)? config,
    TResult Function(List<String> deadRoutes, List<String> deadRemoteRoutes)?
        routeChange,
    TResult Function(Typed<FixedEncodedString43> key,
            List<ValueSubkeyRange> subkeys, int count, ValueData value)?
        valueChange,
    required TResult orElse(),
  }) {
    if (appMessage != null) {
      return appMessage(message, sender);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(VeilidLog value) log,
    required TResult Function(VeilidAppMessage value) appMessage,
    required TResult Function(VeilidAppCall value) appCall,
    required TResult Function(VeilidUpdateAttachment value) attachment,
    required TResult Function(VeilidUpdateNetwork value) network,
    required TResult Function(VeilidUpdateConfig value) config,
    required TResult Function(VeilidUpdateRouteChange value) routeChange,
    required TResult Function(VeilidUpdateValueChange value) valueChange,
  }) {
    return appMessage(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(VeilidLog value)? log,
    TResult? Function(VeilidAppMessage value)? appMessage,
    TResult? Function(VeilidAppCall value)? appCall,
    TResult? Function(VeilidUpdateAttachment value)? attachment,
    TResult? Function(VeilidUpdateNetwork value)? network,
    TResult? Function(VeilidUpdateConfig value)? config,
    TResult? Function(VeilidUpdateRouteChange value)? routeChange,
    TResult? Function(VeilidUpdateValueChange value)? valueChange,
  }) {
    return appMessage?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(VeilidLog value)? log,
    TResult Function(VeilidAppMessage value)? appMessage,
    TResult Function(VeilidAppCall value)? appCall,
    TResult Function(VeilidUpdateAttachment value)? attachment,
    TResult Function(VeilidUpdateNetwork value)? network,
    TResult Function(VeilidUpdateConfig value)? config,
    TResult Function(VeilidUpdateRouteChange value)? routeChange,
    TResult Function(VeilidUpdateValueChange value)? valueChange,
    required TResult orElse(),
  }) {
    if (appMessage != null) {
      return appMessage(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$VeilidAppMessageToJson(
      this,
    );
  }
}

abstract class VeilidAppMessage implements VeilidUpdate {
  const factory VeilidAppMessage(
      {@Uint8ListJsonConverter() required final Uint8List message,
      final Typed<FixedEncodedString43>? sender}) = _$VeilidAppMessage;

  factory VeilidAppMessage.fromJson(Map<String, dynamic> json) =
      _$VeilidAppMessage.fromJson;

  @Uint8ListJsonConverter()
  Uint8List get message;
  Typed<FixedEncodedString43>? get sender;
  @JsonKey(ignore: true)
  _$$VeilidAppMessageCopyWith<_$VeilidAppMessage> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$VeilidAppCallCopyWith<$Res> {
  factory _$$VeilidAppCallCopyWith(
          _$VeilidAppCall value, $Res Function(_$VeilidAppCall) then) =
      __$$VeilidAppCallCopyWithImpl<$Res>;
  @useResult
  $Res call(
      {@Uint8ListJsonConverter() Uint8List message,
      String callId,
      Typed<FixedEncodedString43>? sender});
}

/// @nodoc
class __$$VeilidAppCallCopyWithImpl<$Res>
    extends _$VeilidUpdateCopyWithImpl<$Res, _$VeilidAppCall>
    implements _$$VeilidAppCallCopyWith<$Res> {
  __$$VeilidAppCallCopyWithImpl(
      _$VeilidAppCall _value, $Res Function(_$VeilidAppCall) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? message = null,
    Object? callId = null,
    Object? sender = freezed,
  }) {
    return _then(_$VeilidAppCall(
      message: null == message
          ? _value.message
          : message // ignore: cast_nullable_to_non_nullable
              as Uint8List,
      callId: null == callId
          ? _value.callId
          : callId // ignore: cast_nullable_to_non_nullable
              as String,
      sender: freezed == sender
          ? _value.sender
          : sender // ignore: cast_nullable_to_non_nullable
              as Typed<FixedEncodedString43>?,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$VeilidAppCall implements VeilidAppCall {
  const _$VeilidAppCall(
      {@Uint8ListJsonConverter() required this.message,
      required this.callId,
      this.sender,
      final String? $type})
      : $type = $type ?? 'AppCall';

  factory _$VeilidAppCall.fromJson(Map<String, dynamic> json) =>
      _$$VeilidAppCallFromJson(json);

  @override
  @Uint8ListJsonConverter()
  final Uint8List message;
  @override
  final String callId;
  @override
  final Typed<FixedEncodedString43>? sender;

  @JsonKey(name: 'kind')
  final String $type;

  @override
  String toString() {
    return 'VeilidUpdate.appCall(message: $message, callId: $callId, sender: $sender)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$VeilidAppCall &&
            const DeepCollectionEquality().equals(other.message, message) &&
            (identical(other.callId, callId) || other.callId == callId) &&
            (identical(other.sender, sender) || other.sender == sender));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType,
      const DeepCollectionEquality().hash(message), callId, sender);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$VeilidAppCallCopyWith<_$VeilidAppCall> get copyWith =>
      __$$VeilidAppCallCopyWithImpl<_$VeilidAppCall>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(
            VeilidLogLevel logLevel, String message, String? backtrace)
        log,
    required TResult Function(@Uint8ListJsonConverter() Uint8List message,
            Typed<FixedEncodedString43>? sender)
        appMessage,
    required TResult Function(@Uint8ListJsonConverter() Uint8List message,
            String callId, Typed<FixedEncodedString43>? sender)
        appCall,
    required TResult Function(AttachmentState state, bool publicInternetReady,
            bool localNetworkReady)
        attachment,
    required TResult Function(bool started, BigInt bpsDown, BigInt bpsUp,
            List<PeerTableData> peers)
        network,
    required TResult Function(VeilidConfig config) config,
    required TResult Function(
            List<String> deadRoutes, List<String> deadRemoteRoutes)
        routeChange,
    required TResult Function(Typed<FixedEncodedString43> key,
            List<ValueSubkeyRange> subkeys, int count, ValueData value)
        valueChange,
  }) {
    return appCall(message, callId, sender);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(
            VeilidLogLevel logLevel, String message, String? backtrace)?
        log,
    TResult? Function(@Uint8ListJsonConverter() Uint8List message,
            Typed<FixedEncodedString43>? sender)?
        appMessage,
    TResult? Function(@Uint8ListJsonConverter() Uint8List message,
            String callId, Typed<FixedEncodedString43>? sender)?
        appCall,
    TResult? Function(AttachmentState state, bool publicInternetReady,
            bool localNetworkReady)?
        attachment,
    TResult? Function(bool started, BigInt bpsDown, BigInt bpsUp,
            List<PeerTableData> peers)?
        network,
    TResult? Function(VeilidConfig config)? config,
    TResult? Function(List<String> deadRoutes, List<String> deadRemoteRoutes)?
        routeChange,
    TResult? Function(Typed<FixedEncodedString43> key,
            List<ValueSubkeyRange> subkeys, int count, ValueData value)?
        valueChange,
  }) {
    return appCall?.call(message, callId, sender);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(
            VeilidLogLevel logLevel, String message, String? backtrace)?
        log,
    TResult Function(@Uint8ListJsonConverter() Uint8List message,
            Typed<FixedEncodedString43>? sender)?
        appMessage,
    TResult Function(@Uint8ListJsonConverter() Uint8List message, String callId,
            Typed<FixedEncodedString43>? sender)?
        appCall,
    TResult Function(AttachmentState state, bool publicInternetReady,
            bool localNetworkReady)?
        attachment,
    TResult Function(bool started, BigInt bpsDown, BigInt bpsUp,
            List<PeerTableData> peers)?
        network,
    TResult Function(VeilidConfig config)? config,
    TResult Function(List<String> deadRoutes, List<String> deadRemoteRoutes)?
        routeChange,
    TResult Function(Typed<FixedEncodedString43> key,
            List<ValueSubkeyRange> subkeys, int count, ValueData value)?
        valueChange,
    required TResult orElse(),
  }) {
    if (appCall != null) {
      return appCall(message, callId, sender);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(VeilidLog value) log,
    required TResult Function(VeilidAppMessage value) appMessage,
    required TResult Function(VeilidAppCall value) appCall,
    required TResult Function(VeilidUpdateAttachment value) attachment,
    required TResult Function(VeilidUpdateNetwork value) network,
    required TResult Function(VeilidUpdateConfig value) config,
    required TResult Function(VeilidUpdateRouteChange value) routeChange,
    required TResult Function(VeilidUpdateValueChange value) valueChange,
  }) {
    return appCall(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(VeilidLog value)? log,
    TResult? Function(VeilidAppMessage value)? appMessage,
    TResult? Function(VeilidAppCall value)? appCall,
    TResult? Function(VeilidUpdateAttachment value)? attachment,
    TResult? Function(VeilidUpdateNetwork value)? network,
    TResult? Function(VeilidUpdateConfig value)? config,
    TResult? Function(VeilidUpdateRouteChange value)? routeChange,
    TResult? Function(VeilidUpdateValueChange value)? valueChange,
  }) {
    return appCall?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(VeilidLog value)? log,
    TResult Function(VeilidAppMessage value)? appMessage,
    TResult Function(VeilidAppCall value)? appCall,
    TResult Function(VeilidUpdateAttachment value)? attachment,
    TResult Function(VeilidUpdateNetwork value)? network,
    TResult Function(VeilidUpdateConfig value)? config,
    TResult Function(VeilidUpdateRouteChange value)? routeChange,
    TResult Function(VeilidUpdateValueChange value)? valueChange,
    required TResult orElse(),
  }) {
    if (appCall != null) {
      return appCall(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$VeilidAppCallToJson(
      this,
    );
  }
}

abstract class VeilidAppCall implements VeilidUpdate {
  const factory VeilidAppCall(
      {@Uint8ListJsonConverter() required final Uint8List message,
      required final String callId,
      final Typed<FixedEncodedString43>? sender}) = _$VeilidAppCall;

  factory VeilidAppCall.fromJson(Map<String, dynamic> json) =
      _$VeilidAppCall.fromJson;

  @Uint8ListJsonConverter()
  Uint8List get message;
  String get callId;
  Typed<FixedEncodedString43>? get sender;
  @JsonKey(ignore: true)
  _$$VeilidAppCallCopyWith<_$VeilidAppCall> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$VeilidUpdateAttachmentCopyWith<$Res> {
  factory _$$VeilidUpdateAttachmentCopyWith(_$VeilidUpdateAttachment value,
          $Res Function(_$VeilidUpdateAttachment) then) =
      __$$VeilidUpdateAttachmentCopyWithImpl<$Res>;
  @useResult
  $Res call(
      {AttachmentState state,
      bool publicInternetReady,
      bool localNetworkReady});
}

/// @nodoc
class __$$VeilidUpdateAttachmentCopyWithImpl<$Res>
    extends _$VeilidUpdateCopyWithImpl<$Res, _$VeilidUpdateAttachment>
    implements _$$VeilidUpdateAttachmentCopyWith<$Res> {
  __$$VeilidUpdateAttachmentCopyWithImpl(_$VeilidUpdateAttachment _value,
      $Res Function(_$VeilidUpdateAttachment) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? state = null,
    Object? publicInternetReady = null,
    Object? localNetworkReady = null,
  }) {
    return _then(_$VeilidUpdateAttachment(
      state: null == state
          ? _value.state
          : state // ignore: cast_nullable_to_non_nullable
              as AttachmentState,
      publicInternetReady: null == publicInternetReady
          ? _value.publicInternetReady
          : publicInternetReady // ignore: cast_nullable_to_non_nullable
              as bool,
      localNetworkReady: null == localNetworkReady
          ? _value.localNetworkReady
          : localNetworkReady // ignore: cast_nullable_to_non_nullable
              as bool,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$VeilidUpdateAttachment implements VeilidUpdateAttachment {
  const _$VeilidUpdateAttachment(
      {required this.state,
      required this.publicInternetReady,
      required this.localNetworkReady,
      final String? $type})
      : $type = $type ?? 'Attachment';

  factory _$VeilidUpdateAttachment.fromJson(Map<String, dynamic> json) =>
      _$$VeilidUpdateAttachmentFromJson(json);

  @override
  final AttachmentState state;
  @override
  final bool publicInternetReady;
  @override
  final bool localNetworkReady;

  @JsonKey(name: 'kind')
  final String $type;

  @override
  String toString() {
    return 'VeilidUpdate.attachment(state: $state, publicInternetReady: $publicInternetReady, localNetworkReady: $localNetworkReady)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$VeilidUpdateAttachment &&
            (identical(other.state, state) || other.state == state) &&
            (identical(other.publicInternetReady, publicInternetReady) ||
                other.publicInternetReady == publicInternetReady) &&
            (identical(other.localNetworkReady, localNetworkReady) ||
                other.localNetworkReady == localNetworkReady));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode =>
      Object.hash(runtimeType, state, publicInternetReady, localNetworkReady);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$VeilidUpdateAttachmentCopyWith<_$VeilidUpdateAttachment> get copyWith =>
      __$$VeilidUpdateAttachmentCopyWithImpl<_$VeilidUpdateAttachment>(
          this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(
            VeilidLogLevel logLevel, String message, String? backtrace)
        log,
    required TResult Function(@Uint8ListJsonConverter() Uint8List message,
            Typed<FixedEncodedString43>? sender)
        appMessage,
    required TResult Function(@Uint8ListJsonConverter() Uint8List message,
            String callId, Typed<FixedEncodedString43>? sender)
        appCall,
    required TResult Function(AttachmentState state, bool publicInternetReady,
            bool localNetworkReady)
        attachment,
    required TResult Function(bool started, BigInt bpsDown, BigInt bpsUp,
            List<PeerTableData> peers)
        network,
    required TResult Function(VeilidConfig config) config,
    required TResult Function(
            List<String> deadRoutes, List<String> deadRemoteRoutes)
        routeChange,
    required TResult Function(Typed<FixedEncodedString43> key,
            List<ValueSubkeyRange> subkeys, int count, ValueData value)
        valueChange,
  }) {
    return attachment(state, publicInternetReady, localNetworkReady);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(
            VeilidLogLevel logLevel, String message, String? backtrace)?
        log,
    TResult? Function(@Uint8ListJsonConverter() Uint8List message,
            Typed<FixedEncodedString43>? sender)?
        appMessage,
    TResult? Function(@Uint8ListJsonConverter() Uint8List message,
            String callId, Typed<FixedEncodedString43>? sender)?
        appCall,
    TResult? Function(AttachmentState state, bool publicInternetReady,
            bool localNetworkReady)?
        attachment,
    TResult? Function(bool started, BigInt bpsDown, BigInt bpsUp,
            List<PeerTableData> peers)?
        network,
    TResult? Function(VeilidConfig config)? config,
    TResult? Function(List<String> deadRoutes, List<String> deadRemoteRoutes)?
        routeChange,
    TResult? Function(Typed<FixedEncodedString43> key,
            List<ValueSubkeyRange> subkeys, int count, ValueData value)?
        valueChange,
  }) {
    return attachment?.call(state, publicInternetReady, localNetworkReady);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(
            VeilidLogLevel logLevel, String message, String? backtrace)?
        log,
    TResult Function(@Uint8ListJsonConverter() Uint8List message,
            Typed<FixedEncodedString43>? sender)?
        appMessage,
    TResult Function(@Uint8ListJsonConverter() Uint8List message, String callId,
            Typed<FixedEncodedString43>? sender)?
        appCall,
    TResult Function(AttachmentState state, bool publicInternetReady,
            bool localNetworkReady)?
        attachment,
    TResult Function(bool started, BigInt bpsDown, BigInt bpsUp,
            List<PeerTableData> peers)?
        network,
    TResult Function(VeilidConfig config)? config,
    TResult Function(List<String> deadRoutes, List<String> deadRemoteRoutes)?
        routeChange,
    TResult Function(Typed<FixedEncodedString43> key,
            List<ValueSubkeyRange> subkeys, int count, ValueData value)?
        valueChange,
    required TResult orElse(),
  }) {
    if (attachment != null) {
      return attachment(state, publicInternetReady, localNetworkReady);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(VeilidLog value) log,
    required TResult Function(VeilidAppMessage value) appMessage,
    required TResult Function(VeilidAppCall value) appCall,
    required TResult Function(VeilidUpdateAttachment value) attachment,
    required TResult Function(VeilidUpdateNetwork value) network,
    required TResult Function(VeilidUpdateConfig value) config,
    required TResult Function(VeilidUpdateRouteChange value) routeChange,
    required TResult Function(VeilidUpdateValueChange value) valueChange,
  }) {
    return attachment(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(VeilidLog value)? log,
    TResult? Function(VeilidAppMessage value)? appMessage,
    TResult? Function(VeilidAppCall value)? appCall,
    TResult? Function(VeilidUpdateAttachment value)? attachment,
    TResult? Function(VeilidUpdateNetwork value)? network,
    TResult? Function(VeilidUpdateConfig value)? config,
    TResult? Function(VeilidUpdateRouteChange value)? routeChange,
    TResult? Function(VeilidUpdateValueChange value)? valueChange,
  }) {
    return attachment?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(VeilidLog value)? log,
    TResult Function(VeilidAppMessage value)? appMessage,
    TResult Function(VeilidAppCall value)? appCall,
    TResult Function(VeilidUpdateAttachment value)? attachment,
    TResult Function(VeilidUpdateNetwork value)? network,
    TResult Function(VeilidUpdateConfig value)? config,
    TResult Function(VeilidUpdateRouteChange value)? routeChange,
    TResult Function(VeilidUpdateValueChange value)? valueChange,
    required TResult orElse(),
  }) {
    if (attachment != null) {
      return attachment(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$VeilidUpdateAttachmentToJson(
      this,
    );
  }
}

abstract class VeilidUpdateAttachment implements VeilidUpdate {
  const factory VeilidUpdateAttachment(
      {required final AttachmentState state,
      required final bool publicInternetReady,
      required final bool localNetworkReady}) = _$VeilidUpdateAttachment;

  factory VeilidUpdateAttachment.fromJson(Map<String, dynamic> json) =
      _$VeilidUpdateAttachment.fromJson;

  AttachmentState get state;
  bool get publicInternetReady;
  bool get localNetworkReady;
  @JsonKey(ignore: true)
  _$$VeilidUpdateAttachmentCopyWith<_$VeilidUpdateAttachment> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$VeilidUpdateNetworkCopyWith<$Res> {
  factory _$$VeilidUpdateNetworkCopyWith(_$VeilidUpdateNetwork value,
          $Res Function(_$VeilidUpdateNetwork) then) =
      __$$VeilidUpdateNetworkCopyWithImpl<$Res>;
  @useResult
  $Res call(
      {bool started, BigInt bpsDown, BigInt bpsUp, List<PeerTableData> peers});
}

/// @nodoc
class __$$VeilidUpdateNetworkCopyWithImpl<$Res>
    extends _$VeilidUpdateCopyWithImpl<$Res, _$VeilidUpdateNetwork>
    implements _$$VeilidUpdateNetworkCopyWith<$Res> {
  __$$VeilidUpdateNetworkCopyWithImpl(
      _$VeilidUpdateNetwork _value, $Res Function(_$VeilidUpdateNetwork) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? started = null,
    Object? bpsDown = null,
    Object? bpsUp = null,
    Object? peers = null,
  }) {
    return _then(_$VeilidUpdateNetwork(
      started: null == started
          ? _value.started
          : started // ignore: cast_nullable_to_non_nullable
              as bool,
      bpsDown: null == bpsDown
          ? _value.bpsDown
          : bpsDown // ignore: cast_nullable_to_non_nullable
              as BigInt,
      bpsUp: null == bpsUp
          ? _value.bpsUp
          : bpsUp // ignore: cast_nullable_to_non_nullable
              as BigInt,
      peers: null == peers
          ? _value._peers
          : peers // ignore: cast_nullable_to_non_nullable
              as List<PeerTableData>,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$VeilidUpdateNetwork implements VeilidUpdateNetwork {
  const _$VeilidUpdateNetwork(
      {required this.started,
      required this.bpsDown,
      required this.bpsUp,
      required final List<PeerTableData> peers,
      final String? $type})
      : _peers = peers,
        $type = $type ?? 'Network';

  factory _$VeilidUpdateNetwork.fromJson(Map<String, dynamic> json) =>
      _$$VeilidUpdateNetworkFromJson(json);

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

  @JsonKey(name: 'kind')
  final String $type;

  @override
  String toString() {
    return 'VeilidUpdate.network(started: $started, bpsDown: $bpsDown, bpsUp: $bpsUp, peers: $peers)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$VeilidUpdateNetwork &&
            (identical(other.started, started) || other.started == started) &&
            (identical(other.bpsDown, bpsDown) || other.bpsDown == bpsDown) &&
            (identical(other.bpsUp, bpsUp) || other.bpsUp == bpsUp) &&
            const DeepCollectionEquality().equals(other._peers, _peers));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, started, bpsDown, bpsUp,
      const DeepCollectionEquality().hash(_peers));

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$VeilidUpdateNetworkCopyWith<_$VeilidUpdateNetwork> get copyWith =>
      __$$VeilidUpdateNetworkCopyWithImpl<_$VeilidUpdateNetwork>(
          this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(
            VeilidLogLevel logLevel, String message, String? backtrace)
        log,
    required TResult Function(@Uint8ListJsonConverter() Uint8List message,
            Typed<FixedEncodedString43>? sender)
        appMessage,
    required TResult Function(@Uint8ListJsonConverter() Uint8List message,
            String callId, Typed<FixedEncodedString43>? sender)
        appCall,
    required TResult Function(AttachmentState state, bool publicInternetReady,
            bool localNetworkReady)
        attachment,
    required TResult Function(bool started, BigInt bpsDown, BigInt bpsUp,
            List<PeerTableData> peers)
        network,
    required TResult Function(VeilidConfig config) config,
    required TResult Function(
            List<String> deadRoutes, List<String> deadRemoteRoutes)
        routeChange,
    required TResult Function(Typed<FixedEncodedString43> key,
            List<ValueSubkeyRange> subkeys, int count, ValueData value)
        valueChange,
  }) {
    return network(started, bpsDown, bpsUp, peers);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(
            VeilidLogLevel logLevel, String message, String? backtrace)?
        log,
    TResult? Function(@Uint8ListJsonConverter() Uint8List message,
            Typed<FixedEncodedString43>? sender)?
        appMessage,
    TResult? Function(@Uint8ListJsonConverter() Uint8List message,
            String callId, Typed<FixedEncodedString43>? sender)?
        appCall,
    TResult? Function(AttachmentState state, bool publicInternetReady,
            bool localNetworkReady)?
        attachment,
    TResult? Function(bool started, BigInt bpsDown, BigInt bpsUp,
            List<PeerTableData> peers)?
        network,
    TResult? Function(VeilidConfig config)? config,
    TResult? Function(List<String> deadRoutes, List<String> deadRemoteRoutes)?
        routeChange,
    TResult? Function(Typed<FixedEncodedString43> key,
            List<ValueSubkeyRange> subkeys, int count, ValueData value)?
        valueChange,
  }) {
    return network?.call(started, bpsDown, bpsUp, peers);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(
            VeilidLogLevel logLevel, String message, String? backtrace)?
        log,
    TResult Function(@Uint8ListJsonConverter() Uint8List message,
            Typed<FixedEncodedString43>? sender)?
        appMessage,
    TResult Function(@Uint8ListJsonConverter() Uint8List message, String callId,
            Typed<FixedEncodedString43>? sender)?
        appCall,
    TResult Function(AttachmentState state, bool publicInternetReady,
            bool localNetworkReady)?
        attachment,
    TResult Function(bool started, BigInt bpsDown, BigInt bpsUp,
            List<PeerTableData> peers)?
        network,
    TResult Function(VeilidConfig config)? config,
    TResult Function(List<String> deadRoutes, List<String> deadRemoteRoutes)?
        routeChange,
    TResult Function(Typed<FixedEncodedString43> key,
            List<ValueSubkeyRange> subkeys, int count, ValueData value)?
        valueChange,
    required TResult orElse(),
  }) {
    if (network != null) {
      return network(started, bpsDown, bpsUp, peers);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(VeilidLog value) log,
    required TResult Function(VeilidAppMessage value) appMessage,
    required TResult Function(VeilidAppCall value) appCall,
    required TResult Function(VeilidUpdateAttachment value) attachment,
    required TResult Function(VeilidUpdateNetwork value) network,
    required TResult Function(VeilidUpdateConfig value) config,
    required TResult Function(VeilidUpdateRouteChange value) routeChange,
    required TResult Function(VeilidUpdateValueChange value) valueChange,
  }) {
    return network(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(VeilidLog value)? log,
    TResult? Function(VeilidAppMessage value)? appMessage,
    TResult? Function(VeilidAppCall value)? appCall,
    TResult? Function(VeilidUpdateAttachment value)? attachment,
    TResult? Function(VeilidUpdateNetwork value)? network,
    TResult? Function(VeilidUpdateConfig value)? config,
    TResult? Function(VeilidUpdateRouteChange value)? routeChange,
    TResult? Function(VeilidUpdateValueChange value)? valueChange,
  }) {
    return network?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(VeilidLog value)? log,
    TResult Function(VeilidAppMessage value)? appMessage,
    TResult Function(VeilidAppCall value)? appCall,
    TResult Function(VeilidUpdateAttachment value)? attachment,
    TResult Function(VeilidUpdateNetwork value)? network,
    TResult Function(VeilidUpdateConfig value)? config,
    TResult Function(VeilidUpdateRouteChange value)? routeChange,
    TResult Function(VeilidUpdateValueChange value)? valueChange,
    required TResult orElse(),
  }) {
    if (network != null) {
      return network(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$VeilidUpdateNetworkToJson(
      this,
    );
  }
}

abstract class VeilidUpdateNetwork implements VeilidUpdate {
  const factory VeilidUpdateNetwork(
      {required final bool started,
      required final BigInt bpsDown,
      required final BigInt bpsUp,
      required final List<PeerTableData> peers}) = _$VeilidUpdateNetwork;

  factory VeilidUpdateNetwork.fromJson(Map<String, dynamic> json) =
      _$VeilidUpdateNetwork.fromJson;

  bool get started;
  BigInt get bpsDown;
  BigInt get bpsUp;
  List<PeerTableData> get peers;
  @JsonKey(ignore: true)
  _$$VeilidUpdateNetworkCopyWith<_$VeilidUpdateNetwork> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$VeilidUpdateConfigCopyWith<$Res> {
  factory _$$VeilidUpdateConfigCopyWith(_$VeilidUpdateConfig value,
          $Res Function(_$VeilidUpdateConfig) then) =
      __$$VeilidUpdateConfigCopyWithImpl<$Res>;
  @useResult
  $Res call({VeilidConfig config});

  $VeilidConfigCopyWith<$Res> get config;
}

/// @nodoc
class __$$VeilidUpdateConfigCopyWithImpl<$Res>
    extends _$VeilidUpdateCopyWithImpl<$Res, _$VeilidUpdateConfig>
    implements _$$VeilidUpdateConfigCopyWith<$Res> {
  __$$VeilidUpdateConfigCopyWithImpl(
      _$VeilidUpdateConfig _value, $Res Function(_$VeilidUpdateConfig) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? config = null,
  }) {
    return _then(_$VeilidUpdateConfig(
      config: null == config
          ? _value.config
          : config // ignore: cast_nullable_to_non_nullable
              as VeilidConfig,
    ));
  }

  @override
  @pragma('vm:prefer-inline')
  $VeilidConfigCopyWith<$Res> get config {
    return $VeilidConfigCopyWith<$Res>(_value.config, (value) {
      return _then(_value.copyWith(config: value));
    });
  }
}

/// @nodoc
@JsonSerializable()
class _$VeilidUpdateConfig implements VeilidUpdateConfig {
  const _$VeilidUpdateConfig({required this.config, final String? $type})
      : $type = $type ?? 'Config';

  factory _$VeilidUpdateConfig.fromJson(Map<String, dynamic> json) =>
      _$$VeilidUpdateConfigFromJson(json);

  @override
  final VeilidConfig config;

  @JsonKey(name: 'kind')
  final String $type;

  @override
  String toString() {
    return 'VeilidUpdate.config(config: $config)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$VeilidUpdateConfig &&
            (identical(other.config, config) || other.config == config));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, config);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$VeilidUpdateConfigCopyWith<_$VeilidUpdateConfig> get copyWith =>
      __$$VeilidUpdateConfigCopyWithImpl<_$VeilidUpdateConfig>(
          this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(
            VeilidLogLevel logLevel, String message, String? backtrace)
        log,
    required TResult Function(@Uint8ListJsonConverter() Uint8List message,
            Typed<FixedEncodedString43>? sender)
        appMessage,
    required TResult Function(@Uint8ListJsonConverter() Uint8List message,
            String callId, Typed<FixedEncodedString43>? sender)
        appCall,
    required TResult Function(AttachmentState state, bool publicInternetReady,
            bool localNetworkReady)
        attachment,
    required TResult Function(bool started, BigInt bpsDown, BigInt bpsUp,
            List<PeerTableData> peers)
        network,
    required TResult Function(VeilidConfig config) config,
    required TResult Function(
            List<String> deadRoutes, List<String> deadRemoteRoutes)
        routeChange,
    required TResult Function(Typed<FixedEncodedString43> key,
            List<ValueSubkeyRange> subkeys, int count, ValueData value)
        valueChange,
  }) {
    return config(this.config);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(
            VeilidLogLevel logLevel, String message, String? backtrace)?
        log,
    TResult? Function(@Uint8ListJsonConverter() Uint8List message,
            Typed<FixedEncodedString43>? sender)?
        appMessage,
    TResult? Function(@Uint8ListJsonConverter() Uint8List message,
            String callId, Typed<FixedEncodedString43>? sender)?
        appCall,
    TResult? Function(AttachmentState state, bool publicInternetReady,
            bool localNetworkReady)?
        attachment,
    TResult? Function(bool started, BigInt bpsDown, BigInt bpsUp,
            List<PeerTableData> peers)?
        network,
    TResult? Function(VeilidConfig config)? config,
    TResult? Function(List<String> deadRoutes, List<String> deadRemoteRoutes)?
        routeChange,
    TResult? Function(Typed<FixedEncodedString43> key,
            List<ValueSubkeyRange> subkeys, int count, ValueData value)?
        valueChange,
  }) {
    return config?.call(this.config);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(
            VeilidLogLevel logLevel, String message, String? backtrace)?
        log,
    TResult Function(@Uint8ListJsonConverter() Uint8List message,
            Typed<FixedEncodedString43>? sender)?
        appMessage,
    TResult Function(@Uint8ListJsonConverter() Uint8List message, String callId,
            Typed<FixedEncodedString43>? sender)?
        appCall,
    TResult Function(AttachmentState state, bool publicInternetReady,
            bool localNetworkReady)?
        attachment,
    TResult Function(bool started, BigInt bpsDown, BigInt bpsUp,
            List<PeerTableData> peers)?
        network,
    TResult Function(VeilidConfig config)? config,
    TResult Function(List<String> deadRoutes, List<String> deadRemoteRoutes)?
        routeChange,
    TResult Function(Typed<FixedEncodedString43> key,
            List<ValueSubkeyRange> subkeys, int count, ValueData value)?
        valueChange,
    required TResult orElse(),
  }) {
    if (config != null) {
      return config(this.config);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(VeilidLog value) log,
    required TResult Function(VeilidAppMessage value) appMessage,
    required TResult Function(VeilidAppCall value) appCall,
    required TResult Function(VeilidUpdateAttachment value) attachment,
    required TResult Function(VeilidUpdateNetwork value) network,
    required TResult Function(VeilidUpdateConfig value) config,
    required TResult Function(VeilidUpdateRouteChange value) routeChange,
    required TResult Function(VeilidUpdateValueChange value) valueChange,
  }) {
    return config(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(VeilidLog value)? log,
    TResult? Function(VeilidAppMessage value)? appMessage,
    TResult? Function(VeilidAppCall value)? appCall,
    TResult? Function(VeilidUpdateAttachment value)? attachment,
    TResult? Function(VeilidUpdateNetwork value)? network,
    TResult? Function(VeilidUpdateConfig value)? config,
    TResult? Function(VeilidUpdateRouteChange value)? routeChange,
    TResult? Function(VeilidUpdateValueChange value)? valueChange,
  }) {
    return config?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(VeilidLog value)? log,
    TResult Function(VeilidAppMessage value)? appMessage,
    TResult Function(VeilidAppCall value)? appCall,
    TResult Function(VeilidUpdateAttachment value)? attachment,
    TResult Function(VeilidUpdateNetwork value)? network,
    TResult Function(VeilidUpdateConfig value)? config,
    TResult Function(VeilidUpdateRouteChange value)? routeChange,
    TResult Function(VeilidUpdateValueChange value)? valueChange,
    required TResult orElse(),
  }) {
    if (config != null) {
      return config(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$VeilidUpdateConfigToJson(
      this,
    );
  }
}

abstract class VeilidUpdateConfig implements VeilidUpdate {
  const factory VeilidUpdateConfig({required final VeilidConfig config}) =
      _$VeilidUpdateConfig;

  factory VeilidUpdateConfig.fromJson(Map<String, dynamic> json) =
      _$VeilidUpdateConfig.fromJson;

  VeilidConfig get config;
  @JsonKey(ignore: true)
  _$$VeilidUpdateConfigCopyWith<_$VeilidUpdateConfig> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$VeilidUpdateRouteChangeCopyWith<$Res> {
  factory _$$VeilidUpdateRouteChangeCopyWith(_$VeilidUpdateRouteChange value,
          $Res Function(_$VeilidUpdateRouteChange) then) =
      __$$VeilidUpdateRouteChangeCopyWithImpl<$Res>;
  @useResult
  $Res call({List<String> deadRoutes, List<String> deadRemoteRoutes});
}

/// @nodoc
class __$$VeilidUpdateRouteChangeCopyWithImpl<$Res>
    extends _$VeilidUpdateCopyWithImpl<$Res, _$VeilidUpdateRouteChange>
    implements _$$VeilidUpdateRouteChangeCopyWith<$Res> {
  __$$VeilidUpdateRouteChangeCopyWithImpl(_$VeilidUpdateRouteChange _value,
      $Res Function(_$VeilidUpdateRouteChange) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? deadRoutes = null,
    Object? deadRemoteRoutes = null,
  }) {
    return _then(_$VeilidUpdateRouteChange(
      deadRoutes: null == deadRoutes
          ? _value._deadRoutes
          : deadRoutes // ignore: cast_nullable_to_non_nullable
              as List<String>,
      deadRemoteRoutes: null == deadRemoteRoutes
          ? _value._deadRemoteRoutes
          : deadRemoteRoutes // ignore: cast_nullable_to_non_nullable
              as List<String>,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$VeilidUpdateRouteChange implements VeilidUpdateRouteChange {
  const _$VeilidUpdateRouteChange(
      {required final List<String> deadRoutes,
      required final List<String> deadRemoteRoutes,
      final String? $type})
      : _deadRoutes = deadRoutes,
        _deadRemoteRoutes = deadRemoteRoutes,
        $type = $type ?? 'RouteChange';

  factory _$VeilidUpdateRouteChange.fromJson(Map<String, dynamic> json) =>
      _$$VeilidUpdateRouteChangeFromJson(json);

  final List<String> _deadRoutes;
  @override
  List<String> get deadRoutes {
    if (_deadRoutes is EqualUnmodifiableListView) return _deadRoutes;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_deadRoutes);
  }

  final List<String> _deadRemoteRoutes;
  @override
  List<String> get deadRemoteRoutes {
    if (_deadRemoteRoutes is EqualUnmodifiableListView)
      return _deadRemoteRoutes;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_deadRemoteRoutes);
  }

  @JsonKey(name: 'kind')
  final String $type;

  @override
  String toString() {
    return 'VeilidUpdate.routeChange(deadRoutes: $deadRoutes, deadRemoteRoutes: $deadRemoteRoutes)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$VeilidUpdateRouteChange &&
            const DeepCollectionEquality()
                .equals(other._deadRoutes, _deadRoutes) &&
            const DeepCollectionEquality()
                .equals(other._deadRemoteRoutes, _deadRemoteRoutes));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(
      runtimeType,
      const DeepCollectionEquality().hash(_deadRoutes),
      const DeepCollectionEquality().hash(_deadRemoteRoutes));

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$VeilidUpdateRouteChangeCopyWith<_$VeilidUpdateRouteChange> get copyWith =>
      __$$VeilidUpdateRouteChangeCopyWithImpl<_$VeilidUpdateRouteChange>(
          this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(
            VeilidLogLevel logLevel, String message, String? backtrace)
        log,
    required TResult Function(@Uint8ListJsonConverter() Uint8List message,
            Typed<FixedEncodedString43>? sender)
        appMessage,
    required TResult Function(@Uint8ListJsonConverter() Uint8List message,
            String callId, Typed<FixedEncodedString43>? sender)
        appCall,
    required TResult Function(AttachmentState state, bool publicInternetReady,
            bool localNetworkReady)
        attachment,
    required TResult Function(bool started, BigInt bpsDown, BigInt bpsUp,
            List<PeerTableData> peers)
        network,
    required TResult Function(VeilidConfig config) config,
    required TResult Function(
            List<String> deadRoutes, List<String> deadRemoteRoutes)
        routeChange,
    required TResult Function(Typed<FixedEncodedString43> key,
            List<ValueSubkeyRange> subkeys, int count, ValueData value)
        valueChange,
  }) {
    return routeChange(deadRoutes, deadRemoteRoutes);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(
            VeilidLogLevel logLevel, String message, String? backtrace)?
        log,
    TResult? Function(@Uint8ListJsonConverter() Uint8List message,
            Typed<FixedEncodedString43>? sender)?
        appMessage,
    TResult? Function(@Uint8ListJsonConverter() Uint8List message,
            String callId, Typed<FixedEncodedString43>? sender)?
        appCall,
    TResult? Function(AttachmentState state, bool publicInternetReady,
            bool localNetworkReady)?
        attachment,
    TResult? Function(bool started, BigInt bpsDown, BigInt bpsUp,
            List<PeerTableData> peers)?
        network,
    TResult? Function(VeilidConfig config)? config,
    TResult? Function(List<String> deadRoutes, List<String> deadRemoteRoutes)?
        routeChange,
    TResult? Function(Typed<FixedEncodedString43> key,
            List<ValueSubkeyRange> subkeys, int count, ValueData value)?
        valueChange,
  }) {
    return routeChange?.call(deadRoutes, deadRemoteRoutes);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(
            VeilidLogLevel logLevel, String message, String? backtrace)?
        log,
    TResult Function(@Uint8ListJsonConverter() Uint8List message,
            Typed<FixedEncodedString43>? sender)?
        appMessage,
    TResult Function(@Uint8ListJsonConverter() Uint8List message, String callId,
            Typed<FixedEncodedString43>? sender)?
        appCall,
    TResult Function(AttachmentState state, bool publicInternetReady,
            bool localNetworkReady)?
        attachment,
    TResult Function(bool started, BigInt bpsDown, BigInt bpsUp,
            List<PeerTableData> peers)?
        network,
    TResult Function(VeilidConfig config)? config,
    TResult Function(List<String> deadRoutes, List<String> deadRemoteRoutes)?
        routeChange,
    TResult Function(Typed<FixedEncodedString43> key,
            List<ValueSubkeyRange> subkeys, int count, ValueData value)?
        valueChange,
    required TResult orElse(),
  }) {
    if (routeChange != null) {
      return routeChange(deadRoutes, deadRemoteRoutes);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(VeilidLog value) log,
    required TResult Function(VeilidAppMessage value) appMessage,
    required TResult Function(VeilidAppCall value) appCall,
    required TResult Function(VeilidUpdateAttachment value) attachment,
    required TResult Function(VeilidUpdateNetwork value) network,
    required TResult Function(VeilidUpdateConfig value) config,
    required TResult Function(VeilidUpdateRouteChange value) routeChange,
    required TResult Function(VeilidUpdateValueChange value) valueChange,
  }) {
    return routeChange(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(VeilidLog value)? log,
    TResult? Function(VeilidAppMessage value)? appMessage,
    TResult? Function(VeilidAppCall value)? appCall,
    TResult? Function(VeilidUpdateAttachment value)? attachment,
    TResult? Function(VeilidUpdateNetwork value)? network,
    TResult? Function(VeilidUpdateConfig value)? config,
    TResult? Function(VeilidUpdateRouteChange value)? routeChange,
    TResult? Function(VeilidUpdateValueChange value)? valueChange,
  }) {
    return routeChange?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(VeilidLog value)? log,
    TResult Function(VeilidAppMessage value)? appMessage,
    TResult Function(VeilidAppCall value)? appCall,
    TResult Function(VeilidUpdateAttachment value)? attachment,
    TResult Function(VeilidUpdateNetwork value)? network,
    TResult Function(VeilidUpdateConfig value)? config,
    TResult Function(VeilidUpdateRouteChange value)? routeChange,
    TResult Function(VeilidUpdateValueChange value)? valueChange,
    required TResult orElse(),
  }) {
    if (routeChange != null) {
      return routeChange(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$VeilidUpdateRouteChangeToJson(
      this,
    );
  }
}

abstract class VeilidUpdateRouteChange implements VeilidUpdate {
  const factory VeilidUpdateRouteChange(
          {required final List<String> deadRoutes,
          required final List<String> deadRemoteRoutes}) =
      _$VeilidUpdateRouteChange;

  factory VeilidUpdateRouteChange.fromJson(Map<String, dynamic> json) =
      _$VeilidUpdateRouteChange.fromJson;

  List<String> get deadRoutes;
  List<String> get deadRemoteRoutes;
  @JsonKey(ignore: true)
  _$$VeilidUpdateRouteChangeCopyWith<_$VeilidUpdateRouteChange> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$VeilidUpdateValueChangeCopyWith<$Res> {
  factory _$$VeilidUpdateValueChangeCopyWith(_$VeilidUpdateValueChange value,
          $Res Function(_$VeilidUpdateValueChange) then) =
      __$$VeilidUpdateValueChangeCopyWithImpl<$Res>;
  @useResult
  $Res call(
      {Typed<FixedEncodedString43> key,
      List<ValueSubkeyRange> subkeys,
      int count,
      ValueData value});

  $ValueDataCopyWith<$Res> get value;
}

/// @nodoc
class __$$VeilidUpdateValueChangeCopyWithImpl<$Res>
    extends _$VeilidUpdateCopyWithImpl<$Res, _$VeilidUpdateValueChange>
    implements _$$VeilidUpdateValueChangeCopyWith<$Res> {
  __$$VeilidUpdateValueChangeCopyWithImpl(_$VeilidUpdateValueChange _value,
      $Res Function(_$VeilidUpdateValueChange) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? key = null,
    Object? subkeys = null,
    Object? count = null,
    Object? value = null,
  }) {
    return _then(_$VeilidUpdateValueChange(
      key: null == key
          ? _value.key
          : key // ignore: cast_nullable_to_non_nullable
              as Typed<FixedEncodedString43>,
      subkeys: null == subkeys
          ? _value._subkeys
          : subkeys // ignore: cast_nullable_to_non_nullable
              as List<ValueSubkeyRange>,
      count: null == count
          ? _value.count
          : count // ignore: cast_nullable_to_non_nullable
              as int,
      value: null == value
          ? _value.value
          : value // ignore: cast_nullable_to_non_nullable
              as ValueData,
    ));
  }

  @override
  @pragma('vm:prefer-inline')
  $ValueDataCopyWith<$Res> get value {
    return $ValueDataCopyWith<$Res>(_value.value, (value) {
      return _then(_value.copyWith(value: value));
    });
  }
}

/// @nodoc
@JsonSerializable()
class _$VeilidUpdateValueChange implements VeilidUpdateValueChange {
  const _$VeilidUpdateValueChange(
      {required this.key,
      required final List<ValueSubkeyRange> subkeys,
      required this.count,
      required this.value,
      final String? $type})
      : _subkeys = subkeys,
        $type = $type ?? 'ValueChange';

  factory _$VeilidUpdateValueChange.fromJson(Map<String, dynamic> json) =>
      _$$VeilidUpdateValueChangeFromJson(json);

  @override
  final Typed<FixedEncodedString43> key;
  final List<ValueSubkeyRange> _subkeys;
  @override
  List<ValueSubkeyRange> get subkeys {
    if (_subkeys is EqualUnmodifiableListView) return _subkeys;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_subkeys);
  }

  @override
  final int count;
  @override
  final ValueData value;

  @JsonKey(name: 'kind')
  final String $type;

  @override
  String toString() {
    return 'VeilidUpdate.valueChange(key: $key, subkeys: $subkeys, count: $count, value: $value)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$VeilidUpdateValueChange &&
            (identical(other.key, key) || other.key == key) &&
            const DeepCollectionEquality().equals(other._subkeys, _subkeys) &&
            (identical(other.count, count) || other.count == count) &&
            (identical(other.value, value) || other.value == value));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, key,
      const DeepCollectionEquality().hash(_subkeys), count, value);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$VeilidUpdateValueChangeCopyWith<_$VeilidUpdateValueChange> get copyWith =>
      __$$VeilidUpdateValueChangeCopyWithImpl<_$VeilidUpdateValueChange>(
          this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(
            VeilidLogLevel logLevel, String message, String? backtrace)
        log,
    required TResult Function(@Uint8ListJsonConverter() Uint8List message,
            Typed<FixedEncodedString43>? sender)
        appMessage,
    required TResult Function(@Uint8ListJsonConverter() Uint8List message,
            String callId, Typed<FixedEncodedString43>? sender)
        appCall,
    required TResult Function(AttachmentState state, bool publicInternetReady,
            bool localNetworkReady)
        attachment,
    required TResult Function(bool started, BigInt bpsDown, BigInt bpsUp,
            List<PeerTableData> peers)
        network,
    required TResult Function(VeilidConfig config) config,
    required TResult Function(
            List<String> deadRoutes, List<String> deadRemoteRoutes)
        routeChange,
    required TResult Function(Typed<FixedEncodedString43> key,
            List<ValueSubkeyRange> subkeys, int count, ValueData value)
        valueChange,
  }) {
    return valueChange(key, subkeys, count, value);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(
            VeilidLogLevel logLevel, String message, String? backtrace)?
        log,
    TResult? Function(@Uint8ListJsonConverter() Uint8List message,
            Typed<FixedEncodedString43>? sender)?
        appMessage,
    TResult? Function(@Uint8ListJsonConverter() Uint8List message,
            String callId, Typed<FixedEncodedString43>? sender)?
        appCall,
    TResult? Function(AttachmentState state, bool publicInternetReady,
            bool localNetworkReady)?
        attachment,
    TResult? Function(bool started, BigInt bpsDown, BigInt bpsUp,
            List<PeerTableData> peers)?
        network,
    TResult? Function(VeilidConfig config)? config,
    TResult? Function(List<String> deadRoutes, List<String> deadRemoteRoutes)?
        routeChange,
    TResult? Function(Typed<FixedEncodedString43> key,
            List<ValueSubkeyRange> subkeys, int count, ValueData value)?
        valueChange,
  }) {
    return valueChange?.call(key, subkeys, count, value);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(
            VeilidLogLevel logLevel, String message, String? backtrace)?
        log,
    TResult Function(@Uint8ListJsonConverter() Uint8List message,
            Typed<FixedEncodedString43>? sender)?
        appMessage,
    TResult Function(@Uint8ListJsonConverter() Uint8List message, String callId,
            Typed<FixedEncodedString43>? sender)?
        appCall,
    TResult Function(AttachmentState state, bool publicInternetReady,
            bool localNetworkReady)?
        attachment,
    TResult Function(bool started, BigInt bpsDown, BigInt bpsUp,
            List<PeerTableData> peers)?
        network,
    TResult Function(VeilidConfig config)? config,
    TResult Function(List<String> deadRoutes, List<String> deadRemoteRoutes)?
        routeChange,
    TResult Function(Typed<FixedEncodedString43> key,
            List<ValueSubkeyRange> subkeys, int count, ValueData value)?
        valueChange,
    required TResult orElse(),
  }) {
    if (valueChange != null) {
      return valueChange(key, subkeys, count, value);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(VeilidLog value) log,
    required TResult Function(VeilidAppMessage value) appMessage,
    required TResult Function(VeilidAppCall value) appCall,
    required TResult Function(VeilidUpdateAttachment value) attachment,
    required TResult Function(VeilidUpdateNetwork value) network,
    required TResult Function(VeilidUpdateConfig value) config,
    required TResult Function(VeilidUpdateRouteChange value) routeChange,
    required TResult Function(VeilidUpdateValueChange value) valueChange,
  }) {
    return valueChange(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(VeilidLog value)? log,
    TResult? Function(VeilidAppMessage value)? appMessage,
    TResult? Function(VeilidAppCall value)? appCall,
    TResult? Function(VeilidUpdateAttachment value)? attachment,
    TResult? Function(VeilidUpdateNetwork value)? network,
    TResult? Function(VeilidUpdateConfig value)? config,
    TResult? Function(VeilidUpdateRouteChange value)? routeChange,
    TResult? Function(VeilidUpdateValueChange value)? valueChange,
  }) {
    return valueChange?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(VeilidLog value)? log,
    TResult Function(VeilidAppMessage value)? appMessage,
    TResult Function(VeilidAppCall value)? appCall,
    TResult Function(VeilidUpdateAttachment value)? attachment,
    TResult Function(VeilidUpdateNetwork value)? network,
    TResult Function(VeilidUpdateConfig value)? config,
    TResult Function(VeilidUpdateRouteChange value)? routeChange,
    TResult Function(VeilidUpdateValueChange value)? valueChange,
    required TResult orElse(),
  }) {
    if (valueChange != null) {
      return valueChange(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$VeilidUpdateValueChangeToJson(
      this,
    );
  }
}

abstract class VeilidUpdateValueChange implements VeilidUpdate {
  const factory VeilidUpdateValueChange(
      {required final Typed<FixedEncodedString43> key,
      required final List<ValueSubkeyRange> subkeys,
      required final int count,
      required final ValueData value}) = _$VeilidUpdateValueChange;

  factory VeilidUpdateValueChange.fromJson(Map<String, dynamic> json) =
      _$VeilidUpdateValueChange.fromJson;

  Typed<FixedEncodedString43> get key;
  List<ValueSubkeyRange> get subkeys;
  int get count;
  ValueData get value;
  @JsonKey(ignore: true)
  _$$VeilidUpdateValueChangeCopyWith<_$VeilidUpdateValueChange> get copyWith =>
      throw _privateConstructorUsedError;
}

VeilidStateAttachment _$VeilidStateAttachmentFromJson(
    Map<String, dynamic> json) {
  return _VeilidStateAttachment.fromJson(json);
}

/// @nodoc
mixin _$VeilidStateAttachment {
  AttachmentState get state => throw _privateConstructorUsedError;
  bool get publicInternetReady => throw _privateConstructorUsedError;
  bool get localNetworkReady => throw _privateConstructorUsedError;

  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
  @JsonKey(ignore: true)
  $VeilidStateAttachmentCopyWith<VeilidStateAttachment> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $VeilidStateAttachmentCopyWith<$Res> {
  factory $VeilidStateAttachmentCopyWith(VeilidStateAttachment value,
          $Res Function(VeilidStateAttachment) then) =
      _$VeilidStateAttachmentCopyWithImpl<$Res, VeilidStateAttachment>;
  @useResult
  $Res call(
      {AttachmentState state,
      bool publicInternetReady,
      bool localNetworkReady});
}

/// @nodoc
class _$VeilidStateAttachmentCopyWithImpl<$Res,
        $Val extends VeilidStateAttachment>
    implements $VeilidStateAttachmentCopyWith<$Res> {
  _$VeilidStateAttachmentCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? state = null,
    Object? publicInternetReady = null,
    Object? localNetworkReady = null,
  }) {
    return _then(_value.copyWith(
      state: null == state
          ? _value.state
          : state // ignore: cast_nullable_to_non_nullable
              as AttachmentState,
      publicInternetReady: null == publicInternetReady
          ? _value.publicInternetReady
          : publicInternetReady // ignore: cast_nullable_to_non_nullable
              as bool,
      localNetworkReady: null == localNetworkReady
          ? _value.localNetworkReady
          : localNetworkReady // ignore: cast_nullable_to_non_nullable
              as bool,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$_VeilidStateAttachmentCopyWith<$Res>
    implements $VeilidStateAttachmentCopyWith<$Res> {
  factory _$$_VeilidStateAttachmentCopyWith(_$_VeilidStateAttachment value,
          $Res Function(_$_VeilidStateAttachment) then) =
      __$$_VeilidStateAttachmentCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {AttachmentState state,
      bool publicInternetReady,
      bool localNetworkReady});
}

/// @nodoc
class __$$_VeilidStateAttachmentCopyWithImpl<$Res>
    extends _$VeilidStateAttachmentCopyWithImpl<$Res, _$_VeilidStateAttachment>
    implements _$$_VeilidStateAttachmentCopyWith<$Res> {
  __$$_VeilidStateAttachmentCopyWithImpl(_$_VeilidStateAttachment _value,
      $Res Function(_$_VeilidStateAttachment) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? state = null,
    Object? publicInternetReady = null,
    Object? localNetworkReady = null,
  }) {
    return _then(_$_VeilidStateAttachment(
      state: null == state
          ? _value.state
          : state // ignore: cast_nullable_to_non_nullable
              as AttachmentState,
      publicInternetReady: null == publicInternetReady
          ? _value.publicInternetReady
          : publicInternetReady // ignore: cast_nullable_to_non_nullable
              as bool,
      localNetworkReady: null == localNetworkReady
          ? _value.localNetworkReady
          : localNetworkReady // ignore: cast_nullable_to_non_nullable
              as bool,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$_VeilidStateAttachment implements _VeilidStateAttachment {
  const _$_VeilidStateAttachment(
      {required this.state,
      required this.publicInternetReady,
      required this.localNetworkReady});

  factory _$_VeilidStateAttachment.fromJson(Map<String, dynamic> json) =>
      _$$_VeilidStateAttachmentFromJson(json);

  @override
  final AttachmentState state;
  @override
  final bool publicInternetReady;
  @override
  final bool localNetworkReady;

  @override
  String toString() {
    return 'VeilidStateAttachment(state: $state, publicInternetReady: $publicInternetReady, localNetworkReady: $localNetworkReady)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$_VeilidStateAttachment &&
            (identical(other.state, state) || other.state == state) &&
            (identical(other.publicInternetReady, publicInternetReady) ||
                other.publicInternetReady == publicInternetReady) &&
            (identical(other.localNetworkReady, localNetworkReady) ||
                other.localNetworkReady == localNetworkReady));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode =>
      Object.hash(runtimeType, state, publicInternetReady, localNetworkReady);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$_VeilidStateAttachmentCopyWith<_$_VeilidStateAttachment> get copyWith =>
      __$$_VeilidStateAttachmentCopyWithImpl<_$_VeilidStateAttachment>(
          this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$_VeilidStateAttachmentToJson(
      this,
    );
  }
}

abstract class _VeilidStateAttachment implements VeilidStateAttachment {
  const factory _VeilidStateAttachment(
      {required final AttachmentState state,
      required final bool publicInternetReady,
      required final bool localNetworkReady}) = _$_VeilidStateAttachment;

  factory _VeilidStateAttachment.fromJson(Map<String, dynamic> json) =
      _$_VeilidStateAttachment.fromJson;

  @override
  AttachmentState get state;
  @override
  bool get publicInternetReady;
  @override
  bool get localNetworkReady;
  @override
  @JsonKey(ignore: true)
  _$$_VeilidStateAttachmentCopyWith<_$_VeilidStateAttachment> get copyWith =>
      throw _privateConstructorUsedError;
}

VeilidStateNetwork _$VeilidStateNetworkFromJson(Map<String, dynamic> json) {
  return _VeilidStateNetwork.fromJson(json);
}

/// @nodoc
mixin _$VeilidStateNetwork {
  bool get started => throw _privateConstructorUsedError;
  BigInt get bpsDown => throw _privateConstructorUsedError;
  BigInt get bpsUp => throw _privateConstructorUsedError;
  List<PeerTableData> get peers => throw _privateConstructorUsedError;

  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
  @JsonKey(ignore: true)
  $VeilidStateNetworkCopyWith<VeilidStateNetwork> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $VeilidStateNetworkCopyWith<$Res> {
  factory $VeilidStateNetworkCopyWith(
          VeilidStateNetwork value, $Res Function(VeilidStateNetwork) then) =
      _$VeilidStateNetworkCopyWithImpl<$Res, VeilidStateNetwork>;
  @useResult
  $Res call(
      {bool started, BigInt bpsDown, BigInt bpsUp, List<PeerTableData> peers});
}

/// @nodoc
class _$VeilidStateNetworkCopyWithImpl<$Res, $Val extends VeilidStateNetwork>
    implements $VeilidStateNetworkCopyWith<$Res> {
  _$VeilidStateNetworkCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? started = null,
    Object? bpsDown = null,
    Object? bpsUp = null,
    Object? peers = null,
  }) {
    return _then(_value.copyWith(
      started: null == started
          ? _value.started
          : started // ignore: cast_nullable_to_non_nullable
              as bool,
      bpsDown: null == bpsDown
          ? _value.bpsDown
          : bpsDown // ignore: cast_nullable_to_non_nullable
              as BigInt,
      bpsUp: null == bpsUp
          ? _value.bpsUp
          : bpsUp // ignore: cast_nullable_to_non_nullable
              as BigInt,
      peers: null == peers
          ? _value.peers
          : peers // ignore: cast_nullable_to_non_nullable
              as List<PeerTableData>,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$_VeilidStateNetworkCopyWith<$Res>
    implements $VeilidStateNetworkCopyWith<$Res> {
  factory _$$_VeilidStateNetworkCopyWith(_$_VeilidStateNetwork value,
          $Res Function(_$_VeilidStateNetwork) then) =
      __$$_VeilidStateNetworkCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {bool started, BigInt bpsDown, BigInt bpsUp, List<PeerTableData> peers});
}

/// @nodoc
class __$$_VeilidStateNetworkCopyWithImpl<$Res>
    extends _$VeilidStateNetworkCopyWithImpl<$Res, _$_VeilidStateNetwork>
    implements _$$_VeilidStateNetworkCopyWith<$Res> {
  __$$_VeilidStateNetworkCopyWithImpl(
      _$_VeilidStateNetwork _value, $Res Function(_$_VeilidStateNetwork) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? started = null,
    Object? bpsDown = null,
    Object? bpsUp = null,
    Object? peers = null,
  }) {
    return _then(_$_VeilidStateNetwork(
      started: null == started
          ? _value.started
          : started // ignore: cast_nullable_to_non_nullable
              as bool,
      bpsDown: null == bpsDown
          ? _value.bpsDown
          : bpsDown // ignore: cast_nullable_to_non_nullable
              as BigInt,
      bpsUp: null == bpsUp
          ? _value.bpsUp
          : bpsUp // ignore: cast_nullable_to_non_nullable
              as BigInt,
      peers: null == peers
          ? _value._peers
          : peers // ignore: cast_nullable_to_non_nullable
              as List<PeerTableData>,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$_VeilidStateNetwork implements _VeilidStateNetwork {
  const _$_VeilidStateNetwork(
      {required this.started,
      required this.bpsDown,
      required this.bpsUp,
      required final List<PeerTableData> peers})
      : _peers = peers;

  factory _$_VeilidStateNetwork.fromJson(Map<String, dynamic> json) =>
      _$$_VeilidStateNetworkFromJson(json);

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

  @override
  String toString() {
    return 'VeilidStateNetwork(started: $started, bpsDown: $bpsDown, bpsUp: $bpsUp, peers: $peers)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$_VeilidStateNetwork &&
            (identical(other.started, started) || other.started == started) &&
            (identical(other.bpsDown, bpsDown) || other.bpsDown == bpsDown) &&
            (identical(other.bpsUp, bpsUp) || other.bpsUp == bpsUp) &&
            const DeepCollectionEquality().equals(other._peers, _peers));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, started, bpsDown, bpsUp,
      const DeepCollectionEquality().hash(_peers));

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$_VeilidStateNetworkCopyWith<_$_VeilidStateNetwork> get copyWith =>
      __$$_VeilidStateNetworkCopyWithImpl<_$_VeilidStateNetwork>(
          this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$_VeilidStateNetworkToJson(
      this,
    );
  }
}

abstract class _VeilidStateNetwork implements VeilidStateNetwork {
  const factory _VeilidStateNetwork(
      {required final bool started,
      required final BigInt bpsDown,
      required final BigInt bpsUp,
      required final List<PeerTableData> peers}) = _$_VeilidStateNetwork;

  factory _VeilidStateNetwork.fromJson(Map<String, dynamic> json) =
      _$_VeilidStateNetwork.fromJson;

  @override
  bool get started;
  @override
  BigInt get bpsDown;
  @override
  BigInt get bpsUp;
  @override
  List<PeerTableData> get peers;
  @override
  @JsonKey(ignore: true)
  _$$_VeilidStateNetworkCopyWith<_$_VeilidStateNetwork> get copyWith =>
      throw _privateConstructorUsedError;
}

VeilidStateConfig _$VeilidStateConfigFromJson(Map<String, dynamic> json) {
  return _VeilidStateConfig.fromJson(json);
}

/// @nodoc
mixin _$VeilidStateConfig {
  VeilidConfig get config => throw _privateConstructorUsedError;

  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
  @JsonKey(ignore: true)
  $VeilidStateConfigCopyWith<VeilidStateConfig> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $VeilidStateConfigCopyWith<$Res> {
  factory $VeilidStateConfigCopyWith(
          VeilidStateConfig value, $Res Function(VeilidStateConfig) then) =
      _$VeilidStateConfigCopyWithImpl<$Res, VeilidStateConfig>;
  @useResult
  $Res call({VeilidConfig config});

  $VeilidConfigCopyWith<$Res> get config;
}

/// @nodoc
class _$VeilidStateConfigCopyWithImpl<$Res, $Val extends VeilidStateConfig>
    implements $VeilidStateConfigCopyWith<$Res> {
  _$VeilidStateConfigCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? config = null,
  }) {
    return _then(_value.copyWith(
      config: null == config
          ? _value.config
          : config // ignore: cast_nullable_to_non_nullable
              as VeilidConfig,
    ) as $Val);
  }

  @override
  @pragma('vm:prefer-inline')
  $VeilidConfigCopyWith<$Res> get config {
    return $VeilidConfigCopyWith<$Res>(_value.config, (value) {
      return _then(_value.copyWith(config: value) as $Val);
    });
  }
}

/// @nodoc
abstract class _$$_VeilidStateConfigCopyWith<$Res>
    implements $VeilidStateConfigCopyWith<$Res> {
  factory _$$_VeilidStateConfigCopyWith(_$_VeilidStateConfig value,
          $Res Function(_$_VeilidStateConfig) then) =
      __$$_VeilidStateConfigCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({VeilidConfig config});

  @override
  $VeilidConfigCopyWith<$Res> get config;
}

/// @nodoc
class __$$_VeilidStateConfigCopyWithImpl<$Res>
    extends _$VeilidStateConfigCopyWithImpl<$Res, _$_VeilidStateConfig>
    implements _$$_VeilidStateConfigCopyWith<$Res> {
  __$$_VeilidStateConfigCopyWithImpl(
      _$_VeilidStateConfig _value, $Res Function(_$_VeilidStateConfig) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? config = null,
  }) {
    return _then(_$_VeilidStateConfig(
      config: null == config
          ? _value.config
          : config // ignore: cast_nullable_to_non_nullable
              as VeilidConfig,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$_VeilidStateConfig implements _VeilidStateConfig {
  const _$_VeilidStateConfig({required this.config});

  factory _$_VeilidStateConfig.fromJson(Map<String, dynamic> json) =>
      _$$_VeilidStateConfigFromJson(json);

  @override
  final VeilidConfig config;

  @override
  String toString() {
    return 'VeilidStateConfig(config: $config)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$_VeilidStateConfig &&
            (identical(other.config, config) || other.config == config));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, config);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$_VeilidStateConfigCopyWith<_$_VeilidStateConfig> get copyWith =>
      __$$_VeilidStateConfigCopyWithImpl<_$_VeilidStateConfig>(
          this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$_VeilidStateConfigToJson(
      this,
    );
  }
}

abstract class _VeilidStateConfig implements VeilidStateConfig {
  const factory _VeilidStateConfig({required final VeilidConfig config}) =
      _$_VeilidStateConfig;

  factory _VeilidStateConfig.fromJson(Map<String, dynamic> json) =
      _$_VeilidStateConfig.fromJson;

  @override
  VeilidConfig get config;
  @override
  @JsonKey(ignore: true)
  _$$_VeilidStateConfigCopyWith<_$_VeilidStateConfig> get copyWith =>
      throw _privateConstructorUsedError;
}

VeilidState _$VeilidStateFromJson(Map<String, dynamic> json) {
  return _VeilidState.fromJson(json);
}

/// @nodoc
mixin _$VeilidState {
  VeilidStateAttachment get attachment => throw _privateConstructorUsedError;
  VeilidStateNetwork get network => throw _privateConstructorUsedError;
  VeilidStateConfig get config => throw _privateConstructorUsedError;

  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
  @JsonKey(ignore: true)
  $VeilidStateCopyWith<VeilidState> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $VeilidStateCopyWith<$Res> {
  factory $VeilidStateCopyWith(
          VeilidState value, $Res Function(VeilidState) then) =
      _$VeilidStateCopyWithImpl<$Res, VeilidState>;
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
class _$VeilidStateCopyWithImpl<$Res, $Val extends VeilidState>
    implements $VeilidStateCopyWith<$Res> {
  _$VeilidStateCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? attachment = null,
    Object? network = null,
    Object? config = null,
  }) {
    return _then(_value.copyWith(
      attachment: null == attachment
          ? _value.attachment
          : attachment // ignore: cast_nullable_to_non_nullable
              as VeilidStateAttachment,
      network: null == network
          ? _value.network
          : network // ignore: cast_nullable_to_non_nullable
              as VeilidStateNetwork,
      config: null == config
          ? _value.config
          : config // ignore: cast_nullable_to_non_nullable
              as VeilidStateConfig,
    ) as $Val);
  }

  @override
  @pragma('vm:prefer-inline')
  $VeilidStateAttachmentCopyWith<$Res> get attachment {
    return $VeilidStateAttachmentCopyWith<$Res>(_value.attachment, (value) {
      return _then(_value.copyWith(attachment: value) as $Val);
    });
  }

  @override
  @pragma('vm:prefer-inline')
  $VeilidStateNetworkCopyWith<$Res> get network {
    return $VeilidStateNetworkCopyWith<$Res>(_value.network, (value) {
      return _then(_value.copyWith(network: value) as $Val);
    });
  }

  @override
  @pragma('vm:prefer-inline')
  $VeilidStateConfigCopyWith<$Res> get config {
    return $VeilidStateConfigCopyWith<$Res>(_value.config, (value) {
      return _then(_value.copyWith(config: value) as $Val);
    });
  }
}

/// @nodoc
abstract class _$$_VeilidStateCopyWith<$Res>
    implements $VeilidStateCopyWith<$Res> {
  factory _$$_VeilidStateCopyWith(
          _$_VeilidState value, $Res Function(_$_VeilidState) then) =
      __$$_VeilidStateCopyWithImpl<$Res>;
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
class __$$_VeilidStateCopyWithImpl<$Res>
    extends _$VeilidStateCopyWithImpl<$Res, _$_VeilidState>
    implements _$$_VeilidStateCopyWith<$Res> {
  __$$_VeilidStateCopyWithImpl(
      _$_VeilidState _value, $Res Function(_$_VeilidState) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? attachment = null,
    Object? network = null,
    Object? config = null,
  }) {
    return _then(_$_VeilidState(
      attachment: null == attachment
          ? _value.attachment
          : attachment // ignore: cast_nullable_to_non_nullable
              as VeilidStateAttachment,
      network: null == network
          ? _value.network
          : network // ignore: cast_nullable_to_non_nullable
              as VeilidStateNetwork,
      config: null == config
          ? _value.config
          : config // ignore: cast_nullable_to_non_nullable
              as VeilidStateConfig,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$_VeilidState implements _VeilidState {
  const _$_VeilidState(
      {required this.attachment, required this.network, required this.config});

  factory _$_VeilidState.fromJson(Map<String, dynamic> json) =>
      _$$_VeilidStateFromJson(json);

  @override
  final VeilidStateAttachment attachment;
  @override
  final VeilidStateNetwork network;
  @override
  final VeilidStateConfig config;

  @override
  String toString() {
    return 'VeilidState(attachment: $attachment, network: $network, config: $config)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$_VeilidState &&
            (identical(other.attachment, attachment) ||
                other.attachment == attachment) &&
            (identical(other.network, network) || other.network == network) &&
            (identical(other.config, config) || other.config == config));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, attachment, network, config);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$_VeilidStateCopyWith<_$_VeilidState> get copyWith =>
      __$$_VeilidStateCopyWithImpl<_$_VeilidState>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$_VeilidStateToJson(
      this,
    );
  }
}

abstract class _VeilidState implements VeilidState {
  const factory _VeilidState(
      {required final VeilidStateAttachment attachment,
      required final VeilidStateNetwork network,
      required final VeilidStateConfig config}) = _$_VeilidState;

  factory _VeilidState.fromJson(Map<String, dynamic> json) =
      _$_VeilidState.fromJson;

  @override
  VeilidStateAttachment get attachment;
  @override
  VeilidStateNetwork get network;
  @override
  VeilidStateConfig get config;
  @override
  @JsonKey(ignore: true)
  _$$_VeilidStateCopyWith<_$_VeilidState> get copyWith =>
      throw _privateConstructorUsedError;
}
