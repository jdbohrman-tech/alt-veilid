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

pub async fn test_startup_shutdown_from_config() {
    trace!("test_startup_from_config: starting");
    let config = VeilidConfigInner {
        program_name: "VeilidCoreTests".into(),
        table_store: VeilidConfigTableStore {
            directory: get_table_store_path(),
            delete: true,
            ..Default::default()
        },
        block_store: VeilidConfigBlockStore {
            directory: get_block_store_path(),
            delete: true,
            ..Default::default()
        },
        protected_store: VeilidConfigProtectedStore {
            allow_insecure_fallback: true,
            directory: get_protected_store_path(),
            device_encryption_key_password: "".to_owned(),
            delete: true,
            ..Default::default()
        },
        ..Default::default()
    };
    let api = api_startup_config(Arc::new(|_: VeilidUpdate| {}), config)
        .await
        .expect("startup failed");
    trace!("test_startup_from_config: shutting down");
    api.shutdown().await;
    trace!("test_startup_from_config: finished");
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
    test_startup_shutdown_from_config().await;
    test_attach_detach().await;
}
