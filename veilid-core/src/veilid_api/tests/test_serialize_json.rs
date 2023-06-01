use super::fixtures::*;
use crate::*;

// routing_table/types/peer_info.rs

pub async fn test_round_trip_peerinfo() {
    let pi: routing_table::PeerInfo = deserialize_json(SERIALIZED_PEERINFO).unwrap();

    let back = serialize_json(pi);

    assert_eq!(SERIALIZED_PEERINFO, back);
}

// veilid_api/types/aligned_u64.rs

pub async fn test_alignedu64() {
    let orig = AlignedU64::new(0x0123456789abcdef);
    let copy = deserialize_json(&serialize_json(orig)).unwrap();

    assert_eq!(orig, copy);
}

// veilid_api/types/fourcc.rs

pub async fn test_fourcc() {
    let orig = FourCC::from_str("D34D").unwrap();
    let copy = deserialize_json(&serialize_json(orig)).unwrap();

    assert_eq!(orig, copy);
}

// veilid_api/types/safety.rs

pub async fn test_safetyspec() {
    let orig = SafetySpec {
        preferred_route: Some(fix_cryptokey()),
        hop_count: 23,
        stability: Stability::default(),
        sequencing: Sequencing::default(),
    };
    let copy = deserialize_json(&serialize_json(orig)).unwrap();

    assert_eq!(orig, copy);
}

// veilid_api/types/stats.rs

pub async fn test_latencystats() {
    let orig = fix_latencystats();
    let copy = deserialize_json(&serialize_json(&orig)).unwrap();

    assert_eq!(orig, copy);
}

pub async fn test_transferstats() {
    let orig = fix_transferstats();
    let copy = deserialize_json(&serialize_json(&orig)).unwrap();

    assert_eq!(orig, copy);
}

pub async fn test_transferstatsdownup() {
    let orig = fix_transferstatsdownup();
    let copy = deserialize_json(&serialize_json(&orig)).unwrap();

    assert_eq!(orig, copy);
}

pub async fn test_rpcstats() {
    let orig = fix_rpcstats();
    let copy = deserialize_json(&serialize_json(&orig)).unwrap();

    assert_eq!(orig, copy);
}

pub async fn test_peerstats() {
    let orig = fix_peerstats();
    let copy = deserialize_json(&serialize_json(&orig)).unwrap();

    assert_eq!(orig, copy);
}

//  veilid_api/types/tunnel.rs

pub async fn test_tunnelmode() {
    let orig = TunnelMode::Raw;
    let copy = deserialize_json(&serialize_json(&orig)).unwrap();

    assert_eq!(orig, copy);
}
pub async fn test_tunnelerror() {
    let orig = TunnelError::NoCapacity;
    let copy = deserialize_json(&serialize_json(&orig)).unwrap();

    assert_eq!(orig, copy);
}

pub async fn test_tunnelendpoint() {
    let orig = TunnelEndpoint {
        mode: TunnelMode::Raw,
        description: "Here there be tygers.".to_string(),
    };
    let copy = deserialize_json(&serialize_json(&orig)).unwrap();

    assert_eq!(orig, copy);
}

pub async fn test_fulltunnel() {
    let orig = FullTunnel {
        id: AlignedU64::from(42),
        timeout: AlignedU64::from(3_000_000),
        local: TunnelEndpoint {
            mode: TunnelMode::Turn,
            description: "Left end.".to_string(),
        },
        remote: TunnelEndpoint {
            mode: TunnelMode::Turn,
            description: "Right end.".to_string(),
        },
    };
    let copy = deserialize_json(&serialize_json(&orig)).unwrap();

    assert_eq!(orig, copy);
}

pub async fn test_partialtunnel() {
    let orig = PartialTunnel {
        id: AlignedU64::from(42),
        timeout: AlignedU64::from(3_000_000),
        local: TunnelEndpoint {
            mode: TunnelMode::Turn,
            description: "I'm so lonely.".to_string(),
        },
    };
    let copy = deserialize_json(&serialize_json(&orig)).unwrap();

    assert_eq!(orig, copy);
}

// veilid_api/types/veilid_lo.rs

