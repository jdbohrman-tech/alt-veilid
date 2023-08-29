use cfg_if::*;
use core::future::Future;

cfg_if! {
    if #[cfg(feature="rt-async-std")] {
        pub use async_std::task::JoinHandle;
        pub use async_std::net::TcpStream;
        //pub use async_std::future::TimeoutError;
        pub use async_std::sync::Mutex as AsyncMutex;

        pub fn spawn<F: Future<Output = T> + Send + 'static, T: Send + 'static>(f: F) -> JoinHandle<T> {
            async_std::task::spawn(f)
        }

        pub use async_std::task::sleep;
        pub use async_std::future::timeout;
    } else if #[cfg(feature="rt-tokio")] {
        pub use tokio::task::JoinHandle;
        pub use tokio::net::TcpStream;
        //pub use tokio::time::error::Elapsed as TimeoutError;
        pub use tokio::sync::Mutex as AsyncMutex;
        pub fn spawn<F: Future<Output = T> + Send + 'static, T: Send + 'static>(f: F) -> JoinHandle<T> {
            GLOBAL_RUNTIME.spawn(f)
        }

        pub use tokio::time::sleep;
        pub use tokio::time::timeout;
        lazy_static::lazy_static! {
            static ref GLOBAL_RUNTIME: tokio::runtime::Runtime = tokio::runtime::Runtime::new().unwrap();
        }
    } else {
        compile_error!("needs executor implementation")
    }
}
