use crate::*;

// Fixtures

const SERIALIZED_PEERINFO: &str = r###"{"node_ids":["FAKE:eFOfgm_FNZBsTRi7KAESNwYFAUGgX2uDrTRWAL8ucjM"],"signed_node_info":{"Direct":{"node_info":{"network_class":"InboundCapable","outbound_protocols":1,"address_types":3,"envelope_support":[0],"crypto_support":[[86,76,68,48]],"dial_info_detail_list":[{"class":"Direct","dial_info":{"kind":"UDP","socket_address":{"address":{"IPV4":"1.2.3.4"},"port":5150}}},{"class":"Direct","dial_info":{"kind":"UDP","socket_address":{"address":{"IPV6":"bad:cafe::1"},"port":5150}}},{"class":"Direct","dial_info":{"kind":"TCP","socket_address":{"address":{"IPV4":"5.6.7.8"},"port":5150}}},{"class":"Direct","dial_info":{"kind":"TCP","socket_address":{"address":{"IPV6":"bad:cafe::1"},"port":5150}}},{"class":"Direct","dial_info":{"kind":"WS","socket_address":{"address":{"IPV4":"9.10.11.12"},"port":5150},"request":"bootstrap-1.dev.veilid.net:5150/ws"}},{"class":"Direct","dial_info":{"kind":"WS","socket_address":{"address":{"IPV6":"bad:cafe::1"},"port":5150},"request":"bootstrap-1.dev.veilid.net:5150/ws"}}]},"timestamp":1685058646770389,"signatures":[]}}}"###;

pub async fn test_round_trip_peerinfo() {
    let pi: routing_table::PeerInfo = deserialize_json(SERIALIZED_PEERINFO).unwrap();

    let back = serialize_json(pi);

    assert_eq!(SERIALIZED_PEERINFO, back);
}

pub async fn test_alignedu64() {
    let orig = AlignedU64::new(0x0123456789abcdef);
    let copy = deserialize_json(&serialize_json(orig)).unwrap();

    assert_eq!(orig, copy);
}

pub async fn test_fourcc() {
    let orig = FourCC::from_str("D34D").unwrap();
    let copy = deserialize_json(&serialize_json(orig)).unwrap();

    assert_eq!(orig, copy);
}

pub async fn test_safetyspec() {
    let orig = SafetySpec {
        preferred_route: Some(fix_typedkey().value),
        hop_count: 23,
        stability: Stability::default(),
        sequencing: Sequencing::default(),
    };
    let copy = deserialize_json(&serialize_json(orig)).unwrap();

    assert_eq!(orig, copy);
}

fn fix_latencystats() -> LatencyStats {
    LatencyStats {
        fastest: AlignedU64::from(1234),
        average: AlignedU64::from(2345),
        slowest: AlignedU64::from(3456),
    }
}

pub async fn test_latencystats() {
    let orig = fix_latencystats();
    let copy = deserialize_json(&serialize_json(&orig)).unwrap();

    assert_eq!(orig, copy);
}

fn fix_transferstats() -> TransferStats {
    TransferStats {
        total: AlignedU64::from(1_000_000),
        maximum: AlignedU64::from(3456),
        average: AlignedU64::from(2345),
        minimum: AlignedU64::from(1234),
    }
}

pub async fn test_transferstats() {
    let orig = fix_transferstats();
    let copy = deserialize_json(&serialize_json(&orig)).unwrap();

    assert_eq!(orig, copy);
}

fn fix_transferstatsdownup() -> TransferStatsDownUp {
    TransferStatsDownUp {
        down: fix_transferstats(),
        up: fix_transferstats(),
    }
}

pub async fn test_transferstatsdownup() {
    let orig = fix_transferstatsdownup();
    let copy = deserialize_json(&serialize_json(&orig)).unwrap();

    assert_eq!(orig, copy);
}

fn fix_rpcstats() -> RPCStats {
    RPCStats {
        messages_sent: 1_000_000,
        messages_rcvd: 2_000_000,
        questions_in_flight: 42,
        last_question_ts: Some(AlignedU64::from(1685569084280)),
        last_seen_ts: Some(AlignedU64::from(1685569101256)),
        first_consecutive_seen_ts: Some(AlignedU64::from(1685569111851)),
        recent_lost_answers: 5,
        failed_to_send: 3,
    }
}

pub async fn test_rpcstats() {
    let orig = fix_rpcstats();
    let copy = deserialize_json(&serialize_json(&orig)).unwrap();

    assert_eq!(orig, copy);
}

fn fix_peerstats() -> PeerStats {
    PeerStats {
        time_added: AlignedU64::from(1685569176894),
        rpc_stats: fix_rpcstats(),
        latency: Some(fix_latencystats()),
        transfer: fix_transferstatsdownup(),
    }
}

pub async fn test_peerstats() {
    let orig = fix_peerstats();
    let copy = deserialize_json(&serialize_json(&orig)).unwrap();

    assert_eq!(orig, copy);
}

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

fn fix_typedkey() -> TypedKey {
    let mut fake_key = [0u8; CRYPTO_KEY_LENGTH];
    random_bytes(&mut fake_key);
    let b = TypedKey {
        kind: FourCC::from_str("FAKE").unwrap(),
        value: CryptoKey::new(fake_key),
    };
    b
    //panic!("{}", b);
}

fn fix_peertabledata() -> PeerTableData {
    PeerTableData {
        node_ids: vec![fix_typedkey()],
        peer_address: "123 Main St.".to_string(),
        peer_stats: fix_peerstats(),
    }
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
}
