use super::fixtures::*;
use crate::*;

// aligned_u64

pub fn test_alignedu64() {
    let orig = AlignedU64::new(0x0123456789abcdef);
    let copy = deserialize_json(&serialize_json(orig)).unwrap();

    assert_eq!(orig, copy);
}

// app_messsage_call

pub fn test_veilidappmessage() {
    let orig = VeilidAppMessage::new(
        Some(fix_typednodeid()),
        Some(fix_routeid()),
        b"Hi there!".to_vec(),
    );
    let copy = deserialize_json(&serialize_json(&orig)).unwrap();

    assert_eq!(orig, copy);
}

pub fn test_veilidappcall() {
    let orig = VeilidAppCall::new(
        Some(fix_typednodeid()),
        Some(fix_routeid()),
        b"Well, hello!".to_vec(),
        OperationId::from(123),
    );
    let copy = deserialize_json(&serialize_json(&orig)).unwrap();

    assert_eq!(orig, copy);
}

// fourcc

pub fn test_cryptokind() {
    let orig = CryptoKind::from_str("D34D").unwrap();
    let copy = deserialize_json(&serialize_json(orig)).unwrap();

    assert_eq!(orig, copy);
}

// safety

pub fn test_sequencing() {
    let orig = Sequencing::PreferOrdered;
    let copy = deserialize_json(&serialize_json(orig)).unwrap();

    assert_eq!(orig, copy);
}

pub fn test_stability() {
    let orig = Stability::Reliable;
    let copy = deserialize_json(&serialize_json(orig)).unwrap();

    assert_eq!(orig, copy);
}

pub fn test_safetyselection() {
    let orig = SafetySelection::Unsafe(Sequencing::EnsureOrdered);
    let copy = deserialize_json(&serialize_json(orig)).unwrap();

    assert_eq!(orig, copy);
}

pub fn test_safetyspec() {
    let orig = SafetySpec {
        preferred_route: Some(fix_routeid()),
        hop_count: 23,
        stability: Stability::default(),
        sequencing: Sequencing::default(),
    };
    let copy = deserialize_json(&serialize_json(orig)).unwrap();

    assert_eq!(orig, copy);
}

// stats

pub fn test_latencystats() {
    let orig = fix_latencystats();
    let copy = deserialize_json(&serialize_json(&orig)).unwrap();

    assert_eq!(orig, copy);
}

pub fn test_transferstats() {
    let orig = fix_transferstats();
    let copy = deserialize_json(&serialize_json(&orig)).unwrap();

    assert_eq!(orig, copy);
}

pub fn test_transferstatsdownup() {
    let orig = fix_transferstatsdownup();
    let copy = deserialize_json(&serialize_json(&orig)).unwrap();

    assert_eq!(orig, copy);
}

pub fn test_rpcstats() {
    let orig = fix_rpcstats();
    let copy = deserialize_json(&serialize_json(&orig)).unwrap();

    assert_eq!(orig, copy);
}

pub fn test_peerstats() {
    let orig = fix_peerstats();
    let copy = deserialize_json(&serialize_json(&orig)).unwrap();

    assert_eq!(orig, copy);
}

//  tunnel

#[cfg(feature = "unstable-tunnels")]
pub fn test_tunnelmode() {
    let orig = TunnelMode::Raw;
    let copy = deserialize_json(&serialize_json(&orig)).unwrap();

    assert_eq!(orig, copy);
}

#[cfg(feature = "unstable-tunnels")]
pub fn test_tunnelerror() {
    let orig = TunnelError::NoCapacity;
    let copy = deserialize_json(&serialize_json(&orig)).unwrap();

    assert_eq!(orig, copy);
}

#[cfg(feature = "unstable-tunnels")]
pub fn test_tunnelendpoint() {
    let orig = TunnelEndpoint {
        mode: TunnelMode::Raw,
        description: "Here there be tygers.".to_string(),
    };
    let copy = deserialize_json(&serialize_json(&orig)).unwrap();

    assert_eq!(orig, copy);
}

