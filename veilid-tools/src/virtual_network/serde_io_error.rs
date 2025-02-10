use serde::*;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[allow(deprecated)]
#[non_exhaustive]
#[serde(remote = "std::io::ErrorKind")]
pub enum SerdeIoErrorKindDef {
    NotFound,
    PermissionDenied,
    ConnectionRefused,
    ConnectionReset,
    // #[cfg(feature = "io_error_more")]
    // HostUnreachable,
    // #[cfg(feature = "io_error_more")]
    // NetworkUnreachable,
    ConnectionAborted,
    NotConnected,
    AddrInUse,
    AddrNotAvailable,
    // #[cfg(feature = "io_error_more")]
    // NetworkDown,
    BrokenPipe,
    AlreadyExists,
    WouldBlock,
    // #[cfg(feature = "io_error_more")]
    // NotADirectory,
    // #[cfg(feature = "io_error_more")]
    // IsADirectory,
    // #[cfg(feature = "io_error_more")]
    // DirectoryNotEmpty,
    // #[cfg(feature = "io_error_more")]
    // ReadOnlyFilesystem,
    // #[cfg(feature = "io_error_more")]
    // FilesystemLoop,
    // #[cfg(feature = "io_error_more")]
    // StaleNetworkFileHandle,
    InvalidInput,
    InvalidData,
    TimedOut,
    WriteZero,
    // #[cfg(feature = "io_error_more")]
    // StorageFull,
    // #[cfg(feature = "io_error_more")]
    // NotSeekable,
    // #[cfg(feature = "io_error_more")]
    // FilesystemQuotaExceeded,
    // #[cfg(feature = "io_error_more")]
    // FileTooLarge,
    // #[cfg(feature = "io_error_more")]
    // ResourceBusy,
    // #[cfg(feature = "io_error_more")]
    // ExecutableFileBusy,
    // #[cfg(feature = "io_error_more")]
    // Deadlock,
    // #[cfg(feature = "io_error_more")]
    // CrossesDevices,
    // #[cfg(feature = "io_error_more")]
    // TooManyLinks,
    // #[cfg(feature = "io_error_more")]
    // InvalidFilename,
    // #[cfg(feature = "io_error_more")]
    // ArgumentListTooLong,
    Interrupted,
    Unsupported,
    UnexpectedEof,
    OutOfMemory,
    Other,
    // #[cfg(feature = "io_error_uncategorized")]
    // Uncategorized,
}
