use super::test_veilid_config::*;
use crate::*;

pub async fn test_startup_shutdown() {
    trace!("test_startup_shutdown: starting");
    let (update_callback, config_callback) = setup_veilid_core();
    let api = api_startup(update_callback, config_callback)
        .await
        .expect("startup failed");
    trace!("test_startup_shutdown: shutting down");
    api.shutdown().await;
    trace!("test_startup_shutdown: finished");
}

pub async fn test_attach_detach() {
    info!("--- test normal order ---");
    let (update_callback, config_callback) = setup_veilid_core();
    let api = api_startup(update_callback, config_callback)
        .await
        .expect("startup failed");
    api.attach().await.unwrap();
    sleep(5000).await;
    api.detach().await.unwrap();
    sleep(2000).await;
    api.shutdown().await;

    info!("--- test auto detach ---");
    let (update_callback, config_callback) = setup_veilid_core();
    let api = api_startup(update_callback, config_callback)
        .await
        .expect("startup failed");
    api.attach().await.unwrap();
    sleep(5000).await;
    api.shutdown().await;

    info!("--- test detach without attach ---");
    let (update_callback, config_callback) = setup_veilid_core();
    let api = api_startup(update_callback, config_callback)
        .await
        .expect("startup failed");
    assert!(api.detach().await.is_err());
    api.shutdown().await;
}

pub async fn test_all() {
    test_startup_shutdown().await;
    test_attach_detach().await;
}