#[cfg(feature = "unstable-tunnels")]
pub fn test_fulltunnel() {
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

#[cfg(feature = "unstable-tunnels")]
pub fn test_partialtunnel() {
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

// veilid_log

pub fn test_veilidloglevel() {
    let orig = VeilidLogLevel::Info;
    let copy = deserialize_json(&serialize_json(orig)).unwrap();

    assert_eq!(orig, copy);
}

pub fn test_veilidlog() {
    let orig = VeilidLog {
        log_level: VeilidLogLevel::Debug,
        message: "A log! A log!".to_string(),
        backtrace: Some("Func1 -> Func2 -> Func3".to_string()),
    };
    let copy = deserialize_json(&serialize_json(&orig)).unwrap();

    assert_eq!(orig, copy);
}

// veilid_state

pub fn test_attachmentstate() {
    let orig = AttachmentState::FullyAttached;
    let copy = deserialize_json(&serialize_json(orig)).unwrap();

    assert_eq!(orig, copy);
}

pub fn test_veilidstateattachment() {
    let orig = VeilidStateAttachment {
        state: AttachmentState::OverAttached,
        public_internet_ready: true,
        local_network_ready: false,
        uptime: TimestampDuration::new_secs(10),
        attached_uptime: Some(TimestampDuration::new_secs(10)),
    };
    let copy = deserialize_json(&serialize_json(&orig)).unwrap();

    assert_eq!(orig, copy);
}

pub fn test_peertabledata() {
    let orig = fix_peertabledata();
    let copy = deserialize_json(&serialize_json(&orig)).unwrap();

    assert_eq!(orig, copy);
}

pub fn test_veilidstatenetwork() {
    let orig = VeilidStateNetwork {
        started: true,
        bps_down: ByteCount::from(14_400),
        bps_up: ByteCount::from(1200),
        peers: vec![fix_peertabledata()],
    };
    let copy = deserialize_json(&serialize_json(&orig)).unwrap();

    assert_eq!(orig, copy);
}

pub fn test_veilidroutechange() {
    let orig = VeilidRouteChange {
        dead_routes: vec![fix_routeid()],
        dead_remote_routes: vec![fix_routeid()],
    };
    let copy = deserialize_json(&serialize_json(&orig)).unwrap();

    assert_eq!(orig, copy);
}

pub fn test_veilidstateconfig() {
    let orig = VeilidStateConfig {
        config: fix_veilidconfig(),
    };
    let copy = deserialize_json(&serialize_json(&orig)).unwrap();

    assert_eq!(orig, copy);
}

pub fn test_veilidvaluechange() {
    let orig = fix_veilidvaluechange();
    let copy = deserialize_json(&serialize_json(&orig)).unwrap();

    assert_eq!(orig, copy);
}

pub fn test_veilidupdate() {
    let orig = VeilidUpdate::ValueChange(Box::new(fix_veilidvaluechange()));
    let copy = deserialize_json(&serialize_json(&orig)).unwrap();

    assert_eq!(orig, copy);
}

pub fn test_veilidstate() {
    let orig = VeilidState {
        attachment: Box::new(VeilidStateAttachment {
            state: AttachmentState::OverAttached,
            public_internet_ready: true,
            local_network_ready: false,
            uptime: TimestampDuration::new_secs(900),
            attached_uptime: Some(TimestampDuration::new_secs(600)),
        }),
        network: Box::new(VeilidStateNetwork {
            started: true,
            bps_down: ByteCount::from(14_400),
            bps_up: ByteCount::from(1200),
            peers: vec![fix_peertabledata()],
        }),
        config: Box::new(VeilidStateConfig {
            config: fix_veilidconfig(),
        }),
    };
    let copy = deserialize_json(&serialize_json(&orig)).unwrap();

    assert_eq!(orig, copy);
}
