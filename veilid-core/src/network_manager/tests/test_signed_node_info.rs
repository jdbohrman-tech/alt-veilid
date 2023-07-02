use super::*;
use crate::tests::common::test_veilid_config::*;

pub async fn test_signed_node_info() {
    info!("--- test_signed_node_info ---");

    let (update_callback, config_callback) = setup_veilid_core();
    let api = api_startup(update_callback, config_callback)
        .await
        .expect("startup failed");

    let crypto = api.crypto().unwrap();
    for ck in VALID_CRYPTO_KINDS {
        let vcrypto = crypto.get(ck).unwrap();

        // Test direct
        let node_info = NodeInfo::new(
            NetworkClass::InboundCapable,
            ProtocolTypeSet::all(),
            AddressTypeSet::all(),
            VALID_ENVELOPE_VERSIONS.to_vec(),
            VALID_CRYPTO_KINDS.to_vec(),
            vec![DialInfoDetail {
                class: DialInfoClass::Mapped,
                dial_info: DialInfo::udp(SocketAddress::default()),
            }],
        );

        // Test correct validation
        let keypair = vcrypto.generate_keypair();
        let sni = SignedDirectNodeInfo::make_signatures(
            crypto.clone(),
            vec![TypedKeyPair::new(ck, keypair)],
            node_info.clone(),
        )
        .unwrap();
        let tks: TypedKeyGroup = TypedKey::new(ck, keypair.key).into();
        let oldtkslen = tks.len();
        let sdni = SignedDirectNodeInfo::new(
            node_info.clone(),
            sni.timestamp(),
            sni.signatures().to_vec(),
        );
        let tks_validated = sdni.validate(&tks, crypto.clone()).unwrap();
        assert_eq!(tks_validated.len(), oldtkslen);
        assert_eq!(tks_validated.len(), sni.signatures().len());

        // Test incorrect validation
        let keypair1 = vcrypto.generate_keypair();
        let tks1: TypedKeyGroup = TypedKey::new(ck, keypair1.key).into();
        let sdni = SignedDirectNodeInfo::new(
            node_info.clone(),
            sni.timestamp(),
            sni.signatures().to_vec(),
        );
        sdni.validate(&tks1, crypto.clone()).unwrap_err();

        // Test unsupported cryptosystem validation
        let fake_crypto_kind: CryptoKind = FourCC::from([0, 1, 2, 3]);
        let mut tksfake: TypedKeyGroup =
            TypedKey::new(fake_crypto_kind, PublicKey::default()).into();
        let mut sigsfake = sni.signatures().to_vec();
        sigsfake.push(TypedSignature::new(fake_crypto_kind, Signature::default()));
        tksfake.add(TypedKey::new(ck, keypair.key));
        let sdnifake =
            SignedDirectNodeInfo::new(node_info.clone(), sni.timestamp(), sigsfake.clone());
        let tksfake_validated = sdnifake.validate(&tksfake, crypto.clone()).unwrap();
        assert_eq!(tksfake_validated.len(), 1);
        assert_eq!(sdnifake.signatures().len(), sigsfake.len());

        // Test relayed
        let node_info2 = NodeInfo::new(
            NetworkClass::OutboundOnly,
            ProtocolTypeSet::all(),
            AddressTypeSet::all(),
            VALID_ENVELOPE_VERSIONS.to_vec(),
            VALID_CRYPTO_KINDS.to_vec(),
            vec![DialInfoDetail {
                class: DialInfoClass::Blocked,
                dial_info: DialInfo::udp(SocketAddress::default()),
            }],
        );

        // Test correct validation
        let keypair2 = vcrypto.generate_keypair();
        let tks2: TypedKeyGroup = TypedKey::new(ck, keypair2.key).into();
        let oldtks2len = tks2.len();

        let sni2 = SignedRelayedNodeInfo::make_signatures(
            crypto.clone(),
            vec![TypedKeyPair::new(ck, keypair2)],
            node_info2.clone(),
            tks.clone(),
            sni.clone(),
        )
        .unwrap();
        let srni = SignedRelayedNodeInfo::new(
            node_info2.clone(),
            tks.clone(),
            sni.clone(),
            sni2.timestamp(),
            sni2.signatures().to_vec(),
        );
        let tks2_validated = srni.validate(&tks2, crypto.clone()).unwrap();

        assert_eq!(tks2_validated.len(), oldtks2len);
        assert_eq!(tks2_validated.len(), sni2.signatures().len());

        // Test incorrect validation
        let keypair3 = vcrypto.generate_keypair();
        let tks3: TypedKeyGroup = TypedKey::new(ck, keypair3.key).into();

        let srni = SignedRelayedNodeInfo::new(
            node_info2.clone(),
            tks.clone(),
            sni.clone(),
            sni2.timestamp(),
            sni2.signatures().to_vec(),
        );
        srni.validate(&tks3, crypto.clone()).unwrap_err();

        // Test unsupported cryptosystem validation
        let fake_crypto_kind: CryptoKind = FourCC::from([0, 1, 2, 3]);
        let mut tksfake3: TypedKeyGroup =
            TypedKey::new(fake_crypto_kind, PublicKey::default()).into();
        let mut sigsfake3 = sni2.signatures().to_vec();
        sigsfake3.push(TypedSignature::new(fake_crypto_kind, Signature::default()));
        tksfake3.add(TypedKey::new(ck, keypair2.key));
        let srnifake = SignedRelayedNodeInfo::new(
            node_info2.clone(),
            tks.clone(),
            sni.clone(),
            sni2.timestamp(),
            sigsfake3.clone(),
        );
        let tksfake3_validated = srnifake.validate(&tksfake3, crypto.clone()).unwrap();
        assert_eq!(tksfake3_validated.len(), 1);
        assert_eq!(srnifake.signatures().len(), sigsfake3.len());
    }

    api.shutdown().await;
}

pub async fn test_all() {
    test_signed_node_info().await;
}
