#![allow(clippy::bool_assert_comparison)]

use super::*;
use crate::xx::*;
use core::convert::TryFrom;

static LOREM_IPSUM:&str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum. ";
static CHEEZBURGER: &str = "I can has cheezburger";
static EMPTY_KEY: [u8; key::DHT_KEY_LENGTH] = [0u8; key::DHT_KEY_LENGTH];
static EMPTY_KEY_SECRET: [u8; key::DHT_KEY_SECRET_LENGTH] = [0u8; key::DHT_KEY_SECRET_LENGTH];

pub async fn test_generate_secret() {
    // Verify keys generate
    let (dht_key, dht_key_secret) = key::generate_secret();
    let (dht_key2, dht_key_secret2) = key::generate_secret();

    // Verify byte patterns are different between public and secret
    assert_ne!(dht_key.bytes, dht_key_secret.bytes);
    assert_ne!(dht_key2.bytes, dht_key_secret2.bytes);

    // Verify the keys and secrets are different across keypairs
    assert_ne!(dht_key, dht_key2);
    assert_ne!(dht_key_secret, dht_key_secret2);
}

pub async fn test_sign_and_verify() {
    // Make two keys
    let (dht_key, dht_key_secret) = key::generate_secret();
    let (dht_key2, dht_key_secret2) = key::generate_secret();
    // Sign the same message twice
    let dht_sig = key::sign(&dht_key, &dht_key_secret, LOREM_IPSUM.as_bytes()).unwrap();
    trace!("dht_sig: {:?}", dht_sig);
    let dht_sig_b = key::sign(&dht_key, &dht_key_secret, LOREM_IPSUM.as_bytes()).unwrap();
    // Sign a second message
    let dht_sig_c = key::sign(&dht_key, &dht_key_secret, CHEEZBURGER.as_bytes()).unwrap();
    trace!("dht_sig_c: {:?}", dht_sig_c);
    // Verify they are the same signature
    assert_eq!(dht_sig, dht_sig_b);
    // Sign the same message with a different key
    let dht_sig2 = key::sign(&dht_key2, &dht_key_secret2, LOREM_IPSUM.as_bytes()).unwrap();
    // Verify a different key gives a different signature
    assert_ne!(dht_sig2, dht_sig_b);

    // Try using the wrong secret to sign
    let a1 = key::sign(&dht_key, &dht_key_secret, LOREM_IPSUM.as_bytes()).unwrap();
    let a2 = key::sign(&dht_key2, &dht_key_secret2, LOREM_IPSUM.as_bytes()).unwrap();
    let b1 = key::sign(&dht_key, &dht_key_secret2, LOREM_IPSUM.as_bytes()).unwrap();
    let b2 = key::sign(&dht_key2, &dht_key_secret, LOREM_IPSUM.as_bytes()).unwrap();
    assert_ne!(a1, b1);
    assert_ne!(a2, b2);
    assert_ne!(a1, b2);
    assert_ne!(a2, b1);
    assert_ne!(a1, a2);
    assert_ne!(b1, b2);
    assert_ne!(a1, b2);
    assert_ne!(b1, a2);

    assert_eq!(key::verify(&dht_key, LOREM_IPSUM.as_bytes(), &a1), Ok(()));
    assert_eq!(key::verify(&dht_key2, LOREM_IPSUM.as_bytes(), &a2), Ok(()));
    assert!(key::verify(&dht_key, LOREM_IPSUM.as_bytes(), &b1).is_err());
    assert!(key::verify(&dht_key2, LOREM_IPSUM.as_bytes(), &b2).is_err());

    // Try verifications that should work
    assert_eq!(
        key::verify(&dht_key, LOREM_IPSUM.as_bytes(), &dht_sig),
        Ok(())
    );
    assert_eq!(
        key::verify(&dht_key, LOREM_IPSUM.as_bytes(), &dht_sig_b),
        Ok(())
    );
    assert_eq!(
        key::verify(&dht_key2, LOREM_IPSUM.as_bytes(), &dht_sig2),
        Ok(())
    );
    assert_eq!(
        key::verify(&dht_key, CHEEZBURGER.as_bytes(), &dht_sig_c),
        Ok(())
    );
    // Try verifications that shouldn't work
    assert!(key::verify(&dht_key2, LOREM_IPSUM.as_bytes(), &dht_sig).is_err());
    assert!(key::verify(&dht_key, LOREM_IPSUM.as_bytes(), &dht_sig2).is_err());
    assert!(key::verify(&dht_key2, CHEEZBURGER.as_bytes(), &dht_sig_c).is_err());
    assert!(key::verify(&dht_key, CHEEZBURGER.as_bytes(), &dht_sig).is_err());
}

