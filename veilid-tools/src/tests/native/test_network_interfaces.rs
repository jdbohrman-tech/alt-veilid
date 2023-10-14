use crate::*;

cfg_if! {
    if #[cfg(not(target_arch = "wasm32"))] {
        use network_interfaces::NetworkInterfaces;

        pub async fn test_network_interfaces() {
            info!("testing network interfaces");
            let t1 = get_timestamp();
            let interfaces = NetworkInterfaces::new();
            let count = 100;
            for x in 0..count {
                info!("loop {}", x);
                if let Err(e) = interfaces.refresh().await {
                    error!("error refreshing interfaces: {}", e);
                }
            }
            let t2 = get_timestamp();
            let tdiff = ((t2 - t1) as f64)/1000000.0f64;
            info!("running network interface test with {} iterations took {} seconds", count, tdiff);
            //info!("interfaces: {:#?}", interfaces)
        }
    }
}

pub async fn test_all() {
    #[cfg(not(target_arch = "wasm32"))]
    test_network_interfaces().await;
}