pub async fn test_veilidloglevel() {
    let orig = VeilidLogLevel::Info;
    let copy = deserialize_json(&serialize_json(&orig)).unwrap();

    assert_eq!(orig, copy);
}

pub async fn test_veilidlog() {
    let orig = VeilidLog {
        log_level: VeilidLogLevel::Debug,
        message: "A log! A log!".to_string(),
        backtrace: Some("Func1 -> Func2 -> Func3".to_string()),
    };
    let copy = deserialize_json(&serialize_json(&orig)).unwrap();

    assert_eq!(orig, copy);
}

// veilid_api/types/veilid_state.rs

pub async fn test_attachmentstate() {
    let orig = AttachmentState::FullyAttached;
    let copy = deserialize_json(&serialize_json(&orig)).unwrap();

    assert_eq!(orig, copy);
}

pub async fn test_veilidstateattachment() {
    let orig = VeilidStateAttachment {
        state: AttachmentState::OverAttached,
        public_internet_ready: true,
        local_network_ready: false,
    };
    let copy = deserialize_json(&serialize_json(&orig)).unwrap();

    assert_eq!(orig, copy);
}

pub async fn test_peertabledata() {
    let orig = fix_peertabledata();
    let copy = deserialize_json(&serialize_json(&orig)).unwrap();

    assert_eq!(orig, copy);
}

pub async fn test_veilidstatenetwork() {
    let orig = VeilidStateNetwork {
        started: true,
        bps_down: AlignedU64::from(14_400),
        bps_up: AlignedU64::from(1200),
        peers: vec![fix_peertabledata()],
    };
    let copy = deserialize_json(&serialize_json(&orig)).unwrap();

    assert_eq!(orig, copy);
}

pub async fn test_veilidroutechange() {
    let orig = VeilidRouteChange {
        dead_routes: vec![fix_cryptokey()],
        dead_remote_routes: vec![fix_cryptokey()],
    };
    let copy = deserialize_json(&serialize_json(&orig)).unwrap();

    assert_eq!(orig, copy);
}

pub async fn test_veilidstateconfig() {
    let orig = VeilidStateConfig {
        config: fix_veilidconfiginner(),
    };
    let copy = deserialize_json(&serialize_json(&orig)).unwrap();

    assert_eq!(orig, copy);
}

pub async fn test_veilidvaluechange() {
    let orig = fix_veilidvaluechange();
    let copy = deserialize_json(&serialize_json(&orig)).unwrap();

    assert_eq!(orig, copy);
}

pub async fn test_veilidupdate() {
    let orig = VeilidUpdate::ValueChange(fix_veilidvaluechange());
    let copy = deserialize_json(&serialize_json(&orig)).unwrap();

    assert_eq!(orig, copy);
}

pub async fn test_veilidstate() {
    let orig = VeilidState {
        attachment: VeilidStateAttachment {
            state: AttachmentState::OverAttached,
            public_internet_ready: true,
            local_network_ready: false,
        },
        network: VeilidStateNetwork {
            started: true,
            bps_down: AlignedU64::from(14_400),
            bps_up: AlignedU64::from(1200),
            peers: vec![fix_peertabledata()],
        },
        config: VeilidStateConfig {
            config: fix_veilidconfiginner(),
        },
    };
    let copy = deserialize_json(&serialize_json(&orig)).unwrap();

    assert_eq!(orig, copy);
}

pub async fn test_all() {
    test_round_trip_peerinfo().await;
    test_alignedu64().await;
    test_fourcc().await;
    test_safetyspec().await;
    test_latencystats().await;
    test_transferstats().await;
    test_transferstatsdownup().await;
    test_rpcstats().await;
    test_peerstats().await;
    test_tunnelmode().await;
    test_tunnelmode().await;
    test_tunnelerror().await;
    test_tunnelendpoint().await;
    test_fulltunnel().await;
    test_partialtunnel().await;
    test_veilidloglevel().await;
    test_veilidlog().await;
    test_attachmentstate().await;
    test_veilidstateattachment().await;
    test_peertabledata().await;
    test_veilidstatenetwork().await;
    test_veilidroutechange().await;
    test_veilidstateconfig().await;
    test_veilidvaluechange().await;
    test_veilidupdate().await;
    test_veilidstate().await;
}
