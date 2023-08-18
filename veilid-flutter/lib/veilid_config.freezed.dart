// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'veilid_config.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#custom-getters-and-methods');

VeilidFFIConfigLoggingTerminal _$VeilidFFIConfigLoggingTerminalFromJson(
    Map<String, dynamic> json) {
  return _VeilidFFIConfigLoggingTerminal.fromJson(json);
}

/// @nodoc
mixin _$VeilidFFIConfigLoggingTerminal {
  bool get enabled => throw _privateConstructorUsedError;
  VeilidConfigLogLevel get level => throw _privateConstructorUsedError;

  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
  @JsonKey(ignore: true)
  $VeilidFFIConfigLoggingTerminalCopyWith<VeilidFFIConfigLoggingTerminal>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $VeilidFFIConfigLoggingTerminalCopyWith<$Res> {
  factory $VeilidFFIConfigLoggingTerminalCopyWith(
          VeilidFFIConfigLoggingTerminal value,
          $Res Function(VeilidFFIConfigLoggingTerminal) then) =
      _$VeilidFFIConfigLoggingTerminalCopyWithImpl<$Res,
          VeilidFFIConfigLoggingTerminal>;
  @useResult
  $Res call({bool enabled, VeilidConfigLogLevel level});
}

/// @nodoc
class _$VeilidFFIConfigLoggingTerminalCopyWithImpl<$Res,
        $Val extends VeilidFFIConfigLoggingTerminal>
    implements $VeilidFFIConfigLoggingTerminalCopyWith<$Res> {
  _$VeilidFFIConfigLoggingTerminalCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? enabled = null,
    Object? level = null,
  }) {
    return _then(_value.copyWith(
      enabled: null == enabled
          ? _value.enabled
          : enabled // ignore: cast_nullable_to_non_nullable
              as bool,
      level: null == level
          ? _value.level
          : level // ignore: cast_nullable_to_non_nullable
              as VeilidConfigLogLevel,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$_VeilidFFIConfigLoggingTerminalCopyWith<$Res>
    implements $VeilidFFIConfigLoggingTerminalCopyWith<$Res> {
  factory _$$_VeilidFFIConfigLoggingTerminalCopyWith(
          _$_VeilidFFIConfigLoggingTerminal value,
          $Res Function(_$_VeilidFFIConfigLoggingTerminal) then) =
      __$$_VeilidFFIConfigLoggingTerminalCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({bool enabled, VeilidConfigLogLevel level});
}

/// @nodoc
class __$$_VeilidFFIConfigLoggingTerminalCopyWithImpl<$Res>
    extends _$VeilidFFIConfigLoggingTerminalCopyWithImpl<$Res,
        _$_VeilidFFIConfigLoggingTerminal>
    implements _$$_VeilidFFIConfigLoggingTerminalCopyWith<$Res> {
  __$$_VeilidFFIConfigLoggingTerminalCopyWithImpl(
      _$_VeilidFFIConfigLoggingTerminal _value,
      $Res Function(_$_VeilidFFIConfigLoggingTerminal) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? enabled = null,
    Object? level = null,
  }) {
    return _then(_$_VeilidFFIConfigLoggingTerminal(
      enabled: null == enabled
          ? _value.enabled
          : enabled // ignore: cast_nullable_to_non_nullable
              as bool,
      level: null == level
          ? _value.level
          : level // ignore: cast_nullable_to_non_nullable
              as VeilidConfigLogLevel,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$_VeilidFFIConfigLoggingTerminal
    with DiagnosticableTreeMixin
    implements _VeilidFFIConfigLoggingTerminal {
  const _$_VeilidFFIConfigLoggingTerminal(
      {required this.enabled, required this.level});

  factory _$_VeilidFFIConfigLoggingTerminal.fromJson(
          Map<String, dynamic> json) =>
      _$$_VeilidFFIConfigLoggingTerminalFromJson(json);

  @override
  final bool enabled;
  @override
  final VeilidConfigLogLevel level;

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidFFIConfigLoggingTerminal(enabled: $enabled, level: $level)';
  }

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
    super.debugFillProperties(properties);
    properties
      ..add(DiagnosticsProperty('type', 'VeilidFFIConfigLoggingTerminal'))
      ..add(DiagnosticsProperty('enabled', enabled))
      ..add(DiagnosticsProperty('level', level));
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$_VeilidFFIConfigLoggingTerminal &&
            (identical(other.enabled, enabled) || other.enabled == enabled) &&
            (identical(other.level, level) || other.level == level));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, enabled, level);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$_VeilidFFIConfigLoggingTerminalCopyWith<_$_VeilidFFIConfigLoggingTerminal>
      get copyWith => __$$_VeilidFFIConfigLoggingTerminalCopyWithImpl<
          _$_VeilidFFIConfigLoggingTerminal>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$_VeilidFFIConfigLoggingTerminalToJson(
      this,
    );
  }
}

abstract class _VeilidFFIConfigLoggingTerminal
    implements VeilidFFIConfigLoggingTerminal {
  const factory _VeilidFFIConfigLoggingTerminal(
          {required final bool enabled,
          required final VeilidConfigLogLevel level}) =
      _$_VeilidFFIConfigLoggingTerminal;

  factory _VeilidFFIConfigLoggingTerminal.fromJson(Map<String, dynamic> json) =
      _$_VeilidFFIConfigLoggingTerminal.fromJson;

  @override
  bool get enabled;
  @override
  VeilidConfigLogLevel get level;
  @override
  @JsonKey(ignore: true)
  _$$_VeilidFFIConfigLoggingTerminalCopyWith<_$_VeilidFFIConfigLoggingTerminal>
      get copyWith => throw _privateConstructorUsedError;
}

VeilidFFIConfigLoggingOtlp _$VeilidFFIConfigLoggingOtlpFromJson(
    Map<String, dynamic> json) {
  return _VeilidFFIConfigLoggingOtlp.fromJson(json);
}

/// @nodoc
mixin _$VeilidFFIConfigLoggingOtlp {
  bool get enabled => throw _privateConstructorUsedError;
  VeilidConfigLogLevel get level => throw _privateConstructorUsedError;
  String get grpcEndpoint => throw _privateConstructorUsedError;
  String get serviceName => throw _privateConstructorUsedError;

  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
  @JsonKey(ignore: true)
  $VeilidFFIConfigLoggingOtlpCopyWith<VeilidFFIConfigLoggingOtlp>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $VeilidFFIConfigLoggingOtlpCopyWith<$Res> {
  factory $VeilidFFIConfigLoggingOtlpCopyWith(VeilidFFIConfigLoggingOtlp value,
          $Res Function(VeilidFFIConfigLoggingOtlp) then) =
      _$VeilidFFIConfigLoggingOtlpCopyWithImpl<$Res,
          VeilidFFIConfigLoggingOtlp>;
  @useResult
  $Res call(
      {bool enabled,
      VeilidConfigLogLevel level,
      String grpcEndpoint,
      String serviceName});
}

/// @nodoc
class _$VeilidFFIConfigLoggingOtlpCopyWithImpl<$Res,
        $Val extends VeilidFFIConfigLoggingOtlp>
    implements $VeilidFFIConfigLoggingOtlpCopyWith<$Res> {
  _$VeilidFFIConfigLoggingOtlpCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? enabled = null,
    Object? level = null,
    Object? grpcEndpoint = null,
    Object? serviceName = null,
  }) {
    return _then(_value.copyWith(
      enabled: null == enabled
          ? _value.enabled
          : enabled // ignore: cast_nullable_to_non_nullable
              as bool,
      level: null == level
          ? _value.level
          : level // ignore: cast_nullable_to_non_nullable
              as VeilidConfigLogLevel,
      grpcEndpoint: null == grpcEndpoint
          ? _value.grpcEndpoint
          : grpcEndpoint // ignore: cast_nullable_to_non_nullable
              as String,
      serviceName: null == serviceName
          ? _value.serviceName
          : serviceName // ignore: cast_nullable_to_non_nullable
              as String,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$_VeilidFFIConfigLoggingOtlpCopyWith<$Res>
    implements $VeilidFFIConfigLoggingOtlpCopyWith<$Res> {
  factory _$$_VeilidFFIConfigLoggingOtlpCopyWith(
          _$_VeilidFFIConfigLoggingOtlp value,
          $Res Function(_$_VeilidFFIConfigLoggingOtlp) then) =
      __$$_VeilidFFIConfigLoggingOtlpCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {bool enabled,
      VeilidConfigLogLevel level,
      String grpcEndpoint,
      String serviceName});
}

/// @nodoc
class __$$_VeilidFFIConfigLoggingOtlpCopyWithImpl<$Res>
    extends _$VeilidFFIConfigLoggingOtlpCopyWithImpl<$Res,
        _$_VeilidFFIConfigLoggingOtlp>
    implements _$$_VeilidFFIConfigLoggingOtlpCopyWith<$Res> {
  __$$_VeilidFFIConfigLoggingOtlpCopyWithImpl(
      _$_VeilidFFIConfigLoggingOtlp _value,
      $Res Function(_$_VeilidFFIConfigLoggingOtlp) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? enabled = null,
    Object? level = null,
    Object? grpcEndpoint = null,
    Object? serviceName = null,
  }) {
    return _then(_$_VeilidFFIConfigLoggingOtlp(
      enabled: null == enabled
          ? _value.enabled
          : enabled // ignore: cast_nullable_to_non_nullable
              as bool,
      level: null == level
          ? _value.level
          : level // ignore: cast_nullable_to_non_nullable
              as VeilidConfigLogLevel,
      grpcEndpoint: null == grpcEndpoint
          ? _value.grpcEndpoint
          : grpcEndpoint // ignore: cast_nullable_to_non_nullable
              as String,
      serviceName: null == serviceName
          ? _value.serviceName
          : serviceName // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$_VeilidFFIConfigLoggingOtlp
    with DiagnosticableTreeMixin
    implements _VeilidFFIConfigLoggingOtlp {
  const _$_VeilidFFIConfigLoggingOtlp(
      {required this.enabled,
      required this.level,
      required this.grpcEndpoint,
      required this.serviceName});

  factory _$_VeilidFFIConfigLoggingOtlp.fromJson(Map<String, dynamic> json) =>
      _$$_VeilidFFIConfigLoggingOtlpFromJson(json);

  @override
  final bool enabled;
  @override
  final VeilidConfigLogLevel level;
  @override
  final String grpcEndpoint;
  @override
  final String serviceName;

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidFFIConfigLoggingOtlp(enabled: $enabled, level: $level, grpcEndpoint: $grpcEndpoint, serviceName: $serviceName)';
  }

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
    super.debugFillProperties(properties);
    properties
      ..add(DiagnosticsProperty('type', 'VeilidFFIConfigLoggingOtlp'))
      ..add(DiagnosticsProperty('enabled', enabled))
      ..add(DiagnosticsProperty('level', level))
      ..add(DiagnosticsProperty('grpcEndpoint', grpcEndpoint))
      ..add(DiagnosticsProperty('serviceName', serviceName));
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$_VeilidFFIConfigLoggingOtlp &&
            (identical(other.enabled, enabled) || other.enabled == enabled) &&
            (identical(other.level, level) || other.level == level) &&
            (identical(other.grpcEndpoint, grpcEndpoint) ||
                other.grpcEndpoint == grpcEndpoint) &&
            (identical(other.serviceName, serviceName) ||
                other.serviceName == serviceName));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode =>
      Object.hash(runtimeType, enabled, level, grpcEndpoint, serviceName);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$_VeilidFFIConfigLoggingOtlpCopyWith<_$_VeilidFFIConfigLoggingOtlp>
      get copyWith => __$$_VeilidFFIConfigLoggingOtlpCopyWithImpl<
          _$_VeilidFFIConfigLoggingOtlp>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$_VeilidFFIConfigLoggingOtlpToJson(
      this,
    );
  }
}

abstract class _VeilidFFIConfigLoggingOtlp
    implements VeilidFFIConfigLoggingOtlp {
  const factory _VeilidFFIConfigLoggingOtlp(
      {required final bool enabled,
      required final VeilidConfigLogLevel level,
      required final String grpcEndpoint,
      required final String serviceName}) = _$_VeilidFFIConfigLoggingOtlp;

  factory _VeilidFFIConfigLoggingOtlp.fromJson(Map<String, dynamic> json) =
      _$_VeilidFFIConfigLoggingOtlp.fromJson;

  @override
  bool get enabled;
  @override
  VeilidConfigLogLevel get level;
  @override
  String get grpcEndpoint;
  @override
  String get serviceName;
  @override
  @JsonKey(ignore: true)
  _$$_VeilidFFIConfigLoggingOtlpCopyWith<_$_VeilidFFIConfigLoggingOtlp>
      get copyWith => throw _privateConstructorUsedError;
}

VeilidFFIConfigLoggingApi _$VeilidFFIConfigLoggingApiFromJson(
    Map<String, dynamic> json) {
  return _VeilidFFIConfigLoggingApi.fromJson(json);
}

/// @nodoc
mixin _$VeilidFFIConfigLoggingApi {
  bool get enabled => throw _privateConstructorUsedError;
  VeilidConfigLogLevel get level => throw _privateConstructorUsedError;

  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
  @JsonKey(ignore: true)
  $VeilidFFIConfigLoggingApiCopyWith<VeilidFFIConfigLoggingApi> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $VeilidFFIConfigLoggingApiCopyWith<$Res> {
  factory $VeilidFFIConfigLoggingApiCopyWith(VeilidFFIConfigLoggingApi value,
          $Res Function(VeilidFFIConfigLoggingApi) then) =
      _$VeilidFFIConfigLoggingApiCopyWithImpl<$Res, VeilidFFIConfigLoggingApi>;
  @useResult
  $Res call({bool enabled, VeilidConfigLogLevel level});
}

/// @nodoc
class _$VeilidFFIConfigLoggingApiCopyWithImpl<$Res,
        $Val extends VeilidFFIConfigLoggingApi>
    implements $VeilidFFIConfigLoggingApiCopyWith<$Res> {
  _$VeilidFFIConfigLoggingApiCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? enabled = null,
    Object? level = null,
  }) {
    return _then(_value.copyWith(
      enabled: null == enabled
          ? _value.enabled
          : enabled // ignore: cast_nullable_to_non_nullable
              as bool,
      level: null == level
          ? _value.level
          : level // ignore: cast_nullable_to_non_nullable
              as VeilidConfigLogLevel,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$_VeilidFFIConfigLoggingApiCopyWith<$Res>
    implements $VeilidFFIConfigLoggingApiCopyWith<$Res> {
  factory _$$_VeilidFFIConfigLoggingApiCopyWith(
          _$_VeilidFFIConfigLoggingApi value,
          $Res Function(_$_VeilidFFIConfigLoggingApi) then) =
      __$$_VeilidFFIConfigLoggingApiCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({bool enabled, VeilidConfigLogLevel level});
}

/// @nodoc
class __$$_VeilidFFIConfigLoggingApiCopyWithImpl<$Res>
    extends _$VeilidFFIConfigLoggingApiCopyWithImpl<$Res,
        _$_VeilidFFIConfigLoggingApi>
    implements _$$_VeilidFFIConfigLoggingApiCopyWith<$Res> {
  __$$_VeilidFFIConfigLoggingApiCopyWithImpl(
      _$_VeilidFFIConfigLoggingApi _value,
      $Res Function(_$_VeilidFFIConfigLoggingApi) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? enabled = null,
    Object? level = null,
  }) {
    return _then(_$_VeilidFFIConfigLoggingApi(
      enabled: null == enabled
          ? _value.enabled
          : enabled // ignore: cast_nullable_to_non_nullable
              as bool,
      level: null == level
          ? _value.level
          : level // ignore: cast_nullable_to_non_nullable
              as VeilidConfigLogLevel,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$_VeilidFFIConfigLoggingApi
    with DiagnosticableTreeMixin
    implements _VeilidFFIConfigLoggingApi {
  const _$_VeilidFFIConfigLoggingApi(
      {required this.enabled, required this.level});

  factory _$_VeilidFFIConfigLoggingApi.fromJson(Map<String, dynamic> json) =>
      _$$_VeilidFFIConfigLoggingApiFromJson(json);

  @override
  final bool enabled;
  @override
  final VeilidConfigLogLevel level;

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidFFIConfigLoggingApi(enabled: $enabled, level: $level)';
  }

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
    super.debugFillProperties(properties);
    properties
      ..add(DiagnosticsProperty('type', 'VeilidFFIConfigLoggingApi'))
      ..add(DiagnosticsProperty('enabled', enabled))
      ..add(DiagnosticsProperty('level', level));
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$_VeilidFFIConfigLoggingApi &&
            (identical(other.enabled, enabled) || other.enabled == enabled) &&
            (identical(other.level, level) || other.level == level));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, enabled, level);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$_VeilidFFIConfigLoggingApiCopyWith<_$_VeilidFFIConfigLoggingApi>
      get copyWith => __$$_VeilidFFIConfigLoggingApiCopyWithImpl<
          _$_VeilidFFIConfigLoggingApi>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$_VeilidFFIConfigLoggingApiToJson(
      this,
    );
  }
}

abstract class _VeilidFFIConfigLoggingApi implements VeilidFFIConfigLoggingApi {
  const factory _VeilidFFIConfigLoggingApi(
          {required final bool enabled,
          required final VeilidConfigLogLevel level}) =
      _$_VeilidFFIConfigLoggingApi;

  factory _VeilidFFIConfigLoggingApi.fromJson(Map<String, dynamic> json) =
      _$_VeilidFFIConfigLoggingApi.fromJson;

  @override
  bool get enabled;
  @override
  VeilidConfigLogLevel get level;
  @override
  @JsonKey(ignore: true)
  _$$_VeilidFFIConfigLoggingApiCopyWith<_$_VeilidFFIConfigLoggingApi>
      get copyWith => throw _privateConstructorUsedError;
}

VeilidFFIConfigLogging _$VeilidFFIConfigLoggingFromJson(
    Map<String, dynamic> json) {
  return _VeilidFFIConfigLogging.fromJson(json);
}

/// @nodoc
mixin _$VeilidFFIConfigLogging {
  VeilidFFIConfigLoggingTerminal get terminal =>
      throw _privateConstructorUsedError;
  VeilidFFIConfigLoggingOtlp get otlp => throw _privateConstructorUsedError;
  VeilidFFIConfigLoggingApi get api => throw _privateConstructorUsedError;

  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
  @JsonKey(ignore: true)
  $VeilidFFIConfigLoggingCopyWith<VeilidFFIConfigLogging> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $VeilidFFIConfigLoggingCopyWith<$Res> {
  factory $VeilidFFIConfigLoggingCopyWith(VeilidFFIConfigLogging value,
          $Res Function(VeilidFFIConfigLogging) then) =
      _$VeilidFFIConfigLoggingCopyWithImpl<$Res, VeilidFFIConfigLogging>;
  @useResult
  $Res call(
      {VeilidFFIConfigLoggingTerminal terminal,
      VeilidFFIConfigLoggingOtlp otlp,
      VeilidFFIConfigLoggingApi api});

  $VeilidFFIConfigLoggingTerminalCopyWith<$Res> get terminal;
  $VeilidFFIConfigLoggingOtlpCopyWith<$Res> get otlp;
  $VeilidFFIConfigLoggingApiCopyWith<$Res> get api;
}

/// @nodoc
class _$VeilidFFIConfigLoggingCopyWithImpl<$Res,
        $Val extends VeilidFFIConfigLogging>
    implements $VeilidFFIConfigLoggingCopyWith<$Res> {
  _$VeilidFFIConfigLoggingCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? terminal = null,
    Object? otlp = null,
    Object? api = null,
  }) {
    return _then(_value.copyWith(
      terminal: null == terminal
          ? _value.terminal
          : terminal // ignore: cast_nullable_to_non_nullable
              as VeilidFFIConfigLoggingTerminal,
      otlp: null == otlp
          ? _value.otlp
          : otlp // ignore: cast_nullable_to_non_nullable
              as VeilidFFIConfigLoggingOtlp,
      api: null == api
          ? _value.api
          : api // ignore: cast_nullable_to_non_nullable
              as VeilidFFIConfigLoggingApi,
    ) as $Val);
  }

  @override
  @pragma('vm:prefer-inline')
  $VeilidFFIConfigLoggingTerminalCopyWith<$Res> get terminal {
    return $VeilidFFIConfigLoggingTerminalCopyWith<$Res>(_value.terminal,
        (value) {
      return _then(_value.copyWith(terminal: value) as $Val);
    });
  }

  @override
  @pragma('vm:prefer-inline')
  $VeilidFFIConfigLoggingOtlpCopyWith<$Res> get otlp {
    return $VeilidFFIConfigLoggingOtlpCopyWith<$Res>(_value.otlp, (value) {
      return _then(_value.copyWith(otlp: value) as $Val);
    });
  }

  @override
  @pragma('vm:prefer-inline')
  $VeilidFFIConfigLoggingApiCopyWith<$Res> get api {
    return $VeilidFFIConfigLoggingApiCopyWith<$Res>(_value.api, (value) {
      return _then(_value.copyWith(api: value) as $Val);
    });
  }
}

/// @nodoc
abstract class _$$_VeilidFFIConfigLoggingCopyWith<$Res>
    implements $VeilidFFIConfigLoggingCopyWith<$Res> {
  factory _$$_VeilidFFIConfigLoggingCopyWith(_$_VeilidFFIConfigLogging value,
          $Res Function(_$_VeilidFFIConfigLogging) then) =
      __$$_VeilidFFIConfigLoggingCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {VeilidFFIConfigLoggingTerminal terminal,
      VeilidFFIConfigLoggingOtlp otlp,
      VeilidFFIConfigLoggingApi api});

  @override
  $VeilidFFIConfigLoggingTerminalCopyWith<$Res> get terminal;
  @override
  $VeilidFFIConfigLoggingOtlpCopyWith<$Res> get otlp;
  @override
  $VeilidFFIConfigLoggingApiCopyWith<$Res> get api;
}

/// @nodoc
class __$$_VeilidFFIConfigLoggingCopyWithImpl<$Res>
    extends _$VeilidFFIConfigLoggingCopyWithImpl<$Res,
        _$_VeilidFFIConfigLogging>
    implements _$$_VeilidFFIConfigLoggingCopyWith<$Res> {
  __$$_VeilidFFIConfigLoggingCopyWithImpl(_$_VeilidFFIConfigLogging _value,
      $Res Function(_$_VeilidFFIConfigLogging) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? terminal = null,
    Object? otlp = null,
    Object? api = null,
  }) {
    return _then(_$_VeilidFFIConfigLogging(
      terminal: null == terminal
          ? _value.terminal
          : terminal // ignore: cast_nullable_to_non_nullable
              as VeilidFFIConfigLoggingTerminal,
      otlp: null == otlp
          ? _value.otlp
          : otlp // ignore: cast_nullable_to_non_nullable
              as VeilidFFIConfigLoggingOtlp,
      api: null == api
          ? _value.api
          : api // ignore: cast_nullable_to_non_nullable
              as VeilidFFIConfigLoggingApi,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$_VeilidFFIConfigLogging
    with DiagnosticableTreeMixin
    implements _VeilidFFIConfigLogging {
  const _$_VeilidFFIConfigLogging(
      {required this.terminal, required this.otlp, required this.api});

  factory _$_VeilidFFIConfigLogging.fromJson(Map<String, dynamic> json) =>
      _$$_VeilidFFIConfigLoggingFromJson(json);

  @override
  final VeilidFFIConfigLoggingTerminal terminal;
  @override
  final VeilidFFIConfigLoggingOtlp otlp;
  @override
  final VeilidFFIConfigLoggingApi api;

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidFFIConfigLogging(terminal: $terminal, otlp: $otlp, api: $api)';
  }

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
    super.debugFillProperties(properties);
    properties
      ..add(DiagnosticsProperty('type', 'VeilidFFIConfigLogging'))
      ..add(DiagnosticsProperty('terminal', terminal))
      ..add(DiagnosticsProperty('otlp', otlp))
      ..add(DiagnosticsProperty('api', api));
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$_VeilidFFIConfigLogging &&
            (identical(other.terminal, terminal) ||
                other.terminal == terminal) &&
            (identical(other.otlp, otlp) || other.otlp == otlp) &&
            (identical(other.api, api) || other.api == api));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, terminal, otlp, api);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$_VeilidFFIConfigLoggingCopyWith<_$_VeilidFFIConfigLogging> get copyWith =>
      __$$_VeilidFFIConfigLoggingCopyWithImpl<_$_VeilidFFIConfigLogging>(
          this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$_VeilidFFIConfigLoggingToJson(
      this,
    );
  }
}

abstract class _VeilidFFIConfigLogging implements VeilidFFIConfigLogging {
  const factory _VeilidFFIConfigLogging(
          {required final VeilidFFIConfigLoggingTerminal terminal,
          required final VeilidFFIConfigLoggingOtlp otlp,
          required final VeilidFFIConfigLoggingApi api}) =
      _$_VeilidFFIConfigLogging;

  factory _VeilidFFIConfigLogging.fromJson(Map<String, dynamic> json) =
      _$_VeilidFFIConfigLogging.fromJson;

  @override
  VeilidFFIConfigLoggingTerminal get terminal;
  @override
  VeilidFFIConfigLoggingOtlp get otlp;
  @override
  VeilidFFIConfigLoggingApi get api;
  @override
  @JsonKey(ignore: true)
  _$$_VeilidFFIConfigLoggingCopyWith<_$_VeilidFFIConfigLogging> get copyWith =>
      throw _privateConstructorUsedError;
}

VeilidFFIConfig _$VeilidFFIConfigFromJson(Map<String, dynamic> json) {
  return _VeilidFFIConfig.fromJson(json);
}

/// @nodoc
mixin _$VeilidFFIConfig {
  VeilidFFIConfigLogging get logging => throw _privateConstructorUsedError;

  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
  @JsonKey(ignore: true)
  $VeilidFFIConfigCopyWith<VeilidFFIConfig> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $VeilidFFIConfigCopyWith<$Res> {
  factory $VeilidFFIConfigCopyWith(
          VeilidFFIConfig value, $Res Function(VeilidFFIConfig) then) =
      _$VeilidFFIConfigCopyWithImpl<$Res, VeilidFFIConfig>;
  @useResult
  $Res call({VeilidFFIConfigLogging logging});

  $VeilidFFIConfigLoggingCopyWith<$Res> get logging;
}

/// @nodoc
class _$VeilidFFIConfigCopyWithImpl<$Res, $Val extends VeilidFFIConfig>
    implements $VeilidFFIConfigCopyWith<$Res> {
  _$VeilidFFIConfigCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? logging = null,
  }) {
    return _then(_value.copyWith(
      logging: null == logging
          ? _value.logging
          : logging // ignore: cast_nullable_to_non_nullable
              as VeilidFFIConfigLogging,
    ) as $Val);
  }

  @override
  @pragma('vm:prefer-inline')
  $VeilidFFIConfigLoggingCopyWith<$Res> get logging {
    return $VeilidFFIConfigLoggingCopyWith<$Res>(_value.logging, (value) {
      return _then(_value.copyWith(logging: value) as $Val);
    });
  }
}

/// @nodoc
abstract class _$$_VeilidFFIConfigCopyWith<$Res>
    implements $VeilidFFIConfigCopyWith<$Res> {
  factory _$$_VeilidFFIConfigCopyWith(
          _$_VeilidFFIConfig value, $Res Function(_$_VeilidFFIConfig) then) =
      __$$_VeilidFFIConfigCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({VeilidFFIConfigLogging logging});

  @override
  $VeilidFFIConfigLoggingCopyWith<$Res> get logging;
}

/// @nodoc
class __$$_VeilidFFIConfigCopyWithImpl<$Res>
    extends _$VeilidFFIConfigCopyWithImpl<$Res, _$_VeilidFFIConfig>
    implements _$$_VeilidFFIConfigCopyWith<$Res> {
  __$$_VeilidFFIConfigCopyWithImpl(
      _$_VeilidFFIConfig _value, $Res Function(_$_VeilidFFIConfig) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? logging = null,
  }) {
    return _then(_$_VeilidFFIConfig(
      logging: null == logging
          ? _value.logging
          : logging // ignore: cast_nullable_to_non_nullable
              as VeilidFFIConfigLogging,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$_VeilidFFIConfig
    with DiagnosticableTreeMixin
    implements _VeilidFFIConfig {
  const _$_VeilidFFIConfig({required this.logging});

  factory _$_VeilidFFIConfig.fromJson(Map<String, dynamic> json) =>
      _$$_VeilidFFIConfigFromJson(json);

  @override
  final VeilidFFIConfigLogging logging;

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidFFIConfig(logging: $logging)';
  }

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
    super.debugFillProperties(properties);
    properties
      ..add(DiagnosticsProperty('type', 'VeilidFFIConfig'))
      ..add(DiagnosticsProperty('logging', logging));
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$_VeilidFFIConfig &&
            (identical(other.logging, logging) || other.logging == logging));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, logging);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$_VeilidFFIConfigCopyWith<_$_VeilidFFIConfig> get copyWith =>
      __$$_VeilidFFIConfigCopyWithImpl<_$_VeilidFFIConfig>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$_VeilidFFIConfigToJson(
      this,
    );
  }
}

abstract class _VeilidFFIConfig implements VeilidFFIConfig {
  const factory _VeilidFFIConfig(
      {required final VeilidFFIConfigLogging logging}) = _$_VeilidFFIConfig;

  factory _VeilidFFIConfig.fromJson(Map<String, dynamic> json) =
      _$_VeilidFFIConfig.fromJson;

  @override
  VeilidFFIConfigLogging get logging;
  @override
  @JsonKey(ignore: true)
  _$$_VeilidFFIConfigCopyWith<_$_VeilidFFIConfig> get copyWith =>
      throw _privateConstructorUsedError;
}

VeilidWASMConfigLoggingPerformance _$VeilidWASMConfigLoggingPerformanceFromJson(
    Map<String, dynamic> json) {
  return _VeilidWASMConfigLoggingPerformance.fromJson(json);
}

/// @nodoc
mixin _$VeilidWASMConfigLoggingPerformance {
  bool get enabled => throw _privateConstructorUsedError;
  VeilidConfigLogLevel get level => throw _privateConstructorUsedError;
  bool get logsInTimings => throw _privateConstructorUsedError;
  bool get logsInConsole => throw _privateConstructorUsedError;

  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
  @JsonKey(ignore: true)
  $VeilidWASMConfigLoggingPerformanceCopyWith<
          VeilidWASMConfigLoggingPerformance>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $VeilidWASMConfigLoggingPerformanceCopyWith<$Res> {
  factory $VeilidWASMConfigLoggingPerformanceCopyWith(
          VeilidWASMConfigLoggingPerformance value,
          $Res Function(VeilidWASMConfigLoggingPerformance) then) =
      _$VeilidWASMConfigLoggingPerformanceCopyWithImpl<$Res,
          VeilidWASMConfigLoggingPerformance>;
  @useResult
  $Res call(
      {bool enabled,
      VeilidConfigLogLevel level,
      bool logsInTimings,
      bool logsInConsole});
}

/// @nodoc
class _$VeilidWASMConfigLoggingPerformanceCopyWithImpl<$Res,
        $Val extends VeilidWASMConfigLoggingPerformance>
    implements $VeilidWASMConfigLoggingPerformanceCopyWith<$Res> {
  _$VeilidWASMConfigLoggingPerformanceCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? enabled = null,
    Object? level = null,
    Object? logsInTimings = null,
    Object? logsInConsole = null,
  }) {
    return _then(_value.copyWith(
      enabled: null == enabled
          ? _value.enabled
          : enabled // ignore: cast_nullable_to_non_nullable
              as bool,
      level: null == level
          ? _value.level
          : level // ignore: cast_nullable_to_non_nullable
              as VeilidConfigLogLevel,
      logsInTimings: null == logsInTimings
          ? _value.logsInTimings
          : logsInTimings // ignore: cast_nullable_to_non_nullable
              as bool,
      logsInConsole: null == logsInConsole
          ? _value.logsInConsole
          : logsInConsole // ignore: cast_nullable_to_non_nullable
              as bool,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$_VeilidWASMConfigLoggingPerformanceCopyWith<$Res>
    implements $VeilidWASMConfigLoggingPerformanceCopyWith<$Res> {
  factory _$$_VeilidWASMConfigLoggingPerformanceCopyWith(
          _$_VeilidWASMConfigLoggingPerformance value,
          $Res Function(_$_VeilidWASMConfigLoggingPerformance) then) =
      __$$_VeilidWASMConfigLoggingPerformanceCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {bool enabled,
      VeilidConfigLogLevel level,
      bool logsInTimings,
      bool logsInConsole});
}

/// @nodoc
class __$$_VeilidWASMConfigLoggingPerformanceCopyWithImpl<$Res>
    extends _$VeilidWASMConfigLoggingPerformanceCopyWithImpl<$Res,
        _$_VeilidWASMConfigLoggingPerformance>
    implements _$$_VeilidWASMConfigLoggingPerformanceCopyWith<$Res> {
  __$$_VeilidWASMConfigLoggingPerformanceCopyWithImpl(
      _$_VeilidWASMConfigLoggingPerformance _value,
      $Res Function(_$_VeilidWASMConfigLoggingPerformance) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? enabled = null,
    Object? level = null,
    Object? logsInTimings = null,
    Object? logsInConsole = null,
  }) {
    return _then(_$_VeilidWASMConfigLoggingPerformance(
      enabled: null == enabled
          ? _value.enabled
          : enabled // ignore: cast_nullable_to_non_nullable
              as bool,
      level: null == level
          ? _value.level
          : level // ignore: cast_nullable_to_non_nullable
              as VeilidConfigLogLevel,
      logsInTimings: null == logsInTimings
          ? _value.logsInTimings
          : logsInTimings // ignore: cast_nullable_to_non_nullable
              as bool,
      logsInConsole: null == logsInConsole
          ? _value.logsInConsole
          : logsInConsole // ignore: cast_nullable_to_non_nullable
              as bool,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$_VeilidWASMConfigLoggingPerformance
    with DiagnosticableTreeMixin
    implements _VeilidWASMConfigLoggingPerformance {
  const _$_VeilidWASMConfigLoggingPerformance(
      {required this.enabled,
      required this.level,
      required this.logsInTimings,
      required this.logsInConsole});

  factory _$_VeilidWASMConfigLoggingPerformance.fromJson(
          Map<String, dynamic> json) =>
      _$$_VeilidWASMConfigLoggingPerformanceFromJson(json);

  @override
  final bool enabled;
  @override
  final VeilidConfigLogLevel level;
  @override
  final bool logsInTimings;
  @override
  final bool logsInConsole;

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidWASMConfigLoggingPerformance(enabled: $enabled, level: $level, logsInTimings: $logsInTimings, logsInConsole: $logsInConsole)';
  }

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
    super.debugFillProperties(properties);
    properties
      ..add(DiagnosticsProperty('type', 'VeilidWASMConfigLoggingPerformance'))
      ..add(DiagnosticsProperty('enabled', enabled))
      ..add(DiagnosticsProperty('level', level))
      ..add(DiagnosticsProperty('logsInTimings', logsInTimings))
      ..add(DiagnosticsProperty('logsInConsole', logsInConsole));
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$_VeilidWASMConfigLoggingPerformance &&
            (identical(other.enabled, enabled) || other.enabled == enabled) &&
            (identical(other.level, level) || other.level == level) &&
            (identical(other.logsInTimings, logsInTimings) ||
                other.logsInTimings == logsInTimings) &&
            (identical(other.logsInConsole, logsInConsole) ||
                other.logsInConsole == logsInConsole));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode =>
      Object.hash(runtimeType, enabled, level, logsInTimings, logsInConsole);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$_VeilidWASMConfigLoggingPerformanceCopyWith<
          _$_VeilidWASMConfigLoggingPerformance>
      get copyWith => __$$_VeilidWASMConfigLoggingPerformanceCopyWithImpl<
          _$_VeilidWASMConfigLoggingPerformance>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$_VeilidWASMConfigLoggingPerformanceToJson(
      this,
    );
  }
}

abstract class _VeilidWASMConfigLoggingPerformance
    implements VeilidWASMConfigLoggingPerformance {
  const factory _VeilidWASMConfigLoggingPerformance(
          {required final bool enabled,
          required final VeilidConfigLogLevel level,
          required final bool logsInTimings,
          required final bool logsInConsole}) =
      _$_VeilidWASMConfigLoggingPerformance;

  factory _VeilidWASMConfigLoggingPerformance.fromJson(
          Map<String, dynamic> json) =
      _$_VeilidWASMConfigLoggingPerformance.fromJson;

  @override
  bool get enabled;
  @override
  VeilidConfigLogLevel get level;
  @override
  bool get logsInTimings;
  @override
  bool get logsInConsole;
  @override
  @JsonKey(ignore: true)
  _$$_VeilidWASMConfigLoggingPerformanceCopyWith<
          _$_VeilidWASMConfigLoggingPerformance>
      get copyWith => throw _privateConstructorUsedError;
}

VeilidWASMConfigLoggingApi _$VeilidWASMConfigLoggingApiFromJson(
    Map<String, dynamic> json) {
  return _VeilidWASMConfigLoggingApi.fromJson(json);
}

/// @nodoc
mixin _$VeilidWASMConfigLoggingApi {
  bool get enabled => throw _privateConstructorUsedError;
  VeilidConfigLogLevel get level => throw _privateConstructorUsedError;

  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
  @JsonKey(ignore: true)
  $VeilidWASMConfigLoggingApiCopyWith<VeilidWASMConfigLoggingApi>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $VeilidWASMConfigLoggingApiCopyWith<$Res> {
  factory $VeilidWASMConfigLoggingApiCopyWith(VeilidWASMConfigLoggingApi value,
          $Res Function(VeilidWASMConfigLoggingApi) then) =
      _$VeilidWASMConfigLoggingApiCopyWithImpl<$Res,
          VeilidWASMConfigLoggingApi>;
  @useResult
  $Res call({bool enabled, VeilidConfigLogLevel level});
}

/// @nodoc
class _$VeilidWASMConfigLoggingApiCopyWithImpl<$Res,
        $Val extends VeilidWASMConfigLoggingApi>
    implements $VeilidWASMConfigLoggingApiCopyWith<$Res> {
  _$VeilidWASMConfigLoggingApiCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? enabled = null,
    Object? level = null,
  }) {
    return _then(_value.copyWith(
      enabled: null == enabled
          ? _value.enabled
          : enabled // ignore: cast_nullable_to_non_nullable
              as bool,
      level: null == level
          ? _value.level
          : level // ignore: cast_nullable_to_non_nullable
              as VeilidConfigLogLevel,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$_VeilidWASMConfigLoggingApiCopyWith<$Res>
    implements $VeilidWASMConfigLoggingApiCopyWith<$Res> {
  factory _$$_VeilidWASMConfigLoggingApiCopyWith(
          _$_VeilidWASMConfigLoggingApi value,
          $Res Function(_$_VeilidWASMConfigLoggingApi) then) =
      __$$_VeilidWASMConfigLoggingApiCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({bool enabled, VeilidConfigLogLevel level});
}

/// @nodoc
class __$$_VeilidWASMConfigLoggingApiCopyWithImpl<$Res>
    extends _$VeilidWASMConfigLoggingApiCopyWithImpl<$Res,
        _$_VeilidWASMConfigLoggingApi>
    implements _$$_VeilidWASMConfigLoggingApiCopyWith<$Res> {
  __$$_VeilidWASMConfigLoggingApiCopyWithImpl(
      _$_VeilidWASMConfigLoggingApi _value,
      $Res Function(_$_VeilidWASMConfigLoggingApi) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? enabled = null,
    Object? level = null,
  }) {
    return _then(_$_VeilidWASMConfigLoggingApi(
      enabled: null == enabled
          ? _value.enabled
          : enabled // ignore: cast_nullable_to_non_nullable
              as bool,
      level: null == level
          ? _value.level
          : level // ignore: cast_nullable_to_non_nullable
              as VeilidConfigLogLevel,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$_VeilidWASMConfigLoggingApi
    with DiagnosticableTreeMixin
    implements _VeilidWASMConfigLoggingApi {
  const _$_VeilidWASMConfigLoggingApi(
      {required this.enabled, required this.level});

  factory _$_VeilidWASMConfigLoggingApi.fromJson(Map<String, dynamic> json) =>
      _$$_VeilidWASMConfigLoggingApiFromJson(json);

  @override
  final bool enabled;
  @override
  final VeilidConfigLogLevel level;

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidWASMConfigLoggingApi(enabled: $enabled, level: $level)';
  }

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
    super.debugFillProperties(properties);
    properties
      ..add(DiagnosticsProperty('type', 'VeilidWASMConfigLoggingApi'))
      ..add(DiagnosticsProperty('enabled', enabled))
      ..add(DiagnosticsProperty('level', level));
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$_VeilidWASMConfigLoggingApi &&
            (identical(other.enabled, enabled) || other.enabled == enabled) &&
            (identical(other.level, level) || other.level == level));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, enabled, level);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$_VeilidWASMConfigLoggingApiCopyWith<_$_VeilidWASMConfigLoggingApi>
      get copyWith => __$$_VeilidWASMConfigLoggingApiCopyWithImpl<
          _$_VeilidWASMConfigLoggingApi>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$_VeilidWASMConfigLoggingApiToJson(
      this,
    );
  }
}

abstract class _VeilidWASMConfigLoggingApi
    implements VeilidWASMConfigLoggingApi {
  const factory _VeilidWASMConfigLoggingApi(
          {required final bool enabled,
          required final VeilidConfigLogLevel level}) =
      _$_VeilidWASMConfigLoggingApi;

  factory _VeilidWASMConfigLoggingApi.fromJson(Map<String, dynamic> json) =
      _$_VeilidWASMConfigLoggingApi.fromJson;

  @override
  bool get enabled;
  @override
  VeilidConfigLogLevel get level;
  @override
  @JsonKey(ignore: true)
  _$$_VeilidWASMConfigLoggingApiCopyWith<_$_VeilidWASMConfigLoggingApi>
      get copyWith => throw _privateConstructorUsedError;
}

VeilidWASMConfigLogging _$VeilidWASMConfigLoggingFromJson(
    Map<String, dynamic> json) {
  return _VeilidWASMConfigLogging.fromJson(json);
}

/// @nodoc
mixin _$VeilidWASMConfigLogging {
  VeilidWASMConfigLoggingPerformance get performance =>
      throw _privateConstructorUsedError;
  VeilidWASMConfigLoggingApi get api => throw _privateConstructorUsedError;

  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
  @JsonKey(ignore: true)
  $VeilidWASMConfigLoggingCopyWith<VeilidWASMConfigLogging> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $VeilidWASMConfigLoggingCopyWith<$Res> {
  factory $VeilidWASMConfigLoggingCopyWith(VeilidWASMConfigLogging value,
          $Res Function(VeilidWASMConfigLogging) then) =
      _$VeilidWASMConfigLoggingCopyWithImpl<$Res, VeilidWASMConfigLogging>;
  @useResult
  $Res call(
      {VeilidWASMConfigLoggingPerformance performance,
      VeilidWASMConfigLoggingApi api});

  $VeilidWASMConfigLoggingPerformanceCopyWith<$Res> get performance;
  $VeilidWASMConfigLoggingApiCopyWith<$Res> get api;
}

/// @nodoc
class _$VeilidWASMConfigLoggingCopyWithImpl<$Res,
        $Val extends VeilidWASMConfigLogging>
    implements $VeilidWASMConfigLoggingCopyWith<$Res> {
  _$VeilidWASMConfigLoggingCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? performance = null,
    Object? api = null,
  }) {
    return _then(_value.copyWith(
      performance: null == performance
          ? _value.performance
          : performance // ignore: cast_nullable_to_non_nullable
              as VeilidWASMConfigLoggingPerformance,
      api: null == api
          ? _value.api
          : api // ignore: cast_nullable_to_non_nullable
              as VeilidWASMConfigLoggingApi,
    ) as $Val);
  }

  @override
  @pragma('vm:prefer-inline')
  $VeilidWASMConfigLoggingPerformanceCopyWith<$Res> get performance {
    return $VeilidWASMConfigLoggingPerformanceCopyWith<$Res>(_value.performance,
        (value) {
      return _then(_value.copyWith(performance: value) as $Val);
    });
  }

  @override
  @pragma('vm:prefer-inline')
  $VeilidWASMConfigLoggingApiCopyWith<$Res> get api {
    return $VeilidWASMConfigLoggingApiCopyWith<$Res>(_value.api, (value) {
      return _then(_value.copyWith(api: value) as $Val);
    });
  }
}

/// @nodoc
abstract class _$$_VeilidWASMConfigLoggingCopyWith<$Res>
    implements $VeilidWASMConfigLoggingCopyWith<$Res> {
  factory _$$_VeilidWASMConfigLoggingCopyWith(_$_VeilidWASMConfigLogging value,
          $Res Function(_$_VeilidWASMConfigLogging) then) =
      __$$_VeilidWASMConfigLoggingCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {VeilidWASMConfigLoggingPerformance performance,
      VeilidWASMConfigLoggingApi api});

  @override
  $VeilidWASMConfigLoggingPerformanceCopyWith<$Res> get performance;
  @override
  $VeilidWASMConfigLoggingApiCopyWith<$Res> get api;
}

/// @nodoc
class __$$_VeilidWASMConfigLoggingCopyWithImpl<$Res>
    extends _$VeilidWASMConfigLoggingCopyWithImpl<$Res,
        _$_VeilidWASMConfigLogging>
    implements _$$_VeilidWASMConfigLoggingCopyWith<$Res> {
  __$$_VeilidWASMConfigLoggingCopyWithImpl(_$_VeilidWASMConfigLogging _value,
      $Res Function(_$_VeilidWASMConfigLogging) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? performance = null,
    Object? api = null,
  }) {
    return _then(_$_VeilidWASMConfigLogging(
      performance: null == performance
          ? _value.performance
          : performance // ignore: cast_nullable_to_non_nullable
              as VeilidWASMConfigLoggingPerformance,
      api: null == api
          ? _value.api
          : api // ignore: cast_nullable_to_non_nullable
              as VeilidWASMConfigLoggingApi,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$_VeilidWASMConfigLogging
    with DiagnosticableTreeMixin
    implements _VeilidWASMConfigLogging {
  const _$_VeilidWASMConfigLogging(
      {required this.performance, required this.api});

  factory _$_VeilidWASMConfigLogging.fromJson(Map<String, dynamic> json) =>
      _$$_VeilidWASMConfigLoggingFromJson(json);

  @override
  final VeilidWASMConfigLoggingPerformance performance;
  @override
  final VeilidWASMConfigLoggingApi api;

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidWASMConfigLogging(performance: $performance, api: $api)';
  }

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
    super.debugFillProperties(properties);
    properties
      ..add(DiagnosticsProperty('type', 'VeilidWASMConfigLogging'))
      ..add(DiagnosticsProperty('performance', performance))
      ..add(DiagnosticsProperty('api', api));
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$_VeilidWASMConfigLogging &&
            (identical(other.performance, performance) ||
                other.performance == performance) &&
            (identical(other.api, api) || other.api == api));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, performance, api);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$_VeilidWASMConfigLoggingCopyWith<_$_VeilidWASMConfigLogging>
      get copyWith =>
          __$$_VeilidWASMConfigLoggingCopyWithImpl<_$_VeilidWASMConfigLogging>(
              this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$_VeilidWASMConfigLoggingToJson(
      this,
    );
  }
}

abstract class _VeilidWASMConfigLogging implements VeilidWASMConfigLogging {
  const factory _VeilidWASMConfigLogging(
          {required final VeilidWASMConfigLoggingPerformance performance,
          required final VeilidWASMConfigLoggingApi api}) =
      _$_VeilidWASMConfigLogging;

  factory _VeilidWASMConfigLogging.fromJson(Map<String, dynamic> json) =
      _$_VeilidWASMConfigLogging.fromJson;

  @override
  VeilidWASMConfigLoggingPerformance get performance;
  @override
  VeilidWASMConfigLoggingApi get api;
  @override
  @JsonKey(ignore: true)
  _$$_VeilidWASMConfigLoggingCopyWith<_$_VeilidWASMConfigLogging>
      get copyWith => throw _privateConstructorUsedError;
}

VeilidWASMConfig _$VeilidWASMConfigFromJson(Map<String, dynamic> json) {
  return _VeilidWASMConfig.fromJson(json);
}

/// @nodoc
mixin _$VeilidWASMConfig {
  VeilidWASMConfigLogging get logging => throw _privateConstructorUsedError;

  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
  @JsonKey(ignore: true)
  $VeilidWASMConfigCopyWith<VeilidWASMConfig> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $VeilidWASMConfigCopyWith<$Res> {
  factory $VeilidWASMConfigCopyWith(
          VeilidWASMConfig value, $Res Function(VeilidWASMConfig) then) =
      _$VeilidWASMConfigCopyWithImpl<$Res, VeilidWASMConfig>;
  @useResult
  $Res call({VeilidWASMConfigLogging logging});

  $VeilidWASMConfigLoggingCopyWith<$Res> get logging;
}

/// @nodoc
class _$VeilidWASMConfigCopyWithImpl<$Res, $Val extends VeilidWASMConfig>
    implements $VeilidWASMConfigCopyWith<$Res> {
  _$VeilidWASMConfigCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? logging = null,
  }) {
    return _then(_value.copyWith(
      logging: null == logging
          ? _value.logging
          : logging // ignore: cast_nullable_to_non_nullable
              as VeilidWASMConfigLogging,
    ) as $Val);
  }

  @override
  @pragma('vm:prefer-inline')
  $VeilidWASMConfigLoggingCopyWith<$Res> get logging {
    return $VeilidWASMConfigLoggingCopyWith<$Res>(_value.logging, (value) {
      return _then(_value.copyWith(logging: value) as $Val);
    });
  }
}

/// @nodoc
abstract class _$$_VeilidWASMConfigCopyWith<$Res>
    implements $VeilidWASMConfigCopyWith<$Res> {
  factory _$$_VeilidWASMConfigCopyWith(
          _$_VeilidWASMConfig value, $Res Function(_$_VeilidWASMConfig) then) =
      __$$_VeilidWASMConfigCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({VeilidWASMConfigLogging logging});

  @override
  $VeilidWASMConfigLoggingCopyWith<$Res> get logging;
}

/// @nodoc
class __$$_VeilidWASMConfigCopyWithImpl<$Res>
    extends _$VeilidWASMConfigCopyWithImpl<$Res, _$_VeilidWASMConfig>
    implements _$$_VeilidWASMConfigCopyWith<$Res> {
  __$$_VeilidWASMConfigCopyWithImpl(
      _$_VeilidWASMConfig _value, $Res Function(_$_VeilidWASMConfig) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? logging = null,
  }) {
    return _then(_$_VeilidWASMConfig(
      logging: null == logging
          ? _value.logging
          : logging // ignore: cast_nullable_to_non_nullable
              as VeilidWASMConfigLogging,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$_VeilidWASMConfig
    with DiagnosticableTreeMixin
    implements _VeilidWASMConfig {
  const _$_VeilidWASMConfig({required this.logging});

  factory _$_VeilidWASMConfig.fromJson(Map<String, dynamic> json) =>
      _$$_VeilidWASMConfigFromJson(json);

  @override
  final VeilidWASMConfigLogging logging;

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidWASMConfig(logging: $logging)';
  }

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
    super.debugFillProperties(properties);
    properties
      ..add(DiagnosticsProperty('type', 'VeilidWASMConfig'))
      ..add(DiagnosticsProperty('logging', logging));
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$_VeilidWASMConfig &&
            (identical(other.logging, logging) || other.logging == logging));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, logging);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$_VeilidWASMConfigCopyWith<_$_VeilidWASMConfig> get copyWith =>
      __$$_VeilidWASMConfigCopyWithImpl<_$_VeilidWASMConfig>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$_VeilidWASMConfigToJson(
      this,
    );
  }
}

abstract class _VeilidWASMConfig implements VeilidWASMConfig {
  const factory _VeilidWASMConfig(
      {required final VeilidWASMConfigLogging logging}) = _$_VeilidWASMConfig;

  factory _VeilidWASMConfig.fromJson(Map<String, dynamic> json) =
      _$_VeilidWASMConfig.fromJson;

  @override
  VeilidWASMConfigLogging get logging;
  @override
  @JsonKey(ignore: true)
  _$$_VeilidWASMConfigCopyWith<_$_VeilidWASMConfig> get copyWith =>
      throw _privateConstructorUsedError;
}

VeilidConfigHTTPS _$VeilidConfigHTTPSFromJson(Map<String, dynamic> json) {
  return _VeilidConfigHTTPS.fromJson(json);
}

/// @nodoc
mixin _$VeilidConfigHTTPS {
  bool get enabled => throw _privateConstructorUsedError;
  String get listenAddress => throw _privateConstructorUsedError;
  String get path => throw _privateConstructorUsedError;
  String? get url => throw _privateConstructorUsedError;

  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
  @JsonKey(ignore: true)
  $VeilidConfigHTTPSCopyWith<VeilidConfigHTTPS> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $VeilidConfigHTTPSCopyWith<$Res> {
  factory $VeilidConfigHTTPSCopyWith(
          VeilidConfigHTTPS value, $Res Function(VeilidConfigHTTPS) then) =
      _$VeilidConfigHTTPSCopyWithImpl<$Res, VeilidConfigHTTPS>;
  @useResult
  $Res call({bool enabled, String listenAddress, String path, String? url});
}

/// @nodoc
class _$VeilidConfigHTTPSCopyWithImpl<$Res, $Val extends VeilidConfigHTTPS>
    implements $VeilidConfigHTTPSCopyWith<$Res> {
  _$VeilidConfigHTTPSCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? enabled = null,
    Object? listenAddress = null,
    Object? path = null,
    Object? url = freezed,
  }) {
    return _then(_value.copyWith(
      enabled: null == enabled
          ? _value.enabled
          : enabled // ignore: cast_nullable_to_non_nullable
              as bool,
      listenAddress: null == listenAddress
          ? _value.listenAddress
          : listenAddress // ignore: cast_nullable_to_non_nullable
              as String,
      path: null == path
          ? _value.path
          : path // ignore: cast_nullable_to_non_nullable
              as String,
      url: freezed == url
          ? _value.url
          : url // ignore: cast_nullable_to_non_nullable
              as String?,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$_VeilidConfigHTTPSCopyWith<$Res>
    implements $VeilidConfigHTTPSCopyWith<$Res> {
  factory _$$_VeilidConfigHTTPSCopyWith(_$_VeilidConfigHTTPS value,
          $Res Function(_$_VeilidConfigHTTPS) then) =
      __$$_VeilidConfigHTTPSCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({bool enabled, String listenAddress, String path, String? url});
}

/// @nodoc
class __$$_VeilidConfigHTTPSCopyWithImpl<$Res>
    extends _$VeilidConfigHTTPSCopyWithImpl<$Res, _$_VeilidConfigHTTPS>
    implements _$$_VeilidConfigHTTPSCopyWith<$Res> {
  __$$_VeilidConfigHTTPSCopyWithImpl(
      _$_VeilidConfigHTTPS _value, $Res Function(_$_VeilidConfigHTTPS) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? enabled = null,
    Object? listenAddress = null,
    Object? path = null,
    Object? url = freezed,
  }) {
    return _then(_$_VeilidConfigHTTPS(
      enabled: null == enabled
          ? _value.enabled
          : enabled // ignore: cast_nullable_to_non_nullable
              as bool,
      listenAddress: null == listenAddress
          ? _value.listenAddress
          : listenAddress // ignore: cast_nullable_to_non_nullable
              as String,
      path: null == path
          ? _value.path
          : path // ignore: cast_nullable_to_non_nullable
              as String,
      url: freezed == url
          ? _value.url
          : url // ignore: cast_nullable_to_non_nullable
              as String?,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$_VeilidConfigHTTPS
    with DiagnosticableTreeMixin
    implements _VeilidConfigHTTPS {
  const _$_VeilidConfigHTTPS(
      {required this.enabled,
      required this.listenAddress,
      required this.path,
      this.url});

  factory _$_VeilidConfigHTTPS.fromJson(Map<String, dynamic> json) =>
      _$$_VeilidConfigHTTPSFromJson(json);

  @override
  final bool enabled;
  @override
  final String listenAddress;
  @override
  final String path;
  @override
  final String? url;

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidConfigHTTPS(enabled: $enabled, listenAddress: $listenAddress, path: $path, url: $url)';
  }

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
    super.debugFillProperties(properties);
    properties
      ..add(DiagnosticsProperty('type', 'VeilidConfigHTTPS'))
      ..add(DiagnosticsProperty('enabled', enabled))
      ..add(DiagnosticsProperty('listenAddress', listenAddress))
      ..add(DiagnosticsProperty('path', path))
      ..add(DiagnosticsProperty('url', url));
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$_VeilidConfigHTTPS &&
            (identical(other.enabled, enabled) || other.enabled == enabled) &&
            (identical(other.listenAddress, listenAddress) ||
                other.listenAddress == listenAddress) &&
            (identical(other.path, path) || other.path == path) &&
            (identical(other.url, url) || other.url == url));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode =>
      Object.hash(runtimeType, enabled, listenAddress, path, url);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$_VeilidConfigHTTPSCopyWith<_$_VeilidConfigHTTPS> get copyWith =>
      __$$_VeilidConfigHTTPSCopyWithImpl<_$_VeilidConfigHTTPS>(
          this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$_VeilidConfigHTTPSToJson(
      this,
    );
  }
}

abstract class _VeilidConfigHTTPS implements VeilidConfigHTTPS {
  const factory _VeilidConfigHTTPS(
      {required final bool enabled,
      required final String listenAddress,
      required final String path,
      final String? url}) = _$_VeilidConfigHTTPS;

  factory _VeilidConfigHTTPS.fromJson(Map<String, dynamic> json) =
      _$_VeilidConfigHTTPS.fromJson;

  @override
  bool get enabled;
  @override
  String get listenAddress;
  @override
  String get path;
  @override
  String? get url;
  @override
  @JsonKey(ignore: true)
  _$$_VeilidConfigHTTPSCopyWith<_$_VeilidConfigHTTPS> get copyWith =>
      throw _privateConstructorUsedError;
}

VeilidConfigHTTP _$VeilidConfigHTTPFromJson(Map<String, dynamic> json) {
  return _VeilidConfigHTTP.fromJson(json);
}

/// @nodoc
mixin _$VeilidConfigHTTP {
  bool get enabled => throw _privateConstructorUsedError;
  String get listenAddress => throw _privateConstructorUsedError;
  String get path => throw _privateConstructorUsedError;
  String? get url => throw _privateConstructorUsedError;

  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
  @JsonKey(ignore: true)
  $VeilidConfigHTTPCopyWith<VeilidConfigHTTP> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $VeilidConfigHTTPCopyWith<$Res> {
  factory $VeilidConfigHTTPCopyWith(
          VeilidConfigHTTP value, $Res Function(VeilidConfigHTTP) then) =
      _$VeilidConfigHTTPCopyWithImpl<$Res, VeilidConfigHTTP>;
  @useResult
  $Res call({bool enabled, String listenAddress, String path, String? url});
}

/// @nodoc
class _$VeilidConfigHTTPCopyWithImpl<$Res, $Val extends VeilidConfigHTTP>
    implements $VeilidConfigHTTPCopyWith<$Res> {
  _$VeilidConfigHTTPCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? enabled = null,
    Object? listenAddress = null,
    Object? path = null,
    Object? url = freezed,
  }) {
    return _then(_value.copyWith(
      enabled: null == enabled
          ? _value.enabled
          : enabled // ignore: cast_nullable_to_non_nullable
              as bool,
      listenAddress: null == listenAddress
          ? _value.listenAddress
          : listenAddress // ignore: cast_nullable_to_non_nullable
              as String,
      path: null == path
          ? _value.path
          : path // ignore: cast_nullable_to_non_nullable
              as String,
      url: freezed == url
          ? _value.url
          : url // ignore: cast_nullable_to_non_nullable
              as String?,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$_VeilidConfigHTTPCopyWith<$Res>
    implements $VeilidConfigHTTPCopyWith<$Res> {
  factory _$$_VeilidConfigHTTPCopyWith(
          _$_VeilidConfigHTTP value, $Res Function(_$_VeilidConfigHTTP) then) =
      __$$_VeilidConfigHTTPCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({bool enabled, String listenAddress, String path, String? url});
}

/// @nodoc
class __$$_VeilidConfigHTTPCopyWithImpl<$Res>
    extends _$VeilidConfigHTTPCopyWithImpl<$Res, _$_VeilidConfigHTTP>
    implements _$$_VeilidConfigHTTPCopyWith<$Res> {
  __$$_VeilidConfigHTTPCopyWithImpl(
      _$_VeilidConfigHTTP _value, $Res Function(_$_VeilidConfigHTTP) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? enabled = null,
    Object? listenAddress = null,
    Object? path = null,
    Object? url = freezed,
  }) {
    return _then(_$_VeilidConfigHTTP(
      enabled: null == enabled
          ? _value.enabled
          : enabled // ignore: cast_nullable_to_non_nullable
              as bool,
      listenAddress: null == listenAddress
          ? _value.listenAddress
          : listenAddress // ignore: cast_nullable_to_non_nullable
              as String,
      path: null == path
          ? _value.path
          : path // ignore: cast_nullable_to_non_nullable
              as String,
      url: freezed == url
          ? _value.url
          : url // ignore: cast_nullable_to_non_nullable
              as String?,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$_VeilidConfigHTTP
    with DiagnosticableTreeMixin
    implements _VeilidConfigHTTP {
  const _$_VeilidConfigHTTP(
      {required this.enabled,
      required this.listenAddress,
      required this.path,
      this.url});

  factory _$_VeilidConfigHTTP.fromJson(Map<String, dynamic> json) =>
      _$$_VeilidConfigHTTPFromJson(json);

  @override
  final bool enabled;
  @override
  final String listenAddress;
  @override
  final String path;
  @override
  final String? url;

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidConfigHTTP(enabled: $enabled, listenAddress: $listenAddress, path: $path, url: $url)';
  }

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
    super.debugFillProperties(properties);
    properties
      ..add(DiagnosticsProperty('type', 'VeilidConfigHTTP'))
      ..add(DiagnosticsProperty('enabled', enabled))
      ..add(DiagnosticsProperty('listenAddress', listenAddress))
      ..add(DiagnosticsProperty('path', path))
      ..add(DiagnosticsProperty('url', url));
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$_VeilidConfigHTTP &&
            (identical(other.enabled, enabled) || other.enabled == enabled) &&
            (identical(other.listenAddress, listenAddress) ||
                other.listenAddress == listenAddress) &&
            (identical(other.path, path) || other.path == path) &&
            (identical(other.url, url) || other.url == url));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode =>
      Object.hash(runtimeType, enabled, listenAddress, path, url);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$_VeilidConfigHTTPCopyWith<_$_VeilidConfigHTTP> get copyWith =>
      __$$_VeilidConfigHTTPCopyWithImpl<_$_VeilidConfigHTTP>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$_VeilidConfigHTTPToJson(
      this,
    );
  }
}

abstract class _VeilidConfigHTTP implements VeilidConfigHTTP {
  const factory _VeilidConfigHTTP(
      {required final bool enabled,
      required final String listenAddress,
      required final String path,
      final String? url}) = _$_VeilidConfigHTTP;

  factory _VeilidConfigHTTP.fromJson(Map<String, dynamic> json) =
      _$_VeilidConfigHTTP.fromJson;

  @override
  bool get enabled;
  @override
  String get listenAddress;
  @override
  String get path;
  @override
  String? get url;
  @override
  @JsonKey(ignore: true)
  _$$_VeilidConfigHTTPCopyWith<_$_VeilidConfigHTTP> get copyWith =>
      throw _privateConstructorUsedError;
}

VeilidConfigApplication _$VeilidConfigApplicationFromJson(
    Map<String, dynamic> json) {
  return _VeilidConfigApplication.fromJson(json);
}

/// @nodoc
mixin _$VeilidConfigApplication {
  VeilidConfigHTTPS get https => throw _privateConstructorUsedError;
  VeilidConfigHTTP get http => throw _privateConstructorUsedError;

  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
  @JsonKey(ignore: true)
  $VeilidConfigApplicationCopyWith<VeilidConfigApplication> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $VeilidConfigApplicationCopyWith<$Res> {
  factory $VeilidConfigApplicationCopyWith(VeilidConfigApplication value,
          $Res Function(VeilidConfigApplication) then) =
      _$VeilidConfigApplicationCopyWithImpl<$Res, VeilidConfigApplication>;
  @useResult
  $Res call({VeilidConfigHTTPS https, VeilidConfigHTTP http});

  $VeilidConfigHTTPSCopyWith<$Res> get https;
  $VeilidConfigHTTPCopyWith<$Res> get http;
}

/// @nodoc
class _$VeilidConfigApplicationCopyWithImpl<$Res,
        $Val extends VeilidConfigApplication>
    implements $VeilidConfigApplicationCopyWith<$Res> {
  _$VeilidConfigApplicationCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? https = null,
    Object? http = null,
  }) {
    return _then(_value.copyWith(
      https: null == https
          ? _value.https
          : https // ignore: cast_nullable_to_non_nullable
              as VeilidConfigHTTPS,
      http: null == http
          ? _value.http
          : http // ignore: cast_nullable_to_non_nullable
              as VeilidConfigHTTP,
    ) as $Val);
  }

  @override
  @pragma('vm:prefer-inline')
  $VeilidConfigHTTPSCopyWith<$Res> get https {
    return $VeilidConfigHTTPSCopyWith<$Res>(_value.https, (value) {
      return _then(_value.copyWith(https: value) as $Val);
    });
  }

  @override
  @pragma('vm:prefer-inline')
  $VeilidConfigHTTPCopyWith<$Res> get http {
    return $VeilidConfigHTTPCopyWith<$Res>(_value.http, (value) {
      return _then(_value.copyWith(http: value) as $Val);
    });
  }
}

/// @nodoc
abstract class _$$_VeilidConfigApplicationCopyWith<$Res>
    implements $VeilidConfigApplicationCopyWith<$Res> {
  factory _$$_VeilidConfigApplicationCopyWith(_$_VeilidConfigApplication value,
          $Res Function(_$_VeilidConfigApplication) then) =
      __$$_VeilidConfigApplicationCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({VeilidConfigHTTPS https, VeilidConfigHTTP http});

  @override
  $VeilidConfigHTTPSCopyWith<$Res> get https;
  @override
  $VeilidConfigHTTPCopyWith<$Res> get http;
}

/// @nodoc
class __$$_VeilidConfigApplicationCopyWithImpl<$Res>
    extends _$VeilidConfigApplicationCopyWithImpl<$Res,
        _$_VeilidConfigApplication>
    implements _$$_VeilidConfigApplicationCopyWith<$Res> {
  __$$_VeilidConfigApplicationCopyWithImpl(_$_VeilidConfigApplication _value,
      $Res Function(_$_VeilidConfigApplication) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? https = null,
    Object? http = null,
  }) {
    return _then(_$_VeilidConfigApplication(
      https: null == https
          ? _value.https
          : https // ignore: cast_nullable_to_non_nullable
              as VeilidConfigHTTPS,
      http: null == http
          ? _value.http
          : http // ignore: cast_nullable_to_non_nullable
              as VeilidConfigHTTP,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$_VeilidConfigApplication
    with DiagnosticableTreeMixin
    implements _VeilidConfigApplication {
  const _$_VeilidConfigApplication({required this.https, required this.http});

  factory _$_VeilidConfigApplication.fromJson(Map<String, dynamic> json) =>
      _$$_VeilidConfigApplicationFromJson(json);

  @override
  final VeilidConfigHTTPS https;
  @override
  final VeilidConfigHTTP http;

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidConfigApplication(https: $https, http: $http)';
  }

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
    super.debugFillProperties(properties);
    properties
      ..add(DiagnosticsProperty('type', 'VeilidConfigApplication'))
      ..add(DiagnosticsProperty('https', https))
      ..add(DiagnosticsProperty('http', http));
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$_VeilidConfigApplication &&
            (identical(other.https, https) || other.https == https) &&
            (identical(other.http, http) || other.http == http));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, https, http);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$_VeilidConfigApplicationCopyWith<_$_VeilidConfigApplication>
      get copyWith =>
          __$$_VeilidConfigApplicationCopyWithImpl<_$_VeilidConfigApplication>(
              this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$_VeilidConfigApplicationToJson(
      this,
    );
  }
}

abstract class _VeilidConfigApplication implements VeilidConfigApplication {
  const factory _VeilidConfigApplication(
      {required final VeilidConfigHTTPS https,
      required final VeilidConfigHTTP http}) = _$_VeilidConfigApplication;

  factory _VeilidConfigApplication.fromJson(Map<String, dynamic> json) =
      _$_VeilidConfigApplication.fromJson;

  @override
  VeilidConfigHTTPS get https;
  @override
  VeilidConfigHTTP get http;
  @override
  @JsonKey(ignore: true)
  _$$_VeilidConfigApplicationCopyWith<_$_VeilidConfigApplication>
      get copyWith => throw _privateConstructorUsedError;
}

VeilidConfigUDP _$VeilidConfigUDPFromJson(Map<String, dynamic> json) {
  return _VeilidConfigUDP.fromJson(json);
}

/// @nodoc
mixin _$VeilidConfigUDP {
  bool get enabled => throw _privateConstructorUsedError;
  int get socketPoolSize => throw _privateConstructorUsedError;
  String get listenAddress => throw _privateConstructorUsedError;
  String? get publicAddress => throw _privateConstructorUsedError;

  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
  @JsonKey(ignore: true)
  $VeilidConfigUDPCopyWith<VeilidConfigUDP> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $VeilidConfigUDPCopyWith<$Res> {
  factory $VeilidConfigUDPCopyWith(
          VeilidConfigUDP value, $Res Function(VeilidConfigUDP) then) =
      _$VeilidConfigUDPCopyWithImpl<$Res, VeilidConfigUDP>;
  @useResult
  $Res call(
      {bool enabled,
      int socketPoolSize,
      String listenAddress,
      String? publicAddress});
}

/// @nodoc
class _$VeilidConfigUDPCopyWithImpl<$Res, $Val extends VeilidConfigUDP>
    implements $VeilidConfigUDPCopyWith<$Res> {
  _$VeilidConfigUDPCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? enabled = null,
    Object? socketPoolSize = null,
    Object? listenAddress = null,
    Object? publicAddress = freezed,
  }) {
    return _then(_value.copyWith(
      enabled: null == enabled
          ? _value.enabled
          : enabled // ignore: cast_nullable_to_non_nullable
              as bool,
      socketPoolSize: null == socketPoolSize
          ? _value.socketPoolSize
          : socketPoolSize // ignore: cast_nullable_to_non_nullable
              as int,
      listenAddress: null == listenAddress
          ? _value.listenAddress
          : listenAddress // ignore: cast_nullable_to_non_nullable
              as String,
      publicAddress: freezed == publicAddress
          ? _value.publicAddress
          : publicAddress // ignore: cast_nullable_to_non_nullable
              as String?,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$_VeilidConfigUDPCopyWith<$Res>
    implements $VeilidConfigUDPCopyWith<$Res> {
  factory _$$_VeilidConfigUDPCopyWith(
          _$_VeilidConfigUDP value, $Res Function(_$_VeilidConfigUDP) then) =
      __$$_VeilidConfigUDPCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {bool enabled,
      int socketPoolSize,
      String listenAddress,
      String? publicAddress});
}

/// @nodoc
class __$$_VeilidConfigUDPCopyWithImpl<$Res>
    extends _$VeilidConfigUDPCopyWithImpl<$Res, _$_VeilidConfigUDP>
    implements _$$_VeilidConfigUDPCopyWith<$Res> {
  __$$_VeilidConfigUDPCopyWithImpl(
      _$_VeilidConfigUDP _value, $Res Function(_$_VeilidConfigUDP) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? enabled = null,
    Object? socketPoolSize = null,
    Object? listenAddress = null,
    Object? publicAddress = freezed,
  }) {
    return _then(_$_VeilidConfigUDP(
      enabled: null == enabled
          ? _value.enabled
          : enabled // ignore: cast_nullable_to_non_nullable
              as bool,
      socketPoolSize: null == socketPoolSize
          ? _value.socketPoolSize
          : socketPoolSize // ignore: cast_nullable_to_non_nullable
              as int,
      listenAddress: null == listenAddress
          ? _value.listenAddress
          : listenAddress // ignore: cast_nullable_to_non_nullable
              as String,
      publicAddress: freezed == publicAddress
          ? _value.publicAddress
          : publicAddress // ignore: cast_nullable_to_non_nullable
              as String?,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$_VeilidConfigUDP
    with DiagnosticableTreeMixin
    implements _VeilidConfigUDP {
  const _$_VeilidConfigUDP(
      {required this.enabled,
      required this.socketPoolSize,
      required this.listenAddress,
      this.publicAddress});

  factory _$_VeilidConfigUDP.fromJson(Map<String, dynamic> json) =>
      _$$_VeilidConfigUDPFromJson(json);

  @override
  final bool enabled;
  @override
  final int socketPoolSize;
  @override
  final String listenAddress;
  @override
  final String? publicAddress;

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidConfigUDP(enabled: $enabled, socketPoolSize: $socketPoolSize, listenAddress: $listenAddress, publicAddress: $publicAddress)';
  }

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
    super.debugFillProperties(properties);
    properties
      ..add(DiagnosticsProperty('type', 'VeilidConfigUDP'))
      ..add(DiagnosticsProperty('enabled', enabled))
      ..add(DiagnosticsProperty('socketPoolSize', socketPoolSize))
      ..add(DiagnosticsProperty('listenAddress', listenAddress))
      ..add(DiagnosticsProperty('publicAddress', publicAddress));
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$_VeilidConfigUDP &&
            (identical(other.enabled, enabled) || other.enabled == enabled) &&
            (identical(other.socketPoolSize, socketPoolSize) ||
                other.socketPoolSize == socketPoolSize) &&
            (identical(other.listenAddress, listenAddress) ||
                other.listenAddress == listenAddress) &&
            (identical(other.publicAddress, publicAddress) ||
                other.publicAddress == publicAddress));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(
      runtimeType, enabled, socketPoolSize, listenAddress, publicAddress);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$_VeilidConfigUDPCopyWith<_$_VeilidConfigUDP> get copyWith =>
      __$$_VeilidConfigUDPCopyWithImpl<_$_VeilidConfigUDP>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$_VeilidConfigUDPToJson(
      this,
    );
  }
}

abstract class _VeilidConfigUDP implements VeilidConfigUDP {
  const factory _VeilidConfigUDP(
      {required final bool enabled,
      required final int socketPoolSize,
      required final String listenAddress,
      final String? publicAddress}) = _$_VeilidConfigUDP;

  factory _VeilidConfigUDP.fromJson(Map<String, dynamic> json) =
      _$_VeilidConfigUDP.fromJson;

  @override
  bool get enabled;
  @override
  int get socketPoolSize;
  @override
  String get listenAddress;
  @override
  String? get publicAddress;
  @override
  @JsonKey(ignore: true)
  _$$_VeilidConfigUDPCopyWith<_$_VeilidConfigUDP> get copyWith =>
      throw _privateConstructorUsedError;
}

VeilidConfigTCP _$VeilidConfigTCPFromJson(Map<String, dynamic> json) {
  return _VeilidConfigTCP.fromJson(json);
}

/// @nodoc
mixin _$VeilidConfigTCP {
  bool get connect => throw _privateConstructorUsedError;
  bool get listen => throw _privateConstructorUsedError;
  int get maxConnections => throw _privateConstructorUsedError;
  String get listenAddress => throw _privateConstructorUsedError;
  String? get publicAddress => throw _privateConstructorUsedError;

  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
  @JsonKey(ignore: true)
  $VeilidConfigTCPCopyWith<VeilidConfigTCP> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $VeilidConfigTCPCopyWith<$Res> {
  factory $VeilidConfigTCPCopyWith(
          VeilidConfigTCP value, $Res Function(VeilidConfigTCP) then) =
      _$VeilidConfigTCPCopyWithImpl<$Res, VeilidConfigTCP>;
  @useResult
  $Res call(
      {bool connect,
      bool listen,
      int maxConnections,
      String listenAddress,
      String? publicAddress});
}

/// @nodoc
class _$VeilidConfigTCPCopyWithImpl<$Res, $Val extends VeilidConfigTCP>
    implements $VeilidConfigTCPCopyWith<$Res> {
  _$VeilidConfigTCPCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? connect = null,
    Object? listen = null,
    Object? maxConnections = null,
    Object? listenAddress = null,
    Object? publicAddress = freezed,
  }) {
    return _then(_value.copyWith(
      connect: null == connect
          ? _value.connect
          : connect // ignore: cast_nullable_to_non_nullable
              as bool,
      listen: null == listen
          ? _value.listen
          : listen // ignore: cast_nullable_to_non_nullable
              as bool,
      maxConnections: null == maxConnections
          ? _value.maxConnections
          : maxConnections // ignore: cast_nullable_to_non_nullable
              as int,
      listenAddress: null == listenAddress
          ? _value.listenAddress
          : listenAddress // ignore: cast_nullable_to_non_nullable
              as String,
      publicAddress: freezed == publicAddress
          ? _value.publicAddress
          : publicAddress // ignore: cast_nullable_to_non_nullable
              as String?,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$_VeilidConfigTCPCopyWith<$Res>
    implements $VeilidConfigTCPCopyWith<$Res> {
  factory _$$_VeilidConfigTCPCopyWith(
          _$_VeilidConfigTCP value, $Res Function(_$_VeilidConfigTCP) then) =
      __$$_VeilidConfigTCPCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {bool connect,
      bool listen,
      int maxConnections,
      String listenAddress,
      String? publicAddress});
}

/// @nodoc
class __$$_VeilidConfigTCPCopyWithImpl<$Res>
    extends _$VeilidConfigTCPCopyWithImpl<$Res, _$_VeilidConfigTCP>
    implements _$$_VeilidConfigTCPCopyWith<$Res> {
  __$$_VeilidConfigTCPCopyWithImpl(
      _$_VeilidConfigTCP _value, $Res Function(_$_VeilidConfigTCP) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? connect = null,
    Object? listen = null,
    Object? maxConnections = null,
    Object? listenAddress = null,
    Object? publicAddress = freezed,
  }) {
    return _then(_$_VeilidConfigTCP(
      connect: null == connect
          ? _value.connect
          : connect // ignore: cast_nullable_to_non_nullable
              as bool,
      listen: null == listen
          ? _value.listen
          : listen // ignore: cast_nullable_to_non_nullable
              as bool,
      maxConnections: null == maxConnections
          ? _value.maxConnections
          : maxConnections // ignore: cast_nullable_to_non_nullable
              as int,
      listenAddress: null == listenAddress
          ? _value.listenAddress
          : listenAddress // ignore: cast_nullable_to_non_nullable
              as String,
      publicAddress: freezed == publicAddress
          ? _value.publicAddress
          : publicAddress // ignore: cast_nullable_to_non_nullable
              as String?,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$_VeilidConfigTCP
    with DiagnosticableTreeMixin
    implements _VeilidConfigTCP {
  const _$_VeilidConfigTCP(
      {required this.connect,
      required this.listen,
      required this.maxConnections,
      required this.listenAddress,
      this.publicAddress});

  factory _$_VeilidConfigTCP.fromJson(Map<String, dynamic> json) =>
      _$$_VeilidConfigTCPFromJson(json);

  @override
  final bool connect;
  @override
  final bool listen;
  @override
  final int maxConnections;
  @override
  final String listenAddress;
  @override
  final String? publicAddress;

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidConfigTCP(connect: $connect, listen: $listen, maxConnections: $maxConnections, listenAddress: $listenAddress, publicAddress: $publicAddress)';
  }

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
    super.debugFillProperties(properties);
    properties
      ..add(DiagnosticsProperty('type', 'VeilidConfigTCP'))
      ..add(DiagnosticsProperty('connect', connect))
      ..add(DiagnosticsProperty('listen', listen))
      ..add(DiagnosticsProperty('maxConnections', maxConnections))
      ..add(DiagnosticsProperty('listenAddress', listenAddress))
      ..add(DiagnosticsProperty('publicAddress', publicAddress));
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$_VeilidConfigTCP &&
            (identical(other.connect, connect) || other.connect == connect) &&
            (identical(other.listen, listen) || other.listen == listen) &&
            (identical(other.maxConnections, maxConnections) ||
                other.maxConnections == maxConnections) &&
            (identical(other.listenAddress, listenAddress) ||
                other.listenAddress == listenAddress) &&
            (identical(other.publicAddress, publicAddress) ||
                other.publicAddress == publicAddress));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, connect, listen, maxConnections,
      listenAddress, publicAddress);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$_VeilidConfigTCPCopyWith<_$_VeilidConfigTCP> get copyWith =>
      __$$_VeilidConfigTCPCopyWithImpl<_$_VeilidConfigTCP>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$_VeilidConfigTCPToJson(
      this,
    );
  }
}

abstract class _VeilidConfigTCP implements VeilidConfigTCP {
  const factory _VeilidConfigTCP(
      {required final bool connect,
      required final bool listen,
      required final int maxConnections,
      required final String listenAddress,
      final String? publicAddress}) = _$_VeilidConfigTCP;

  factory _VeilidConfigTCP.fromJson(Map<String, dynamic> json) =
      _$_VeilidConfigTCP.fromJson;

  @override
  bool get connect;
  @override
  bool get listen;
  @override
  int get maxConnections;
  @override
  String get listenAddress;
  @override
  String? get publicAddress;
  @override
  @JsonKey(ignore: true)
  _$$_VeilidConfigTCPCopyWith<_$_VeilidConfigTCP> get copyWith =>
      throw _privateConstructorUsedError;
}

VeilidConfigWS _$VeilidConfigWSFromJson(Map<String, dynamic> json) {
  return _VeilidConfigWS.fromJson(json);
}

/// @nodoc
mixin _$VeilidConfigWS {
  bool get connect => throw _privateConstructorUsedError;
  bool get listen => throw _privateConstructorUsedError;
  int get maxConnections => throw _privateConstructorUsedError;
  String get listenAddress => throw _privateConstructorUsedError;
  String get path => throw _privateConstructorUsedError;
  String? get url => throw _privateConstructorUsedError;

  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
  @JsonKey(ignore: true)
  $VeilidConfigWSCopyWith<VeilidConfigWS> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $VeilidConfigWSCopyWith<$Res> {
  factory $VeilidConfigWSCopyWith(
          VeilidConfigWS value, $Res Function(VeilidConfigWS) then) =
      _$VeilidConfigWSCopyWithImpl<$Res, VeilidConfigWS>;
  @useResult
  $Res call(
      {bool connect,
      bool listen,
      int maxConnections,
      String listenAddress,
      String path,
      String? url});
}

/// @nodoc
class _$VeilidConfigWSCopyWithImpl<$Res, $Val extends VeilidConfigWS>
    implements $VeilidConfigWSCopyWith<$Res> {
  _$VeilidConfigWSCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? connect = null,
    Object? listen = null,
    Object? maxConnections = null,
    Object? listenAddress = null,
    Object? path = null,
    Object? url = freezed,
  }) {
    return _then(_value.copyWith(
      connect: null == connect
          ? _value.connect
          : connect // ignore: cast_nullable_to_non_nullable
              as bool,
      listen: null == listen
          ? _value.listen
          : listen // ignore: cast_nullable_to_non_nullable
              as bool,
      maxConnections: null == maxConnections
          ? _value.maxConnections
          : maxConnections // ignore: cast_nullable_to_non_nullable
              as int,
      listenAddress: null == listenAddress
          ? _value.listenAddress
          : listenAddress // ignore: cast_nullable_to_non_nullable
              as String,
      path: null == path
          ? _value.path
          : path // ignore: cast_nullable_to_non_nullable
              as String,
      url: freezed == url
          ? _value.url
          : url // ignore: cast_nullable_to_non_nullable
              as String?,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$_VeilidConfigWSCopyWith<$Res>
    implements $VeilidConfigWSCopyWith<$Res> {
  factory _$$_VeilidConfigWSCopyWith(
          _$_VeilidConfigWS value, $Res Function(_$_VeilidConfigWS) then) =
      __$$_VeilidConfigWSCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {bool connect,
      bool listen,
      int maxConnections,
      String listenAddress,
      String path,
      String? url});
}

/// @nodoc
class __$$_VeilidConfigWSCopyWithImpl<$Res>
    extends _$VeilidConfigWSCopyWithImpl<$Res, _$_VeilidConfigWS>
    implements _$$_VeilidConfigWSCopyWith<$Res> {
  __$$_VeilidConfigWSCopyWithImpl(
      _$_VeilidConfigWS _value, $Res Function(_$_VeilidConfigWS) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? connect = null,
    Object? listen = null,
    Object? maxConnections = null,
    Object? listenAddress = null,
    Object? path = null,
    Object? url = freezed,
  }) {
    return _then(_$_VeilidConfigWS(
      connect: null == connect
          ? _value.connect
          : connect // ignore: cast_nullable_to_non_nullable
              as bool,
      listen: null == listen
          ? _value.listen
          : listen // ignore: cast_nullable_to_non_nullable
              as bool,
      maxConnections: null == maxConnections
          ? _value.maxConnections
          : maxConnections // ignore: cast_nullable_to_non_nullable
              as int,
      listenAddress: null == listenAddress
          ? _value.listenAddress
          : listenAddress // ignore: cast_nullable_to_non_nullable
              as String,
      path: null == path
          ? _value.path
          : path // ignore: cast_nullable_to_non_nullable
              as String,
      url: freezed == url
          ? _value.url
          : url // ignore: cast_nullable_to_non_nullable
              as String?,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$_VeilidConfigWS
    with DiagnosticableTreeMixin
    implements _VeilidConfigWS {
  const _$_VeilidConfigWS(
      {required this.connect,
      required this.listen,
      required this.maxConnections,
      required this.listenAddress,
      required this.path,
      this.url});

  factory _$_VeilidConfigWS.fromJson(Map<String, dynamic> json) =>
      _$$_VeilidConfigWSFromJson(json);

  @override
  final bool connect;
  @override
  final bool listen;
  @override
  final int maxConnections;
  @override
  final String listenAddress;
  @override
  final String path;
  @override
  final String? url;

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidConfigWS(connect: $connect, listen: $listen, maxConnections: $maxConnections, listenAddress: $listenAddress, path: $path, url: $url)';
  }

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
    super.debugFillProperties(properties);
    properties
      ..add(DiagnosticsProperty('type', 'VeilidConfigWS'))
      ..add(DiagnosticsProperty('connect', connect))
      ..add(DiagnosticsProperty('listen', listen))
      ..add(DiagnosticsProperty('maxConnections', maxConnections))
      ..add(DiagnosticsProperty('listenAddress', listenAddress))
      ..add(DiagnosticsProperty('path', path))
      ..add(DiagnosticsProperty('url', url));
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$_VeilidConfigWS &&
            (identical(other.connect, connect) || other.connect == connect) &&
            (identical(other.listen, listen) || other.listen == listen) &&
            (identical(other.maxConnections, maxConnections) ||
                other.maxConnections == maxConnections) &&
            (identical(other.listenAddress, listenAddress) ||
                other.listenAddress == listenAddress) &&
            (identical(other.path, path) || other.path == path) &&
            (identical(other.url, url) || other.url == url));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(
      runtimeType, connect, listen, maxConnections, listenAddress, path, url);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$_VeilidConfigWSCopyWith<_$_VeilidConfigWS> get copyWith =>
      __$$_VeilidConfigWSCopyWithImpl<_$_VeilidConfigWS>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$_VeilidConfigWSToJson(
      this,
    );
  }
}

abstract class _VeilidConfigWS implements VeilidConfigWS {
  const factory _VeilidConfigWS(
      {required final bool connect,
      required final bool listen,
      required final int maxConnections,
      required final String listenAddress,
      required final String path,
      final String? url}) = _$_VeilidConfigWS;

  factory _VeilidConfigWS.fromJson(Map<String, dynamic> json) =
      _$_VeilidConfigWS.fromJson;

  @override
  bool get connect;
  @override
  bool get listen;
  @override
  int get maxConnections;
  @override
  String get listenAddress;
  @override
  String get path;
  @override
  String? get url;
  @override
  @JsonKey(ignore: true)
  _$$_VeilidConfigWSCopyWith<_$_VeilidConfigWS> get copyWith =>
      throw _privateConstructorUsedError;
}

VeilidConfigWSS _$VeilidConfigWSSFromJson(Map<String, dynamic> json) {
  return _VeilidConfigWSS.fromJson(json);
}

/// @nodoc
mixin _$VeilidConfigWSS {
  bool get connect => throw _privateConstructorUsedError;
  bool get listen => throw _privateConstructorUsedError;
  int get maxConnections => throw _privateConstructorUsedError;
  String get listenAddress => throw _privateConstructorUsedError;
  String get path => throw _privateConstructorUsedError;
  String? get url => throw _privateConstructorUsedError;

  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
  @JsonKey(ignore: true)
  $VeilidConfigWSSCopyWith<VeilidConfigWSS> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $VeilidConfigWSSCopyWith<$Res> {
  factory $VeilidConfigWSSCopyWith(
          VeilidConfigWSS value, $Res Function(VeilidConfigWSS) then) =
      _$VeilidConfigWSSCopyWithImpl<$Res, VeilidConfigWSS>;
  @useResult
  $Res call(
      {bool connect,
      bool listen,
      int maxConnections,
      String listenAddress,
      String path,
      String? url});
}

/// @nodoc
class _$VeilidConfigWSSCopyWithImpl<$Res, $Val extends VeilidConfigWSS>
    implements $VeilidConfigWSSCopyWith<$Res> {
  _$VeilidConfigWSSCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? connect = null,
    Object? listen = null,
    Object? maxConnections = null,
    Object? listenAddress = null,
    Object? path = null,
    Object? url = freezed,
  }) {
    return _then(_value.copyWith(
      connect: null == connect
          ? _value.connect
          : connect // ignore: cast_nullable_to_non_nullable
              as bool,
      listen: null == listen
          ? _value.listen
          : listen // ignore: cast_nullable_to_non_nullable
              as bool,
      maxConnections: null == maxConnections
          ? _value.maxConnections
          : maxConnections // ignore: cast_nullable_to_non_nullable
              as int,
      listenAddress: null == listenAddress
          ? _value.listenAddress
          : listenAddress // ignore: cast_nullable_to_non_nullable
              as String,
      path: null == path
          ? _value.path
          : path // ignore: cast_nullable_to_non_nullable
              as String,
      url: freezed == url
          ? _value.url
          : url // ignore: cast_nullable_to_non_nullable
              as String?,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$_VeilidConfigWSSCopyWith<$Res>
    implements $VeilidConfigWSSCopyWith<$Res> {
  factory _$$_VeilidConfigWSSCopyWith(
          _$_VeilidConfigWSS value, $Res Function(_$_VeilidConfigWSS) then) =
      __$$_VeilidConfigWSSCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {bool connect,
      bool listen,
      int maxConnections,
      String listenAddress,
      String path,
      String? url});
}

/// @nodoc
class __$$_VeilidConfigWSSCopyWithImpl<$Res>
    extends _$VeilidConfigWSSCopyWithImpl<$Res, _$_VeilidConfigWSS>
    implements _$$_VeilidConfigWSSCopyWith<$Res> {
  __$$_VeilidConfigWSSCopyWithImpl(
      _$_VeilidConfigWSS _value, $Res Function(_$_VeilidConfigWSS) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? connect = null,
    Object? listen = null,
    Object? maxConnections = null,
    Object? listenAddress = null,
    Object? path = null,
    Object? url = freezed,
  }) {
    return _then(_$_VeilidConfigWSS(
      connect: null == connect
          ? _value.connect
          : connect // ignore: cast_nullable_to_non_nullable
              as bool,
      listen: null == listen
          ? _value.listen
          : listen // ignore: cast_nullable_to_non_nullable
              as bool,
      maxConnections: null == maxConnections
          ? _value.maxConnections
          : maxConnections // ignore: cast_nullable_to_non_nullable
              as int,
      listenAddress: null == listenAddress
          ? _value.listenAddress
          : listenAddress // ignore: cast_nullable_to_non_nullable
              as String,
      path: null == path
          ? _value.path
          : path // ignore: cast_nullable_to_non_nullable
              as String,
      url: freezed == url
          ? _value.url
          : url // ignore: cast_nullable_to_non_nullable
              as String?,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$_VeilidConfigWSS
    with DiagnosticableTreeMixin
    implements _VeilidConfigWSS {
  const _$_VeilidConfigWSS(
      {required this.connect,
      required this.listen,
      required this.maxConnections,
      required this.listenAddress,
      required this.path,
      this.url});

  factory _$_VeilidConfigWSS.fromJson(Map<String, dynamic> json) =>
      _$$_VeilidConfigWSSFromJson(json);

  @override
  final bool connect;
  @override
  final bool listen;
  @override
  final int maxConnections;
  @override
  final String listenAddress;
  @override
  final String path;
  @override
  final String? url;

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidConfigWSS(connect: $connect, listen: $listen, maxConnections: $maxConnections, listenAddress: $listenAddress, path: $path, url: $url)';
  }

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
    super.debugFillProperties(properties);
    properties
      ..add(DiagnosticsProperty('type', 'VeilidConfigWSS'))
      ..add(DiagnosticsProperty('connect', connect))
      ..add(DiagnosticsProperty('listen', listen))
      ..add(DiagnosticsProperty('maxConnections', maxConnections))
      ..add(DiagnosticsProperty('listenAddress', listenAddress))
      ..add(DiagnosticsProperty('path', path))
      ..add(DiagnosticsProperty('url', url));
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$_VeilidConfigWSS &&
            (identical(other.connect, connect) || other.connect == connect) &&
            (identical(other.listen, listen) || other.listen == listen) &&
            (identical(other.maxConnections, maxConnections) ||
                other.maxConnections == maxConnections) &&
            (identical(other.listenAddress, listenAddress) ||
                other.listenAddress == listenAddress) &&
            (identical(other.path, path) || other.path == path) &&
            (identical(other.url, url) || other.url == url));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(
      runtimeType, connect, listen, maxConnections, listenAddress, path, url);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$_VeilidConfigWSSCopyWith<_$_VeilidConfigWSS> get copyWith =>
      __$$_VeilidConfigWSSCopyWithImpl<_$_VeilidConfigWSS>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$_VeilidConfigWSSToJson(
      this,
    );
  }
}

abstract class _VeilidConfigWSS implements VeilidConfigWSS {
  const factory _VeilidConfigWSS(
      {required final bool connect,
      required final bool listen,
      required final int maxConnections,
      required final String listenAddress,
      required final String path,
      final String? url}) = _$_VeilidConfigWSS;

  factory _VeilidConfigWSS.fromJson(Map<String, dynamic> json) =
      _$_VeilidConfigWSS.fromJson;

  @override
  bool get connect;
  @override
  bool get listen;
  @override
  int get maxConnections;
  @override
  String get listenAddress;
  @override
  String get path;
  @override
  String? get url;
  @override
  @JsonKey(ignore: true)
  _$$_VeilidConfigWSSCopyWith<_$_VeilidConfigWSS> get copyWith =>
      throw _privateConstructorUsedError;
}

VeilidConfigProtocol _$VeilidConfigProtocolFromJson(Map<String, dynamic> json) {
  return _VeilidConfigProtocol.fromJson(json);
}

/// @nodoc
mixin _$VeilidConfigProtocol {
  VeilidConfigUDP get udp => throw _privateConstructorUsedError;
  VeilidConfigTCP get tcp => throw _privateConstructorUsedError;
  VeilidConfigWS get ws => throw _privateConstructorUsedError;
  VeilidConfigWSS get wss => throw _privateConstructorUsedError;

  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
  @JsonKey(ignore: true)
  $VeilidConfigProtocolCopyWith<VeilidConfigProtocol> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $VeilidConfigProtocolCopyWith<$Res> {
  factory $VeilidConfigProtocolCopyWith(VeilidConfigProtocol value,
          $Res Function(VeilidConfigProtocol) then) =
      _$VeilidConfigProtocolCopyWithImpl<$Res, VeilidConfigProtocol>;
  @useResult
  $Res call(
      {VeilidConfigUDP udp,
      VeilidConfigTCP tcp,
      VeilidConfigWS ws,
      VeilidConfigWSS wss});

  $VeilidConfigUDPCopyWith<$Res> get udp;
  $VeilidConfigTCPCopyWith<$Res> get tcp;
  $VeilidConfigWSCopyWith<$Res> get ws;
  $VeilidConfigWSSCopyWith<$Res> get wss;
}

/// @nodoc
class _$VeilidConfigProtocolCopyWithImpl<$Res,
        $Val extends VeilidConfigProtocol>
    implements $VeilidConfigProtocolCopyWith<$Res> {
  _$VeilidConfigProtocolCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? udp = null,
    Object? tcp = null,
    Object? ws = null,
    Object? wss = null,
  }) {
    return _then(_value.copyWith(
      udp: null == udp
          ? _value.udp
          : udp // ignore: cast_nullable_to_non_nullable
              as VeilidConfigUDP,
      tcp: null == tcp
          ? _value.tcp
          : tcp // ignore: cast_nullable_to_non_nullable
              as VeilidConfigTCP,
      ws: null == ws
          ? _value.ws
          : ws // ignore: cast_nullable_to_non_nullable
              as VeilidConfigWS,
      wss: null == wss
          ? _value.wss
          : wss // ignore: cast_nullable_to_non_nullable
              as VeilidConfigWSS,
    ) as $Val);
  }

  @override
  @pragma('vm:prefer-inline')
  $VeilidConfigUDPCopyWith<$Res> get udp {
    return $VeilidConfigUDPCopyWith<$Res>(_value.udp, (value) {
      return _then(_value.copyWith(udp: value) as $Val);
    });
  }

  @override
  @pragma('vm:prefer-inline')
  $VeilidConfigTCPCopyWith<$Res> get tcp {
    return $VeilidConfigTCPCopyWith<$Res>(_value.tcp, (value) {
      return _then(_value.copyWith(tcp: value) as $Val);
    });
  }

  @override
  @pragma('vm:prefer-inline')
  $VeilidConfigWSCopyWith<$Res> get ws {
    return $VeilidConfigWSCopyWith<$Res>(_value.ws, (value) {
      return _then(_value.copyWith(ws: value) as $Val);
    });
  }

  @override
  @pragma('vm:prefer-inline')
  $VeilidConfigWSSCopyWith<$Res> get wss {
    return $VeilidConfigWSSCopyWith<$Res>(_value.wss, (value) {
      return _then(_value.copyWith(wss: value) as $Val);
    });
  }
}

/// @nodoc
abstract class _$$_VeilidConfigProtocolCopyWith<$Res>
    implements $VeilidConfigProtocolCopyWith<$Res> {
  factory _$$_VeilidConfigProtocolCopyWith(_$_VeilidConfigProtocol value,
          $Res Function(_$_VeilidConfigProtocol) then) =
      __$$_VeilidConfigProtocolCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {VeilidConfigUDP udp,
      VeilidConfigTCP tcp,
      VeilidConfigWS ws,
      VeilidConfigWSS wss});

  @override
  $VeilidConfigUDPCopyWith<$Res> get udp;
  @override
  $VeilidConfigTCPCopyWith<$Res> get tcp;
  @override
  $VeilidConfigWSCopyWith<$Res> get ws;
  @override
  $VeilidConfigWSSCopyWith<$Res> get wss;
}

/// @nodoc
class __$$_VeilidConfigProtocolCopyWithImpl<$Res>
    extends _$VeilidConfigProtocolCopyWithImpl<$Res, _$_VeilidConfigProtocol>
    implements _$$_VeilidConfigProtocolCopyWith<$Res> {
  __$$_VeilidConfigProtocolCopyWithImpl(_$_VeilidConfigProtocol _value,
      $Res Function(_$_VeilidConfigProtocol) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? udp = null,
    Object? tcp = null,
    Object? ws = null,
    Object? wss = null,
  }) {
    return _then(_$_VeilidConfigProtocol(
      udp: null == udp
          ? _value.udp
          : udp // ignore: cast_nullable_to_non_nullable
              as VeilidConfigUDP,
      tcp: null == tcp
          ? _value.tcp
          : tcp // ignore: cast_nullable_to_non_nullable
              as VeilidConfigTCP,
      ws: null == ws
          ? _value.ws
          : ws // ignore: cast_nullable_to_non_nullable
              as VeilidConfigWS,
      wss: null == wss
          ? _value.wss
          : wss // ignore: cast_nullable_to_non_nullable
              as VeilidConfigWSS,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$_VeilidConfigProtocol
    with DiagnosticableTreeMixin
    implements _VeilidConfigProtocol {
  const _$_VeilidConfigProtocol(
      {required this.udp,
      required this.tcp,
      required this.ws,
      required this.wss});

  factory _$_VeilidConfigProtocol.fromJson(Map<String, dynamic> json) =>
      _$$_VeilidConfigProtocolFromJson(json);

  @override
  final VeilidConfigUDP udp;
  @override
  final VeilidConfigTCP tcp;
  @override
  final VeilidConfigWS ws;
  @override
  final VeilidConfigWSS wss;

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidConfigProtocol(udp: $udp, tcp: $tcp, ws: $ws, wss: $wss)';
  }

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
    super.debugFillProperties(properties);
    properties
      ..add(DiagnosticsProperty('type', 'VeilidConfigProtocol'))
      ..add(DiagnosticsProperty('udp', udp))
      ..add(DiagnosticsProperty('tcp', tcp))
      ..add(DiagnosticsProperty('ws', ws))
      ..add(DiagnosticsProperty('wss', wss));
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$_VeilidConfigProtocol &&
            (identical(other.udp, udp) || other.udp == udp) &&
            (identical(other.tcp, tcp) || other.tcp == tcp) &&
            (identical(other.ws, ws) || other.ws == ws) &&
            (identical(other.wss, wss) || other.wss == wss));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, udp, tcp, ws, wss);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$_VeilidConfigProtocolCopyWith<_$_VeilidConfigProtocol> get copyWith =>
      __$$_VeilidConfigProtocolCopyWithImpl<_$_VeilidConfigProtocol>(
          this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$_VeilidConfigProtocolToJson(
      this,
    );
  }
}

abstract class _VeilidConfigProtocol implements VeilidConfigProtocol {
  const factory _VeilidConfigProtocol(
      {required final VeilidConfigUDP udp,
      required final VeilidConfigTCP tcp,
      required final VeilidConfigWS ws,
      required final VeilidConfigWSS wss}) = _$_VeilidConfigProtocol;

  factory _VeilidConfigProtocol.fromJson(Map<String, dynamic> json) =
      _$_VeilidConfigProtocol.fromJson;

  @override
  VeilidConfigUDP get udp;
  @override
  VeilidConfigTCP get tcp;
  @override
  VeilidConfigWS get ws;
  @override
  VeilidConfigWSS get wss;
  @override
  @JsonKey(ignore: true)
  _$$_VeilidConfigProtocolCopyWith<_$_VeilidConfigProtocol> get copyWith =>
      throw _privateConstructorUsedError;
}

VeilidConfigTLS _$VeilidConfigTLSFromJson(Map<String, dynamic> json) {
  return _VeilidConfigTLS.fromJson(json);
}

/// @nodoc
mixin _$VeilidConfigTLS {
  String get certificatePath => throw _privateConstructorUsedError;
  String get privateKeyPath => throw _privateConstructorUsedError;
  int get connectionInitialTimeoutMs => throw _privateConstructorUsedError;

  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
  @JsonKey(ignore: true)
  $VeilidConfigTLSCopyWith<VeilidConfigTLS> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $VeilidConfigTLSCopyWith<$Res> {
  factory $VeilidConfigTLSCopyWith(
          VeilidConfigTLS value, $Res Function(VeilidConfigTLS) then) =
      _$VeilidConfigTLSCopyWithImpl<$Res, VeilidConfigTLS>;
  @useResult
  $Res call(
      {String certificatePath,
      String privateKeyPath,
      int connectionInitialTimeoutMs});
}

/// @nodoc
class _$VeilidConfigTLSCopyWithImpl<$Res, $Val extends VeilidConfigTLS>
    implements $VeilidConfigTLSCopyWith<$Res> {
  _$VeilidConfigTLSCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? certificatePath = null,
    Object? privateKeyPath = null,
    Object? connectionInitialTimeoutMs = null,
  }) {
    return _then(_value.copyWith(
      certificatePath: null == certificatePath
          ? _value.certificatePath
          : certificatePath // ignore: cast_nullable_to_non_nullable
              as String,
      privateKeyPath: null == privateKeyPath
          ? _value.privateKeyPath
          : privateKeyPath // ignore: cast_nullable_to_non_nullable
              as String,
      connectionInitialTimeoutMs: null == connectionInitialTimeoutMs
          ? _value.connectionInitialTimeoutMs
          : connectionInitialTimeoutMs // ignore: cast_nullable_to_non_nullable
              as int,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$_VeilidConfigTLSCopyWith<$Res>
    implements $VeilidConfigTLSCopyWith<$Res> {
  factory _$$_VeilidConfigTLSCopyWith(
          _$_VeilidConfigTLS value, $Res Function(_$_VeilidConfigTLS) then) =
      __$$_VeilidConfigTLSCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {String certificatePath,
      String privateKeyPath,
      int connectionInitialTimeoutMs});
}

/// @nodoc
class __$$_VeilidConfigTLSCopyWithImpl<$Res>
    extends _$VeilidConfigTLSCopyWithImpl<$Res, _$_VeilidConfigTLS>
    implements _$$_VeilidConfigTLSCopyWith<$Res> {
  __$$_VeilidConfigTLSCopyWithImpl(
      _$_VeilidConfigTLS _value, $Res Function(_$_VeilidConfigTLS) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? certificatePath = null,
    Object? privateKeyPath = null,
    Object? connectionInitialTimeoutMs = null,
  }) {
    return _then(_$_VeilidConfigTLS(
      certificatePath: null == certificatePath
          ? _value.certificatePath
          : certificatePath // ignore: cast_nullable_to_non_nullable
              as String,
      privateKeyPath: null == privateKeyPath
          ? _value.privateKeyPath
          : privateKeyPath // ignore: cast_nullable_to_non_nullable
              as String,
      connectionInitialTimeoutMs: null == connectionInitialTimeoutMs
          ? _value.connectionInitialTimeoutMs
          : connectionInitialTimeoutMs // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$_VeilidConfigTLS
    with DiagnosticableTreeMixin
    implements _VeilidConfigTLS {
  const _$_VeilidConfigTLS(
      {required this.certificatePath,
      required this.privateKeyPath,
      required this.connectionInitialTimeoutMs});

  factory _$_VeilidConfigTLS.fromJson(Map<String, dynamic> json) =>
      _$$_VeilidConfigTLSFromJson(json);

  @override
  final String certificatePath;
  @override
  final String privateKeyPath;
  @override
  final int connectionInitialTimeoutMs;

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidConfigTLS(certificatePath: $certificatePath, privateKeyPath: $privateKeyPath, connectionInitialTimeoutMs: $connectionInitialTimeoutMs)';
  }

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
    super.debugFillProperties(properties);
    properties
      ..add(DiagnosticsProperty('type', 'VeilidConfigTLS'))
      ..add(DiagnosticsProperty('certificatePath', certificatePath))
      ..add(DiagnosticsProperty('privateKeyPath', privateKeyPath))
      ..add(DiagnosticsProperty(
          'connectionInitialTimeoutMs', connectionInitialTimeoutMs));
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$_VeilidConfigTLS &&
            (identical(other.certificatePath, certificatePath) ||
                other.certificatePath == certificatePath) &&
            (identical(other.privateKeyPath, privateKeyPath) ||
                other.privateKeyPath == privateKeyPath) &&
            (identical(other.connectionInitialTimeoutMs,
                    connectionInitialTimeoutMs) ||
                other.connectionInitialTimeoutMs ==
                    connectionInitialTimeoutMs));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(
      runtimeType, certificatePath, privateKeyPath, connectionInitialTimeoutMs);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$_VeilidConfigTLSCopyWith<_$_VeilidConfigTLS> get copyWith =>
      __$$_VeilidConfigTLSCopyWithImpl<_$_VeilidConfigTLS>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$_VeilidConfigTLSToJson(
      this,
    );
  }
}

abstract class _VeilidConfigTLS implements VeilidConfigTLS {
  const factory _VeilidConfigTLS(
      {required final String certificatePath,
      required final String privateKeyPath,
      required final int connectionInitialTimeoutMs}) = _$_VeilidConfigTLS;

  factory _VeilidConfigTLS.fromJson(Map<String, dynamic> json) =
      _$_VeilidConfigTLS.fromJson;

  @override
  String get certificatePath;
  @override
  String get privateKeyPath;
  @override
  int get connectionInitialTimeoutMs;
  @override
  @JsonKey(ignore: true)
  _$$_VeilidConfigTLSCopyWith<_$_VeilidConfigTLS> get copyWith =>
      throw _privateConstructorUsedError;
}

VeilidConfigDHT _$VeilidConfigDHTFromJson(Map<String, dynamic> json) {
  return _VeilidConfigDHT.fromJson(json);
}

/// @nodoc
mixin _$VeilidConfigDHT {
  int get resolveNodeTimeoutMs => throw _privateConstructorUsedError;
  int get resolveNodeCount => throw _privateConstructorUsedError;
  int get resolveNodeFanout => throw _privateConstructorUsedError;
  int get maxFindNodeCount => throw _privateConstructorUsedError;
  int get getValueTimeoutMs => throw _privateConstructorUsedError;
  int get getValueCount => throw _privateConstructorUsedError;
  int get getValueFanout => throw _privateConstructorUsedError;
  int get setValueTimeoutMs => throw _privateConstructorUsedError;
  int get setValueCount => throw _privateConstructorUsedError;
  int get setValueFanout => throw _privateConstructorUsedError;
  int get minPeerCount => throw _privateConstructorUsedError;
  int get minPeerRefreshTimeMs => throw _privateConstructorUsedError;
  int get validateDialInfoReceiptTimeMs => throw _privateConstructorUsedError;
  int get localSubkeyCacheSize => throw _privateConstructorUsedError;
  int get localMaxSubkeyCacheMemoryMb => throw _privateConstructorUsedError;
  int get remoteSubkeyCacheSize => throw _privateConstructorUsedError;
  int get remoteMaxRecords => throw _privateConstructorUsedError;
  int get remoteMaxSubkeyCacheMemoryMb => throw _privateConstructorUsedError;
  int get remoteMaxStorageSpaceMb => throw _privateConstructorUsedError;

  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
  @JsonKey(ignore: true)
  $VeilidConfigDHTCopyWith<VeilidConfigDHT> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $VeilidConfigDHTCopyWith<$Res> {
  factory $VeilidConfigDHTCopyWith(
          VeilidConfigDHT value, $Res Function(VeilidConfigDHT) then) =
      _$VeilidConfigDHTCopyWithImpl<$Res, VeilidConfigDHT>;
  @useResult
  $Res call(
      {int resolveNodeTimeoutMs,
      int resolveNodeCount,
      int resolveNodeFanout,
      int maxFindNodeCount,
      int getValueTimeoutMs,
      int getValueCount,
      int getValueFanout,
      int setValueTimeoutMs,
      int setValueCount,
      int setValueFanout,
      int minPeerCount,
      int minPeerRefreshTimeMs,
      int validateDialInfoReceiptTimeMs,
      int localSubkeyCacheSize,
      int localMaxSubkeyCacheMemoryMb,
      int remoteSubkeyCacheSize,
      int remoteMaxRecords,
      int remoteMaxSubkeyCacheMemoryMb,
      int remoteMaxStorageSpaceMb});
}

/// @nodoc
class _$VeilidConfigDHTCopyWithImpl<$Res, $Val extends VeilidConfigDHT>
    implements $VeilidConfigDHTCopyWith<$Res> {
  _$VeilidConfigDHTCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? resolveNodeTimeoutMs = null,
    Object? resolveNodeCount = null,
    Object? resolveNodeFanout = null,
    Object? maxFindNodeCount = null,
    Object? getValueTimeoutMs = null,
    Object? getValueCount = null,
    Object? getValueFanout = null,
    Object? setValueTimeoutMs = null,
    Object? setValueCount = null,
    Object? setValueFanout = null,
    Object? minPeerCount = null,
    Object? minPeerRefreshTimeMs = null,
    Object? validateDialInfoReceiptTimeMs = null,
    Object? localSubkeyCacheSize = null,
    Object? localMaxSubkeyCacheMemoryMb = null,
    Object? remoteSubkeyCacheSize = null,
    Object? remoteMaxRecords = null,
    Object? remoteMaxSubkeyCacheMemoryMb = null,
    Object? remoteMaxStorageSpaceMb = null,
  }) {
    return _then(_value.copyWith(
      resolveNodeTimeoutMs: null == resolveNodeTimeoutMs
          ? _value.resolveNodeTimeoutMs
          : resolveNodeTimeoutMs // ignore: cast_nullable_to_non_nullable
              as int,
      resolveNodeCount: null == resolveNodeCount
          ? _value.resolveNodeCount
          : resolveNodeCount // ignore: cast_nullable_to_non_nullable
              as int,
      resolveNodeFanout: null == resolveNodeFanout
          ? _value.resolveNodeFanout
          : resolveNodeFanout // ignore: cast_nullable_to_non_nullable
              as int,
      maxFindNodeCount: null == maxFindNodeCount
          ? _value.maxFindNodeCount
          : maxFindNodeCount // ignore: cast_nullable_to_non_nullable
              as int,
      getValueTimeoutMs: null == getValueTimeoutMs
          ? _value.getValueTimeoutMs
          : getValueTimeoutMs // ignore: cast_nullable_to_non_nullable
              as int,
      getValueCount: null == getValueCount
          ? _value.getValueCount
          : getValueCount // ignore: cast_nullable_to_non_nullable
              as int,
      getValueFanout: null == getValueFanout
          ? _value.getValueFanout
          : getValueFanout // ignore: cast_nullable_to_non_nullable
              as int,
      setValueTimeoutMs: null == setValueTimeoutMs
          ? _value.setValueTimeoutMs
          : setValueTimeoutMs // ignore: cast_nullable_to_non_nullable
              as int,
      setValueCount: null == setValueCount
          ? _value.setValueCount
          : setValueCount // ignore: cast_nullable_to_non_nullable
              as int,
      setValueFanout: null == setValueFanout
          ? _value.setValueFanout
          : setValueFanout // ignore: cast_nullable_to_non_nullable
              as int,
      minPeerCount: null == minPeerCount
          ? _value.minPeerCount
          : minPeerCount // ignore: cast_nullable_to_non_nullable
              as int,
      minPeerRefreshTimeMs: null == minPeerRefreshTimeMs
          ? _value.minPeerRefreshTimeMs
          : minPeerRefreshTimeMs // ignore: cast_nullable_to_non_nullable
              as int,
      validateDialInfoReceiptTimeMs: null == validateDialInfoReceiptTimeMs
          ? _value.validateDialInfoReceiptTimeMs
          : validateDialInfoReceiptTimeMs // ignore: cast_nullable_to_non_nullable
              as int,
      localSubkeyCacheSize: null == localSubkeyCacheSize
          ? _value.localSubkeyCacheSize
          : localSubkeyCacheSize // ignore: cast_nullable_to_non_nullable
              as int,
      localMaxSubkeyCacheMemoryMb: null == localMaxSubkeyCacheMemoryMb
          ? _value.localMaxSubkeyCacheMemoryMb
          : localMaxSubkeyCacheMemoryMb // ignore: cast_nullable_to_non_nullable
              as int,
      remoteSubkeyCacheSize: null == remoteSubkeyCacheSize
          ? _value.remoteSubkeyCacheSize
          : remoteSubkeyCacheSize // ignore: cast_nullable_to_non_nullable
              as int,
      remoteMaxRecords: null == remoteMaxRecords
          ? _value.remoteMaxRecords
          : remoteMaxRecords // ignore: cast_nullable_to_non_nullable
              as int,
      remoteMaxSubkeyCacheMemoryMb: null == remoteMaxSubkeyCacheMemoryMb
          ? _value.remoteMaxSubkeyCacheMemoryMb
          : remoteMaxSubkeyCacheMemoryMb // ignore: cast_nullable_to_non_nullable
              as int,
      remoteMaxStorageSpaceMb: null == remoteMaxStorageSpaceMb
          ? _value.remoteMaxStorageSpaceMb
          : remoteMaxStorageSpaceMb // ignore: cast_nullable_to_non_nullable
              as int,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$_VeilidConfigDHTCopyWith<$Res>
    implements $VeilidConfigDHTCopyWith<$Res> {
  factory _$$_VeilidConfigDHTCopyWith(
          _$_VeilidConfigDHT value, $Res Function(_$_VeilidConfigDHT) then) =
      __$$_VeilidConfigDHTCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {int resolveNodeTimeoutMs,
      int resolveNodeCount,
      int resolveNodeFanout,
      int maxFindNodeCount,
      int getValueTimeoutMs,
      int getValueCount,
      int getValueFanout,
      int setValueTimeoutMs,
      int setValueCount,
      int setValueFanout,
      int minPeerCount,
      int minPeerRefreshTimeMs,
      int validateDialInfoReceiptTimeMs,
      int localSubkeyCacheSize,
      int localMaxSubkeyCacheMemoryMb,
      int remoteSubkeyCacheSize,
      int remoteMaxRecords,
      int remoteMaxSubkeyCacheMemoryMb,
      int remoteMaxStorageSpaceMb});
}

/// @nodoc
class __$$_VeilidConfigDHTCopyWithImpl<$Res>
    extends _$VeilidConfigDHTCopyWithImpl<$Res, _$_VeilidConfigDHT>
    implements _$$_VeilidConfigDHTCopyWith<$Res> {
  __$$_VeilidConfigDHTCopyWithImpl(
      _$_VeilidConfigDHT _value, $Res Function(_$_VeilidConfigDHT) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? resolveNodeTimeoutMs = null,
    Object? resolveNodeCount = null,
    Object? resolveNodeFanout = null,
    Object? maxFindNodeCount = null,
    Object? getValueTimeoutMs = null,
    Object? getValueCount = null,
    Object? getValueFanout = null,
    Object? setValueTimeoutMs = null,
    Object? setValueCount = null,
    Object? setValueFanout = null,
    Object? minPeerCount = null,
    Object? minPeerRefreshTimeMs = null,
    Object? validateDialInfoReceiptTimeMs = null,
    Object? localSubkeyCacheSize = null,
    Object? localMaxSubkeyCacheMemoryMb = null,
    Object? remoteSubkeyCacheSize = null,
    Object? remoteMaxRecords = null,
    Object? remoteMaxSubkeyCacheMemoryMb = null,
    Object? remoteMaxStorageSpaceMb = null,
  }) {
    return _then(_$_VeilidConfigDHT(
      resolveNodeTimeoutMs: null == resolveNodeTimeoutMs
          ? _value.resolveNodeTimeoutMs
          : resolveNodeTimeoutMs // ignore: cast_nullable_to_non_nullable
              as int,
      resolveNodeCount: null == resolveNodeCount
          ? _value.resolveNodeCount
          : resolveNodeCount // ignore: cast_nullable_to_non_nullable
              as int,
      resolveNodeFanout: null == resolveNodeFanout
          ? _value.resolveNodeFanout
          : resolveNodeFanout // ignore: cast_nullable_to_non_nullable
              as int,
      maxFindNodeCount: null == maxFindNodeCount
          ? _value.maxFindNodeCount
          : maxFindNodeCount // ignore: cast_nullable_to_non_nullable
              as int,
      getValueTimeoutMs: null == getValueTimeoutMs
          ? _value.getValueTimeoutMs
          : getValueTimeoutMs // ignore: cast_nullable_to_non_nullable
              as int,
      getValueCount: null == getValueCount
          ? _value.getValueCount
          : getValueCount // ignore: cast_nullable_to_non_nullable
              as int,
      getValueFanout: null == getValueFanout
          ? _value.getValueFanout
          : getValueFanout // ignore: cast_nullable_to_non_nullable
              as int,
      setValueTimeoutMs: null == setValueTimeoutMs
          ? _value.setValueTimeoutMs
          : setValueTimeoutMs // ignore: cast_nullable_to_non_nullable
              as int,
      setValueCount: null == setValueCount
          ? _value.setValueCount
          : setValueCount // ignore: cast_nullable_to_non_nullable
              as int,
      setValueFanout: null == setValueFanout
          ? _value.setValueFanout
          : setValueFanout // ignore: cast_nullable_to_non_nullable
              as int,
      minPeerCount: null == minPeerCount
          ? _value.minPeerCount
          : minPeerCount // ignore: cast_nullable_to_non_nullable
              as int,
      minPeerRefreshTimeMs: null == minPeerRefreshTimeMs
          ? _value.minPeerRefreshTimeMs
          : minPeerRefreshTimeMs // ignore: cast_nullable_to_non_nullable
              as int,
      validateDialInfoReceiptTimeMs: null == validateDialInfoReceiptTimeMs
          ? _value.validateDialInfoReceiptTimeMs
          : validateDialInfoReceiptTimeMs // ignore: cast_nullable_to_non_nullable
              as int,
      localSubkeyCacheSize: null == localSubkeyCacheSize
          ? _value.localSubkeyCacheSize
          : localSubkeyCacheSize // ignore: cast_nullable_to_non_nullable
              as int,
      localMaxSubkeyCacheMemoryMb: null == localMaxSubkeyCacheMemoryMb
          ? _value.localMaxSubkeyCacheMemoryMb
          : localMaxSubkeyCacheMemoryMb // ignore: cast_nullable_to_non_nullable
              as int,
      remoteSubkeyCacheSize: null == remoteSubkeyCacheSize
          ? _value.remoteSubkeyCacheSize
          : remoteSubkeyCacheSize // ignore: cast_nullable_to_non_nullable
              as int,
      remoteMaxRecords: null == remoteMaxRecords
          ? _value.remoteMaxRecords
          : remoteMaxRecords // ignore: cast_nullable_to_non_nullable
              as int,
      remoteMaxSubkeyCacheMemoryMb: null == remoteMaxSubkeyCacheMemoryMb
          ? _value.remoteMaxSubkeyCacheMemoryMb
          : remoteMaxSubkeyCacheMemoryMb // ignore: cast_nullable_to_non_nullable
              as int,
      remoteMaxStorageSpaceMb: null == remoteMaxStorageSpaceMb
          ? _value.remoteMaxStorageSpaceMb
          : remoteMaxStorageSpaceMb // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$_VeilidConfigDHT
    with DiagnosticableTreeMixin
    implements _VeilidConfigDHT {
  const _$_VeilidConfigDHT(
      {required this.resolveNodeTimeoutMs,
      required this.resolveNodeCount,
      required this.resolveNodeFanout,
      required this.maxFindNodeCount,
      required this.getValueTimeoutMs,
      required this.getValueCount,
      required this.getValueFanout,
      required this.setValueTimeoutMs,
      required this.setValueCount,
      required this.setValueFanout,
      required this.minPeerCount,
      required this.minPeerRefreshTimeMs,
      required this.validateDialInfoReceiptTimeMs,
      required this.localSubkeyCacheSize,
      required this.localMaxSubkeyCacheMemoryMb,
      required this.remoteSubkeyCacheSize,
      required this.remoteMaxRecords,
      required this.remoteMaxSubkeyCacheMemoryMb,
      required this.remoteMaxStorageSpaceMb});

  factory _$_VeilidConfigDHT.fromJson(Map<String, dynamic> json) =>
      _$$_VeilidConfigDHTFromJson(json);

  @override
  final int resolveNodeTimeoutMs;
  @override
  final int resolveNodeCount;
  @override
  final int resolveNodeFanout;
  @override
  final int maxFindNodeCount;
  @override
  final int getValueTimeoutMs;
  @override
  final int getValueCount;
  @override
  final int getValueFanout;
  @override
  final int setValueTimeoutMs;
  @override
  final int setValueCount;
  @override
  final int setValueFanout;
  @override
  final int minPeerCount;
  @override
  final int minPeerRefreshTimeMs;
  @override
  final int validateDialInfoReceiptTimeMs;
  @override
  final int localSubkeyCacheSize;
  @override
  final int localMaxSubkeyCacheMemoryMb;
  @override
  final int remoteSubkeyCacheSize;
  @override
  final int remoteMaxRecords;
  @override
  final int remoteMaxSubkeyCacheMemoryMb;
  @override
  final int remoteMaxStorageSpaceMb;

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidConfigDHT(resolveNodeTimeoutMs: $resolveNodeTimeoutMs, resolveNodeCount: $resolveNodeCount, resolveNodeFanout: $resolveNodeFanout, maxFindNodeCount: $maxFindNodeCount, getValueTimeoutMs: $getValueTimeoutMs, getValueCount: $getValueCount, getValueFanout: $getValueFanout, setValueTimeoutMs: $setValueTimeoutMs, setValueCount: $setValueCount, setValueFanout: $setValueFanout, minPeerCount: $minPeerCount, minPeerRefreshTimeMs: $minPeerRefreshTimeMs, validateDialInfoReceiptTimeMs: $validateDialInfoReceiptTimeMs, localSubkeyCacheSize: $localSubkeyCacheSize, localMaxSubkeyCacheMemoryMb: $localMaxSubkeyCacheMemoryMb, remoteSubkeyCacheSize: $remoteSubkeyCacheSize, remoteMaxRecords: $remoteMaxRecords, remoteMaxSubkeyCacheMemoryMb: $remoteMaxSubkeyCacheMemoryMb, remoteMaxStorageSpaceMb: $remoteMaxStorageSpaceMb)';
  }

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
    super.debugFillProperties(properties);
    properties
      ..add(DiagnosticsProperty('type', 'VeilidConfigDHT'))
      ..add(DiagnosticsProperty('resolveNodeTimeoutMs', resolveNodeTimeoutMs))
      ..add(DiagnosticsProperty('resolveNodeCount', resolveNodeCount))
      ..add(DiagnosticsProperty('resolveNodeFanout', resolveNodeFanout))
      ..add(DiagnosticsProperty('maxFindNodeCount', maxFindNodeCount))
      ..add(DiagnosticsProperty('getValueTimeoutMs', getValueTimeoutMs))
      ..add(DiagnosticsProperty('getValueCount', getValueCount))
      ..add(DiagnosticsProperty('getValueFanout', getValueFanout))
      ..add(DiagnosticsProperty('setValueTimeoutMs', setValueTimeoutMs))
      ..add(DiagnosticsProperty('setValueCount', setValueCount))
      ..add(DiagnosticsProperty('setValueFanout', setValueFanout))
      ..add(DiagnosticsProperty('minPeerCount', minPeerCount))
      ..add(DiagnosticsProperty('minPeerRefreshTimeMs', minPeerRefreshTimeMs))
      ..add(DiagnosticsProperty(
          'validateDialInfoReceiptTimeMs', validateDialInfoReceiptTimeMs))
      ..add(DiagnosticsProperty('localSubkeyCacheSize', localSubkeyCacheSize))
      ..add(DiagnosticsProperty(
          'localMaxSubkeyCacheMemoryMb', localMaxSubkeyCacheMemoryMb))
      ..add(DiagnosticsProperty('remoteSubkeyCacheSize', remoteSubkeyCacheSize))
      ..add(DiagnosticsProperty('remoteMaxRecords', remoteMaxRecords))
      ..add(DiagnosticsProperty(
          'remoteMaxSubkeyCacheMemoryMb', remoteMaxSubkeyCacheMemoryMb))
      ..add(DiagnosticsProperty(
          'remoteMaxStorageSpaceMb', remoteMaxStorageSpaceMb));
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$_VeilidConfigDHT &&
            (identical(other.resolveNodeTimeoutMs, resolveNodeTimeoutMs) ||
                other.resolveNodeTimeoutMs == resolveNodeTimeoutMs) &&
            (identical(other.resolveNodeCount, resolveNodeCount) ||
                other.resolveNodeCount == resolveNodeCount) &&
            (identical(other.resolveNodeFanout, resolveNodeFanout) ||
                other.resolveNodeFanout == resolveNodeFanout) &&
            (identical(other.maxFindNodeCount, maxFindNodeCount) ||
                other.maxFindNodeCount == maxFindNodeCount) &&
            (identical(other.getValueTimeoutMs, getValueTimeoutMs) ||
                other.getValueTimeoutMs == getValueTimeoutMs) &&
            (identical(other.getValueCount, getValueCount) ||
                other.getValueCount == getValueCount) &&
            (identical(other.getValueFanout, getValueFanout) ||
                other.getValueFanout == getValueFanout) &&
            (identical(other.setValueTimeoutMs, setValueTimeoutMs) ||
                other.setValueTimeoutMs == setValueTimeoutMs) &&
            (identical(other.setValueCount, setValueCount) ||
                other.setValueCount == setValueCount) &&
            (identical(other.setValueFanout, setValueFanout) ||
                other.setValueFanout == setValueFanout) &&
            (identical(other.minPeerCount, minPeerCount) ||
                other.minPeerCount == minPeerCount) &&
            (identical(other.minPeerRefreshTimeMs, minPeerRefreshTimeMs) ||
                other.minPeerRefreshTimeMs == minPeerRefreshTimeMs) &&
            (identical(other.validateDialInfoReceiptTimeMs,
                    validateDialInfoReceiptTimeMs) ||
                other.validateDialInfoReceiptTimeMs ==
                    validateDialInfoReceiptTimeMs) &&
            (identical(other.localSubkeyCacheSize, localSubkeyCacheSize) ||
                other.localSubkeyCacheSize == localSubkeyCacheSize) &&
            (identical(other.localMaxSubkeyCacheMemoryMb,
                    localMaxSubkeyCacheMemoryMb) ||
                other.localMaxSubkeyCacheMemoryMb ==
                    localMaxSubkeyCacheMemoryMb) &&
            (identical(other.remoteSubkeyCacheSize, remoteSubkeyCacheSize) ||
                other.remoteSubkeyCacheSize == remoteSubkeyCacheSize) &&
            (identical(other.remoteMaxRecords, remoteMaxRecords) ||
                other.remoteMaxRecords == remoteMaxRecords) &&
            (identical(other.remoteMaxSubkeyCacheMemoryMb,
                    remoteMaxSubkeyCacheMemoryMb) ||
                other.remoteMaxSubkeyCacheMemoryMb ==
                    remoteMaxSubkeyCacheMemoryMb) &&
            (identical(
                    other.remoteMaxStorageSpaceMb, remoteMaxStorageSpaceMb) ||
                other.remoteMaxStorageSpaceMb == remoteMaxStorageSpaceMb));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hashAll([
        runtimeType,
        resolveNodeTimeoutMs,
        resolveNodeCount,
        resolveNodeFanout,
        maxFindNodeCount,
        getValueTimeoutMs,
        getValueCount,
        getValueFanout,
        setValueTimeoutMs,
        setValueCount,
        setValueFanout,
        minPeerCount,
        minPeerRefreshTimeMs,
        validateDialInfoReceiptTimeMs,
        localSubkeyCacheSize,
        localMaxSubkeyCacheMemoryMb,
        remoteSubkeyCacheSize,
        remoteMaxRecords,
        remoteMaxSubkeyCacheMemoryMb,
        remoteMaxStorageSpaceMb
      ]);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$_VeilidConfigDHTCopyWith<_$_VeilidConfigDHT> get copyWith =>
      __$$_VeilidConfigDHTCopyWithImpl<_$_VeilidConfigDHT>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$_VeilidConfigDHTToJson(
      this,
    );
  }
}

abstract class _VeilidConfigDHT implements VeilidConfigDHT {
  const factory _VeilidConfigDHT(
      {required final int resolveNodeTimeoutMs,
      required final int resolveNodeCount,
      required final int resolveNodeFanout,
      required final int maxFindNodeCount,
      required final int getValueTimeoutMs,
      required final int getValueCount,
      required final int getValueFanout,
      required final int setValueTimeoutMs,
      required final int setValueCount,
      required final int setValueFanout,
      required final int minPeerCount,
      required final int minPeerRefreshTimeMs,
      required final int validateDialInfoReceiptTimeMs,
      required final int localSubkeyCacheSize,
      required final int localMaxSubkeyCacheMemoryMb,
      required final int remoteSubkeyCacheSize,
      required final int remoteMaxRecords,
      required final int remoteMaxSubkeyCacheMemoryMb,
      required final int remoteMaxStorageSpaceMb}) = _$_VeilidConfigDHT;

  factory _VeilidConfigDHT.fromJson(Map<String, dynamic> json) =
      _$_VeilidConfigDHT.fromJson;

  @override
  int get resolveNodeTimeoutMs;
  @override
  int get resolveNodeCount;
  @override
  int get resolveNodeFanout;
  @override
  int get maxFindNodeCount;
  @override
  int get getValueTimeoutMs;
  @override
  int get getValueCount;
  @override
  int get getValueFanout;
  @override
  int get setValueTimeoutMs;
  @override
  int get setValueCount;
  @override
  int get setValueFanout;
  @override
  int get minPeerCount;
  @override
  int get minPeerRefreshTimeMs;
  @override
  int get validateDialInfoReceiptTimeMs;
  @override
  int get localSubkeyCacheSize;
  @override
  int get localMaxSubkeyCacheMemoryMb;
  @override
  int get remoteSubkeyCacheSize;
  @override
  int get remoteMaxRecords;
  @override
  int get remoteMaxSubkeyCacheMemoryMb;
  @override
  int get remoteMaxStorageSpaceMb;
  @override
  @JsonKey(ignore: true)
  _$$_VeilidConfigDHTCopyWith<_$_VeilidConfigDHT> get copyWith =>
      throw _privateConstructorUsedError;
}

VeilidConfigRPC _$VeilidConfigRPCFromJson(Map<String, dynamic> json) {
  return _VeilidConfigRPC.fromJson(json);
}

/// @nodoc
mixin _$VeilidConfigRPC {
  int get concurrency => throw _privateConstructorUsedError;
  int get queueSize => throw _privateConstructorUsedError;
  int get timeoutMs => throw _privateConstructorUsedError;
  int get maxRouteHopCount => throw _privateConstructorUsedError;
  int get defaultRouteHopCount => throw _privateConstructorUsedError;
  int? get maxTimestampBehindMs => throw _privateConstructorUsedError;
  int? get maxTimestampAheadMs => throw _privateConstructorUsedError;

  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
  @JsonKey(ignore: true)
  $VeilidConfigRPCCopyWith<VeilidConfigRPC> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $VeilidConfigRPCCopyWith<$Res> {
  factory $VeilidConfigRPCCopyWith(
          VeilidConfigRPC value, $Res Function(VeilidConfigRPC) then) =
      _$VeilidConfigRPCCopyWithImpl<$Res, VeilidConfigRPC>;
  @useResult
  $Res call(
      {int concurrency,
      int queueSize,
      int timeoutMs,
      int maxRouteHopCount,
      int defaultRouteHopCount,
      int? maxTimestampBehindMs,
      int? maxTimestampAheadMs});
}

/// @nodoc
class _$VeilidConfigRPCCopyWithImpl<$Res, $Val extends VeilidConfigRPC>
    implements $VeilidConfigRPCCopyWith<$Res> {
  _$VeilidConfigRPCCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? concurrency = null,
    Object? queueSize = null,
    Object? timeoutMs = null,
    Object? maxRouteHopCount = null,
    Object? defaultRouteHopCount = null,
    Object? maxTimestampBehindMs = freezed,
    Object? maxTimestampAheadMs = freezed,
  }) {
    return _then(_value.copyWith(
      concurrency: null == concurrency
          ? _value.concurrency
          : concurrency // ignore: cast_nullable_to_non_nullable
              as int,
      queueSize: null == queueSize
          ? _value.queueSize
          : queueSize // ignore: cast_nullable_to_non_nullable
              as int,
      timeoutMs: null == timeoutMs
          ? _value.timeoutMs
          : timeoutMs // ignore: cast_nullable_to_non_nullable
              as int,
      maxRouteHopCount: null == maxRouteHopCount
          ? _value.maxRouteHopCount
          : maxRouteHopCount // ignore: cast_nullable_to_non_nullable
              as int,
      defaultRouteHopCount: null == defaultRouteHopCount
          ? _value.defaultRouteHopCount
          : defaultRouteHopCount // ignore: cast_nullable_to_non_nullable
              as int,
      maxTimestampBehindMs: freezed == maxTimestampBehindMs
          ? _value.maxTimestampBehindMs
          : maxTimestampBehindMs // ignore: cast_nullable_to_non_nullable
              as int?,
      maxTimestampAheadMs: freezed == maxTimestampAheadMs
          ? _value.maxTimestampAheadMs
          : maxTimestampAheadMs // ignore: cast_nullable_to_non_nullable
              as int?,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$_VeilidConfigRPCCopyWith<$Res>
    implements $VeilidConfigRPCCopyWith<$Res> {
  factory _$$_VeilidConfigRPCCopyWith(
          _$_VeilidConfigRPC value, $Res Function(_$_VeilidConfigRPC) then) =
      __$$_VeilidConfigRPCCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {int concurrency,
      int queueSize,
      int timeoutMs,
      int maxRouteHopCount,
      int defaultRouteHopCount,
      int? maxTimestampBehindMs,
      int? maxTimestampAheadMs});
}

/// @nodoc
class __$$_VeilidConfigRPCCopyWithImpl<$Res>
    extends _$VeilidConfigRPCCopyWithImpl<$Res, _$_VeilidConfigRPC>
    implements _$$_VeilidConfigRPCCopyWith<$Res> {
  __$$_VeilidConfigRPCCopyWithImpl(
      _$_VeilidConfigRPC _value, $Res Function(_$_VeilidConfigRPC) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? concurrency = null,
    Object? queueSize = null,
    Object? timeoutMs = null,
    Object? maxRouteHopCount = null,
    Object? defaultRouteHopCount = null,
    Object? maxTimestampBehindMs = freezed,
    Object? maxTimestampAheadMs = freezed,
  }) {
    return _then(_$_VeilidConfigRPC(
      concurrency: null == concurrency
          ? _value.concurrency
          : concurrency // ignore: cast_nullable_to_non_nullable
              as int,
      queueSize: null == queueSize
          ? _value.queueSize
          : queueSize // ignore: cast_nullable_to_non_nullable
              as int,
      timeoutMs: null == timeoutMs
          ? _value.timeoutMs
          : timeoutMs // ignore: cast_nullable_to_non_nullable
              as int,
      maxRouteHopCount: null == maxRouteHopCount
          ? _value.maxRouteHopCount
          : maxRouteHopCount // ignore: cast_nullable_to_non_nullable
              as int,
      defaultRouteHopCount: null == defaultRouteHopCount
          ? _value.defaultRouteHopCount
          : defaultRouteHopCount // ignore: cast_nullable_to_non_nullable
              as int,
      maxTimestampBehindMs: freezed == maxTimestampBehindMs
          ? _value.maxTimestampBehindMs
          : maxTimestampBehindMs // ignore: cast_nullable_to_non_nullable
              as int?,
      maxTimestampAheadMs: freezed == maxTimestampAheadMs
          ? _value.maxTimestampAheadMs
          : maxTimestampAheadMs // ignore: cast_nullable_to_non_nullable
              as int?,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$_VeilidConfigRPC
    with DiagnosticableTreeMixin
    implements _VeilidConfigRPC {
  const _$_VeilidConfigRPC(
      {required this.concurrency,
      required this.queueSize,
      required this.timeoutMs,
      required this.maxRouteHopCount,
      required this.defaultRouteHopCount,
      this.maxTimestampBehindMs,
      this.maxTimestampAheadMs});

  factory _$_VeilidConfigRPC.fromJson(Map<String, dynamic> json) =>
      _$$_VeilidConfigRPCFromJson(json);

  @override
  final int concurrency;
  @override
  final int queueSize;
  @override
  final int timeoutMs;
  @override
  final int maxRouteHopCount;
  @override
  final int defaultRouteHopCount;
  @override
  final int? maxTimestampBehindMs;
  @override
  final int? maxTimestampAheadMs;

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidConfigRPC(concurrency: $concurrency, queueSize: $queueSize, timeoutMs: $timeoutMs, maxRouteHopCount: $maxRouteHopCount, defaultRouteHopCount: $defaultRouteHopCount, maxTimestampBehindMs: $maxTimestampBehindMs, maxTimestampAheadMs: $maxTimestampAheadMs)';
  }

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
    super.debugFillProperties(properties);
    properties
      ..add(DiagnosticsProperty('type', 'VeilidConfigRPC'))
      ..add(DiagnosticsProperty('concurrency', concurrency))
      ..add(DiagnosticsProperty('queueSize', queueSize))
      ..add(DiagnosticsProperty('timeoutMs', timeoutMs))
      ..add(DiagnosticsProperty('maxRouteHopCount', maxRouteHopCount))
      ..add(DiagnosticsProperty('defaultRouteHopCount', defaultRouteHopCount))
      ..add(DiagnosticsProperty('maxTimestampBehindMs', maxTimestampBehindMs))
      ..add(DiagnosticsProperty('maxTimestampAheadMs', maxTimestampAheadMs));
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$_VeilidConfigRPC &&
            (identical(other.concurrency, concurrency) ||
                other.concurrency == concurrency) &&
            (identical(other.queueSize, queueSize) ||
                other.queueSize == queueSize) &&
            (identical(other.timeoutMs, timeoutMs) ||
                other.timeoutMs == timeoutMs) &&
            (identical(other.maxRouteHopCount, maxRouteHopCount) ||
                other.maxRouteHopCount == maxRouteHopCount) &&
            (identical(other.defaultRouteHopCount, defaultRouteHopCount) ||
                other.defaultRouteHopCount == defaultRouteHopCount) &&
            (identical(other.maxTimestampBehindMs, maxTimestampBehindMs) ||
                other.maxTimestampBehindMs == maxTimestampBehindMs) &&
            (identical(other.maxTimestampAheadMs, maxTimestampAheadMs) ||
                other.maxTimestampAheadMs == maxTimestampAheadMs));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(
      runtimeType,
      concurrency,
      queueSize,
      timeoutMs,
      maxRouteHopCount,
      defaultRouteHopCount,
      maxTimestampBehindMs,
      maxTimestampAheadMs);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$_VeilidConfigRPCCopyWith<_$_VeilidConfigRPC> get copyWith =>
      __$$_VeilidConfigRPCCopyWithImpl<_$_VeilidConfigRPC>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$_VeilidConfigRPCToJson(
      this,
    );
  }
}

abstract class _VeilidConfigRPC implements VeilidConfigRPC {
  const factory _VeilidConfigRPC(
      {required final int concurrency,
      required final int queueSize,
      required final int timeoutMs,
      required final int maxRouteHopCount,
      required final int defaultRouteHopCount,
      final int? maxTimestampBehindMs,
      final int? maxTimestampAheadMs}) = _$_VeilidConfigRPC;

  factory _VeilidConfigRPC.fromJson(Map<String, dynamic> json) =
      _$_VeilidConfigRPC.fromJson;

  @override
  int get concurrency;
  @override
  int get queueSize;
  @override
  int get timeoutMs;
  @override
  int get maxRouteHopCount;
  @override
  int get defaultRouteHopCount;
  @override
  int? get maxTimestampBehindMs;
  @override
  int? get maxTimestampAheadMs;
  @override
  @JsonKey(ignore: true)
  _$$_VeilidConfigRPCCopyWith<_$_VeilidConfigRPC> get copyWith =>
      throw _privateConstructorUsedError;
}

VeilidConfigRoutingTable _$VeilidConfigRoutingTableFromJson(
    Map<String, dynamic> json) {
  return _VeilidConfigRoutingTable.fromJson(json);
}

/// @nodoc
mixin _$VeilidConfigRoutingTable {
  List<Typed<FixedEncodedString43>> get nodeId =>
      throw _privateConstructorUsedError;
  List<Typed<FixedEncodedString43>> get nodeIdSecret =>
      throw _privateConstructorUsedError;
  List<String> get bootstrap => throw _privateConstructorUsedError;
  int get limitOverAttached => throw _privateConstructorUsedError;
  int get limitFullyAttached => throw _privateConstructorUsedError;
  int get limitAttachedStrong => throw _privateConstructorUsedError;
  int get limitAttachedGood => throw _privateConstructorUsedError;
  int get limitAttachedWeak => throw _privateConstructorUsedError;

  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
  @JsonKey(ignore: true)
  $VeilidConfigRoutingTableCopyWith<VeilidConfigRoutingTable> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $VeilidConfigRoutingTableCopyWith<$Res> {
  factory $VeilidConfigRoutingTableCopyWith(VeilidConfigRoutingTable value,
          $Res Function(VeilidConfigRoutingTable) then) =
      _$VeilidConfigRoutingTableCopyWithImpl<$Res, VeilidConfigRoutingTable>;
  @useResult
  $Res call(
      {List<Typed<FixedEncodedString43>> nodeId,
      List<Typed<FixedEncodedString43>> nodeIdSecret,
      List<String> bootstrap,
      int limitOverAttached,
      int limitFullyAttached,
      int limitAttachedStrong,
      int limitAttachedGood,
      int limitAttachedWeak});
}

/// @nodoc
class _$VeilidConfigRoutingTableCopyWithImpl<$Res,
        $Val extends VeilidConfigRoutingTable>
    implements $VeilidConfigRoutingTableCopyWith<$Res> {
  _$VeilidConfigRoutingTableCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? nodeId = null,
    Object? nodeIdSecret = null,
    Object? bootstrap = null,
    Object? limitOverAttached = null,
    Object? limitFullyAttached = null,
    Object? limitAttachedStrong = null,
    Object? limitAttachedGood = null,
    Object? limitAttachedWeak = null,
  }) {
    return _then(_value.copyWith(
      nodeId: null == nodeId
          ? _value.nodeId
          : nodeId // ignore: cast_nullable_to_non_nullable
              as List<Typed<FixedEncodedString43>>,
      nodeIdSecret: null == nodeIdSecret
          ? _value.nodeIdSecret
          : nodeIdSecret // ignore: cast_nullable_to_non_nullable
              as List<Typed<FixedEncodedString43>>,
      bootstrap: null == bootstrap
          ? _value.bootstrap
          : bootstrap // ignore: cast_nullable_to_non_nullable
              as List<String>,
      limitOverAttached: null == limitOverAttached
          ? _value.limitOverAttached
          : limitOverAttached // ignore: cast_nullable_to_non_nullable
              as int,
      limitFullyAttached: null == limitFullyAttached
          ? _value.limitFullyAttached
          : limitFullyAttached // ignore: cast_nullable_to_non_nullable
              as int,
      limitAttachedStrong: null == limitAttachedStrong
          ? _value.limitAttachedStrong
          : limitAttachedStrong // ignore: cast_nullable_to_non_nullable
              as int,
      limitAttachedGood: null == limitAttachedGood
          ? _value.limitAttachedGood
          : limitAttachedGood // ignore: cast_nullable_to_non_nullable
              as int,
      limitAttachedWeak: null == limitAttachedWeak
          ? _value.limitAttachedWeak
          : limitAttachedWeak // ignore: cast_nullable_to_non_nullable
              as int,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$_VeilidConfigRoutingTableCopyWith<$Res>
    implements $VeilidConfigRoutingTableCopyWith<$Res> {
  factory _$$_VeilidConfigRoutingTableCopyWith(
          _$_VeilidConfigRoutingTable value,
          $Res Function(_$_VeilidConfigRoutingTable) then) =
      __$$_VeilidConfigRoutingTableCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {List<Typed<FixedEncodedString43>> nodeId,
      List<Typed<FixedEncodedString43>> nodeIdSecret,
      List<String> bootstrap,
      int limitOverAttached,
      int limitFullyAttached,
      int limitAttachedStrong,
      int limitAttachedGood,
      int limitAttachedWeak});
}

/// @nodoc
class __$$_VeilidConfigRoutingTableCopyWithImpl<$Res>
    extends _$VeilidConfigRoutingTableCopyWithImpl<$Res,
        _$_VeilidConfigRoutingTable>
    implements _$$_VeilidConfigRoutingTableCopyWith<$Res> {
  __$$_VeilidConfigRoutingTableCopyWithImpl(_$_VeilidConfigRoutingTable _value,
      $Res Function(_$_VeilidConfigRoutingTable) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? nodeId = null,
    Object? nodeIdSecret = null,
    Object? bootstrap = null,
    Object? limitOverAttached = null,
    Object? limitFullyAttached = null,
    Object? limitAttachedStrong = null,
    Object? limitAttachedGood = null,
    Object? limitAttachedWeak = null,
  }) {
    return _then(_$_VeilidConfigRoutingTable(
      nodeId: null == nodeId
          ? _value._nodeId
          : nodeId // ignore: cast_nullable_to_non_nullable
              as List<Typed<FixedEncodedString43>>,
      nodeIdSecret: null == nodeIdSecret
          ? _value._nodeIdSecret
          : nodeIdSecret // ignore: cast_nullable_to_non_nullable
              as List<Typed<FixedEncodedString43>>,
      bootstrap: null == bootstrap
          ? _value._bootstrap
          : bootstrap // ignore: cast_nullable_to_non_nullable
              as List<String>,
      limitOverAttached: null == limitOverAttached
          ? _value.limitOverAttached
          : limitOverAttached // ignore: cast_nullable_to_non_nullable
              as int,
      limitFullyAttached: null == limitFullyAttached
          ? _value.limitFullyAttached
          : limitFullyAttached // ignore: cast_nullable_to_non_nullable
              as int,
      limitAttachedStrong: null == limitAttachedStrong
          ? _value.limitAttachedStrong
          : limitAttachedStrong // ignore: cast_nullable_to_non_nullable
              as int,
      limitAttachedGood: null == limitAttachedGood
          ? _value.limitAttachedGood
          : limitAttachedGood // ignore: cast_nullable_to_non_nullable
              as int,
      limitAttachedWeak: null == limitAttachedWeak
          ? _value.limitAttachedWeak
          : limitAttachedWeak // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$_VeilidConfigRoutingTable
    with DiagnosticableTreeMixin
    implements _VeilidConfigRoutingTable {
  const _$_VeilidConfigRoutingTable(
      {required final List<Typed<FixedEncodedString43>> nodeId,
      required final List<Typed<FixedEncodedString43>> nodeIdSecret,
      required final List<String> bootstrap,
      required this.limitOverAttached,
      required this.limitFullyAttached,
      required this.limitAttachedStrong,
      required this.limitAttachedGood,
      required this.limitAttachedWeak})
      : _nodeId = nodeId,
        _nodeIdSecret = nodeIdSecret,
        _bootstrap = bootstrap;

  factory _$_VeilidConfigRoutingTable.fromJson(Map<String, dynamic> json) =>
      _$$_VeilidConfigRoutingTableFromJson(json);

  final List<Typed<FixedEncodedString43>> _nodeId;
  @override
  List<Typed<FixedEncodedString43>> get nodeId {
    if (_nodeId is EqualUnmodifiableListView) return _nodeId;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_nodeId);
  }

  final List<Typed<FixedEncodedString43>> _nodeIdSecret;
  @override
  List<Typed<FixedEncodedString43>> get nodeIdSecret {
    if (_nodeIdSecret is EqualUnmodifiableListView) return _nodeIdSecret;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_nodeIdSecret);
  }

  final List<String> _bootstrap;
  @override
  List<String> get bootstrap {
    if (_bootstrap is EqualUnmodifiableListView) return _bootstrap;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_bootstrap);
  }

  @override
  final int limitOverAttached;
  @override
  final int limitFullyAttached;
  @override
  final int limitAttachedStrong;
  @override
  final int limitAttachedGood;
  @override
  final int limitAttachedWeak;

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidConfigRoutingTable(nodeId: $nodeId, nodeIdSecret: $nodeIdSecret, bootstrap: $bootstrap, limitOverAttached: $limitOverAttached, limitFullyAttached: $limitFullyAttached, limitAttachedStrong: $limitAttachedStrong, limitAttachedGood: $limitAttachedGood, limitAttachedWeak: $limitAttachedWeak)';
  }

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
    super.debugFillProperties(properties);
    properties
      ..add(DiagnosticsProperty('type', 'VeilidConfigRoutingTable'))
      ..add(DiagnosticsProperty('nodeId', nodeId))
      ..add(DiagnosticsProperty('nodeIdSecret', nodeIdSecret))
      ..add(DiagnosticsProperty('bootstrap', bootstrap))
      ..add(DiagnosticsProperty('limitOverAttached', limitOverAttached))
      ..add(DiagnosticsProperty('limitFullyAttached', limitFullyAttached))
      ..add(DiagnosticsProperty('limitAttachedStrong', limitAttachedStrong))
      ..add(DiagnosticsProperty('limitAttachedGood', limitAttachedGood))
      ..add(DiagnosticsProperty('limitAttachedWeak', limitAttachedWeak));
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$_VeilidConfigRoutingTable &&
            const DeepCollectionEquality().equals(other._nodeId, _nodeId) &&
            const DeepCollectionEquality()
                .equals(other._nodeIdSecret, _nodeIdSecret) &&
            const DeepCollectionEquality()
                .equals(other._bootstrap, _bootstrap) &&
            (identical(other.limitOverAttached, limitOverAttached) ||
                other.limitOverAttached == limitOverAttached) &&
            (identical(other.limitFullyAttached, limitFullyAttached) ||
                other.limitFullyAttached == limitFullyAttached) &&
            (identical(other.limitAttachedStrong, limitAttachedStrong) ||
                other.limitAttachedStrong == limitAttachedStrong) &&
            (identical(other.limitAttachedGood, limitAttachedGood) ||
                other.limitAttachedGood == limitAttachedGood) &&
            (identical(other.limitAttachedWeak, limitAttachedWeak) ||
                other.limitAttachedWeak == limitAttachedWeak));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(
      runtimeType,
      const DeepCollectionEquality().hash(_nodeId),
      const DeepCollectionEquality().hash(_nodeIdSecret),
      const DeepCollectionEquality().hash(_bootstrap),
      limitOverAttached,
      limitFullyAttached,
      limitAttachedStrong,
      limitAttachedGood,
      limitAttachedWeak);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$_VeilidConfigRoutingTableCopyWith<_$_VeilidConfigRoutingTable>
      get copyWith => __$$_VeilidConfigRoutingTableCopyWithImpl<
          _$_VeilidConfigRoutingTable>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$_VeilidConfigRoutingTableToJson(
      this,
    );
  }
}

abstract class _VeilidConfigRoutingTable implements VeilidConfigRoutingTable {
  const factory _VeilidConfigRoutingTable(
      {required final List<Typed<FixedEncodedString43>> nodeId,
      required final List<Typed<FixedEncodedString43>> nodeIdSecret,
      required final List<String> bootstrap,
      required final int limitOverAttached,
      required final int limitFullyAttached,
      required final int limitAttachedStrong,
      required final int limitAttachedGood,
      required final int limitAttachedWeak}) = _$_VeilidConfigRoutingTable;

  factory _VeilidConfigRoutingTable.fromJson(Map<String, dynamic> json) =
      _$_VeilidConfigRoutingTable.fromJson;

  @override
  List<Typed<FixedEncodedString43>> get nodeId;
  @override
  List<Typed<FixedEncodedString43>> get nodeIdSecret;
  @override
  List<String> get bootstrap;
  @override
  int get limitOverAttached;
  @override
  int get limitFullyAttached;
  @override
  int get limitAttachedStrong;
  @override
  int get limitAttachedGood;
  @override
  int get limitAttachedWeak;
  @override
  @JsonKey(ignore: true)
  _$$_VeilidConfigRoutingTableCopyWith<_$_VeilidConfigRoutingTable>
      get copyWith => throw _privateConstructorUsedError;
}

VeilidConfigNetwork _$VeilidConfigNetworkFromJson(Map<String, dynamic> json) {
  return _VeilidConfigNetwork.fromJson(json);
}

/// @nodoc
mixin _$VeilidConfigNetwork {
  int get connectionInitialTimeoutMs => throw _privateConstructorUsedError;
  int get connectionInactivityTimeoutMs => throw _privateConstructorUsedError;
  int get maxConnectionsPerIp4 => throw _privateConstructorUsedError;
  int get maxConnectionsPerIp6Prefix => throw _privateConstructorUsedError;
  int get maxConnectionsPerIp6PrefixSize => throw _privateConstructorUsedError;
  int get maxConnectionFrequencyPerMin => throw _privateConstructorUsedError;
  int get clientWhitelistTimeoutMs => throw _privateConstructorUsedError;
  int get reverseConnectionReceiptTimeMs => throw _privateConstructorUsedError;
  int get holePunchReceiptTimeMs => throw _privateConstructorUsedError;
  VeilidConfigRoutingTable get routingTable =>
      throw _privateConstructorUsedError;
  VeilidConfigRPC get rpc => throw _privateConstructorUsedError;
  VeilidConfigDHT get dht => throw _privateConstructorUsedError;
  bool get upnp => throw _privateConstructorUsedError;
  bool get detectAddressChanges => throw _privateConstructorUsedError;
  int get restrictedNatRetries => throw _privateConstructorUsedError;
  VeilidConfigTLS get tls => throw _privateConstructorUsedError;
  VeilidConfigApplication get application => throw _privateConstructorUsedError;
  VeilidConfigProtocol get protocol => throw _privateConstructorUsedError;
  String? get networkKeyPassword => throw _privateConstructorUsedError;

  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
  @JsonKey(ignore: true)
  $VeilidConfigNetworkCopyWith<VeilidConfigNetwork> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $VeilidConfigNetworkCopyWith<$Res> {
  factory $VeilidConfigNetworkCopyWith(
          VeilidConfigNetwork value, $Res Function(VeilidConfigNetwork) then) =
      _$VeilidConfigNetworkCopyWithImpl<$Res, VeilidConfigNetwork>;
  @useResult
  $Res call(
      {int connectionInitialTimeoutMs,
      int connectionInactivityTimeoutMs,
      int maxConnectionsPerIp4,
      int maxConnectionsPerIp6Prefix,
      int maxConnectionsPerIp6PrefixSize,
      int maxConnectionFrequencyPerMin,
      int clientWhitelistTimeoutMs,
      int reverseConnectionReceiptTimeMs,
      int holePunchReceiptTimeMs,
      VeilidConfigRoutingTable routingTable,
      VeilidConfigRPC rpc,
      VeilidConfigDHT dht,
      bool upnp,
      bool detectAddressChanges,
      int restrictedNatRetries,
      VeilidConfigTLS tls,
      VeilidConfigApplication application,
      VeilidConfigProtocol protocol,
      String? networkKeyPassword});

  $VeilidConfigRoutingTableCopyWith<$Res> get routingTable;
  $VeilidConfigRPCCopyWith<$Res> get rpc;
  $VeilidConfigDHTCopyWith<$Res> get dht;
  $VeilidConfigTLSCopyWith<$Res> get tls;
  $VeilidConfigApplicationCopyWith<$Res> get application;
  $VeilidConfigProtocolCopyWith<$Res> get protocol;
}

/// @nodoc
class _$VeilidConfigNetworkCopyWithImpl<$Res, $Val extends VeilidConfigNetwork>
    implements $VeilidConfigNetworkCopyWith<$Res> {
  _$VeilidConfigNetworkCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? connectionInitialTimeoutMs = null,
    Object? connectionInactivityTimeoutMs = null,
    Object? maxConnectionsPerIp4 = null,
    Object? maxConnectionsPerIp6Prefix = null,
    Object? maxConnectionsPerIp6PrefixSize = null,
    Object? maxConnectionFrequencyPerMin = null,
    Object? clientWhitelistTimeoutMs = null,
    Object? reverseConnectionReceiptTimeMs = null,
    Object? holePunchReceiptTimeMs = null,
    Object? routingTable = null,
    Object? rpc = null,
    Object? dht = null,
    Object? upnp = null,
    Object? detectAddressChanges = null,
    Object? restrictedNatRetries = null,
    Object? tls = null,
    Object? application = null,
    Object? protocol = null,
    Object? networkKeyPassword = freezed,
  }) {
    return _then(_value.copyWith(
      connectionInitialTimeoutMs: null == connectionInitialTimeoutMs
          ? _value.connectionInitialTimeoutMs
          : connectionInitialTimeoutMs // ignore: cast_nullable_to_non_nullable
              as int,
      connectionInactivityTimeoutMs: null == connectionInactivityTimeoutMs
          ? _value.connectionInactivityTimeoutMs
          : connectionInactivityTimeoutMs // ignore: cast_nullable_to_non_nullable
              as int,
      maxConnectionsPerIp4: null == maxConnectionsPerIp4
          ? _value.maxConnectionsPerIp4
          : maxConnectionsPerIp4 // ignore: cast_nullable_to_non_nullable
              as int,
      maxConnectionsPerIp6Prefix: null == maxConnectionsPerIp6Prefix
          ? _value.maxConnectionsPerIp6Prefix
          : maxConnectionsPerIp6Prefix // ignore: cast_nullable_to_non_nullable
              as int,
      maxConnectionsPerIp6PrefixSize: null == maxConnectionsPerIp6PrefixSize
          ? _value.maxConnectionsPerIp6PrefixSize
          : maxConnectionsPerIp6PrefixSize // ignore: cast_nullable_to_non_nullable
              as int,
      maxConnectionFrequencyPerMin: null == maxConnectionFrequencyPerMin
          ? _value.maxConnectionFrequencyPerMin
          : maxConnectionFrequencyPerMin // ignore: cast_nullable_to_non_nullable
              as int,
      clientWhitelistTimeoutMs: null == clientWhitelistTimeoutMs
          ? _value.clientWhitelistTimeoutMs
          : clientWhitelistTimeoutMs // ignore: cast_nullable_to_non_nullable
              as int,
      reverseConnectionReceiptTimeMs: null == reverseConnectionReceiptTimeMs
          ? _value.reverseConnectionReceiptTimeMs
          : reverseConnectionReceiptTimeMs // ignore: cast_nullable_to_non_nullable
              as int,
      holePunchReceiptTimeMs: null == holePunchReceiptTimeMs
          ? _value.holePunchReceiptTimeMs
          : holePunchReceiptTimeMs // ignore: cast_nullable_to_non_nullable
              as int,
      routingTable: null == routingTable
          ? _value.routingTable
          : routingTable // ignore: cast_nullable_to_non_nullable
              as VeilidConfigRoutingTable,
      rpc: null == rpc
          ? _value.rpc
          : rpc // ignore: cast_nullable_to_non_nullable
              as VeilidConfigRPC,
      dht: null == dht
          ? _value.dht
          : dht // ignore: cast_nullable_to_non_nullable
              as VeilidConfigDHT,
      upnp: null == upnp
          ? _value.upnp
          : upnp // ignore: cast_nullable_to_non_nullable
              as bool,
      detectAddressChanges: null == detectAddressChanges
          ? _value.detectAddressChanges
          : detectAddressChanges // ignore: cast_nullable_to_non_nullable
              as bool,
      restrictedNatRetries: null == restrictedNatRetries
          ? _value.restrictedNatRetries
          : restrictedNatRetries // ignore: cast_nullable_to_non_nullable
              as int,
      tls: null == tls
          ? _value.tls
          : tls // ignore: cast_nullable_to_non_nullable
              as VeilidConfigTLS,
      application: null == application
          ? _value.application
          : application // ignore: cast_nullable_to_non_nullable
              as VeilidConfigApplication,
      protocol: null == protocol
          ? _value.protocol
          : protocol // ignore: cast_nullable_to_non_nullable
              as VeilidConfigProtocol,
      networkKeyPassword: freezed == networkKeyPassword
          ? _value.networkKeyPassword
          : networkKeyPassword // ignore: cast_nullable_to_non_nullable
              as String?,
    ) as $Val);
  }

  @override
  @pragma('vm:prefer-inline')
  $VeilidConfigRoutingTableCopyWith<$Res> get routingTable {
    return $VeilidConfigRoutingTableCopyWith<$Res>(_value.routingTable,
        (value) {
      return _then(_value.copyWith(routingTable: value) as $Val);
    });
  }

  @override
  @pragma('vm:prefer-inline')
  $VeilidConfigRPCCopyWith<$Res> get rpc {
    return $VeilidConfigRPCCopyWith<$Res>(_value.rpc, (value) {
      return _then(_value.copyWith(rpc: value) as $Val);
    });
  }

  @override
  @pragma('vm:prefer-inline')
  $VeilidConfigDHTCopyWith<$Res> get dht {
    return $VeilidConfigDHTCopyWith<$Res>(_value.dht, (value) {
      return _then(_value.copyWith(dht: value) as $Val);
    });
  }

  @override
  @pragma('vm:prefer-inline')
  $VeilidConfigTLSCopyWith<$Res> get tls {
    return $VeilidConfigTLSCopyWith<$Res>(_value.tls, (value) {
      return _then(_value.copyWith(tls: value) as $Val);
    });
  }

  @override
  @pragma('vm:prefer-inline')
  $VeilidConfigApplicationCopyWith<$Res> get application {
    return $VeilidConfigApplicationCopyWith<$Res>(_value.application, (value) {
      return _then(_value.copyWith(application: value) as $Val);
    });
  }

  @override
  @pragma('vm:prefer-inline')
  $VeilidConfigProtocolCopyWith<$Res> get protocol {
    return $VeilidConfigProtocolCopyWith<$Res>(_value.protocol, (value) {
      return _then(_value.copyWith(protocol: value) as $Val);
    });
  }
}

/// @nodoc
abstract class _$$_VeilidConfigNetworkCopyWith<$Res>
    implements $VeilidConfigNetworkCopyWith<$Res> {
  factory _$$_VeilidConfigNetworkCopyWith(_$_VeilidConfigNetwork value,
          $Res Function(_$_VeilidConfigNetwork) then) =
      __$$_VeilidConfigNetworkCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {int connectionInitialTimeoutMs,
      int connectionInactivityTimeoutMs,
      int maxConnectionsPerIp4,
      int maxConnectionsPerIp6Prefix,
      int maxConnectionsPerIp6PrefixSize,
      int maxConnectionFrequencyPerMin,
      int clientWhitelistTimeoutMs,
      int reverseConnectionReceiptTimeMs,
      int holePunchReceiptTimeMs,
      VeilidConfigRoutingTable routingTable,
      VeilidConfigRPC rpc,
      VeilidConfigDHT dht,
      bool upnp,
      bool detectAddressChanges,
      int restrictedNatRetries,
      VeilidConfigTLS tls,
      VeilidConfigApplication application,
      VeilidConfigProtocol protocol,
      String? networkKeyPassword});

  @override
  $VeilidConfigRoutingTableCopyWith<$Res> get routingTable;
  @override
  $VeilidConfigRPCCopyWith<$Res> get rpc;
  @override
  $VeilidConfigDHTCopyWith<$Res> get dht;
  @override
  $VeilidConfigTLSCopyWith<$Res> get tls;
  @override
  $VeilidConfigApplicationCopyWith<$Res> get application;
  @override
  $VeilidConfigProtocolCopyWith<$Res> get protocol;
}

/// @nodoc
class __$$_VeilidConfigNetworkCopyWithImpl<$Res>
    extends _$VeilidConfigNetworkCopyWithImpl<$Res, _$_VeilidConfigNetwork>
    implements _$$_VeilidConfigNetworkCopyWith<$Res> {
  __$$_VeilidConfigNetworkCopyWithImpl(_$_VeilidConfigNetwork _value,
      $Res Function(_$_VeilidConfigNetwork) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? connectionInitialTimeoutMs = null,
    Object? connectionInactivityTimeoutMs = null,
    Object? maxConnectionsPerIp4 = null,
    Object? maxConnectionsPerIp6Prefix = null,
    Object? maxConnectionsPerIp6PrefixSize = null,
    Object? maxConnectionFrequencyPerMin = null,
    Object? clientWhitelistTimeoutMs = null,
    Object? reverseConnectionReceiptTimeMs = null,
    Object? holePunchReceiptTimeMs = null,
    Object? routingTable = null,
    Object? rpc = null,
    Object? dht = null,
    Object? upnp = null,
    Object? detectAddressChanges = null,
    Object? restrictedNatRetries = null,
    Object? tls = null,
    Object? application = null,
    Object? protocol = null,
    Object? networkKeyPassword = freezed,
  }) {
    return _then(_$_VeilidConfigNetwork(
      connectionInitialTimeoutMs: null == connectionInitialTimeoutMs
          ? _value.connectionInitialTimeoutMs
          : connectionInitialTimeoutMs // ignore: cast_nullable_to_non_nullable
              as int,
      connectionInactivityTimeoutMs: null == connectionInactivityTimeoutMs
          ? _value.connectionInactivityTimeoutMs
          : connectionInactivityTimeoutMs // ignore: cast_nullable_to_non_nullable
              as int,
      maxConnectionsPerIp4: null == maxConnectionsPerIp4
          ? _value.maxConnectionsPerIp4
          : maxConnectionsPerIp4 // ignore: cast_nullable_to_non_nullable
              as int,
      maxConnectionsPerIp6Prefix: null == maxConnectionsPerIp6Prefix
          ? _value.maxConnectionsPerIp6Prefix
          : maxConnectionsPerIp6Prefix // ignore: cast_nullable_to_non_nullable
              as int,
      maxConnectionsPerIp6PrefixSize: null == maxConnectionsPerIp6PrefixSize
          ? _value.maxConnectionsPerIp6PrefixSize
          : maxConnectionsPerIp6PrefixSize // ignore: cast_nullable_to_non_nullable
              as int,
      maxConnectionFrequencyPerMin: null == maxConnectionFrequencyPerMin
          ? _value.maxConnectionFrequencyPerMin
          : maxConnectionFrequencyPerMin // ignore: cast_nullable_to_non_nullable
              as int,
      clientWhitelistTimeoutMs: null == clientWhitelistTimeoutMs
          ? _value.clientWhitelistTimeoutMs
          : clientWhitelistTimeoutMs // ignore: cast_nullable_to_non_nullable
              as int,
      reverseConnectionReceiptTimeMs: null == reverseConnectionReceiptTimeMs
          ? _value.reverseConnectionReceiptTimeMs
          : reverseConnectionReceiptTimeMs // ignore: cast_nullable_to_non_nullable
              as int,
      holePunchReceiptTimeMs: null == holePunchReceiptTimeMs
          ? _value.holePunchReceiptTimeMs
          : holePunchReceiptTimeMs // ignore: cast_nullable_to_non_nullable
              as int,
      routingTable: null == routingTable
          ? _value.routingTable
          : routingTable // ignore: cast_nullable_to_non_nullable
              as VeilidConfigRoutingTable,
      rpc: null == rpc
          ? _value.rpc
          : rpc // ignore: cast_nullable_to_non_nullable
              as VeilidConfigRPC,
      dht: null == dht
          ? _value.dht
          : dht // ignore: cast_nullable_to_non_nullable
              as VeilidConfigDHT,
      upnp: null == upnp
          ? _value.upnp
          : upnp // ignore: cast_nullable_to_non_nullable
              as bool,
      detectAddressChanges: null == detectAddressChanges
          ? _value.detectAddressChanges
          : detectAddressChanges // ignore: cast_nullable_to_non_nullable
              as bool,
      restrictedNatRetries: null == restrictedNatRetries
          ? _value.restrictedNatRetries
          : restrictedNatRetries // ignore: cast_nullable_to_non_nullable
              as int,
      tls: null == tls
          ? _value.tls
          : tls // ignore: cast_nullable_to_non_nullable
              as VeilidConfigTLS,
      application: null == application
          ? _value.application
          : application // ignore: cast_nullable_to_non_nullable
              as VeilidConfigApplication,
      protocol: null == protocol
          ? _value.protocol
          : protocol // ignore: cast_nullable_to_non_nullable
              as VeilidConfigProtocol,
      networkKeyPassword: freezed == networkKeyPassword
          ? _value.networkKeyPassword
          : networkKeyPassword // ignore: cast_nullable_to_non_nullable
              as String?,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$_VeilidConfigNetwork
    with DiagnosticableTreeMixin
    implements _VeilidConfigNetwork {
  const _$_VeilidConfigNetwork(
      {required this.connectionInitialTimeoutMs,
      required this.connectionInactivityTimeoutMs,
      required this.maxConnectionsPerIp4,
      required this.maxConnectionsPerIp6Prefix,
      required this.maxConnectionsPerIp6PrefixSize,
      required this.maxConnectionFrequencyPerMin,
      required this.clientWhitelistTimeoutMs,
      required this.reverseConnectionReceiptTimeMs,
      required this.holePunchReceiptTimeMs,
      required this.routingTable,
      required this.rpc,
      required this.dht,
      required this.upnp,
      required this.detectAddressChanges,
      required this.restrictedNatRetries,
      required this.tls,
      required this.application,
      required this.protocol,
      this.networkKeyPassword});

  factory _$_VeilidConfigNetwork.fromJson(Map<String, dynamic> json) =>
      _$$_VeilidConfigNetworkFromJson(json);

  @override
  final int connectionInitialTimeoutMs;
  @override
  final int connectionInactivityTimeoutMs;
  @override
  final int maxConnectionsPerIp4;
  @override
  final int maxConnectionsPerIp6Prefix;
  @override
  final int maxConnectionsPerIp6PrefixSize;
  @override
  final int maxConnectionFrequencyPerMin;
  @override
  final int clientWhitelistTimeoutMs;
  @override
  final int reverseConnectionReceiptTimeMs;
  @override
  final int holePunchReceiptTimeMs;
  @override
  final VeilidConfigRoutingTable routingTable;
  @override
  final VeilidConfigRPC rpc;
  @override
  final VeilidConfigDHT dht;
  @override
  final bool upnp;
  @override
  final bool detectAddressChanges;
  @override
  final int restrictedNatRetries;
  @override
  final VeilidConfigTLS tls;
  @override
  final VeilidConfigApplication application;
  @override
  final VeilidConfigProtocol protocol;
  @override
  final String? networkKeyPassword;

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidConfigNetwork(connectionInitialTimeoutMs: $connectionInitialTimeoutMs, connectionInactivityTimeoutMs: $connectionInactivityTimeoutMs, maxConnectionsPerIp4: $maxConnectionsPerIp4, maxConnectionsPerIp6Prefix: $maxConnectionsPerIp6Prefix, maxConnectionsPerIp6PrefixSize: $maxConnectionsPerIp6PrefixSize, maxConnectionFrequencyPerMin: $maxConnectionFrequencyPerMin, clientWhitelistTimeoutMs: $clientWhitelistTimeoutMs, reverseConnectionReceiptTimeMs: $reverseConnectionReceiptTimeMs, holePunchReceiptTimeMs: $holePunchReceiptTimeMs, routingTable: $routingTable, rpc: $rpc, dht: $dht, upnp: $upnp, detectAddressChanges: $detectAddressChanges, restrictedNatRetries: $restrictedNatRetries, tls: $tls, application: $application, protocol: $protocol, networkKeyPassword: $networkKeyPassword)';
  }

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
    super.debugFillProperties(properties);
    properties
      ..add(DiagnosticsProperty('type', 'VeilidConfigNetwork'))
      ..add(DiagnosticsProperty(
          'connectionInitialTimeoutMs', connectionInitialTimeoutMs))
      ..add(DiagnosticsProperty(
          'connectionInactivityTimeoutMs', connectionInactivityTimeoutMs))
      ..add(DiagnosticsProperty('maxConnectionsPerIp4', maxConnectionsPerIp4))
      ..add(DiagnosticsProperty(
          'maxConnectionsPerIp6Prefix', maxConnectionsPerIp6Prefix))
      ..add(DiagnosticsProperty(
          'maxConnectionsPerIp6PrefixSize', maxConnectionsPerIp6PrefixSize))
      ..add(DiagnosticsProperty(
          'maxConnectionFrequencyPerMin', maxConnectionFrequencyPerMin))
      ..add(DiagnosticsProperty(
          'clientWhitelistTimeoutMs', clientWhitelistTimeoutMs))
      ..add(DiagnosticsProperty(
          'reverseConnectionReceiptTimeMs', reverseConnectionReceiptTimeMs))
      ..add(
          DiagnosticsProperty('holePunchReceiptTimeMs', holePunchReceiptTimeMs))
      ..add(DiagnosticsProperty('routingTable', routingTable))
      ..add(DiagnosticsProperty('rpc', rpc))
      ..add(DiagnosticsProperty('dht', dht))
      ..add(DiagnosticsProperty('upnp', upnp))
      ..add(DiagnosticsProperty('detectAddressChanges', detectAddressChanges))
      ..add(DiagnosticsProperty('restrictedNatRetries', restrictedNatRetries))
      ..add(DiagnosticsProperty('tls', tls))
      ..add(DiagnosticsProperty('application', application))
      ..add(DiagnosticsProperty('protocol', protocol))
      ..add(DiagnosticsProperty('networkKeyPassword', networkKeyPassword));
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$_VeilidConfigNetwork &&
            (identical(other.connectionInitialTimeoutMs, connectionInitialTimeoutMs) ||
                other.connectionInitialTimeoutMs ==
                    connectionInitialTimeoutMs) &&
            (identical(other.connectionInactivityTimeoutMs, connectionInactivityTimeoutMs) ||
                other.connectionInactivityTimeoutMs ==
                    connectionInactivityTimeoutMs) &&
            (identical(other.maxConnectionsPerIp4, maxConnectionsPerIp4) ||
                other.maxConnectionsPerIp4 == maxConnectionsPerIp4) &&
            (identical(other.maxConnectionsPerIp6Prefix, maxConnectionsPerIp6Prefix) ||
                other.maxConnectionsPerIp6Prefix ==
                    maxConnectionsPerIp6Prefix) &&
            (identical(other.maxConnectionsPerIp6PrefixSize, maxConnectionsPerIp6PrefixSize) ||
                other.maxConnectionsPerIp6PrefixSize ==
                    maxConnectionsPerIp6PrefixSize) &&
            (identical(other.maxConnectionFrequencyPerMin, maxConnectionFrequencyPerMin) ||
                other.maxConnectionFrequencyPerMin ==
                    maxConnectionFrequencyPerMin) &&
            (identical(other.clientWhitelistTimeoutMs, clientWhitelistTimeoutMs) ||
                other.clientWhitelistTimeoutMs == clientWhitelistTimeoutMs) &&
            (identical(other.reverseConnectionReceiptTimeMs,
                    reverseConnectionReceiptTimeMs) ||
                other.reverseConnectionReceiptTimeMs ==
                    reverseConnectionReceiptTimeMs) &&
            (identical(other.holePunchReceiptTimeMs, holePunchReceiptTimeMs) ||
                other.holePunchReceiptTimeMs == holePunchReceiptTimeMs) &&
            (identical(other.routingTable, routingTable) ||
                other.routingTable == routingTable) &&
            (identical(other.rpc, rpc) || other.rpc == rpc) &&
            (identical(other.dht, dht) || other.dht == dht) &&
            (identical(other.upnp, upnp) || other.upnp == upnp) &&
            (identical(other.detectAddressChanges, detectAddressChanges) ||
                other.detectAddressChanges == detectAddressChanges) &&
            (identical(other.restrictedNatRetries, restrictedNatRetries) ||
                other.restrictedNatRetries == restrictedNatRetries) &&
            (identical(other.tls, tls) || other.tls == tls) &&
            (identical(other.application, application) ||
                other.application == application) &&
            (identical(other.protocol, protocol) ||
                other.protocol == protocol) &&
            (identical(other.networkKeyPassword, networkKeyPassword) ||
                other.networkKeyPassword == networkKeyPassword));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hashAll([
        runtimeType,
        connectionInitialTimeoutMs,
        connectionInactivityTimeoutMs,
        maxConnectionsPerIp4,
        maxConnectionsPerIp6Prefix,
        maxConnectionsPerIp6PrefixSize,
        maxConnectionFrequencyPerMin,
        clientWhitelistTimeoutMs,
        reverseConnectionReceiptTimeMs,
        holePunchReceiptTimeMs,
        routingTable,
        rpc,
        dht,
        upnp,
        detectAddressChanges,
        restrictedNatRetries,
        tls,
        application,
        protocol,
        networkKeyPassword
      ]);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$_VeilidConfigNetworkCopyWith<_$_VeilidConfigNetwork> get copyWith =>
      __$$_VeilidConfigNetworkCopyWithImpl<_$_VeilidConfigNetwork>(
          this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$_VeilidConfigNetworkToJson(
      this,
    );
  }
}

abstract class _VeilidConfigNetwork implements VeilidConfigNetwork {
  const factory _VeilidConfigNetwork(
      {required final int connectionInitialTimeoutMs,
      required final int connectionInactivityTimeoutMs,
      required final int maxConnectionsPerIp4,
      required final int maxConnectionsPerIp6Prefix,
      required final int maxConnectionsPerIp6PrefixSize,
      required final int maxConnectionFrequencyPerMin,
      required final int clientWhitelistTimeoutMs,
      required final int reverseConnectionReceiptTimeMs,
      required final int holePunchReceiptTimeMs,
      required final VeilidConfigRoutingTable routingTable,
      required final VeilidConfigRPC rpc,
      required final VeilidConfigDHT dht,
      required final bool upnp,
      required final bool detectAddressChanges,
      required final int restrictedNatRetries,
      required final VeilidConfigTLS tls,
      required final VeilidConfigApplication application,
      required final VeilidConfigProtocol protocol,
      final String? networkKeyPassword}) = _$_VeilidConfigNetwork;

  factory _VeilidConfigNetwork.fromJson(Map<String, dynamic> json) =
      _$_VeilidConfigNetwork.fromJson;

  @override
  int get connectionInitialTimeoutMs;
  @override
  int get connectionInactivityTimeoutMs;
  @override
  int get maxConnectionsPerIp4;
  @override
  int get maxConnectionsPerIp6Prefix;
  @override
  int get maxConnectionsPerIp6PrefixSize;
  @override
  int get maxConnectionFrequencyPerMin;
  @override
  int get clientWhitelistTimeoutMs;
  @override
  int get reverseConnectionReceiptTimeMs;
  @override
  int get holePunchReceiptTimeMs;
  @override
  VeilidConfigRoutingTable get routingTable;
  @override
  VeilidConfigRPC get rpc;
  @override
  VeilidConfigDHT get dht;
  @override
  bool get upnp;
  @override
  bool get detectAddressChanges;
  @override
  int get restrictedNatRetries;
  @override
  VeilidConfigTLS get tls;
  @override
  VeilidConfigApplication get application;
  @override
  VeilidConfigProtocol get protocol;
  @override
  String? get networkKeyPassword;
  @override
  @JsonKey(ignore: true)
  _$$_VeilidConfigNetworkCopyWith<_$_VeilidConfigNetwork> get copyWith =>
      throw _privateConstructorUsedError;
}

VeilidConfigTableStore _$VeilidConfigTableStoreFromJson(
    Map<String, dynamic> json) {
  return _VeilidConfigTableStore.fromJson(json);
}

/// @nodoc
mixin _$VeilidConfigTableStore {
  String get directory => throw _privateConstructorUsedError;
  bool get delete => throw _privateConstructorUsedError;

  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
  @JsonKey(ignore: true)
  $VeilidConfigTableStoreCopyWith<VeilidConfigTableStore> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $VeilidConfigTableStoreCopyWith<$Res> {
  factory $VeilidConfigTableStoreCopyWith(VeilidConfigTableStore value,
          $Res Function(VeilidConfigTableStore) then) =
      _$VeilidConfigTableStoreCopyWithImpl<$Res, VeilidConfigTableStore>;
  @useResult
  $Res call({String directory, bool delete});
}

/// @nodoc
class _$VeilidConfigTableStoreCopyWithImpl<$Res,
        $Val extends VeilidConfigTableStore>
    implements $VeilidConfigTableStoreCopyWith<$Res> {
  _$VeilidConfigTableStoreCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? directory = null,
    Object? delete = null,
  }) {
    return _then(_value.copyWith(
      directory: null == directory
          ? _value.directory
          : directory // ignore: cast_nullable_to_non_nullable
              as String,
      delete: null == delete
          ? _value.delete
          : delete // ignore: cast_nullable_to_non_nullable
              as bool,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$_VeilidConfigTableStoreCopyWith<$Res>
    implements $VeilidConfigTableStoreCopyWith<$Res> {
  factory _$$_VeilidConfigTableStoreCopyWith(_$_VeilidConfigTableStore value,
          $Res Function(_$_VeilidConfigTableStore) then) =
      __$$_VeilidConfigTableStoreCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String directory, bool delete});
}

/// @nodoc
class __$$_VeilidConfigTableStoreCopyWithImpl<$Res>
    extends _$VeilidConfigTableStoreCopyWithImpl<$Res,
        _$_VeilidConfigTableStore>
    implements _$$_VeilidConfigTableStoreCopyWith<$Res> {
  __$$_VeilidConfigTableStoreCopyWithImpl(_$_VeilidConfigTableStore _value,
      $Res Function(_$_VeilidConfigTableStore) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? directory = null,
    Object? delete = null,
  }) {
    return _then(_$_VeilidConfigTableStore(
      directory: null == directory
          ? _value.directory
          : directory // ignore: cast_nullable_to_non_nullable
              as String,
      delete: null == delete
          ? _value.delete
          : delete // ignore: cast_nullable_to_non_nullable
              as bool,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$_VeilidConfigTableStore
    with DiagnosticableTreeMixin
    implements _VeilidConfigTableStore {
  const _$_VeilidConfigTableStore(
      {required this.directory, required this.delete});

  factory _$_VeilidConfigTableStore.fromJson(Map<String, dynamic> json) =>
      _$$_VeilidConfigTableStoreFromJson(json);

  @override
  final String directory;
  @override
  final bool delete;

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidConfigTableStore(directory: $directory, delete: $delete)';
  }

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
    super.debugFillProperties(properties);
    properties
      ..add(DiagnosticsProperty('type', 'VeilidConfigTableStore'))
      ..add(DiagnosticsProperty('directory', directory))
      ..add(DiagnosticsProperty('delete', delete));
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$_VeilidConfigTableStore &&
            (identical(other.directory, directory) ||
                other.directory == directory) &&
            (identical(other.delete, delete) || other.delete == delete));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, directory, delete);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$_VeilidConfigTableStoreCopyWith<_$_VeilidConfigTableStore> get copyWith =>
      __$$_VeilidConfigTableStoreCopyWithImpl<_$_VeilidConfigTableStore>(
          this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$_VeilidConfigTableStoreToJson(
      this,
    );
  }
}

abstract class _VeilidConfigTableStore implements VeilidConfigTableStore {
  const factory _VeilidConfigTableStore(
      {required final String directory,
      required final bool delete}) = _$_VeilidConfigTableStore;

  factory _VeilidConfigTableStore.fromJson(Map<String, dynamic> json) =
      _$_VeilidConfigTableStore.fromJson;

  @override
  String get directory;
  @override
  bool get delete;
  @override
  @JsonKey(ignore: true)
  _$$_VeilidConfigTableStoreCopyWith<_$_VeilidConfigTableStore> get copyWith =>
      throw _privateConstructorUsedError;
}

VeilidConfigBlockStore _$VeilidConfigBlockStoreFromJson(
    Map<String, dynamic> json) {
  return _VeilidConfigBlockStore.fromJson(json);
}

/// @nodoc
mixin _$VeilidConfigBlockStore {
  String get directory => throw _privateConstructorUsedError;
  bool get delete => throw _privateConstructorUsedError;

  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
  @JsonKey(ignore: true)
  $VeilidConfigBlockStoreCopyWith<VeilidConfigBlockStore> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $VeilidConfigBlockStoreCopyWith<$Res> {
  factory $VeilidConfigBlockStoreCopyWith(VeilidConfigBlockStore value,
          $Res Function(VeilidConfigBlockStore) then) =
      _$VeilidConfigBlockStoreCopyWithImpl<$Res, VeilidConfigBlockStore>;
  @useResult
  $Res call({String directory, bool delete});
}

/// @nodoc
class _$VeilidConfigBlockStoreCopyWithImpl<$Res,
        $Val extends VeilidConfigBlockStore>
    implements $VeilidConfigBlockStoreCopyWith<$Res> {
  _$VeilidConfigBlockStoreCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? directory = null,
    Object? delete = null,
  }) {
    return _then(_value.copyWith(
      directory: null == directory
          ? _value.directory
          : directory // ignore: cast_nullable_to_non_nullable
              as String,
      delete: null == delete
          ? _value.delete
          : delete // ignore: cast_nullable_to_non_nullable
              as bool,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$_VeilidConfigBlockStoreCopyWith<$Res>
    implements $VeilidConfigBlockStoreCopyWith<$Res> {
  factory _$$_VeilidConfigBlockStoreCopyWith(_$_VeilidConfigBlockStore value,
          $Res Function(_$_VeilidConfigBlockStore) then) =
      __$$_VeilidConfigBlockStoreCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String directory, bool delete});
}

/// @nodoc
class __$$_VeilidConfigBlockStoreCopyWithImpl<$Res>
    extends _$VeilidConfigBlockStoreCopyWithImpl<$Res,
        _$_VeilidConfigBlockStore>
    implements _$$_VeilidConfigBlockStoreCopyWith<$Res> {
  __$$_VeilidConfigBlockStoreCopyWithImpl(_$_VeilidConfigBlockStore _value,
      $Res Function(_$_VeilidConfigBlockStore) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? directory = null,
    Object? delete = null,
  }) {
    return _then(_$_VeilidConfigBlockStore(
      directory: null == directory
          ? _value.directory
          : directory // ignore: cast_nullable_to_non_nullable
              as String,
      delete: null == delete
          ? _value.delete
          : delete // ignore: cast_nullable_to_non_nullable
              as bool,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$_VeilidConfigBlockStore
    with DiagnosticableTreeMixin
    implements _VeilidConfigBlockStore {
  const _$_VeilidConfigBlockStore(
      {required this.directory, required this.delete});

  factory _$_VeilidConfigBlockStore.fromJson(Map<String, dynamic> json) =>
      _$$_VeilidConfigBlockStoreFromJson(json);

  @override
  final String directory;
  @override
  final bool delete;

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidConfigBlockStore(directory: $directory, delete: $delete)';
  }

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
    super.debugFillProperties(properties);
    properties
      ..add(DiagnosticsProperty('type', 'VeilidConfigBlockStore'))
      ..add(DiagnosticsProperty('directory', directory))
      ..add(DiagnosticsProperty('delete', delete));
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$_VeilidConfigBlockStore &&
            (identical(other.directory, directory) ||
                other.directory == directory) &&
            (identical(other.delete, delete) || other.delete == delete));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, directory, delete);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$_VeilidConfigBlockStoreCopyWith<_$_VeilidConfigBlockStore> get copyWith =>
      __$$_VeilidConfigBlockStoreCopyWithImpl<_$_VeilidConfigBlockStore>(
          this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$_VeilidConfigBlockStoreToJson(
      this,
    );
  }
}

abstract class _VeilidConfigBlockStore implements VeilidConfigBlockStore {
  const factory _VeilidConfigBlockStore(
      {required final String directory,
      required final bool delete}) = _$_VeilidConfigBlockStore;

  factory _VeilidConfigBlockStore.fromJson(Map<String, dynamic> json) =
      _$_VeilidConfigBlockStore.fromJson;

  @override
  String get directory;
  @override
  bool get delete;
  @override
  @JsonKey(ignore: true)
  _$$_VeilidConfigBlockStoreCopyWith<_$_VeilidConfigBlockStore> get copyWith =>
      throw _privateConstructorUsedError;
}

VeilidConfigProtectedStore _$VeilidConfigProtectedStoreFromJson(
    Map<String, dynamic> json) {
  return _VeilidConfigProtectedStore.fromJson(json);
}

/// @nodoc
mixin _$VeilidConfigProtectedStore {
  bool get allowInsecureFallback => throw _privateConstructorUsedError;
  bool get alwaysUseInsecureStorage => throw _privateConstructorUsedError;
  String get directory => throw _privateConstructorUsedError;
  bool get delete => throw _privateConstructorUsedError;
  String get deviceEncryptionKeyPassword => throw _privateConstructorUsedError;
  String? get newDeviceEncryptionKeyPassword =>
      throw _privateConstructorUsedError;

  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
  @JsonKey(ignore: true)
  $VeilidConfigProtectedStoreCopyWith<VeilidConfigProtectedStore>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $VeilidConfigProtectedStoreCopyWith<$Res> {
  factory $VeilidConfigProtectedStoreCopyWith(VeilidConfigProtectedStore value,
          $Res Function(VeilidConfigProtectedStore) then) =
      _$VeilidConfigProtectedStoreCopyWithImpl<$Res,
          VeilidConfigProtectedStore>;
  @useResult
  $Res call(
      {bool allowInsecureFallback,
      bool alwaysUseInsecureStorage,
      String directory,
      bool delete,
      String deviceEncryptionKeyPassword,
      String? newDeviceEncryptionKeyPassword});
}

/// @nodoc
class _$VeilidConfigProtectedStoreCopyWithImpl<$Res,
        $Val extends VeilidConfigProtectedStore>
    implements $VeilidConfigProtectedStoreCopyWith<$Res> {
  _$VeilidConfigProtectedStoreCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? allowInsecureFallback = null,
    Object? alwaysUseInsecureStorage = null,
    Object? directory = null,
    Object? delete = null,
    Object? deviceEncryptionKeyPassword = null,
    Object? newDeviceEncryptionKeyPassword = freezed,
  }) {
    return _then(_value.copyWith(
      allowInsecureFallback: null == allowInsecureFallback
          ? _value.allowInsecureFallback
          : allowInsecureFallback // ignore: cast_nullable_to_non_nullable
              as bool,
      alwaysUseInsecureStorage: null == alwaysUseInsecureStorage
          ? _value.alwaysUseInsecureStorage
          : alwaysUseInsecureStorage // ignore: cast_nullable_to_non_nullable
              as bool,
      directory: null == directory
          ? _value.directory
          : directory // ignore: cast_nullable_to_non_nullable
              as String,
      delete: null == delete
          ? _value.delete
          : delete // ignore: cast_nullable_to_non_nullable
              as bool,
      deviceEncryptionKeyPassword: null == deviceEncryptionKeyPassword
          ? _value.deviceEncryptionKeyPassword
          : deviceEncryptionKeyPassword // ignore: cast_nullable_to_non_nullable
              as String,
      newDeviceEncryptionKeyPassword: freezed == newDeviceEncryptionKeyPassword
          ? _value.newDeviceEncryptionKeyPassword
          : newDeviceEncryptionKeyPassword // ignore: cast_nullable_to_non_nullable
              as String?,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$_VeilidConfigProtectedStoreCopyWith<$Res>
    implements $VeilidConfigProtectedStoreCopyWith<$Res> {
  factory _$$_VeilidConfigProtectedStoreCopyWith(
          _$_VeilidConfigProtectedStore value,
          $Res Function(_$_VeilidConfigProtectedStore) then) =
      __$$_VeilidConfigProtectedStoreCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {bool allowInsecureFallback,
      bool alwaysUseInsecureStorage,
      String directory,
      bool delete,
      String deviceEncryptionKeyPassword,
      String? newDeviceEncryptionKeyPassword});
}

/// @nodoc
class __$$_VeilidConfigProtectedStoreCopyWithImpl<$Res>
    extends _$VeilidConfigProtectedStoreCopyWithImpl<$Res,
        _$_VeilidConfigProtectedStore>
    implements _$$_VeilidConfigProtectedStoreCopyWith<$Res> {
  __$$_VeilidConfigProtectedStoreCopyWithImpl(
      _$_VeilidConfigProtectedStore _value,
      $Res Function(_$_VeilidConfigProtectedStore) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? allowInsecureFallback = null,
    Object? alwaysUseInsecureStorage = null,
    Object? directory = null,
    Object? delete = null,
    Object? deviceEncryptionKeyPassword = null,
    Object? newDeviceEncryptionKeyPassword = freezed,
  }) {
    return _then(_$_VeilidConfigProtectedStore(
      allowInsecureFallback: null == allowInsecureFallback
          ? _value.allowInsecureFallback
          : allowInsecureFallback // ignore: cast_nullable_to_non_nullable
              as bool,
      alwaysUseInsecureStorage: null == alwaysUseInsecureStorage
          ? _value.alwaysUseInsecureStorage
          : alwaysUseInsecureStorage // ignore: cast_nullable_to_non_nullable
              as bool,
      directory: null == directory
          ? _value.directory
          : directory // ignore: cast_nullable_to_non_nullable
              as String,
      delete: null == delete
          ? _value.delete
          : delete // ignore: cast_nullable_to_non_nullable
              as bool,
      deviceEncryptionKeyPassword: null == deviceEncryptionKeyPassword
          ? _value.deviceEncryptionKeyPassword
          : deviceEncryptionKeyPassword // ignore: cast_nullable_to_non_nullable
              as String,
      newDeviceEncryptionKeyPassword: freezed == newDeviceEncryptionKeyPassword
          ? _value.newDeviceEncryptionKeyPassword
          : newDeviceEncryptionKeyPassword // ignore: cast_nullable_to_non_nullable
              as String?,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$_VeilidConfigProtectedStore
    with DiagnosticableTreeMixin
    implements _VeilidConfigProtectedStore {
  const _$_VeilidConfigProtectedStore(
      {required this.allowInsecureFallback,
      required this.alwaysUseInsecureStorage,
      required this.directory,
      required this.delete,
      required this.deviceEncryptionKeyPassword,
      this.newDeviceEncryptionKeyPassword});

  factory _$_VeilidConfigProtectedStore.fromJson(Map<String, dynamic> json) =>
      _$$_VeilidConfigProtectedStoreFromJson(json);

  @override
  final bool allowInsecureFallback;
  @override
  final bool alwaysUseInsecureStorage;
  @override
  final String directory;
  @override
  final bool delete;
  @override
  final String deviceEncryptionKeyPassword;
  @override
  final String? newDeviceEncryptionKeyPassword;

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidConfigProtectedStore(allowInsecureFallback: $allowInsecureFallback, alwaysUseInsecureStorage: $alwaysUseInsecureStorage, directory: $directory, delete: $delete, deviceEncryptionKeyPassword: $deviceEncryptionKeyPassword, newDeviceEncryptionKeyPassword: $newDeviceEncryptionKeyPassword)';
  }

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
    super.debugFillProperties(properties);
    properties
      ..add(DiagnosticsProperty('type', 'VeilidConfigProtectedStore'))
      ..add(DiagnosticsProperty('allowInsecureFallback', allowInsecureFallback))
      ..add(DiagnosticsProperty(
          'alwaysUseInsecureStorage', alwaysUseInsecureStorage))
      ..add(DiagnosticsProperty('directory', directory))
      ..add(DiagnosticsProperty('delete', delete))
      ..add(DiagnosticsProperty(
          'deviceEncryptionKeyPassword', deviceEncryptionKeyPassword))
      ..add(DiagnosticsProperty(
          'newDeviceEncryptionKeyPassword', newDeviceEncryptionKeyPassword));
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$_VeilidConfigProtectedStore &&
            (identical(other.allowInsecureFallback, allowInsecureFallback) ||
                other.allowInsecureFallback == allowInsecureFallback) &&
            (identical(
                    other.alwaysUseInsecureStorage, alwaysUseInsecureStorage) ||
                other.alwaysUseInsecureStorage == alwaysUseInsecureStorage) &&
            (identical(other.directory, directory) ||
                other.directory == directory) &&
            (identical(other.delete, delete) || other.delete == delete) &&
            (identical(other.deviceEncryptionKeyPassword,
                    deviceEncryptionKeyPassword) ||
                other.deviceEncryptionKeyPassword ==
                    deviceEncryptionKeyPassword) &&
            (identical(other.newDeviceEncryptionKeyPassword,
                    newDeviceEncryptionKeyPassword) ||
                other.newDeviceEncryptionKeyPassword ==
                    newDeviceEncryptionKeyPassword));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(
      runtimeType,
      allowInsecureFallback,
      alwaysUseInsecureStorage,
      directory,
      delete,
      deviceEncryptionKeyPassword,
      newDeviceEncryptionKeyPassword);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$_VeilidConfigProtectedStoreCopyWith<_$_VeilidConfigProtectedStore>
      get copyWith => __$$_VeilidConfigProtectedStoreCopyWithImpl<
          _$_VeilidConfigProtectedStore>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$_VeilidConfigProtectedStoreToJson(
      this,
    );
  }
}

abstract class _VeilidConfigProtectedStore
    implements VeilidConfigProtectedStore {
  const factory _VeilidConfigProtectedStore(
          {required final bool allowInsecureFallback,
          required final bool alwaysUseInsecureStorage,
          required final String directory,
          required final bool delete,
          required final String deviceEncryptionKeyPassword,
          final String? newDeviceEncryptionKeyPassword}) =
      _$_VeilidConfigProtectedStore;

  factory _VeilidConfigProtectedStore.fromJson(Map<String, dynamic> json) =
      _$_VeilidConfigProtectedStore.fromJson;

  @override
  bool get allowInsecureFallback;
  @override
  bool get alwaysUseInsecureStorage;
  @override
  String get directory;
  @override
  bool get delete;
  @override
  String get deviceEncryptionKeyPassword;
  @override
  String? get newDeviceEncryptionKeyPassword;
  @override
  @JsonKey(ignore: true)
  _$$_VeilidConfigProtectedStoreCopyWith<_$_VeilidConfigProtectedStore>
      get copyWith => throw _privateConstructorUsedError;
}

VeilidConfigCapabilities _$VeilidConfigCapabilitiesFromJson(
    Map<String, dynamic> json) {
  return _VeilidConfigCapabilities.fromJson(json);
}

/// @nodoc
mixin _$VeilidConfigCapabilities {
  List<String> get disable => throw _privateConstructorUsedError;

  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
  @JsonKey(ignore: true)
  $VeilidConfigCapabilitiesCopyWith<VeilidConfigCapabilities> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $VeilidConfigCapabilitiesCopyWith<$Res> {
  factory $VeilidConfigCapabilitiesCopyWith(VeilidConfigCapabilities value,
          $Res Function(VeilidConfigCapabilities) then) =
      _$VeilidConfigCapabilitiesCopyWithImpl<$Res, VeilidConfigCapabilities>;
  @useResult
  $Res call({List<String> disable});
}

/// @nodoc
class _$VeilidConfigCapabilitiesCopyWithImpl<$Res,
        $Val extends VeilidConfigCapabilities>
    implements $VeilidConfigCapabilitiesCopyWith<$Res> {
  _$VeilidConfigCapabilitiesCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? disable = null,
  }) {
    return _then(_value.copyWith(
      disable: null == disable
          ? _value.disable
          : disable // ignore: cast_nullable_to_non_nullable
              as List<String>,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$_VeilidConfigCapabilitiesCopyWith<$Res>
    implements $VeilidConfigCapabilitiesCopyWith<$Res> {
  factory _$$_VeilidConfigCapabilitiesCopyWith(
          _$_VeilidConfigCapabilities value,
          $Res Function(_$_VeilidConfigCapabilities) then) =
      __$$_VeilidConfigCapabilitiesCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({List<String> disable});
}

/// @nodoc
class __$$_VeilidConfigCapabilitiesCopyWithImpl<$Res>
    extends _$VeilidConfigCapabilitiesCopyWithImpl<$Res,
        _$_VeilidConfigCapabilities>
    implements _$$_VeilidConfigCapabilitiesCopyWith<$Res> {
  __$$_VeilidConfigCapabilitiesCopyWithImpl(_$_VeilidConfigCapabilities _value,
      $Res Function(_$_VeilidConfigCapabilities) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? disable = null,
  }) {
    return _then(_$_VeilidConfigCapabilities(
      disable: null == disable
          ? _value._disable
          : disable // ignore: cast_nullable_to_non_nullable
              as List<String>,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$_VeilidConfigCapabilities
    with DiagnosticableTreeMixin
    implements _VeilidConfigCapabilities {
  const _$_VeilidConfigCapabilities({required final List<String> disable})
      : _disable = disable;

  factory _$_VeilidConfigCapabilities.fromJson(Map<String, dynamic> json) =>
      _$$_VeilidConfigCapabilitiesFromJson(json);

  final List<String> _disable;
  @override
  List<String> get disable {
    if (_disable is EqualUnmodifiableListView) return _disable;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_disable);
  }

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidConfigCapabilities(disable: $disable)';
  }

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
    super.debugFillProperties(properties);
    properties
      ..add(DiagnosticsProperty('type', 'VeilidConfigCapabilities'))
      ..add(DiagnosticsProperty('disable', disable));
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$_VeilidConfigCapabilities &&
            const DeepCollectionEquality().equals(other._disable, _disable));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(_disable));

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$_VeilidConfigCapabilitiesCopyWith<_$_VeilidConfigCapabilities>
      get copyWith => __$$_VeilidConfigCapabilitiesCopyWithImpl<
          _$_VeilidConfigCapabilities>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$_VeilidConfigCapabilitiesToJson(
      this,
    );
  }
}

abstract class _VeilidConfigCapabilities implements VeilidConfigCapabilities {
  const factory _VeilidConfigCapabilities(
      {required final List<String> disable}) = _$_VeilidConfigCapabilities;

  factory _VeilidConfigCapabilities.fromJson(Map<String, dynamic> json) =
      _$_VeilidConfigCapabilities.fromJson;

  @override
  List<String> get disable;
  @override
  @JsonKey(ignore: true)
  _$$_VeilidConfigCapabilitiesCopyWith<_$_VeilidConfigCapabilities>
      get copyWith => throw _privateConstructorUsedError;
}

VeilidConfig _$VeilidConfigFromJson(Map<String, dynamic> json) {
  return _VeilidConfig.fromJson(json);
}

/// @nodoc
mixin _$VeilidConfig {
  String get programName => throw _privateConstructorUsedError;
  String get namespace => throw _privateConstructorUsedError;
  VeilidConfigCapabilities get capabilities =>
      throw _privateConstructorUsedError;
  VeilidConfigProtectedStore get protectedStore =>
      throw _privateConstructorUsedError;
  VeilidConfigTableStore get tableStore => throw _privateConstructorUsedError;
  VeilidConfigBlockStore get blockStore => throw _privateConstructorUsedError;
  VeilidConfigNetwork get network => throw _privateConstructorUsedError;

  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
  @JsonKey(ignore: true)
  $VeilidConfigCopyWith<VeilidConfig> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $VeilidConfigCopyWith<$Res> {
  factory $VeilidConfigCopyWith(
          VeilidConfig value, $Res Function(VeilidConfig) then) =
      _$VeilidConfigCopyWithImpl<$Res, VeilidConfig>;
  @useResult
  $Res call(
      {String programName,
      String namespace,
      VeilidConfigCapabilities capabilities,
      VeilidConfigProtectedStore protectedStore,
      VeilidConfigTableStore tableStore,
      VeilidConfigBlockStore blockStore,
      VeilidConfigNetwork network});

  $VeilidConfigCapabilitiesCopyWith<$Res> get capabilities;
  $VeilidConfigProtectedStoreCopyWith<$Res> get protectedStore;
  $VeilidConfigTableStoreCopyWith<$Res> get tableStore;
  $VeilidConfigBlockStoreCopyWith<$Res> get blockStore;
  $VeilidConfigNetworkCopyWith<$Res> get network;
}

/// @nodoc
class _$VeilidConfigCopyWithImpl<$Res, $Val extends VeilidConfig>
    implements $VeilidConfigCopyWith<$Res> {
  _$VeilidConfigCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? programName = null,
    Object? namespace = null,
    Object? capabilities = null,
    Object? protectedStore = null,
    Object? tableStore = null,
    Object? blockStore = null,
    Object? network = null,
  }) {
    return _then(_value.copyWith(
      programName: null == programName
          ? _value.programName
          : programName // ignore: cast_nullable_to_non_nullable
              as String,
      namespace: null == namespace
          ? _value.namespace
          : namespace // ignore: cast_nullable_to_non_nullable
              as String,
      capabilities: null == capabilities
          ? _value.capabilities
          : capabilities // ignore: cast_nullable_to_non_nullable
              as VeilidConfigCapabilities,
      protectedStore: null == protectedStore
          ? _value.protectedStore
          : protectedStore // ignore: cast_nullable_to_non_nullable
              as VeilidConfigProtectedStore,
      tableStore: null == tableStore
          ? _value.tableStore
          : tableStore // ignore: cast_nullable_to_non_nullable
              as VeilidConfigTableStore,
      blockStore: null == blockStore
          ? _value.blockStore
          : blockStore // ignore: cast_nullable_to_non_nullable
              as VeilidConfigBlockStore,
      network: null == network
          ? _value.network
          : network // ignore: cast_nullable_to_non_nullable
              as VeilidConfigNetwork,
    ) as $Val);
  }

  @override
  @pragma('vm:prefer-inline')
  $VeilidConfigCapabilitiesCopyWith<$Res> get capabilities {
    return $VeilidConfigCapabilitiesCopyWith<$Res>(_value.capabilities,
        (value) {
      return _then(_value.copyWith(capabilities: value) as $Val);
    });
  }

  @override
  @pragma('vm:prefer-inline')
  $VeilidConfigProtectedStoreCopyWith<$Res> get protectedStore {
    return $VeilidConfigProtectedStoreCopyWith<$Res>(_value.protectedStore,
        (value) {
      return _then(_value.copyWith(protectedStore: value) as $Val);
    });
  }

  @override
  @pragma('vm:prefer-inline')
  $VeilidConfigTableStoreCopyWith<$Res> get tableStore {
    return $VeilidConfigTableStoreCopyWith<$Res>(_value.tableStore, (value) {
      return _then(_value.copyWith(tableStore: value) as $Val);
    });
  }

  @override
  @pragma('vm:prefer-inline')
  $VeilidConfigBlockStoreCopyWith<$Res> get blockStore {
    return $VeilidConfigBlockStoreCopyWith<$Res>(_value.blockStore, (value) {
      return _then(_value.copyWith(blockStore: value) as $Val);
    });
  }

  @override
  @pragma('vm:prefer-inline')
  $VeilidConfigNetworkCopyWith<$Res> get network {
    return $VeilidConfigNetworkCopyWith<$Res>(_value.network, (value) {
      return _then(_value.copyWith(network: value) as $Val);
    });
  }
}

/// @nodoc
abstract class _$$_VeilidConfigCopyWith<$Res>
    implements $VeilidConfigCopyWith<$Res> {
  factory _$$_VeilidConfigCopyWith(
          _$_VeilidConfig value, $Res Function(_$_VeilidConfig) then) =
      __$$_VeilidConfigCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {String programName,
      String namespace,
      VeilidConfigCapabilities capabilities,
      VeilidConfigProtectedStore protectedStore,
      VeilidConfigTableStore tableStore,
      VeilidConfigBlockStore blockStore,
      VeilidConfigNetwork network});

  @override
  $VeilidConfigCapabilitiesCopyWith<$Res> get capabilities;
  @override
  $VeilidConfigProtectedStoreCopyWith<$Res> get protectedStore;
  @override
  $VeilidConfigTableStoreCopyWith<$Res> get tableStore;
  @override
  $VeilidConfigBlockStoreCopyWith<$Res> get blockStore;
  @override
  $VeilidConfigNetworkCopyWith<$Res> get network;
}

/// @nodoc
class __$$_VeilidConfigCopyWithImpl<$Res>
    extends _$VeilidConfigCopyWithImpl<$Res, _$_VeilidConfig>
    implements _$$_VeilidConfigCopyWith<$Res> {
  __$$_VeilidConfigCopyWithImpl(
      _$_VeilidConfig _value, $Res Function(_$_VeilidConfig) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? programName = null,
    Object? namespace = null,
    Object? capabilities = null,
    Object? protectedStore = null,
    Object? tableStore = null,
    Object? blockStore = null,
    Object? network = null,
  }) {
    return _then(_$_VeilidConfig(
      programName: null == programName
          ? _value.programName
          : programName // ignore: cast_nullable_to_non_nullable
              as String,
      namespace: null == namespace
          ? _value.namespace
          : namespace // ignore: cast_nullable_to_non_nullable
              as String,
      capabilities: null == capabilities
          ? _value.capabilities
          : capabilities // ignore: cast_nullable_to_non_nullable
              as VeilidConfigCapabilities,
      protectedStore: null == protectedStore
          ? _value.protectedStore
          : protectedStore // ignore: cast_nullable_to_non_nullable
              as VeilidConfigProtectedStore,
      tableStore: null == tableStore
          ? _value.tableStore
          : tableStore // ignore: cast_nullable_to_non_nullable
              as VeilidConfigTableStore,
      blockStore: null == blockStore
          ? _value.blockStore
          : blockStore // ignore: cast_nullable_to_non_nullable
              as VeilidConfigBlockStore,
      network: null == network
          ? _value.network
          : network // ignore: cast_nullable_to_non_nullable
              as VeilidConfigNetwork,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$_VeilidConfig with DiagnosticableTreeMixin implements _VeilidConfig {
  const _$_VeilidConfig(
      {required this.programName,
      required this.namespace,
      required this.capabilities,
      required this.protectedStore,
      required this.tableStore,
      required this.blockStore,
      required this.network});

  factory _$_VeilidConfig.fromJson(Map<String, dynamic> json) =>
      _$$_VeilidConfigFromJson(json);

  @override
  final String programName;
  @override
  final String namespace;
  @override
  final VeilidConfigCapabilities capabilities;
  @override
  final VeilidConfigProtectedStore protectedStore;
  @override
  final VeilidConfigTableStore tableStore;
  @override
  final VeilidConfigBlockStore blockStore;
  @override
  final VeilidConfigNetwork network;

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidConfig(programName: $programName, namespace: $namespace, capabilities: $capabilities, protectedStore: $protectedStore, tableStore: $tableStore, blockStore: $blockStore, network: $network)';
  }

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
    super.debugFillProperties(properties);
    properties
      ..add(DiagnosticsProperty('type', 'VeilidConfig'))
      ..add(DiagnosticsProperty('programName', programName))
      ..add(DiagnosticsProperty('namespace', namespace))
      ..add(DiagnosticsProperty('capabilities', capabilities))
      ..add(DiagnosticsProperty('protectedStore', protectedStore))
      ..add(DiagnosticsProperty('tableStore', tableStore))
      ..add(DiagnosticsProperty('blockStore', blockStore))
      ..add(DiagnosticsProperty('network', network));
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$_VeilidConfig &&
            (identical(other.programName, programName) ||
                other.programName == programName) &&
            (identical(other.namespace, namespace) ||
                other.namespace == namespace) &&
            (identical(other.capabilities, capabilities) ||
                other.capabilities == capabilities) &&
            (identical(other.protectedStore, protectedStore) ||
                other.protectedStore == protectedStore) &&
            (identical(other.tableStore, tableStore) ||
                other.tableStore == tableStore) &&
            (identical(other.blockStore, blockStore) ||
                other.blockStore == blockStore) &&
            (identical(other.network, network) || other.network == network));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, programName, namespace,
      capabilities, protectedStore, tableStore, blockStore, network);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$_VeilidConfigCopyWith<_$_VeilidConfig> get copyWith =>
      __$$_VeilidConfigCopyWithImpl<_$_VeilidConfig>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$_VeilidConfigToJson(
      this,
    );
  }
}

abstract class _VeilidConfig implements VeilidConfig {
  const factory _VeilidConfig(
      {required final String programName,
      required final String namespace,
      required final VeilidConfigCapabilities capabilities,
      required final VeilidConfigProtectedStore protectedStore,
      required final VeilidConfigTableStore tableStore,
      required final VeilidConfigBlockStore blockStore,
      required final VeilidConfigNetwork network}) = _$_VeilidConfig;

  factory _VeilidConfig.fromJson(Map<String, dynamic> json) =
      _$_VeilidConfig.fromJson;

  @override
  String get programName;
  @override
  String get namespace;
  @override
  VeilidConfigCapabilities get capabilities;
  @override
  VeilidConfigProtectedStore get protectedStore;
  @override
  VeilidConfigTableStore get tableStore;
  @override
  VeilidConfigBlockStore get blockStore;
  @override
  VeilidConfigNetwork get network;
  @override
  @JsonKey(ignore: true)
  _$$_VeilidConfigCopyWith<_$_VeilidConfig> get copyWith =>
      throw _privateConstructorUsedError;
}
