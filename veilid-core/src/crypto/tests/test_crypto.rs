use super::*;

static LOREM_IPSUM:&[u8] = b"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum. ";

pub async fn test_aead(vcrypto: CryptoSystemVersion) {
    trace!("test_aead");

    let n1 = vcrypto.random_nonce();
    let n2 = loop {
        let n = vcrypto.random_nonce();
        if n != n1 {
            break n;
        }
    };

    let ss1 = vcrypto.random_shared_secret();
    let ss2 = loop {
        let ss = vcrypto.random_shared_secret();
        if ss != ss1 {
            break ss;
        }
    };

    let mut body = LOREM_IPSUM.to_vec();
    let body2 = body.clone();
    let size_before_encrypt = body.len();
    assert!(
        vcrypto
            .encrypt_in_place_aead(&mut body, &n1, &ss1, None)
            .is_ok(),
        "encrypt should succeed"
    );
    let size_after_encrypt = body.len();
    assert!(
        size_after_encrypt - size_before_encrypt == vcrypto.aead_overhead(),
        "overhead should match"
    );
    let mut body3 = body.clone();
    let mut body4 = body.clone();
    let mut body5 = body.clone();
    assert!(
        vcrypto
            .decrypt_in_place_aead(&mut body, &n1, &ss1, None)
            .is_ok(),
        "decrypt should succeed"
    );
    assert_eq!(body, body2, "results should be the same");

    assert!(
        vcrypto
            .decrypt_in_place_aead(&mut body3, &n2, &ss1, None)
            .is_err(),
        "decrypt with wrong nonce should fail"
    );
    assert_ne!(body3, body, "failure changes data");

    assert!(
        vcrypto
            .decrypt_in_place_aead(&mut body4, &n1, &ss2, None)
            .is_err(),
        "decrypt with wrong secret should fail"
    );
    assert_ne!(body4, body, "failure changes data");

    assert!(
        vcrypto
            .decrypt_in_place_aead(&mut body5, &n1, &ss2, Some(b"foobar"))
            .is_err(),
        "decrypt with wrong associated data should fail"
    );
    assert_ne!(body5, body, "failure changes data");

    assert!(
        vcrypto.decrypt_aead(LOREM_IPSUM, &n1, &ss1, None).is_err(),
        "should fail authentication"
    );

    let body5 = vcrypto.encrypt_aead(LOREM_IPSUM, &n1, &ss1, None).unwrap();
    let body6 = vcrypto.decrypt_aead(&body5, &n1, &ss1, None).unwrap();
    let body7 = vcrypto.encrypt_aead(LOREM_IPSUM, &n1, &ss1, None).unwrap();
    assert_eq!(body6, LOREM_IPSUM);
    assert_eq!(body5, body7);
}

pub async fn test_no_auth(vcrypto: CryptoSystemVersion) {
    trace!("test_no_auth");

    let n1 = vcrypto.random_nonce();
    let n2 = loop {
        let n = vcrypto.random_nonce();
        if n != n1 {
            break n;
        }
    };

    let ss1 = vcrypto.random_shared_secret();
    let ss2 = loop {
        let ss = vcrypto.random_shared_secret();
        if ss != ss1 {
            break ss;
        }
    };

    let mut body = LOREM_IPSUM.to_vec();
    let body2 = body.clone();
    let size_before_encrypt = body.len();
    vcrypto.crypt_in_place_no_auth(&mut body, &n1, &ss1);

    let size_after_encrypt = body.len();
    assert_eq!(
        size_after_encrypt, size_before_encrypt,
        "overhead should match"
    );
    let mut body3 = body.clone();
    let mut body4 = body.clone();

    vcrypto.crypt_in_place_no_auth(&mut body, &n1, &ss1);
    assert_eq!(body, body2, "result after decrypt should be the same");

    vcrypto.crypt_in_place_no_auth(&mut body3, &n2, &ss1);
    assert_ne!(body3, body, "decrypt should not be equal with wrong nonce");

    vcrypto.crypt_in_place_no_auth(&mut body4, &n1, &ss2);
    assert_ne!(body4, body, "decrypt should not be equal with wrong secret");

    let body5 = vcrypto.crypt_no_auth_unaligned(LOREM_IPSUM, &n1, &ss1);
    let body6 = vcrypto.crypt_no_auth_unaligned(&body5, &n1, &ss1);
    let body7 = vcrypto.crypt_no_auth_unaligned(LOREM_IPSUM, &n1, &ss1);
    assert_eq!(body6, LOREM_IPSUM);
    assert_eq!(body5, body7);

    let body5 = vcrypto.crypt_no_auth_aligned_8(LOREM_IPSUM, &n1, &ss1);
    let body6 = vcrypto.crypt_no_auth_aligned_8(&body5, &n1, &ss1);
    let body7 = vcrypto.crypt_no_auth_aligned_8(LOREM_IPSUM, &n1, &ss1);
    assert_eq!(body6, LOREM_IPSUM);
    assert_eq!(body5, body7);
}

