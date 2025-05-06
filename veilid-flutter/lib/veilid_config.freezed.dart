// dart format width=80
// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'veilid_config.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$VeilidFFIConfigLoggingTerminal implements DiagnosticableTreeMixin {
  bool get enabled;
  VeilidConfigLogLevel get level;
  List<String> get ignoreLogTargets;

  /// Create a copy of VeilidFFIConfigLoggingTerminal
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $VeilidFFIConfigLoggingTerminalCopyWith<VeilidFFIConfigLoggingTerminal>
      get copyWith => _$VeilidFFIConfigLoggingTerminalCopyWithImpl<
              VeilidFFIConfigLoggingTerminal>(
          this as VeilidFFIConfigLoggingTerminal, _$identity);

  /// Serializes this VeilidFFIConfigLoggingTerminal to a JSON map.
  Map<String, dynamic> toJson();

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
    properties
      ..add(DiagnosticsProperty('type', 'VeilidFFIConfigLoggingTerminal'))
      ..add(DiagnosticsProperty('enabled', enabled))
      ..add(DiagnosticsProperty('level', level))
      ..add(DiagnosticsProperty('ignoreLogTargets', ignoreLogTargets));
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is VeilidFFIConfigLoggingTerminal &&
            (identical(other.enabled, enabled) || other.enabled == enabled) &&
            (identical(other.level, level) || other.level == level) &&
            const DeepCollectionEquality()
                .equals(other.ignoreLogTargets, ignoreLogTargets));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, enabled, level,
      const DeepCollectionEquality().hash(ignoreLogTargets));

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidFFIConfigLoggingTerminal(enabled: $enabled, level: $level, ignoreLogTargets: $ignoreLogTargets)';
  }
}

/// @nodoc
abstract mixin class $VeilidFFIConfigLoggingTerminalCopyWith<$Res> {
  factory $VeilidFFIConfigLoggingTerminalCopyWith(
          VeilidFFIConfigLoggingTerminal value,
          $Res Function(VeilidFFIConfigLoggingTerminal) _then) =
      _$VeilidFFIConfigLoggingTerminalCopyWithImpl;
  @useResult
  $Res call(
      {bool enabled,
      VeilidConfigLogLevel level,
      List<String> ignoreLogTargets});
}

/// @nodoc
class _$VeilidFFIConfigLoggingTerminalCopyWithImpl<$Res>
    implements $VeilidFFIConfigLoggingTerminalCopyWith<$Res> {
  _$VeilidFFIConfigLoggingTerminalCopyWithImpl(this._self, this._then);

  final VeilidFFIConfigLoggingTerminal _self;
  final $Res Function(VeilidFFIConfigLoggingTerminal) _then;

  /// Create a copy of VeilidFFIConfigLoggingTerminal
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? enabled = null,
    Object? level = null,
    Object? ignoreLogTargets = null,
  }) {
    return _then(_self.copyWith(
      enabled: null == enabled
          ? _self.enabled
          : enabled // ignore: cast_nullable_to_non_nullable
              as bool,
      level: null == level
          ? _self.level
          : level // ignore: cast_nullable_to_non_nullable
              as VeilidConfigLogLevel,
      ignoreLogTargets: null == ignoreLogTargets
          ? _self.ignoreLogTargets
          : ignoreLogTargets // ignore: cast_nullable_to_non_nullable
              as List<String>,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _VeilidFFIConfigLoggingTerminal
    with DiagnosticableTreeMixin
    implements VeilidFFIConfigLoggingTerminal {
  const _VeilidFFIConfigLoggingTerminal(
      {required this.enabled,
      required this.level,
      final List<String> ignoreLogTargets = const []})
      : _ignoreLogTargets = ignoreLogTargets;
  factory _VeilidFFIConfigLoggingTerminal.fromJson(Map<String, dynamic> json) =>
      _$VeilidFFIConfigLoggingTerminalFromJson(json);

  @override
  final bool enabled;
  @override
  final VeilidConfigLogLevel level;
  final List<String> _ignoreLogTargets;
  @override
  @JsonKey()
  List<String> get ignoreLogTargets {
    if (_ignoreLogTargets is EqualUnmodifiableListView)
      return _ignoreLogTargets;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_ignoreLogTargets);
  }

  /// Create a copy of VeilidFFIConfigLoggingTerminal
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  _$VeilidFFIConfigLoggingTerminalCopyWith<_VeilidFFIConfigLoggingTerminal>
      get copyWith => __$VeilidFFIConfigLoggingTerminalCopyWithImpl<
          _VeilidFFIConfigLoggingTerminal>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$VeilidFFIConfigLoggingTerminalToJson(
      this,
    );
  }

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
    properties
      ..add(DiagnosticsProperty('type', 'VeilidFFIConfigLoggingTerminal'))
      ..add(DiagnosticsProperty('enabled', enabled))
      ..add(DiagnosticsProperty('level', level))
      ..add(DiagnosticsProperty('ignoreLogTargets', ignoreLogTargets));
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _VeilidFFIConfigLoggingTerminal &&
            (identical(other.enabled, enabled) || other.enabled == enabled) &&
            (identical(other.level, level) || other.level == level) &&
            const DeepCollectionEquality()
                .equals(other._ignoreLogTargets, _ignoreLogTargets));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, enabled, level,
      const DeepCollectionEquality().hash(_ignoreLogTargets));

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidFFIConfigLoggingTerminal(enabled: $enabled, level: $level, ignoreLogTargets: $ignoreLogTargets)';
  }
}

/// @nodoc
abstract mixin class _$VeilidFFIConfigLoggingTerminalCopyWith<$Res>
    implements $VeilidFFIConfigLoggingTerminalCopyWith<$Res> {
  factory _$VeilidFFIConfigLoggingTerminalCopyWith(
          _VeilidFFIConfigLoggingTerminal value,
          $Res Function(_VeilidFFIConfigLoggingTerminal) _then) =
      __$VeilidFFIConfigLoggingTerminalCopyWithImpl;
  @override
  @useResult
  $Res call(
      {bool enabled,
      VeilidConfigLogLevel level,
      List<String> ignoreLogTargets});
}

/// @nodoc
class __$VeilidFFIConfigLoggingTerminalCopyWithImpl<$Res>
    implements _$VeilidFFIConfigLoggingTerminalCopyWith<$Res> {
  __$VeilidFFIConfigLoggingTerminalCopyWithImpl(this._self, this._then);

  final _VeilidFFIConfigLoggingTerminal _self;
  final $Res Function(_VeilidFFIConfigLoggingTerminal) _then;

  /// Create a copy of VeilidFFIConfigLoggingTerminal
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? enabled = null,
    Object? level = null,
    Object? ignoreLogTargets = null,
  }) {
    return _then(_VeilidFFIConfigLoggingTerminal(
      enabled: null == enabled
          ? _self.enabled
          : enabled // ignore: cast_nullable_to_non_nullable
              as bool,
      level: null == level
          ? _self.level
          : level // ignore: cast_nullable_to_non_nullable
              as VeilidConfigLogLevel,
      ignoreLogTargets: null == ignoreLogTargets
          ? _self._ignoreLogTargets
          : ignoreLogTargets // ignore: cast_nullable_to_non_nullable
              as List<String>,
    ));
  }
}

/// @nodoc
mixin _$VeilidFFIConfigLoggingOtlp implements DiagnosticableTreeMixin {
  bool get enabled;
  VeilidConfigLogLevel get level;
  String get grpcEndpoint;
  String get serviceName;
  List<String> get ignoreLogTargets;

  /// Create a copy of VeilidFFIConfigLoggingOtlp
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $VeilidFFIConfigLoggingOtlpCopyWith<VeilidFFIConfigLoggingOtlp>
      get copyWith =>
          _$VeilidFFIConfigLoggingOtlpCopyWithImpl<VeilidFFIConfigLoggingOtlp>(
              this as VeilidFFIConfigLoggingOtlp, _$identity);

  /// Serializes this VeilidFFIConfigLoggingOtlp to a JSON map.
  Map<String, dynamic> toJson();

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
    properties
      ..add(DiagnosticsProperty('type', 'VeilidFFIConfigLoggingOtlp'))
      ..add(DiagnosticsProperty('enabled', enabled))
      ..add(DiagnosticsProperty('level', level))
      ..add(DiagnosticsProperty('grpcEndpoint', grpcEndpoint))
      ..add(DiagnosticsProperty('serviceName', serviceName))
      ..add(DiagnosticsProperty('ignoreLogTargets', ignoreLogTargets));
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is VeilidFFIConfigLoggingOtlp &&
            (identical(other.enabled, enabled) || other.enabled == enabled) &&
            (identical(other.level, level) || other.level == level) &&
            (identical(other.grpcEndpoint, grpcEndpoint) ||
                other.grpcEndpoint == grpcEndpoint) &&
            (identical(other.serviceName, serviceName) ||
                other.serviceName == serviceName) &&
            const DeepCollectionEquality()
                .equals(other.ignoreLogTargets, ignoreLogTargets));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, enabled, level, grpcEndpoint,
      serviceName, const DeepCollectionEquality().hash(ignoreLogTargets));

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidFFIConfigLoggingOtlp(enabled: $enabled, level: $level, grpcEndpoint: $grpcEndpoint, serviceName: $serviceName, ignoreLogTargets: $ignoreLogTargets)';
  }
}

/// @nodoc
abstract mixin class $VeilidFFIConfigLoggingOtlpCopyWith<$Res> {
  factory $VeilidFFIConfigLoggingOtlpCopyWith(VeilidFFIConfigLoggingOtlp value,
          $Res Function(VeilidFFIConfigLoggingOtlp) _then) =
      _$VeilidFFIConfigLoggingOtlpCopyWithImpl;
  @useResult
  $Res call(
      {bool enabled,
      VeilidConfigLogLevel level,
      String grpcEndpoint,
      String serviceName,
      List<String> ignoreLogTargets});
}

/// @nodoc
class _$VeilidFFIConfigLoggingOtlpCopyWithImpl<$Res>
    implements $VeilidFFIConfigLoggingOtlpCopyWith<$Res> {
  _$VeilidFFIConfigLoggingOtlpCopyWithImpl(this._self, this._then);

  final VeilidFFIConfigLoggingOtlp _self;
  final $Res Function(VeilidFFIConfigLoggingOtlp) _then;

  /// Create a copy of VeilidFFIConfigLoggingOtlp
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? enabled = null,
    Object? level = null,
    Object? grpcEndpoint = null,
    Object? serviceName = null,
    Object? ignoreLogTargets = null,
  }) {
    return _then(_self.copyWith(
      enabled: null == enabled
          ? _self.enabled
          : enabled // ignore: cast_nullable_to_non_nullable
              as bool,
      level: null == level
          ? _self.level
          : level // ignore: cast_nullable_to_non_nullable
              as VeilidConfigLogLevel,
      grpcEndpoint: null == grpcEndpoint
          ? _self.grpcEndpoint
          : grpcEndpoint // ignore: cast_nullable_to_non_nullable
              as String,
      serviceName: null == serviceName
          ? _self.serviceName
          : serviceName // ignore: cast_nullable_to_non_nullable
              as String,
      ignoreLogTargets: null == ignoreLogTargets
          ? _self.ignoreLogTargets
          : ignoreLogTargets // ignore: cast_nullable_to_non_nullable
              as List<String>,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _VeilidFFIConfigLoggingOtlp
    with DiagnosticableTreeMixin
    implements VeilidFFIConfigLoggingOtlp {
  const _VeilidFFIConfigLoggingOtlp(
      {required this.enabled,
      required this.level,
      required this.grpcEndpoint,
      required this.serviceName,
      final List<String> ignoreLogTargets = const []})
      : _ignoreLogTargets = ignoreLogTargets;
  factory _VeilidFFIConfigLoggingOtlp.fromJson(Map<String, dynamic> json) =>
      _$VeilidFFIConfigLoggingOtlpFromJson(json);

  @override
  final bool enabled;
  @override
  final VeilidConfigLogLevel level;
  @override
  final String grpcEndpoint;
  @override
  final String serviceName;
  final List<String> _ignoreLogTargets;
  @override
  @JsonKey()
  List<String> get ignoreLogTargets {
    if (_ignoreLogTargets is EqualUnmodifiableListView)
      return _ignoreLogTargets;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_ignoreLogTargets);
  }

  /// Create a copy of VeilidFFIConfigLoggingOtlp
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  _$VeilidFFIConfigLoggingOtlpCopyWith<_VeilidFFIConfigLoggingOtlp>
      get copyWith => __$VeilidFFIConfigLoggingOtlpCopyWithImpl<
          _VeilidFFIConfigLoggingOtlp>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$VeilidFFIConfigLoggingOtlpToJson(
      this,
    );
  }

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
    properties
      ..add(DiagnosticsProperty('type', 'VeilidFFIConfigLoggingOtlp'))
      ..add(DiagnosticsProperty('enabled', enabled))
      ..add(DiagnosticsProperty('level', level))
      ..add(DiagnosticsProperty('grpcEndpoint', grpcEndpoint))
      ..add(DiagnosticsProperty('serviceName', serviceName))
      ..add(DiagnosticsProperty('ignoreLogTargets', ignoreLogTargets));
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _VeilidFFIConfigLoggingOtlp &&
            (identical(other.enabled, enabled) || other.enabled == enabled) &&
            (identical(other.level, level) || other.level == level) &&
            (identical(other.grpcEndpoint, grpcEndpoint) ||
                other.grpcEndpoint == grpcEndpoint) &&
            (identical(other.serviceName, serviceName) ||
                other.serviceName == serviceName) &&
            const DeepCollectionEquality()
                .equals(other._ignoreLogTargets, _ignoreLogTargets));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, enabled, level, grpcEndpoint,
      serviceName, const DeepCollectionEquality().hash(_ignoreLogTargets));

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidFFIConfigLoggingOtlp(enabled: $enabled, level: $level, grpcEndpoint: $grpcEndpoint, serviceName: $serviceName, ignoreLogTargets: $ignoreLogTargets)';
  }
}

/// @nodoc
abstract mixin class _$VeilidFFIConfigLoggingOtlpCopyWith<$Res>
    implements $VeilidFFIConfigLoggingOtlpCopyWith<$Res> {
  factory _$VeilidFFIConfigLoggingOtlpCopyWith(
          _VeilidFFIConfigLoggingOtlp value,
          $Res Function(_VeilidFFIConfigLoggingOtlp) _then) =
      __$VeilidFFIConfigLoggingOtlpCopyWithImpl;
  @override
  @useResult
  $Res call(
      {bool enabled,
      VeilidConfigLogLevel level,
      String grpcEndpoint,
      String serviceName,
      List<String> ignoreLogTargets});
}

/// @nodoc
class __$VeilidFFIConfigLoggingOtlpCopyWithImpl<$Res>
    implements _$VeilidFFIConfigLoggingOtlpCopyWith<$Res> {
  __$VeilidFFIConfigLoggingOtlpCopyWithImpl(this._self, this._then);

  final _VeilidFFIConfigLoggingOtlp _self;
  final $Res Function(_VeilidFFIConfigLoggingOtlp) _then;

  /// Create a copy of VeilidFFIConfigLoggingOtlp
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? enabled = null,
    Object? level = null,
    Object? grpcEndpoint = null,
    Object? serviceName = null,
    Object? ignoreLogTargets = null,
  }) {
    return _then(_VeilidFFIConfigLoggingOtlp(
      enabled: null == enabled
          ? _self.enabled
          : enabled // ignore: cast_nullable_to_non_nullable
              as bool,
      level: null == level
          ? _self.level
          : level // ignore: cast_nullable_to_non_nullable
              as VeilidConfigLogLevel,
      grpcEndpoint: null == grpcEndpoint
          ? _self.grpcEndpoint
          : grpcEndpoint // ignore: cast_nullable_to_non_nullable
              as String,
      serviceName: null == serviceName
          ? _self.serviceName
          : serviceName // ignore: cast_nullable_to_non_nullable
              as String,
      ignoreLogTargets: null == ignoreLogTargets
          ? _self._ignoreLogTargets
          : ignoreLogTargets // ignore: cast_nullable_to_non_nullable
              as List<String>,
    ));
  }
}

/// @nodoc
mixin _$VeilidFFIConfigLoggingApi implements DiagnosticableTreeMixin {
  bool get enabled;
  VeilidConfigLogLevel get level;
  List<String> get ignoreLogTargets;

  /// Create a copy of VeilidFFIConfigLoggingApi
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $VeilidFFIConfigLoggingApiCopyWith<VeilidFFIConfigLoggingApi> get copyWith =>
      _$VeilidFFIConfigLoggingApiCopyWithImpl<VeilidFFIConfigLoggingApi>(
          this as VeilidFFIConfigLoggingApi, _$identity);

  /// Serializes this VeilidFFIConfigLoggingApi to a JSON map.
  Map<String, dynamic> toJson();

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
    properties
      ..add(DiagnosticsProperty('type', 'VeilidFFIConfigLoggingApi'))
      ..add(DiagnosticsProperty('enabled', enabled))
      ..add(DiagnosticsProperty('level', level))
      ..add(DiagnosticsProperty('ignoreLogTargets', ignoreLogTargets));
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is VeilidFFIConfigLoggingApi &&
            (identical(other.enabled, enabled) || other.enabled == enabled) &&
            (identical(other.level, level) || other.level == level) &&
            const DeepCollectionEquality()
                .equals(other.ignoreLogTargets, ignoreLogTargets));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, enabled, level,
      const DeepCollectionEquality().hash(ignoreLogTargets));

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidFFIConfigLoggingApi(enabled: $enabled, level: $level, ignoreLogTargets: $ignoreLogTargets)';
  }
}

/// @nodoc
abstract mixin class $VeilidFFIConfigLoggingApiCopyWith<$Res> {
  factory $VeilidFFIConfigLoggingApiCopyWith(VeilidFFIConfigLoggingApi value,
          $Res Function(VeilidFFIConfigLoggingApi) _then) =
      _$VeilidFFIConfigLoggingApiCopyWithImpl;
  @useResult
  $Res call(
      {bool enabled,
      VeilidConfigLogLevel level,
      List<String> ignoreLogTargets});
}

/// @nodoc
class _$VeilidFFIConfigLoggingApiCopyWithImpl<$Res>
    implements $VeilidFFIConfigLoggingApiCopyWith<$Res> {
  _$VeilidFFIConfigLoggingApiCopyWithImpl(this._self, this._then);

  final VeilidFFIConfigLoggingApi _self;
  final $Res Function(VeilidFFIConfigLoggingApi) _then;

  /// Create a copy of VeilidFFIConfigLoggingApi
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? enabled = null,
    Object? level = null,
    Object? ignoreLogTargets = null,
  }) {
    return _then(_self.copyWith(
      enabled: null == enabled
          ? _self.enabled
          : enabled // ignore: cast_nullable_to_non_nullable
              as bool,
      level: null == level
          ? _self.level
          : level // ignore: cast_nullable_to_non_nullable
              as VeilidConfigLogLevel,
      ignoreLogTargets: null == ignoreLogTargets
          ? _self.ignoreLogTargets
          : ignoreLogTargets // ignore: cast_nullable_to_non_nullable
              as List<String>,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _VeilidFFIConfigLoggingApi
    with DiagnosticableTreeMixin
    implements VeilidFFIConfigLoggingApi {
  const _VeilidFFIConfigLoggingApi(
      {required this.enabled,
      required this.level,
      final List<String> ignoreLogTargets = const []})
      : _ignoreLogTargets = ignoreLogTargets;
  factory _VeilidFFIConfigLoggingApi.fromJson(Map<String, dynamic> json) =>
      _$VeilidFFIConfigLoggingApiFromJson(json);

  @override
  final bool enabled;
  @override
  final VeilidConfigLogLevel level;
  final List<String> _ignoreLogTargets;
  @override
  @JsonKey()
  List<String> get ignoreLogTargets {
    if (_ignoreLogTargets is EqualUnmodifiableListView)
      return _ignoreLogTargets;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_ignoreLogTargets);
  }

  /// Create a copy of VeilidFFIConfigLoggingApi
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  _$VeilidFFIConfigLoggingApiCopyWith<_VeilidFFIConfigLoggingApi>
      get copyWith =>
          __$VeilidFFIConfigLoggingApiCopyWithImpl<_VeilidFFIConfigLoggingApi>(
              this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$VeilidFFIConfigLoggingApiToJson(
      this,
    );
  }

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
    properties
      ..add(DiagnosticsProperty('type', 'VeilidFFIConfigLoggingApi'))
      ..add(DiagnosticsProperty('enabled', enabled))
      ..add(DiagnosticsProperty('level', level))
      ..add(DiagnosticsProperty('ignoreLogTargets', ignoreLogTargets));
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _VeilidFFIConfigLoggingApi &&
            (identical(other.enabled, enabled) || other.enabled == enabled) &&
            (identical(other.level, level) || other.level == level) &&
            const DeepCollectionEquality()
                .equals(other._ignoreLogTargets, _ignoreLogTargets));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, enabled, level,
      const DeepCollectionEquality().hash(_ignoreLogTargets));

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidFFIConfigLoggingApi(enabled: $enabled, level: $level, ignoreLogTargets: $ignoreLogTargets)';
  }
}

/// @nodoc
abstract mixin class _$VeilidFFIConfigLoggingApiCopyWith<$Res>
    implements $VeilidFFIConfigLoggingApiCopyWith<$Res> {
  factory _$VeilidFFIConfigLoggingApiCopyWith(_VeilidFFIConfigLoggingApi value,
          $Res Function(_VeilidFFIConfigLoggingApi) _then) =
      __$VeilidFFIConfigLoggingApiCopyWithImpl;
  @override
  @useResult
  $Res call(
      {bool enabled,
      VeilidConfigLogLevel level,
      List<String> ignoreLogTargets});
}

/// @nodoc
class __$VeilidFFIConfigLoggingApiCopyWithImpl<$Res>
    implements _$VeilidFFIConfigLoggingApiCopyWith<$Res> {
  __$VeilidFFIConfigLoggingApiCopyWithImpl(this._self, this._then);

  final _VeilidFFIConfigLoggingApi _self;
  final $Res Function(_VeilidFFIConfigLoggingApi) _then;

  /// Create a copy of VeilidFFIConfigLoggingApi
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? enabled = null,
    Object? level = null,
    Object? ignoreLogTargets = null,
  }) {
    return _then(_VeilidFFIConfigLoggingApi(
      enabled: null == enabled
          ? _self.enabled
          : enabled // ignore: cast_nullable_to_non_nullable
              as bool,
      level: null == level
          ? _self.level
          : level // ignore: cast_nullable_to_non_nullable
              as VeilidConfigLogLevel,
      ignoreLogTargets: null == ignoreLogTargets
          ? _self._ignoreLogTargets
          : ignoreLogTargets // ignore: cast_nullable_to_non_nullable
              as List<String>,
    ));
  }
}

/// @nodoc
mixin _$VeilidFFIConfigLoggingFlame implements DiagnosticableTreeMixin {
  bool get enabled;
  String get path;

  /// Create a copy of VeilidFFIConfigLoggingFlame
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $VeilidFFIConfigLoggingFlameCopyWith<VeilidFFIConfigLoggingFlame>
      get copyWith => _$VeilidFFIConfigLoggingFlameCopyWithImpl<
              VeilidFFIConfigLoggingFlame>(
          this as VeilidFFIConfigLoggingFlame, _$identity);

  /// Serializes this VeilidFFIConfigLoggingFlame to a JSON map.
  Map<String, dynamic> toJson();

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
    properties
      ..add(DiagnosticsProperty('type', 'VeilidFFIConfigLoggingFlame'))
      ..add(DiagnosticsProperty('enabled', enabled))
      ..add(DiagnosticsProperty('path', path));
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is VeilidFFIConfigLoggingFlame &&
            (identical(other.enabled, enabled) || other.enabled == enabled) &&
            (identical(other.path, path) || other.path == path));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, enabled, path);

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidFFIConfigLoggingFlame(enabled: $enabled, path: $path)';
  }
}

/// @nodoc
abstract mixin class $VeilidFFIConfigLoggingFlameCopyWith<$Res> {
  factory $VeilidFFIConfigLoggingFlameCopyWith(
          VeilidFFIConfigLoggingFlame value,
          $Res Function(VeilidFFIConfigLoggingFlame) _then) =
      _$VeilidFFIConfigLoggingFlameCopyWithImpl;
  @useResult
  $Res call({bool enabled, String path});
}

