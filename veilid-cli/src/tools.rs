use cfg_if::*;
use core::future::Future;

cfg_if! {
    if #[cfg(feature="rt-async-std")] {
        pub use async_std::net::TcpStream;
        pub fn block_on<F: Future<Output = T>, T>(f: F) -> T {
            async_std::task::block_on(f)
        }
    } else if #[cfg(feature="rt-tokio")] {
        pub use tokio::net::TcpStream;
        pub fn block_on<F: Future<Output = T>, T>(f: F) -> T {
            let rt = tokio::runtime::Runtime::new().unwrap();
            let local = tokio::task::LocalSet::new();
            local.block_on(&rt, f)
        }

    }
}
