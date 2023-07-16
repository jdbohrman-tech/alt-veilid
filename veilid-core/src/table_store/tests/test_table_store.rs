use crate::tests::test_veilid_config::*;
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

pub async fn test_delete_open_delete(ts: TableStore) {
    trace!("test_delete_open_delete");

    let _ = ts.delete("test");
    let db = ts.open("test", 3).await.expect("should have opened");
    assert!(
        ts.delete("test").await.is_err(),
        "should fail because file is opened"
    );
    drop(db);
    assert!(
        ts.delete("test").await.is_ok(),
        "should succeed because file is closed"
    );
    let db = ts.open("test", 3).await.expect("should have opened");
    assert!(
        ts.delete("test").await.is_err(),
        "should fail because file is opened"
    );
    drop(db);
    let db = ts.open("test", 3).await.expect("should have opened");
    assert!(
        ts.delete("test").await.is_err(),
        "should fail because file is opened"
    );
    drop(db);
    assert!(
        ts.delete("test").await.is_ok(),
        "should succeed because file is closed"
    );
}

pub async fn test_store_delete_load(ts: TableStore) {
    trace!("test_store_delete_load");

    let _ = ts.delete("test");
    let db = ts.open("test", 3).await.expect("should have opened");
    assert!(
        ts.delete("test").await.is_err(),
        "should fail because file is opened"
    );

    assert_eq!(
        db.load(0, b"foo").await.unwrap(),
        None,
        "should not load missing key"
    );
    assert!(
        db.store(1, b"foo", b"1234567890").await.is_ok(),
        "should store new key"
    );
    assert_eq!(
        db.load(0, b"foo").await.unwrap(),
        None,
        "should not load missing key"
    );
    assert_eq!(
        db.load(1, b"foo").await.unwrap(),
        Some(b"1234567890".to_vec())
    );

    assert!(
        db.store(1, b"bar", b"FNORD").await.is_ok(),
        "should store new key"
    );
    assert!(
        db.store(0, b"bar", b"ABCDEFGHIJKLMNOPQRSTUVWXYZ")
            .await
            .is_ok(),
        "should store new key"
    );
    assert!(
        db.store(2, b"bar", b"FNORD").await.is_ok(),
        "should store new key"
    );
    assert!(
        db.store(2, b"baz", b"QWERTY").await.is_ok(),
        "should store new key"
    );
    assert!(
        db.store(2, b"bar", b"QWERTYUIOP").await.is_ok(),
        "should store new key"
    );

    assert_eq!(db.load(1, b"bar").await.unwrap(), Some(b"FNORD".to_vec()));
    assert_eq!(
        db.load(0, b"bar").await.unwrap(),
        Some(b"ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_vec())
    );
    assert_eq!(
        db.load(2, b"bar").await.unwrap(),
        Some(b"QWERTYUIOP".to_vec())
    );
    assert_eq!(db.load(2, b"baz").await.unwrap(), Some(b"QWERTY".to_vec()));

    assert_eq!(db.delete(1, b"bar").await.unwrap(), Some(b"FNORD".to_vec()));
    assert_eq!(db.delete(1, b"bar").await.unwrap(), None);
    assert!(
        db.delete(4, b"bar").await.is_err(),
        "can't delete from column that doesn't exist"
    );

    drop(db);
    let db = ts.open("test", 3).await.expect("should have opened");

    assert_eq!(db.load(1, b"bar").await.unwrap(), None);
    assert_eq!(
        db.load(0, b"bar").await.unwrap(),
        Some(b"ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_vec())
    );
    assert_eq!(
        db.load(2, b"bar").await.unwrap(),
        Some(b"QWERTYUIOP".to_vec())
    );
    assert_eq!(db.load(2, b"baz").await.unwrap(), Some(b"QWERTY".to_vec()));
}

pub async fn test_transaction(ts: TableStore) {
    trace!("test_transaction");

    let _ = ts.delete("test");
    let db = ts.open("test", 3).await.expect("should have opened");
    assert!(
        ts.delete("test").await.is_err(),
        "should fail because file is opened"
    );

    let tx = db.transact();
    assert!(tx.store(0, b"aaa", b"a-value").is_ok());
    assert!(tx.store_json(1, b"bbb", &"b-value".to_owned()).is_ok());
    assert!(tx.store(3, b"ddd", b"d-value").is_err());
    assert!(tx.store(0, b"ddd", b"d-value").is_ok());
    assert!(tx.delete(0, b"ddd").is_ok());
    assert!(tx.commit().await.is_ok());

    let tx = db.transact();
    assert!(tx.delete(2, b"ccc").is_ok());
    tx.rollback();

    assert_eq!(db.load(0, b"aaa").await, Ok(Some(b"a-value".to_vec())));
    assert_eq!(
        db.load_json::<String>(1, b"bbb").await,
        Ok(Some("b-value".to_owned()))
    );
    assert_eq!(db.load(0, b"ddd").await, Ok(None));
}

pub async fn test_json(vcrypto: CryptoSystemVersion, ts: TableStore) {
    trace!("test_json");

    let _ = ts.delete("test");
    let db = ts.open("test", 3).await.expect("should have opened");
    let keypair = vcrypto.generate_keypair();

    assert!(db.store_json(0, b"asdf", &keypair).await.is_ok());

    assert_eq!(db.load_json::<KeyPair>(0, b"qwer").await.unwrap(), None);

    let d = match db.load_json::<KeyPair>(0, b"asdf").await {
        Ok(x) => x,
        Err(e) => {
            panic!("couldn't decode: {}", e);
        }
    };
    assert_eq!(d, Some(keypair), "keys should be equal");

    let d = match db.delete_json::<KeyPair>(0, b"asdf").await {
        Ok(x) => x,
        Err(e) => {
            panic!("couldn't decode: {}", e);
        }
    };
    assert_eq!(d, Some(keypair), "keys should be equal");

    assert!(
        db.store(1, b"foo", b"1234567890").await.is_ok(),
        "should store new key"
    );

    assert!(
        db.load_json::<TypedKey>(1, b"foo").await.is_err(),
        "should fail to unfreeze"
    );
}

pub async fn test_protect_unprotect(vcrypto: CryptoSystemVersion, ts: TableStore) {
    trace!("test_protect_unprotect");

    let dek1 = TypedSharedSecret::new(
        vcrypto.kind(),
        SharedSecret::new([
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ]),
    );
    let dek2 = TypedSharedSecret::new(
        vcrypto.kind(),
        SharedSecret::new([
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0xFF,
        ]),
    );
    let dek3 = TypedSharedSecret::new(
        vcrypto.kind(),
        SharedSecret::new([0x80u8; SHARED_SECRET_LENGTH]),
    );

    let deks = [dek1, dek2, dek3];
    let passwords = ["", " ", "  ", "12345678", "|/\\!@#$%^&*()_+", "Ⓜ️", "🔥🔥♾️"];

    for dek in deks {
        for password in passwords {
            let dek_bytes = ts
                .maybe_protect_device_encryption_key(dek, password)
                .expect(&format!("protect: dek: '{}' pw: '{}'", dek, password));
            let unprotected = ts
                .maybe_unprotect_device_encryption_key(&dek_bytes, password)
                .expect(&format!("unprotect: dek: '{}' pw: '{}'", dek, password));
            assert_eq!(unprotected, dek);
            let invalid_password = format!("{}x", password);
            let _ = ts
                .maybe_unprotect_device_encryption_key(&dek_bytes, &invalid_password)
                .expect_err(&format!(
                    "invalid_password: dek: '{}' pw: '{}'",
                    dek, &invalid_password
                ));
            if password != "" {
                let _ = ts
                    .maybe_unprotect_device_encryption_key(&dek_bytes, "")
                    .expect_err(&format!("empty_password: dek: '{}' pw: ''", dek));
            }
        }
    }
}

pub async fn test_all() {
    let api = startup().await;
    let crypto = api.crypto().unwrap();
    let ts = api.table_store().unwrap();

    for ck in VALID_CRYPTO_KINDS {
        let vcrypto = crypto.get(ck).unwrap();
        test_protect_unprotect(vcrypto.clone(), ts.clone()).await;
        test_delete_open_delete(ts.clone()).await;
        test_store_delete_load(ts.clone()).await;
        test_transaction(ts.clone()).await;
        test_json(vcrypto, ts.clone()).await;
        let _ = ts.delete("test").await;
    }

    shutdown(api).await;
}