/// @nodoc
class _$VeilidFFIConfigLoggingFlameCopyWithImpl<$Res>
    implements $VeilidFFIConfigLoggingFlameCopyWith<$Res> {
  _$VeilidFFIConfigLoggingFlameCopyWithImpl(this._self, this._then);

  final VeilidFFIConfigLoggingFlame _self;
  final $Res Function(VeilidFFIConfigLoggingFlame) _then;

  /// Create a copy of VeilidFFIConfigLoggingFlame
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? enabled = null,
    Object? path = null,
  }) {
    return _then(_self.copyWith(
      enabled: null == enabled
          ? _self.enabled
          : enabled // ignore: cast_nullable_to_non_nullable
              as bool,
      path: null == path
          ? _self.path
          : path // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _VeilidFFIConfigLoggingFlame
    with DiagnosticableTreeMixin
    implements VeilidFFIConfigLoggingFlame {
  const _VeilidFFIConfigLoggingFlame(
      {required this.enabled, required this.path});
  factory _VeilidFFIConfigLoggingFlame.fromJson(Map<String, dynamic> json) =>
      _$VeilidFFIConfigLoggingFlameFromJson(json);

  @override
  final bool enabled;
  @override
  final String path;

  /// Create a copy of VeilidFFIConfigLoggingFlame
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  _$VeilidFFIConfigLoggingFlameCopyWith<_VeilidFFIConfigLoggingFlame>
      get copyWith => __$VeilidFFIConfigLoggingFlameCopyWithImpl<
          _VeilidFFIConfigLoggingFlame>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$VeilidFFIConfigLoggingFlameToJson(
      this,
    );
  }

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
    properties
      ..add(DiagnosticsProperty('type', 'VeilidFFIConfigLoggingFlame'))
      ..add(DiagnosticsProperty('enabled', enabled))
      ..add(DiagnosticsProperty('path', path));
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _VeilidFFIConfigLoggingFlame &&
            (identical(other.enabled, enabled) || other.enabled == enabled) &&
            (identical(other.path, path) || other.path == path));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, enabled, path);

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidFFIConfigLoggingFlame(enabled: $enabled, path: $path)';
  }
}

/// @nodoc
abstract mixin class _$VeilidFFIConfigLoggingFlameCopyWith<$Res>
    implements $VeilidFFIConfigLoggingFlameCopyWith<$Res> {
  factory _$VeilidFFIConfigLoggingFlameCopyWith(
          _VeilidFFIConfigLoggingFlame value,
          $Res Function(_VeilidFFIConfigLoggingFlame) _then) =
      __$VeilidFFIConfigLoggingFlameCopyWithImpl;
  @override
  @useResult
  $Res call({bool enabled, String path});
}

/// @nodoc
class __$VeilidFFIConfigLoggingFlameCopyWithImpl<$Res>
    implements _$VeilidFFIConfigLoggingFlameCopyWith<$Res> {
  __$VeilidFFIConfigLoggingFlameCopyWithImpl(this._self, this._then);

  final _VeilidFFIConfigLoggingFlame _self;
  final $Res Function(_VeilidFFIConfigLoggingFlame) _then;

  /// Create a copy of VeilidFFIConfigLoggingFlame
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? enabled = null,
    Object? path = null,
  }) {
    return _then(_VeilidFFIConfigLoggingFlame(
      enabled: null == enabled
          ? _self.enabled
          : enabled // ignore: cast_nullable_to_non_nullable
              as bool,
      path: null == path
          ? _self.path
          : path // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc
mixin _$VeilidFFIConfigLogging implements DiagnosticableTreeMixin {
  VeilidFFIConfigLoggingTerminal get terminal;
  VeilidFFIConfigLoggingOtlp get otlp;
  VeilidFFIConfigLoggingApi get api;
  VeilidFFIConfigLoggingFlame get flame;

  /// Create a copy of VeilidFFIConfigLogging
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $VeilidFFIConfigLoggingCopyWith<VeilidFFIConfigLogging> get copyWith =>
      _$VeilidFFIConfigLoggingCopyWithImpl<VeilidFFIConfigLogging>(
          this as VeilidFFIConfigLogging, _$identity);

  /// Serializes this VeilidFFIConfigLogging to a JSON map.
  Map<String, dynamic> toJson();

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
    properties
      ..add(DiagnosticsProperty('type', 'VeilidFFIConfigLogging'))
      ..add(DiagnosticsProperty('terminal', terminal))
      ..add(DiagnosticsProperty('otlp', otlp))
      ..add(DiagnosticsProperty('api', api))
      ..add(DiagnosticsProperty('flame', flame));
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is VeilidFFIConfigLogging &&
            (identical(other.terminal, terminal) ||
                other.terminal == terminal) &&
            (identical(other.otlp, otlp) || other.otlp == otlp) &&
            (identical(other.api, api) || other.api == api) &&
            (identical(other.flame, flame) || other.flame == flame));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, terminal, otlp, api, flame);

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidFFIConfigLogging(terminal: $terminal, otlp: $otlp, api: $api, flame: $flame)';
  }
}

/// @nodoc
abstract mixin class $VeilidFFIConfigLoggingCopyWith<$Res> {
  factory $VeilidFFIConfigLoggingCopyWith(VeilidFFIConfigLogging value,
          $Res Function(VeilidFFIConfigLogging) _then) =
      _$VeilidFFIConfigLoggingCopyWithImpl;
  @useResult
  $Res call(
      {VeilidFFIConfigLoggingTerminal terminal,
      VeilidFFIConfigLoggingOtlp otlp,
      VeilidFFIConfigLoggingApi api,
      VeilidFFIConfigLoggingFlame flame});

  $VeilidFFIConfigLoggingTerminalCopyWith<$Res> get terminal;
  $VeilidFFIConfigLoggingOtlpCopyWith<$Res> get otlp;
  $VeilidFFIConfigLoggingApiCopyWith<$Res> get api;
  $VeilidFFIConfigLoggingFlameCopyWith<$Res> get flame;
}

/// @nodoc
class _$VeilidFFIConfigLoggingCopyWithImpl<$Res>
    implements $VeilidFFIConfigLoggingCopyWith<$Res> {
  _$VeilidFFIConfigLoggingCopyWithImpl(this._self, this._then);

  final VeilidFFIConfigLogging _self;
  final $Res Function(VeilidFFIConfigLogging) _then;

  /// Create a copy of VeilidFFIConfigLogging
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? terminal = null,
    Object? otlp = null,
    Object? api = null,
    Object? flame = null,
  }) {
    return _then(_self.copyWith(
      terminal: null == terminal
          ? _self.terminal
          : terminal // ignore: cast_nullable_to_non_nullable
              as VeilidFFIConfigLoggingTerminal,
      otlp: null == otlp
          ? _self.otlp
          : otlp // ignore: cast_nullable_to_non_nullable
              as VeilidFFIConfigLoggingOtlp,
      api: null == api
          ? _self.api
          : api // ignore: cast_nullable_to_non_nullable
              as VeilidFFIConfigLoggingApi,
      flame: null == flame
          ? _self.flame
          : flame // ignore: cast_nullable_to_non_nullable
              as VeilidFFIConfigLoggingFlame,
    ));
  }

  /// Create a copy of VeilidFFIConfigLogging
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $VeilidFFIConfigLoggingTerminalCopyWith<$Res> get terminal {
    return $VeilidFFIConfigLoggingTerminalCopyWith<$Res>(_self.terminal,
        (value) {
      return _then(_self.copyWith(terminal: value));
    });
  }

  /// Create a copy of VeilidFFIConfigLogging
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $VeilidFFIConfigLoggingOtlpCopyWith<$Res> get otlp {
    return $VeilidFFIConfigLoggingOtlpCopyWith<$Res>(_self.otlp, (value) {
      return _then(_self.copyWith(otlp: value));
    });
  }

  /// Create a copy of VeilidFFIConfigLogging
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $VeilidFFIConfigLoggingApiCopyWith<$Res> get api {
    return $VeilidFFIConfigLoggingApiCopyWith<$Res>(_self.api, (value) {
      return _then(_self.copyWith(api: value));
    });
  }

  /// Create a copy of VeilidFFIConfigLogging
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $VeilidFFIConfigLoggingFlameCopyWith<$Res> get flame {
    return $VeilidFFIConfigLoggingFlameCopyWith<$Res>(_self.flame, (value) {
      return _then(_self.copyWith(flame: value));
    });
  }
}

/// @nodoc
@JsonSerializable()
class _VeilidFFIConfigLogging
    with DiagnosticableTreeMixin
    implements VeilidFFIConfigLogging {
  const _VeilidFFIConfigLogging(
      {required this.terminal,
      required this.otlp,
      required this.api,
      required this.flame});
  factory _VeilidFFIConfigLogging.fromJson(Map<String, dynamic> json) =>
      _$VeilidFFIConfigLoggingFromJson(json);

  @override
  final VeilidFFIConfigLoggingTerminal terminal;
  @override
  final VeilidFFIConfigLoggingOtlp otlp;
  @override
  final VeilidFFIConfigLoggingApi api;
  @override
  final VeilidFFIConfigLoggingFlame flame;

  /// Create a copy of VeilidFFIConfigLogging
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  _$VeilidFFIConfigLoggingCopyWith<_VeilidFFIConfigLogging> get copyWith =>
      __$VeilidFFIConfigLoggingCopyWithImpl<_VeilidFFIConfigLogging>(
          this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$VeilidFFIConfigLoggingToJson(
      this,
    );
  }

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
    properties
      ..add(DiagnosticsProperty('type', 'VeilidFFIConfigLogging'))
      ..add(DiagnosticsProperty('terminal', terminal))
      ..add(DiagnosticsProperty('otlp', otlp))
      ..add(DiagnosticsProperty('api', api))
      ..add(DiagnosticsProperty('flame', flame));
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _VeilidFFIConfigLogging &&
            (identical(other.terminal, terminal) ||
                other.terminal == terminal) &&
            (identical(other.otlp, otlp) || other.otlp == otlp) &&
            (identical(other.api, api) || other.api == api) &&
            (identical(other.flame, flame) || other.flame == flame));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, terminal, otlp, api, flame);

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidFFIConfigLogging(terminal: $terminal, otlp: $otlp, api: $api, flame: $flame)';
  }
}

/// @nodoc
abstract mixin class _$VeilidFFIConfigLoggingCopyWith<$Res>
    implements $VeilidFFIConfigLoggingCopyWith<$Res> {
  factory _$VeilidFFIConfigLoggingCopyWith(_VeilidFFIConfigLogging value,
          $Res Function(_VeilidFFIConfigLogging) _then) =
      __$VeilidFFIConfigLoggingCopyWithImpl;
  @override
  @useResult
  $Res call(
      {VeilidFFIConfigLoggingTerminal terminal,
      VeilidFFIConfigLoggingOtlp otlp,
      VeilidFFIConfigLoggingApi api,
      VeilidFFIConfigLoggingFlame flame});

  @override
  $VeilidFFIConfigLoggingTerminalCopyWith<$Res> get terminal;
  @override
  $VeilidFFIConfigLoggingOtlpCopyWith<$Res> get otlp;
  @override
  $VeilidFFIConfigLoggingApiCopyWith<$Res> get api;
  @override
  $VeilidFFIConfigLoggingFlameCopyWith<$Res> get flame;
}

/// @nodoc
class __$VeilidFFIConfigLoggingCopyWithImpl<$Res>
    implements _$VeilidFFIConfigLoggingCopyWith<$Res> {
  __$VeilidFFIConfigLoggingCopyWithImpl(this._self, this._then);

  final _VeilidFFIConfigLogging _self;
  final $Res Function(_VeilidFFIConfigLogging) _then;

  /// Create a copy of VeilidFFIConfigLogging
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? terminal = null,
    Object? otlp = null,
    Object? api = null,
    Object? flame = null,
  }) {
    return _then(_VeilidFFIConfigLogging(
      terminal: null == terminal
          ? _self.terminal
          : terminal // ignore: cast_nullable_to_non_nullable
              as VeilidFFIConfigLoggingTerminal,
      otlp: null == otlp
          ? _self.otlp
          : otlp // ignore: cast_nullable_to_non_nullable
              as VeilidFFIConfigLoggingOtlp,
      api: null == api
          ? _self.api
          : api // ignore: cast_nullable_to_non_nullable
              as VeilidFFIConfigLoggingApi,
      flame: null == flame
          ? _self.flame
          : flame // ignore: cast_nullable_to_non_nullable
              as VeilidFFIConfigLoggingFlame,
    ));
  }

  /// Create a copy of VeilidFFIConfigLogging
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $VeilidFFIConfigLoggingTerminalCopyWith<$Res> get terminal {
    return $VeilidFFIConfigLoggingTerminalCopyWith<$Res>(_self.terminal,
        (value) {
      return _then(_self.copyWith(terminal: value));
    });
  }

  /// Create a copy of VeilidFFIConfigLogging
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $VeilidFFIConfigLoggingOtlpCopyWith<$Res> get otlp {
    return $VeilidFFIConfigLoggingOtlpCopyWith<$Res>(_self.otlp, (value) {
      return _then(_self.copyWith(otlp: value));
    });
  }

  /// Create a copy of VeilidFFIConfigLogging
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $VeilidFFIConfigLoggingApiCopyWith<$Res> get api {
    return $VeilidFFIConfigLoggingApiCopyWith<$Res>(_self.api, (value) {
      return _then(_self.copyWith(api: value));
    });
  }

  /// Create a copy of VeilidFFIConfigLogging
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $VeilidFFIConfigLoggingFlameCopyWith<$Res> get flame {
    return $VeilidFFIConfigLoggingFlameCopyWith<$Res>(_self.flame, (value) {
      return _then(_self.copyWith(flame: value));
    });
  }
}

/// @nodoc
mixin _$VeilidFFIConfig implements DiagnosticableTreeMixin {
  VeilidFFIConfigLogging get logging;

  /// Create a copy of VeilidFFIConfig
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $VeilidFFIConfigCopyWith<VeilidFFIConfig> get copyWith =>
      _$VeilidFFIConfigCopyWithImpl<VeilidFFIConfig>(
          this as VeilidFFIConfig, _$identity);

  /// Serializes this VeilidFFIConfig to a JSON map.
  Map<String, dynamic> toJson();

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
    properties
      ..add(DiagnosticsProperty('type', 'VeilidFFIConfig'))
      ..add(DiagnosticsProperty('logging', logging));
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is VeilidFFIConfig &&
            (identical(other.logging, logging) || other.logging == logging));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, logging);

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidFFIConfig(logging: $logging)';
  }
}

/// @nodoc
abstract mixin class $VeilidFFIConfigCopyWith<$Res> {
  factory $VeilidFFIConfigCopyWith(
          VeilidFFIConfig value, $Res Function(VeilidFFIConfig) _then) =
      _$VeilidFFIConfigCopyWithImpl;
  @useResult
  $Res call({VeilidFFIConfigLogging logging});

  $VeilidFFIConfigLoggingCopyWith<$Res> get logging;
}

/// @nodoc
class _$VeilidFFIConfigCopyWithImpl<$Res>
    implements $VeilidFFIConfigCopyWith<$Res> {
  _$VeilidFFIConfigCopyWithImpl(this._self, this._then);

  final VeilidFFIConfig _self;
  final $Res Function(VeilidFFIConfig) _then;

  /// Create a copy of VeilidFFIConfig
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? logging = null,
  }) {
    return _then(_self.copyWith(
      logging: null == logging
          ? _self.logging
          : logging // ignore: cast_nullable_to_non_nullable
              as VeilidFFIConfigLogging,
    ));
  }

  /// Create a copy of VeilidFFIConfig
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $VeilidFFIConfigLoggingCopyWith<$Res> get logging {
    return $VeilidFFIConfigLoggingCopyWith<$Res>(_self.logging, (value) {
      return _then(_self.copyWith(logging: value));
    });
  }
}

/// @nodoc
@JsonSerializable()
class _VeilidFFIConfig with DiagnosticableTreeMixin implements VeilidFFIConfig {
  const _VeilidFFIConfig({required this.logging});
  factory _VeilidFFIConfig.fromJson(Map<String, dynamic> json) =>
      _$VeilidFFIConfigFromJson(json);

  @override
  final VeilidFFIConfigLogging logging;

  /// Create a copy of VeilidFFIConfig
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  _$VeilidFFIConfigCopyWith<_VeilidFFIConfig> get copyWith =>
      __$VeilidFFIConfigCopyWithImpl<_VeilidFFIConfig>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$VeilidFFIConfigToJson(
      this,
    );
  }

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
    properties
      ..add(DiagnosticsProperty('type', 'VeilidFFIConfig'))
      ..add(DiagnosticsProperty('logging', logging));
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _VeilidFFIConfig &&
            (identical(other.logging, logging) || other.logging == logging));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, logging);

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidFFIConfig(logging: $logging)';
  }
}

/// @nodoc
abstract mixin class _$VeilidFFIConfigCopyWith<$Res>
    implements $VeilidFFIConfigCopyWith<$Res> {
  factory _$VeilidFFIConfigCopyWith(
          _VeilidFFIConfig value, $Res Function(_VeilidFFIConfig) _then) =
      __$VeilidFFIConfigCopyWithImpl;
  @override
  @useResult
  $Res call({VeilidFFIConfigLogging logging});

  @override
  $VeilidFFIConfigLoggingCopyWith<$Res> get logging;
}

/// @nodoc
class __$VeilidFFIConfigCopyWithImpl<$Res>
    implements _$VeilidFFIConfigCopyWith<$Res> {
  __$VeilidFFIConfigCopyWithImpl(this._self, this._then);

  final _VeilidFFIConfig _self;
  final $Res Function(_VeilidFFIConfig) _then;

  /// Create a copy of VeilidFFIConfig
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? logging = null,
  }) {
    return _then(_VeilidFFIConfig(
      logging: null == logging
          ? _self.logging
          : logging // ignore: cast_nullable_to_non_nullable
              as VeilidFFIConfigLogging,
    ));
  }

  /// Create a copy of VeilidFFIConfig
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $VeilidFFIConfigLoggingCopyWith<$Res> get logging {
    return $VeilidFFIConfigLoggingCopyWith<$Res>(_self.logging, (value) {
      return _then(_self.copyWith(logging: value));
    });
  }
}

/// @nodoc
mixin _$VeilidWASMConfigLoggingPerformance implements DiagnosticableTreeMixin {
  bool get enabled;
  VeilidConfigLogLevel get level;
  bool get logsInTimings;
  bool get logsInConsole;
  List<String> get ignoreLogTargets;

  /// Create a copy of VeilidWASMConfigLoggingPerformance
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $VeilidWASMConfigLoggingPerformanceCopyWith<
          VeilidWASMConfigLoggingPerformance>
      get copyWith => _$VeilidWASMConfigLoggingPerformanceCopyWithImpl<
              VeilidWASMConfigLoggingPerformance>(
          this as VeilidWASMConfigLoggingPerformance, _$identity);

  /// Serializes this VeilidWASMConfigLoggingPerformance to a JSON map.
  Map<String, dynamic> toJson();

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
    properties
      ..add(DiagnosticsProperty('type', 'VeilidWASMConfigLoggingPerformance'))
      ..add(DiagnosticsProperty('enabled', enabled))
      ..add(DiagnosticsProperty('level', level))
      ..add(DiagnosticsProperty('logsInTimings', logsInTimings))
      ..add(DiagnosticsProperty('logsInConsole', logsInConsole))
      ..add(DiagnosticsProperty('ignoreLogTargets', ignoreLogTargets));
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is VeilidWASMConfigLoggingPerformance &&
            (identical(other.enabled, enabled) || other.enabled == enabled) &&
            (identical(other.level, level) || other.level == level) &&
            (identical(other.logsInTimings, logsInTimings) ||
                other.logsInTimings == logsInTimings) &&
            (identical(other.logsInConsole, logsInConsole) ||
                other.logsInConsole == logsInConsole) &&
            const DeepCollectionEquality()
                .equals(other.ignoreLogTargets, ignoreLogTargets));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, enabled, level, logsInTimings,
      logsInConsole, const DeepCollectionEquality().hash(ignoreLogTargets));

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidWASMConfigLoggingPerformance(enabled: $enabled, level: $level, logsInTimings: $logsInTimings, logsInConsole: $logsInConsole, ignoreLogTargets: $ignoreLogTargets)';
  }
}

/// @nodoc
abstract mixin class $VeilidWASMConfigLoggingPerformanceCopyWith<$Res> {
  factory $VeilidWASMConfigLoggingPerformanceCopyWith(
          VeilidWASMConfigLoggingPerformance value,
          $Res Function(VeilidWASMConfigLoggingPerformance) _then) =
      _$VeilidWASMConfigLoggingPerformanceCopyWithImpl;
  @useResult
  $Res call(
      {bool enabled,
      VeilidConfigLogLevel level,
      bool logsInTimings,
      bool logsInConsole,
      List<String> ignoreLogTargets});
}

/// @nodoc
class _$VeilidWASMConfigLoggingPerformanceCopyWithImpl<$Res>
    implements $VeilidWASMConfigLoggingPerformanceCopyWith<$Res> {
  _$VeilidWASMConfigLoggingPerformanceCopyWithImpl(this._self, this._then);

  final VeilidWASMConfigLoggingPerformance _self;
  final $Res Function(VeilidWASMConfigLoggingPerformance) _then;

  /// Create a copy of VeilidWASMConfigLoggingPerformance
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? enabled = null,
    Object? level = null,
    Object? logsInTimings = null,
    Object? logsInConsole = null,
    Object? ignoreLogTargets = null,
  }) {
    return _then(_self.copyWith(
      enabled: null == enabled
          ? _self.enabled
          : enabled // ignore: cast_nullable_to_non_nullable
              as bool,
      level: null == level
          ? _self.level
          : level // ignore: cast_nullable_to_non_nullable
              as VeilidConfigLogLevel,
      logsInTimings: null == logsInTimings
          ? _self.logsInTimings
          : logsInTimings // ignore: cast_nullable_to_non_nullable
              as bool,
      logsInConsole: null == logsInConsole
          ? _self.logsInConsole
          : logsInConsole // ignore: cast_nullable_to_non_nullable
              as bool,
      ignoreLogTargets: null == ignoreLogTargets
          ? _self.ignoreLogTargets
          : ignoreLogTargets // ignore: cast_nullable_to_non_nullable
              as List<String>,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _VeilidWASMConfigLoggingPerformance
    with DiagnosticableTreeMixin
    implements VeilidWASMConfigLoggingPerformance {
  const _VeilidWASMConfigLoggingPerformance(
      {required this.enabled,
      required this.level,
      required this.logsInTimings,
      required this.logsInConsole,
      final List<String> ignoreLogTargets = const []})
      : _ignoreLogTargets = ignoreLogTargets;
  factory _VeilidWASMConfigLoggingPerformance.fromJson(
          Map<String, dynamic> json) =>
      _$VeilidWASMConfigLoggingPerformanceFromJson(json);

  @override
  final bool enabled;
  @override
  final VeilidConfigLogLevel level;
  @override
  final bool logsInTimings;
  @override
  final bool logsInConsole;
  final List<String> _ignoreLogTargets;
  @override
  @JsonKey()
  List<String> get ignoreLogTargets {
    if (_ignoreLogTargets is EqualUnmodifiableListView)
      return _ignoreLogTargets;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_ignoreLogTargets);
  }

  /// Create a copy of VeilidWASMConfigLoggingPerformance
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  _$VeilidWASMConfigLoggingPerformanceCopyWith<
          _VeilidWASMConfigLoggingPerformance>
      get copyWith => __$VeilidWASMConfigLoggingPerformanceCopyWithImpl<
          _VeilidWASMConfigLoggingPerformance>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$VeilidWASMConfigLoggingPerformanceToJson(
      this,
    );
  }

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
    properties
      ..add(DiagnosticsProperty('type', 'VeilidWASMConfigLoggingPerformance'))
      ..add(DiagnosticsProperty('enabled', enabled))
      ..add(DiagnosticsProperty('level', level))
      ..add(DiagnosticsProperty('logsInTimings', logsInTimings))
      ..add(DiagnosticsProperty('logsInConsole', logsInConsole))
      ..add(DiagnosticsProperty('ignoreLogTargets', ignoreLogTargets));
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _VeilidWASMConfigLoggingPerformance &&
            (identical(other.enabled, enabled) || other.enabled == enabled) &&
            (identical(other.level, level) || other.level == level) &&
            (identical(other.logsInTimings, logsInTimings) ||
                other.logsInTimings == logsInTimings) &&
            (identical(other.logsInConsole, logsInConsole) ||
                other.logsInConsole == logsInConsole) &&
            const DeepCollectionEquality()
                .equals(other._ignoreLogTargets, _ignoreLogTargets));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, enabled, level, logsInTimings,
      logsInConsole, const DeepCollectionEquality().hash(_ignoreLogTargets));

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidWASMConfigLoggingPerformance(enabled: $enabled, level: $level, logsInTimings: $logsInTimings, logsInConsole: $logsInConsole, ignoreLogTargets: $ignoreLogTargets)';
  }
}

/// @nodoc
abstract mixin class _$VeilidWASMConfigLoggingPerformanceCopyWith<$Res>
    implements $VeilidWASMConfigLoggingPerformanceCopyWith<$Res> {
  factory _$VeilidWASMConfigLoggingPerformanceCopyWith(
          _VeilidWASMConfigLoggingPerformance value,
          $Res Function(_VeilidWASMConfigLoggingPerformance) _then) =
      __$VeilidWASMConfigLoggingPerformanceCopyWithImpl;
  @override
  @useResult
  $Res call(
      {bool enabled,
      VeilidConfigLogLevel level,
      bool logsInTimings,
      bool logsInConsole,
      List<String> ignoreLogTargets});
}

