use cfg_if::*;
use std::path::Path;

cfg_if! {
    if #[cfg(feature="rt-tokio")] {
        mod ipc_tokio;
        pub use ipc_tokio::*;
    } else if #[cfg(feature="rt-async-std")] {
        mod ipc_async_std;
        pub use ipc_async_std::*;
    }
}

#[allow(unused_variables)]
pub fn is_ipc_socket_path<P: AsRef<Path>>(path: P) -> bool {
    cfg_if! {
        if #[cfg(windows)] {
            let p = path.as_ref().to_path_buf().to_string_lossy().to_string().to_lowercase();
            p.starts_with(r"\\.\pipe") && path.as_ref().exists()
        } else if #[cfg(unix)] {
            use std::os::unix::fs::FileTypeExt;
            let meta = match std::fs::metadata(path) {
                Ok(v) => v,
                Err(_) => {
                    return false;
                }
            };
            meta.file_type().is_socket()
        } else {
            false
        }
    }
}
