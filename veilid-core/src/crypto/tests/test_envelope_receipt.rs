use super::*;
use crate::tests::common::test_veilid_config::*;

pub async fn test_envelope_round_trip() {
    info!("--- test envelope round trip ---");
    let (update_callback, config_callback) = setup_veilid_core();
    let api = api_startup(update_callback, config_callback)
        .await
        .expect("startup failed");

    // Get crypto
    let crypto = api.crypto().unwrap();

    // Create envelope
    let ts = 0x12345678ABCDEF69u64;
    let nonce = Crypto::get_random_nonce();
    let (sender_id, sender_secret) = generate_secret();
    let (recipient_id, recipient_secret) = generate_secret();
    let envelope = Envelope::new(0, ts, nonce, sender_id, recipient_id);

    // Create arbitrary body
    let body = b"This is an arbitrary body";

    // Serialize to bytes
    let enc_data = envelope
        .to_encrypted_data(crypto.clone(), body, &sender_secret)
        .expect("failed to encrypt data");

    // Deserialize from bytes
    let envelope2 =
        Envelope::from_signed_data(&enc_data).expect("failed to deserialize envelope from data");

    let body2 = envelope2
        .decrypt_body(crypto.clone(), &enc_data, &recipient_secret)
        .expect("failed to decrypt envelope body");

    // Compare envelope and body
    assert_eq!(envelope, envelope2);
    assert_eq!(body.to_vec(), body2);

    // Deserialize from modified bytes
    let enc_data_len = enc_data.len();
    let mut mod_enc_data = enc_data.clone();
    mod_enc_data[enc_data_len - 1] ^= 0x80u8;
    assert!(
        Envelope::from_signed_data(&mod_enc_data).is_err(),
        "should have failed to decode envelope with modified signature"
    );
    let mut mod_enc_data2 = enc_data.clone();
    mod_enc_data2[enc_data_len - 65] ^= 0x80u8;
    assert!(
        Envelope::from_signed_data(&mod_enc_data2).is_err(),
        "should have failed to decode envelope with modified data"
    );

    api.shutdown().await;
}

pub async fn test_receipt_round_trip() {
    info!("--- test receipt round trip ---");
    // Create arbitrary body
    let body = b"This is an arbitrary body";

    // Create receipt
    let nonce = Crypto::get_random_nonce();
    let (sender_id, sender_secret) = generate_secret();
    let receipt = Receipt::try_new(0, nonce, sender_id, body).expect("should not fail");

    // Serialize to bytes
    let mut enc_data = receipt
        .to_signed_data(&sender_secret)
        .expect("failed to make signed data");

    // Deserialize from bytes
    let receipt2 =
        Receipt::from_signed_data(&enc_data).expect("failed to deserialize envelope from data");

    // Should not validate even when a single bit is changed
    enc_data[5] = 0x01;
    Receipt::from_signed_data(&enc_data)
        .expect_err("should have failed to decrypt using wrong secret");

    // Compare receipts
    assert_eq!(receipt, receipt2);
}

pub async fn test_all() {
    test_envelope_round_trip().await;
    test_receipt_round_trip().await;
}
