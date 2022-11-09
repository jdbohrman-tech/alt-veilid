use super::test_veilid_config::*;
use crate::xx::*;
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
    intf::sleep(5000).await;
    api.detach().await.unwrap();
    intf::sleep(2000).await;
    api.shutdown().await;

    info!("--- test auto detach ---");
    let (update_callback, config_callback) = setup_veilid_core();
    let api = api_startup(update_callback, config_callback)
        .await
        .expect("startup failed");
    api.attach().await.unwrap();
    intf::sleep(5000).await;
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

    // Test direct
    let node_info = NodeInfo {
        network_class: NetworkClass::InboundCapable,
        outbound_protocols: ProtocolTypeSet::all(),
        address_types: AddressTypeSet::all(),
        min_version: 0,
        max_version: 0,
        dial_info_detail_list: vec![DialInfoDetail {
            class: DialInfoClass::Mapped,
            dial_info: DialInfo::udp(SocketAddress::default()),
        }],
    };

    let (pkey, skey) = generate_secret();

    let sni =
        SignedDirectNodeInfo::with_secret(NodeId::new(pkey.clone()), node_info.clone(), &skey)
            .unwrap();
    let _ = SignedDirectNodeInfo::new(
        NodeId::new(pkey),
        node_info.clone(),
        sni.timestamp,
        sni.signature,
    )
    .unwrap();

    // Test relayed
    let node_info2 = NodeInfo {
        network_class: NetworkClass::OutboundOnly,
        outbound_protocols: ProtocolTypeSet::all(),
        address_types: AddressTypeSet::all(),
        min_version: 0,
        max_version: 0,
        dial_info_detail_list: vec![DialInfoDetail {
            class: DialInfoClass::Blocked,
            dial_info: DialInfo::udp(SocketAddress::default()),
        }],
    };

    let (pkey2, skey2) = generate_secret();

    let sni2 = SignedRelayedNodeInfo::with_secret(
        NodeId::new(pkey2.clone()),
        node_info2.clone(),
        NodeId::new(pkey.clone()),
        sni.clone(),
        &skey2,
    )
    .unwrap();
    let _ = SignedRelayedNodeInfo::new(
        NodeId::new(pkey2),
        node_info2,
        NodeId::new(pkey),
        sni,
        sni2.timestamp,
        sni2.signature,
    )
    .unwrap();

    api.shutdown().await;
}

pub async fn test_all() {
    test_startup_shutdown().await;
    test_attach_detach().await;
    test_signed_node_info().await;
}
