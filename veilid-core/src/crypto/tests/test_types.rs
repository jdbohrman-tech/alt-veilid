#![allow(clippy::bool_assert_comparison)]

use super::*;
use core::convert::TryFrom;

static LOREM_IPSUM:&str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum. ";
static CHEEZBURGER: &str = "I can has cheezburger";
static EMPTY_KEY: [u8; PUBLIC_KEY_LENGTH] = [0u8; PUBLIC_KEY_LENGTH];
static EMPTY_KEY_SECRET: [u8; SECRET_KEY_LENGTH] = [0u8; SECRET_KEY_LENGTH];

pub async fn test_generate_secret(vcrypto: CryptoSystemVersion) {
    // Verify keys generate
    let (dht_key, dht_key_secret) = vcrypto.generate_keypair().into_split();
    let (dht_key2, dht_key_secret2) = vcrypto.generate_keypair().into_split();

    // Verify byte patterns are different between public and secret
    assert_ne!(dht_key.bytes, dht_key_secret.bytes);
    assert_ne!(dht_key2.bytes, dht_key_secret2.bytes);

    // Verify the keys and secrets are different across keypairs
    assert_ne!(dht_key, dht_key2);
    assert_ne!(dht_key_secret, dht_key_secret2);
}

pub async fn test_sign_and_verify(vcrypto: CryptoSystemVersion) {
    // Make two keys
    let (dht_key, dht_key_secret) = vcrypto.generate_keypair().into_split();
    let (dht_key2, dht_key_secret2) = vcrypto.generate_keypair().into_split();
    // Sign the same message twice
    let dht_sig = vcrypto
        .sign(&dht_key, &dht_key_secret, LOREM_IPSUM.as_bytes())
        .unwrap();
    trace!("dht_sig: {:?}", dht_sig);
    let dht_sig_b = vcrypto
        .sign(&dht_key, &dht_key_secret, LOREM_IPSUM.as_bytes())
        .unwrap();
    // Sign a second message
    let dht_sig_c = vcrypto
        .sign(&dht_key, &dht_key_secret, CHEEZBURGER.as_bytes())
        .unwrap();
    trace!("dht_sig_c: {:?}", dht_sig_c);
    // Verify they are the same signature
    assert_eq!(dht_sig, dht_sig_b);
    // Sign the same message with a different key
    let dht_sig2 = vcrypto
        .sign(&dht_key2, &dht_key_secret2, LOREM_IPSUM.as_bytes())
        .unwrap();
    // Verify a different key gives a different signature
    assert_ne!(dht_sig2, dht_sig_b);

    // Try using the wrong secret to sign
    let a1 = vcrypto
        .sign(&dht_key, &dht_key_secret, LOREM_IPSUM.as_bytes())
        .unwrap();
    let a2 = vcrypto
        .sign(&dht_key2, &dht_key_secret2, LOREM_IPSUM.as_bytes())
        .unwrap();
    let _b1 = vcrypto
        .sign(&dht_key, &dht_key_secret2, LOREM_IPSUM.as_bytes())
        .unwrap_err();
    let _b2 = vcrypto
        .sign(&dht_key2, &dht_key_secret, LOREM_IPSUM.as_bytes())
        .unwrap_err();

    assert_ne!(a1, a2);

    assert_eq!(
        vcrypto.verify(&dht_key, LOREM_IPSUM.as_bytes(), &a1),
        Ok(())
    );
    assert_eq!(
        vcrypto.verify(&dht_key2, LOREM_IPSUM.as_bytes(), &a2),
        Ok(())
    );
    assert!(vcrypto
        .verify(&dht_key, LOREM_IPSUM.as_bytes(), &a2)
        .is_err());
    assert!(vcrypto
        .verify(&dht_key2, LOREM_IPSUM.as_bytes(), &a1)
        .is_err());

    // Try verifications that should work
    assert_eq!(
        vcrypto.verify(&dht_key, LOREM_IPSUM.as_bytes(), &dht_sig),
        Ok(())
    );
    assert_eq!(
        vcrypto.verify(&dht_key, LOREM_IPSUM.as_bytes(), &dht_sig_b),
        Ok(())
    );
    assert_eq!(
        vcrypto.verify(&dht_key2, LOREM_IPSUM.as_bytes(), &dht_sig2),
        Ok(())
    );
    assert_eq!(
        vcrypto.verify(&dht_key, CHEEZBURGER.as_bytes(), &dht_sig_c),
        Ok(())
    );
    // Try verifications that shouldn't work
    assert!(vcrypto
        .verify(&dht_key2, LOREM_IPSUM.as_bytes(), &dht_sig)
        .is_err());
    assert!(vcrypto
        .verify(&dht_key, LOREM_IPSUM.as_bytes(), &dht_sig2)
        .is_err());
    assert!(vcrypto
        .verify(&dht_key2, CHEEZBURGER.as_bytes(), &dht_sig_c)
        .is_err());
    assert!(vcrypto
        .verify(&dht_key, CHEEZBURGER.as_bytes(), &dht_sig)
        .is_err());
}

