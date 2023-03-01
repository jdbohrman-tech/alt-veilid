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

        let (pkey, skey) = vcrypto.generate_keypair();

        let sni = SignedDirectNodeInfo::make_signatures(
            crypto.clone(),
            vec![TypedKeyPair::new(ck, KeyPair::new(pkey, skey))],
            node_info.clone(),
        )
        .unwrap();
        let mut tks: TypedKeySet = TypedKey::new(ck, pkey).into();
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

        let (pkey2, skey2) = vcrypto.generate_keypair();
        let mut tks2: TypedKeySet = TypedKey::new(ck, pkey2).into();
        let oldtks2len = tks2.len();

        let sni2 = SignedRelayedNodeInfo::make_signatures(
            crypto.clone(),
            vec![TypedKeyPair::new(ck, KeyPair::new(pkey2, skey2))],
            node_info2.clone(),
            tks.clone(),
            sni.clone(),
        )
        .unwrap();
        let _ = SignedRelayedNodeInfo::new(
            crypto.clone(),
            &mut tks2,
            node_info2,
            tks,
            sni,
            sni2.timestamp,
            sni2.signatures.clone(),
        )
        .unwrap();

        assert_eq!(tks2.len(), oldtks2len);
        assert_eq!(tks2.len(), sni2.signatures.len());
    }

    api.shutdown().await;
}

pub async fn test_all() {
    test_startup_shutdown().await;
    test_attach_detach().await;
    test_signed_node_info().await;
}
