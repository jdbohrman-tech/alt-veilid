import 'package:freezed_annotation/freezed_annotation.dart';

//////////////////////////////////////
/// VeilidAPIException

@immutable
abstract class VeilidAPIException implements Exception {
  factory VeilidAPIException.fromJson(dynamic j) {
    final json = j as Map<String, dynamic>;
    switch (json['kind']! as String) {
      case 'NotInitialized':
        {
          return VeilidAPIExceptionNotInitialized();
        }
      case 'AlreadyInitialized':
        {
          return VeilidAPIExceptionAlreadyInitialized();
        }
      case 'Timeout':
        {
          return VeilidAPIExceptionTimeout();
        }
      case 'TryAgain':
        {
          return VeilidAPIExceptionTryAgain();
        }
      case 'Shutdown':
        {
          return VeilidAPIExceptionShutdown();
        }
      case 'InvalidTarget':
        {
          return VeilidAPIExceptionInvalidTarget();
        }
      case 'NoConnection':
        {
          return VeilidAPIExceptionNoConnection(json['message']! as String);
        }
      case 'KeyNotFound':
        {
          return VeilidAPIExceptionKeyNotFound(json['key']! as String);
        }
      case 'Internal':
        {
          return VeilidAPIExceptionInternal(json['message']! as String);
        }
      case 'Unimplemented':
        {
          return VeilidAPIExceptionUnimplemented(
              json['unimplemented']! as String);
        }
      case 'ParseError':
        {
          return VeilidAPIExceptionParseError(
              json['message']! as String, json['value']! as String);
        }
      case 'InvalidArgument':
        {
          return VeilidAPIExceptionInvalidArgument(json['context']! as String,
              json['argument']! as String, json['value']! as String);
        }
      case 'MissingArgument':
        {
          return VeilidAPIExceptionMissingArgument(
              json['context']! as String, json['argument']! as String);
        }
      case 'Generic':
        {
          return VeilidAPIExceptionGeneric(json['message']! as String);
        }
      default:
        {
          throw VeilidAPIExceptionInternal(
              "Invalid VeilidAPIException type: ${json['kind']! as String}");
        }
    }
  }

  String toDisplayError();
}

@immutable
class VeilidAPIExceptionNotInitialized implements VeilidAPIException {
  @override
  String toString() => 'VeilidAPIException: NotInitialized';

  @override
  String toDisplayError() => 'Not initialized';
}

@immutable
class VeilidAPIExceptionAlreadyInitialized implements VeilidAPIException {
  @override
  String toString() => 'VeilidAPIException: AlreadyInitialized';

  @override
  String toDisplayError() => 'Already initialized';
}

@immutable
class VeilidAPIExceptionTimeout implements VeilidAPIException {
  @override
  String toString() => 'VeilidAPIException: Timeout';

  @override
  String toDisplayError() => 'Timeout';
}

@immutable
class VeilidAPIExceptionTryAgain implements VeilidAPIException {
  @override
  String toString() => 'VeilidAPIException: TryAgain';

  @override
  String toDisplayError() => 'Try again';
}

@immutable
class VeilidAPIExceptionShutdown implements VeilidAPIException {
  @override
  String toString() => 'VeilidAPIException: Shutdown';

  @override
  String toDisplayError() => 'Currently shut down';
}

@immutable
class VeilidAPIExceptionInvalidTarget implements VeilidAPIException {
  @override
  String toString() => 'VeilidAPIException: InvalidTarget';

  @override
  String toDisplayError() => 'Invalid target';
}

@immutable
class VeilidAPIExceptionNoConnection implements VeilidAPIException {
  //
  const VeilidAPIExceptionNoConnection(this.message);
  final String message;
  @override
  String toString() => 'VeilidAPIException: NoConnection (message: $message)';

  @override
  String toDisplayError() => 'No connection: $message';
}

@immutable
class VeilidAPIExceptionKeyNotFound implements VeilidAPIException {
  //
  const VeilidAPIExceptionKeyNotFound(this.key);
  final String key;
  @override
  String toString() => 'VeilidAPIException: KeyNotFound (key: $key)';

  @override
  String toDisplayError() => 'Key not found: $key';
}

@immutable
class VeilidAPIExceptionInternal implements VeilidAPIException {
  //
  const VeilidAPIExceptionInternal(this.message);
  final String message;

  @override
  String toString() => 'VeilidAPIException: Internal ($message)';

  @override
  String toDisplayError() => 'Internal error: $message';
}

@immutable
class VeilidAPIExceptionUnimplemented implements VeilidAPIException {
  //
  const VeilidAPIExceptionUnimplemented(this.message);
  final String message;

  @override
  String toString() => 'VeilidAPIException: Unimplemented ($message)';

  @override
  String toDisplayError() => 'Unimplemented: $message';
}

@immutable
class VeilidAPIExceptionParseError implements VeilidAPIException {
  //
  const VeilidAPIExceptionParseError(this.message, this.value);
  final String message;
  final String value;

  @override
  String toString() =>
      'VeilidAPIException: ParseError ($message)\n    value: $value';

  @override
  String toDisplayError() => 'Parse error: $message';
}

@immutable
class VeilidAPIExceptionInvalidArgument implements VeilidAPIException {
  //
  const VeilidAPIExceptionInvalidArgument(
      this.context, this.argument, this.value);
  final String context;
  final String argument;
  final String value;

  @override
  String toString() => 'VeilidAPIException: InvalidArgument'
      ' ($context:$argument)\n    value: $value';

  @override
  String toDisplayError() => 'Invalid argument for $context: $argument';
}

@immutable
class VeilidAPIExceptionMissingArgument implements VeilidAPIException {
  //
  const VeilidAPIExceptionMissingArgument(this.context, this.argument);
  final String context;
  final String argument;

  @override
  String toString() =>
      'VeilidAPIException: MissingArgument ($context:$argument)';

  @override
  String toDisplayError() => 'Missing argument for $context: $argument';
}

@immutable
class VeilidAPIExceptionGeneric implements VeilidAPIException {
  //
  const VeilidAPIExceptionGeneric(this.message);
  final String message;

  @override
  String toString() => 'VeilidAPIException: Generic (message: $message)';

  @override
  String toDisplayError() => message;
}
