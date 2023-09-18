use super::test_veilid_config::*;
use crate::*;

async fn startup() -> VeilidAPI {
    trace!("test_table_store: starting");
    let (update_callback, config_callback) = setup_veilid_core();
    api_startup(update_callback, config_callback)
        .await
        .expect("startup failed")
}

async fn shutdown(api: VeilidAPI) {
    trace!("test_table_store: shutting down");
    api.shutdown().await;
    trace!("test_table_store: finished");
}

pub async fn test_protected_store(ps: ProtectedStore) {
    info!("testing protected store");

    let _ = ps.remove_user_secret("_test_key").await;
    let _ = ps.remove_user_secret("_test_broken").await;

    let d1: [u8; 0] = [];

    assert!(!ps
        .save_user_secret("_test_key", &[2u8, 3u8, 4u8])
        .await
        .unwrap());
    info!("testing saving user secret");
    assert!(ps.save_user_secret("_test_key", &d1).await.unwrap());
    info!("testing loading user secret");
    assert_eq!(
        ps.load_user_secret("_test_key").await.unwrap(),
        Some(d1.to_vec())
    );
    info!("testing loading user secret again");
    assert_eq!(
        ps.load_user_secret("_test_key").await.unwrap(),
        Some(d1.to_vec())
    );
    info!("testing loading broken user secret");
    assert_eq!(ps.load_user_secret("_test_broken").await.unwrap(), None);
    info!("testing loading broken user secret again");
    assert_eq!(ps.load_user_secret("_test_broken").await.unwrap(), None);
    info!("testing remove user secret");
    assert!(ps.remove_user_secret("_test_key").await.unwrap());
    info!("testing remove user secret again");
    assert!(!ps.remove_user_secret("_test_key").await.unwrap());
    info!("testing remove broken user secret");
    assert!(!ps.remove_user_secret("_test_broken").await.unwrap());
    info!("testing remove broken user secret again");
    assert!(!ps.remove_user_secret("_test_broken").await.unwrap());

    let d2: [u8; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    assert!(!ps
        .save_user_secret("_test_key", &[2u8, 3u8, 4u8])
        .await
        .unwrap());
    assert!(ps.save_user_secret("_test_key", &d2).await.unwrap());
    assert_eq!(
        ps.load_user_secret("_test_key").await.unwrap(),
        Some(d2.to_vec())
    );
    assert_eq!(
        ps.load_user_secret("_test_key").await.unwrap(),
        Some(d2.to_vec())
    );
    assert_eq!(ps.load_user_secret("_test_broken").await.unwrap(), None);
    assert_eq!(ps.load_user_secret("_test_broken").await.unwrap(), None);
    assert!(ps.remove_user_secret("_test_key").await.unwrap());
    assert!(!ps.remove_user_secret("_test_key").await.unwrap());
    assert!(!ps.remove_user_secret("_test_key").await.unwrap());
    assert!(!ps.remove_user_secret("_test_broken").await.unwrap());

    let _ = ps.remove_user_secret("_test_key").await;
    let _ = ps.remove_user_secret("_test_broken").await;
}

pub async fn test_all() {
    let api = startup().await;
    let ps = api.protected_store().unwrap();
    test_protected_store(ps.clone()).await;

    shutdown(api).await;
}
