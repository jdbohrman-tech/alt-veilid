pub use cfg_if::*;
pub use log::*;
pub use parking_lot::*;
pub use veilid_tools::*;

use core::future::Future;
use core::str::FromStr;

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
    } else {
        compile_error!("needs executor implementation")
    }
}

pub fn json_str_u64(value: &json::JsonValue) -> u64 {
    u64::from_str(value.as_str().unwrap_or_default()).unwrap_or_default()
}

pub fn json_str_vec_u8(value: &json::JsonValue) -> Vec<u8> {
    data_encoding::BASE64URL_NOPAD
        .decode(value.as_str().unwrap_or_default().as_bytes())
        .unwrap_or_default()
}