pub async fn test_key_conversions() {
    // Test default key
    let (dht_key, dht_key_secret) = (key::DHTKey::default(), key::DHTKeySecret::default());
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
    let (dht_key2, dht_key_secret2) = key::generate_secret();
    trace!("dht_key2: {:?}", dht_key2);
    trace!("dht_key_secret2: {:?}", dht_key_secret2);
    let (dht_key3, _dht_key_secret3) = key::generate_secret();
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
    let dht_key_back = key::DHTKey::try_from(dht_key_string.as_str()).unwrap();
    let dht_key_back2 = key::DHTKey::try_from(dht_key_string2.as_str()).unwrap();
    assert_eq!(dht_key_back, dht_key_back2);
    assert_eq!(dht_key_back, dht_key);
    assert_eq!(dht_key_back2, dht_key);

    let dht_key_secret_back = key::DHTKeySecret::try_from(dht_key_secret_string.as_str()).unwrap();
    assert_eq!(dht_key_secret_back, dht_key_secret);

    let dht_key2_back = key::DHTKey::try_from(dht_key2_string.as_str()).unwrap();
    let dht_key2_back2 = key::DHTKey::try_from(dht_key2_string2.as_str()).unwrap();
    assert_eq!(dht_key2_back, dht_key2_back2);
    assert_eq!(dht_key2_back, dht_key2);
    assert_eq!(dht_key2_back2, dht_key2);

    let dht_key_secret2_back =
        key::DHTKeySecret::try_from(dht_key_secret2_string.as_str()).unwrap();
    assert_eq!(dht_key_secret2_back, dht_key_secret2);

    // Assert string roundtrip
    assert_eq!(String::from(&dht_key2_back), dht_key2_string);
    assert!(key::DHTKey::try_from("") == Ok(key::DHTKey::default()));
    assert!(key::DHTKeySecret::try_from("") == Ok(key::DHTKeySecret::default()));
    // These conversions should fail
    assert!(key::DHTKey::try_from("whatever").is_err());
    assert!(key::DHTKeySecret::try_from("whatever").is_err());
    assert!(key::DHTKey::try_from(" ").is_err());
    assert!(key::DHTKeySecret::try_from(" ").is_err());
    assert!(key::DHTKey::try_from(
        "qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq"
    )
    .is_err());
    assert!(key::DHTKeySecret::try_from(
        "qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq"
    )
    .is_err());
}

pub async fn test_encode_decode() {
    let dht_key = key::DHTKey::try_decode("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA").unwrap();
    let dht_key_secret =
        key::DHTKeySecret::try_decode("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA").unwrap();
    let dht_key_b = key::DHTKey::new(EMPTY_KEY);
    let dht_key_secret_b = key::DHTKeySecret::new(EMPTY_KEY_SECRET);
    assert_eq!(dht_key, dht_key_b);
    assert_eq!(dht_key_secret, dht_key_secret_b);

    let (dht_key2, dht_key_secret2) = key::generate_secret();

    let e1 = dht_key.encode();
    trace!("e1:  {:?}", e1);
    assert_eq!(e1, "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA".to_owned());
    let e1s = dht_key_secret.encode();
    trace!("e1s: {:?}", e1s);
    let e2 = dht_key2.encode();
    trace!("e2:  {:?}", e2);
    let e2s = dht_key_secret2.encode();
    trace!("e2s: {:?}", e2s);

    let d1 = key::DHTKey::try_decode(e1.as_str()).unwrap();
    trace!("d1:  {:?}", d1);
    assert_eq!(dht_key, d1);

    let d1s = key::DHTKeySecret::try_decode(e1s.as_str()).unwrap();
    trace!("d1s: {:?}", d1s);
    assert_eq!(dht_key_secret, d1s);

    let d2 = key::DHTKey::try_decode(e2.as_str()).unwrap();
    trace!("d2:  {:?}", d2);
    assert_eq!(dht_key2, d2);

    let d2s = key::DHTKeySecret::try_decode(e2s.as_str()).unwrap();
    trace!("d2s: {:?}", d2s);
    assert_eq!(dht_key_secret2, d2s);

    // Failures
    let f1 = key::DHTKeySecret::try_decode("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");
    assert!(f1.is_err());
    let f2 = key::DHTKeySecret::try_decode("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA&");
    assert!(f2.is_err());
}

async fn test_hash() {
    let mut s = BTreeSet::<key::DHTKey>::new();

    let k1 = key::generate_hash("abc".as_bytes());
    let k2 = key::generate_hash("abcd".as_bytes());
    let k3 = key::generate_hash("".as_bytes());
    let k4 = key::generate_hash(" ".as_bytes());
    let k5 = key::generate_hash(LOREM_IPSUM.as_bytes());
    let k6 = key::generate_hash(CHEEZBURGER.as_bytes());

    s.insert(k1);
    s.insert(k2);
    s.insert(k3);
    s.insert(k4);
    s.insert(k5);
    s.insert(k6);
    assert_eq!(s.len(), 6);

    let v1 = key::generate_hash("abc".as_bytes());
    let v2 = key::generate_hash("abcd".as_bytes());
    let v3 = key::generate_hash("".as_bytes());
    let v4 = key::generate_hash(" ".as_bytes());
    let v5 = key::generate_hash(LOREM_IPSUM.as_bytes());
    let v6 = key::generate_hash(CHEEZBURGER.as_bytes());

    assert_eq!(k1, v1);
    assert_eq!(k2, v2);
    assert_eq!(k3, v3);
    assert_eq!(k4, v4);
    assert_eq!(k5, v5);
    assert_eq!(k6, v6);

    key::validate_hash("abc".as_bytes(), &v1);
    key::validate_hash("abcd".as_bytes(), &v2);
    key::validate_hash("".as_bytes(), &v3);
    key::validate_hash(" ".as_bytes(), &v4);
    key::validate_hash(LOREM_IPSUM.as_bytes(), &v5);
    key::validate_hash(CHEEZBURGER.as_bytes(), &v6);
}

async fn test_operations() {
    let k1 = key::generate_hash(LOREM_IPSUM.as_bytes());
    let k2 = key::generate_hash(CHEEZBURGER.as_bytes());
    let k3 = key::generate_hash("abc".as_bytes());

    // Get distance
    let d1 = key::distance(&k1, &k2);
    let d2 = key::distance(&k2, &k1);
    let d3 = key::distance(&k1, &k3);
    let d4 = key::distance(&k2, &k3);

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

pub async fn test_all() {
    test_generate_secret().await;
    test_sign_and_verify().await;
    test_key_conversions().await;
    test_encode_decode().await;
    test_hash().await;
    test_operations().await;
}
