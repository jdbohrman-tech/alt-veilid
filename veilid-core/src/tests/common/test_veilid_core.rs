use super::test_veilid_config::*;
use crate::*;

pub async fn test_startup_shutdown() {
    trace!("test_startup_shutdown: starting");
    let (update_callback, config_callback) = setup_veilid_core();
    let api = api_startup(update_callback, config_callback)
        .await
        .expect("startup failed");
    trace!("test_startup_shutdown: shutting down");
    api.shutdown().await;
    trace!("test_startup_shutdown: finished");
}

pub async fn test_attach_detach() {
    info!("--- test normal order ---");
    let (update_callback, config_callback) = setup_veilid_core();
    let api = api_startup(update_callback, config_callback)
        .await
        .expect("startup failed");
    api.attach().await.unwrap();
    sleep(5000).await;
    api.detach().await.unwrap();
    sleep(2000).await;
    api.shutdown().await;

    info!("--- test auto detach ---");
    let (update_callback, config_callback) = setup_veilid_core();
    let api = api_startup(update_callback, config_callback)
        .await
        .expect("startup failed");
    api.attach().await.unwrap();
    sleep(5000).await;
    api.shutdown().await;

    info!("--- test detach without attach ---");
    let (update_callback, config_callback) = setup_veilid_core();
    let api = api_startup(update_callback, config_callback)
        .await
        .expect("startup failed");
    assert!(api.detach().await.is_err());
    api.shutdown().await;
}

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
        let node_info = NodeInfo {
            network_class: NetworkClass::InboundCapable,
            outbound_protocols: ProtocolTypeSet::all(),
            address_types: AddressTypeSet::all(),
            envelope_support: VALID_ENVELOPE_VERSIONS.to_vec(),
            crypto_support: VALID_CRYPTO_KINDS.to_vec(),
            dial_info_detail_list: vec![DialInfoDetail {
                class: DialInfoClass::Mapped,
                dial_info: DialInfo::udp(SocketAddress::default()),
            }],
        };

        // Test correct validation
        let keypair = vcrypto.generate_keypair();
        let sni = SignedDirectNodeInfo::make_signatures(
            crypto.clone(),
            vec![TypedKeyPair::new(ck, keypair)],
            node_info.clone(),
        )
        .unwrap();
        let mut tks: TypedKeySet = TypedKey::new(ck, keypair.key).into();
        let oldtkslen = tks.len();
        let _ = SignedDirectNodeInfo::new(
            crypto.clone(),
            &mut tks,
            node_info.clone(),
            sni.timestamp,
            sni.signatures.clone(),
        )
        .unwrap();
        assert_eq!(tks.len(), oldtkslen);
        assert_eq!(tks.len(), sni.signatures.len());

        // Test incorrect validation
        let keypair1 = vcrypto.generate_keypair();
        let mut tks1: TypedKeySet = TypedKey::new(ck, keypair1.key).into();
        let oldtks1len = tks1.len();
        let _ = SignedDirectNodeInfo::new(
            crypto.clone(),
            &mut tks1,
            node_info.clone(),
            sni.timestamp,
            sni.signatures.clone(),
        )
        .unwrap_err();
        assert_eq!(tks1.len(), oldtks1len);
        assert_eq!(tks1.len(), sni.signatures.len());

        // Test unsupported cryptosystem validation
        let fake_crypto_kind: CryptoKind = FourCC::from([0, 1, 2, 3]);
        let mut tksfake: TypedKeySet = TypedKey::new(fake_crypto_kind, PublicKey::default()).into();
        let mut sigsfake = sni.signatures.clone();
        sigsfake.push(TypedSignature::new(fake_crypto_kind, Signature::default()));
        tksfake.add(TypedKey::new(ck, keypair.key));
        let sdnifake = SignedDirectNodeInfo::new(
            crypto.clone(),
            &mut tksfake,
            node_info.clone(),
            sni.timestamp,
            sigsfake.clone(),
        )
        .unwrap();
        assert_eq!(tksfake.len(), 1);
        assert_eq!(sdnifake.signatures.len(), sigsfake.len());

        // Test relayed
        let node_info2 = NodeInfo {
            network_class: NetworkClass::OutboundOnly,
            outbound_protocols: ProtocolTypeSet::all(),
            address_types: AddressTypeSet::all(),
            envelope_support: VALID_ENVELOPE_VERSIONS.to_vec(),
            crypto_support: VALID_CRYPTO_KINDS.to_vec(),
            dial_info_detail_list: vec![DialInfoDetail {
                class: DialInfoClass::Blocked,
                dial_info: DialInfo::udp(SocketAddress::default()),
            }],
        };

        // Test correct validation
        let keypair2 = vcrypto.generate_keypair();
        let mut tks2: TypedKeySet = TypedKey::new(ck, keypair2.key).into();
        let oldtks2len = tks2.len();

        let sni2 = SignedRelayedNodeInfo::make_signatures(
            crypto.clone(),
            vec![TypedKeyPair::new(ck, keypair2)],
            node_info2.clone(),
            tks.clone(),
            sni.clone(),
        )
        .unwrap();
        let _ = SignedRelayedNodeInfo::new(
            crypto.clone(),
            &mut tks2,
            node_info2.clone(),
            tks.clone(),
            sni.clone(),
            sni2.timestamp,
            sni2.signatures.clone(),
        )
        .unwrap();

        assert_eq!(tks2.len(), oldtks2len);
        assert_eq!(tks2.len(), sni2.signatures.len());

        // Test incorrect validation
        let keypair3 = vcrypto.generate_keypair();
        let mut tks3: TypedKeySet = TypedKey::new(ck, keypair3.key).into();
        let oldtks3len = tks3.len();

        let _ = SignedRelayedNodeInfo::new(
            crypto.clone(),
            &mut tks3,
            node_info2.clone(),
            tks.clone(),
            sni.clone(),
            sni2.timestamp,
            sni2.signatures.clone(),
        )
        .unwrap_err();

        assert_eq!(tks3.len(), oldtks3len);
        assert_eq!(tks3.len(), sni2.signatures.len());

        // Test unsupported cryptosystem validation
        let fake_crypto_kind: CryptoKind = FourCC::from([0, 1, 2, 3]);
        let mut tksfake3: TypedKeySet =
            TypedKey::new(fake_crypto_kind, PublicKey::default()).into();
        let mut sigsfake3 = sni2.signatures.clone();
        sigsfake3.push(TypedSignature::new(fake_crypto_kind, Signature::default()));
        tksfake3.add(TypedKey::new(ck, keypair2.key));
        let srnifake = SignedRelayedNodeInfo::new(
            crypto.clone(),
            &mut tksfake3,
            node_info2.clone(),
            tks.clone(),
            sni.clone(),
            sni2.timestamp,
            sigsfake3.clone(),
        )
        .unwrap();
        assert_eq!(tksfake3.len(), 1);
        assert_eq!(srnifake.signatures.len(), sigsfake3.len());
    }

    api.shutdown().await;
}

pub async fn test_all() {
    test_startup_shutdown().await;
    test_attach_detach().await;
    test_signed_node_info().await;
}