/// @nodoc
class __$VeilidWASMConfigLoggingPerformanceCopyWithImpl<$Res>
    implements _$VeilidWASMConfigLoggingPerformanceCopyWith<$Res> {
  __$VeilidWASMConfigLoggingPerformanceCopyWithImpl(this._self, this._then);

  final _VeilidWASMConfigLoggingPerformance _self;
  final $Res Function(_VeilidWASMConfigLoggingPerformance) _then;

  /// Create a copy of VeilidWASMConfigLoggingPerformance
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? enabled = null,
    Object? level = null,
    Object? logsInTimings = null,
    Object? logsInConsole = null,
    Object? ignoreLogTargets = null,
  }) {
    return _then(_VeilidWASMConfigLoggingPerformance(
      enabled: null == enabled
          ? _self.enabled
          : enabled // ignore: cast_nullable_to_non_nullable
              as bool,
      level: null == level
          ? _self.level
          : level // ignore: cast_nullable_to_non_nullable
              as VeilidConfigLogLevel,
      logsInTimings: null == logsInTimings
          ? _self.logsInTimings
          : logsInTimings // ignore: cast_nullable_to_non_nullable
              as bool,
      logsInConsole: null == logsInConsole
          ? _self.logsInConsole
          : logsInConsole // ignore: cast_nullable_to_non_nullable
              as bool,
      ignoreLogTargets: null == ignoreLogTargets
          ? _self._ignoreLogTargets
          : ignoreLogTargets // ignore: cast_nullable_to_non_nullable
              as List<String>,
    ));
  }
}

/// @nodoc
mixin _$VeilidWASMConfigLoggingApi implements DiagnosticableTreeMixin {
  bool get enabled;
  VeilidConfigLogLevel get level;
  List<String> get ignoreLogTargets;

  /// Create a copy of VeilidWASMConfigLoggingApi
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $VeilidWASMConfigLoggingApiCopyWith<VeilidWASMConfigLoggingApi>
      get copyWith =>
          _$VeilidWASMConfigLoggingApiCopyWithImpl<VeilidWASMConfigLoggingApi>(
              this as VeilidWASMConfigLoggingApi, _$identity);

  /// Serializes this VeilidWASMConfigLoggingApi to a JSON map.
  Map<String, dynamic> toJson();

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
    properties
      ..add(DiagnosticsProperty('type', 'VeilidWASMConfigLoggingApi'))
      ..add(DiagnosticsProperty('enabled', enabled))
      ..add(DiagnosticsProperty('level', level))
      ..add(DiagnosticsProperty('ignoreLogTargets', ignoreLogTargets));
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is VeilidWASMConfigLoggingApi &&
            (identical(other.enabled, enabled) || other.enabled == enabled) &&
            (identical(other.level, level) || other.level == level) &&
            const DeepCollectionEquality()
                .equals(other.ignoreLogTargets, ignoreLogTargets));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, enabled, level,
      const DeepCollectionEquality().hash(ignoreLogTargets));

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidWASMConfigLoggingApi(enabled: $enabled, level: $level, ignoreLogTargets: $ignoreLogTargets)';
  }
}

/// @nodoc
abstract mixin class $VeilidWASMConfigLoggingApiCopyWith<$Res> {
  factory $VeilidWASMConfigLoggingApiCopyWith(VeilidWASMConfigLoggingApi value,
          $Res Function(VeilidWASMConfigLoggingApi) _then) =
      _$VeilidWASMConfigLoggingApiCopyWithImpl;
  @useResult
  $Res call(
      {bool enabled,
      VeilidConfigLogLevel level,
      List<String> ignoreLogTargets});
}

/// @nodoc
class _$VeilidWASMConfigLoggingApiCopyWithImpl<$Res>
    implements $VeilidWASMConfigLoggingApiCopyWith<$Res> {
  _$VeilidWASMConfigLoggingApiCopyWithImpl(this._self, this._then);

  final VeilidWASMConfigLoggingApi _self;
  final $Res Function(VeilidWASMConfigLoggingApi) _then;

  /// Create a copy of VeilidWASMConfigLoggingApi
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? enabled = null,
    Object? level = null,
    Object? ignoreLogTargets = null,
  }) {
    return _then(_self.copyWith(
      enabled: null == enabled
          ? _self.enabled
          : enabled // ignore: cast_nullable_to_non_nullable
              as bool,
      level: null == level
          ? _self.level
          : level // ignore: cast_nullable_to_non_nullable
              as VeilidConfigLogLevel,
      ignoreLogTargets: null == ignoreLogTargets
          ? _self.ignoreLogTargets
          : ignoreLogTargets // ignore: cast_nullable_to_non_nullable
              as List<String>,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _VeilidWASMConfigLoggingApi
    with DiagnosticableTreeMixin
    implements VeilidWASMConfigLoggingApi {
  const _VeilidWASMConfigLoggingApi(
      {required this.enabled,
      required this.level,
      final List<String> ignoreLogTargets = const []})
      : _ignoreLogTargets = ignoreLogTargets;
  factory _VeilidWASMConfigLoggingApi.fromJson(Map<String, dynamic> json) =>
      _$VeilidWASMConfigLoggingApiFromJson(json);

  @override
  final bool enabled;
  @override
  final VeilidConfigLogLevel level;
  final List<String> _ignoreLogTargets;
  @override
  @JsonKey()
  List<String> get ignoreLogTargets {
    if (_ignoreLogTargets is EqualUnmodifiableListView)
      return _ignoreLogTargets;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_ignoreLogTargets);
  }

  /// Create a copy of VeilidWASMConfigLoggingApi
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  _$VeilidWASMConfigLoggingApiCopyWith<_VeilidWASMConfigLoggingApi>
      get copyWith => __$VeilidWASMConfigLoggingApiCopyWithImpl<
          _VeilidWASMConfigLoggingApi>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$VeilidWASMConfigLoggingApiToJson(
      this,
    );
  }

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
    properties
      ..add(DiagnosticsProperty('type', 'VeilidWASMConfigLoggingApi'))
      ..add(DiagnosticsProperty('enabled', enabled))
      ..add(DiagnosticsProperty('level', level))
      ..add(DiagnosticsProperty('ignoreLogTargets', ignoreLogTargets));
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _VeilidWASMConfigLoggingApi &&
            (identical(other.enabled, enabled) || other.enabled == enabled) &&
            (identical(other.level, level) || other.level == level) &&
            const DeepCollectionEquality()
                .equals(other._ignoreLogTargets, _ignoreLogTargets));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, enabled, level,
      const DeepCollectionEquality().hash(_ignoreLogTargets));

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidWASMConfigLoggingApi(enabled: $enabled, level: $level, ignoreLogTargets: $ignoreLogTargets)';
  }
}

/// @nodoc
abstract mixin class _$VeilidWASMConfigLoggingApiCopyWith<$Res>
    implements $VeilidWASMConfigLoggingApiCopyWith<$Res> {
  factory _$VeilidWASMConfigLoggingApiCopyWith(
          _VeilidWASMConfigLoggingApi value,
          $Res Function(_VeilidWASMConfigLoggingApi) _then) =
      __$VeilidWASMConfigLoggingApiCopyWithImpl;
  @override
  @useResult
  $Res call(
      {bool enabled,
      VeilidConfigLogLevel level,
      List<String> ignoreLogTargets});
}

/// @nodoc
class __$VeilidWASMConfigLoggingApiCopyWithImpl<$Res>
    implements _$VeilidWASMConfigLoggingApiCopyWith<$Res> {
  __$VeilidWASMConfigLoggingApiCopyWithImpl(this._self, this._then);

  final _VeilidWASMConfigLoggingApi _self;
  final $Res Function(_VeilidWASMConfigLoggingApi) _then;

  /// Create a copy of VeilidWASMConfigLoggingApi
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? enabled = null,
    Object? level = null,
    Object? ignoreLogTargets = null,
  }) {
    return _then(_VeilidWASMConfigLoggingApi(
      enabled: null == enabled
          ? _self.enabled
          : enabled // ignore: cast_nullable_to_non_nullable
              as bool,
      level: null == level
          ? _self.level
          : level // ignore: cast_nullable_to_non_nullable
              as VeilidConfigLogLevel,
      ignoreLogTargets: null == ignoreLogTargets
          ? _self._ignoreLogTargets
          : ignoreLogTargets // ignore: cast_nullable_to_non_nullable
              as List<String>,
    ));
  }
}

/// @nodoc
mixin _$VeilidWASMConfigLogging implements DiagnosticableTreeMixin {
  VeilidWASMConfigLoggingPerformance get performance;
  VeilidWASMConfigLoggingApi get api;

  /// Create a copy of VeilidWASMConfigLogging
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $VeilidWASMConfigLoggingCopyWith<VeilidWASMConfigLogging> get copyWith =>
      _$VeilidWASMConfigLoggingCopyWithImpl<VeilidWASMConfigLogging>(
          this as VeilidWASMConfigLogging, _$identity);

  /// Serializes this VeilidWASMConfigLogging to a JSON map.
  Map<String, dynamic> toJson();

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
    properties
      ..add(DiagnosticsProperty('type', 'VeilidWASMConfigLogging'))
      ..add(DiagnosticsProperty('performance', performance))
      ..add(DiagnosticsProperty('api', api));
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is VeilidWASMConfigLogging &&
            (identical(other.performance, performance) ||
                other.performance == performance) &&
            (identical(other.api, api) || other.api == api));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, performance, api);

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidWASMConfigLogging(performance: $performance, api: $api)';
  }
}

/// @nodoc
abstract mixin class $VeilidWASMConfigLoggingCopyWith<$Res> {
  factory $VeilidWASMConfigLoggingCopyWith(VeilidWASMConfigLogging value,
          $Res Function(VeilidWASMConfigLogging) _then) =
      _$VeilidWASMConfigLoggingCopyWithImpl;
  @useResult
  $Res call(
      {VeilidWASMConfigLoggingPerformance performance,
      VeilidWASMConfigLoggingApi api});

  $VeilidWASMConfigLoggingPerformanceCopyWith<$Res> get performance;
  $VeilidWASMConfigLoggingApiCopyWith<$Res> get api;
}

/// @nodoc
class _$VeilidWASMConfigLoggingCopyWithImpl<$Res>
    implements $VeilidWASMConfigLoggingCopyWith<$Res> {
  _$VeilidWASMConfigLoggingCopyWithImpl(this._self, this._then);

  final VeilidWASMConfigLogging _self;
  final $Res Function(VeilidWASMConfigLogging) _then;

  /// Create a copy of VeilidWASMConfigLogging
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? performance = null,
    Object? api = null,
  }) {
    return _then(_self.copyWith(
      performance: null == performance
          ? _self.performance
          : performance // ignore: cast_nullable_to_non_nullable
              as VeilidWASMConfigLoggingPerformance,
      api: null == api
          ? _self.api
          : api // ignore: cast_nullable_to_non_nullable
              as VeilidWASMConfigLoggingApi,
    ));
  }

  /// Create a copy of VeilidWASMConfigLogging
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $VeilidWASMConfigLoggingPerformanceCopyWith<$Res> get performance {
    return $VeilidWASMConfigLoggingPerformanceCopyWith<$Res>(_self.performance,
        (value) {
      return _then(_self.copyWith(performance: value));
    });
  }

  /// Create a copy of VeilidWASMConfigLogging
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $VeilidWASMConfigLoggingApiCopyWith<$Res> get api {
    return $VeilidWASMConfigLoggingApiCopyWith<$Res>(_self.api, (value) {
      return _then(_self.copyWith(api: value));
    });
  }
}

/// @nodoc
@JsonSerializable()
class _VeilidWASMConfigLogging
    with DiagnosticableTreeMixin
    implements VeilidWASMConfigLogging {
  const _VeilidWASMConfigLogging(
      {required this.performance, required this.api});
  factory _VeilidWASMConfigLogging.fromJson(Map<String, dynamic> json) =>
      _$VeilidWASMConfigLoggingFromJson(json);

  @override
  final VeilidWASMConfigLoggingPerformance performance;
  @override
  final VeilidWASMConfigLoggingApi api;

  /// Create a copy of VeilidWASMConfigLogging
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  _$VeilidWASMConfigLoggingCopyWith<_VeilidWASMConfigLogging> get copyWith =>
      __$VeilidWASMConfigLoggingCopyWithImpl<_VeilidWASMConfigLogging>(
          this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$VeilidWASMConfigLoggingToJson(
      this,
    );
  }

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
    properties
      ..add(DiagnosticsProperty('type', 'VeilidWASMConfigLogging'))
      ..add(DiagnosticsProperty('performance', performance))
      ..add(DiagnosticsProperty('api', api));
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _VeilidWASMConfigLogging &&
            (identical(other.performance, performance) ||
                other.performance == performance) &&
            (identical(other.api, api) || other.api == api));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, performance, api);

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidWASMConfigLogging(performance: $performance, api: $api)';
  }
}

/// @nodoc
abstract mixin class _$VeilidWASMConfigLoggingCopyWith<$Res>
    implements $VeilidWASMConfigLoggingCopyWith<$Res> {
  factory _$VeilidWASMConfigLoggingCopyWith(_VeilidWASMConfigLogging value,
          $Res Function(_VeilidWASMConfigLogging) _then) =
      __$VeilidWASMConfigLoggingCopyWithImpl;
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
class __$VeilidWASMConfigLoggingCopyWithImpl<$Res>
    implements _$VeilidWASMConfigLoggingCopyWith<$Res> {
  __$VeilidWASMConfigLoggingCopyWithImpl(this._self, this._then);

  final _VeilidWASMConfigLogging _self;
  final $Res Function(_VeilidWASMConfigLogging) _then;

  /// Create a copy of VeilidWASMConfigLogging
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? performance = null,
    Object? api = null,
  }) {
    return _then(_VeilidWASMConfigLogging(
      performance: null == performance
          ? _self.performance
          : performance // ignore: cast_nullable_to_non_nullable
              as VeilidWASMConfigLoggingPerformance,
      api: null == api
          ? _self.api
          : api // ignore: cast_nullable_to_non_nullable
              as VeilidWASMConfigLoggingApi,
    ));
  }

  /// Create a copy of VeilidWASMConfigLogging
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $VeilidWASMConfigLoggingPerformanceCopyWith<$Res> get performance {
    return $VeilidWASMConfigLoggingPerformanceCopyWith<$Res>(_self.performance,
        (value) {
      return _then(_self.copyWith(performance: value));
    });
  }

  /// Create a copy of VeilidWASMConfigLogging
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $VeilidWASMConfigLoggingApiCopyWith<$Res> get api {
    return $VeilidWASMConfigLoggingApiCopyWith<$Res>(_self.api, (value) {
      return _then(_self.copyWith(api: value));
    });
  }
}

/// @nodoc
mixin _$VeilidWASMConfig implements DiagnosticableTreeMixin {
  VeilidWASMConfigLogging get logging;

  /// Create a copy of VeilidWASMConfig
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $VeilidWASMConfigCopyWith<VeilidWASMConfig> get copyWith =>
      _$VeilidWASMConfigCopyWithImpl<VeilidWASMConfig>(
          this as VeilidWASMConfig, _$identity);

  /// Serializes this VeilidWASMConfig to a JSON map.
  Map<String, dynamic> toJson();

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
    properties
      ..add(DiagnosticsProperty('type', 'VeilidWASMConfig'))
      ..add(DiagnosticsProperty('logging', logging));
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is VeilidWASMConfig &&
            (identical(other.logging, logging) || other.logging == logging));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, logging);

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidWASMConfig(logging: $logging)';
  }
}

/// @nodoc
abstract mixin class $VeilidWASMConfigCopyWith<$Res> {
  factory $VeilidWASMConfigCopyWith(
          VeilidWASMConfig value, $Res Function(VeilidWASMConfig) _then) =
      _$VeilidWASMConfigCopyWithImpl;
  @useResult
  $Res call({VeilidWASMConfigLogging logging});

  $VeilidWASMConfigLoggingCopyWith<$Res> get logging;
}

/// @nodoc
class _$VeilidWASMConfigCopyWithImpl<$Res>
    implements $VeilidWASMConfigCopyWith<$Res> {
  _$VeilidWASMConfigCopyWithImpl(this._self, this._then);

  final VeilidWASMConfig _self;
  final $Res Function(VeilidWASMConfig) _then;

  /// Create a copy of VeilidWASMConfig
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? logging = null,
  }) {
    return _then(_self.copyWith(
      logging: null == logging
          ? _self.logging
          : logging // ignore: cast_nullable_to_non_nullable
              as VeilidWASMConfigLogging,
    ));
  }

  /// Create a copy of VeilidWASMConfig
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $VeilidWASMConfigLoggingCopyWith<$Res> get logging {
    return $VeilidWASMConfigLoggingCopyWith<$Res>(_self.logging, (value) {
      return _then(_self.copyWith(logging: value));
    });
  }
}

/// @nodoc
@JsonSerializable()
class _VeilidWASMConfig
    with DiagnosticableTreeMixin
    implements VeilidWASMConfig {
  const _VeilidWASMConfig({required this.logging});
  factory _VeilidWASMConfig.fromJson(Map<String, dynamic> json) =>
      _$VeilidWASMConfigFromJson(json);

  @override
  final VeilidWASMConfigLogging logging;

  /// Create a copy of VeilidWASMConfig
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  _$VeilidWASMConfigCopyWith<_VeilidWASMConfig> get copyWith =>
      __$VeilidWASMConfigCopyWithImpl<_VeilidWASMConfig>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$VeilidWASMConfigToJson(
      this,
    );
  }

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
    properties
      ..add(DiagnosticsProperty('type', 'VeilidWASMConfig'))
      ..add(DiagnosticsProperty('logging', logging));
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _VeilidWASMConfig &&
            (identical(other.logging, logging) || other.logging == logging));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, logging);

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidWASMConfig(logging: $logging)';
  }
}

/// @nodoc
abstract mixin class _$VeilidWASMConfigCopyWith<$Res>
    implements $VeilidWASMConfigCopyWith<$Res> {
  factory _$VeilidWASMConfigCopyWith(
          _VeilidWASMConfig value, $Res Function(_VeilidWASMConfig) _then) =
      __$VeilidWASMConfigCopyWithImpl;
  @override
  @useResult
  $Res call({VeilidWASMConfigLogging logging});

  @override
  $VeilidWASMConfigLoggingCopyWith<$Res> get logging;
}

/// @nodoc
class __$VeilidWASMConfigCopyWithImpl<$Res>
    implements _$VeilidWASMConfigCopyWith<$Res> {
  __$VeilidWASMConfigCopyWithImpl(this._self, this._then);

  final _VeilidWASMConfig _self;
  final $Res Function(_VeilidWASMConfig) _then;

  /// Create a copy of VeilidWASMConfig
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? logging = null,
  }) {
    return _then(_VeilidWASMConfig(
      logging: null == logging
          ? _self.logging
          : logging // ignore: cast_nullable_to_non_nullable
              as VeilidWASMConfigLogging,
    ));
  }

  /// Create a copy of VeilidWASMConfig
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $VeilidWASMConfigLoggingCopyWith<$Res> get logging {
    return $VeilidWASMConfigLoggingCopyWith<$Res>(_self.logging, (value) {
      return _then(_self.copyWith(logging: value));
    });
  }
}

/// @nodoc
mixin _$VeilidConfigHTTPS implements DiagnosticableTreeMixin {
  bool get enabled;
  String get listenAddress;
  String get path;
  String? get url;

  /// Create a copy of VeilidConfigHTTPS
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $VeilidConfigHTTPSCopyWith<VeilidConfigHTTPS> get copyWith =>
      _$VeilidConfigHTTPSCopyWithImpl<VeilidConfigHTTPS>(
          this as VeilidConfigHTTPS, _$identity);

  /// Serializes this VeilidConfigHTTPS to a JSON map.
  Map<String, dynamic> toJson();

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
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
            other is VeilidConfigHTTPS &&
            (identical(other.enabled, enabled) || other.enabled == enabled) &&
            (identical(other.listenAddress, listenAddress) ||
                other.listenAddress == listenAddress) &&
            (identical(other.path, path) || other.path == path) &&
            (identical(other.url, url) || other.url == url));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode =>
      Object.hash(runtimeType, enabled, listenAddress, path, url);

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidConfigHTTPS(enabled: $enabled, listenAddress: $listenAddress, path: $path, url: $url)';
  }
}

/// @nodoc
abstract mixin class $VeilidConfigHTTPSCopyWith<$Res> {
  factory $VeilidConfigHTTPSCopyWith(
          VeilidConfigHTTPS value, $Res Function(VeilidConfigHTTPS) _then) =
      _$VeilidConfigHTTPSCopyWithImpl;
  @useResult
  $Res call({bool enabled, String listenAddress, String path, String? url});
}

/// @nodoc
class _$VeilidConfigHTTPSCopyWithImpl<$Res>
    implements $VeilidConfigHTTPSCopyWith<$Res> {
  _$VeilidConfigHTTPSCopyWithImpl(this._self, this._then);

  final VeilidConfigHTTPS _self;
  final $Res Function(VeilidConfigHTTPS) _then;

  /// Create a copy of VeilidConfigHTTPS
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? enabled = null,
    Object? listenAddress = null,
    Object? path = null,
    Object? url = freezed,
  }) {
    return _then(_self.copyWith(
      enabled: null == enabled
          ? _self.enabled
          : enabled // ignore: cast_nullable_to_non_nullable
              as bool,
      listenAddress: null == listenAddress
          ? _self.listenAddress
          : listenAddress // ignore: cast_nullable_to_non_nullable
              as String,
      path: null == path
          ? _self.path
          : path // ignore: cast_nullable_to_non_nullable
              as String,
      url: freezed == url
          ? _self.url
          : url // ignore: cast_nullable_to_non_nullable
              as String?,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _VeilidConfigHTTPS
    with DiagnosticableTreeMixin
    implements VeilidConfigHTTPS {
  const _VeilidConfigHTTPS(
      {required this.enabled,
      required this.listenAddress,
      required this.path,
      this.url});
  factory _VeilidConfigHTTPS.fromJson(Map<String, dynamic> json) =>
      _$VeilidConfigHTTPSFromJson(json);

  @override
  final bool enabled;
  @override
  final String listenAddress;
  @override
  final String path;
  @override
  final String? url;

  /// Create a copy of VeilidConfigHTTPS
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  _$VeilidConfigHTTPSCopyWith<_VeilidConfigHTTPS> get copyWith =>
      __$VeilidConfigHTTPSCopyWithImpl<_VeilidConfigHTTPS>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$VeilidConfigHTTPSToJson(
      this,
    );
  }

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
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
            other is _VeilidConfigHTTPS &&
            (identical(other.enabled, enabled) || other.enabled == enabled) &&
            (identical(other.listenAddress, listenAddress) ||
                other.listenAddress == listenAddress) &&
            (identical(other.path, path) || other.path == path) &&
            (identical(other.url, url) || other.url == url));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode =>
      Object.hash(runtimeType, enabled, listenAddress, path, url);

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidConfigHTTPS(enabled: $enabled, listenAddress: $listenAddress, path: $path, url: $url)';
  }
}

/// @nodoc
abstract mixin class _$VeilidConfigHTTPSCopyWith<$Res>
    implements $VeilidConfigHTTPSCopyWith<$Res> {
  factory _$VeilidConfigHTTPSCopyWith(
          _VeilidConfigHTTPS value, $Res Function(_VeilidConfigHTTPS) _then) =
      __$VeilidConfigHTTPSCopyWithImpl;
  @override
  @useResult
  $Res call({bool enabled, String listenAddress, String path, String? url});
}

/// @nodoc
class __$VeilidConfigHTTPSCopyWithImpl<$Res>
    implements _$VeilidConfigHTTPSCopyWith<$Res> {
  __$VeilidConfigHTTPSCopyWithImpl(this._self, this._then);

  final _VeilidConfigHTTPS _self;
  final $Res Function(_VeilidConfigHTTPS) _then;

  /// Create a copy of VeilidConfigHTTPS
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? enabled = null,
    Object? listenAddress = null,
    Object? path = null,
    Object? url = freezed,
  }) {
    return _then(_VeilidConfigHTTPS(
      enabled: null == enabled
          ? _self.enabled
          : enabled // ignore: cast_nullable_to_non_nullable
              as bool,
      listenAddress: null == listenAddress
          ? _self.listenAddress
          : listenAddress // ignore: cast_nullable_to_non_nullable
              as String,
      path: null == path
          ? _self.path
          : path // ignore: cast_nullable_to_non_nullable
              as String,
      url: freezed == url
          ? _self.url
          : url // ignore: cast_nullable_to_non_nullable
              as String?,
    ));
  }
}

/// @nodoc
mixin _$VeilidConfigHTTP implements DiagnosticableTreeMixin {
  bool get enabled;
  String get listenAddress;
  String get path;
  String? get url;

  /// Create a copy of VeilidConfigHTTP
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $VeilidConfigHTTPCopyWith<VeilidConfigHTTP> get copyWith =>
      _$VeilidConfigHTTPCopyWithImpl<VeilidConfigHTTP>(
          this as VeilidConfigHTTP, _$identity);

  /// Serializes this VeilidConfigHTTP to a JSON map.
  Map<String, dynamic> toJson();

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
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
            other is VeilidConfigHTTP &&
            (identical(other.enabled, enabled) || other.enabled == enabled) &&
            (identical(other.listenAddress, listenAddress) ||
                other.listenAddress == listenAddress) &&
            (identical(other.path, path) || other.path == path) &&
            (identical(other.url, url) || other.url == url));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode =>
      Object.hash(runtimeType, enabled, listenAddress, path, url);

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidConfigHTTP(enabled: $enabled, listenAddress: $listenAddress, path: $path, url: $url)';
  }
}

