use super::*;

pub async fn test_envelope_round_trip(
    envelope_version: EnvelopeVersion,
    vcrypto: &AsyncCryptoSystemGuard<'_>,
    network_key: Option<SharedSecret>,
) {
    let crypto = vcrypto.crypto();
    if network_key.is_some() {
        info!(
            "--- test envelope round trip {} w/network key ---",
            vcrypto.kind()
        );
    } else {
        info!("--- test envelope round trip {} ---", vcrypto.kind());
    }

    // Create envelope
    let ts = Timestamp::from(0x12345678ABCDEF69u64);
    let nonce = vcrypto.random_nonce().await;
    let (sender_id, sender_secret) = vcrypto.generate_keypair().await.into_split();
    let (recipient_id, recipient_secret) = vcrypto.generate_keypair().await.into_split();
    let envelope = Envelope::new(
        envelope_version,
        vcrypto.kind(),
        ts,
        nonce,
        sender_id.into(),
        recipient_id.into(),
    );

    // Create arbitrary body
    let body = b"This is an arbitrary body";

    // Serialize to bytes
    let enc_data = envelope
        .to_encrypted_data(&crypto, body, &sender_secret, &network_key)
        .expect("failed to encrypt data");

    // Deserialize from bytes
    let envelope2 = Envelope::from_signed_data(&crypto, &enc_data, &network_key)
        .expect("failed to deserialize envelope from data");

    let body2 = envelope2
        .decrypt_body(&crypto, &enc_data, &recipient_secret, &network_key)
        .expect("failed to decrypt envelope body");

    // Compare envelope and body
    assert_eq!(envelope, envelope2);
    assert_eq!(body.to_vec(), body2);

    // Deserialize from modified bytes
    let enc_data_len = enc_data.len();
    let mut mod_enc_data = enc_data.clone();
    mod_enc_data[enc_data_len - 1] ^= 0x80u8;
    assert!(
        Envelope::from_signed_data(&crypto, &mod_enc_data, &network_key).is_err(),
        "should have failed to decode envelope with modified signature"
    );
    let mut mod_enc_data2 = enc_data.clone();
    mod_enc_data2[enc_data_len - 65] ^= 0x80u8;
    assert!(
        Envelope::from_signed_data(&crypto, &mod_enc_data2, &network_key).is_err(),
        "should have failed to decode envelope with modified data"
    );
}

pub async fn test_receipt_round_trip(
    envelope_version: EnvelopeVersion,
    vcrypto: &AsyncCryptoSystemGuard<'_>,
) {
    let crypto = vcrypto.crypto();
    info!("--- test receipt round trip ---");
    // Create arbitrary body
    let body = b"This is an arbitrary body";

    // Create receipt
    let nonce = vcrypto.random_nonce().await;
    let (sender_id, sender_secret) = vcrypto.generate_keypair().await.into_split();
    let receipt = Receipt::try_new(
        envelope_version,
        vcrypto.kind(),
        nonce,
        sender_id.into(),
        body,
    )
    .expect("should not fail");

    // Serialize to bytes
    let mut enc_data = receipt
        .to_signed_data(&crypto, &sender_secret)
        .expect("failed to make signed data");

    // Deserialize from bytes
    let receipt2 = Receipt::from_signed_data(&crypto, &enc_data)
        .expect("failed to deserialize envelope from data");

    // Should not validate even when a single bit is changed
    enc_data[5] = 0x01;
    let _ = Receipt::from_signed_data(&crypto, &enc_data)
        .expect_err("should have failed to decrypt using wrong secret");

    // Compare receipts
    assert_eq!(receipt, receipt2);
}

pub async fn test_all() {
    let api = crypto_tests_startup().await;
    let crypto = api.crypto().unwrap();

    // Test versions
    for ev in VALID_ENVELOPE_VERSIONS {
        for v in VALID_CRYPTO_KINDS {
            let vcrypto = crypto.get_async(v).unwrap();

            test_envelope_round_trip(ev, &vcrypto, None).await;
            test_envelope_round_trip(ev, &vcrypto, Some(vcrypto.random_shared_secret().await))
                .await;
            test_receipt_round_trip(ev, &vcrypto).await;
        }
    }

    crypto_tests_shutdown(api.clone()).await;
    assert!(api.is_shutdown());
}
