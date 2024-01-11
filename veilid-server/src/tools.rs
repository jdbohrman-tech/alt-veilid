pub use cfg_if::*;
pub use color_eyre::eyre::{bail, eyre, Result as EyreResult, WrapErr};
pub use core::future::Future;
pub use parking_lot::*;
pub use tracing::*;

cfg_if! {
    if #[cfg(feature="rt-async-std")] {
//        pub use async_std::task::JoinHandle;
        pub use async_std::net::TcpListener;
        pub use async_std::net::TcpStream;
        pub use async_std::io::BufReader;
        //pub use async_std::future::TimeoutError;
        //pub fn spawn_detached<F: Future<Output = T> + Send + 'static, T: Send + 'static>(f: F) -> JoinHandle<T> {
            //async_std::task::spawn(f)
        //}
        // pub fn spawn_local<F: Future<Output = T> + 'static, T: 'static>(f: F) -> JoinHandle<T> {
        //     async_std::task::spawn_local(f)
        // }
        // pub fn spawn_detached_local<F: Future<Output = T> + 'static, T: 'static>(f: F) {
        //     let _ = async_std::task::spawn_local(f);
        // }
        //pub use async_std::task::sleep;
        //pub use async_std::future::timeout;
        pub fn block_on<F: Future<Output = T>, T>(f: F) -> T {
            async_std::task::block_on(f)
        }
    } else if #[cfg(feature="rt-tokio")] {
        //pub use tokio::task::JoinHandle;
        pub use tokio::net::TcpListener;
        pub use tokio::net::TcpStream;
        pub use tokio::io::BufReader;
        //pub use tokio_util::compat::*;
        //pub use tokio::time::error::Elapsed as TimeoutError;
        //pub fn spawn_detached<F: Future<Output = T> + Send + 'static, T:  Send + 'static>(f: F) -> JoinHandle<T> {
            //tokio::task::spawn(f)
        //}
        // pub fn spawn_local<F: Future<Output = T> + 'static, T: 'static>(f: F) -> JoinHandle<T> {
        //     tokio::task::spawn_local(f)
        // }
        // pub fn spawn_detached_local<F: Future<Output = T> + 'static, T: 'static>(f: F) {
        //     let _ = tokio::task::spawn_local(f);
        // }
        //pub use tokio::time::sleep;
        //pub use tokio::time::timeout;
        pub fn block_on<F: Future<Output = T>, T>(f: F) -> T {
            let rt = tokio::runtime::Runtime::new().unwrap();
            let local = tokio::task::LocalSet::new();
            local.block_on(&rt, f)
        }
    } else {
        compile_error!("needs executor implementation")
    }
}