/// @nodoc
abstract mixin class $VeilidConfigHTTPCopyWith<$Res> {
  factory $VeilidConfigHTTPCopyWith(
          VeilidConfigHTTP value, $Res Function(VeilidConfigHTTP) _then) =
      _$VeilidConfigHTTPCopyWithImpl;
  @useResult
  $Res call({bool enabled, String listenAddress, String path, String? url});
}

/// @nodoc
class _$VeilidConfigHTTPCopyWithImpl<$Res>
    implements $VeilidConfigHTTPCopyWith<$Res> {
  _$VeilidConfigHTTPCopyWithImpl(this._self, this._then);

  final VeilidConfigHTTP _self;
  final $Res Function(VeilidConfigHTTP) _then;

  /// Create a copy of VeilidConfigHTTP
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? enabled = null,
    Object? listenAddress = null,
    Object? path = null,
    Object? url = freezed,
  }) {
    return _then(_self.copyWith(
      enabled: null == enabled
          ? _self.enabled
          : enabled // ignore: cast_nullable_to_non_nullable
              as bool,
      listenAddress: null == listenAddress
          ? _self.listenAddress
          : listenAddress // ignore: cast_nullable_to_non_nullable
              as String,
      path: null == path
          ? _self.path
          : path // ignore: cast_nullable_to_non_nullable
              as String,
      url: freezed == url
          ? _self.url
          : url // ignore: cast_nullable_to_non_nullable
              as String?,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _VeilidConfigHTTP
    with DiagnosticableTreeMixin
    implements VeilidConfigHTTP {
  const _VeilidConfigHTTP(
      {required this.enabled,
      required this.listenAddress,
      required this.path,
      this.url});
  factory _VeilidConfigHTTP.fromJson(Map<String, dynamic> json) =>
      _$VeilidConfigHTTPFromJson(json);

  @override
  final bool enabled;
  @override
  final String listenAddress;
  @override
  final String path;
  @override
  final String? url;

  /// Create a copy of VeilidConfigHTTP
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  _$VeilidConfigHTTPCopyWith<_VeilidConfigHTTP> get copyWith =>
      __$VeilidConfigHTTPCopyWithImpl<_VeilidConfigHTTP>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$VeilidConfigHTTPToJson(
      this,
    );
  }

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
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
            other is _VeilidConfigHTTP &&
            (identical(other.enabled, enabled) || other.enabled == enabled) &&
            (identical(other.listenAddress, listenAddress) ||
                other.listenAddress == listenAddress) &&
            (identical(other.path, path) || other.path == path) &&
            (identical(other.url, url) || other.url == url));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode =>
      Object.hash(runtimeType, enabled, listenAddress, path, url);

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidConfigHTTP(enabled: $enabled, listenAddress: $listenAddress, path: $path, url: $url)';
  }
}

/// @nodoc
abstract mixin class _$VeilidConfigHTTPCopyWith<$Res>
    implements $VeilidConfigHTTPCopyWith<$Res> {
  factory _$VeilidConfigHTTPCopyWith(
          _VeilidConfigHTTP value, $Res Function(_VeilidConfigHTTP) _then) =
      __$VeilidConfigHTTPCopyWithImpl;
  @override
  @useResult
  $Res call({bool enabled, String listenAddress, String path, String? url});
}

/// @nodoc
class __$VeilidConfigHTTPCopyWithImpl<$Res>
    implements _$VeilidConfigHTTPCopyWith<$Res> {
  __$VeilidConfigHTTPCopyWithImpl(this._self, this._then);

  final _VeilidConfigHTTP _self;
  final $Res Function(_VeilidConfigHTTP) _then;

  /// Create a copy of VeilidConfigHTTP
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? enabled = null,
    Object? listenAddress = null,
    Object? path = null,
    Object? url = freezed,
  }) {
    return _then(_VeilidConfigHTTP(
      enabled: null == enabled
          ? _self.enabled
          : enabled // ignore: cast_nullable_to_non_nullable
              as bool,
      listenAddress: null == listenAddress
          ? _self.listenAddress
          : listenAddress // ignore: cast_nullable_to_non_nullable
              as String,
      path: null == path
          ? _self.path
          : path // ignore: cast_nullable_to_non_nullable
              as String,
      url: freezed == url
          ? _self.url
          : url // ignore: cast_nullable_to_non_nullable
              as String?,
    ));
  }
}

/// @nodoc
mixin _$VeilidConfigApplication implements DiagnosticableTreeMixin {
  VeilidConfigHTTPS get https;
  VeilidConfigHTTP get http;

  /// Create a copy of VeilidConfigApplication
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $VeilidConfigApplicationCopyWith<VeilidConfigApplication> get copyWith =>
      _$VeilidConfigApplicationCopyWithImpl<VeilidConfigApplication>(
          this as VeilidConfigApplication, _$identity);

  /// Serializes this VeilidConfigApplication to a JSON map.
  Map<String, dynamic> toJson();

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
    properties
      ..add(DiagnosticsProperty('type', 'VeilidConfigApplication'))
      ..add(DiagnosticsProperty('https', https))
      ..add(DiagnosticsProperty('http', http));
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is VeilidConfigApplication &&
            (identical(other.https, https) || other.https == https) &&
            (identical(other.http, http) || other.http == http));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, https, http);

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidConfigApplication(https: $https, http: $http)';
  }
}

/// @nodoc
abstract mixin class $VeilidConfigApplicationCopyWith<$Res> {
  factory $VeilidConfigApplicationCopyWith(VeilidConfigApplication value,
          $Res Function(VeilidConfigApplication) _then) =
      _$VeilidConfigApplicationCopyWithImpl;
  @useResult
  $Res call({VeilidConfigHTTPS https, VeilidConfigHTTP http});

  $VeilidConfigHTTPSCopyWith<$Res> get https;
  $VeilidConfigHTTPCopyWith<$Res> get http;
}

/// @nodoc
class _$VeilidConfigApplicationCopyWithImpl<$Res>
    implements $VeilidConfigApplicationCopyWith<$Res> {
  _$VeilidConfigApplicationCopyWithImpl(this._self, this._then);

  final VeilidConfigApplication _self;
  final $Res Function(VeilidConfigApplication) _then;

  /// Create a copy of VeilidConfigApplication
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? https = null,
    Object? http = null,
  }) {
    return _then(_self.copyWith(
      https: null == https
          ? _self.https
          : https // ignore: cast_nullable_to_non_nullable
              as VeilidConfigHTTPS,
      http: null == http
          ? _self.http
          : http // ignore: cast_nullable_to_non_nullable
              as VeilidConfigHTTP,
    ));
  }

  /// Create a copy of VeilidConfigApplication
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $VeilidConfigHTTPSCopyWith<$Res> get https {
    return $VeilidConfigHTTPSCopyWith<$Res>(_self.https, (value) {
      return _then(_self.copyWith(https: value));
    });
  }

  /// Create a copy of VeilidConfigApplication
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $VeilidConfigHTTPCopyWith<$Res> get http {
    return $VeilidConfigHTTPCopyWith<$Res>(_self.http, (value) {
      return _then(_self.copyWith(http: value));
    });
  }
}

/// @nodoc
@JsonSerializable()
class _VeilidConfigApplication
    with DiagnosticableTreeMixin
    implements VeilidConfigApplication {
  const _VeilidConfigApplication({required this.https, required this.http});
  factory _VeilidConfigApplication.fromJson(Map<String, dynamic> json) =>
      _$VeilidConfigApplicationFromJson(json);

  @override
  final VeilidConfigHTTPS https;
  @override
  final VeilidConfigHTTP http;

  /// Create a copy of VeilidConfigApplication
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  _$VeilidConfigApplicationCopyWith<_VeilidConfigApplication> get copyWith =>
      __$VeilidConfigApplicationCopyWithImpl<_VeilidConfigApplication>(
          this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$VeilidConfigApplicationToJson(
      this,
    );
  }

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
    properties
      ..add(DiagnosticsProperty('type', 'VeilidConfigApplication'))
      ..add(DiagnosticsProperty('https', https))
      ..add(DiagnosticsProperty('http', http));
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _VeilidConfigApplication &&
            (identical(other.https, https) || other.https == https) &&
            (identical(other.http, http) || other.http == http));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, https, http);

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidConfigApplication(https: $https, http: $http)';
  }
}

/// @nodoc
abstract mixin class _$VeilidConfigApplicationCopyWith<$Res>
    implements $VeilidConfigApplicationCopyWith<$Res> {
  factory _$VeilidConfigApplicationCopyWith(_VeilidConfigApplication value,
          $Res Function(_VeilidConfigApplication) _then) =
      __$VeilidConfigApplicationCopyWithImpl;
  @override
  @useResult
  $Res call({VeilidConfigHTTPS https, VeilidConfigHTTP http});

  @override
  $VeilidConfigHTTPSCopyWith<$Res> get https;
  @override
  $VeilidConfigHTTPCopyWith<$Res> get http;
}

/// @nodoc
class __$VeilidConfigApplicationCopyWithImpl<$Res>
    implements _$VeilidConfigApplicationCopyWith<$Res> {
  __$VeilidConfigApplicationCopyWithImpl(this._self, this._then);

  final _VeilidConfigApplication _self;
  final $Res Function(_VeilidConfigApplication) _then;

  /// Create a copy of VeilidConfigApplication
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? https = null,
    Object? http = null,
  }) {
    return _then(_VeilidConfigApplication(
      https: null == https
          ? _self.https
          : https // ignore: cast_nullable_to_non_nullable
              as VeilidConfigHTTPS,
      http: null == http
          ? _self.http
          : http // ignore: cast_nullable_to_non_nullable
              as VeilidConfigHTTP,
    ));
  }

  /// Create a copy of VeilidConfigApplication
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $VeilidConfigHTTPSCopyWith<$Res> get https {
    return $VeilidConfigHTTPSCopyWith<$Res>(_self.https, (value) {
      return _then(_self.copyWith(https: value));
    });
  }

  /// Create a copy of VeilidConfigApplication
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $VeilidConfigHTTPCopyWith<$Res> get http {
    return $VeilidConfigHTTPCopyWith<$Res>(_self.http, (value) {
      return _then(_self.copyWith(http: value));
    });
  }
}

/// @nodoc
mixin _$VeilidConfigUDP implements DiagnosticableTreeMixin {
  bool get enabled;
  int get socketPoolSize;
  String get listenAddress;
  String? get publicAddress;

  /// Create a copy of VeilidConfigUDP
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $VeilidConfigUDPCopyWith<VeilidConfigUDP> get copyWith =>
      _$VeilidConfigUDPCopyWithImpl<VeilidConfigUDP>(
          this as VeilidConfigUDP, _$identity);

  /// Serializes this VeilidConfigUDP to a JSON map.
  Map<String, dynamic> toJson();

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
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
            other is VeilidConfigUDP &&
            (identical(other.enabled, enabled) || other.enabled == enabled) &&
            (identical(other.socketPoolSize, socketPoolSize) ||
                other.socketPoolSize == socketPoolSize) &&
            (identical(other.listenAddress, listenAddress) ||
                other.listenAddress == listenAddress) &&
            (identical(other.publicAddress, publicAddress) ||
                other.publicAddress == publicAddress));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(
      runtimeType, enabled, socketPoolSize, listenAddress, publicAddress);

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidConfigUDP(enabled: $enabled, socketPoolSize: $socketPoolSize, listenAddress: $listenAddress, publicAddress: $publicAddress)';
  }
}

/// @nodoc
abstract mixin class $VeilidConfigUDPCopyWith<$Res> {
  factory $VeilidConfigUDPCopyWith(
          VeilidConfigUDP value, $Res Function(VeilidConfigUDP) _then) =
      _$VeilidConfigUDPCopyWithImpl;
  @useResult
  $Res call(
      {bool enabled,
      int socketPoolSize,
      String listenAddress,
      String? publicAddress});
}

/// @nodoc
class _$VeilidConfigUDPCopyWithImpl<$Res>
    implements $VeilidConfigUDPCopyWith<$Res> {
  _$VeilidConfigUDPCopyWithImpl(this._self, this._then);

  final VeilidConfigUDP _self;
  final $Res Function(VeilidConfigUDP) _then;

  /// Create a copy of VeilidConfigUDP
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? enabled = null,
    Object? socketPoolSize = null,
    Object? listenAddress = null,
    Object? publicAddress = freezed,
  }) {
    return _then(_self.copyWith(
      enabled: null == enabled
          ? _self.enabled
          : enabled // ignore: cast_nullable_to_non_nullable
              as bool,
      socketPoolSize: null == socketPoolSize
          ? _self.socketPoolSize
          : socketPoolSize // ignore: cast_nullable_to_non_nullable
              as int,
      listenAddress: null == listenAddress
          ? _self.listenAddress
          : listenAddress // ignore: cast_nullable_to_non_nullable
              as String,
      publicAddress: freezed == publicAddress
          ? _self.publicAddress
          : publicAddress // ignore: cast_nullable_to_non_nullable
              as String?,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _VeilidConfigUDP with DiagnosticableTreeMixin implements VeilidConfigUDP {
  const _VeilidConfigUDP(
      {required this.enabled,
      required this.socketPoolSize,
      required this.listenAddress,
      this.publicAddress});
  factory _VeilidConfigUDP.fromJson(Map<String, dynamic> json) =>
      _$VeilidConfigUDPFromJson(json);

  @override
  final bool enabled;
  @override
  final int socketPoolSize;
  @override
  final String listenAddress;
  @override
  final String? publicAddress;

  /// Create a copy of VeilidConfigUDP
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  _$VeilidConfigUDPCopyWith<_VeilidConfigUDP> get copyWith =>
      __$VeilidConfigUDPCopyWithImpl<_VeilidConfigUDP>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$VeilidConfigUDPToJson(
      this,
    );
  }

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
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
            other is _VeilidConfigUDP &&
            (identical(other.enabled, enabled) || other.enabled == enabled) &&
            (identical(other.socketPoolSize, socketPoolSize) ||
                other.socketPoolSize == socketPoolSize) &&
            (identical(other.listenAddress, listenAddress) ||
                other.listenAddress == listenAddress) &&
            (identical(other.publicAddress, publicAddress) ||
                other.publicAddress == publicAddress));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(
      runtimeType, enabled, socketPoolSize, listenAddress, publicAddress);

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidConfigUDP(enabled: $enabled, socketPoolSize: $socketPoolSize, listenAddress: $listenAddress, publicAddress: $publicAddress)';
  }
}

/// @nodoc
abstract mixin class _$VeilidConfigUDPCopyWith<$Res>
    implements $VeilidConfigUDPCopyWith<$Res> {
  factory _$VeilidConfigUDPCopyWith(
          _VeilidConfigUDP value, $Res Function(_VeilidConfigUDP) _then) =
      __$VeilidConfigUDPCopyWithImpl;
  @override
  @useResult
  $Res call(
      {bool enabled,
      int socketPoolSize,
      String listenAddress,
      String? publicAddress});
}

/// @nodoc
class __$VeilidConfigUDPCopyWithImpl<$Res>
    implements _$VeilidConfigUDPCopyWith<$Res> {
  __$VeilidConfigUDPCopyWithImpl(this._self, this._then);

  final _VeilidConfigUDP _self;
  final $Res Function(_VeilidConfigUDP) _then;

  /// Create a copy of VeilidConfigUDP
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? enabled = null,
    Object? socketPoolSize = null,
    Object? listenAddress = null,
    Object? publicAddress = freezed,
  }) {
    return _then(_VeilidConfigUDP(
      enabled: null == enabled
          ? _self.enabled
          : enabled // ignore: cast_nullable_to_non_nullable
              as bool,
      socketPoolSize: null == socketPoolSize
          ? _self.socketPoolSize
          : socketPoolSize // ignore: cast_nullable_to_non_nullable
              as int,
      listenAddress: null == listenAddress
          ? _self.listenAddress
          : listenAddress // ignore: cast_nullable_to_non_nullable
              as String,
      publicAddress: freezed == publicAddress
          ? _self.publicAddress
          : publicAddress // ignore: cast_nullable_to_non_nullable
              as String?,
    ));
  }
}

/// @nodoc
mixin _$VeilidConfigTCP implements DiagnosticableTreeMixin {
  bool get connect;
  bool get listen;
  int get maxConnections;
  String get listenAddress;
  String? get publicAddress;

  /// Create a copy of VeilidConfigTCP
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $VeilidConfigTCPCopyWith<VeilidConfigTCP> get copyWith =>
      _$VeilidConfigTCPCopyWithImpl<VeilidConfigTCP>(
          this as VeilidConfigTCP, _$identity);

  /// Serializes this VeilidConfigTCP to a JSON map.
  Map<String, dynamic> toJson();

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
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
            other is VeilidConfigTCP &&
            (identical(other.connect, connect) || other.connect == connect) &&
            (identical(other.listen, listen) || other.listen == listen) &&
            (identical(other.maxConnections, maxConnections) ||
                other.maxConnections == maxConnections) &&
            (identical(other.listenAddress, listenAddress) ||
                other.listenAddress == listenAddress) &&
            (identical(other.publicAddress, publicAddress) ||
                other.publicAddress == publicAddress));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, connect, listen, maxConnections,
      listenAddress, publicAddress);

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidConfigTCP(connect: $connect, listen: $listen, maxConnections: $maxConnections, listenAddress: $listenAddress, publicAddress: $publicAddress)';
  }
}

/// @nodoc
abstract mixin class $VeilidConfigTCPCopyWith<$Res> {
  factory $VeilidConfigTCPCopyWith(
          VeilidConfigTCP value, $Res Function(VeilidConfigTCP) _then) =
      _$VeilidConfigTCPCopyWithImpl;
  @useResult
  $Res call(
      {bool connect,
      bool listen,
      int maxConnections,
      String listenAddress,
      String? publicAddress});
}

/// @nodoc
class _$VeilidConfigTCPCopyWithImpl<$Res>
    implements $VeilidConfigTCPCopyWith<$Res> {
  _$VeilidConfigTCPCopyWithImpl(this._self, this._then);

  final VeilidConfigTCP _self;
  final $Res Function(VeilidConfigTCP) _then;

  /// Create a copy of VeilidConfigTCP
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? connect = null,
    Object? listen = null,
    Object? maxConnections = null,
    Object? listenAddress = null,
    Object? publicAddress = freezed,
  }) {
    return _then(_self.copyWith(
      connect: null == connect
          ? _self.connect
          : connect // ignore: cast_nullable_to_non_nullable
              as bool,
      listen: null == listen
          ? _self.listen
          : listen // ignore: cast_nullable_to_non_nullable
              as bool,
      maxConnections: null == maxConnections
          ? _self.maxConnections
          : maxConnections // ignore: cast_nullable_to_non_nullable
              as int,
      listenAddress: null == listenAddress
          ? _self.listenAddress
          : listenAddress // ignore: cast_nullable_to_non_nullable
              as String,
      publicAddress: freezed == publicAddress
          ? _self.publicAddress
          : publicAddress // ignore: cast_nullable_to_non_nullable
              as String?,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _VeilidConfigTCP with DiagnosticableTreeMixin implements VeilidConfigTCP {
  const _VeilidConfigTCP(
      {required this.connect,
      required this.listen,
      required this.maxConnections,
      required this.listenAddress,
      this.publicAddress});
  factory _VeilidConfigTCP.fromJson(Map<String, dynamic> json) =>
      _$VeilidConfigTCPFromJson(json);

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

  /// Create a copy of VeilidConfigTCP
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  _$VeilidConfigTCPCopyWith<_VeilidConfigTCP> get copyWith =>
      __$VeilidConfigTCPCopyWithImpl<_VeilidConfigTCP>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$VeilidConfigTCPToJson(
      this,
    );
  }

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
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
            other is _VeilidConfigTCP &&
            (identical(other.connect, connect) || other.connect == connect) &&
            (identical(other.listen, listen) || other.listen == listen) &&
            (identical(other.maxConnections, maxConnections) ||
                other.maxConnections == maxConnections) &&
            (identical(other.listenAddress, listenAddress) ||
                other.listenAddress == listenAddress) &&
            (identical(other.publicAddress, publicAddress) ||
                other.publicAddress == publicAddress));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, connect, listen, maxConnections,
      listenAddress, publicAddress);

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidConfigTCP(connect: $connect, listen: $listen, maxConnections: $maxConnections, listenAddress: $listenAddress, publicAddress: $publicAddress)';
  }
}

/// @nodoc
abstract mixin class _$VeilidConfigTCPCopyWith<$Res>
    implements $VeilidConfigTCPCopyWith<$Res> {
  factory _$VeilidConfigTCPCopyWith(
          _VeilidConfigTCP value, $Res Function(_VeilidConfigTCP) _then) =
      __$VeilidConfigTCPCopyWithImpl;
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
class __$VeilidConfigTCPCopyWithImpl<$Res>
    implements _$VeilidConfigTCPCopyWith<$Res> {
  __$VeilidConfigTCPCopyWithImpl(this._self, this._then);

  final _VeilidConfigTCP _self;
  final $Res Function(_VeilidConfigTCP) _then;

  /// Create a copy of VeilidConfigTCP
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? connect = null,
    Object? listen = null,
    Object? maxConnections = null,
    Object? listenAddress = null,
    Object? publicAddress = freezed,
  }) {
    return _then(_VeilidConfigTCP(
      connect: null == connect
          ? _self.connect
          : connect // ignore: cast_nullable_to_non_nullable
              as bool,
      listen: null == listen
          ? _self.listen
          : listen // ignore: cast_nullable_to_non_nullable
              as bool,
      maxConnections: null == maxConnections
          ? _self.maxConnections
          : maxConnections // ignore: cast_nullable_to_non_nullable
              as int,
      listenAddress: null == listenAddress
          ? _self.listenAddress
          : listenAddress // ignore: cast_nullable_to_non_nullable
              as String,
      publicAddress: freezed == publicAddress
          ? _self.publicAddress
          : publicAddress // ignore: cast_nullable_to_non_nullable
              as String?,
    ));
  }
}

/// @nodoc
mixin _$VeilidConfigWS implements DiagnosticableTreeMixin {
  bool get connect;
  bool get listen;
  int get maxConnections;
  String get listenAddress;
  String get path;
  String? get url;

  /// Create a copy of VeilidConfigWS
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $VeilidConfigWSCopyWith<VeilidConfigWS> get copyWith =>
      _$VeilidConfigWSCopyWithImpl<VeilidConfigWS>(
          this as VeilidConfigWS, _$identity);

  /// Serializes this VeilidConfigWS to a JSON map.
  Map<String, dynamic> toJson();

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
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
            other is VeilidConfigWS &&
            (identical(other.connect, connect) || other.connect == connect) &&
            (identical(other.listen, listen) || other.listen == listen) &&
            (identical(other.maxConnections, maxConnections) ||
                other.maxConnections == maxConnections) &&
            (identical(other.listenAddress, listenAddress) ||
                other.listenAddress == listenAddress) &&
            (identical(other.path, path) || other.path == path) &&
            (identical(other.url, url) || other.url == url));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(
      runtimeType, connect, listen, maxConnections, listenAddress, path, url);

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidConfigWS(connect: $connect, listen: $listen, maxConnections: $maxConnections, listenAddress: $listenAddress, path: $path, url: $url)';
  }
}

