use cfg_if::*;
use core::future::Future;

cfg_if! {
    if #[cfg(feature="rt-async-std")] {
        pub use async_std::task::JoinHandle;
        pub use async_std::net::TcpStream;
        pub use async_std::future::TimeoutError;

        pub use async_std::task::sleep;
        pub use async_std::future::timeout;
        pub fn block_on<F: Future<Output = T>, T>(f: F) -> T {
            async_std::task::block_on(f)
        }
    } else if #[cfg(feature="rt-tokio")] {
        pub use tokio::task::JoinHandle;
        pub use tokio::net::TcpStream;
        pub use tokio::time::error::Elapsed as TimeoutError;

        pub use tokio::time::sleep;
        pub use tokio::time::timeout;
        pub fn block_on<F: Future<Output = T>, T>(f: F) -> T {
            let rt = tokio::runtime::Runtime::new().unwrap();
            let local = tokio::task::LocalSet::new();
            local.block_on(&rt, f)
        }

    }
}