pub async fn test_key_conversions(vcrypto: CryptoSystemVersion) {
    // Test default key
    let (dht_key, dht_key_secret) = (PublicKey::default(), SecretKey::default());
    assert_eq!(dht_key.bytes, EMPTY_KEY);
    assert_eq!(dht_key_secret.bytes, EMPTY_KEY_SECRET);
    let dht_key_string = String::from(&dht_key);
    trace!("dht_key_string: {:?}", dht_key_string);
    let dht_key_string2 = String::from(&dht_key);
    trace!("dht_key_string2: {:?}", dht_key_string2);
    assert_eq!(dht_key_string, dht_key_string2);

    let dht_key_secret_string = String::from(&dht_key_secret);
    trace!("dht_key_secret_string: {:?}", dht_key_secret_string);
    assert_eq!(dht_key_secret_string, dht_key_string);

    // Make different keys
    let (dht_key2, dht_key_secret2) = vcrypto.generate_keypair().into_split();
    trace!("dht_key2: {:?}", dht_key2);
    trace!("dht_key_secret2: {:?}", dht_key_secret2);
    let (dht_key3, _dht_key_secret3) = vcrypto.generate_keypair().into_split();
    trace!("dht_key3: {:?}", dht_key3);
    trace!("_dht_key_secret3: {:?}", _dht_key_secret3);

    let dht_key2_string = String::from(&dht_key2);
    let dht_key2_string2 = String::from(&dht_key2);
    let dht_key3_string = String::from(&dht_key3);
    assert_eq!(dht_key2_string, dht_key2_string2);
    assert_ne!(dht_key3_string, dht_key2_string);
    let dht_key_secret2_string = String::from(&dht_key_secret2);
    assert_ne!(dht_key_secret2_string, dht_key_secret_string);
    assert_ne!(dht_key_secret2_string, dht_key2_string);

    // Assert they convert back correctly
    let dht_key_back = PublicKey::try_from(dht_key_string.as_str()).unwrap();
    let dht_key_back2 = PublicKey::try_from(dht_key_string2.as_str()).unwrap();
    assert_eq!(dht_key_back, dht_key_back2);
    assert_eq!(dht_key_back, dht_key);
    assert_eq!(dht_key_back2, dht_key);

    let dht_key_secret_back = SecretKey::try_from(dht_key_secret_string.as_str()).unwrap();
    assert_eq!(dht_key_secret_back, dht_key_secret);

    let dht_key2_back = PublicKey::try_from(dht_key2_string.as_str()).unwrap();
    let dht_key2_back2 = PublicKey::try_from(dht_key2_string2.as_str()).unwrap();
    assert_eq!(dht_key2_back, dht_key2_back2);
    assert_eq!(dht_key2_back, dht_key2);
    assert_eq!(dht_key2_back2, dht_key2);

    let dht_key_secret2_back = SecretKey::try_from(dht_key_secret2_string.as_str()).unwrap();
    assert_eq!(dht_key_secret2_back, dht_key_secret2);

    // Assert string roundtrip
    assert_eq!(String::from(&dht_key2_back), dht_key2_string);
    // These conversions should fail
    assert!(PublicKey::try_from("whatever").is_err());
    assert!(SecretKey::try_from("whatever").is_err());
    assert!(PublicKey::try_from("").is_err());
    assert!(SecretKey::try_from("").is_err());
    assert!(PublicKey::try_from(" ").is_err());
    assert!(SecretKey::try_from(" ").is_err());
    assert!(PublicKey::try_from(
        "qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq"
    )
    .is_err());
    assert!(SecretKey::try_from(
        "qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq"
    )
    .is_err());
}

