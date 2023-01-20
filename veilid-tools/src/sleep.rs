use super::*;
use std::time::Duration;

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        use async_executors::{Bindgen, Timer};

        pub async fn sleep(millis: u32) {
            Bindgen.sleep(Duration::from_millis(millis.into())).await
        }

    } else {

        pub async fn sleep(millis: u32) {
            if millis == 0 {
                cfg_if! {
                    if #[cfg(feature="rt-async-std")] {
                        async_std::task::yield_now().await;
                    } else if #[cfg(feature="rt-tokio")] {
                        tokio::task::yield_now().await;
                    }
                }
            } else {
                cfg_if! {
                    if #[cfg(feature="rt-async-std")] {
                        async_std::task::sleep(Duration::from_millis(u64::from(millis))).await;
                    } else if #[cfg(feature="rt-tokio")] {
                        tokio::time::sleep(Duration::from_millis(u64::from(millis))).await;
                    }
                }
            }
        }
    }
}