/// @nodoc
abstract mixin class $VeilidConfigWSCopyWith<$Res> {
  factory $VeilidConfigWSCopyWith(
          VeilidConfigWS value, $Res Function(VeilidConfigWS) _then) =
      _$VeilidConfigWSCopyWithImpl;
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
class _$VeilidConfigWSCopyWithImpl<$Res>
    implements $VeilidConfigWSCopyWith<$Res> {
  _$VeilidConfigWSCopyWithImpl(this._self, this._then);

  final VeilidConfigWS _self;
  final $Res Function(VeilidConfigWS) _then;

  /// Create a copy of VeilidConfigWS
  /// with the given fields replaced by the non-null parameter values.
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
    return _then(_self.copyWith(
      connect: null == connect
          ? _self.connect
          : connect // ignore: cast_nullable_to_non_nullable
              as bool,
      listen: null == listen
          ? _self.listen
          : listen // ignore: cast_nullable_to_non_nullable
              as bool,
      maxConnections: null == maxConnections
          ? _self.maxConnections
          : maxConnections // ignore: cast_nullable_to_non_nullable
              as int,
      listenAddress: null == listenAddress
          ? _self.listenAddress
          : listenAddress // ignore: cast_nullable_to_non_nullable
              as String,
      path: null == path
          ? _self.path
          : path // ignore: cast_nullable_to_non_nullable
              as String,
      url: freezed == url
          ? _self.url
          : url // ignore: cast_nullable_to_non_nullable
              as String?,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _VeilidConfigWS with DiagnosticableTreeMixin implements VeilidConfigWS {
  const _VeilidConfigWS(
      {required this.connect,
      required this.listen,
      required this.maxConnections,
      required this.listenAddress,
      required this.path,
      this.url});
  factory _VeilidConfigWS.fromJson(Map<String, dynamic> json) =>
      _$VeilidConfigWSFromJson(json);

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

  /// Create a copy of VeilidConfigWS
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  _$VeilidConfigWSCopyWith<_VeilidConfigWS> get copyWith =>
      __$VeilidConfigWSCopyWithImpl<_VeilidConfigWS>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$VeilidConfigWSToJson(
      this,
    );
  }

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
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
            other is _VeilidConfigWS &&
            (identical(other.connect, connect) || other.connect == connect) &&
            (identical(other.listen, listen) || other.listen == listen) &&
            (identical(other.maxConnections, maxConnections) ||
                other.maxConnections == maxConnections) &&
            (identical(other.listenAddress, listenAddress) ||
                other.listenAddress == listenAddress) &&
            (identical(other.path, path) || other.path == path) &&
            (identical(other.url, url) || other.url == url));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(
      runtimeType, connect, listen, maxConnections, listenAddress, path, url);

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidConfigWS(connect: $connect, listen: $listen, maxConnections: $maxConnections, listenAddress: $listenAddress, path: $path, url: $url)';
  }
}

/// @nodoc
abstract mixin class _$VeilidConfigWSCopyWith<$Res>
    implements $VeilidConfigWSCopyWith<$Res> {
  factory _$VeilidConfigWSCopyWith(
          _VeilidConfigWS value, $Res Function(_VeilidConfigWS) _then) =
      __$VeilidConfigWSCopyWithImpl;
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
class __$VeilidConfigWSCopyWithImpl<$Res>
    implements _$VeilidConfigWSCopyWith<$Res> {
  __$VeilidConfigWSCopyWithImpl(this._self, this._then);

  final _VeilidConfigWS _self;
  final $Res Function(_VeilidConfigWS) _then;

  /// Create a copy of VeilidConfigWS
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? connect = null,
    Object? listen = null,
    Object? maxConnections = null,
    Object? listenAddress = null,
    Object? path = null,
    Object? url = freezed,
  }) {
    return _then(_VeilidConfigWS(
      connect: null == connect
          ? _self.connect
          : connect // ignore: cast_nullable_to_non_nullable
              as bool,
      listen: null == listen
          ? _self.listen
          : listen // ignore: cast_nullable_to_non_nullable
              as bool,
      maxConnections: null == maxConnections
          ? _self.maxConnections
          : maxConnections // ignore: cast_nullable_to_non_nullable
              as int,
      listenAddress: null == listenAddress
          ? _self.listenAddress
          : listenAddress // ignore: cast_nullable_to_non_nullable
              as String,
      path: null == path
          ? _self.path
          : path // ignore: cast_nullable_to_non_nullable
              as String,
      url: freezed == url
          ? _self.url
          : url // ignore: cast_nullable_to_non_nullable
              as String?,
    ));
  }
}

/// @nodoc
mixin _$VeilidConfigWSS implements DiagnosticableTreeMixin {
  bool get connect;
  bool get listen;
  int get maxConnections;
  String get listenAddress;
  String get path;
  String? get url;

  /// Create a copy of VeilidConfigWSS
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $VeilidConfigWSSCopyWith<VeilidConfigWSS> get copyWith =>
      _$VeilidConfigWSSCopyWithImpl<VeilidConfigWSS>(
          this as VeilidConfigWSS, _$identity);

  /// Serializes this VeilidConfigWSS to a JSON map.
  Map<String, dynamic> toJson();

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
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
            other is VeilidConfigWSS &&
            (identical(other.connect, connect) || other.connect == connect) &&
            (identical(other.listen, listen) || other.listen == listen) &&
            (identical(other.maxConnections, maxConnections) ||
                other.maxConnections == maxConnections) &&
            (identical(other.listenAddress, listenAddress) ||
                other.listenAddress == listenAddress) &&
            (identical(other.path, path) || other.path == path) &&
            (identical(other.url, url) || other.url == url));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(
      runtimeType, connect, listen, maxConnections, listenAddress, path, url);

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidConfigWSS(connect: $connect, listen: $listen, maxConnections: $maxConnections, listenAddress: $listenAddress, path: $path, url: $url)';
  }
}

/// @nodoc
abstract mixin class $VeilidConfigWSSCopyWith<$Res> {
  factory $VeilidConfigWSSCopyWith(
          VeilidConfigWSS value, $Res Function(VeilidConfigWSS) _then) =
      _$VeilidConfigWSSCopyWithImpl;
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
class _$VeilidConfigWSSCopyWithImpl<$Res>
    implements $VeilidConfigWSSCopyWith<$Res> {
  _$VeilidConfigWSSCopyWithImpl(this._self, this._then);

  final VeilidConfigWSS _self;
  final $Res Function(VeilidConfigWSS) _then;

  /// Create a copy of VeilidConfigWSS
  /// with the given fields replaced by the non-null parameter values.
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
    return _then(_self.copyWith(
      connect: null == connect
          ? _self.connect
          : connect // ignore: cast_nullable_to_non_nullable
              as bool,
      listen: null == listen
          ? _self.listen
          : listen // ignore: cast_nullable_to_non_nullable
              as bool,
      maxConnections: null == maxConnections
          ? _self.maxConnections
          : maxConnections // ignore: cast_nullable_to_non_nullable
              as int,
      listenAddress: null == listenAddress
          ? _self.listenAddress
          : listenAddress // ignore: cast_nullable_to_non_nullable
              as String,
      path: null == path
          ? _self.path
          : path // ignore: cast_nullable_to_non_nullable
              as String,
      url: freezed == url
          ? _self.url
          : url // ignore: cast_nullable_to_non_nullable
              as String?,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _VeilidConfigWSS with DiagnosticableTreeMixin implements VeilidConfigWSS {
  const _VeilidConfigWSS(
      {required this.connect,
      required this.listen,
      required this.maxConnections,
      required this.listenAddress,
      required this.path,
      this.url});
  factory _VeilidConfigWSS.fromJson(Map<String, dynamic> json) =>
      _$VeilidConfigWSSFromJson(json);

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

  /// Create a copy of VeilidConfigWSS
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  _$VeilidConfigWSSCopyWith<_VeilidConfigWSS> get copyWith =>
      __$VeilidConfigWSSCopyWithImpl<_VeilidConfigWSS>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$VeilidConfigWSSToJson(
      this,
    );
  }

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
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
            other is _VeilidConfigWSS &&
            (identical(other.connect, connect) || other.connect == connect) &&
            (identical(other.listen, listen) || other.listen == listen) &&
            (identical(other.maxConnections, maxConnections) ||
                other.maxConnections == maxConnections) &&
            (identical(other.listenAddress, listenAddress) ||
                other.listenAddress == listenAddress) &&
            (identical(other.path, path) || other.path == path) &&
            (identical(other.url, url) || other.url == url));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(
      runtimeType, connect, listen, maxConnections, listenAddress, path, url);

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidConfigWSS(connect: $connect, listen: $listen, maxConnections: $maxConnections, listenAddress: $listenAddress, path: $path, url: $url)';
  }
}

/// @nodoc
abstract mixin class _$VeilidConfigWSSCopyWith<$Res>
    implements $VeilidConfigWSSCopyWith<$Res> {
  factory _$VeilidConfigWSSCopyWith(
          _VeilidConfigWSS value, $Res Function(_VeilidConfigWSS) _then) =
      __$VeilidConfigWSSCopyWithImpl;
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
class __$VeilidConfigWSSCopyWithImpl<$Res>
    implements _$VeilidConfigWSSCopyWith<$Res> {
  __$VeilidConfigWSSCopyWithImpl(this._self, this._then);

  final _VeilidConfigWSS _self;
  final $Res Function(_VeilidConfigWSS) _then;

  /// Create a copy of VeilidConfigWSS
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? connect = null,
    Object? listen = null,
    Object? maxConnections = null,
    Object? listenAddress = null,
    Object? path = null,
    Object? url = freezed,
  }) {
    return _then(_VeilidConfigWSS(
      connect: null == connect
          ? _self.connect
          : connect // ignore: cast_nullable_to_non_nullable
              as bool,
      listen: null == listen
          ? _self.listen
          : listen // ignore: cast_nullable_to_non_nullable
              as bool,
      maxConnections: null == maxConnections
          ? _self.maxConnections
          : maxConnections // ignore: cast_nullable_to_non_nullable
              as int,
      listenAddress: null == listenAddress
          ? _self.listenAddress
          : listenAddress // ignore: cast_nullable_to_non_nullable
              as String,
      path: null == path
          ? _self.path
          : path // ignore: cast_nullable_to_non_nullable
              as String,
      url: freezed == url
          ? _self.url
          : url // ignore: cast_nullable_to_non_nullable
              as String?,
    ));
  }
}

/// @nodoc
mixin _$VeilidConfigProtocol implements DiagnosticableTreeMixin {
  VeilidConfigUDP get udp;
  VeilidConfigTCP get tcp;
  VeilidConfigWS get ws;
  VeilidConfigWSS get wss;

  /// Create a copy of VeilidConfigProtocol
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $VeilidConfigProtocolCopyWith<VeilidConfigProtocol> get copyWith =>
      _$VeilidConfigProtocolCopyWithImpl<VeilidConfigProtocol>(
          this as VeilidConfigProtocol, _$identity);

  /// Serializes this VeilidConfigProtocol to a JSON map.
  Map<String, dynamic> toJson();

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
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
            other is VeilidConfigProtocol &&
            (identical(other.udp, udp) || other.udp == udp) &&
            (identical(other.tcp, tcp) || other.tcp == tcp) &&
            (identical(other.ws, ws) || other.ws == ws) &&
            (identical(other.wss, wss) || other.wss == wss));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, udp, tcp, ws, wss);

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidConfigProtocol(udp: $udp, tcp: $tcp, ws: $ws, wss: $wss)';
  }
}

/// @nodoc
abstract mixin class $VeilidConfigProtocolCopyWith<$Res> {
  factory $VeilidConfigProtocolCopyWith(VeilidConfigProtocol value,
          $Res Function(VeilidConfigProtocol) _then) =
      _$VeilidConfigProtocolCopyWithImpl;
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
class _$VeilidConfigProtocolCopyWithImpl<$Res>
    implements $VeilidConfigProtocolCopyWith<$Res> {
  _$VeilidConfigProtocolCopyWithImpl(this._self, this._then);

  final VeilidConfigProtocol _self;
  final $Res Function(VeilidConfigProtocol) _then;

  /// Create a copy of VeilidConfigProtocol
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? udp = null,
    Object? tcp = null,
    Object? ws = null,
    Object? wss = null,
  }) {
    return _then(_self.copyWith(
      udp: null == udp
          ? _self.udp
          : udp // ignore: cast_nullable_to_non_nullable
              as VeilidConfigUDP,
      tcp: null == tcp
          ? _self.tcp
          : tcp // ignore: cast_nullable_to_non_nullable
              as VeilidConfigTCP,
      ws: null == ws
          ? _self.ws
          : ws // ignore: cast_nullable_to_non_nullable
              as VeilidConfigWS,
      wss: null == wss
          ? _self.wss
          : wss // ignore: cast_nullable_to_non_nullable
              as VeilidConfigWSS,
    ));
  }

  /// Create a copy of VeilidConfigProtocol
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $VeilidConfigUDPCopyWith<$Res> get udp {
    return $VeilidConfigUDPCopyWith<$Res>(_self.udp, (value) {
      return _then(_self.copyWith(udp: value));
    });
  }

  /// Create a copy of VeilidConfigProtocol
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $VeilidConfigTCPCopyWith<$Res> get tcp {
    return $VeilidConfigTCPCopyWith<$Res>(_self.tcp, (value) {
      return _then(_self.copyWith(tcp: value));
    });
  }

  /// Create a copy of VeilidConfigProtocol
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $VeilidConfigWSCopyWith<$Res> get ws {
    return $VeilidConfigWSCopyWith<$Res>(_self.ws, (value) {
      return _then(_self.copyWith(ws: value));
    });
  }

  /// Create a copy of VeilidConfigProtocol
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $VeilidConfigWSSCopyWith<$Res> get wss {
    return $VeilidConfigWSSCopyWith<$Res>(_self.wss, (value) {
      return _then(_self.copyWith(wss: value));
    });
  }
}

/// @nodoc
@JsonSerializable()
class _VeilidConfigProtocol
    with DiagnosticableTreeMixin
    implements VeilidConfigProtocol {
  const _VeilidConfigProtocol(
      {required this.udp,
      required this.tcp,
      required this.ws,
      required this.wss});
  factory _VeilidConfigProtocol.fromJson(Map<String, dynamic> json) =>
      _$VeilidConfigProtocolFromJson(json);

  @override
  final VeilidConfigUDP udp;
  @override
  final VeilidConfigTCP tcp;
  @override
  final VeilidConfigWS ws;
  @override
  final VeilidConfigWSS wss;

  /// Create a copy of VeilidConfigProtocol
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  _$VeilidConfigProtocolCopyWith<_VeilidConfigProtocol> get copyWith =>
      __$VeilidConfigProtocolCopyWithImpl<_VeilidConfigProtocol>(
          this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$VeilidConfigProtocolToJson(
      this,
    );
  }

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
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
            other is _VeilidConfigProtocol &&
            (identical(other.udp, udp) || other.udp == udp) &&
            (identical(other.tcp, tcp) || other.tcp == tcp) &&
            (identical(other.ws, ws) || other.ws == ws) &&
            (identical(other.wss, wss) || other.wss == wss));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, udp, tcp, ws, wss);

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidConfigProtocol(udp: $udp, tcp: $tcp, ws: $ws, wss: $wss)';
  }
}

/// @nodoc
abstract mixin class _$VeilidConfigProtocolCopyWith<$Res>
    implements $VeilidConfigProtocolCopyWith<$Res> {
  factory _$VeilidConfigProtocolCopyWith(_VeilidConfigProtocol value,
          $Res Function(_VeilidConfigProtocol) _then) =
      __$VeilidConfigProtocolCopyWithImpl;
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
class __$VeilidConfigProtocolCopyWithImpl<$Res>
    implements _$VeilidConfigProtocolCopyWith<$Res> {
  __$VeilidConfigProtocolCopyWithImpl(this._self, this._then);

  final _VeilidConfigProtocol _self;
  final $Res Function(_VeilidConfigProtocol) _then;

  /// Create a copy of VeilidConfigProtocol
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? udp = null,
    Object? tcp = null,
    Object? ws = null,
    Object? wss = null,
  }) {
    return _then(_VeilidConfigProtocol(
      udp: null == udp
          ? _self.udp
          : udp // ignore: cast_nullable_to_non_nullable
              as VeilidConfigUDP,
      tcp: null == tcp
          ? _self.tcp
          : tcp // ignore: cast_nullable_to_non_nullable
              as VeilidConfigTCP,
      ws: null == ws
          ? _self.ws
          : ws // ignore: cast_nullable_to_non_nullable
              as VeilidConfigWS,
      wss: null == wss
          ? _self.wss
          : wss // ignore: cast_nullable_to_non_nullable
              as VeilidConfigWSS,
    ));
  }

  /// Create a copy of VeilidConfigProtocol
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $VeilidConfigUDPCopyWith<$Res> get udp {
    return $VeilidConfigUDPCopyWith<$Res>(_self.udp, (value) {
      return _then(_self.copyWith(udp: value));
    });
  }

  /// Create a copy of VeilidConfigProtocol
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $VeilidConfigTCPCopyWith<$Res> get tcp {
    return $VeilidConfigTCPCopyWith<$Res>(_self.tcp, (value) {
      return _then(_self.copyWith(tcp: value));
    });
  }

  /// Create a copy of VeilidConfigProtocol
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $VeilidConfigWSCopyWith<$Res> get ws {
    return $VeilidConfigWSCopyWith<$Res>(_self.ws, (value) {
      return _then(_self.copyWith(ws: value));
    });
  }

  /// Create a copy of VeilidConfigProtocol
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $VeilidConfigWSSCopyWith<$Res> get wss {
    return $VeilidConfigWSSCopyWith<$Res>(_self.wss, (value) {
      return _then(_self.copyWith(wss: value));
    });
  }
}

/// @nodoc
mixin _$VeilidConfigTLS implements DiagnosticableTreeMixin {
  String get certificatePath;
  String get privateKeyPath;
  int get connectionInitialTimeoutMs;

  /// Create a copy of VeilidConfigTLS
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $VeilidConfigTLSCopyWith<VeilidConfigTLS> get copyWith =>
      _$VeilidConfigTLSCopyWithImpl<VeilidConfigTLS>(
          this as VeilidConfigTLS, _$identity);

  /// Serializes this VeilidConfigTLS to a JSON map.
  Map<String, dynamic> toJson();

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
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
            other is VeilidConfigTLS &&
            (identical(other.certificatePath, certificatePath) ||
                other.certificatePath == certificatePath) &&
            (identical(other.privateKeyPath, privateKeyPath) ||
                other.privateKeyPath == privateKeyPath) &&
            (identical(other.connectionInitialTimeoutMs,
                    connectionInitialTimeoutMs) ||
                other.connectionInitialTimeoutMs ==
                    connectionInitialTimeoutMs));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(
      runtimeType, certificatePath, privateKeyPath, connectionInitialTimeoutMs);

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidConfigTLS(certificatePath: $certificatePath, privateKeyPath: $privateKeyPath, connectionInitialTimeoutMs: $connectionInitialTimeoutMs)';
  }
}

/// @nodoc
abstract mixin class $VeilidConfigTLSCopyWith<$Res> {
  factory $VeilidConfigTLSCopyWith(
          VeilidConfigTLS value, $Res Function(VeilidConfigTLS) _then) =
      _$VeilidConfigTLSCopyWithImpl;
  @useResult
  $Res call(
      {String certificatePath,
      String privateKeyPath,
      int connectionInitialTimeoutMs});
}

/// @nodoc
class _$VeilidConfigTLSCopyWithImpl<$Res>
    implements $VeilidConfigTLSCopyWith<$Res> {
  _$VeilidConfigTLSCopyWithImpl(this._self, this._then);

  final VeilidConfigTLS _self;
  final $Res Function(VeilidConfigTLS) _then;

  /// Create a copy of VeilidConfigTLS
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? certificatePath = null,
    Object? privateKeyPath = null,
    Object? connectionInitialTimeoutMs = null,
  }) {
    return _then(_self.copyWith(
      certificatePath: null == certificatePath
          ? _self.certificatePath
          : certificatePath // ignore: cast_nullable_to_non_nullable
              as String,
      privateKeyPath: null == privateKeyPath
          ? _self.privateKeyPath
          : privateKeyPath // ignore: cast_nullable_to_non_nullable
              as String,
      connectionInitialTimeoutMs: null == connectionInitialTimeoutMs
          ? _self.connectionInitialTimeoutMs
          : connectionInitialTimeoutMs // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _VeilidConfigTLS with DiagnosticableTreeMixin implements VeilidConfigTLS {
  const _VeilidConfigTLS(
      {required this.certificatePath,
      required this.privateKeyPath,
      required this.connectionInitialTimeoutMs});
  factory _VeilidConfigTLS.fromJson(Map<String, dynamic> json) =>
      _$VeilidConfigTLSFromJson(json);

  @override
  final String certificatePath;
  @override
  final String privateKeyPath;
  @override
  final int connectionInitialTimeoutMs;

  /// Create a copy of VeilidConfigTLS
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  _$VeilidConfigTLSCopyWith<_VeilidConfigTLS> get copyWith =>
      __$VeilidConfigTLSCopyWithImpl<_VeilidConfigTLS>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$VeilidConfigTLSToJson(
      this,
    );
  }

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
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
            other is _VeilidConfigTLS &&
            (identical(other.certificatePath, certificatePath) ||
                other.certificatePath == certificatePath) &&
            (identical(other.privateKeyPath, privateKeyPath) ||
                other.privateKeyPath == privateKeyPath) &&
            (identical(other.connectionInitialTimeoutMs,
                    connectionInitialTimeoutMs) ||
                other.connectionInitialTimeoutMs ==
                    connectionInitialTimeoutMs));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(
      runtimeType, certificatePath, privateKeyPath, connectionInitialTimeoutMs);

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidConfigTLS(certificatePath: $certificatePath, privateKeyPath: $privateKeyPath, connectionInitialTimeoutMs: $connectionInitialTimeoutMs)';
  }
}

/// @nodoc
abstract mixin class _$VeilidConfigTLSCopyWith<$Res>
    implements $VeilidConfigTLSCopyWith<$Res> {
  factory _$VeilidConfigTLSCopyWith(
          _VeilidConfigTLS value, $Res Function(_VeilidConfigTLS) _then) =
      __$VeilidConfigTLSCopyWithImpl;
  @override
  @useResult
  $Res call(
      {String certificatePath,
      String privateKeyPath,
      int connectionInitialTimeoutMs});
}

