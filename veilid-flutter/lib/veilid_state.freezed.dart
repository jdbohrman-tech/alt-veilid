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
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#adding-getters-and-methods-to-our-models');

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
abstract class _$$LatencyStatsImplCopyWith<$Res>
    implements $LatencyStatsCopyWith<$Res> {
  factory _$$LatencyStatsImplCopyWith(
          _$LatencyStatsImpl value, $Res Function(_$LatencyStatsImpl) then) =
      __$$LatencyStatsImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {TimestampDuration fastest,
      TimestampDuration average,
      TimestampDuration slowest});
}

/// @nodoc
class __$$LatencyStatsImplCopyWithImpl<$Res>
    extends _$LatencyStatsCopyWithImpl<$Res, _$LatencyStatsImpl>
    implements _$$LatencyStatsImplCopyWith<$Res> {
  __$$LatencyStatsImplCopyWithImpl(
      _$LatencyStatsImpl _value, $Res Function(_$LatencyStatsImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? fastest = null,
    Object? average = null,
    Object? slowest = null,
  }) {
    return _then(_$LatencyStatsImpl(
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
class _$LatencyStatsImpl implements _LatencyStats {
  const _$LatencyStatsImpl(
      {required this.fastest, required this.average, required this.slowest});

  factory _$LatencyStatsImpl.fromJson(Map<String, dynamic> json) =>
      _$$LatencyStatsImplFromJson(json);

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
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LatencyStatsImpl &&
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
  _$$LatencyStatsImplCopyWith<_$LatencyStatsImpl> get copyWith =>
      __$$LatencyStatsImplCopyWithImpl<_$LatencyStatsImpl>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$LatencyStatsImplToJson(
      this,
    );
  }
}

abstract class _LatencyStats implements LatencyStats {
  const factory _LatencyStats(
      {required final TimestampDuration fastest,
      required final TimestampDuration average,
      required final TimestampDuration slowest}) = _$LatencyStatsImpl;

  factory _LatencyStats.fromJson(Map<String, dynamic> json) =
      _$LatencyStatsImpl.fromJson;

  @override
  TimestampDuration get fastest;
  @override
  TimestampDuration get average;
  @override
  TimestampDuration get slowest;
  @override
  @JsonKey(ignore: true)
  _$$LatencyStatsImplCopyWith<_$LatencyStatsImpl> get copyWith =>
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
abstract class _$$TransferStatsImplCopyWith<$Res>
    implements $TransferStatsCopyWith<$Res> {
  factory _$$TransferStatsImplCopyWith(
          _$TransferStatsImpl value, $Res Function(_$TransferStatsImpl) then) =
      __$$TransferStatsImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({BigInt total, BigInt maximum, BigInt average, BigInt minimum});
}

/// @nodoc
class __$$TransferStatsImplCopyWithImpl<$Res>
    extends _$TransferStatsCopyWithImpl<$Res, _$TransferStatsImpl>
    implements _$$TransferStatsImplCopyWith<$Res> {
  __$$TransferStatsImplCopyWithImpl(
      _$TransferStatsImpl _value, $Res Function(_$TransferStatsImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? total = null,
    Object? maximum = null,
    Object? average = null,
    Object? minimum = null,
  }) {
    return _then(_$TransferStatsImpl(
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
class _$TransferStatsImpl implements _TransferStats {
  const _$TransferStatsImpl(
      {required this.total,
      required this.maximum,
      required this.average,
      required this.minimum});

  factory _$TransferStatsImpl.fromJson(Map<String, dynamic> json) =>
      _$$TransferStatsImplFromJson(json);

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
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$TransferStatsImpl &&
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
  _$$TransferStatsImplCopyWith<_$TransferStatsImpl> get copyWith =>
      __$$TransferStatsImplCopyWithImpl<_$TransferStatsImpl>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$TransferStatsImplToJson(
      this,
    );
  }
}

abstract class _TransferStats implements TransferStats {
  const factory _TransferStats(
      {required final BigInt total,
      required final BigInt maximum,
      required final BigInt average,
      required final BigInt minimum}) = _$TransferStatsImpl;

  factory _TransferStats.fromJson(Map<String, dynamic> json) =
      _$TransferStatsImpl.fromJson;

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
  _$$TransferStatsImplCopyWith<_$TransferStatsImpl> get copyWith =>
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
abstract class _$$TransferStatsDownUpImplCopyWith<$Res>
    implements $TransferStatsDownUpCopyWith<$Res> {
  factory _$$TransferStatsDownUpImplCopyWith(_$TransferStatsDownUpImpl value,
          $Res Function(_$TransferStatsDownUpImpl) then) =
      __$$TransferStatsDownUpImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({TransferStats down, TransferStats up});

  @override
  $TransferStatsCopyWith<$Res> get down;
  @override
  $TransferStatsCopyWith<$Res> get up;
}

/// @nodoc
class __$$TransferStatsDownUpImplCopyWithImpl<$Res>
    extends _$TransferStatsDownUpCopyWithImpl<$Res, _$TransferStatsDownUpImpl>
    implements _$$TransferStatsDownUpImplCopyWith<$Res> {
  __$$TransferStatsDownUpImplCopyWithImpl(_$TransferStatsDownUpImpl _value,
      $Res Function(_$TransferStatsDownUpImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? down = null,
    Object? up = null,
  }) {
    return _then(_$TransferStatsDownUpImpl(
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
class _$TransferStatsDownUpImpl implements _TransferStatsDownUp {
  const _$TransferStatsDownUpImpl({required this.down, required this.up});

  factory _$TransferStatsDownUpImpl.fromJson(Map<String, dynamic> json) =>
      _$$TransferStatsDownUpImplFromJson(json);

  @override
  final TransferStats down;
  @override
  final TransferStats up;

  @override
  String toString() {
    return 'TransferStatsDownUp(down: $down, up: $up)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$TransferStatsDownUpImpl &&
            (identical(other.down, down) || other.down == down) &&
            (identical(other.up, up) || other.up == up));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, down, up);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$TransferStatsDownUpImplCopyWith<_$TransferStatsDownUpImpl> get copyWith =>
      __$$TransferStatsDownUpImplCopyWithImpl<_$TransferStatsDownUpImpl>(
          this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$TransferStatsDownUpImplToJson(
      this,
    );
  }
}

abstract class _TransferStatsDownUp implements TransferStatsDownUp {
  const factory _TransferStatsDownUp(
      {required final TransferStats down,
      required final TransferStats up}) = _$TransferStatsDownUpImpl;

  factory _TransferStatsDownUp.fromJson(Map<String, dynamic> json) =
      _$TransferStatsDownUpImpl.fromJson;

  @override
  TransferStats get down;
  @override
  TransferStats get up;
  @override
  @JsonKey(ignore: true)
  _$$TransferStatsDownUpImplCopyWith<_$TransferStatsDownUpImpl> get copyWith =>
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
abstract class _$$RPCStatsImplCopyWith<$Res>
    implements $RPCStatsCopyWith<$Res> {
  factory _$$RPCStatsImplCopyWith(
          _$RPCStatsImpl value, $Res Function(_$RPCStatsImpl) then) =
      __$$RPCStatsImplCopyWithImpl<$Res>;
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
class __$$RPCStatsImplCopyWithImpl<$Res>
    extends _$RPCStatsCopyWithImpl<$Res, _$RPCStatsImpl>
    implements _$$RPCStatsImplCopyWith<$Res> {
  __$$RPCStatsImplCopyWithImpl(
      _$RPCStatsImpl _value, $Res Function(_$RPCStatsImpl) _then)
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
    return _then(_$RPCStatsImpl(
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
class _$RPCStatsImpl implements _RPCStats {
  const _$RPCStatsImpl(
      {required this.messagesSent,
      required this.messagesRcvd,
      required this.questionsInFlight,
      required this.lastQuestion,
      required this.lastSeenTs,
      required this.firstConsecutiveSeenTs,
      required this.recentLostAnswers,
      required this.failedToSend});

  factory _$RPCStatsImpl.fromJson(Map<String, dynamic> json) =>
      _$$RPCStatsImplFromJson(json);

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
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$RPCStatsImpl &&
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
  _$$RPCStatsImplCopyWith<_$RPCStatsImpl> get copyWith =>
      __$$RPCStatsImplCopyWithImpl<_$RPCStatsImpl>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$RPCStatsImplToJson(
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
      required final int failedToSend}) = _$RPCStatsImpl;

  factory _RPCStats.fromJson(Map<String, dynamic> json) =
      _$RPCStatsImpl.fromJson;

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
  _$$RPCStatsImplCopyWith<_$RPCStatsImpl> get copyWith =>
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
abstract class _$$PeerStatsImplCopyWith<$Res>
    implements $PeerStatsCopyWith<$Res> {
  factory _$$PeerStatsImplCopyWith(
          _$PeerStatsImpl value, $Res Function(_$PeerStatsImpl) then) =
      __$$PeerStatsImplCopyWithImpl<$Res>;
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
class __$$PeerStatsImplCopyWithImpl<$Res>
    extends _$PeerStatsCopyWithImpl<$Res, _$PeerStatsImpl>
    implements _$$PeerStatsImplCopyWith<$Res> {
  __$$PeerStatsImplCopyWithImpl(
      _$PeerStatsImpl _value, $Res Function(_$PeerStatsImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? timeAdded = null,
    Object? rpcStats = null,
    Object? transfer = null,
    Object? latency = freezed,
  }) {
    return _then(_$PeerStatsImpl(
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
class _$PeerStatsImpl implements _PeerStats {
  const _$PeerStatsImpl(
      {required this.timeAdded,
      required this.rpcStats,
      required this.transfer,
      this.latency});

  factory _$PeerStatsImpl.fromJson(Map<String, dynamic> json) =>
      _$$PeerStatsImplFromJson(json);

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
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$PeerStatsImpl &&
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
  _$$PeerStatsImplCopyWith<_$PeerStatsImpl> get copyWith =>
      __$$PeerStatsImplCopyWithImpl<_$PeerStatsImpl>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$PeerStatsImplToJson(
      this,
    );
  }
}

abstract class _PeerStats implements PeerStats {
  const factory _PeerStats(
      {required final Timestamp timeAdded,
      required final RPCStats rpcStats,
      required final TransferStatsDownUp transfer,
      final LatencyStats? latency}) = _$PeerStatsImpl;

  factory _PeerStats.fromJson(Map<String, dynamic> json) =
      _$PeerStatsImpl.fromJson;

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
  _$$PeerStatsImplCopyWith<_$PeerStatsImpl> get copyWith =>
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
abstract class _$$PeerTableDataImplCopyWith<$Res>
    implements $PeerTableDataCopyWith<$Res> {
  factory _$$PeerTableDataImplCopyWith(
          _$PeerTableDataImpl value, $Res Function(_$PeerTableDataImpl) then) =
      __$$PeerTableDataImplCopyWithImpl<$Res>;
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
class __$$PeerTableDataImplCopyWithImpl<$Res>
    extends _$PeerTableDataCopyWithImpl<$Res, _$PeerTableDataImpl>
    implements _$$PeerTableDataImplCopyWith<$Res> {
  __$$PeerTableDataImplCopyWithImpl(
      _$PeerTableDataImpl _value, $Res Function(_$PeerTableDataImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? nodeIds = null,
    Object? peerAddress = null,
    Object? peerStats = null,
  }) {
    return _then(_$PeerTableDataImpl(
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
class _$PeerTableDataImpl implements _PeerTableData {
  const _$PeerTableDataImpl(
      {required final List<Typed<FixedEncodedString43>> nodeIds,
      required this.peerAddress,
      required this.peerStats})
      : _nodeIds = nodeIds;

  factory _$PeerTableDataImpl.fromJson(Map<String, dynamic> json) =>
      _$$PeerTableDataImplFromJson(json);

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
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$PeerTableDataImpl &&
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
  _$$PeerTableDataImplCopyWith<_$PeerTableDataImpl> get copyWith =>
      __$$PeerTableDataImplCopyWithImpl<_$PeerTableDataImpl>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$PeerTableDataImplToJson(
      this,
    );
  }
}

abstract class _PeerTableData implements PeerTableData {
  const factory _PeerTableData(
      {required final List<Typed<FixedEncodedString43>> nodeIds,
      required final String peerAddress,
      required final PeerStats peerStats}) = _$PeerTableDataImpl;

  factory _PeerTableData.fromJson(Map<String, dynamic> json) =
      _$PeerTableDataImpl.fromJson;

  @override
  List<Typed<FixedEncodedString43>> get nodeIds;
  @override
  String get peerAddress;
  @override
  PeerStats get peerStats;
  @override
  @JsonKey(ignore: true)
  _$$PeerTableDataImplCopyWith<_$PeerTableDataImpl> get copyWith =>
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
abstract class _$$VeilidLogImplCopyWith<$Res> {
  factory _$$VeilidLogImplCopyWith(
          _$VeilidLogImpl value, $Res Function(_$VeilidLogImpl) then) =
      __$$VeilidLogImplCopyWithImpl<$Res>;
  @useResult
  $Res call({VeilidLogLevel logLevel, String message, String? backtrace});
}

/// @nodoc
class __$$VeilidLogImplCopyWithImpl<$Res>
    extends _$VeilidUpdateCopyWithImpl<$Res, _$VeilidLogImpl>
    implements _$$VeilidLogImplCopyWith<$Res> {
  __$$VeilidLogImplCopyWithImpl(
      _$VeilidLogImpl _value, $Res Function(_$VeilidLogImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? logLevel = null,
    Object? message = null,
    Object? backtrace = freezed,
  }) {
    return _then(_$VeilidLogImpl(
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
class _$VeilidLogImpl implements VeilidLog {
  const _$VeilidLogImpl(
      {required this.logLevel,
      required this.message,
      this.backtrace,
      final String? $type})
      : $type = $type ?? 'Log';

  factory _$VeilidLogImpl.fromJson(Map<String, dynamic> json) =>
      _$$VeilidLogImplFromJson(json);

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
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$VeilidLogImpl &&
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
  _$$VeilidLogImplCopyWith<_$VeilidLogImpl> get copyWith =>
      __$$VeilidLogImplCopyWithImpl<_$VeilidLogImpl>(this, _$identity);

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
    return _$$VeilidLogImplToJson(
      this,
    );
  }
}

abstract class VeilidLog implements VeilidUpdate {
  const factory VeilidLog(
      {required final VeilidLogLevel logLevel,
      required final String message,
      final String? backtrace}) = _$VeilidLogImpl;

  factory VeilidLog.fromJson(Map<String, dynamic> json) =
      _$VeilidLogImpl.fromJson;

  VeilidLogLevel get logLevel;
  String get message;
  String? get backtrace;
  @JsonKey(ignore: true)
  _$$VeilidLogImplCopyWith<_$VeilidLogImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$VeilidAppMessageImplCopyWith<$Res> {
  factory _$$VeilidAppMessageImplCopyWith(_$VeilidAppMessageImpl value,
          $Res Function(_$VeilidAppMessageImpl) then) =
      __$$VeilidAppMessageImplCopyWithImpl<$Res>;
  @useResult
  $Res call(
      {@Uint8ListJsonConverter() Uint8List message,
      Typed<FixedEncodedString43>? sender});
}

/// @nodoc
class __$$VeilidAppMessageImplCopyWithImpl<$Res>
    extends _$VeilidUpdateCopyWithImpl<$Res, _$VeilidAppMessageImpl>
    implements _$$VeilidAppMessageImplCopyWith<$Res> {
  __$$VeilidAppMessageImplCopyWithImpl(_$VeilidAppMessageImpl _value,
      $Res Function(_$VeilidAppMessageImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? message = null,
    Object? sender = freezed,
  }) {
    return _then(_$VeilidAppMessageImpl(
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
class _$VeilidAppMessageImpl implements VeilidAppMessage {
  const _$VeilidAppMessageImpl(
      {@Uint8ListJsonConverter() required this.message,
      this.sender,
      final String? $type})
      : $type = $type ?? 'AppMessage';

  factory _$VeilidAppMessageImpl.fromJson(Map<String, dynamic> json) =>
      _$$VeilidAppMessageImplFromJson(json);

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
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$VeilidAppMessageImpl &&
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
  _$$VeilidAppMessageImplCopyWith<_$VeilidAppMessageImpl> get copyWith =>
      __$$VeilidAppMessageImplCopyWithImpl<_$VeilidAppMessageImpl>(
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
    return _$$VeilidAppMessageImplToJson(
      this,
    );
  }
}

abstract class VeilidAppMessage implements VeilidUpdate {
  const factory VeilidAppMessage(
      {@Uint8ListJsonConverter() required final Uint8List message,
      final Typed<FixedEncodedString43>? sender}) = _$VeilidAppMessageImpl;

  factory VeilidAppMessage.fromJson(Map<String, dynamic> json) =
      _$VeilidAppMessageImpl.fromJson;

  @Uint8ListJsonConverter()
  Uint8List get message;
  Typed<FixedEncodedString43>? get sender;
  @JsonKey(ignore: true)
  _$$VeilidAppMessageImplCopyWith<_$VeilidAppMessageImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$VeilidAppCallImplCopyWith<$Res> {
  factory _$$VeilidAppCallImplCopyWith(
          _$VeilidAppCallImpl value, $Res Function(_$VeilidAppCallImpl) then) =
      __$$VeilidAppCallImplCopyWithImpl<$Res>;
  @useResult
  $Res call(
      {@Uint8ListJsonConverter() Uint8List message,
      String callId,
      Typed<FixedEncodedString43>? sender});
}

/// @nodoc
class __$$VeilidAppCallImplCopyWithImpl<$Res>
    extends _$VeilidUpdateCopyWithImpl<$Res, _$VeilidAppCallImpl>
    implements _$$VeilidAppCallImplCopyWith<$Res> {
  __$$VeilidAppCallImplCopyWithImpl(
      _$VeilidAppCallImpl _value, $Res Function(_$VeilidAppCallImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? message = null,
    Object? callId = null,
    Object? sender = freezed,
  }) {
    return _then(_$VeilidAppCallImpl(
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
class _$VeilidAppCallImpl implements VeilidAppCall {
  const _$VeilidAppCallImpl(
      {@Uint8ListJsonConverter() required this.message,
      required this.callId,
      this.sender,
      final String? $type})
      : $type = $type ?? 'AppCall';

  factory _$VeilidAppCallImpl.fromJson(Map<String, dynamic> json) =>
      _$$VeilidAppCallImplFromJson(json);

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
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$VeilidAppCallImpl &&
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
  _$$VeilidAppCallImplCopyWith<_$VeilidAppCallImpl> get copyWith =>
      __$$VeilidAppCallImplCopyWithImpl<_$VeilidAppCallImpl>(this, _$identity);

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
    return _$$VeilidAppCallImplToJson(
      this,
    );
  }
}

abstract class VeilidAppCall implements VeilidUpdate {
  const factory VeilidAppCall(
      {@Uint8ListJsonConverter() required final Uint8List message,
      required final String callId,
      final Typed<FixedEncodedString43>? sender}) = _$VeilidAppCallImpl;

  factory VeilidAppCall.fromJson(Map<String, dynamic> json) =
      _$VeilidAppCallImpl.fromJson;

  @Uint8ListJsonConverter()
  Uint8List get message;
  String get callId;
  Typed<FixedEncodedString43>? get sender;
  @JsonKey(ignore: true)
  _$$VeilidAppCallImplCopyWith<_$VeilidAppCallImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$VeilidUpdateAttachmentImplCopyWith<$Res> {
  factory _$$VeilidUpdateAttachmentImplCopyWith(
          _$VeilidUpdateAttachmentImpl value,
          $Res Function(_$VeilidUpdateAttachmentImpl) then) =
      __$$VeilidUpdateAttachmentImplCopyWithImpl<$Res>;
  @useResult
  $Res call(
      {AttachmentState state,
      bool publicInternetReady,
      bool localNetworkReady});
}

/// @nodoc
class __$$VeilidUpdateAttachmentImplCopyWithImpl<$Res>
    extends _$VeilidUpdateCopyWithImpl<$Res, _$VeilidUpdateAttachmentImpl>
    implements _$$VeilidUpdateAttachmentImplCopyWith<$Res> {
  __$$VeilidUpdateAttachmentImplCopyWithImpl(
      _$VeilidUpdateAttachmentImpl _value,
      $Res Function(_$VeilidUpdateAttachmentImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? state = null,
    Object? publicInternetReady = null,
    Object? localNetworkReady = null,
  }) {
    return _then(_$VeilidUpdateAttachmentImpl(
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
class _$VeilidUpdateAttachmentImpl implements VeilidUpdateAttachment {
  const _$VeilidUpdateAttachmentImpl(
      {required this.state,
      required this.publicInternetReady,
      required this.localNetworkReady,
      final String? $type})
      : $type = $type ?? 'Attachment';

  factory _$VeilidUpdateAttachmentImpl.fromJson(Map<String, dynamic> json) =>
      _$$VeilidUpdateAttachmentImplFromJson(json);

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
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$VeilidUpdateAttachmentImpl &&
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
  _$$VeilidUpdateAttachmentImplCopyWith<_$VeilidUpdateAttachmentImpl>
      get copyWith => __$$VeilidUpdateAttachmentImplCopyWithImpl<
          _$VeilidUpdateAttachmentImpl>(this, _$identity);

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
    return _$$VeilidUpdateAttachmentImplToJson(
      this,
    );
  }
}

abstract class VeilidUpdateAttachment implements VeilidUpdate {
  const factory VeilidUpdateAttachment(
      {required final AttachmentState state,
      required final bool publicInternetReady,
      required final bool localNetworkReady}) = _$VeilidUpdateAttachmentImpl;

  factory VeilidUpdateAttachment.fromJson(Map<String, dynamic> json) =
      _$VeilidUpdateAttachmentImpl.fromJson;

  AttachmentState get state;
  bool get publicInternetReady;
  bool get localNetworkReady;
  @JsonKey(ignore: true)
  _$$VeilidUpdateAttachmentImplCopyWith<_$VeilidUpdateAttachmentImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$VeilidUpdateNetworkImplCopyWith<$Res> {
  factory _$$VeilidUpdateNetworkImplCopyWith(_$VeilidUpdateNetworkImpl value,
          $Res Function(_$VeilidUpdateNetworkImpl) then) =
      __$$VeilidUpdateNetworkImplCopyWithImpl<$Res>;
  @useResult
  $Res call(
      {bool started, BigInt bpsDown, BigInt bpsUp, List<PeerTableData> peers});
}

/// @nodoc
class __$$VeilidUpdateNetworkImplCopyWithImpl<$Res>
    extends _$VeilidUpdateCopyWithImpl<$Res, _$VeilidUpdateNetworkImpl>
    implements _$$VeilidUpdateNetworkImplCopyWith<$Res> {
  __$$VeilidUpdateNetworkImplCopyWithImpl(_$VeilidUpdateNetworkImpl _value,
      $Res Function(_$VeilidUpdateNetworkImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? started = null,
    Object? bpsDown = null,
    Object? bpsUp = null,
    Object? peers = null,
  }) {
    return _then(_$VeilidUpdateNetworkImpl(
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
class _$VeilidUpdateNetworkImpl implements VeilidUpdateNetwork {
  const _$VeilidUpdateNetworkImpl(
      {required this.started,
      required this.bpsDown,
      required this.bpsUp,
      required final List<PeerTableData> peers,
      final String? $type})
      : _peers = peers,
        $type = $type ?? 'Network';

  factory _$VeilidUpdateNetworkImpl.fromJson(Map<String, dynamic> json) =>
      _$$VeilidUpdateNetworkImplFromJson(json);

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
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$VeilidUpdateNetworkImpl &&
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
  _$$VeilidUpdateNetworkImplCopyWith<_$VeilidUpdateNetworkImpl> get copyWith =>
      __$$VeilidUpdateNetworkImplCopyWithImpl<_$VeilidUpdateNetworkImpl>(
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
    return _$$VeilidUpdateNetworkImplToJson(
      this,
    );
  }
}

abstract class VeilidUpdateNetwork implements VeilidUpdate {
  const factory VeilidUpdateNetwork(
      {required final bool started,
      required final BigInt bpsDown,
      required final BigInt bpsUp,
      required final List<PeerTableData> peers}) = _$VeilidUpdateNetworkImpl;

  factory VeilidUpdateNetwork.fromJson(Map<String, dynamic> json) =
      _$VeilidUpdateNetworkImpl.fromJson;

  bool get started;
  BigInt get bpsDown;
  BigInt get bpsUp;
  List<PeerTableData> get peers;
  @JsonKey(ignore: true)
  _$$VeilidUpdateNetworkImplCopyWith<_$VeilidUpdateNetworkImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$VeilidUpdateConfigImplCopyWith<$Res> {
  factory _$$VeilidUpdateConfigImplCopyWith(_$VeilidUpdateConfigImpl value,
          $Res Function(_$VeilidUpdateConfigImpl) then) =
      __$$VeilidUpdateConfigImplCopyWithImpl<$Res>;
  @useResult
  $Res call({VeilidConfig config});

  $VeilidConfigCopyWith<$Res> get config;
}

/// @nodoc
class __$$VeilidUpdateConfigImplCopyWithImpl<$Res>
    extends _$VeilidUpdateCopyWithImpl<$Res, _$VeilidUpdateConfigImpl>
    implements _$$VeilidUpdateConfigImplCopyWith<$Res> {
  __$$VeilidUpdateConfigImplCopyWithImpl(_$VeilidUpdateConfigImpl _value,
      $Res Function(_$VeilidUpdateConfigImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? config = null,
  }) {
    return _then(_$VeilidUpdateConfigImpl(
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
class _$VeilidUpdateConfigImpl implements VeilidUpdateConfig {
  const _$VeilidUpdateConfigImpl({required this.config, final String? $type})
      : $type = $type ?? 'Config';

  factory _$VeilidUpdateConfigImpl.fromJson(Map<String, dynamic> json) =>
      _$$VeilidUpdateConfigImplFromJson(json);

  @override
  final VeilidConfig config;

  @JsonKey(name: 'kind')
  final String $type;

  @override
  String toString() {
    return 'VeilidUpdate.config(config: $config)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$VeilidUpdateConfigImpl &&
            (identical(other.config, config) || other.config == config));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, config);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$VeilidUpdateConfigImplCopyWith<_$VeilidUpdateConfigImpl> get copyWith =>
      __$$VeilidUpdateConfigImplCopyWithImpl<_$VeilidUpdateConfigImpl>(
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
    return _$$VeilidUpdateConfigImplToJson(
      this,
    );
  }
}

abstract class VeilidUpdateConfig implements VeilidUpdate {
  const factory VeilidUpdateConfig({required final VeilidConfig config}) =
      _$VeilidUpdateConfigImpl;

  factory VeilidUpdateConfig.fromJson(Map<String, dynamic> json) =
      _$VeilidUpdateConfigImpl.fromJson;

  VeilidConfig get config;
  @JsonKey(ignore: true)
  _$$VeilidUpdateConfigImplCopyWith<_$VeilidUpdateConfigImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$VeilidUpdateRouteChangeImplCopyWith<$Res> {
  factory _$$VeilidUpdateRouteChangeImplCopyWith(
          _$VeilidUpdateRouteChangeImpl value,
          $Res Function(_$VeilidUpdateRouteChangeImpl) then) =
      __$$VeilidUpdateRouteChangeImplCopyWithImpl<$Res>;
  @useResult
  $Res call({List<String> deadRoutes, List<String> deadRemoteRoutes});
}

/// @nodoc
class __$$VeilidUpdateRouteChangeImplCopyWithImpl<$Res>
    extends _$VeilidUpdateCopyWithImpl<$Res, _$VeilidUpdateRouteChangeImpl>
    implements _$$VeilidUpdateRouteChangeImplCopyWith<$Res> {
  __$$VeilidUpdateRouteChangeImplCopyWithImpl(
      _$VeilidUpdateRouteChangeImpl _value,
      $Res Function(_$VeilidUpdateRouteChangeImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? deadRoutes = null,
    Object? deadRemoteRoutes = null,
  }) {
    return _then(_$VeilidUpdateRouteChangeImpl(
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
class _$VeilidUpdateRouteChangeImpl implements VeilidUpdateRouteChange {
  const _$VeilidUpdateRouteChangeImpl(
      {required final List<String> deadRoutes,
      required final List<String> deadRemoteRoutes,
      final String? $type})
      : _deadRoutes = deadRoutes,
        _deadRemoteRoutes = deadRemoteRoutes,
        $type = $type ?? 'RouteChange';

  factory _$VeilidUpdateRouteChangeImpl.fromJson(Map<String, dynamic> json) =>
      _$$VeilidUpdateRouteChangeImplFromJson(json);

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
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$VeilidUpdateRouteChangeImpl &&
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
  _$$VeilidUpdateRouteChangeImplCopyWith<_$VeilidUpdateRouteChangeImpl>
      get copyWith => __$$VeilidUpdateRouteChangeImplCopyWithImpl<
          _$VeilidUpdateRouteChangeImpl>(this, _$identity);

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
    return _$$VeilidUpdateRouteChangeImplToJson(
      this,
    );
  }
}

abstract class VeilidUpdateRouteChange implements VeilidUpdate {
  const factory VeilidUpdateRouteChange(
          {required final List<String> deadRoutes,
          required final List<String> deadRemoteRoutes}) =
      _$VeilidUpdateRouteChangeImpl;

  factory VeilidUpdateRouteChange.fromJson(Map<String, dynamic> json) =
      _$VeilidUpdateRouteChangeImpl.fromJson;

  List<String> get deadRoutes;
  List<String> get deadRemoteRoutes;
  @JsonKey(ignore: true)
  _$$VeilidUpdateRouteChangeImplCopyWith<_$VeilidUpdateRouteChangeImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$VeilidUpdateValueChangeImplCopyWith<$Res> {
  factory _$$VeilidUpdateValueChangeImplCopyWith(
          _$VeilidUpdateValueChangeImpl value,
          $Res Function(_$VeilidUpdateValueChangeImpl) then) =
      __$$VeilidUpdateValueChangeImplCopyWithImpl<$Res>;
  @useResult
  $Res call(
      {Typed<FixedEncodedString43> key,
      List<ValueSubkeyRange> subkeys,
      int count,
      ValueData value});

  $ValueDataCopyWith<$Res> get value;
}

/// @nodoc
class __$$VeilidUpdateValueChangeImplCopyWithImpl<$Res>
    extends _$VeilidUpdateCopyWithImpl<$Res, _$VeilidUpdateValueChangeImpl>
    implements _$$VeilidUpdateValueChangeImplCopyWith<$Res> {
  __$$VeilidUpdateValueChangeImplCopyWithImpl(
      _$VeilidUpdateValueChangeImpl _value,
      $Res Function(_$VeilidUpdateValueChangeImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? key = null,
    Object? subkeys = null,
    Object? count = null,
    Object? value = null,
  }) {
    return _then(_$VeilidUpdateValueChangeImpl(
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
class _$VeilidUpdateValueChangeImpl implements VeilidUpdateValueChange {
  const _$VeilidUpdateValueChangeImpl(
      {required this.key,
      required final List<ValueSubkeyRange> subkeys,
      required this.count,
      required this.value,
      final String? $type})
      : _subkeys = subkeys,
        $type = $type ?? 'ValueChange';

  factory _$VeilidUpdateValueChangeImpl.fromJson(Map<String, dynamic> json) =>
      _$$VeilidUpdateValueChangeImplFromJson(json);

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
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$VeilidUpdateValueChangeImpl &&
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
  _$$VeilidUpdateValueChangeImplCopyWith<_$VeilidUpdateValueChangeImpl>
      get copyWith => __$$VeilidUpdateValueChangeImplCopyWithImpl<
          _$VeilidUpdateValueChangeImpl>(this, _$identity);

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
    return _$$VeilidUpdateValueChangeImplToJson(
      this,
    );
  }
}

abstract class VeilidUpdateValueChange implements VeilidUpdate {
  const factory VeilidUpdateValueChange(
      {required final Typed<FixedEncodedString43> key,
      required final List<ValueSubkeyRange> subkeys,
      required final int count,
      required final ValueData value}) = _$VeilidUpdateValueChangeImpl;

  factory VeilidUpdateValueChange.fromJson(Map<String, dynamic> json) =
      _$VeilidUpdateValueChangeImpl.fromJson;

  Typed<FixedEncodedString43> get key;
  List<ValueSubkeyRange> get subkeys;
  int get count;
  ValueData get value;
  @JsonKey(ignore: true)
  _$$VeilidUpdateValueChangeImplCopyWith<_$VeilidUpdateValueChangeImpl>
      get copyWith => throw _privateConstructorUsedError;
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
abstract class _$$VeilidStateAttachmentImplCopyWith<$Res>
    implements $VeilidStateAttachmentCopyWith<$Res> {
  factory _$$VeilidStateAttachmentImplCopyWith(
          _$VeilidStateAttachmentImpl value,
          $Res Function(_$VeilidStateAttachmentImpl) then) =
      __$$VeilidStateAttachmentImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {AttachmentState state,
      bool publicInternetReady,
      bool localNetworkReady});
}

/// @nodoc
class __$$VeilidStateAttachmentImplCopyWithImpl<$Res>
    extends _$VeilidStateAttachmentCopyWithImpl<$Res,
        _$VeilidStateAttachmentImpl>
    implements _$$VeilidStateAttachmentImplCopyWith<$Res> {
  __$$VeilidStateAttachmentImplCopyWithImpl(_$VeilidStateAttachmentImpl _value,
      $Res Function(_$VeilidStateAttachmentImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? state = null,
    Object? publicInternetReady = null,
    Object? localNetworkReady = null,
  }) {
    return _then(_$VeilidStateAttachmentImpl(
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
class _$VeilidStateAttachmentImpl implements _VeilidStateAttachment {
  const _$VeilidStateAttachmentImpl(
      {required this.state,
      required this.publicInternetReady,
      required this.localNetworkReady});

  factory _$VeilidStateAttachmentImpl.fromJson(Map<String, dynamic> json) =>
      _$$VeilidStateAttachmentImplFromJson(json);

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
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$VeilidStateAttachmentImpl &&
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
  _$$VeilidStateAttachmentImplCopyWith<_$VeilidStateAttachmentImpl>
      get copyWith => __$$VeilidStateAttachmentImplCopyWithImpl<
          _$VeilidStateAttachmentImpl>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$VeilidStateAttachmentImplToJson(
      this,
    );
  }
}

abstract class _VeilidStateAttachment implements VeilidStateAttachment {
  const factory _VeilidStateAttachment(
      {required final AttachmentState state,
      required final bool publicInternetReady,
      required final bool localNetworkReady}) = _$VeilidStateAttachmentImpl;

  factory _VeilidStateAttachment.fromJson(Map<String, dynamic> json) =
      _$VeilidStateAttachmentImpl.fromJson;

  @override
  AttachmentState get state;
  @override
  bool get publicInternetReady;
  @override
  bool get localNetworkReady;
  @override
  @JsonKey(ignore: true)
  _$$VeilidStateAttachmentImplCopyWith<_$VeilidStateAttachmentImpl>
      get copyWith => throw _privateConstructorUsedError;
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
abstract class _$$VeilidStateNetworkImplCopyWith<$Res>
    implements $VeilidStateNetworkCopyWith<$Res> {
  factory _$$VeilidStateNetworkImplCopyWith(_$VeilidStateNetworkImpl value,
          $Res Function(_$VeilidStateNetworkImpl) then) =
      __$$VeilidStateNetworkImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {bool started, BigInt bpsDown, BigInt bpsUp, List<PeerTableData> peers});
}

/// @nodoc
class __$$VeilidStateNetworkImplCopyWithImpl<$Res>
    extends _$VeilidStateNetworkCopyWithImpl<$Res, _$VeilidStateNetworkImpl>
    implements _$$VeilidStateNetworkImplCopyWith<$Res> {
  __$$VeilidStateNetworkImplCopyWithImpl(_$VeilidStateNetworkImpl _value,
      $Res Function(_$VeilidStateNetworkImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? started = null,
    Object? bpsDown = null,
    Object? bpsUp = null,
    Object? peers = null,
  }) {
    return _then(_$VeilidStateNetworkImpl(
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
class _$VeilidStateNetworkImpl implements _VeilidStateNetwork {
  const _$VeilidStateNetworkImpl(
      {required this.started,
      required this.bpsDown,
      required this.bpsUp,
      required final List<PeerTableData> peers})
      : _peers = peers;

  factory _$VeilidStateNetworkImpl.fromJson(Map<String, dynamic> json) =>
      _$$VeilidStateNetworkImplFromJson(json);

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
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$VeilidStateNetworkImpl &&
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
  _$$VeilidStateNetworkImplCopyWith<_$VeilidStateNetworkImpl> get copyWith =>
      __$$VeilidStateNetworkImplCopyWithImpl<_$VeilidStateNetworkImpl>(
          this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$VeilidStateNetworkImplToJson(
      this,
    );
  }
}

abstract class _VeilidStateNetwork implements VeilidStateNetwork {
  const factory _VeilidStateNetwork(
      {required final bool started,
      required final BigInt bpsDown,
      required final BigInt bpsUp,
      required final List<PeerTableData> peers}) = _$VeilidStateNetworkImpl;

  factory _VeilidStateNetwork.fromJson(Map<String, dynamic> json) =
      _$VeilidStateNetworkImpl.fromJson;

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
  _$$VeilidStateNetworkImplCopyWith<_$VeilidStateNetworkImpl> get copyWith =>
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
abstract class _$$VeilidStateConfigImplCopyWith<$Res>
    implements $VeilidStateConfigCopyWith<$Res> {
  factory _$$VeilidStateConfigImplCopyWith(_$VeilidStateConfigImpl value,
          $Res Function(_$VeilidStateConfigImpl) then) =
      __$$VeilidStateConfigImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({VeilidConfig config});

  @override
  $VeilidConfigCopyWith<$Res> get config;
}

/// @nodoc
class __$$VeilidStateConfigImplCopyWithImpl<$Res>
    extends _$VeilidStateConfigCopyWithImpl<$Res, _$VeilidStateConfigImpl>
    implements _$$VeilidStateConfigImplCopyWith<$Res> {
  __$$VeilidStateConfigImplCopyWithImpl(_$VeilidStateConfigImpl _value,
      $Res Function(_$VeilidStateConfigImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? config = null,
  }) {
    return _then(_$VeilidStateConfigImpl(
      config: null == config
          ? _value.config
          : config // ignore: cast_nullable_to_non_nullable
              as VeilidConfig,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$VeilidStateConfigImpl implements _VeilidStateConfig {
  const _$VeilidStateConfigImpl({required this.config});

  factory _$VeilidStateConfigImpl.fromJson(Map<String, dynamic> json) =>
      _$$VeilidStateConfigImplFromJson(json);

  @override
  final VeilidConfig config;

  @override
  String toString() {
    return 'VeilidStateConfig(config: $config)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$VeilidStateConfigImpl &&
            (identical(other.config, config) || other.config == config));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, config);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$VeilidStateConfigImplCopyWith<_$VeilidStateConfigImpl> get copyWith =>
      __$$VeilidStateConfigImplCopyWithImpl<_$VeilidStateConfigImpl>(
          this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$VeilidStateConfigImplToJson(
      this,
    );
  }
}

abstract class _VeilidStateConfig implements VeilidStateConfig {
  const factory _VeilidStateConfig({required final VeilidConfig config}) =
      _$VeilidStateConfigImpl;

  factory _VeilidStateConfig.fromJson(Map<String, dynamic> json) =
      _$VeilidStateConfigImpl.fromJson;

  @override
  VeilidConfig get config;
  @override
  @JsonKey(ignore: true)
  _$$VeilidStateConfigImplCopyWith<_$VeilidStateConfigImpl> get copyWith =>
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
abstract class _$$VeilidStateImplCopyWith<$Res>
    implements $VeilidStateCopyWith<$Res> {
  factory _$$VeilidStateImplCopyWith(
          _$VeilidStateImpl value, $Res Function(_$VeilidStateImpl) then) =
      __$$VeilidStateImplCopyWithImpl<$Res>;
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
class __$$VeilidStateImplCopyWithImpl<$Res>
    extends _$VeilidStateCopyWithImpl<$Res, _$VeilidStateImpl>
    implements _$$VeilidStateImplCopyWith<$Res> {
  __$$VeilidStateImplCopyWithImpl(
      _$VeilidStateImpl _value, $Res Function(_$VeilidStateImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? attachment = null,
    Object? network = null,
    Object? config = null,
  }) {
    return _then(_$VeilidStateImpl(
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
class _$VeilidStateImpl implements _VeilidState {
  const _$VeilidStateImpl(
      {required this.attachment, required this.network, required this.config});

  factory _$VeilidStateImpl.fromJson(Map<String, dynamic> json) =>
      _$$VeilidStateImplFromJson(json);

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
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$VeilidStateImpl &&
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
  _$$VeilidStateImplCopyWith<_$VeilidStateImpl> get copyWith =>
      __$$VeilidStateImplCopyWithImpl<_$VeilidStateImpl>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$VeilidStateImplToJson(
      this,
    );
  }
}

abstract class _VeilidState implements VeilidState {
  const factory _VeilidState(
      {required final VeilidStateAttachment attachment,
      required final VeilidStateNetwork network,
      required final VeilidStateConfig config}) = _$VeilidStateImpl;

  factory _VeilidState.fromJson(Map<String, dynamic> json) =
      _$VeilidStateImpl.fromJson;

  @override
  VeilidStateAttachment get attachment;
  @override
  VeilidStateNetwork get network;
  @override
  VeilidStateConfig get config;
  @override
  @JsonKey(ignore: true)
  _$$VeilidStateImplCopyWith<_$VeilidStateImpl> get copyWith =>
      throw _privateConstructorUsedError;
}
