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
abstract class _$$VeilidFFIConfigLoggingTerminalImplCopyWith<$Res>
    implements $VeilidFFIConfigLoggingTerminalCopyWith<$Res> {
  factory _$$VeilidFFIConfigLoggingTerminalImplCopyWith(
          _$VeilidFFIConfigLoggingTerminalImpl value,
          $Res Function(_$VeilidFFIConfigLoggingTerminalImpl) then) =
      __$$VeilidFFIConfigLoggingTerminalImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({bool enabled, VeilidConfigLogLevel level});
}

/// @nodoc
class __$$VeilidFFIConfigLoggingTerminalImplCopyWithImpl<$Res>
    extends _$VeilidFFIConfigLoggingTerminalCopyWithImpl<$Res,
        _$VeilidFFIConfigLoggingTerminalImpl>
    implements _$$VeilidFFIConfigLoggingTerminalImplCopyWith<$Res> {
  __$$VeilidFFIConfigLoggingTerminalImplCopyWithImpl(
      _$VeilidFFIConfigLoggingTerminalImpl _value,
      $Res Function(_$VeilidFFIConfigLoggingTerminalImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? enabled = null,
    Object? level = null,
  }) {
    return _then(_$VeilidFFIConfigLoggingTerminalImpl(
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
class _$VeilidFFIConfigLoggingTerminalImpl
    with DiagnosticableTreeMixin
    implements _VeilidFFIConfigLoggingTerminal {
  const _$VeilidFFIConfigLoggingTerminalImpl(
      {required this.enabled, required this.level});

  factory _$VeilidFFIConfigLoggingTerminalImpl.fromJson(
          Map<String, dynamic> json) =>
      _$$VeilidFFIConfigLoggingTerminalImplFromJson(json);

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
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$VeilidFFIConfigLoggingTerminalImpl &&
            (identical(other.enabled, enabled) || other.enabled == enabled) &&
            (identical(other.level, level) || other.level == level));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, enabled, level);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$VeilidFFIConfigLoggingTerminalImplCopyWith<
          _$VeilidFFIConfigLoggingTerminalImpl>
      get copyWith => __$$VeilidFFIConfigLoggingTerminalImplCopyWithImpl<
          _$VeilidFFIConfigLoggingTerminalImpl>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$VeilidFFIConfigLoggingTerminalImplToJson(
      this,
    );
  }
}

abstract class _VeilidFFIConfigLoggingTerminal
    implements VeilidFFIConfigLoggingTerminal {
  const factory _VeilidFFIConfigLoggingTerminal(
          {required final bool enabled,
          required final VeilidConfigLogLevel level}) =
      _$VeilidFFIConfigLoggingTerminalImpl;

  factory _VeilidFFIConfigLoggingTerminal.fromJson(Map<String, dynamic> json) =
      _$VeilidFFIConfigLoggingTerminalImpl.fromJson;

  @override
  bool get enabled;
  @override
  VeilidConfigLogLevel get level;
  @override
  @JsonKey(ignore: true)
  _$$VeilidFFIConfigLoggingTerminalImplCopyWith<
          _$VeilidFFIConfigLoggingTerminalImpl>
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
abstract class _$$VeilidFFIConfigLoggingOtlpImplCopyWith<$Res>
    implements $VeilidFFIConfigLoggingOtlpCopyWith<$Res> {
  factory _$$VeilidFFIConfigLoggingOtlpImplCopyWith(
          _$VeilidFFIConfigLoggingOtlpImpl value,
          $Res Function(_$VeilidFFIConfigLoggingOtlpImpl) then) =
      __$$VeilidFFIConfigLoggingOtlpImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {bool enabled,
      VeilidConfigLogLevel level,
      String grpcEndpoint,
      String serviceName});
}

/// @nodoc
class __$$VeilidFFIConfigLoggingOtlpImplCopyWithImpl<$Res>
    extends _$VeilidFFIConfigLoggingOtlpCopyWithImpl<$Res,
        _$VeilidFFIConfigLoggingOtlpImpl>
    implements _$$VeilidFFIConfigLoggingOtlpImplCopyWith<$Res> {
  __$$VeilidFFIConfigLoggingOtlpImplCopyWithImpl(
      _$VeilidFFIConfigLoggingOtlpImpl _value,
      $Res Function(_$VeilidFFIConfigLoggingOtlpImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? enabled = null,
    Object? level = null,
    Object? grpcEndpoint = null,
    Object? serviceName = null,
  }) {
    return _then(_$VeilidFFIConfigLoggingOtlpImpl(
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
class _$VeilidFFIConfigLoggingOtlpImpl
    with DiagnosticableTreeMixin
    implements _VeilidFFIConfigLoggingOtlp {
  const _$VeilidFFIConfigLoggingOtlpImpl(
      {required this.enabled,
      required this.level,
      required this.grpcEndpoint,
      required this.serviceName});

  factory _$VeilidFFIConfigLoggingOtlpImpl.fromJson(
          Map<String, dynamic> json) =>
      _$$VeilidFFIConfigLoggingOtlpImplFromJson(json);

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
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$VeilidFFIConfigLoggingOtlpImpl &&
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
  _$$VeilidFFIConfigLoggingOtlpImplCopyWith<_$VeilidFFIConfigLoggingOtlpImpl>
      get copyWith => __$$VeilidFFIConfigLoggingOtlpImplCopyWithImpl<
          _$VeilidFFIConfigLoggingOtlpImpl>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$VeilidFFIConfigLoggingOtlpImplToJson(
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
      required final String serviceName}) = _$VeilidFFIConfigLoggingOtlpImpl;

  factory _VeilidFFIConfigLoggingOtlp.fromJson(Map<String, dynamic> json) =
      _$VeilidFFIConfigLoggingOtlpImpl.fromJson;

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
  _$$VeilidFFIConfigLoggingOtlpImplCopyWith<_$VeilidFFIConfigLoggingOtlpImpl>
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
abstract class _$$VeilidFFIConfigLoggingApiImplCopyWith<$Res>
    implements $VeilidFFIConfigLoggingApiCopyWith<$Res> {
  factory _$$VeilidFFIConfigLoggingApiImplCopyWith(
          _$VeilidFFIConfigLoggingApiImpl value,
          $Res Function(_$VeilidFFIConfigLoggingApiImpl) then) =
      __$$VeilidFFIConfigLoggingApiImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({bool enabled, VeilidConfigLogLevel level});
}

/// @nodoc
class __$$VeilidFFIConfigLoggingApiImplCopyWithImpl<$Res>
    extends _$VeilidFFIConfigLoggingApiCopyWithImpl<$Res,
        _$VeilidFFIConfigLoggingApiImpl>
    implements _$$VeilidFFIConfigLoggingApiImplCopyWith<$Res> {
  __$$VeilidFFIConfigLoggingApiImplCopyWithImpl(
      _$VeilidFFIConfigLoggingApiImpl _value,
      $Res Function(_$VeilidFFIConfigLoggingApiImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? enabled = null,
    Object? level = null,
  }) {
    return _then(_$VeilidFFIConfigLoggingApiImpl(
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
class _$VeilidFFIConfigLoggingApiImpl
    with DiagnosticableTreeMixin
    implements _VeilidFFIConfigLoggingApi {
  const _$VeilidFFIConfigLoggingApiImpl(
      {required this.enabled, required this.level});

  factory _$VeilidFFIConfigLoggingApiImpl.fromJson(Map<String, dynamic> json) =>
      _$$VeilidFFIConfigLoggingApiImplFromJson(json);

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
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$VeilidFFIConfigLoggingApiImpl &&
            (identical(other.enabled, enabled) || other.enabled == enabled) &&
            (identical(other.level, level) || other.level == level));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, enabled, level);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$VeilidFFIConfigLoggingApiImplCopyWith<_$VeilidFFIConfigLoggingApiImpl>
      get copyWith => __$$VeilidFFIConfigLoggingApiImplCopyWithImpl<
          _$VeilidFFIConfigLoggingApiImpl>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$VeilidFFIConfigLoggingApiImplToJson(
      this,
    );
  }
}

abstract class _VeilidFFIConfigLoggingApi implements VeilidFFIConfigLoggingApi {
  const factory _VeilidFFIConfigLoggingApi(
          {required final bool enabled,
          required final VeilidConfigLogLevel level}) =
      _$VeilidFFIConfigLoggingApiImpl;

  factory _VeilidFFIConfigLoggingApi.fromJson(Map<String, dynamic> json) =
      _$VeilidFFIConfigLoggingApiImpl.fromJson;

  @override
  bool get enabled;
  @override
  VeilidConfigLogLevel get level;
  @override
  @JsonKey(ignore: true)
  _$$VeilidFFIConfigLoggingApiImplCopyWith<_$VeilidFFIConfigLoggingApiImpl>
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
abstract class _$$VeilidFFIConfigLoggingImplCopyWith<$Res>
    implements $VeilidFFIConfigLoggingCopyWith<$Res> {
  factory _$$VeilidFFIConfigLoggingImplCopyWith(
          _$VeilidFFIConfigLoggingImpl value,
          $Res Function(_$VeilidFFIConfigLoggingImpl) then) =
      __$$VeilidFFIConfigLoggingImplCopyWithImpl<$Res>;
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
class __$$VeilidFFIConfigLoggingImplCopyWithImpl<$Res>
    extends _$VeilidFFIConfigLoggingCopyWithImpl<$Res,
        _$VeilidFFIConfigLoggingImpl>
    implements _$$VeilidFFIConfigLoggingImplCopyWith<$Res> {
  __$$VeilidFFIConfigLoggingImplCopyWithImpl(
      _$VeilidFFIConfigLoggingImpl _value,
      $Res Function(_$VeilidFFIConfigLoggingImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? terminal = null,
    Object? otlp = null,
    Object? api = null,
  }) {
    return _then(_$VeilidFFIConfigLoggingImpl(
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
class _$VeilidFFIConfigLoggingImpl
    with DiagnosticableTreeMixin
    implements _VeilidFFIConfigLogging {
  const _$VeilidFFIConfigLoggingImpl(
      {required this.terminal, required this.otlp, required this.api});

  factory _$VeilidFFIConfigLoggingImpl.fromJson(Map<String, dynamic> json) =>
      _$$VeilidFFIConfigLoggingImplFromJson(json);

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
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$VeilidFFIConfigLoggingImpl &&
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
  _$$VeilidFFIConfigLoggingImplCopyWith<_$VeilidFFIConfigLoggingImpl>
      get copyWith => __$$VeilidFFIConfigLoggingImplCopyWithImpl<
          _$VeilidFFIConfigLoggingImpl>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$VeilidFFIConfigLoggingImplToJson(
      this,
    );
  }
}

abstract class _VeilidFFIConfigLogging implements VeilidFFIConfigLogging {
  const factory _VeilidFFIConfigLogging(
          {required final VeilidFFIConfigLoggingTerminal terminal,
          required final VeilidFFIConfigLoggingOtlp otlp,
          required final VeilidFFIConfigLoggingApi api}) =
      _$VeilidFFIConfigLoggingImpl;

  factory _VeilidFFIConfigLogging.fromJson(Map<String, dynamic> json) =
      _$VeilidFFIConfigLoggingImpl.fromJson;

  @override
  VeilidFFIConfigLoggingTerminal get terminal;
  @override
  VeilidFFIConfigLoggingOtlp get otlp;
  @override
  VeilidFFIConfigLoggingApi get api;
  @override
  @JsonKey(ignore: true)
  _$$VeilidFFIConfigLoggingImplCopyWith<_$VeilidFFIConfigLoggingImpl>
      get copyWith => throw _privateConstructorUsedError;
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
abstract class _$$VeilidFFIConfigImplCopyWith<$Res>
    implements $VeilidFFIConfigCopyWith<$Res> {
  factory _$$VeilidFFIConfigImplCopyWith(_$VeilidFFIConfigImpl value,
          $Res Function(_$VeilidFFIConfigImpl) then) =
      __$$VeilidFFIConfigImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({VeilidFFIConfigLogging logging});

  @override
  $VeilidFFIConfigLoggingCopyWith<$Res> get logging;
}

/// @nodoc
class __$$VeilidFFIConfigImplCopyWithImpl<$Res>
    extends _$VeilidFFIConfigCopyWithImpl<$Res, _$VeilidFFIConfigImpl>
    implements _$$VeilidFFIConfigImplCopyWith<$Res> {
  __$$VeilidFFIConfigImplCopyWithImpl(
      _$VeilidFFIConfigImpl _value, $Res Function(_$VeilidFFIConfigImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? logging = null,
  }) {
    return _then(_$VeilidFFIConfigImpl(
      logging: null == logging
          ? _value.logging
          : logging // ignore: cast_nullable_to_non_nullable
              as VeilidFFIConfigLogging,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$VeilidFFIConfigImpl
    with DiagnosticableTreeMixin
    implements _VeilidFFIConfig {
  const _$VeilidFFIConfigImpl({required this.logging});

  factory _$VeilidFFIConfigImpl.fromJson(Map<String, dynamic> json) =>
      _$$VeilidFFIConfigImplFromJson(json);

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
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$VeilidFFIConfigImpl &&
            (identical(other.logging, logging) || other.logging == logging));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, logging);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$VeilidFFIConfigImplCopyWith<_$VeilidFFIConfigImpl> get copyWith =>
      __$$VeilidFFIConfigImplCopyWithImpl<_$VeilidFFIConfigImpl>(
          this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$VeilidFFIConfigImplToJson(
      this,
    );
  }
}

abstract class _VeilidFFIConfig implements VeilidFFIConfig {
  const factory _VeilidFFIConfig(
      {required final VeilidFFIConfigLogging logging}) = _$VeilidFFIConfigImpl;

  factory _VeilidFFIConfig.fromJson(Map<String, dynamic> json) =
      _$VeilidFFIConfigImpl.fromJson;

  @override
  VeilidFFIConfigLogging get logging;
  @override
  @JsonKey(ignore: true)
  _$$VeilidFFIConfigImplCopyWith<_$VeilidFFIConfigImpl> get copyWith =>
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
abstract class _$$VeilidWASMConfigLoggingPerformanceImplCopyWith<$Res>
    implements $VeilidWASMConfigLoggingPerformanceCopyWith<$Res> {
  factory _$$VeilidWASMConfigLoggingPerformanceImplCopyWith(
          _$VeilidWASMConfigLoggingPerformanceImpl value,
          $Res Function(_$VeilidWASMConfigLoggingPerformanceImpl) then) =
      __$$VeilidWASMConfigLoggingPerformanceImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {bool enabled,
      VeilidConfigLogLevel level,
      bool logsInTimings,
      bool logsInConsole});
}

/// @nodoc
class __$$VeilidWASMConfigLoggingPerformanceImplCopyWithImpl<$Res>
    extends _$VeilidWASMConfigLoggingPerformanceCopyWithImpl<$Res,
        _$VeilidWASMConfigLoggingPerformanceImpl>
    implements _$$VeilidWASMConfigLoggingPerformanceImplCopyWith<$Res> {
  __$$VeilidWASMConfigLoggingPerformanceImplCopyWithImpl(
      _$VeilidWASMConfigLoggingPerformanceImpl _value,
      $Res Function(_$VeilidWASMConfigLoggingPerformanceImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? enabled = null,
    Object? level = null,
    Object? logsInTimings = null,
    Object? logsInConsole = null,
  }) {
    return _then(_$VeilidWASMConfigLoggingPerformanceImpl(
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
class _$VeilidWASMConfigLoggingPerformanceImpl
    with DiagnosticableTreeMixin
    implements _VeilidWASMConfigLoggingPerformance {
  const _$VeilidWASMConfigLoggingPerformanceImpl(
      {required this.enabled,
      required this.level,
      required this.logsInTimings,
      required this.logsInConsole});

  factory _$VeilidWASMConfigLoggingPerformanceImpl.fromJson(
          Map<String, dynamic> json) =>
      _$$VeilidWASMConfigLoggingPerformanceImplFromJson(json);

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
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$VeilidWASMConfigLoggingPerformanceImpl &&
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
  _$$VeilidWASMConfigLoggingPerformanceImplCopyWith<
          _$VeilidWASMConfigLoggingPerformanceImpl>
      get copyWith => __$$VeilidWASMConfigLoggingPerformanceImplCopyWithImpl<
          _$VeilidWASMConfigLoggingPerformanceImpl>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$VeilidWASMConfigLoggingPerformanceImplToJson(
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
      _$VeilidWASMConfigLoggingPerformanceImpl;

  factory _VeilidWASMConfigLoggingPerformance.fromJson(
          Map<String, dynamic> json) =
      _$VeilidWASMConfigLoggingPerformanceImpl.fromJson;

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
  _$$VeilidWASMConfigLoggingPerformanceImplCopyWith<
          _$VeilidWASMConfigLoggingPerformanceImpl>
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
abstract class _$$VeilidWASMConfigLoggingApiImplCopyWith<$Res>
    implements $VeilidWASMConfigLoggingApiCopyWith<$Res> {
  factory _$$VeilidWASMConfigLoggingApiImplCopyWith(
          _$VeilidWASMConfigLoggingApiImpl value,
          $Res Function(_$VeilidWASMConfigLoggingApiImpl) then) =
      __$$VeilidWASMConfigLoggingApiImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({bool enabled, VeilidConfigLogLevel level});
}

/// @nodoc
class __$$VeilidWASMConfigLoggingApiImplCopyWithImpl<$Res>
    extends _$VeilidWASMConfigLoggingApiCopyWithImpl<$Res,
        _$VeilidWASMConfigLoggingApiImpl>
    implements _$$VeilidWASMConfigLoggingApiImplCopyWith<$Res> {
  __$$VeilidWASMConfigLoggingApiImplCopyWithImpl(
      _$VeilidWASMConfigLoggingApiImpl _value,
      $Res Function(_$VeilidWASMConfigLoggingApiImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? enabled = null,
    Object? level = null,
  }) {
    return _then(_$VeilidWASMConfigLoggingApiImpl(
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
class _$VeilidWASMConfigLoggingApiImpl
    with DiagnosticableTreeMixin
    implements _VeilidWASMConfigLoggingApi {
  const _$VeilidWASMConfigLoggingApiImpl(
      {required this.enabled, required this.level});

  factory _$VeilidWASMConfigLoggingApiImpl.fromJson(
          Map<String, dynamic> json) =>
      _$$VeilidWASMConfigLoggingApiImplFromJson(json);

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
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$VeilidWASMConfigLoggingApiImpl &&
            (identical(other.enabled, enabled) || other.enabled == enabled) &&
            (identical(other.level, level) || other.level == level));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, enabled, level);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$VeilidWASMConfigLoggingApiImplCopyWith<_$VeilidWASMConfigLoggingApiImpl>
      get copyWith => __$$VeilidWASMConfigLoggingApiImplCopyWithImpl<
          _$VeilidWASMConfigLoggingApiImpl>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$VeilidWASMConfigLoggingApiImplToJson(
      this,
    );
  }
}

abstract class _VeilidWASMConfigLoggingApi
    implements VeilidWASMConfigLoggingApi {
  const factory _VeilidWASMConfigLoggingApi(
          {required final bool enabled,
          required final VeilidConfigLogLevel level}) =
      _$VeilidWASMConfigLoggingApiImpl;

  factory _VeilidWASMConfigLoggingApi.fromJson(Map<String, dynamic> json) =
      _$VeilidWASMConfigLoggingApiImpl.fromJson;

  @override
  bool get enabled;
  @override
  VeilidConfigLogLevel get level;
  @override
  @JsonKey(ignore: true)
  _$$VeilidWASMConfigLoggingApiImplCopyWith<_$VeilidWASMConfigLoggingApiImpl>
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
abstract class _$$VeilidWASMConfigLoggingImplCopyWith<$Res>
    implements $VeilidWASMConfigLoggingCopyWith<$Res> {
  factory _$$VeilidWASMConfigLoggingImplCopyWith(
          _$VeilidWASMConfigLoggingImpl value,
          $Res Function(_$VeilidWASMConfigLoggingImpl) then) =
      __$$VeilidWASMConfigLoggingImplCopyWithImpl<$Res>;
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
class __$$VeilidWASMConfigLoggingImplCopyWithImpl<$Res>
    extends _$VeilidWASMConfigLoggingCopyWithImpl<$Res,
        _$VeilidWASMConfigLoggingImpl>
    implements _$$VeilidWASMConfigLoggingImplCopyWith<$Res> {
  __$$VeilidWASMConfigLoggingImplCopyWithImpl(
      _$VeilidWASMConfigLoggingImpl _value,
      $Res Function(_$VeilidWASMConfigLoggingImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? performance = null,
    Object? api = null,
  }) {
    return _then(_$VeilidWASMConfigLoggingImpl(
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
class _$VeilidWASMConfigLoggingImpl
    with DiagnosticableTreeMixin
    implements _VeilidWASMConfigLogging {
  const _$VeilidWASMConfigLoggingImpl(
      {required this.performance, required this.api});

  factory _$VeilidWASMConfigLoggingImpl.fromJson(Map<String, dynamic> json) =>
      _$$VeilidWASMConfigLoggingImplFromJson(json);

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
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$VeilidWASMConfigLoggingImpl &&
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
  _$$VeilidWASMConfigLoggingImplCopyWith<_$VeilidWASMConfigLoggingImpl>
      get copyWith => __$$VeilidWASMConfigLoggingImplCopyWithImpl<
          _$VeilidWASMConfigLoggingImpl>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$VeilidWASMConfigLoggingImplToJson(
      this,
    );
  }
}

abstract class _VeilidWASMConfigLogging implements VeilidWASMConfigLogging {
  const factory _VeilidWASMConfigLogging(
          {required final VeilidWASMConfigLoggingPerformance performance,
          required final VeilidWASMConfigLoggingApi api}) =
      _$VeilidWASMConfigLoggingImpl;

  factory _VeilidWASMConfigLogging.fromJson(Map<String, dynamic> json) =
      _$VeilidWASMConfigLoggingImpl.fromJson;

  @override
  VeilidWASMConfigLoggingPerformance get performance;
  @override
  VeilidWASMConfigLoggingApi get api;
  @override
  @JsonKey(ignore: true)
  _$$VeilidWASMConfigLoggingImplCopyWith<_$VeilidWASMConfigLoggingImpl>
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
abstract class _$$VeilidWASMConfigImplCopyWith<$Res>
    implements $VeilidWASMConfigCopyWith<$Res> {
  factory _$$VeilidWASMConfigImplCopyWith(_$VeilidWASMConfigImpl value,
          $Res Function(_$VeilidWASMConfigImpl) then) =
      __$$VeilidWASMConfigImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({VeilidWASMConfigLogging logging});

  @override
  $VeilidWASMConfigLoggingCopyWith<$Res> get logging;
}

/// @nodoc
class __$$VeilidWASMConfigImplCopyWithImpl<$Res>
    extends _$VeilidWASMConfigCopyWithImpl<$Res, _$VeilidWASMConfigImpl>
    implements _$$VeilidWASMConfigImplCopyWith<$Res> {
  __$$VeilidWASMConfigImplCopyWithImpl(_$VeilidWASMConfigImpl _value,
      $Res Function(_$VeilidWASMConfigImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? logging = null,
  }) {
    return _then(_$VeilidWASMConfigImpl(
      logging: null == logging
          ? _value.logging
          : logging // ignore: cast_nullable_to_non_nullable
              as VeilidWASMConfigLogging,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$VeilidWASMConfigImpl
    with DiagnosticableTreeMixin
    implements _VeilidWASMConfig {
  const _$VeilidWASMConfigImpl({required this.logging});

  factory _$VeilidWASMConfigImpl.fromJson(Map<String, dynamic> json) =>
      _$$VeilidWASMConfigImplFromJson(json);

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
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$VeilidWASMConfigImpl &&
            (identical(other.logging, logging) || other.logging == logging));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, logging);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$VeilidWASMConfigImplCopyWith<_$VeilidWASMConfigImpl> get copyWith =>
      __$$VeilidWASMConfigImplCopyWithImpl<_$VeilidWASMConfigImpl>(
          this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$VeilidWASMConfigImplToJson(
      this,
    );
  }
}

abstract class _VeilidWASMConfig implements VeilidWASMConfig {
  const factory _VeilidWASMConfig(
          {required final VeilidWASMConfigLogging logging}) =
      _$VeilidWASMConfigImpl;

  factory _VeilidWASMConfig.fromJson(Map<String, dynamic> json) =
      _$VeilidWASMConfigImpl.fromJson;

  @override
  VeilidWASMConfigLogging get logging;
  @override
  @JsonKey(ignore: true)
  _$$VeilidWASMConfigImplCopyWith<_$VeilidWASMConfigImpl> get copyWith =>
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
abstract class _$$VeilidConfigHTTPSImplCopyWith<$Res>
    implements $VeilidConfigHTTPSCopyWith<$Res> {
  factory _$$VeilidConfigHTTPSImplCopyWith(_$VeilidConfigHTTPSImpl value,
          $Res Function(_$VeilidConfigHTTPSImpl) then) =
      __$$VeilidConfigHTTPSImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({bool enabled, String listenAddress, String path, String? url});
}

/// @nodoc
class __$$VeilidConfigHTTPSImplCopyWithImpl<$Res>
    extends _$VeilidConfigHTTPSCopyWithImpl<$Res, _$VeilidConfigHTTPSImpl>
    implements _$$VeilidConfigHTTPSImplCopyWith<$Res> {
  __$$VeilidConfigHTTPSImplCopyWithImpl(_$VeilidConfigHTTPSImpl _value,
      $Res Function(_$VeilidConfigHTTPSImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? enabled = null,
    Object? listenAddress = null,
    Object? path = null,
    Object? url = freezed,
  }) {
    return _then(_$VeilidConfigHTTPSImpl(
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
class _$VeilidConfigHTTPSImpl
    with DiagnosticableTreeMixin
    implements _VeilidConfigHTTPS {
  const _$VeilidConfigHTTPSImpl(
      {required this.enabled,
      required this.listenAddress,
      required this.path,
      this.url});

  factory _$VeilidConfigHTTPSImpl.fromJson(Map<String, dynamic> json) =>
      _$$VeilidConfigHTTPSImplFromJson(json);

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
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$VeilidConfigHTTPSImpl &&
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
  _$$VeilidConfigHTTPSImplCopyWith<_$VeilidConfigHTTPSImpl> get copyWith =>
      __$$VeilidConfigHTTPSImplCopyWithImpl<_$VeilidConfigHTTPSImpl>(
          this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$VeilidConfigHTTPSImplToJson(
      this,
    );
  }
}

abstract class _VeilidConfigHTTPS implements VeilidConfigHTTPS {
  const factory _VeilidConfigHTTPS(
      {required final bool enabled,
      required final String listenAddress,
      required final String path,
      final String? url}) = _$VeilidConfigHTTPSImpl;

  factory _VeilidConfigHTTPS.fromJson(Map<String, dynamic> json) =
      _$VeilidConfigHTTPSImpl.fromJson;

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
  _$$VeilidConfigHTTPSImplCopyWith<_$VeilidConfigHTTPSImpl> get copyWith =>
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
abstract class _$$VeilidConfigHTTPImplCopyWith<$Res>
    implements $VeilidConfigHTTPCopyWith<$Res> {
  factory _$$VeilidConfigHTTPImplCopyWith(_$VeilidConfigHTTPImpl value,
          $Res Function(_$VeilidConfigHTTPImpl) then) =
      __$$VeilidConfigHTTPImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({bool enabled, String listenAddress, String path, String? url});
}

/// @nodoc
class __$$VeilidConfigHTTPImplCopyWithImpl<$Res>
    extends _$VeilidConfigHTTPCopyWithImpl<$Res, _$VeilidConfigHTTPImpl>
    implements _$$VeilidConfigHTTPImplCopyWith<$Res> {
  __$$VeilidConfigHTTPImplCopyWithImpl(_$VeilidConfigHTTPImpl _value,
      $Res Function(_$VeilidConfigHTTPImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? enabled = null,
    Object? listenAddress = null,
    Object? path = null,
    Object? url = freezed,
  }) {
    return _then(_$VeilidConfigHTTPImpl(
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
class _$VeilidConfigHTTPImpl
    with DiagnosticableTreeMixin
    implements _VeilidConfigHTTP {
  const _$VeilidConfigHTTPImpl(
      {required this.enabled,
      required this.listenAddress,
      required this.path,
      this.url});

  factory _$VeilidConfigHTTPImpl.fromJson(Map<String, dynamic> json) =>
      _$$VeilidConfigHTTPImplFromJson(json);

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
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$VeilidConfigHTTPImpl &&
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
  _$$VeilidConfigHTTPImplCopyWith<_$VeilidConfigHTTPImpl> get copyWith =>
      __$$VeilidConfigHTTPImplCopyWithImpl<_$VeilidConfigHTTPImpl>(
          this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$VeilidConfigHTTPImplToJson(
      this,
    );
  }
}

abstract class _VeilidConfigHTTP implements VeilidConfigHTTP {
  const factory _VeilidConfigHTTP(
      {required final bool enabled,
      required final String listenAddress,
      required final String path,
      final String? url}) = _$VeilidConfigHTTPImpl;

  factory _VeilidConfigHTTP.fromJson(Map<String, dynamic> json) =
      _$VeilidConfigHTTPImpl.fromJson;

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
  _$$VeilidConfigHTTPImplCopyWith<_$VeilidConfigHTTPImpl> get copyWith =>
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
abstract class _$$VeilidConfigApplicationImplCopyWith<$Res>
    implements $VeilidConfigApplicationCopyWith<$Res> {
  factory _$$VeilidConfigApplicationImplCopyWith(
          _$VeilidConfigApplicationImpl value,
          $Res Function(_$VeilidConfigApplicationImpl) then) =
      __$$VeilidConfigApplicationImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({VeilidConfigHTTPS https, VeilidConfigHTTP http});

  @override
  $VeilidConfigHTTPSCopyWith<$Res> get https;
  @override
  $VeilidConfigHTTPCopyWith<$Res> get http;
}

/// @nodoc
class __$$VeilidConfigApplicationImplCopyWithImpl<$Res>
    extends _$VeilidConfigApplicationCopyWithImpl<$Res,
        _$VeilidConfigApplicationImpl>
    implements _$$VeilidConfigApplicationImplCopyWith<$Res> {
  __$$VeilidConfigApplicationImplCopyWithImpl(
      _$VeilidConfigApplicationImpl _value,
      $Res Function(_$VeilidConfigApplicationImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? https = null,
    Object? http = null,
  }) {
    return _then(_$VeilidConfigApplicationImpl(
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
class _$VeilidConfigApplicationImpl
    with DiagnosticableTreeMixin
    implements _VeilidConfigApplication {
  const _$VeilidConfigApplicationImpl(
      {required this.https, required this.http});

  factory _$VeilidConfigApplicationImpl.fromJson(Map<String, dynamic> json) =>
      _$$VeilidConfigApplicationImplFromJson(json);

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
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$VeilidConfigApplicationImpl &&
            (identical(other.https, https) || other.https == https) &&
            (identical(other.http, http) || other.http == http));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, https, http);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$VeilidConfigApplicationImplCopyWith<_$VeilidConfigApplicationImpl>
      get copyWith => __$$VeilidConfigApplicationImplCopyWithImpl<
          _$VeilidConfigApplicationImpl>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$VeilidConfigApplicationImplToJson(
      this,
    );
  }
}

abstract class _VeilidConfigApplication implements VeilidConfigApplication {
  const factory _VeilidConfigApplication(
      {required final VeilidConfigHTTPS https,
      required final VeilidConfigHTTP http}) = _$VeilidConfigApplicationImpl;

  factory _VeilidConfigApplication.fromJson(Map<String, dynamic> json) =
      _$VeilidConfigApplicationImpl.fromJson;

  @override
  VeilidConfigHTTPS get https;
  @override
  VeilidConfigHTTP get http;
  @override
  @JsonKey(ignore: true)
  _$$VeilidConfigApplicationImplCopyWith<_$VeilidConfigApplicationImpl>
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
abstract class _$$VeilidConfigUDPImplCopyWith<$Res>
    implements $VeilidConfigUDPCopyWith<$Res> {
  factory _$$VeilidConfigUDPImplCopyWith(_$VeilidConfigUDPImpl value,
          $Res Function(_$VeilidConfigUDPImpl) then) =
      __$$VeilidConfigUDPImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {bool enabled,
      int socketPoolSize,
      String listenAddress,
      String? publicAddress});
}

/// @nodoc
class __$$VeilidConfigUDPImplCopyWithImpl<$Res>
    extends _$VeilidConfigUDPCopyWithImpl<$Res, _$VeilidConfigUDPImpl>
    implements _$$VeilidConfigUDPImplCopyWith<$Res> {
  __$$VeilidConfigUDPImplCopyWithImpl(
      _$VeilidConfigUDPImpl _value, $Res Function(_$VeilidConfigUDPImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? enabled = null,
    Object? socketPoolSize = null,
    Object? listenAddress = null,
    Object? publicAddress = freezed,
  }) {
    return _then(_$VeilidConfigUDPImpl(
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
class _$VeilidConfigUDPImpl
    with DiagnosticableTreeMixin
    implements _VeilidConfigUDP {
  const _$VeilidConfigUDPImpl(
      {required this.enabled,
      required this.socketPoolSize,
      required this.listenAddress,
      this.publicAddress});

  factory _$VeilidConfigUDPImpl.fromJson(Map<String, dynamic> json) =>
      _$$VeilidConfigUDPImplFromJson(json);

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
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$VeilidConfigUDPImpl &&
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
  _$$VeilidConfigUDPImplCopyWith<_$VeilidConfigUDPImpl> get copyWith =>
      __$$VeilidConfigUDPImplCopyWithImpl<_$VeilidConfigUDPImpl>(
          this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$VeilidConfigUDPImplToJson(
      this,
    );
  }
}

abstract class _VeilidConfigUDP implements VeilidConfigUDP {
  const factory _VeilidConfigUDP(
      {required final bool enabled,
      required final int socketPoolSize,
      required final String listenAddress,
      final String? publicAddress}) = _$VeilidConfigUDPImpl;

  factory _VeilidConfigUDP.fromJson(Map<String, dynamic> json) =
      _$VeilidConfigUDPImpl.fromJson;

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
  _$$VeilidConfigUDPImplCopyWith<_$VeilidConfigUDPImpl> get copyWith =>
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
abstract class _$$VeilidConfigTCPImplCopyWith<$Res>
    implements $VeilidConfigTCPCopyWith<$Res> {
  factory _$$VeilidConfigTCPImplCopyWith(_$VeilidConfigTCPImpl value,
          $Res Function(_$VeilidConfigTCPImpl) then) =
      __$$VeilidConfigTCPImplCopyWithImpl<$Res>;
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
class __$$VeilidConfigTCPImplCopyWithImpl<$Res>
    extends _$VeilidConfigTCPCopyWithImpl<$Res, _$VeilidConfigTCPImpl>
    implements _$$VeilidConfigTCPImplCopyWith<$Res> {
  __$$VeilidConfigTCPImplCopyWithImpl(
      _$VeilidConfigTCPImpl _value, $Res Function(_$VeilidConfigTCPImpl) _then)
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
    return _then(_$VeilidConfigTCPImpl(
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
class _$VeilidConfigTCPImpl
    with DiagnosticableTreeMixin
    implements _VeilidConfigTCP {
  const _$VeilidConfigTCPImpl(
      {required this.connect,
      required this.listen,
      required this.maxConnections,
      required this.listenAddress,
      this.publicAddress});

  factory _$VeilidConfigTCPImpl.fromJson(Map<String, dynamic> json) =>
      _$$VeilidConfigTCPImplFromJson(json);

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
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$VeilidConfigTCPImpl &&
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
  _$$VeilidConfigTCPImplCopyWith<_$VeilidConfigTCPImpl> get copyWith =>
      __$$VeilidConfigTCPImplCopyWithImpl<_$VeilidConfigTCPImpl>(
          this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$VeilidConfigTCPImplToJson(
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
      final String? publicAddress}) = _$VeilidConfigTCPImpl;

  factory _VeilidConfigTCP.fromJson(Map<String, dynamic> json) =
      _$VeilidConfigTCPImpl.fromJson;

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
  _$$VeilidConfigTCPImplCopyWith<_$VeilidConfigTCPImpl> get copyWith =>
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
abstract class _$$VeilidConfigWSImplCopyWith<$Res>
    implements $VeilidConfigWSCopyWith<$Res> {
  factory _$$VeilidConfigWSImplCopyWith(_$VeilidConfigWSImpl value,
          $Res Function(_$VeilidConfigWSImpl) then) =
      __$$VeilidConfigWSImplCopyWithImpl<$Res>;
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
class __$$VeilidConfigWSImplCopyWithImpl<$Res>
    extends _$VeilidConfigWSCopyWithImpl<$Res, _$VeilidConfigWSImpl>
    implements _$$VeilidConfigWSImplCopyWith<$Res> {
  __$$VeilidConfigWSImplCopyWithImpl(
      _$VeilidConfigWSImpl _value, $Res Function(_$VeilidConfigWSImpl) _then)
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
    return _then(_$VeilidConfigWSImpl(
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
class _$VeilidConfigWSImpl
    with DiagnosticableTreeMixin
    implements _VeilidConfigWS {
  const _$VeilidConfigWSImpl(
      {required this.connect,
      required this.listen,
      required this.maxConnections,
      required this.listenAddress,
      required this.path,
      this.url});

  factory _$VeilidConfigWSImpl.fromJson(Map<String, dynamic> json) =>
      _$$VeilidConfigWSImplFromJson(json);

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
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$VeilidConfigWSImpl &&
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
  _$$VeilidConfigWSImplCopyWith<_$VeilidConfigWSImpl> get copyWith =>
      __$$VeilidConfigWSImplCopyWithImpl<_$VeilidConfigWSImpl>(
          this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$VeilidConfigWSImplToJson(
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
      final String? url}) = _$VeilidConfigWSImpl;

  factory _VeilidConfigWS.fromJson(Map<String, dynamic> json) =
      _$VeilidConfigWSImpl.fromJson;

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
  _$$VeilidConfigWSImplCopyWith<_$VeilidConfigWSImpl> get copyWith =>
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
abstract class _$$VeilidConfigWSSImplCopyWith<$Res>
    implements $VeilidConfigWSSCopyWith<$Res> {
  factory _$$VeilidConfigWSSImplCopyWith(_$VeilidConfigWSSImpl value,
          $Res Function(_$VeilidConfigWSSImpl) then) =
      __$$VeilidConfigWSSImplCopyWithImpl<$Res>;
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
class __$$VeilidConfigWSSImplCopyWithImpl<$Res>
    extends _$VeilidConfigWSSCopyWithImpl<$Res, _$VeilidConfigWSSImpl>
    implements _$$VeilidConfigWSSImplCopyWith<$Res> {
  __$$VeilidConfigWSSImplCopyWithImpl(
      _$VeilidConfigWSSImpl _value, $Res Function(_$VeilidConfigWSSImpl) _then)
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
    return _then(_$VeilidConfigWSSImpl(
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
class _$VeilidConfigWSSImpl
    with DiagnosticableTreeMixin
    implements _VeilidConfigWSS {
  const _$VeilidConfigWSSImpl(
      {required this.connect,
      required this.listen,
      required this.maxConnections,
      required this.listenAddress,
      required this.path,
      this.url});

  factory _$VeilidConfigWSSImpl.fromJson(Map<String, dynamic> json) =>
      _$$VeilidConfigWSSImplFromJson(json);

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
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$VeilidConfigWSSImpl &&
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
  _$$VeilidConfigWSSImplCopyWith<_$VeilidConfigWSSImpl> get copyWith =>
      __$$VeilidConfigWSSImplCopyWithImpl<_$VeilidConfigWSSImpl>(
          this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$VeilidConfigWSSImplToJson(
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
      final String? url}) = _$VeilidConfigWSSImpl;

  factory _VeilidConfigWSS.fromJson(Map<String, dynamic> json) =
      _$VeilidConfigWSSImpl.fromJson;

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
  _$$VeilidConfigWSSImplCopyWith<_$VeilidConfigWSSImpl> get copyWith =>
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
abstract class _$$VeilidConfigProtocolImplCopyWith<$Res>
    implements $VeilidConfigProtocolCopyWith<$Res> {
  factory _$$VeilidConfigProtocolImplCopyWith(_$VeilidConfigProtocolImpl value,
          $Res Function(_$VeilidConfigProtocolImpl) then) =
      __$$VeilidConfigProtocolImplCopyWithImpl<$Res>;
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
class __$$VeilidConfigProtocolImplCopyWithImpl<$Res>
    extends _$VeilidConfigProtocolCopyWithImpl<$Res, _$VeilidConfigProtocolImpl>
    implements _$$VeilidConfigProtocolImplCopyWith<$Res> {
  __$$VeilidConfigProtocolImplCopyWithImpl(_$VeilidConfigProtocolImpl _value,
      $Res Function(_$VeilidConfigProtocolImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? udp = null,
    Object? tcp = null,
    Object? ws = null,
    Object? wss = null,
  }) {
    return _then(_$VeilidConfigProtocolImpl(
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
class _$VeilidConfigProtocolImpl
    with DiagnosticableTreeMixin
    implements _VeilidConfigProtocol {
  const _$VeilidConfigProtocolImpl(
      {required this.udp,
      required this.tcp,
      required this.ws,
      required this.wss});

  factory _$VeilidConfigProtocolImpl.fromJson(Map<String, dynamic> json) =>
      _$$VeilidConfigProtocolImplFromJson(json);

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
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$VeilidConfigProtocolImpl &&
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
  _$$VeilidConfigProtocolImplCopyWith<_$VeilidConfigProtocolImpl>
      get copyWith =>
          __$$VeilidConfigProtocolImplCopyWithImpl<_$VeilidConfigProtocolImpl>(
              this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$VeilidConfigProtocolImplToJson(
      this,
    );
  }
}

abstract class _VeilidConfigProtocol implements VeilidConfigProtocol {
  const factory _VeilidConfigProtocol(
      {required final VeilidConfigUDP udp,
      required final VeilidConfigTCP tcp,
      required final VeilidConfigWS ws,
      required final VeilidConfigWSS wss}) = _$VeilidConfigProtocolImpl;

  factory _VeilidConfigProtocol.fromJson(Map<String, dynamic> json) =
      _$VeilidConfigProtocolImpl.fromJson;

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
  _$$VeilidConfigProtocolImplCopyWith<_$VeilidConfigProtocolImpl>
      get copyWith => throw _privateConstructorUsedError;
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
abstract class _$$VeilidConfigTLSImplCopyWith<$Res>
    implements $VeilidConfigTLSCopyWith<$Res> {
  factory _$$VeilidConfigTLSImplCopyWith(_$VeilidConfigTLSImpl value,
          $Res Function(_$VeilidConfigTLSImpl) then) =
      __$$VeilidConfigTLSImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {String certificatePath,
      String privateKeyPath,
      int connectionInitialTimeoutMs});
}

/// @nodoc
class __$$VeilidConfigTLSImplCopyWithImpl<$Res>
    extends _$VeilidConfigTLSCopyWithImpl<$Res, _$VeilidConfigTLSImpl>
    implements _$$VeilidConfigTLSImplCopyWith<$Res> {
  __$$VeilidConfigTLSImplCopyWithImpl(
      _$VeilidConfigTLSImpl _value, $Res Function(_$VeilidConfigTLSImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? certificatePath = null,
    Object? privateKeyPath = null,
    Object? connectionInitialTimeoutMs = null,
  }) {
    return _then(_$VeilidConfigTLSImpl(
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
class _$VeilidConfigTLSImpl
    with DiagnosticableTreeMixin
    implements _VeilidConfigTLS {
  const _$VeilidConfigTLSImpl(
      {required this.certificatePath,
      required this.privateKeyPath,
      required this.connectionInitialTimeoutMs});

  factory _$VeilidConfigTLSImpl.fromJson(Map<String, dynamic> json) =>
      _$$VeilidConfigTLSImplFromJson(json);

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
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$VeilidConfigTLSImpl &&
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
  _$$VeilidConfigTLSImplCopyWith<_$VeilidConfigTLSImpl> get copyWith =>
      __$$VeilidConfigTLSImplCopyWithImpl<_$VeilidConfigTLSImpl>(
          this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$VeilidConfigTLSImplToJson(
      this,
    );
  }
}

abstract class _VeilidConfigTLS implements VeilidConfigTLS {
  const factory _VeilidConfigTLS(
      {required final String certificatePath,
      required final String privateKeyPath,
      required final int connectionInitialTimeoutMs}) = _$VeilidConfigTLSImpl;

  factory _VeilidConfigTLS.fromJson(Map<String, dynamic> json) =
      _$VeilidConfigTLSImpl.fromJson;

  @override
  String get certificatePath;
  @override
  String get privateKeyPath;
  @override
  int get connectionInitialTimeoutMs;
  @override
  @JsonKey(ignore: true)
  _$$VeilidConfigTLSImplCopyWith<_$VeilidConfigTLSImpl> get copyWith =>
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
  int get publicWatchLimit => throw _privateConstructorUsedError;
  int get memberWatchLimit => throw _privateConstructorUsedError;
  int get maxWatchExpirationMs => throw _privateConstructorUsedError;

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
      int remoteMaxStorageSpaceMb,
      int publicWatchLimit,
      int memberWatchLimit,
      int maxWatchExpirationMs});
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
    Object? publicWatchLimit = null,
    Object? memberWatchLimit = null,
    Object? maxWatchExpirationMs = null,
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
      publicWatchLimit: null == publicWatchLimit
          ? _value.publicWatchLimit
          : publicWatchLimit // ignore: cast_nullable_to_non_nullable
              as int,
      memberWatchLimit: null == memberWatchLimit
          ? _value.memberWatchLimit
          : memberWatchLimit // ignore: cast_nullable_to_non_nullable
              as int,
      maxWatchExpirationMs: null == maxWatchExpirationMs
          ? _value.maxWatchExpirationMs
          : maxWatchExpirationMs // ignore: cast_nullable_to_non_nullable
              as int,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$VeilidConfigDHTImplCopyWith<$Res>
    implements $VeilidConfigDHTCopyWith<$Res> {
  factory _$$VeilidConfigDHTImplCopyWith(_$VeilidConfigDHTImpl value,
          $Res Function(_$VeilidConfigDHTImpl) then) =
      __$$VeilidConfigDHTImplCopyWithImpl<$Res>;
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
      int remoteMaxStorageSpaceMb,
      int publicWatchLimit,
      int memberWatchLimit,
      int maxWatchExpirationMs});
}

/// @nodoc
class __$$VeilidConfigDHTImplCopyWithImpl<$Res>
    extends _$VeilidConfigDHTCopyWithImpl<$Res, _$VeilidConfigDHTImpl>
    implements _$$VeilidConfigDHTImplCopyWith<$Res> {
  __$$VeilidConfigDHTImplCopyWithImpl(
      _$VeilidConfigDHTImpl _value, $Res Function(_$VeilidConfigDHTImpl) _then)
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
    Object? publicWatchLimit = null,
    Object? memberWatchLimit = null,
    Object? maxWatchExpirationMs = null,
  }) {
    return _then(_$VeilidConfigDHTImpl(
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
      publicWatchLimit: null == publicWatchLimit
          ? _value.publicWatchLimit
          : publicWatchLimit // ignore: cast_nullable_to_non_nullable
              as int,
      memberWatchLimit: null == memberWatchLimit
          ? _value.memberWatchLimit
          : memberWatchLimit // ignore: cast_nullable_to_non_nullable
              as int,
      maxWatchExpirationMs: null == maxWatchExpirationMs
          ? _value.maxWatchExpirationMs
          : maxWatchExpirationMs // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$VeilidConfigDHTImpl
    with DiagnosticableTreeMixin
    implements _VeilidConfigDHT {
  const _$VeilidConfigDHTImpl(
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
      required this.remoteMaxStorageSpaceMb,
      required this.publicWatchLimit,
      required this.memberWatchLimit,
      required this.maxWatchExpirationMs});

  factory _$VeilidConfigDHTImpl.fromJson(Map<String, dynamic> json) =>
      _$$VeilidConfigDHTImplFromJson(json);

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
  final int publicWatchLimit;
  @override
  final int memberWatchLimit;
  @override
  final int maxWatchExpirationMs;

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidConfigDHT(resolveNodeTimeoutMs: $resolveNodeTimeoutMs, resolveNodeCount: $resolveNodeCount, resolveNodeFanout: $resolveNodeFanout, maxFindNodeCount: $maxFindNodeCount, getValueTimeoutMs: $getValueTimeoutMs, getValueCount: $getValueCount, getValueFanout: $getValueFanout, setValueTimeoutMs: $setValueTimeoutMs, setValueCount: $setValueCount, setValueFanout: $setValueFanout, minPeerCount: $minPeerCount, minPeerRefreshTimeMs: $minPeerRefreshTimeMs, validateDialInfoReceiptTimeMs: $validateDialInfoReceiptTimeMs, localSubkeyCacheSize: $localSubkeyCacheSize, localMaxSubkeyCacheMemoryMb: $localMaxSubkeyCacheMemoryMb, remoteSubkeyCacheSize: $remoteSubkeyCacheSize, remoteMaxRecords: $remoteMaxRecords, remoteMaxSubkeyCacheMemoryMb: $remoteMaxSubkeyCacheMemoryMb, remoteMaxStorageSpaceMb: $remoteMaxStorageSpaceMb, publicWatchLimit: $publicWatchLimit, memberWatchLimit: $memberWatchLimit, maxWatchExpirationMs: $maxWatchExpirationMs)';
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
          'remoteMaxStorageSpaceMb', remoteMaxStorageSpaceMb))
      ..add(DiagnosticsProperty('publicWatchLimit', publicWatchLimit))
      ..add(DiagnosticsProperty('memberWatchLimit', memberWatchLimit))
      ..add(DiagnosticsProperty('maxWatchExpirationMs', maxWatchExpirationMs));
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$VeilidConfigDHTImpl &&
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
            (identical(other.remoteMaxStorageSpaceMb, remoteMaxStorageSpaceMb) ||
                other.remoteMaxStorageSpaceMb == remoteMaxStorageSpaceMb) &&
            (identical(other.publicWatchLimit, publicWatchLimit) ||
                other.publicWatchLimit == publicWatchLimit) &&
            (identical(other.memberWatchLimit, memberWatchLimit) ||
                other.memberWatchLimit == memberWatchLimit) &&
            (identical(other.maxWatchExpirationMs, maxWatchExpirationMs) ||
                other.maxWatchExpirationMs == maxWatchExpirationMs));
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
        remoteMaxStorageSpaceMb,
        publicWatchLimit,
        memberWatchLimit,
        maxWatchExpirationMs
      ]);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$VeilidConfigDHTImplCopyWith<_$VeilidConfigDHTImpl> get copyWith =>
      __$$VeilidConfigDHTImplCopyWithImpl<_$VeilidConfigDHTImpl>(
          this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$VeilidConfigDHTImplToJson(
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
      required final int remoteMaxStorageSpaceMb,
      required final int publicWatchLimit,
      required final int memberWatchLimit,
      required final int maxWatchExpirationMs}) = _$VeilidConfigDHTImpl;

  factory _VeilidConfigDHT.fromJson(Map<String, dynamic> json) =
      _$VeilidConfigDHTImpl.fromJson;

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
  int get publicWatchLimit;
  @override
  int get memberWatchLimit;
  @override
  int get maxWatchExpirationMs;
  @override
  @JsonKey(ignore: true)
  _$$VeilidConfigDHTImplCopyWith<_$VeilidConfigDHTImpl> get copyWith =>
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
abstract class _$$VeilidConfigRPCImplCopyWith<$Res>
    implements $VeilidConfigRPCCopyWith<$Res> {
  factory _$$VeilidConfigRPCImplCopyWith(_$VeilidConfigRPCImpl value,
          $Res Function(_$VeilidConfigRPCImpl) then) =
      __$$VeilidConfigRPCImplCopyWithImpl<$Res>;
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
class __$$VeilidConfigRPCImplCopyWithImpl<$Res>
    extends _$VeilidConfigRPCCopyWithImpl<$Res, _$VeilidConfigRPCImpl>
    implements _$$VeilidConfigRPCImplCopyWith<$Res> {
  __$$VeilidConfigRPCImplCopyWithImpl(
      _$VeilidConfigRPCImpl _value, $Res Function(_$VeilidConfigRPCImpl) _then)
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
    return _then(_$VeilidConfigRPCImpl(
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
class _$VeilidConfigRPCImpl
    with DiagnosticableTreeMixin
    implements _VeilidConfigRPC {
  const _$VeilidConfigRPCImpl(
      {required this.concurrency,
      required this.queueSize,
      required this.timeoutMs,
      required this.maxRouteHopCount,
      required this.defaultRouteHopCount,
      this.maxTimestampBehindMs,
      this.maxTimestampAheadMs});

  factory _$VeilidConfigRPCImpl.fromJson(Map<String, dynamic> json) =>
      _$$VeilidConfigRPCImplFromJson(json);

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
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$VeilidConfigRPCImpl &&
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
  _$$VeilidConfigRPCImplCopyWith<_$VeilidConfigRPCImpl> get copyWith =>
      __$$VeilidConfigRPCImplCopyWithImpl<_$VeilidConfigRPCImpl>(
          this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$VeilidConfigRPCImplToJson(
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
      final int? maxTimestampAheadMs}) = _$VeilidConfigRPCImpl;

  factory _VeilidConfigRPC.fromJson(Map<String, dynamic> json) =
      _$VeilidConfigRPCImpl.fromJson;

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
  _$$VeilidConfigRPCImplCopyWith<_$VeilidConfigRPCImpl> get copyWith =>
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
abstract class _$$VeilidConfigRoutingTableImplCopyWith<$Res>
    implements $VeilidConfigRoutingTableCopyWith<$Res> {
  factory _$$VeilidConfigRoutingTableImplCopyWith(
          _$VeilidConfigRoutingTableImpl value,
          $Res Function(_$VeilidConfigRoutingTableImpl) then) =
      __$$VeilidConfigRoutingTableImplCopyWithImpl<$Res>;
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
class __$$VeilidConfigRoutingTableImplCopyWithImpl<$Res>
    extends _$VeilidConfigRoutingTableCopyWithImpl<$Res,
        _$VeilidConfigRoutingTableImpl>
    implements _$$VeilidConfigRoutingTableImplCopyWith<$Res> {
  __$$VeilidConfigRoutingTableImplCopyWithImpl(
      _$VeilidConfigRoutingTableImpl _value,
      $Res Function(_$VeilidConfigRoutingTableImpl) _then)
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
    return _then(_$VeilidConfigRoutingTableImpl(
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
class _$VeilidConfigRoutingTableImpl
    with DiagnosticableTreeMixin
    implements _VeilidConfigRoutingTable {
  const _$VeilidConfigRoutingTableImpl(
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

  factory _$VeilidConfigRoutingTableImpl.fromJson(Map<String, dynamic> json) =>
      _$$VeilidConfigRoutingTableImplFromJson(json);

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
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$VeilidConfigRoutingTableImpl &&
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
  _$$VeilidConfigRoutingTableImplCopyWith<_$VeilidConfigRoutingTableImpl>
      get copyWith => __$$VeilidConfigRoutingTableImplCopyWithImpl<
          _$VeilidConfigRoutingTableImpl>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$VeilidConfigRoutingTableImplToJson(
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
      required final int limitAttachedWeak}) = _$VeilidConfigRoutingTableImpl;

  factory _VeilidConfigRoutingTable.fromJson(Map<String, dynamic> json) =
      _$VeilidConfigRoutingTableImpl.fromJson;

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
  _$$VeilidConfigRoutingTableImplCopyWith<_$VeilidConfigRoutingTableImpl>
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
  int get clientAllowlistTimeoutMs => throw _privateConstructorUsedError;
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
      int clientAllowlistTimeoutMs,
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
    Object? clientAllowlistTimeoutMs = null,
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
      clientAllowlistTimeoutMs: null == clientAllowlistTimeoutMs
          ? _value.clientAllowlistTimeoutMs
          : clientAllowlistTimeoutMs // ignore: cast_nullable_to_non_nullable
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
abstract class _$$VeilidConfigNetworkImplCopyWith<$Res>
    implements $VeilidConfigNetworkCopyWith<$Res> {
  factory _$$VeilidConfigNetworkImplCopyWith(_$VeilidConfigNetworkImpl value,
          $Res Function(_$VeilidConfigNetworkImpl) then) =
      __$$VeilidConfigNetworkImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {int connectionInitialTimeoutMs,
      int connectionInactivityTimeoutMs,
      int maxConnectionsPerIp4,
      int maxConnectionsPerIp6Prefix,
      int maxConnectionsPerIp6PrefixSize,
      int maxConnectionFrequencyPerMin,
      int clientAllowlistTimeoutMs,
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
class __$$VeilidConfigNetworkImplCopyWithImpl<$Res>
    extends _$VeilidConfigNetworkCopyWithImpl<$Res, _$VeilidConfigNetworkImpl>
    implements _$$VeilidConfigNetworkImplCopyWith<$Res> {
  __$$VeilidConfigNetworkImplCopyWithImpl(_$VeilidConfigNetworkImpl _value,
      $Res Function(_$VeilidConfigNetworkImpl) _then)
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
    Object? clientAllowlistTimeoutMs = null,
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
    return _then(_$VeilidConfigNetworkImpl(
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
      clientAllowlistTimeoutMs: null == clientAllowlistTimeoutMs
          ? _value.clientAllowlistTimeoutMs
          : clientAllowlistTimeoutMs // ignore: cast_nullable_to_non_nullable
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
class _$VeilidConfigNetworkImpl
    with DiagnosticableTreeMixin
    implements _VeilidConfigNetwork {
  const _$VeilidConfigNetworkImpl(
      {required this.connectionInitialTimeoutMs,
      required this.connectionInactivityTimeoutMs,
      required this.maxConnectionsPerIp4,
      required this.maxConnectionsPerIp6Prefix,
      required this.maxConnectionsPerIp6PrefixSize,
      required this.maxConnectionFrequencyPerMin,
      required this.clientAllowlistTimeoutMs,
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

  factory _$VeilidConfigNetworkImpl.fromJson(Map<String, dynamic> json) =>
      _$$VeilidConfigNetworkImplFromJson(json);

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
  final int clientAllowlistTimeoutMs;
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
    return 'VeilidConfigNetwork(connectionInitialTimeoutMs: $connectionInitialTimeoutMs, connectionInactivityTimeoutMs: $connectionInactivityTimeoutMs, maxConnectionsPerIp4: $maxConnectionsPerIp4, maxConnectionsPerIp6Prefix: $maxConnectionsPerIp6Prefix, maxConnectionsPerIp6PrefixSize: $maxConnectionsPerIp6PrefixSize, maxConnectionFrequencyPerMin: $maxConnectionFrequencyPerMin, clientAllowlistTimeoutMs: $clientAllowlistTimeoutMs, reverseConnectionReceiptTimeMs: $reverseConnectionReceiptTimeMs, holePunchReceiptTimeMs: $holePunchReceiptTimeMs, routingTable: $routingTable, rpc: $rpc, dht: $dht, upnp: $upnp, detectAddressChanges: $detectAddressChanges, restrictedNatRetries: $restrictedNatRetries, tls: $tls, application: $application, protocol: $protocol, networkKeyPassword: $networkKeyPassword)';
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
          'clientAllowlistTimeoutMs', clientAllowlistTimeoutMs))
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
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$VeilidConfigNetworkImpl &&
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
            (identical(other.clientAllowlistTimeoutMs, clientAllowlistTimeoutMs) ||
                other.clientAllowlistTimeoutMs == clientAllowlistTimeoutMs) &&
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
        clientAllowlistTimeoutMs,
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
  _$$VeilidConfigNetworkImplCopyWith<_$VeilidConfigNetworkImpl> get copyWith =>
      __$$VeilidConfigNetworkImplCopyWithImpl<_$VeilidConfigNetworkImpl>(
          this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$VeilidConfigNetworkImplToJson(
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
      required final int clientAllowlistTimeoutMs,
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
      final String? networkKeyPassword}) = _$VeilidConfigNetworkImpl;

  factory _VeilidConfigNetwork.fromJson(Map<String, dynamic> json) =
      _$VeilidConfigNetworkImpl.fromJson;

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
  int get clientAllowlistTimeoutMs;
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
  _$$VeilidConfigNetworkImplCopyWith<_$VeilidConfigNetworkImpl> get copyWith =>
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
abstract class _$$VeilidConfigTableStoreImplCopyWith<$Res>
    implements $VeilidConfigTableStoreCopyWith<$Res> {
  factory _$$VeilidConfigTableStoreImplCopyWith(
          _$VeilidConfigTableStoreImpl value,
          $Res Function(_$VeilidConfigTableStoreImpl) then) =
      __$$VeilidConfigTableStoreImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String directory, bool delete});
}

/// @nodoc
class __$$VeilidConfigTableStoreImplCopyWithImpl<$Res>
    extends _$VeilidConfigTableStoreCopyWithImpl<$Res,
        _$VeilidConfigTableStoreImpl>
    implements _$$VeilidConfigTableStoreImplCopyWith<$Res> {
  __$$VeilidConfigTableStoreImplCopyWithImpl(
      _$VeilidConfigTableStoreImpl _value,
      $Res Function(_$VeilidConfigTableStoreImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? directory = null,
    Object? delete = null,
  }) {
    return _then(_$VeilidConfigTableStoreImpl(
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
class _$VeilidConfigTableStoreImpl
    with DiagnosticableTreeMixin
    implements _VeilidConfigTableStore {
  const _$VeilidConfigTableStoreImpl(
      {required this.directory, required this.delete});

  factory _$VeilidConfigTableStoreImpl.fromJson(Map<String, dynamic> json) =>
      _$$VeilidConfigTableStoreImplFromJson(json);

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
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$VeilidConfigTableStoreImpl &&
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
  _$$VeilidConfigTableStoreImplCopyWith<_$VeilidConfigTableStoreImpl>
      get copyWith => __$$VeilidConfigTableStoreImplCopyWithImpl<
          _$VeilidConfigTableStoreImpl>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$VeilidConfigTableStoreImplToJson(
      this,
    );
  }
}

abstract class _VeilidConfigTableStore implements VeilidConfigTableStore {
  const factory _VeilidConfigTableStore(
      {required final String directory,
      required final bool delete}) = _$VeilidConfigTableStoreImpl;

  factory _VeilidConfigTableStore.fromJson(Map<String, dynamic> json) =
      _$VeilidConfigTableStoreImpl.fromJson;

  @override
  String get directory;
  @override
  bool get delete;
  @override
  @JsonKey(ignore: true)
  _$$VeilidConfigTableStoreImplCopyWith<_$VeilidConfigTableStoreImpl>
      get copyWith => throw _privateConstructorUsedError;
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
abstract class _$$VeilidConfigBlockStoreImplCopyWith<$Res>
    implements $VeilidConfigBlockStoreCopyWith<$Res> {
  factory _$$VeilidConfigBlockStoreImplCopyWith(
          _$VeilidConfigBlockStoreImpl value,
          $Res Function(_$VeilidConfigBlockStoreImpl) then) =
      __$$VeilidConfigBlockStoreImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String directory, bool delete});
}

/// @nodoc
class __$$VeilidConfigBlockStoreImplCopyWithImpl<$Res>
    extends _$VeilidConfigBlockStoreCopyWithImpl<$Res,
        _$VeilidConfigBlockStoreImpl>
    implements _$$VeilidConfigBlockStoreImplCopyWith<$Res> {
  __$$VeilidConfigBlockStoreImplCopyWithImpl(
      _$VeilidConfigBlockStoreImpl _value,
      $Res Function(_$VeilidConfigBlockStoreImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? directory = null,
    Object? delete = null,
  }) {
    return _then(_$VeilidConfigBlockStoreImpl(
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
class _$VeilidConfigBlockStoreImpl
    with DiagnosticableTreeMixin
    implements _VeilidConfigBlockStore {
  const _$VeilidConfigBlockStoreImpl(
      {required this.directory, required this.delete});

  factory _$VeilidConfigBlockStoreImpl.fromJson(Map<String, dynamic> json) =>
      _$$VeilidConfigBlockStoreImplFromJson(json);

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
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$VeilidConfigBlockStoreImpl &&
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
  _$$VeilidConfigBlockStoreImplCopyWith<_$VeilidConfigBlockStoreImpl>
      get copyWith => __$$VeilidConfigBlockStoreImplCopyWithImpl<
          _$VeilidConfigBlockStoreImpl>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$VeilidConfigBlockStoreImplToJson(
      this,
    );
  }
}

abstract class _VeilidConfigBlockStore implements VeilidConfigBlockStore {
  const factory _VeilidConfigBlockStore(
      {required final String directory,
      required final bool delete}) = _$VeilidConfigBlockStoreImpl;

  factory _VeilidConfigBlockStore.fromJson(Map<String, dynamic> json) =
      _$VeilidConfigBlockStoreImpl.fromJson;

  @override
  String get directory;
  @override
  bool get delete;
  @override
  @JsonKey(ignore: true)
  _$$VeilidConfigBlockStoreImplCopyWith<_$VeilidConfigBlockStoreImpl>
      get copyWith => throw _privateConstructorUsedError;
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
abstract class _$$VeilidConfigProtectedStoreImplCopyWith<$Res>
    implements $VeilidConfigProtectedStoreCopyWith<$Res> {
  factory _$$VeilidConfigProtectedStoreImplCopyWith(
          _$VeilidConfigProtectedStoreImpl value,
          $Res Function(_$VeilidConfigProtectedStoreImpl) then) =
      __$$VeilidConfigProtectedStoreImplCopyWithImpl<$Res>;
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
class __$$VeilidConfigProtectedStoreImplCopyWithImpl<$Res>
    extends _$VeilidConfigProtectedStoreCopyWithImpl<$Res,
        _$VeilidConfigProtectedStoreImpl>
    implements _$$VeilidConfigProtectedStoreImplCopyWith<$Res> {
  __$$VeilidConfigProtectedStoreImplCopyWithImpl(
      _$VeilidConfigProtectedStoreImpl _value,
      $Res Function(_$VeilidConfigProtectedStoreImpl) _then)
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
    return _then(_$VeilidConfigProtectedStoreImpl(
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
class _$VeilidConfigProtectedStoreImpl
    with DiagnosticableTreeMixin
    implements _VeilidConfigProtectedStore {
  const _$VeilidConfigProtectedStoreImpl(
      {required this.allowInsecureFallback,
      required this.alwaysUseInsecureStorage,
      required this.directory,
      required this.delete,
      required this.deviceEncryptionKeyPassword,
      this.newDeviceEncryptionKeyPassword});

  factory _$VeilidConfigProtectedStoreImpl.fromJson(
          Map<String, dynamic> json) =>
      _$$VeilidConfigProtectedStoreImplFromJson(json);

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
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$VeilidConfigProtectedStoreImpl &&
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
  _$$VeilidConfigProtectedStoreImplCopyWith<_$VeilidConfigProtectedStoreImpl>
      get copyWith => __$$VeilidConfigProtectedStoreImplCopyWithImpl<
          _$VeilidConfigProtectedStoreImpl>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$VeilidConfigProtectedStoreImplToJson(
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
      _$VeilidConfigProtectedStoreImpl;

  factory _VeilidConfigProtectedStore.fromJson(Map<String, dynamic> json) =
      _$VeilidConfigProtectedStoreImpl.fromJson;

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
  _$$VeilidConfigProtectedStoreImplCopyWith<_$VeilidConfigProtectedStoreImpl>
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
abstract class _$$VeilidConfigCapabilitiesImplCopyWith<$Res>
    implements $VeilidConfigCapabilitiesCopyWith<$Res> {
  factory _$$VeilidConfigCapabilitiesImplCopyWith(
          _$VeilidConfigCapabilitiesImpl value,
          $Res Function(_$VeilidConfigCapabilitiesImpl) then) =
      __$$VeilidConfigCapabilitiesImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({List<String> disable});
}

/// @nodoc
class __$$VeilidConfigCapabilitiesImplCopyWithImpl<$Res>
    extends _$VeilidConfigCapabilitiesCopyWithImpl<$Res,
        _$VeilidConfigCapabilitiesImpl>
    implements _$$VeilidConfigCapabilitiesImplCopyWith<$Res> {
  __$$VeilidConfigCapabilitiesImplCopyWithImpl(
      _$VeilidConfigCapabilitiesImpl _value,
      $Res Function(_$VeilidConfigCapabilitiesImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? disable = null,
  }) {
    return _then(_$VeilidConfigCapabilitiesImpl(
      disable: null == disable
          ? _value._disable
          : disable // ignore: cast_nullable_to_non_nullable
              as List<String>,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$VeilidConfigCapabilitiesImpl
    with DiagnosticableTreeMixin
    implements _VeilidConfigCapabilities {
  const _$VeilidConfigCapabilitiesImpl({required final List<String> disable})
      : _disable = disable;

  factory _$VeilidConfigCapabilitiesImpl.fromJson(Map<String, dynamic> json) =>
      _$$VeilidConfigCapabilitiesImplFromJson(json);

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
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$VeilidConfigCapabilitiesImpl &&
            const DeepCollectionEquality().equals(other._disable, _disable));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(_disable));

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$VeilidConfigCapabilitiesImplCopyWith<_$VeilidConfigCapabilitiesImpl>
      get copyWith => __$$VeilidConfigCapabilitiesImplCopyWithImpl<
          _$VeilidConfigCapabilitiesImpl>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$VeilidConfigCapabilitiesImplToJson(
      this,
    );
  }
}

abstract class _VeilidConfigCapabilities implements VeilidConfigCapabilities {
  const factory _VeilidConfigCapabilities(
      {required final List<String> disable}) = _$VeilidConfigCapabilitiesImpl;

  factory _VeilidConfigCapabilities.fromJson(Map<String, dynamic> json) =
      _$VeilidConfigCapabilitiesImpl.fromJson;

  @override
  List<String> get disable;
  @override
  @JsonKey(ignore: true)
  _$$VeilidConfigCapabilitiesImplCopyWith<_$VeilidConfigCapabilitiesImpl>
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
abstract class _$$VeilidConfigImplCopyWith<$Res>
    implements $VeilidConfigCopyWith<$Res> {
  factory _$$VeilidConfigImplCopyWith(
          _$VeilidConfigImpl value, $Res Function(_$VeilidConfigImpl) then) =
      __$$VeilidConfigImplCopyWithImpl<$Res>;
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
class __$$VeilidConfigImplCopyWithImpl<$Res>
    extends _$VeilidConfigCopyWithImpl<$Res, _$VeilidConfigImpl>
    implements _$$VeilidConfigImplCopyWith<$Res> {
  __$$VeilidConfigImplCopyWithImpl(
      _$VeilidConfigImpl _value, $Res Function(_$VeilidConfigImpl) _then)
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
    return _then(_$VeilidConfigImpl(
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
class _$VeilidConfigImpl with DiagnosticableTreeMixin implements _VeilidConfig {
  const _$VeilidConfigImpl(
      {required this.programName,
      required this.namespace,
      required this.capabilities,
      required this.protectedStore,
      required this.tableStore,
      required this.blockStore,
      required this.network});

  factory _$VeilidConfigImpl.fromJson(Map<String, dynamic> json) =>
      _$$VeilidConfigImplFromJson(json);

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
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$VeilidConfigImpl &&
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
  _$$VeilidConfigImplCopyWith<_$VeilidConfigImpl> get copyWith =>
      __$$VeilidConfigImplCopyWithImpl<_$VeilidConfigImpl>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$VeilidConfigImplToJson(
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
      required final VeilidConfigNetwork network}) = _$VeilidConfigImpl;

  factory _VeilidConfig.fromJson(Map<String, dynamic> json) =
      _$VeilidConfigImpl.fromJson;

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
  _$$VeilidConfigImplCopyWith<_$VeilidConfigImpl> get copyWith =>
      throw _privateConstructorUsedError;
}