/// @nodoc
class __$VeilidConfigTLSCopyWithImpl<$Res>
    implements _$VeilidConfigTLSCopyWith<$Res> {
  __$VeilidConfigTLSCopyWithImpl(this._self, this._then);

  final _VeilidConfigTLS _self;
  final $Res Function(_VeilidConfigTLS) _then;

  /// Create a copy of VeilidConfigTLS
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? certificatePath = null,
    Object? privateKeyPath = null,
    Object? connectionInitialTimeoutMs = null,
  }) {
    return _then(_VeilidConfigTLS(
      certificatePath: null == certificatePath
          ? _self.certificatePath
          : certificatePath // ignore: cast_nullable_to_non_nullable
              as String,
      privateKeyPath: null == privateKeyPath
          ? _self.privateKeyPath
          : privateKeyPath // ignore: cast_nullable_to_non_nullable
              as String,
      connectionInitialTimeoutMs: null == connectionInitialTimeoutMs
          ? _self.connectionInitialTimeoutMs
          : connectionInitialTimeoutMs // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc
mixin _$VeilidConfigDHT implements DiagnosticableTreeMixin {
  int get resolveNodeTimeoutMs;
  int get resolveNodeCount;
  int get resolveNodeFanout;
  int get maxFindNodeCount;
  int get getValueTimeoutMs;
  int get getValueCount;
  int get getValueFanout;
  int get setValueTimeoutMs;
  int get setValueCount;
  int get setValueFanout;
  int get minPeerCount;
  int get minPeerRefreshTimeMs;
  int get validateDialInfoReceiptTimeMs;
  int get localSubkeyCacheSize;
  int get localMaxSubkeyCacheMemoryMb;
  int get remoteSubkeyCacheSize;
  int get remoteMaxRecords;
  int get remoteMaxSubkeyCacheMemoryMb;
  int get remoteMaxStorageSpaceMb;
  int get publicWatchLimit;
  int get memberWatchLimit;
  int get maxWatchExpirationMs;

  /// Create a copy of VeilidConfigDHT
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $VeilidConfigDHTCopyWith<VeilidConfigDHT> get copyWith =>
      _$VeilidConfigDHTCopyWithImpl<VeilidConfigDHT>(
          this as VeilidConfigDHT, _$identity);

  /// Serializes this VeilidConfigDHT to a JSON map.
  Map<String, dynamic> toJson();

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
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
            other is VeilidConfigDHT &&
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

  @JsonKey(includeFromJson: false, includeToJson: false)
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

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidConfigDHT(resolveNodeTimeoutMs: $resolveNodeTimeoutMs, resolveNodeCount: $resolveNodeCount, resolveNodeFanout: $resolveNodeFanout, maxFindNodeCount: $maxFindNodeCount, getValueTimeoutMs: $getValueTimeoutMs, getValueCount: $getValueCount, getValueFanout: $getValueFanout, setValueTimeoutMs: $setValueTimeoutMs, setValueCount: $setValueCount, setValueFanout: $setValueFanout, minPeerCount: $minPeerCount, minPeerRefreshTimeMs: $minPeerRefreshTimeMs, validateDialInfoReceiptTimeMs: $validateDialInfoReceiptTimeMs, localSubkeyCacheSize: $localSubkeyCacheSize, localMaxSubkeyCacheMemoryMb: $localMaxSubkeyCacheMemoryMb, remoteSubkeyCacheSize: $remoteSubkeyCacheSize, remoteMaxRecords: $remoteMaxRecords, remoteMaxSubkeyCacheMemoryMb: $remoteMaxSubkeyCacheMemoryMb, remoteMaxStorageSpaceMb: $remoteMaxStorageSpaceMb, publicWatchLimit: $publicWatchLimit, memberWatchLimit: $memberWatchLimit, maxWatchExpirationMs: $maxWatchExpirationMs)';
  }
}

/// @nodoc
abstract mixin class $VeilidConfigDHTCopyWith<$Res> {
  factory $VeilidConfigDHTCopyWith(
          VeilidConfigDHT value, $Res Function(VeilidConfigDHT) _then) =
      _$VeilidConfigDHTCopyWithImpl;
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
class _$VeilidConfigDHTCopyWithImpl<$Res>
    implements $VeilidConfigDHTCopyWith<$Res> {
  _$VeilidConfigDHTCopyWithImpl(this._self, this._then);

  final VeilidConfigDHT _self;
  final $Res Function(VeilidConfigDHT) _then;

  /// Create a copy of VeilidConfigDHT
  /// with the given fields replaced by the non-null parameter values.
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
    return _then(_self.copyWith(
      resolveNodeTimeoutMs: null == resolveNodeTimeoutMs
          ? _self.resolveNodeTimeoutMs
          : resolveNodeTimeoutMs // ignore: cast_nullable_to_non_nullable
              as int,
      resolveNodeCount: null == resolveNodeCount
          ? _self.resolveNodeCount
          : resolveNodeCount // ignore: cast_nullable_to_non_nullable
              as int,
      resolveNodeFanout: null == resolveNodeFanout
          ? _self.resolveNodeFanout
          : resolveNodeFanout // ignore: cast_nullable_to_non_nullable
              as int,
      maxFindNodeCount: null == maxFindNodeCount
          ? _self.maxFindNodeCount
          : maxFindNodeCount // ignore: cast_nullable_to_non_nullable
              as int,
      getValueTimeoutMs: null == getValueTimeoutMs
          ? _self.getValueTimeoutMs
          : getValueTimeoutMs // ignore: cast_nullable_to_non_nullable
              as int,
      getValueCount: null == getValueCount
          ? _self.getValueCount
          : getValueCount // ignore: cast_nullable_to_non_nullable
              as int,
      getValueFanout: null == getValueFanout
          ? _self.getValueFanout
          : getValueFanout // ignore: cast_nullable_to_non_nullable
              as int,
      setValueTimeoutMs: null == setValueTimeoutMs
          ? _self.setValueTimeoutMs
          : setValueTimeoutMs // ignore: cast_nullable_to_non_nullable
              as int,
      setValueCount: null == setValueCount
          ? _self.setValueCount
          : setValueCount // ignore: cast_nullable_to_non_nullable
              as int,
      setValueFanout: null == setValueFanout
          ? _self.setValueFanout
          : setValueFanout // ignore: cast_nullable_to_non_nullable
              as int,
      minPeerCount: null == minPeerCount
          ? _self.minPeerCount
          : minPeerCount // ignore: cast_nullable_to_non_nullable
              as int,
      minPeerRefreshTimeMs: null == minPeerRefreshTimeMs
          ? _self.minPeerRefreshTimeMs
          : minPeerRefreshTimeMs // ignore: cast_nullable_to_non_nullable
              as int,
      validateDialInfoReceiptTimeMs: null == validateDialInfoReceiptTimeMs
          ? _self.validateDialInfoReceiptTimeMs
          : validateDialInfoReceiptTimeMs // ignore: cast_nullable_to_non_nullable
              as int,
      localSubkeyCacheSize: null == localSubkeyCacheSize
          ? _self.localSubkeyCacheSize
          : localSubkeyCacheSize // ignore: cast_nullable_to_non_nullable
              as int,
      localMaxSubkeyCacheMemoryMb: null == localMaxSubkeyCacheMemoryMb
          ? _self.localMaxSubkeyCacheMemoryMb
          : localMaxSubkeyCacheMemoryMb // ignore: cast_nullable_to_non_nullable
              as int,
      remoteSubkeyCacheSize: null == remoteSubkeyCacheSize
          ? _self.remoteSubkeyCacheSize
          : remoteSubkeyCacheSize // ignore: cast_nullable_to_non_nullable
              as int,
      remoteMaxRecords: null == remoteMaxRecords
          ? _self.remoteMaxRecords
          : remoteMaxRecords // ignore: cast_nullable_to_non_nullable
              as int,
      remoteMaxSubkeyCacheMemoryMb: null == remoteMaxSubkeyCacheMemoryMb
          ? _self.remoteMaxSubkeyCacheMemoryMb
          : remoteMaxSubkeyCacheMemoryMb // ignore: cast_nullable_to_non_nullable
              as int,
      remoteMaxStorageSpaceMb: null == remoteMaxStorageSpaceMb
          ? _self.remoteMaxStorageSpaceMb
          : remoteMaxStorageSpaceMb // ignore: cast_nullable_to_non_nullable
              as int,
      publicWatchLimit: null == publicWatchLimit
          ? _self.publicWatchLimit
          : publicWatchLimit // ignore: cast_nullable_to_non_nullable
              as int,
      memberWatchLimit: null == memberWatchLimit
          ? _self.memberWatchLimit
          : memberWatchLimit // ignore: cast_nullable_to_non_nullable
              as int,
      maxWatchExpirationMs: null == maxWatchExpirationMs
          ? _self.maxWatchExpirationMs
          : maxWatchExpirationMs // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _VeilidConfigDHT with DiagnosticableTreeMixin implements VeilidConfigDHT {
  const _VeilidConfigDHT(
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
  factory _VeilidConfigDHT.fromJson(Map<String, dynamic> json) =>
      _$VeilidConfigDHTFromJson(json);

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

  /// Create a copy of VeilidConfigDHT
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  _$VeilidConfigDHTCopyWith<_VeilidConfigDHT> get copyWith =>
      __$VeilidConfigDHTCopyWithImpl<_VeilidConfigDHT>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$VeilidConfigDHTToJson(
      this,
    );
  }

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
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
            other is _VeilidConfigDHT &&
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

  @JsonKey(includeFromJson: false, includeToJson: false)
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

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidConfigDHT(resolveNodeTimeoutMs: $resolveNodeTimeoutMs, resolveNodeCount: $resolveNodeCount, resolveNodeFanout: $resolveNodeFanout, maxFindNodeCount: $maxFindNodeCount, getValueTimeoutMs: $getValueTimeoutMs, getValueCount: $getValueCount, getValueFanout: $getValueFanout, setValueTimeoutMs: $setValueTimeoutMs, setValueCount: $setValueCount, setValueFanout: $setValueFanout, minPeerCount: $minPeerCount, minPeerRefreshTimeMs: $minPeerRefreshTimeMs, validateDialInfoReceiptTimeMs: $validateDialInfoReceiptTimeMs, localSubkeyCacheSize: $localSubkeyCacheSize, localMaxSubkeyCacheMemoryMb: $localMaxSubkeyCacheMemoryMb, remoteSubkeyCacheSize: $remoteSubkeyCacheSize, remoteMaxRecords: $remoteMaxRecords, remoteMaxSubkeyCacheMemoryMb: $remoteMaxSubkeyCacheMemoryMb, remoteMaxStorageSpaceMb: $remoteMaxStorageSpaceMb, publicWatchLimit: $publicWatchLimit, memberWatchLimit: $memberWatchLimit, maxWatchExpirationMs: $maxWatchExpirationMs)';
  }
}

/// @nodoc
abstract mixin class _$VeilidConfigDHTCopyWith<$Res>
    implements $VeilidConfigDHTCopyWith<$Res> {
  factory _$VeilidConfigDHTCopyWith(
          _VeilidConfigDHT value, $Res Function(_VeilidConfigDHT) _then) =
      __$VeilidConfigDHTCopyWithImpl;
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
class __$VeilidConfigDHTCopyWithImpl<$Res>
    implements _$VeilidConfigDHTCopyWith<$Res> {
  __$VeilidConfigDHTCopyWithImpl(this._self, this._then);

  final _VeilidConfigDHT _self;
  final $Res Function(_VeilidConfigDHT) _then;

  /// Create a copy of VeilidConfigDHT
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
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
    return _then(_VeilidConfigDHT(
      resolveNodeTimeoutMs: null == resolveNodeTimeoutMs
          ? _self.resolveNodeTimeoutMs
          : resolveNodeTimeoutMs // ignore: cast_nullable_to_non_nullable
              as int,
      resolveNodeCount: null == resolveNodeCount
          ? _self.resolveNodeCount
          : resolveNodeCount // ignore: cast_nullable_to_non_nullable
              as int,
      resolveNodeFanout: null == resolveNodeFanout
          ? _self.resolveNodeFanout
          : resolveNodeFanout // ignore: cast_nullable_to_non_nullable
              as int,
      maxFindNodeCount: null == maxFindNodeCount
          ? _self.maxFindNodeCount
          : maxFindNodeCount // ignore: cast_nullable_to_non_nullable
              as int,
      getValueTimeoutMs: null == getValueTimeoutMs
          ? _self.getValueTimeoutMs
          : getValueTimeoutMs // ignore: cast_nullable_to_non_nullable
              as int,
      getValueCount: null == getValueCount
          ? _self.getValueCount
          : getValueCount // ignore: cast_nullable_to_non_nullable
              as int,
      getValueFanout: null == getValueFanout
          ? _self.getValueFanout
          : getValueFanout // ignore: cast_nullable_to_non_nullable
              as int,
      setValueTimeoutMs: null == setValueTimeoutMs
          ? _self.setValueTimeoutMs
          : setValueTimeoutMs // ignore: cast_nullable_to_non_nullable
              as int,
      setValueCount: null == setValueCount
          ? _self.setValueCount
          : setValueCount // ignore: cast_nullable_to_non_nullable
              as int,
      setValueFanout: null == setValueFanout
          ? _self.setValueFanout
          : setValueFanout // ignore: cast_nullable_to_non_nullable
              as int,
      minPeerCount: null == minPeerCount
          ? _self.minPeerCount
          : minPeerCount // ignore: cast_nullable_to_non_nullable
              as int,
      minPeerRefreshTimeMs: null == minPeerRefreshTimeMs
          ? _self.minPeerRefreshTimeMs
          : minPeerRefreshTimeMs // ignore: cast_nullable_to_non_nullable
              as int,
      validateDialInfoReceiptTimeMs: null == validateDialInfoReceiptTimeMs
          ? _self.validateDialInfoReceiptTimeMs
          : validateDialInfoReceiptTimeMs // ignore: cast_nullable_to_non_nullable
              as int,
      localSubkeyCacheSize: null == localSubkeyCacheSize
          ? _self.localSubkeyCacheSize
          : localSubkeyCacheSize // ignore: cast_nullable_to_non_nullable
              as int,
      localMaxSubkeyCacheMemoryMb: null == localMaxSubkeyCacheMemoryMb
          ? _self.localMaxSubkeyCacheMemoryMb
          : localMaxSubkeyCacheMemoryMb // ignore: cast_nullable_to_non_nullable
              as int,
      remoteSubkeyCacheSize: null == remoteSubkeyCacheSize
          ? _self.remoteSubkeyCacheSize
          : remoteSubkeyCacheSize // ignore: cast_nullable_to_non_nullable
              as int,
      remoteMaxRecords: null == remoteMaxRecords
          ? _self.remoteMaxRecords
          : remoteMaxRecords // ignore: cast_nullable_to_non_nullable
              as int,
      remoteMaxSubkeyCacheMemoryMb: null == remoteMaxSubkeyCacheMemoryMb
          ? _self.remoteMaxSubkeyCacheMemoryMb
          : remoteMaxSubkeyCacheMemoryMb // ignore: cast_nullable_to_non_nullable
              as int,
      remoteMaxStorageSpaceMb: null == remoteMaxStorageSpaceMb
          ? _self.remoteMaxStorageSpaceMb
          : remoteMaxStorageSpaceMb // ignore: cast_nullable_to_non_nullable
              as int,
      publicWatchLimit: null == publicWatchLimit
          ? _self.publicWatchLimit
          : publicWatchLimit // ignore: cast_nullable_to_non_nullable
              as int,
      memberWatchLimit: null == memberWatchLimit
          ? _self.memberWatchLimit
          : memberWatchLimit // ignore: cast_nullable_to_non_nullable
              as int,
      maxWatchExpirationMs: null == maxWatchExpirationMs
          ? _self.maxWatchExpirationMs
          : maxWatchExpirationMs // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc
mixin _$VeilidConfigRPC implements DiagnosticableTreeMixin {
  int get concurrency;
  int get queueSize;
  int get timeoutMs;
  int get maxRouteHopCount;
  int get defaultRouteHopCount;
  int? get maxTimestampBehindMs;
  int? get maxTimestampAheadMs;

  /// Create a copy of VeilidConfigRPC
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $VeilidConfigRPCCopyWith<VeilidConfigRPC> get copyWith =>
      _$VeilidConfigRPCCopyWithImpl<VeilidConfigRPC>(
          this as VeilidConfigRPC, _$identity);

  /// Serializes this VeilidConfigRPC to a JSON map.
  Map<String, dynamic> toJson();

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
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
            other is VeilidConfigRPC &&
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

  @JsonKey(includeFromJson: false, includeToJson: false)
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

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidConfigRPC(concurrency: $concurrency, queueSize: $queueSize, timeoutMs: $timeoutMs, maxRouteHopCount: $maxRouteHopCount, defaultRouteHopCount: $defaultRouteHopCount, maxTimestampBehindMs: $maxTimestampBehindMs, maxTimestampAheadMs: $maxTimestampAheadMs)';
  }
}

/// @nodoc
abstract mixin class $VeilidConfigRPCCopyWith<$Res> {
  factory $VeilidConfigRPCCopyWith(
          VeilidConfigRPC value, $Res Function(VeilidConfigRPC) _then) =
      _$VeilidConfigRPCCopyWithImpl;
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
class _$VeilidConfigRPCCopyWithImpl<$Res>
    implements $VeilidConfigRPCCopyWith<$Res> {
  _$VeilidConfigRPCCopyWithImpl(this._self, this._then);

  final VeilidConfigRPC _self;
  final $Res Function(VeilidConfigRPC) _then;

  /// Create a copy of VeilidConfigRPC
  /// with the given fields replaced by the non-null parameter values.
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
    return _then(_self.copyWith(
      concurrency: null == concurrency
          ? _self.concurrency
          : concurrency // ignore: cast_nullable_to_non_nullable
              as int,
      queueSize: null == queueSize
          ? _self.queueSize
          : queueSize // ignore: cast_nullable_to_non_nullable
              as int,
      timeoutMs: null == timeoutMs
          ? _self.timeoutMs
          : timeoutMs // ignore: cast_nullable_to_non_nullable
              as int,
      maxRouteHopCount: null == maxRouteHopCount
          ? _self.maxRouteHopCount
          : maxRouteHopCount // ignore: cast_nullable_to_non_nullable
              as int,
      defaultRouteHopCount: null == defaultRouteHopCount
          ? _self.defaultRouteHopCount
          : defaultRouteHopCount // ignore: cast_nullable_to_non_nullable
              as int,
      maxTimestampBehindMs: freezed == maxTimestampBehindMs
          ? _self.maxTimestampBehindMs
          : maxTimestampBehindMs // ignore: cast_nullable_to_non_nullable
              as int?,
      maxTimestampAheadMs: freezed == maxTimestampAheadMs
          ? _self.maxTimestampAheadMs
          : maxTimestampAheadMs // ignore: cast_nullable_to_non_nullable
              as int?,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _VeilidConfigRPC with DiagnosticableTreeMixin implements VeilidConfigRPC {
  const _VeilidConfigRPC(
      {required this.concurrency,
      required this.queueSize,
      required this.timeoutMs,
      required this.maxRouteHopCount,
      required this.defaultRouteHopCount,
      this.maxTimestampBehindMs,
      this.maxTimestampAheadMs});
  factory _VeilidConfigRPC.fromJson(Map<String, dynamic> json) =>
      _$VeilidConfigRPCFromJson(json);

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

  /// Create a copy of VeilidConfigRPC
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  _$VeilidConfigRPCCopyWith<_VeilidConfigRPC> get copyWith =>
      __$VeilidConfigRPCCopyWithImpl<_VeilidConfigRPC>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$VeilidConfigRPCToJson(
      this,
    );
  }

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
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
            other is _VeilidConfigRPC &&
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

  @JsonKey(includeFromJson: false, includeToJson: false)
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

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidConfigRPC(concurrency: $concurrency, queueSize: $queueSize, timeoutMs: $timeoutMs, maxRouteHopCount: $maxRouteHopCount, defaultRouteHopCount: $defaultRouteHopCount, maxTimestampBehindMs: $maxTimestampBehindMs, maxTimestampAheadMs: $maxTimestampAheadMs)';
  }
}

/// @nodoc
abstract mixin class _$VeilidConfigRPCCopyWith<$Res>
    implements $VeilidConfigRPCCopyWith<$Res> {
  factory _$VeilidConfigRPCCopyWith(
          _VeilidConfigRPC value, $Res Function(_VeilidConfigRPC) _then) =
      __$VeilidConfigRPCCopyWithImpl;
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
class __$VeilidConfigRPCCopyWithImpl<$Res>
    implements _$VeilidConfigRPCCopyWith<$Res> {
  __$VeilidConfigRPCCopyWithImpl(this._self, this._then);

  final _VeilidConfigRPC _self;
  final $Res Function(_VeilidConfigRPC) _then;

  /// Create a copy of VeilidConfigRPC
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? concurrency = null,
    Object? queueSize = null,
    Object? timeoutMs = null,
    Object? maxRouteHopCount = null,
    Object? defaultRouteHopCount = null,
    Object? maxTimestampBehindMs = freezed,
    Object? maxTimestampAheadMs = freezed,
  }) {
    return _then(_VeilidConfigRPC(
      concurrency: null == concurrency
          ? _self.concurrency
          : concurrency // ignore: cast_nullable_to_non_nullable
              as int,
      queueSize: null == queueSize
          ? _self.queueSize
          : queueSize // ignore: cast_nullable_to_non_nullable
              as int,
      timeoutMs: null == timeoutMs
          ? _self.timeoutMs
          : timeoutMs // ignore: cast_nullable_to_non_nullable
              as int,
      maxRouteHopCount: null == maxRouteHopCount
          ? _self.maxRouteHopCount
          : maxRouteHopCount // ignore: cast_nullable_to_non_nullable
              as int,
      defaultRouteHopCount: null == defaultRouteHopCount
          ? _self.defaultRouteHopCount
          : defaultRouteHopCount // ignore: cast_nullable_to_non_nullable
              as int,
      maxTimestampBehindMs: freezed == maxTimestampBehindMs
          ? _self.maxTimestampBehindMs
          : maxTimestampBehindMs // ignore: cast_nullable_to_non_nullable
              as int?,
      maxTimestampAheadMs: freezed == maxTimestampAheadMs
          ? _self.maxTimestampAheadMs
          : maxTimestampAheadMs // ignore: cast_nullable_to_non_nullable
              as int?,
    ));
  }
}

/// @nodoc
mixin _$VeilidConfigRoutingTable implements DiagnosticableTreeMixin {
  List<TypedKey> get nodeId;
  List<TypedSecret> get nodeIdSecret;
  List<String> get bootstrap;
  List<TypedKey> get bootstrapKeys;
  int get limitOverAttached;
  int get limitFullyAttached;
  int get limitAttachedStrong;
  int get limitAttachedGood;
  int get limitAttachedWeak;

  /// Create a copy of VeilidConfigRoutingTable
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $VeilidConfigRoutingTableCopyWith<VeilidConfigRoutingTable> get copyWith =>
      _$VeilidConfigRoutingTableCopyWithImpl<VeilidConfigRoutingTable>(
          this as VeilidConfigRoutingTable, _$identity);

  /// Serializes this VeilidConfigRoutingTable to a JSON map.
  Map<String, dynamic> toJson();

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
    properties
      ..add(DiagnosticsProperty('type', 'VeilidConfigRoutingTable'))
      ..add(DiagnosticsProperty('nodeId', nodeId))
      ..add(DiagnosticsProperty('nodeIdSecret', nodeIdSecret))
      ..add(DiagnosticsProperty('bootstrap', bootstrap))
      ..add(DiagnosticsProperty('bootstrapKeys', bootstrapKeys))
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
            other is VeilidConfigRoutingTable &&
            const DeepCollectionEquality().equals(other.nodeId, nodeId) &&
            const DeepCollectionEquality()
                .equals(other.nodeIdSecret, nodeIdSecret) &&
            const DeepCollectionEquality().equals(other.bootstrap, bootstrap) &&
            const DeepCollectionEquality()
                .equals(other.bootstrapKeys, bootstrapKeys) &&
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

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(
      runtimeType,
      const DeepCollectionEquality().hash(nodeId),
      const DeepCollectionEquality().hash(nodeIdSecret),
      const DeepCollectionEquality().hash(bootstrap),
      const DeepCollectionEquality().hash(bootstrapKeys),
      limitOverAttached,
      limitFullyAttached,
      limitAttachedStrong,
      limitAttachedGood,
      limitAttachedWeak);

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidConfigRoutingTable(nodeId: $nodeId, nodeIdSecret: $nodeIdSecret, bootstrap: $bootstrap, bootstrapKeys: $bootstrapKeys, limitOverAttached: $limitOverAttached, limitFullyAttached: $limitFullyAttached, limitAttachedStrong: $limitAttachedStrong, limitAttachedGood: $limitAttachedGood, limitAttachedWeak: $limitAttachedWeak)';
  }
}

/// @nodoc
abstract mixin class $VeilidConfigRoutingTableCopyWith<$Res> {
  factory $VeilidConfigRoutingTableCopyWith(VeilidConfigRoutingTable value,
          $Res Function(VeilidConfigRoutingTable) _then) =
      _$VeilidConfigRoutingTableCopyWithImpl;
  @useResult
  $Res call(
      {List<TypedKey> nodeId,
      List<TypedSecret> nodeIdSecret,
      List<String> bootstrap,
      List<TypedKey> bootstrapKeys,
      int limitOverAttached,
      int limitFullyAttached,
      int limitAttachedStrong,
      int limitAttachedGood,
      int limitAttachedWeak});
}

/// @nodoc
class _$VeilidConfigRoutingTableCopyWithImpl<$Res>
    implements $VeilidConfigRoutingTableCopyWith<$Res> {
  _$VeilidConfigRoutingTableCopyWithImpl(this._self, this._then);

  final VeilidConfigRoutingTable _self;
  final $Res Function(VeilidConfigRoutingTable) _then;

  /// Create a copy of VeilidConfigRoutingTable
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? nodeId = null,
    Object? nodeIdSecret = null,
    Object? bootstrap = null,
    Object? bootstrapKeys = null,
    Object? limitOverAttached = null,
    Object? limitFullyAttached = null,
    Object? limitAttachedStrong = null,
    Object? limitAttachedGood = null,
    Object? limitAttachedWeak = null,
  }) {
    return _then(_self.copyWith(
      nodeId: null == nodeId
          ? _self.nodeId
          : nodeId // ignore: cast_nullable_to_non_nullable
              as List<TypedKey>,
      nodeIdSecret: null == nodeIdSecret
          ? _self.nodeIdSecret
          : nodeIdSecret // ignore: cast_nullable_to_non_nullable
              as List<TypedSecret>,
      bootstrap: null == bootstrap
          ? _self.bootstrap
          : bootstrap // ignore: cast_nullable_to_non_nullable
              as List<String>,
      bootstrapKeys: null == bootstrapKeys
          ? _self.bootstrapKeys
          : bootstrapKeys // ignore: cast_nullable_to_non_nullable
              as List<TypedKey>,
      limitOverAttached: null == limitOverAttached
          ? _self.limitOverAttached
          : limitOverAttached // ignore: cast_nullable_to_non_nullable
              as int,
      limitFullyAttached: null == limitFullyAttached
          ? _self.limitFullyAttached
          : limitFullyAttached // ignore: cast_nullable_to_non_nullable
              as int,
      limitAttachedStrong: null == limitAttachedStrong
          ? _self.limitAttachedStrong
          : limitAttachedStrong // ignore: cast_nullable_to_non_nullable
              as int,
      limitAttachedGood: null == limitAttachedGood
          ? _self.limitAttachedGood
          : limitAttachedGood // ignore: cast_nullable_to_non_nullable
              as int,
      limitAttachedWeak: null == limitAttachedWeak
          ? _self.limitAttachedWeak
          : limitAttachedWeak // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _VeilidConfigRoutingTable
    with DiagnosticableTreeMixin
    implements VeilidConfigRoutingTable {
  const _VeilidConfigRoutingTable(
      {required final List<TypedKey> nodeId,
      required final List<TypedSecret> nodeIdSecret,
      required final List<String> bootstrap,
      required final List<TypedKey> bootstrapKeys,
      required this.limitOverAttached,
      required this.limitFullyAttached,
      required this.limitAttachedStrong,
      required this.limitAttachedGood,
      required this.limitAttachedWeak})
      : _nodeId = nodeId,
        _nodeIdSecret = nodeIdSecret,
        _bootstrap = bootstrap,
        _bootstrapKeys = bootstrapKeys;
  factory _VeilidConfigRoutingTable.fromJson(Map<String, dynamic> json) =>
      _$VeilidConfigRoutingTableFromJson(json);

  final List<TypedKey> _nodeId;
  @override
  List<TypedKey> get nodeId {
    if (_nodeId is EqualUnmodifiableListView) return _nodeId;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_nodeId);
  }

  final List<TypedSecret> _nodeIdSecret;
  @override
  List<TypedSecret> get nodeIdSecret {
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

  final List<TypedKey> _bootstrapKeys;
  @override
  List<TypedKey> get bootstrapKeys {
    if (_bootstrapKeys is EqualUnmodifiableListView) return _bootstrapKeys;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_bootstrapKeys);
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

  /// Create a copy of VeilidConfigRoutingTable
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  _$VeilidConfigRoutingTableCopyWith<_VeilidConfigRoutingTable> get copyWith =>
      __$VeilidConfigRoutingTableCopyWithImpl<_VeilidConfigRoutingTable>(
          this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$VeilidConfigRoutingTableToJson(
      this,
    );
  }

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
    properties
      ..add(DiagnosticsProperty('type', 'VeilidConfigRoutingTable'))
      ..add(DiagnosticsProperty('nodeId', nodeId))
      ..add(DiagnosticsProperty('nodeIdSecret', nodeIdSecret))
      ..add(DiagnosticsProperty('bootstrap', bootstrap))
      ..add(DiagnosticsProperty('bootstrapKeys', bootstrapKeys))
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
            other is _VeilidConfigRoutingTable &&
            const DeepCollectionEquality().equals(other._nodeId, _nodeId) &&
            const DeepCollectionEquality()
                .equals(other._nodeIdSecret, _nodeIdSecret) &&
            const DeepCollectionEquality()
                .equals(other._bootstrap, _bootstrap) &&
            const DeepCollectionEquality()
                .equals(other._bootstrapKeys, _bootstrapKeys) &&
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

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(
      runtimeType,
      const DeepCollectionEquality().hash(_nodeId),
      const DeepCollectionEquality().hash(_nodeIdSecret),
      const DeepCollectionEquality().hash(_bootstrap),
      const DeepCollectionEquality().hash(_bootstrapKeys),
      limitOverAttached,
      limitFullyAttached,
      limitAttachedStrong,
      limitAttachedGood,
      limitAttachedWeak);

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidConfigRoutingTable(nodeId: $nodeId, nodeIdSecret: $nodeIdSecret, bootstrap: $bootstrap, bootstrapKeys: $bootstrapKeys, limitOverAttached: $limitOverAttached, limitFullyAttached: $limitFullyAttached, limitAttachedStrong: $limitAttachedStrong, limitAttachedGood: $limitAttachedGood, limitAttachedWeak: $limitAttachedWeak)';
  }
}

/// @nodoc
abstract mixin class _$VeilidConfigRoutingTableCopyWith<$Res>
    implements $VeilidConfigRoutingTableCopyWith<$Res> {
  factory _$VeilidConfigRoutingTableCopyWith(_VeilidConfigRoutingTable value,
          $Res Function(_VeilidConfigRoutingTable) _then) =
      __$VeilidConfigRoutingTableCopyWithImpl;
  @override
  @useResult
  $Res call(
      {List<TypedKey> nodeId,
      List<TypedSecret> nodeIdSecret,
      List<String> bootstrap,
      List<TypedKey> bootstrapKeys,
      int limitOverAttached,
      int limitFullyAttached,
      int limitAttachedStrong,
      int limitAttachedGood,
      int limitAttachedWeak});
}

/// @nodoc
class __$VeilidConfigRoutingTableCopyWithImpl<$Res>
    implements _$VeilidConfigRoutingTableCopyWith<$Res> {
  __$VeilidConfigRoutingTableCopyWithImpl(this._self, this._then);

  final _VeilidConfigRoutingTable _self;
  final $Res Function(_VeilidConfigRoutingTable) _then;

  /// Create a copy of VeilidConfigRoutingTable
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? nodeId = null,
    Object? nodeIdSecret = null,
    Object? bootstrap = null,
    Object? bootstrapKeys = null,
    Object? limitOverAttached = null,
    Object? limitFullyAttached = null,
    Object? limitAttachedStrong = null,
    Object? limitAttachedGood = null,
    Object? limitAttachedWeak = null,
  }) {
    return _then(_VeilidConfigRoutingTable(
      nodeId: null == nodeId
          ? _self._nodeId
          : nodeId // ignore: cast_nullable_to_non_nullable
              as List<TypedKey>,
      nodeIdSecret: null == nodeIdSecret
          ? _self._nodeIdSecret
          : nodeIdSecret // ignore: cast_nullable_to_non_nullable
              as List<TypedSecret>,
      bootstrap: null == bootstrap
          ? _self._bootstrap
          : bootstrap // ignore: cast_nullable_to_non_nullable
              as List<String>,
      bootstrapKeys: null == bootstrapKeys
          ? _self._bootstrapKeys
          : bootstrapKeys // ignore: cast_nullable_to_non_nullable
              as List<TypedKey>,
      limitOverAttached: null == limitOverAttached
          ? _self.limitOverAttached
          : limitOverAttached // ignore: cast_nullable_to_non_nullable
              as int,
      limitFullyAttached: null == limitFullyAttached
          ? _self.limitFullyAttached
          : limitFullyAttached // ignore: cast_nullable_to_non_nullable
              as int,
      limitAttachedStrong: null == limitAttachedStrong
          ? _self.limitAttachedStrong
          : limitAttachedStrong // ignore: cast_nullable_to_non_nullable
              as int,
      limitAttachedGood: null == limitAttachedGood
          ? _self.limitAttachedGood
          : limitAttachedGood // ignore: cast_nullable_to_non_nullable
              as int,
      limitAttachedWeak: null == limitAttachedWeak
          ? _self.limitAttachedWeak
          : limitAttachedWeak // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc
mixin _$VeilidConfigNetwork implements DiagnosticableTreeMixin {
  int get connectionInitialTimeoutMs;
  int get connectionInactivityTimeoutMs;
  int get maxConnectionsPerIp4;
  int get maxConnectionsPerIp6Prefix;
  int get maxConnectionsPerIp6PrefixSize;
  int get maxConnectionFrequencyPerMin;
  int get clientAllowlistTimeoutMs;
  int get reverseConnectionReceiptTimeMs;
  int get holePunchReceiptTimeMs;
  VeilidConfigRoutingTable get routingTable;
  VeilidConfigRPC get rpc;
  VeilidConfigDHT get dht;
  bool get upnp;
  bool get detectAddressChanges;
  int get restrictedNatRetries;
  VeilidConfigTLS get tls;
  VeilidConfigApplication get application;
  VeilidConfigProtocol get protocol;
  String? get networkKeyPassword;

  /// Create a copy of VeilidConfigNetwork
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $VeilidConfigNetworkCopyWith<VeilidConfigNetwork> get copyWith =>
      _$VeilidConfigNetworkCopyWithImpl<VeilidConfigNetwork>(
          this as VeilidConfigNetwork, _$identity);

  /// Serializes this VeilidConfigNetwork to a JSON map.
  Map<String, dynamic> toJson();

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
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
            other is VeilidConfigNetwork &&
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

  @JsonKey(includeFromJson: false, includeToJson: false)
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

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidConfigNetwork(connectionInitialTimeoutMs: $connectionInitialTimeoutMs, connectionInactivityTimeoutMs: $connectionInactivityTimeoutMs, maxConnectionsPerIp4: $maxConnectionsPerIp4, maxConnectionsPerIp6Prefix: $maxConnectionsPerIp6Prefix, maxConnectionsPerIp6PrefixSize: $maxConnectionsPerIp6PrefixSize, maxConnectionFrequencyPerMin: $maxConnectionFrequencyPerMin, clientAllowlistTimeoutMs: $clientAllowlistTimeoutMs, reverseConnectionReceiptTimeMs: $reverseConnectionReceiptTimeMs, holePunchReceiptTimeMs: $holePunchReceiptTimeMs, routingTable: $routingTable, rpc: $rpc, dht: $dht, upnp: $upnp, detectAddressChanges: $detectAddressChanges, restrictedNatRetries: $restrictedNatRetries, tls: $tls, application: $application, protocol: $protocol, networkKeyPassword: $networkKeyPassword)';
  }
}

/// @nodoc
abstract mixin class $VeilidConfigNetworkCopyWith<$Res> {
  factory $VeilidConfigNetworkCopyWith(
          VeilidConfigNetwork value, $Res Function(VeilidConfigNetwork) _then) =
      _$VeilidConfigNetworkCopyWithImpl;
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
class _$VeilidConfigNetworkCopyWithImpl<$Res>
    implements $VeilidConfigNetworkCopyWith<$Res> {
  _$VeilidConfigNetworkCopyWithImpl(this._self, this._then);

  final VeilidConfigNetwork _self;
  final $Res Function(VeilidConfigNetwork) _then;

  /// Create a copy of VeilidConfigNetwork
  /// with the given fields replaced by the non-null parameter values.
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
    return _then(_self.copyWith(
      connectionInitialTimeoutMs: null == connectionInitialTimeoutMs
          ? _self.connectionInitialTimeoutMs
          : connectionInitialTimeoutMs // ignore: cast_nullable_to_non_nullable
              as int,
      connectionInactivityTimeoutMs: null == connectionInactivityTimeoutMs
          ? _self.connectionInactivityTimeoutMs
          : connectionInactivityTimeoutMs // ignore: cast_nullable_to_non_nullable
              as int,
      maxConnectionsPerIp4: null == maxConnectionsPerIp4
          ? _self.maxConnectionsPerIp4
          : maxConnectionsPerIp4 // ignore: cast_nullable_to_non_nullable
              as int,
      maxConnectionsPerIp6Prefix: null == maxConnectionsPerIp6Prefix
          ? _self.maxConnectionsPerIp6Prefix
          : maxConnectionsPerIp6Prefix // ignore: cast_nullable_to_non_nullable
              as int,
      maxConnectionsPerIp6PrefixSize: null == maxConnectionsPerIp6PrefixSize
          ? _self.maxConnectionsPerIp6PrefixSize
          : maxConnectionsPerIp6PrefixSize // ignore: cast_nullable_to_non_nullable
              as int,
      maxConnectionFrequencyPerMin: null == maxConnectionFrequencyPerMin
          ? _self.maxConnectionFrequencyPerMin
          : maxConnectionFrequencyPerMin // ignore: cast_nullable_to_non_nullable
              as int,
      clientAllowlistTimeoutMs: null == clientAllowlistTimeoutMs
          ? _self.clientAllowlistTimeoutMs
          : clientAllowlistTimeoutMs // ignore: cast_nullable_to_non_nullable
              as int,
      reverseConnectionReceiptTimeMs: null == reverseConnectionReceiptTimeMs
          ? _self.reverseConnectionReceiptTimeMs
          : reverseConnectionReceiptTimeMs // ignore: cast_nullable_to_non_nullable
              as int,
      holePunchReceiptTimeMs: null == holePunchReceiptTimeMs
          ? _self.holePunchReceiptTimeMs
          : holePunchReceiptTimeMs // ignore: cast_nullable_to_non_nullable
              as int,
      routingTable: null == routingTable
          ? _self.routingTable
          : routingTable // ignore: cast_nullable_to_non_nullable
              as VeilidConfigRoutingTable,
      rpc: null == rpc
          ? _self.rpc
          : rpc // ignore: cast_nullable_to_non_nullable
              as VeilidConfigRPC,
      dht: null == dht
          ? _self.dht
          : dht // ignore: cast_nullable_to_non_nullable
              as VeilidConfigDHT,
      upnp: null == upnp
          ? _self.upnp
          : upnp // ignore: cast_nullable_to_non_nullable
              as bool,
      detectAddressChanges: null == detectAddressChanges
          ? _self.detectAddressChanges
          : detectAddressChanges // ignore: cast_nullable_to_non_nullable
              as bool,
      restrictedNatRetries: null == restrictedNatRetries
          ? _self.restrictedNatRetries
          : restrictedNatRetries // ignore: cast_nullable_to_non_nullable
              as int,
      tls: null == tls
          ? _self.tls
          : tls // ignore: cast_nullable_to_non_nullable
              as VeilidConfigTLS,
      application: null == application
          ? _self.application
          : application // ignore: cast_nullable_to_non_nullable
              as VeilidConfigApplication,
      protocol: null == protocol
          ? _self.protocol
          : protocol // ignore: cast_nullable_to_non_nullable
              as VeilidConfigProtocol,
      networkKeyPassword: freezed == networkKeyPassword
          ? _self.networkKeyPassword
          : networkKeyPassword // ignore: cast_nullable_to_non_nullable
              as String?,
    ));
  }

  /// Create a copy of VeilidConfigNetwork
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $VeilidConfigRoutingTableCopyWith<$Res> get routingTable {
    return $VeilidConfigRoutingTableCopyWith<$Res>(_self.routingTable, (value) {
      return _then(_self.copyWith(routingTable: value));
    });
  }

  /// Create a copy of VeilidConfigNetwork
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $VeilidConfigRPCCopyWith<$Res> get rpc {
    return $VeilidConfigRPCCopyWith<$Res>(_self.rpc, (value) {
      return _then(_self.copyWith(rpc: value));
    });
  }

  /// Create a copy of VeilidConfigNetwork
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $VeilidConfigDHTCopyWith<$Res> get dht {
    return $VeilidConfigDHTCopyWith<$Res>(_self.dht, (value) {
      return _then(_self.copyWith(dht: value));
    });
  }

  /// Create a copy of VeilidConfigNetwork
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $VeilidConfigTLSCopyWith<$Res> get tls {
    return $VeilidConfigTLSCopyWith<$Res>(_self.tls, (value) {
      return _then(_self.copyWith(tls: value));
    });
  }

  /// Create a copy of VeilidConfigNetwork
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $VeilidConfigApplicationCopyWith<$Res> get application {
    return $VeilidConfigApplicationCopyWith<$Res>(_self.application, (value) {
      return _then(_self.copyWith(application: value));
    });
  }

  /// Create a copy of VeilidConfigNetwork
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $VeilidConfigProtocolCopyWith<$Res> get protocol {
    return $VeilidConfigProtocolCopyWith<$Res>(_self.protocol, (value) {
      return _then(_self.copyWith(protocol: value));
    });
  }
}

/// @nodoc
@JsonSerializable()
class _VeilidConfigNetwork
    with DiagnosticableTreeMixin
    implements VeilidConfigNetwork {
  const _VeilidConfigNetwork(
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
  factory _VeilidConfigNetwork.fromJson(Map<String, dynamic> json) =>
      _$VeilidConfigNetworkFromJson(json);

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

  /// Create a copy of VeilidConfigNetwork
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  _$VeilidConfigNetworkCopyWith<_VeilidConfigNetwork> get copyWith =>
      __$VeilidConfigNetworkCopyWithImpl<_VeilidConfigNetwork>(
          this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$VeilidConfigNetworkToJson(
      this,
    );
  }

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
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
            other is _VeilidConfigNetwork &&
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

  @JsonKey(includeFromJson: false, includeToJson: false)
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

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidConfigNetwork(connectionInitialTimeoutMs: $connectionInitialTimeoutMs, connectionInactivityTimeoutMs: $connectionInactivityTimeoutMs, maxConnectionsPerIp4: $maxConnectionsPerIp4, maxConnectionsPerIp6Prefix: $maxConnectionsPerIp6Prefix, maxConnectionsPerIp6PrefixSize: $maxConnectionsPerIp6PrefixSize, maxConnectionFrequencyPerMin: $maxConnectionFrequencyPerMin, clientAllowlistTimeoutMs: $clientAllowlistTimeoutMs, reverseConnectionReceiptTimeMs: $reverseConnectionReceiptTimeMs, holePunchReceiptTimeMs: $holePunchReceiptTimeMs, routingTable: $routingTable, rpc: $rpc, dht: $dht, upnp: $upnp, detectAddressChanges: $detectAddressChanges, restrictedNatRetries: $restrictedNatRetries, tls: $tls, application: $application, protocol: $protocol, networkKeyPassword: $networkKeyPassword)';
  }
}

/// @nodoc
abstract mixin class _$VeilidConfigNetworkCopyWith<$Res>
    implements $VeilidConfigNetworkCopyWith<$Res> {
  factory _$VeilidConfigNetworkCopyWith(_VeilidConfigNetwork value,
          $Res Function(_VeilidConfigNetwork) _then) =
      __$VeilidConfigNetworkCopyWithImpl;
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
class __$VeilidConfigNetworkCopyWithImpl<$Res>
    implements _$VeilidConfigNetworkCopyWith<$Res> {
  __$VeilidConfigNetworkCopyWithImpl(this._self, this._then);

  final _VeilidConfigNetwork _self;
  final $Res Function(_VeilidConfigNetwork) _then;

  /// Create a copy of VeilidConfigNetwork
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
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
    return _then(_VeilidConfigNetwork(
      connectionInitialTimeoutMs: null == connectionInitialTimeoutMs
          ? _self.connectionInitialTimeoutMs
          : connectionInitialTimeoutMs // ignore: cast_nullable_to_non_nullable
              as int,
      connectionInactivityTimeoutMs: null == connectionInactivityTimeoutMs
          ? _self.connectionInactivityTimeoutMs
          : connectionInactivityTimeoutMs // ignore: cast_nullable_to_non_nullable
              as int,
      maxConnectionsPerIp4: null == maxConnectionsPerIp4
          ? _self.maxConnectionsPerIp4
          : maxConnectionsPerIp4 // ignore: cast_nullable_to_non_nullable
              as int,
      maxConnectionsPerIp6Prefix: null == maxConnectionsPerIp6Prefix
          ? _self.maxConnectionsPerIp6Prefix
          : maxConnectionsPerIp6Prefix // ignore: cast_nullable_to_non_nullable
              as int,
      maxConnectionsPerIp6PrefixSize: null == maxConnectionsPerIp6PrefixSize
          ? _self.maxConnectionsPerIp6PrefixSize
          : maxConnectionsPerIp6PrefixSize // ignore: cast_nullable_to_non_nullable
              as int,
      maxConnectionFrequencyPerMin: null == maxConnectionFrequencyPerMin
          ? _self.maxConnectionFrequencyPerMin
          : maxConnectionFrequencyPerMin // ignore: cast_nullable_to_non_nullable
              as int,
      clientAllowlistTimeoutMs: null == clientAllowlistTimeoutMs
          ? _self.clientAllowlistTimeoutMs
          : clientAllowlistTimeoutMs // ignore: cast_nullable_to_non_nullable
              as int,
      reverseConnectionReceiptTimeMs: null == reverseConnectionReceiptTimeMs
          ? _self.reverseConnectionReceiptTimeMs
          : reverseConnectionReceiptTimeMs // ignore: cast_nullable_to_non_nullable
              as int,
      holePunchReceiptTimeMs: null == holePunchReceiptTimeMs
          ? _self.holePunchReceiptTimeMs
          : holePunchReceiptTimeMs // ignore: cast_nullable_to_non_nullable
              as int,
      routingTable: null == routingTable
          ? _self.routingTable
          : routingTable // ignore: cast_nullable_to_non_nullable
              as VeilidConfigRoutingTable,
      rpc: null == rpc
          ? _self.rpc
          : rpc // ignore: cast_nullable_to_non_nullable
              as VeilidConfigRPC,
      dht: null == dht
          ? _self.dht
          : dht // ignore: cast_nullable_to_non_nullable
              as VeilidConfigDHT,
      upnp: null == upnp
          ? _self.upnp
          : upnp // ignore: cast_nullable_to_non_nullable
              as bool,
      detectAddressChanges: null == detectAddressChanges
          ? _self.detectAddressChanges
          : detectAddressChanges // ignore: cast_nullable_to_non_nullable
              as bool,
      restrictedNatRetries: null == restrictedNatRetries
          ? _self.restrictedNatRetries
          : restrictedNatRetries // ignore: cast_nullable_to_non_nullable
              as int,
      tls: null == tls
          ? _self.tls
          : tls // ignore: cast_nullable_to_non_nullable
              as VeilidConfigTLS,
      application: null == application
          ? _self.application
          : application // ignore: cast_nullable_to_non_nullable
              as VeilidConfigApplication,
      protocol: null == protocol
          ? _self.protocol
          : protocol // ignore: cast_nullable_to_non_nullable
              as VeilidConfigProtocol,
      networkKeyPassword: freezed == networkKeyPassword
          ? _self.networkKeyPassword
          : networkKeyPassword // ignore: cast_nullable_to_non_nullable
              as String?,
    ));
  }

  /// Create a copy of VeilidConfigNetwork
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $VeilidConfigRoutingTableCopyWith<$Res> get routingTable {
    return $VeilidConfigRoutingTableCopyWith<$Res>(_self.routingTable, (value) {
      return _then(_self.copyWith(routingTable: value));
    });
  }

  /// Create a copy of VeilidConfigNetwork
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $VeilidConfigRPCCopyWith<$Res> get rpc {
    return $VeilidConfigRPCCopyWith<$Res>(_self.rpc, (value) {
      return _then(_self.copyWith(rpc: value));
    });
  }

  /// Create a copy of VeilidConfigNetwork
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $VeilidConfigDHTCopyWith<$Res> get dht {
    return $VeilidConfigDHTCopyWith<$Res>(_self.dht, (value) {
      return _then(_self.copyWith(dht: value));
    });
  }

  /// Create a copy of VeilidConfigNetwork
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $VeilidConfigTLSCopyWith<$Res> get tls {
    return $VeilidConfigTLSCopyWith<$Res>(_self.tls, (value) {
      return _then(_self.copyWith(tls: value));
    });
  }

  /// Create a copy of VeilidConfigNetwork
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $VeilidConfigApplicationCopyWith<$Res> get application {
    return $VeilidConfigApplicationCopyWith<$Res>(_self.application, (value) {
      return _then(_self.copyWith(application: value));
    });
  }

  /// Create a copy of VeilidConfigNetwork
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $VeilidConfigProtocolCopyWith<$Res> get protocol {
    return $VeilidConfigProtocolCopyWith<$Res>(_self.protocol, (value) {
      return _then(_self.copyWith(protocol: value));
    });
  }
}

/// @nodoc
mixin _$VeilidConfigTableStore implements DiagnosticableTreeMixin {
  String get directory;
  bool get delete;

  /// Create a copy of VeilidConfigTableStore
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $VeilidConfigTableStoreCopyWith<VeilidConfigTableStore> get copyWith =>
      _$VeilidConfigTableStoreCopyWithImpl<VeilidConfigTableStore>(
          this as VeilidConfigTableStore, _$identity);

  /// Serializes this VeilidConfigTableStore to a JSON map.
  Map<String, dynamic> toJson();

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
    properties
      ..add(DiagnosticsProperty('type', 'VeilidConfigTableStore'))
      ..add(DiagnosticsProperty('directory', directory))
      ..add(DiagnosticsProperty('delete', delete));
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is VeilidConfigTableStore &&
            (identical(other.directory, directory) ||
                other.directory == directory) &&
            (identical(other.delete, delete) || other.delete == delete));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, directory, delete);

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidConfigTableStore(directory: $directory, delete: $delete)';
  }
}

/// @nodoc
abstract mixin class $VeilidConfigTableStoreCopyWith<$Res> {
  factory $VeilidConfigTableStoreCopyWith(VeilidConfigTableStore value,
          $Res Function(VeilidConfigTableStore) _then) =
      _$VeilidConfigTableStoreCopyWithImpl;
  @useResult
  $Res call({String directory, bool delete});
}

/// @nodoc
class _$VeilidConfigTableStoreCopyWithImpl<$Res>
    implements $VeilidConfigTableStoreCopyWith<$Res> {
  _$VeilidConfigTableStoreCopyWithImpl(this._self, this._then);

  final VeilidConfigTableStore _self;
  final $Res Function(VeilidConfigTableStore) _then;

  /// Create a copy of VeilidConfigTableStore
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? directory = null,
    Object? delete = null,
  }) {
    return _then(_self.copyWith(
      directory: null == directory
          ? _self.directory
          : directory // ignore: cast_nullable_to_non_nullable
              as String,
      delete: null == delete
          ? _self.delete
          : delete // ignore: cast_nullable_to_non_nullable
              as bool,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _VeilidConfigTableStore
    with DiagnosticableTreeMixin
    implements VeilidConfigTableStore {
  const _VeilidConfigTableStore(
      {required this.directory, required this.delete});
  factory _VeilidConfigTableStore.fromJson(Map<String, dynamic> json) =>
      _$VeilidConfigTableStoreFromJson(json);

  @override
  final String directory;
  @override
  final bool delete;

  /// Create a copy of VeilidConfigTableStore
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  _$VeilidConfigTableStoreCopyWith<_VeilidConfigTableStore> get copyWith =>
      __$VeilidConfigTableStoreCopyWithImpl<_VeilidConfigTableStore>(
          this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$VeilidConfigTableStoreToJson(
      this,
    );
  }

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
    properties
      ..add(DiagnosticsProperty('type', 'VeilidConfigTableStore'))
      ..add(DiagnosticsProperty('directory', directory))
      ..add(DiagnosticsProperty('delete', delete));
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _VeilidConfigTableStore &&
            (identical(other.directory, directory) ||
                other.directory == directory) &&
            (identical(other.delete, delete) || other.delete == delete));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, directory, delete);

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidConfigTableStore(directory: $directory, delete: $delete)';
  }
}

/// @nodoc
abstract mixin class _$VeilidConfigTableStoreCopyWith<$Res>
    implements $VeilidConfigTableStoreCopyWith<$Res> {
  factory _$VeilidConfigTableStoreCopyWith(_VeilidConfigTableStore value,
          $Res Function(_VeilidConfigTableStore) _then) =
      __$VeilidConfigTableStoreCopyWithImpl;
  @override
  @useResult
  $Res call({String directory, bool delete});
}

/// @nodoc
class __$VeilidConfigTableStoreCopyWithImpl<$Res>
    implements _$VeilidConfigTableStoreCopyWith<$Res> {
  __$VeilidConfigTableStoreCopyWithImpl(this._self, this._then);

  final _VeilidConfigTableStore _self;
  final $Res Function(_VeilidConfigTableStore) _then;

  /// Create a copy of VeilidConfigTableStore
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? directory = null,
    Object? delete = null,
  }) {
    return _then(_VeilidConfigTableStore(
      directory: null == directory
          ? _self.directory
          : directory // ignore: cast_nullable_to_non_nullable
              as String,
      delete: null == delete
          ? _self.delete
          : delete // ignore: cast_nullable_to_non_nullable
              as bool,
    ));
  }
}

/// @nodoc
mixin _$VeilidConfigBlockStore implements DiagnosticableTreeMixin {
  String get directory;
  bool get delete;

  /// Create a copy of VeilidConfigBlockStore
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $VeilidConfigBlockStoreCopyWith<VeilidConfigBlockStore> get copyWith =>
      _$VeilidConfigBlockStoreCopyWithImpl<VeilidConfigBlockStore>(
          this as VeilidConfigBlockStore, _$identity);

  /// Serializes this VeilidConfigBlockStore to a JSON map.
  Map<String, dynamic> toJson();

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
    properties
      ..add(DiagnosticsProperty('type', 'VeilidConfigBlockStore'))
      ..add(DiagnosticsProperty('directory', directory))
      ..add(DiagnosticsProperty('delete', delete));
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is VeilidConfigBlockStore &&
            (identical(other.directory, directory) ||
                other.directory == directory) &&
            (identical(other.delete, delete) || other.delete == delete));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, directory, delete);

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidConfigBlockStore(directory: $directory, delete: $delete)';
  }
}

