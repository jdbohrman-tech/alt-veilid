from typing import Self

class VeilidAPIError(Exception):
    """Veilid API error exception base class"""
    pass
    @staticmethod
    def from_json(j: dict) -> Self:
        match j['kind']:
            case 'NotInitialized':
                return VeilidAPIErrorNotInitialized()
            case 'AlreadyInitialized':
                return VeilidAPIErrorAlreadyInitialized()
            case 'Timeout':
                return VeilidAPIErrorTimeout()
            case 'TryAgain':
                return VeilidAPIErrorTryAgain()
            case 'Shutdown':
                return VeilidAPIErrorShutdown()
            case 'InvalidTarget':
                return VeilidAPIErrorInvalidTarget()
            case 'NoConnection':
                return VeilidAPIErrorNoConnection(j['message'])
            case 'KeyNotFound':
                return VeilidAPIErrorKeyNotFound(j['key'])
            case 'Internal':
                return VeilidAPIErrorInternal(j['message'])
            case 'Unimplemented':
                return VeilidAPIErrorUnimplemented(j['message'])
            case 'ParseError':
                return VeilidAPIErrorParseError(j['message'], j['value'])
            case 'InvalidArgument':
                return VeilidAPIErrorInvalidArgument(j['context'], j['argument'], j['value'])
            case 'MissingArgument':
                return VeilidAPIErrorMissingArgument(j['context'], j['argument'])
            case 'Generic':
                return VeilidAPIErrorGeneric(j['message'])
            case _:
                return VeilidAPIError("Unknown exception type: {}".format(j['kind']))


class VeilidAPIErrorNotInitialized(VeilidAPIError):
    """Veilid was not initialized"""
    def __init__(self):
        super().__init__("Not initialized")

class VeilidAPIErrorAlreadyInitialized(VeilidAPIError):
    """Veilid was already initialized"""
    def __init__(self):
        super().__init__("Already initialized")

class VeilidAPIErrorTimeout(VeilidAPIError):
    """Veilid operation timed out"""
    def __init__(self):
        super().__init__("Timeout")

class VeilidAPIErrorTryAgain(VeilidAPIError):
    """Operation could not be performed at this time, retry again later"""
    def __init__(self):
        super().__init__("Try again")

class VeilidAPIErrorShutdown(VeilidAPIError):
    """Veilid was already shut down"""
    def __init__(self):
        super().__init__("Shutdown")

class VeilidAPIErrorInvalidTarget(VeilidAPIError):
    """Target of operation is not valid"""
    def __init__(self):
        super().__init__("Invalid target")

class VeilidAPIErrorNoConnection(VeilidAPIError):
    """Connection could not be established"""
    message: str
    def __init__(self, message: str):
        super().__init__("No connection")
        self.message = message

class VeilidAPIErrorKeyNotFound(VeilidAPIError):
    """Key was not found"""
    key: str
    def __init__(self, key: str):
        super().__init__("Key not found")
        self.key = key

class VeilidAPIErrorInternal(VeilidAPIError):
    """Veilid experienced an internal failure"""
    message: str
    def __init__(self, message: str):
        super().__init__("Internal")
        self.message = message

class VeilidAPIErrorUnimplemented(VeilidAPIError):
    """Functionality is not yet implemented"""
    message: str
    def __init__(self, message: str):
        super().__init__("Unimplemented")
        self.message = message

class VeilidAPIErrorParseError(VeilidAPIError):
    """Value was not in a parseable format"""
    message: str
    value: str
    def __init__(self, message: str, value: str):
        super().__init__("Parse error")
        self.message = message
        self.value = value

class VeilidAPIErrorInvalidArgument(VeilidAPIError):
    """Argument is not valid in this context"""
    context: str
    argument: str
    value: str
    def __init__(self, context: str, argument: str, value: str):
        super().__init__("Invalid argument")
        self.context = context
        self.argument = argument
        self.value = value

class VeilidAPIErrorMissingArgument(VeilidAPIError):
    """Required argument was missing"""
    context: str
    argument: str
    def __init__(self, context: str, argument: str):
        super().__init__("Missing argument")
        self.context = context
        self.argument = argument

class VeilidAPIErrorGeneric(VeilidAPIError):
    """Generic error message"""
    message: str
    def __init__(self, message: str):
        super().__init__("Generic")
        self.message = message
