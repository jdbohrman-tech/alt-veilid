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
        preferred_route: Some(fix_cryptokey()),
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

fn fix_cryptokey() -> CryptoKey {
    let mut fake_key = [0u8; CRYPTO_KEY_LENGTH];
    random_bytes(&mut fake_key);
    CryptoKey::new(fake_key)
}

fn fix_typedkey() -> TypedKey {
    let mut fake_key = [0u8; CRYPTO_KEY_LENGTH];
    random_bytes(&mut fake_key);
    TypedKey {
        kind: FourCC::from_str("FAKE").unwrap(),
        value: fix_cryptokey(),
    }
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

pub async fn test_veilidroutechange() {
    let orig = VeilidRouteChange {
        dead_routes: vec![fix_cryptokey()],
        dead_remote_routes: vec![fix_cryptokey()],
    };
    let copy = deserialize_json(&serialize_json(&orig)).unwrap();

    assert_eq!(orig, copy);
}

fn fix_veilidconfiginner() -> VeilidConfigInner {
    VeilidConfigInner {
        program_name: "Bob".to_string(),
        namespace: "Internets".to_string(),
        capabilities: VeilidConfigCapabilities {
            protocol_udp: false,
            protocol_connect_tcp: true,
            protocol_accept_tcp: false,
            protocol_connect_ws: true,
            protocol_accept_ws: false,
            protocol_connect_wss: true,
            protocol_accept_wss: false,
        },
        protected_store: VeilidConfigProtectedStore {
            allow_insecure_fallback: true,
            always_use_insecure_storage: false,
            directory: "/root".to_string(),
            delete: true,
            device_encryption_key_password: "1234".to_string(),
            new_device_encryption_key_password: Some("5678".to_string()),
        },
        table_store: VeilidConfigTableStore {
            directory: "Yellow Pages".to_string(),
            delete: false,
        },
        block_store: VeilidConfigBlockStore {
            directory: "C:\\Program Files".to_string(),
            delete: true,
        },
        network: VeilidConfigNetwork {
            connection_initial_timeout_ms: 1000,
            connection_inactivity_timeout_ms: 2000,
            max_connections_per_ip4: 3000,
            max_connections_per_ip6_prefix: 4000,
            max_connections_per_ip6_prefix_size: 5000,
            max_connection_frequency_per_min: 6000,
            client_whitelist_timeout_ms: 7000,
            reverse_connection_receipt_time_ms: 8000,
            hole_punch_receipt_time_ms: 9000,
            routing_table: VeilidConfigRoutingTable {
                node_id: TypedKeySet::new(),
                node_id_secret: TypedSecretSet::new(),
                bootstrap: vec!["boots".to_string()],
                limit_over_attached: 1,
                limit_fully_attached: 2,
                limit_attached_strong: 3,
                limit_attached_good: 4,
                limit_attached_weak: 5,
            },
            rpc: VeilidConfigRPC {
                concurrency: 5,
                queue_size: 6,
                max_timestamp_behind_ms: Some(1000),
                max_timestamp_ahead_ms: Some(2000),
                timeout_ms: 3000,
                max_route_hop_count: 7,
                default_route_hop_count: 8,
            },
            dht: VeilidConfigDHT {
                max_find_node_count: 1,
                resolve_node_timeout_ms: 2,
                resolve_node_count: 3,
                resolve_node_fanout: 4,
                get_value_timeout_ms: 5,
                get_value_count: 6,
                get_value_fanout: 7,
                set_value_timeout_ms: 8,
                set_value_count: 9,
                set_value_fanout: 10,
                min_peer_count: 11,
                min_peer_refresh_time_ms: 12,
                validate_dial_info_receipt_time_ms: 13,
                local_subkey_cache_size: 14,
                local_max_subkey_cache_memory_mb: 15,
                remote_subkey_cache_size: 16,
                remote_max_records: 17,
                remote_max_subkey_cache_memory_mb: 18,
                remote_max_storage_space_mb: 19,
            },
            upnp: true,
            detect_address_changes: false,
            restricted_nat_retries: 10000,
            tls: VeilidConfigTLS {
                certificate_path: "/etc/ssl/certs/cert.pem".to_string(),
                private_key_path: "/etc/ssl/keys/key.pem".to_string(),
                connection_initial_timeout_ms: 1000,
            },
            application: VeilidConfigApplication {
                https: VeilidConfigHTTPS {
                    enabled: true,
                    listen_address: "10.0.0.3".to_string(),
                    path: "/https_path/".to_string(),
                    url: Some("https://veilid.com/".to_string()),
                },
                http: VeilidConfigHTTP {
                    enabled: true,
                    listen_address: "10.0.0.4".to_string(),
                    path: "/http_path/".to_string(),
                    url: Some("http://veilid.com/".to_string()),
                },
            },
            protocol: VeilidConfigProtocol {
                udp: VeilidConfigUDP {
                    enabled: false,
                    socket_pool_size: 30,
                    listen_address: "10.0.0.2".to_string(),
                    public_address: Some("2.3.4.5".to_string()),
                },
                tcp: VeilidConfigTCP {
                    connect: true,
                    listen: false,
                    max_connections: 8,
                    listen_address: "10.0.0.1".to_string(),
                    public_address: Some("1.2.3.4".to_string()),
                },
                ws: VeilidConfigWS {
                    connect: false,
                    listen: true,
                    max_connections: 9,
                    listen_address: "127.0.0.1".to_string(),
                    path: "Straight".to_string(),
                    url: Some("https://veilid.com/ws".to_string()),
                },
                wss: VeilidConfigWSS {
                    connect: true,
                    listen: false,
                    max_connections: 10,
                    listen_address: "::1".to_string(),
                    path: "Curved".to_string(),
                    url: Some("https://veilid.com/wss".to_string()),
                },
            },
        },
    }
}

pub async fn test_veilidstateconfig() {
    let orig = VeilidStateConfig {
        config: fix_veilidconfiginner(),
    };
    let copy = deserialize_json(&serialize_json(&orig)).unwrap();

    assert_eq!(orig, copy);
}

fn fix_veilidvaluechange() -> VeilidValueChange {
    VeilidValueChange {
        key: fix_typedkey(),
        subkeys: vec![1, 2, 3, 4],
        count: 5,
        value: ValueData {
            seq: 23,
            data: b"ValueData".to_vec(),
            writer: fix_cryptokey(),
        },
    }
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