pub async fn test_encode_decode(vcrypto: CryptoSystemVersion) {
    let dht_key = PublicKey::try_decode("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA").unwrap();
    let dht_key_secret =
        SecretKey::try_decode("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA").unwrap();
    let dht_key_b = PublicKey::new(EMPTY_KEY);
    let dht_key_secret_b = SecretKey::new(EMPTY_KEY_SECRET);
    assert_eq!(dht_key, dht_key_b);
    assert_eq!(dht_key_secret, dht_key_secret_b);

    let (dht_key2, dht_key_secret2) = vcrypto.generate_keypair().into_split();

    let e1 = dht_key.encode();
    trace!("e1:  {:?}", e1);
    assert_eq!(e1, "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA".to_owned());
    let e1s = dht_key_secret.encode();
    trace!("e1s: {:?}", e1s);
    let e2 = dht_key2.encode();
    trace!("e2:  {:?}", e2);
    let e2s = dht_key_secret2.encode();
    trace!("e2s: {:?}", e2s);

    let d1 = PublicKey::try_decode(e1.as_str()).unwrap();
    trace!("d1:  {:?}", d1);
    assert_eq!(dht_key, d1);

    let d1s = SecretKey::try_decode(e1s.as_str()).unwrap();
    trace!("d1s: {:?}", d1s);
    assert_eq!(dht_key_secret, d1s);

    let d2 = PublicKey::try_decode(e2.as_str()).unwrap();
    trace!("d2:  {:?}", d2);
    assert_eq!(dht_key2, d2);

    let d2s = SecretKey::try_decode(e2s.as_str()).unwrap();
    trace!("d2s: {:?}", d2s);
    assert_eq!(dht_key_secret2, d2s);

    // Failures
    let f1 = SecretKey::try_decode("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");
    assert!(f1.is_err());
    let f2 = SecretKey::try_decode("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA&");
    assert!(f2.is_err());
}

pub async fn test_typed_convert(vcrypto: CryptoSystemVersion) {
    let tks1 = format!(
        "{}:7lxDEabK_qgjbe38RtBa3IZLrud84P6NhGP-pRTZzdQ",
        vcrypto.kind()
    );
    let tk1 = TypedKey::from_str(&tks1).expect("failed");
    let tks1x = tk1.to_string();
    assert_eq!(tks1, tks1x);

    let tks2 = format!(
        "{}:7lxDEabK_qgjbe38RtBa3IZLrud84P6NhGP-pRTZzd",
        vcrypto.kind()
    );
    let _tk2 = TypedKey::from_str(&tks2).expect_err("succeeded when it shouldnt have");

    let tks3 = "XXXX:7lxDEabK_qgjbe38RtBa3IZLrud84P6NhGP-pRTZzdQ".to_string();
    let tk3 = TypedKey::from_str(&tks3).expect("failed");
    let tks3x = tk3.to_string();
    assert_eq!(tks3, tks3x);

    let tks4 = "XXXX:7lxDEabK_qgjbe38RtBa3IZLrud84P6NhGP-pRTZzd".to_string();
    let _tk4 = TypedKey::from_str(&tks4).expect_err("succeeded when it shouldnt have");

    let tks5 = "XXX:7lxDEabK_qgjbe38RtBa3IZLrud84P6NhGP-pRTZzdQ".to_string();
    let _tk5 = TypedKey::from_str(&tks5).expect_err("succeeded when it shouldnt have");

    let tks6 = "7lxDEabK_qgjbe38RtBa3IZLrud84P6NhGP-pRTZzdQ".to_string();
    let tk6 = TypedKey::from_str(&tks6).expect("failed");
    let tks6x = tk6.to_string();
    assert!(tks6x.ends_with(&tks6));
}

async fn test_hash(vcrypto: CryptoSystemVersion) {
    let mut s = BTreeSet::<PublicKey>::new();

    let k1 = vcrypto.generate_hash("abc".as_bytes());
    let k2 = vcrypto.generate_hash("abcd".as_bytes());
    let k3 = vcrypto.generate_hash("".as_bytes());
    let k4 = vcrypto.generate_hash(" ".as_bytes());
    let k5 = vcrypto.generate_hash(LOREM_IPSUM.as_bytes());
    let k6 = vcrypto.generate_hash(CHEEZBURGER.as_bytes());

    s.insert(k1);
    s.insert(k2);
    s.insert(k3);
    s.insert(k4);
    s.insert(k5);
    s.insert(k6);
    assert_eq!(s.len(), 6);

    let v1 = vcrypto.generate_hash("abc".as_bytes());
    let v2 = vcrypto.generate_hash("abcd".as_bytes());
    let v3 = vcrypto.generate_hash("".as_bytes());
    let v4 = vcrypto.generate_hash(" ".as_bytes());
    let v5 = vcrypto.generate_hash(LOREM_IPSUM.as_bytes());
    let v6 = vcrypto.generate_hash(CHEEZBURGER.as_bytes());

    assert_eq!(k1, v1);
    assert_eq!(k2, v2);
    assert_eq!(k3, v3);
    assert_eq!(k4, v4);
    assert_eq!(k5, v5);
    assert_eq!(k6, v6);

    vcrypto.validate_hash("abc".as_bytes(), &v1);
    vcrypto.validate_hash("abcd".as_bytes(), &v2);
    vcrypto.validate_hash("".as_bytes(), &v3);
    vcrypto.validate_hash(" ".as_bytes(), &v4);
    vcrypto.validate_hash(LOREM_IPSUM.as_bytes(), &v5);
    vcrypto.validate_hash(CHEEZBURGER.as_bytes(), &v6);
}

