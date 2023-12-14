use cfg_if::*;

cfg_if! {
    if #[cfg(feature="rt-tokio")] {
        mod ipc_tokio;
        pub use ipc_tokio::*;
    } else if #[cfg(feature="rt-async-std")] {
        mod ipc_async_std;
        pub use ipc_async_std::*;
    }
}