pub async fn test_dh(vcrypto: CryptoSystemVersion) {
    trace!("test_dh");
    let (dht_key, dht_key_secret) = vcrypto.generate_keypair().into_split();
    assert!(vcrypto.validate_keypair(&dht_key, &dht_key_secret));
    let (dht_key2, dht_key_secret2) = vcrypto.generate_keypair().into_split();
    assert!(vcrypto.validate_keypair(&dht_key2, &dht_key_secret2));

    let r1 = vcrypto.compute_dh(&dht_key, &dht_key_secret2).unwrap();
    let r2 = vcrypto.compute_dh(&dht_key2, &dht_key_secret).unwrap();
    let r3 = vcrypto.compute_dh(&dht_key, &dht_key_secret2).unwrap();
    let r4 = vcrypto.compute_dh(&dht_key2, &dht_key_secret).unwrap();
    assert_eq!(r1, r2);
    assert_eq!(r3, r4);
    assert_eq!(r2, r3);
    trace!("dh: {:?}", r1);

    // test cache
    let r5 = vcrypto.cached_dh(&dht_key, &dht_key_secret2).unwrap();
    let r6 = vcrypto.cached_dh(&dht_key2, &dht_key_secret).unwrap();
    let r7 = vcrypto.cached_dh(&dht_key, &dht_key_secret2).unwrap();
    let r8 = vcrypto.cached_dh(&dht_key2, &dht_key_secret).unwrap();
    assert_eq!(r1, r5);
    assert_eq!(r2, r6);
    assert_eq!(r3, r7);
    assert_eq!(r4, r8);
    trace!("cached_dh: {:?}", r5);
}

pub async fn test_generation(vcrypto: CryptoSystemVersion) {
    let b1 = vcrypto.random_bytes(32);
    let b2 = vcrypto.random_bytes(32);
    assert_ne!(b1, b2);
    assert_eq!(b1.len(), 32);
    assert_eq!(b2.len(), 32);
    let b3 = vcrypto.random_bytes(0);
    let b4 = vcrypto.random_bytes(0);
    assert_eq!(b3, b4);
    assert_eq!(b3.len(), 0);

    assert_ne!(vcrypto.default_salt_length(), 0);

    let pstr1 = vcrypto.hash_password(b"abc123", b"qwerasdf").unwrap();
    let pstr2 = vcrypto.hash_password(b"abc123", b"qwerasdf").unwrap();
    assert_eq!(pstr1, pstr2);
    let pstr3 = vcrypto.hash_password(b"abc123", b"qwerasdg").unwrap();
    assert_ne!(pstr1, pstr3);
    let pstr4 = vcrypto.hash_password(b"abc124", b"qwerasdf").unwrap();
    assert_ne!(pstr1, pstr4);
    let pstr5 = vcrypto.hash_password(b"abc124", b"qwerasdg").unwrap();
    assert_ne!(pstr3, pstr5);

    vcrypto
        .hash_password(b"abc123", b"qwe")
        .expect_err("should reject short salt");
    vcrypto
        .hash_password(
            b"abc123",
            b"qwerqwerqwerqwerqwerqwerqwerqwerqwerqwerqwerqwerqwerqwerqwerqwerz",
        )
        .expect_err("should reject long salt");

    assert!(vcrypto.verify_password(b"abc123", &pstr1).unwrap());
    assert!(vcrypto.verify_password(b"abc123", &pstr2).unwrap());
    assert!(vcrypto.verify_password(b"abc123", &pstr3).unwrap());
    assert!(!vcrypto.verify_password(b"abc123", &pstr4).unwrap());
    assert!(!vcrypto.verify_password(b"abc123", &pstr5).unwrap());

    let ss1 = vcrypto.derive_shared_secret(b"abc123", b"qwerasdf");
    let ss2 = vcrypto.derive_shared_secret(b"abc123", b"qwerasdf");
    assert_eq!(ss1, ss2);
    let ss3 = vcrypto.derive_shared_secret(b"abc123", b"qwerasdg");
    assert_ne!(ss1, ss3);
    let ss4 = vcrypto.derive_shared_secret(b"abc124", b"qwerasdf");
    assert_ne!(ss1, ss4);
    let ss5 = vcrypto.derive_shared_secret(b"abc124", b"qwerasdg");
    assert_ne!(ss3, ss5);

    vcrypto
        .derive_shared_secret(b"abc123", b"qwe")
        .expect_err("should reject short salt");
    vcrypto
        .derive_shared_secret(
            b"abc123",
            b"qwerqwerqwerqwerqwerqwerqwerqwerqwerqwerqwerqwerqwerqwerqwerqwerz",
        )
        .expect_err("should reject long salt");
}

pub async fn test_all() {
    let api = crypto_tests_startup().await;
    let crypto = api.crypto().unwrap();

    // Test versions
    for v in VALID_CRYPTO_KINDS {
        let vcrypto = crypto.get(v).unwrap();
        test_aead(vcrypto.clone()).await;
        test_no_auth(vcrypto.clone()).await;
        test_dh(vcrypto.clone()).await;
        test_generation(vcrypto).await;
    }

    crypto_tests_shutdown(api.clone()).await;
    assert!(api.is_shutdown());
}