/// @nodoc
abstract mixin class $VeilidConfigBlockStoreCopyWith<$Res> {
  factory $VeilidConfigBlockStoreCopyWith(VeilidConfigBlockStore value,
          $Res Function(VeilidConfigBlockStore) _then) =
      _$VeilidConfigBlockStoreCopyWithImpl;
  @useResult
  $Res call({String directory, bool delete});
}

/// @nodoc
class _$VeilidConfigBlockStoreCopyWithImpl<$Res>
    implements $VeilidConfigBlockStoreCopyWith<$Res> {
  _$VeilidConfigBlockStoreCopyWithImpl(this._self, this._then);

  final VeilidConfigBlockStore _self;
  final $Res Function(VeilidConfigBlockStore) _then;

  /// Create a copy of VeilidConfigBlockStore
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? directory = null,
    Object? delete = null,
  }) {
    return _then(_self.copyWith(
      directory: null == directory
          ? _self.directory
          : directory // ignore: cast_nullable_to_non_nullable
              as String,
      delete: null == delete
          ? _self.delete
          : delete // ignore: cast_nullable_to_non_nullable
              as bool,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _VeilidConfigBlockStore
    with DiagnosticableTreeMixin
    implements VeilidConfigBlockStore {
  const _VeilidConfigBlockStore(
      {required this.directory, required this.delete});
  factory _VeilidConfigBlockStore.fromJson(Map<String, dynamic> json) =>
      _$VeilidConfigBlockStoreFromJson(json);

  @override
  final String directory;
  @override
  final bool delete;

  /// Create a copy of VeilidConfigBlockStore
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  _$VeilidConfigBlockStoreCopyWith<_VeilidConfigBlockStore> get copyWith =>
      __$VeilidConfigBlockStoreCopyWithImpl<_VeilidConfigBlockStore>(
          this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$VeilidConfigBlockStoreToJson(
      this,
    );
  }

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
    properties
      ..add(DiagnosticsProperty('type', 'VeilidConfigBlockStore'))
      ..add(DiagnosticsProperty('directory', directory))
      ..add(DiagnosticsProperty('delete', delete));
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _VeilidConfigBlockStore &&
            (identical(other.directory, directory) ||
                other.directory == directory) &&
            (identical(other.delete, delete) || other.delete == delete));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, directory, delete);

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidConfigBlockStore(directory: $directory, delete: $delete)';
  }
}

