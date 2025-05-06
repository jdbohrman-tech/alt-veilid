use super::*;
use crate::tests::mock_registry;

fn make_mock_bootstrap_record(include_timestamp: bool) -> BootstrapRecord {
    let mut node_ids = CryptoTypedGroup::new();
    node_ids.add(
        TypedKey::from_str("VLD0:f8G4Ckr1UR8YXnmAllwfvBEvXGgfYicZllb7jEpJeSU")
            .expect("should parse key"),
    );
    let envelope_support = VALID_ENVELOPE_VERSIONS.to_vec();
    let dial_info_details = vec![
        DialInfoDetail {
            class: DialInfoClass::Direct,
            dial_info: DialInfo::try_ws(
                SocketAddress::new(Address::IPV4(Ipv4Addr::UNSPECIFIED), 5150),
                "ws://example.com:5150/ws".to_owned(),
            )
            .expect("should make ws dialinfo"),
        },
        DialInfoDetail {
            class: DialInfoClass::Direct,
            dial_info: DialInfo::try_wss(
                SocketAddress::new(Address::IPV4(Ipv4Addr::UNSPECIFIED), 5150),
                "wss://example.com:5150/wss".to_owned(),
            )
            .expect("should make wss dialinfo"),
        },
    ];
    let opt_timestamp = if include_timestamp {
        Some(Timestamp::now().as_u64() / 1_000_000u64)
    } else {
        None
    };
    BootstrapRecord::new(
        node_ids,
        envelope_support,
        dial_info_details,
        opt_timestamp,
        vec![],
    )
}

pub async fn test_bootstrap_v0() {
    let registry = mock_registry::init("").await;
    let network_manager = registry.network_manager();
    let dial_info_converter = MockDialInfoConverter::default();

    let bsrec = make_mock_bootstrap_record(false);
    let v0str = bsrec
        .to_v0_string(&dial_info_converter)
        .await
        .expect("should make string");
    let bsrec2 = BootstrapRecord::new_from_v0_str(&network_manager, &dial_info_converter, &v0str)
        .expect("should parse string")
        .expect("should be valid record");
    assert_eq!(bsrec, bsrec2);

    mock_registry::terminate(registry).await;
}

pub async fn test_bootstrap_v1() {
    let registry = mock_registry::init("").await;
    let network_manager = registry.network_manager();
    let dial_info_converter = MockDialInfoConverter::default();

    let bsrec = make_mock_bootstrap_record(true);
    let signing_key_pairs = [TypedKeyPair::from_str("VLD0:W7ENB-SUWpPA7usY8ORVQf_si5QmFbD1Uqa89Jg2Uc0:hbdjau5sr3rBNwN68XeWLg3rfXnXLaLqfbbqhELqV1E").expect("should parse keypair"),
        TypedKeyPair::from_str("VLD0:v6XPfyOoCP_ZP-CWFNrf_pF_dpxsq74p2LW_Q5Q4yPQ:n-DhHtOU7KWQkdp5to8cpBa_u0RFt2IDZzXPqMTq4O0").expect("should parse keypair")];
    let signing_keys = signing_key_pairs
        .iter()
        .map(|skp| TypedKey::new(skp.kind, skp.value.key))
        .collect::<Vec<_>>();
    let v1str = bsrec
        .to_v1_string(&network_manager, &dial_info_converter, signing_key_pairs[0])
        .await
        .expect("should make string");
    let bsrec2 = BootstrapRecord::new_from_v1_str(
        &network_manager,
        &dial_info_converter,
        &v1str,
        &signing_keys,
    )
    .expect("should parse string")
    .expect("should be valid record");
    assert_eq!(bsrec, bsrec2);

    mock_registry::terminate(registry).await;
}

pub async fn test_all() {
    test_bootstrap_v0().await;
    test_bootstrap_v1().await;
}