async fn test_operations(vcrypto: CryptoSystemVersion) {
    let k1 = vcrypto.generate_hash(LOREM_IPSUM.as_bytes());
    let k2 = vcrypto.generate_hash(CHEEZBURGER.as_bytes());
    let k3 = vcrypto.generate_hash("abc".as_bytes());

    // Get distance
    let d1 = vcrypto.distance(&k1, &k2);
    let d2 = vcrypto.distance(&k2, &k1);
    let d3 = vcrypto.distance(&k1, &k3);
    let d4 = vcrypto.distance(&k2, &k3);

    trace!("d1={:?}", d1);
    trace!("d2={:?}", d2);
    trace!("d3={:?}", d3);
    trace!("d4={:?}", d4);

    // Verify commutativity
    assert_eq!(d1, d2);
    assert!(d1 <= d2);
    assert!(d1 >= d2);
    assert!(d1 >= d2);
    assert!(d1 <= d2);
    assert_eq!(d2, d1);
    assert!(d2 <= d1);
    assert!(d2 >= d1);
    assert!(d2 >= d1);
    assert!(d2 <= d1);

    // Verify nibbles
    assert_eq!(d1.nibble(0), 0x9u8);
    assert_eq!(d1.nibble(1), 0x4u8);
    assert_eq!(d1.nibble(2), 0x3u8);
    assert_eq!(d1.nibble(3), 0x6u8);
    assert_eq!(d1.nibble(63), 0x6u8);

    assert_eq!(d1.first_nonzero_nibble(), Some((0, 0x9u8)));
    assert_eq!(d2.first_nonzero_nibble(), Some((0, 0x9u8)));
    assert_eq!(d3.first_nonzero_nibble(), Some((1, 0x4u8)));
    assert_eq!(d4.first_nonzero_nibble(), Some((0, 0x9u8)));

    // Verify bits
    assert_eq!(d1.bit(0), true);
    assert_eq!(d1.bit(1), false);
    assert_eq!(d1.bit(7), false);
    assert_eq!(d1.bit(8), false);
    assert_eq!(d1.bit(14), true);
    assert_eq!(d1.bit(15), false);
    assert_eq!(d1.bit(254), true);
    assert_eq!(d1.bit(255), false);

    assert_eq!(d1.first_nonzero_bit(), Some(0));
    assert_eq!(d2.first_nonzero_bit(), Some(0));
    assert_eq!(d3.first_nonzero_bit(), Some(5));
    assert_eq!(d4.first_nonzero_bit(), Some(0));
}

pub async fn test_crypto_key_ordering() {
    let k1 = CryptoKey::new([
        128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ]);
    let k2 = CryptoKey::new([
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ]);
    let k3 = CryptoKey::new([
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 128,
    ]);
    let k4 = CryptoKey::new([
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 1,
    ]);
    let k5 = CryptoKey::new([
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ]);

    assert!(k2 < k1);
    assert!(k3 < k2);
    assert!(k4 < k3);
    assert!(k5 < k4);
}

pub async fn test_all() {
    let api = crypto_tests_startup().await;
    let crypto = api.crypto().unwrap();

    test_crypto_key_ordering().await;

    // Test versions
    for v in VALID_CRYPTO_KINDS {
        let vcrypto = crypto.get(v).unwrap();

        test_generate_secret(vcrypto.clone()).await;
        test_sign_and_verify(vcrypto.clone()).await;
        test_key_conversions(vcrypto.clone()).await;
        test_encode_decode(vcrypto.clone()).await;
        test_typed_convert(vcrypto.clone()).await;
        test_hash(vcrypto.clone()).await;
        test_operations(vcrypto).await;
    }

    crypto_tests_shutdown(api.clone()).await;
    assert!(api.is_shutdown());
}