/// @nodoc
abstract mixin class _$VeilidConfigBlockStoreCopyWith<$Res>
    implements $VeilidConfigBlockStoreCopyWith<$Res> {
  factory _$VeilidConfigBlockStoreCopyWith(_VeilidConfigBlockStore value,
          $Res Function(_VeilidConfigBlockStore) _then) =
      __$VeilidConfigBlockStoreCopyWithImpl;
  @override
  @useResult
  $Res call({String directory, bool delete});
}

/// @nodoc
class __$VeilidConfigBlockStoreCopyWithImpl<$Res>
    implements _$VeilidConfigBlockStoreCopyWith<$Res> {
  __$VeilidConfigBlockStoreCopyWithImpl(this._self, this._then);

  final _VeilidConfigBlockStore _self;
  final $Res Function(_VeilidConfigBlockStore) _then;

  /// Create a copy of VeilidConfigBlockStore
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? directory = null,
    Object? delete = null,
  }) {
    return _then(_VeilidConfigBlockStore(
      directory: null == directory
          ? _self.directory
          : directory // ignore: cast_nullable_to_non_nullable
              as String,
      delete: null == delete
          ? _self.delete
          : delete // ignore: cast_nullable_to_non_nullable
              as bool,
    ));
  }
}

/// @nodoc
mixin _$VeilidConfigProtectedStore implements DiagnosticableTreeMixin {
  bool get allowInsecureFallback;
  bool get alwaysUseInsecureStorage;
  String get directory;
  bool get delete;
  String get deviceEncryptionKeyPassword;
  String? get newDeviceEncryptionKeyPassword;

  /// Create a copy of VeilidConfigProtectedStore
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $VeilidConfigProtectedStoreCopyWith<VeilidConfigProtectedStore>
      get copyWith =>
          _$VeilidConfigProtectedStoreCopyWithImpl<VeilidConfigProtectedStore>(
              this as VeilidConfigProtectedStore, _$identity);

  /// Serializes this VeilidConfigProtectedStore to a JSON map.
  Map<String, dynamic> toJson();

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
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
            other is VeilidConfigProtectedStore &&
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

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(
      runtimeType,
      allowInsecureFallback,
      alwaysUseInsecureStorage,
      directory,
      delete,
      deviceEncryptionKeyPassword,
      newDeviceEncryptionKeyPassword);

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidConfigProtectedStore(allowInsecureFallback: $allowInsecureFallback, alwaysUseInsecureStorage: $alwaysUseInsecureStorage, directory: $directory, delete: $delete, deviceEncryptionKeyPassword: $deviceEncryptionKeyPassword, newDeviceEncryptionKeyPassword: $newDeviceEncryptionKeyPassword)';
  }
}

/// @nodoc
abstract mixin class $VeilidConfigProtectedStoreCopyWith<$Res> {
  factory $VeilidConfigProtectedStoreCopyWith(VeilidConfigProtectedStore value,
          $Res Function(VeilidConfigProtectedStore) _then) =
      _$VeilidConfigProtectedStoreCopyWithImpl;
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
class _$VeilidConfigProtectedStoreCopyWithImpl<$Res>
    implements $VeilidConfigProtectedStoreCopyWith<$Res> {
  _$VeilidConfigProtectedStoreCopyWithImpl(this._self, this._then);

  final VeilidConfigProtectedStore _self;
  final $Res Function(VeilidConfigProtectedStore) _then;

  /// Create a copy of VeilidConfigProtectedStore
  /// with the given fields replaced by the non-null parameter values.
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
    return _then(_self.copyWith(
      allowInsecureFallback: null == allowInsecureFallback
          ? _self.allowInsecureFallback
          : allowInsecureFallback // ignore: cast_nullable_to_non_nullable
              as bool,
      alwaysUseInsecureStorage: null == alwaysUseInsecureStorage
          ? _self.alwaysUseInsecureStorage
          : alwaysUseInsecureStorage // ignore: cast_nullable_to_non_nullable
              as bool,
      directory: null == directory
          ? _self.directory
          : directory // ignore: cast_nullable_to_non_nullable
              as String,
      delete: null == delete
          ? _self.delete
          : delete // ignore: cast_nullable_to_non_nullable
              as bool,
      deviceEncryptionKeyPassword: null == deviceEncryptionKeyPassword
          ? _self.deviceEncryptionKeyPassword
          : deviceEncryptionKeyPassword // ignore: cast_nullable_to_non_nullable
              as String,
      newDeviceEncryptionKeyPassword: freezed == newDeviceEncryptionKeyPassword
          ? _self.newDeviceEncryptionKeyPassword
          : newDeviceEncryptionKeyPassword // ignore: cast_nullable_to_non_nullable
              as String?,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _VeilidConfigProtectedStore
    with DiagnosticableTreeMixin
    implements VeilidConfigProtectedStore {
  const _VeilidConfigProtectedStore(
      {required this.allowInsecureFallback,
      required this.alwaysUseInsecureStorage,
      required this.directory,
      required this.delete,
      required this.deviceEncryptionKeyPassword,
      this.newDeviceEncryptionKeyPassword});
  factory _VeilidConfigProtectedStore.fromJson(Map<String, dynamic> json) =>
      _$VeilidConfigProtectedStoreFromJson(json);

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

  /// Create a copy of VeilidConfigProtectedStore
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  _$VeilidConfigProtectedStoreCopyWith<_VeilidConfigProtectedStore>
      get copyWith => __$VeilidConfigProtectedStoreCopyWithImpl<
          _VeilidConfigProtectedStore>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$VeilidConfigProtectedStoreToJson(
      this,
    );
  }

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
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
            other is _VeilidConfigProtectedStore &&
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

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(
      runtimeType,
      allowInsecureFallback,
      alwaysUseInsecureStorage,
      directory,
      delete,
      deviceEncryptionKeyPassword,
      newDeviceEncryptionKeyPassword);

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidConfigProtectedStore(allowInsecureFallback: $allowInsecureFallback, alwaysUseInsecureStorage: $alwaysUseInsecureStorage, directory: $directory, delete: $delete, deviceEncryptionKeyPassword: $deviceEncryptionKeyPassword, newDeviceEncryptionKeyPassword: $newDeviceEncryptionKeyPassword)';
  }
}

/// @nodoc
abstract mixin class _$VeilidConfigProtectedStoreCopyWith<$Res>
    implements $VeilidConfigProtectedStoreCopyWith<$Res> {
  factory _$VeilidConfigProtectedStoreCopyWith(
          _VeilidConfigProtectedStore value,
          $Res Function(_VeilidConfigProtectedStore) _then) =
      __$VeilidConfigProtectedStoreCopyWithImpl;
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
class __$VeilidConfigProtectedStoreCopyWithImpl<$Res>
    implements _$VeilidConfigProtectedStoreCopyWith<$Res> {
  __$VeilidConfigProtectedStoreCopyWithImpl(this._self, this._then);

  final _VeilidConfigProtectedStore _self;
  final $Res Function(_VeilidConfigProtectedStore) _then;

  /// Create a copy of VeilidConfigProtectedStore
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? allowInsecureFallback = null,
    Object? alwaysUseInsecureStorage = null,
    Object? directory = null,
    Object? delete = null,
    Object? deviceEncryptionKeyPassword = null,
    Object? newDeviceEncryptionKeyPassword = freezed,
  }) {
    return _then(_VeilidConfigProtectedStore(
      allowInsecureFallback: null == allowInsecureFallback
          ? _self.allowInsecureFallback
          : allowInsecureFallback // ignore: cast_nullable_to_non_nullable
              as bool,
      alwaysUseInsecureStorage: null == alwaysUseInsecureStorage
          ? _self.alwaysUseInsecureStorage
          : alwaysUseInsecureStorage // ignore: cast_nullable_to_non_nullable
              as bool,
      directory: null == directory
          ? _self.directory
          : directory // ignore: cast_nullable_to_non_nullable
              as String,
      delete: null == delete
          ? _self.delete
          : delete // ignore: cast_nullable_to_non_nullable
              as bool,
      deviceEncryptionKeyPassword: null == deviceEncryptionKeyPassword
          ? _self.deviceEncryptionKeyPassword
          : deviceEncryptionKeyPassword // ignore: cast_nullable_to_non_nullable
              as String,
      newDeviceEncryptionKeyPassword: freezed == newDeviceEncryptionKeyPassword
          ? _self.newDeviceEncryptionKeyPassword
          : newDeviceEncryptionKeyPassword // ignore: cast_nullable_to_non_nullable
              as String?,
    ));
  }
}

/// @nodoc
mixin _$VeilidConfigCapabilities implements DiagnosticableTreeMixin {
  List<String> get disable;

  /// Create a copy of VeilidConfigCapabilities
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $VeilidConfigCapabilitiesCopyWith<VeilidConfigCapabilities> get copyWith =>
      _$VeilidConfigCapabilitiesCopyWithImpl<VeilidConfigCapabilities>(
          this as VeilidConfigCapabilities, _$identity);

  /// Serializes this VeilidConfigCapabilities to a JSON map.
  Map<String, dynamic> toJson();

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
    properties
      ..add(DiagnosticsProperty('type', 'VeilidConfigCapabilities'))
      ..add(DiagnosticsProperty('disable', disable));
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is VeilidConfigCapabilities &&
            const DeepCollectionEquality().equals(other.disable, disable));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(disable));

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidConfigCapabilities(disable: $disable)';
  }
}

/// @nodoc
abstract mixin class $VeilidConfigCapabilitiesCopyWith<$Res> {
  factory $VeilidConfigCapabilitiesCopyWith(VeilidConfigCapabilities value,
          $Res Function(VeilidConfigCapabilities) _then) =
      _$VeilidConfigCapabilitiesCopyWithImpl;
  @useResult
  $Res call({List<String> disable});
}

/// @nodoc
class _$VeilidConfigCapabilitiesCopyWithImpl<$Res>
    implements $VeilidConfigCapabilitiesCopyWith<$Res> {
  _$VeilidConfigCapabilitiesCopyWithImpl(this._self, this._then);

  final VeilidConfigCapabilities _self;
  final $Res Function(VeilidConfigCapabilities) _then;

  /// Create a copy of VeilidConfigCapabilities
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? disable = null,
  }) {
    return _then(_self.copyWith(
      disable: null == disable
          ? _self.disable
          : disable // ignore: cast_nullable_to_non_nullable
              as List<String>,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _VeilidConfigCapabilities
    with DiagnosticableTreeMixin
    implements VeilidConfigCapabilities {
  const _VeilidConfigCapabilities({required final List<String> disable})
      : _disable = disable;
  factory _VeilidConfigCapabilities.fromJson(Map<String, dynamic> json) =>
      _$VeilidConfigCapabilitiesFromJson(json);

  final List<String> _disable;
  @override
  List<String> get disable {
    if (_disable is EqualUnmodifiableListView) return _disable;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_disable);
  }

  /// Create a copy of VeilidConfigCapabilities
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  _$VeilidConfigCapabilitiesCopyWith<_VeilidConfigCapabilities> get copyWith =>
      __$VeilidConfigCapabilitiesCopyWithImpl<_VeilidConfigCapabilities>(
          this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$VeilidConfigCapabilitiesToJson(
      this,
    );
  }

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
    properties
      ..add(DiagnosticsProperty('type', 'VeilidConfigCapabilities'))
      ..add(DiagnosticsProperty('disable', disable));
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _VeilidConfigCapabilities &&
            const DeepCollectionEquality().equals(other._disable, _disable));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(_disable));

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidConfigCapabilities(disable: $disable)';
  }
}

/// @nodoc
abstract mixin class _$VeilidConfigCapabilitiesCopyWith<$Res>
    implements $VeilidConfigCapabilitiesCopyWith<$Res> {
  factory _$VeilidConfigCapabilitiesCopyWith(_VeilidConfigCapabilities value,
          $Res Function(_VeilidConfigCapabilities) _then) =
      __$VeilidConfigCapabilitiesCopyWithImpl;
  @override
  @useResult
  $Res call({List<String> disable});
}

/// @nodoc
class __$VeilidConfigCapabilitiesCopyWithImpl<$Res>
    implements _$VeilidConfigCapabilitiesCopyWith<$Res> {
  __$VeilidConfigCapabilitiesCopyWithImpl(this._self, this._then);

  final _VeilidConfigCapabilities _self;
  final $Res Function(_VeilidConfigCapabilities) _then;

  /// Create a copy of VeilidConfigCapabilities
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? disable = null,
  }) {
    return _then(_VeilidConfigCapabilities(
      disable: null == disable
          ? _self._disable
          : disable // ignore: cast_nullable_to_non_nullable
              as List<String>,
    ));
  }
}

/// @nodoc
mixin _$VeilidConfig implements DiagnosticableTreeMixin {
  String get programName;
  String get namespace;
  VeilidConfigCapabilities get capabilities;
  VeilidConfigProtectedStore get protectedStore;
  VeilidConfigTableStore get tableStore;
  VeilidConfigBlockStore get blockStore;
  VeilidConfigNetwork get network;

  /// Create a copy of VeilidConfig
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $VeilidConfigCopyWith<VeilidConfig> get copyWith =>
      _$VeilidConfigCopyWithImpl<VeilidConfig>(
          this as VeilidConfig, _$identity);

  /// Serializes this VeilidConfig to a JSON map.
  Map<String, dynamic> toJson();

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
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
            other is VeilidConfig &&
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

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, programName, namespace,
      capabilities, protectedStore, tableStore, blockStore, network);

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidConfig(programName: $programName, namespace: $namespace, capabilities: $capabilities, protectedStore: $protectedStore, tableStore: $tableStore, blockStore: $blockStore, network: $network)';
  }
}

/// @nodoc
abstract mixin class $VeilidConfigCopyWith<$Res> {
  factory $VeilidConfigCopyWith(
          VeilidConfig value, $Res Function(VeilidConfig) _then) =
      _$VeilidConfigCopyWithImpl;
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
class _$VeilidConfigCopyWithImpl<$Res> implements $VeilidConfigCopyWith<$Res> {
  _$VeilidConfigCopyWithImpl(this._self, this._then);

  final VeilidConfig _self;
  final $Res Function(VeilidConfig) _then;

  /// Create a copy of VeilidConfig
  /// with the given fields replaced by the non-null parameter values.
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
    return _then(_self.copyWith(
      programName: null == programName
          ? _self.programName
          : programName // ignore: cast_nullable_to_non_nullable
              as String,
      namespace: null == namespace
          ? _self.namespace
          : namespace // ignore: cast_nullable_to_non_nullable
              as String,
      capabilities: null == capabilities
          ? _self.capabilities
          : capabilities // ignore: cast_nullable_to_non_nullable
              as VeilidConfigCapabilities,
      protectedStore: null == protectedStore
          ? _self.protectedStore
          : protectedStore // ignore: cast_nullable_to_non_nullable
              as VeilidConfigProtectedStore,
      tableStore: null == tableStore
          ? _self.tableStore
          : tableStore // ignore: cast_nullable_to_non_nullable
              as VeilidConfigTableStore,
      blockStore: null == blockStore
          ? _self.blockStore
          : blockStore // ignore: cast_nullable_to_non_nullable
              as VeilidConfigBlockStore,
      network: null == network
          ? _self.network
          : network // ignore: cast_nullable_to_non_nullable
              as VeilidConfigNetwork,
    ));
  }

  /// Create a copy of VeilidConfig
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $VeilidConfigCapabilitiesCopyWith<$Res> get capabilities {
    return $VeilidConfigCapabilitiesCopyWith<$Res>(_self.capabilities, (value) {
      return _then(_self.copyWith(capabilities: value));
    });
  }

  /// Create a copy of VeilidConfig
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $VeilidConfigProtectedStoreCopyWith<$Res> get protectedStore {
    return $VeilidConfigProtectedStoreCopyWith<$Res>(_self.protectedStore,
        (value) {
      return _then(_self.copyWith(protectedStore: value));
    });
  }

  /// Create a copy of VeilidConfig
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $VeilidConfigTableStoreCopyWith<$Res> get tableStore {
    return $VeilidConfigTableStoreCopyWith<$Res>(_self.tableStore, (value) {
      return _then(_self.copyWith(tableStore: value));
    });
  }

  /// Create a copy of VeilidConfig
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $VeilidConfigBlockStoreCopyWith<$Res> get blockStore {
    return $VeilidConfigBlockStoreCopyWith<$Res>(_self.blockStore, (value) {
      return _then(_self.copyWith(blockStore: value));
    });
  }

  /// Create a copy of VeilidConfig
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $VeilidConfigNetworkCopyWith<$Res> get network {
    return $VeilidConfigNetworkCopyWith<$Res>(_self.network, (value) {
      return _then(_self.copyWith(network: value));
    });
  }
}

/// @nodoc
@JsonSerializable()
class _VeilidConfig with DiagnosticableTreeMixin implements VeilidConfig {
  const _VeilidConfig(
      {required this.programName,
      required this.namespace,
      required this.capabilities,
      required this.protectedStore,
      required this.tableStore,
      required this.blockStore,
      required this.network});
  factory _VeilidConfig.fromJson(Map<String, dynamic> json) =>
      _$VeilidConfigFromJson(json);

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

  /// Create a copy of VeilidConfig
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  _$VeilidConfigCopyWith<_VeilidConfig> get copyWith =>
      __$VeilidConfigCopyWithImpl<_VeilidConfig>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$VeilidConfigToJson(
      this,
    );
  }

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
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
            other is _VeilidConfig &&
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

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, programName, namespace,
      capabilities, protectedStore, tableStore, blockStore, network);

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'VeilidConfig(programName: $programName, namespace: $namespace, capabilities: $capabilities, protectedStore: $protectedStore, tableStore: $tableStore, blockStore: $blockStore, network: $network)';
  }
}

/// @nodoc
abstract mixin class _$VeilidConfigCopyWith<$Res>
    implements $VeilidConfigCopyWith<$Res> {
  factory _$VeilidConfigCopyWith(
          _VeilidConfig value, $Res Function(_VeilidConfig) _then) =
      __$VeilidConfigCopyWithImpl;
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
class __$VeilidConfigCopyWithImpl<$Res>
    implements _$VeilidConfigCopyWith<$Res> {
  __$VeilidConfigCopyWithImpl(this._self, this._then);

  final _VeilidConfig _self;
  final $Res Function(_VeilidConfig) _then;

  /// Create a copy of VeilidConfig
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? programName = null,
    Object? namespace = null,
    Object? capabilities = null,
    Object? protectedStore = null,
    Object? tableStore = null,
    Object? blockStore = null,
    Object? network = null,
  }) {
    return _then(_VeilidConfig(
      programName: null == programName
          ? _self.programName
          : programName // ignore: cast_nullable_to_non_nullable
              as String,
      namespace: null == namespace
          ? _self.namespace
          : namespace // ignore: cast_nullable_to_non_nullable
              as String,
      capabilities: null == capabilities
          ? _self.capabilities
          : capabilities // ignore: cast_nullable_to_non_nullable
              as VeilidConfigCapabilities,
      protectedStore: null == protectedStore
          ? _self.protectedStore
          : protectedStore // ignore: cast_nullable_to_non_nullable
              as VeilidConfigProtectedStore,
      tableStore: null == tableStore
          ? _self.tableStore
          : tableStore // ignore: cast_nullable_to_non_nullable
              as VeilidConfigTableStore,
      blockStore: null == blockStore
          ? _self.blockStore
          : blockStore // ignore: cast_nullable_to_non_nullable
              as VeilidConfigBlockStore,
      network: null == network
          ? _self.network
          : network // ignore: cast_nullable_to_non_nullable
              as VeilidConfigNetwork,
    ));
  }

  /// Create a copy of VeilidConfig
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $VeilidConfigCapabilitiesCopyWith<$Res> get capabilities {
    return $VeilidConfigCapabilitiesCopyWith<$Res>(_self.capabilities, (value) {
      return _then(_self.copyWith(capabilities: value));
    });
  }

  /// Create a copy of VeilidConfig
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $VeilidConfigProtectedStoreCopyWith<$Res> get protectedStore {
    return $VeilidConfigProtectedStoreCopyWith<$Res>(_self.protectedStore,
        (value) {
      return _then(_self.copyWith(protectedStore: value));
    });
  }

  /// Create a copy of VeilidConfig
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $VeilidConfigTableStoreCopyWith<$Res> get tableStore {
    return $VeilidConfigTableStoreCopyWith<$Res>(_self.tableStore, (value) {
      return _then(_self.copyWith(tableStore: value));
    });
  }

  /// Create a copy of VeilidConfig
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $VeilidConfigBlockStoreCopyWith<$Res> get blockStore {
    return $VeilidConfigBlockStoreCopyWith<$Res>(_self.blockStore, (value) {
      return _then(_self.copyWith(blockStore: value));
    });
  }

  /// Create a copy of VeilidConfig
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $VeilidConfigNetworkCopyWith<$Res> get network {
    return $VeilidConfigNetworkCopyWith<$Res>(_self.network, (value) {
      return _then(_self.copyWith(network: value));
    });
  }
}

// dart format on
