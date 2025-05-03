use super::test_veilid_config::*;
use crate::*;

pub async fn test_startup_shutdown() {
    trace!("test_startup_shutdown: starting");
    let (update_callback, config_callback) = setup_veilid_core();
    let api = api_startup(update_callback, config_callback)
        .await
        .expect("startup failed");

    // Test initial state
    assert!(!api.is_shutdown(), "API should not be shut down initially");

    trace!("test_startup_shutdown: shutting down");
    let api_clone = api.clone();
    api.shutdown().await;

    // Test state after shutdown
    assert!(
        api_clone.is_shutdown(),
        "API should be shut down after shutdown()"
    );

    trace!("test_startup_shutdown: finished");
}

pub async fn test_startup_shutdown_from_config() {
    trace!("test_startup_from_config: starting");
    let config = VeilidConfig {
        program_name: "VeilidCoreTests".into(),
        table_store: VeilidConfigTableStore {
            directory: get_table_store_path(),
            delete: true,
            // ..Default::default()
        },
        block_store: VeilidConfigBlockStore {
            directory: get_block_store_path(),
            delete: true,
            //..Default::default()
        },
        protected_store: VeilidConfigProtectedStore {
            allow_insecure_fallback: true,
            always_use_insecure_storage: true,
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
    trace!("test_attach_detach: --- test normal order ---");
    let (update_callback, config_callback) = setup_veilid_core();
    let api = api_startup(update_callback, config_callback)
        .await
        .expect("startup failed");
    api.attach().await.unwrap();
    sleep(5000).await;
    api.detach().await.unwrap();
    sleep(2000).await;
    api.shutdown().await;

    trace!("test_attach_detach: --- test auto detach ---");
    let (update_callback, config_callback) = setup_veilid_core();
    let api = api_startup(update_callback, config_callback)
        .await
        .expect("startup failed");
    api.attach().await.unwrap();
    sleep(5000).await;
    api.shutdown().await;

    trace!("test_attach_detach: --- test detach without attach ---");
    let (update_callback, config_callback) = setup_veilid_core();
    let api = api_startup(update_callback, config_callback)
        .await
        .expect("startup failed");
    assert!(api.detach().await.is_err());
    api.shutdown().await;
}

pub async fn test_startup_shutdown_multiple() {
    trace!("test_startup_shutdown_multiple: starting");
    let namespaces = (0..3).map(|x| format!("ns_{}", x)).collect::<Vec<_>>();

    let mut apis = vec![];
    for ns in &namespaces {
        let (update_callback, config_callback) = setup_veilid_core_with_namespace(ns);
        let api = api_startup(update_callback, config_callback)
            .await
            .expect("startup failed");
        apis.push(api);
    }
    trace!("test_startup_shutdown_multiple: shutting down");
    for api in apis {
        api.shutdown().await;
    }
    trace!("test_startup_shutdown_multiple: finished");
}

pub async fn test_startup_shutdown_from_config_multiple() {
    trace!("test_startup_from_config_multiple: starting");

    let namespaces = (0..3).map(|x| format!("ns_{}", x)).collect::<Vec<_>>();
    let mut apis = vec![];
    for ns in &namespaces {
        let config = VeilidConfig {
            program_name: "VeilidCoreTests".into(),
            namespace: ns.to_owned(),
            table_store: VeilidConfigTableStore {
                directory: get_table_store_path(),
                delete: true,
                // ..Default::default()
            },
            block_store: VeilidConfigBlockStore {
                directory: get_block_store_path(),
                delete: true,
                //..Default::default()
            },
            protected_store: VeilidConfigProtectedStore {
                allow_insecure_fallback: true,
                always_use_insecure_storage: true,
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
        apis.push(api);
    }
    trace!("test_startup_from_config_multiple: shutting down");
    for api in apis {
        api.shutdown().await;
    }
    trace!("test_startup_from_config_multiple: finished");
}

pub async fn test_attach_detach_multiple() {
    trace!("test_attach_detach_multiple: --- test normal order ---");
    let namespaces = (0..3).map(|x| format!("ns_{}", x)).collect::<Vec<_>>();
    let mut apis = vec![];
    for ns in &namespaces {
        let (update_callback, config_callback) = setup_veilid_core_with_namespace(ns);
        let api = api_startup(update_callback, config_callback)
            .await
            .expect("startup failed");
        apis.push(api);
    }
    for api in &apis {
        api.attach().await.unwrap();
    }
    sleep(5000).await;
    for api in &apis {
        api.detach().await.unwrap();
    }
    sleep(2000).await;
    for api in apis {
        api.shutdown().await;
    }

    trace!("test_attach_detach_multiple: --- test auto detach ---");
    let mut apis = vec![];
    for ns in &namespaces {
        let (update_callback, config_callback) = setup_veilid_core_with_namespace(ns);
        let api = api_startup(update_callback, config_callback)
            .await
            .expect("startup failed");
        apis.push(api);
    }

    for api in &apis {
        api.attach().await.unwrap();
    }
    sleep(5000).await;
    for api in apis {
        api.shutdown().await;
    }

    trace!("test_attach_detach_multiple: --- test detach without attach ---");
    let mut apis = vec![];
    for ns in &namespaces {
        let (update_callback, config_callback) = setup_veilid_core_with_namespace(ns);
        let api = api_startup(update_callback, config_callback)
            .await
            .expect("startup failed");
        apis.push(api);
    }
    for api in &apis {
        assert!(api.detach().await.is_err());
    }
    for api in apis {
        api.shutdown().await;
    }
}

pub async fn test_all() {
    test_startup_shutdown().await;
    test_startup_shutdown_from_config().await;
    test_attach_detach().await;
    test_startup_shutdown_multiple().await;
    test_startup_shutdown_from_config_multiple().await;
    test_attach_detach_multiple().await;
}
