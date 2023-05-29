//////////////////////////////////////
/// VeilidAPIException

abstract class VeilidAPIException implements Exception {
  factory VeilidAPIException.fromJson(dynamic json) {
    switch (json["kind"]) {
      case "NotInitialized":
        {
          return VeilidAPIExceptionNotInitialized();
        }
      case "AlreadyInitialized":
        {
          return VeilidAPIExceptionAlreadyInitialized();
        }
      case "Timeout":
        {
          return VeilidAPIExceptionTimeout();
        }
      case "TryAgain":
        {
          return VeilidAPIExceptionTryAgain();
        }
      case "Shutdown":
        {
          return VeilidAPIExceptionShutdown();
        }
      case "InvalidTarget":
        {
          return VeilidAPIExceptionInvalidTarget();
        }
      case "NoConnection":
        {
          return VeilidAPIExceptionNoConnection(json["message"]);
        }
      case "KeyNotFound":
        {
          return VeilidAPIExceptionKeyNotFound(json["key"]);
        }
      case "Internal":
        {
          return VeilidAPIExceptionInternal(json["message"]);
        }
      case "Unimplemented":
        {
          return VeilidAPIExceptionUnimplemented(json["unimplemented"]);
        }
      case "ParseError":
        {
          return VeilidAPIExceptionParseError(json["message"], json["value"]);
        }
      case "InvalidArgument":
        {
          return VeilidAPIExceptionInvalidArgument(
              json["context"], json["argument"], json["value"]);
        }
      case "MissingArgument":
        {
          return VeilidAPIExceptionMissingArgument(
              json["context"], json["argument"]);
        }
      case "Generic":
        {
          return VeilidAPIExceptionGeneric(json["message"]);
        }
      default:
        {
          throw VeilidAPIExceptionInternal(
              "Invalid VeilidAPIException type: ${json['kind']}");
        }
    }
  }

  String toDisplayError();
}

class VeilidAPIExceptionNotInitialized implements VeilidAPIException {
  @override
  String toString() {
    return "VeilidAPIException: NotInitialized";
  }

  @override
  String toDisplayError() {
    return "Not initialized";
  }
}

class VeilidAPIExceptionAlreadyInitialized implements VeilidAPIException {
  @override
  String toString() {
    return "VeilidAPIException: AlreadyInitialized";
  }

  @override
  String toDisplayError() {
    return "Already initialized";
  }
}

class VeilidAPIExceptionTimeout implements VeilidAPIException {
  @override
  String toString() {
    return "VeilidAPIException: Timeout";
  }

  @override
  String toDisplayError() {
    return "Timeout";
  }
}

class VeilidAPIExceptionTryAgain implements VeilidAPIException {
  @override
  String toString() {
    return "VeilidAPIException: TryAgain";
  }

  @override
  String toDisplayError() {
    return "Try again";
  }
}

class VeilidAPIExceptionShutdown implements VeilidAPIException {
  @override
  String toString() {
    return "VeilidAPIException: Shutdown";
  }

  @override
  String toDisplayError() {
    return "Currently shut down";
  }
}

class VeilidAPIExceptionInvalidTarget implements VeilidAPIException {
  @override
  String toString() {
    return "VeilidAPIException: InvalidTarget";
  }

  @override
  String toDisplayError() {
    return "Invalid target";
  }
}

class VeilidAPIExceptionNoConnection implements VeilidAPIException {
  final String message;

  @override
  String toString() {
    return "VeilidAPIException: NoConnection (message: $message)";
  }

  @override
  String toDisplayError() {
    return "No connection: $message";
  }

  //
  VeilidAPIExceptionNoConnection(this.message);
}

class VeilidAPIExceptionKeyNotFound implements VeilidAPIException {
  final String key;

  @override
  String toString() {
    return "VeilidAPIException: KeyNotFound (key: $key)";
  }

  @override
  String toDisplayError() {
    return "Key not found: $key";
  }

  //
  VeilidAPIExceptionKeyNotFound(this.key);
}

class VeilidAPIExceptionInternal implements VeilidAPIException {
  final String message;

  @override
  String toString() {
    return "VeilidAPIException: Internal ($message)";
  }

  @override
  String toDisplayError() {
    return "Internal error: $message";
  }

  //
  VeilidAPIExceptionInternal(this.message);
}

class VeilidAPIExceptionUnimplemented implements VeilidAPIException {
  final String message;

  @override
  String toString() {
    return "VeilidAPIException: Unimplemented ($message)";
  }

  @override
  String toDisplayError() {
    return "Unimplemented: $message";
  }

  //
  VeilidAPIExceptionUnimplemented(this.message);
}

class VeilidAPIExceptionParseError implements VeilidAPIException {
  final String message;
  final String value;

  @override
  String toString() {
    return "VeilidAPIException: ParseError ($message)\n    value: $value";
  }

  @override
  String toDisplayError() {
    return "Parse error: $message";
  }

  //
  VeilidAPIExceptionParseError(this.message, this.value);
}

class VeilidAPIExceptionInvalidArgument implements VeilidAPIException {
  final String context;
  final String argument;
  final String value;

  @override
  String toString() {
    return "VeilidAPIException: InvalidArgument ($context:$argument)\n    value: $value";
  }

  @override
  String toDisplayError() {
    return "Invalid argument for $context: $argument";
  }

  //
  VeilidAPIExceptionInvalidArgument(this.context, this.argument, this.value);
}

class VeilidAPIExceptionMissingArgument implements VeilidAPIException {
  final String context;
  final String argument;

  @override
  String toString() {
    return "VeilidAPIException: MissingArgument ($context:$argument)";
  }

  @override
  String toDisplayError() {
    return "Missing argument for $context: $argument";
  }

  //
  VeilidAPIExceptionMissingArgument(this.context, this.argument);
}

class VeilidAPIExceptionGeneric implements VeilidAPIException {
  final String message;

  @override
  String toString() {
    return "VeilidAPIException: Generic (message: $message)";
  }

  @override
  String toDisplayError() {
    return message;
  }

  //
  VeilidAPIExceptionGeneric(this.message);
}
