use super::*;

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        use futures_util::future::{select, Either};

        pub async fn timeout<F, T>(dur_ms: u32, f: F) -> Result<T, TimeoutError>
        where
            F: Future<Output = T>,
        {
            match select(Box::pin(sleep(dur_ms)), Box::pin(f)).await {
                Either::Left((_x, _b)) => Err(TimeoutError()),
                Either::Right((y, _a)) => Ok(y),
            }
        }

    } else {

        pub async fn timeout<F, T>(dur_ms: u32, f: F) -> Result<T, TimeoutError>
        where
            F: Future<Output = T>,
        {
            cfg_if! {
                if #[cfg(feature="rt-async-std")] {
                    async_std::future::timeout(Duration::from_millis(dur_ms as u64), f).await.map_err(|e| e.into())
                } else if #[cfg(feature="rt-tokio")] {
                    tokio::time::timeout(Duration::from_millis(dur_ms as u64), f).await.map_err(|e| e.into())
                }
            }
        }

    }
}
