use super::*;

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {

        pub fn interval<F, FUT>(name: &str, freq_ms: u32, callback: F) -> SendPinBoxFuture<()>
        where
            F: Fn() -> FUT + Send + Sync + 'static,
            FUT: Future<Output = ()> + Send,
        {
            let e = Eventual::new();

            let ie = e.clone();
            let jh = spawn(name, Box::pin(async move {
                while timeout(freq_ms, ie.instance_clone(())).await.is_err() {
                    callback().await;
                }
            }));

            Box::pin(async move {
                e.resolve().await;
                jh.await;
            })
        }

    } else {

        pub fn interval<F, FUT>(name: &str, freq_ms: u32, callback: F) -> SendPinBoxFuture<()>
        where
            F: Fn() -> FUT + Send + Sync + 'static,
            FUT: Future<Output = ()> + Send,
        {
            let e = Eventual::new();

            let ie = e.clone();
            let jh = spawn(name, async move {
                while timeout(freq_ms, ie.instance_clone(())).await.is_err() {
                    callback().await;
                }
            });

            Box::pin(async move {
                e.resolve().await;
                jh.await;
            })
        }

    }
}
