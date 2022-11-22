use super::*;
use crate::tests::common::test_veilid_config::*;
use crate::xx::*;
use crate::*;

static LOREM_IPSUM:&[u8] = b"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum. ";

async fn startup() -> VeilidAPI {
    trace!("test_table_store: starting");
    let (update_callback, config_callback) = setup_veilid_core();
    let api = api_startup(update_callback, config_callback)
        .await
        .expect("startup failed");
    api
}

async fn shutdown(api: VeilidAPI) {
    trace!("test_table_store: shutting down");
    api.shutdown().await;
    trace!("test_table_store: finished");
}

pub async fn test_aead() {
    trace!("test_aead");

    let n1 = Crypto::get_random_nonce();
    let n2 = loop {
        let n = Crypto::get_random_nonce();
        if n != n1 {
            break n;
        }
    };

    let ss1 = Crypto::get_random_secret();
    let ss2 = loop {
        let ss = Crypto::get_random_secret();
        if ss != ss1 {
            break ss;
        }
    };

    let mut body = LOREM_IPSUM.to_vec();
    let body2 = body.clone();
    let size_before_encrypt = body.len();
    assert!(
        Crypto::encrypt_in_place_aead(&mut body, &n1, &ss1, None).is_ok(),
        "encrypt should succeed"
    );
    let size_after_encrypt = body.len();
    assert!(
        size_after_encrypt - size_before_encrypt == AEAD_OVERHEAD,
        "overhead should match"
    );
    let mut body3 = body.clone();
    let mut body4 = body.clone();
    let mut body5 = body.clone();
    assert!(
        Crypto::decrypt_in_place_aead(&mut body, &n1, &ss1, None).is_ok(),
        "decrypt should succeed"
    );
    assert_eq!(body, body2, "results should be the same");

    assert!(
        Crypto::decrypt_in_place_aead(&mut body3, &n2, &ss1, None).is_err(),
        "decrypt with wrong nonce should fail"
    );
    assert_ne!(body3, body, "failure changes data");

    assert!(
        Crypto::decrypt_in_place_aead(&mut body4, &n1, &ss2, None).is_err(),
        "decrypt with wrong secret should fail"
    );
    assert_ne!(body4, body, "failure changes data");

    assert!(
        Crypto::decrypt_in_place_aead(&mut body5, &n1, &ss2, Some(b"foobar")).is_err(),
        "decrypt with wrong associated data should fail"
    );
    assert_ne!(body5, body, "failure changes data");

    assert!(
        Crypto::decrypt_aead(LOREM_IPSUM, &n1, &ss1, None).is_err(),
        "should fail authentication"
    );

    let body5 = Crypto::encrypt_aead(LOREM_IPSUM, &n1, &ss1, None).unwrap();
    let body6 = Crypto::decrypt_aead(&body5, &n1, &ss1, None).unwrap();
    let body7 = Crypto::encrypt_aead(LOREM_IPSUM, &n1, &ss1, None).unwrap();
    assert_eq!(body6, LOREM_IPSUM);
    assert_eq!(body5, body7);
}

pub async fn test_no_auth() {
    trace!("test_no_auth");

    let n1 = Crypto::get_random_nonce();
    let n2 = loop {
        let n = Crypto::get_random_nonce();
        if n != n1 {
            break n;
        }
    };

    let ss1 = Crypto::get_random_secret();
    let ss2 = loop {
        let ss = Crypto::get_random_secret();
        if ss != ss1 {
            break ss;
        }
    };

    let mut body = LOREM_IPSUM.to_vec();
    let body2 = body.clone();
    let size_before_encrypt = body.len();
    Crypto::crypt_in_place_no_auth(&mut body, &n1, &ss1);

    let size_after_encrypt = body.len();
    assert_eq!(
        size_after_encrypt, size_before_encrypt,
        "overhead should match"
    );
    let mut body3 = body.clone();
    let mut body4 = body.clone();

    Crypto::crypt_in_place_no_auth(&mut body, &n1, &ss1);
    assert_eq!(body, body2, "result after decrypt should be the same");

    Crypto::crypt_in_place_no_auth(&mut body3, &n2, &ss1);
    assert_ne!(body3, body, "decrypt should not be equal with wrong nonce");

    Crypto::crypt_in_place_no_auth(&mut body4, &n1, &ss2);
    assert_ne!(body4, body, "decrypt should not be equal with wrong secret");

    let body5 = Crypto::crypt_no_auth(LOREM_IPSUM, &n1, &ss1);
    let body6 = Crypto::crypt_no_auth(&body5, &n1, &ss1);
    let body7 = Crypto::crypt_no_auth(LOREM_IPSUM, &n1, &ss1);
    assert_eq!(body6, LOREM_IPSUM);
    assert_eq!(body5, body7);
}

pub async fn test_dh(crypto: Crypto) {
    trace!("test_dh");
    let (dht_key, dht_key_secret) = key::generate_secret();
    let (dht_key2, dht_key_secret2) = key::generate_secret();

    let r1 = Crypto::compute_dh(&dht_key, &dht_key_secret2).unwrap();
    let r2 = Crypto::compute_dh(&dht_key2, &dht_key_secret).unwrap();
    let r3 = Crypto::compute_dh(&dht_key, &dht_key_secret2).unwrap();
    let r4 = Crypto::compute_dh(&dht_key2, &dht_key_secret).unwrap();
    assert_eq!(r1, r2);
    assert_eq!(r3, r4);
    assert_eq!(r2, r3);
    trace!("dh: {:?}", r1);

    // test cache
    let r5 = crypto.cached_dh(&dht_key, &dht_key_secret2).unwrap();
    let r6 = crypto.cached_dh(&dht_key2, &dht_key_secret).unwrap();
    let r7 = crypto.cached_dh(&dht_key, &dht_key_secret2).unwrap();
    let r8 = crypto.cached_dh(&dht_key2, &dht_key_secret).unwrap();
    assert_eq!(r1, r5);
    assert_eq!(r2, r6);
    assert_eq!(r3, r7);
    assert_eq!(r4, r8);
    trace!("cached_dh: {:?}", r5);
}

pub async fn test_all() {
    let api = startup().await;
    let crypto = api.crypto().unwrap();
    test_aead().await;
    test_no_auth().await;
    test_dh(crypto).await;
    shutdown(api.clone()).await;
    assert!(api.is_shutdown());
}
