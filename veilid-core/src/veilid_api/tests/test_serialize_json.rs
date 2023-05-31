use crate::*;

const SERIALIZED_PEERINFO: &str = r###"{"node_ids":["VLD0:grOBXsrkgw4aBbmz6cFSUFkDan2_OFOwk6j-SayrQtA"],"signed_node_info":{"Direct":{"node_info":{"network_class":"InboundCapable","outbound_protocols":1,"address_types":3,"envelope_support":[0],"crypto_support":[[86,76,68,48]],"dial_info_detail_list":[{"class":"Direct","dial_info":{"kind":"UDP","socket_address":{"address":{"IPV4":"1.2.3.4"},"port":5150}}},{"class":"Direct","dial_info":{"kind":"UDP","socket_address":{"address":{"IPV6":"bad:cafe::1"},"port":5150}}},{"class":"Direct","dial_info":{"kind":"TCP","socket_address":{"address":{"IPV4":"5.6.7.8"},"port":5150}}},{"class":"Direct","dial_info":{"kind":"TCP","socket_address":{"address":{"IPV6":"bad:cafe::1"},"port":5150}}},{"class":"Direct","dial_info":{"kind":"WS","socket_address":{"address":{"IPV4":"9.10.11.12"},"port":5150},"request":"bootstrap-1.dev.veilid.net:5150/ws"}},{"class":"Direct","dial_info":{"kind":"WS","socket_address":{"address":{"IPV6":"bad:cafe::1"},"port":5150},"request":"bootstrap-1.dev.veilid.net:5150/ws"}}]},"timestamp":1685058646770389,"signatures":[]}}}"###;

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
        preferred_route: Some(CryptoKey::new(*b"thisISaKEYthat's32charsLONGitIS!")),
        hop_count: 23,
        stability: Stability::default(),
        sequencing: Sequencing::default(),
    };
    let copy = deserialize_json(&serialize_json(orig)).unwrap();

    assert_eq!(orig, copy);
}

pub async fn test_latencystats() {
    let orig = LatencyStats {
        fastest: AlignedU64::from(1234),
        average: AlignedU64::from(2345),
        slowest: AlignedU64::from(3456),
    };
    let copy = deserialize_json(&serialize_json(&orig)).unwrap();

    assert_eq!(orig, copy);
}

pub async fn test_transferstats() {
    let orig = TransferStats {
        total: AlignedU64::from(1_000_000),
        maximum: AlignedU64::from(3456),
        average: AlignedU64::from(2345),
        minimum: AlignedU64::from(1234),
    };
    let copy = deserialize_json(&serialize_json(&orig)).unwrap();

    assert_eq!(orig, copy);
}

pub async fn test_transferstatsdownup() {
    let orig = TransferStatsDownUp {
        down: TransferStats {
            total: AlignedU64::from(1_000_000),
            maximum: AlignedU64::from(3456),
            average: AlignedU64::from(2345),
            minimum: AlignedU64::from(1234),
        },
        up: TransferStats {
            total: AlignedU64::from(1_000_000 * 2),
            maximum: AlignedU64::from(3456 * 2),
            average: AlignedU64::from(2345 * 2),
            minimum: AlignedU64::from(1234 * 2),
        },
    };
    let copy = deserialize_json(&serialize_json(&orig)).unwrap();

    assert_eq!(orig, copy);
}

pub async fn test_rpcstats() {
    let orig = RPCStats {
        messages_sent: 1_000_000,
        messages_rcvd: 2_000_000,
        questions_in_flight: 42,
        last_question_ts: Some(AlignedU64::from(1685569084280)),
        last_seen_ts: Some(AlignedU64::from(1685569101256)),
        first_consecutive_seen_ts: Some(AlignedU64::from(1685569111851)),
        recent_lost_answers: 5,
        failed_to_send: 3,
    };
    let copy = deserialize_json(&serialize_json(&orig)).unwrap();

    assert_eq!(orig, copy);
}

pub async fn test_peerstats() {
    let orig = PeerStats {
        time_added: AlignedU64::from(1685569176894),
        rpc_stats: RPCStats {
            messages_sent: 1_000_000,
            messages_rcvd: 2_000_000,
            questions_in_flight: 42,
            last_question_ts: Some(AlignedU64::from(1685569084280)),
            last_seen_ts: Some(AlignedU64::from(1685569101256)),
            first_consecutive_seen_ts: Some(AlignedU64::from(1685569111851)),
            recent_lost_answers: 5,
            failed_to_send: 3,
        },
        latency: Some(LatencyStats {
            fastest: AlignedU64::from(1234),
            average: AlignedU64::from(2345),
            slowest: AlignedU64::from(3456),
        }),
        transfer: TransferStatsDownUp {
            down: TransferStats {
                total: AlignedU64::from(1_000_000),
                maximum: AlignedU64::from(3456),
                average: AlignedU64::from(2345),
                minimum: AlignedU64::from(1234),
            },
            up: TransferStats {
                total: AlignedU64::from(1_000_000 * 2),
                maximum: AlignedU64::from(3456 * 2),
                average: AlignedU64::from(2345 * 2),
                minimum: AlignedU64::from(1234 * 2),
            },
        },
    };
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
}
