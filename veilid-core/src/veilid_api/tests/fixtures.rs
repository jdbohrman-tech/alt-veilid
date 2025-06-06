use crate::*;

// Fixtures used by various tests

pub fn fix_latencystats() -> LatencyStats {
    LatencyStats {
        fastest: TimestampDuration::from(1234),
        average: TimestampDuration::from(2345),
        slowest: TimestampDuration::from(3456),
        tm90: TimestampDuration::from(4567),
        tm75: TimestampDuration::from(5678),
        p90: TimestampDuration::from(6789),
        p75: TimestampDuration::from(7890),
    }
}

pub fn fix_transferstats() -> TransferStats {
    TransferStats {
        total: ByteCount::from(1_000_000),
        maximum: ByteCount::from(3456),
        average: ByteCount::from(2345),
        minimum: ByteCount::from(1234),
    }
}

pub fn fix_transferstatsdownup() -> TransferStatsDownUp {
    TransferStatsDownUp {
        down: fix_transferstats(),
        up: fix_transferstats(),
    }
}

pub fn fix_answerstats() -> AnswerStats {
    AnswerStats {
        span: TimestampDuration::new_secs(10),
        questions: 10,
        answers: 8,
        lost_answers: 0,
        consecutive_answers_maximum: 1,
        consecutive_answers_average: 2,
        consecutive_answers_minimum: 3,
        consecutive_lost_answers_maximum: 4,
        consecutive_lost_answers_average: 5,
        consecutive_lost_answers_minimum: 6,
    }
}

pub fn fix_rpcstats() -> RPCStats {
    RPCStats {
        messages_sent: 1_000_000,
        messages_rcvd: 2_000_000,
        questions_in_flight: 42,
        last_question_ts: Some(Timestamp::from(1685569084280)),
        last_seen_ts: Some(Timestamp::from(1685569101256)),
        first_consecutive_seen_ts: Some(Timestamp::from(1685569111851)),
        recent_lost_answers_unordered: 5,
        recent_lost_answers_ordered: 6,
        failed_to_send: 3,
        answer_unordered: fix_answerstats(),
        answer_ordered: fix_answerstats(),
    }
}

pub fn fix_statestats() -> StateStats {
    StateStats {
        span: TimestampDuration::new_secs(10),
        reliable: TimestampDuration::new_secs(5),
        unreliable: TimestampDuration::new_secs(5),
        dead: TimestampDuration::new_secs(0),
        punished: TimestampDuration::new_secs(0),
        reason: StateReasonStats {
            can_not_send: TimestampDuration::new_secs(1),
            too_many_lost_answers: TimestampDuration::new_secs(2),
            no_ping_response: TimestampDuration::new_secs(3),
            failed_to_send: TimestampDuration::new_secs(4),
            lost_answers: TimestampDuration::new_secs(5),
            not_seen_consecutively: TimestampDuration::new_secs(6),
            in_unreliable_ping_span: TimestampDuration::new_secs(7),
        },
    }
}

pub fn fix_peerstats() -> PeerStats {
    PeerStats {
        time_added: Timestamp::from(1685569176894),
        rpc_stats: fix_rpcstats(),
        latency: Some(fix_latencystats()),
        transfer: fix_transferstatsdownup(),
        state: fix_statestats(),
    }
}

pub fn fix_publickey() -> PublicKey {
    let mut fake_key = [0u8; CRYPTO_KEY_LENGTH];
    random_bytes(&mut fake_key);
    PublicKey::new(fake_key)
}

pub fn fix_recordkey() -> RecordKey {
    let mut fake_key = [0u8; CRYPTO_KEY_LENGTH];
    random_bytes(&mut fake_key);
    RecordKey::new(fake_key)
}

pub fn fix_routeid() -> RouteId {
    let mut fake_key = [0u8; CRYPTO_KEY_LENGTH];
    random_bytes(&mut fake_key);
    RouteId::new(fake_key)
}

pub fn fix_nodeid() -> NodeId {
    let mut fake_key = [0u8; CRYPTO_KEY_LENGTH];
    random_bytes(&mut fake_key);
    NodeId::new(fake_key)
}

pub fn fix_typednodeid() -> TypedNodeId {
    let mut fake_key = [0u8; CRYPTO_KEY_LENGTH];
    random_bytes(&mut fake_key);
    TypedNodeId {
        kind: CryptoKind::from_str("FAKE").unwrap(),
        value: fix_nodeid(),
    }
}

pub fn fix_typedrecordkey() -> TypedRecordKey {
    let mut fake_key = [0u8; CRYPTO_KEY_LENGTH];
    random_bytes(&mut fake_key);
    TypedRecordKey {
        kind: CryptoKind::from_str("FAKE").unwrap(),
        value: fix_recordkey(),
    }
}

pub fn fix_secretkey() -> SecretKey {
    let mut fake_key = [0u8; CRYPTO_KEY_LENGTH];
    random_bytes(&mut fake_key);
    SecretKey::new(fake_key)
}

pub fn fix_peertabledata() -> PeerTableData {
    PeerTableData {
        node_ids: vec![fix_typednodeid()],
        peer_address: "123 Main St.".to_string(),
        peer_stats: fix_peerstats(),
    }
}

pub fn fix_veilidconfig() -> VeilidConfig {
    VeilidConfig {
        program_name: "Bob".to_string(),
        namespace: "Internets".to_string(),
        capabilities: VeilidConfigCapabilities {
            disable: Vec::new(),
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
            client_allowlist_timeout_ms: 7000,
            reverse_connection_receipt_time_ms: 8000,
            hole_punch_receipt_time_ms: 9000,
            network_key_password: None,
            routing_table: VeilidConfigRoutingTable {
                node_id: TypedNodeIdGroup::new(),
                node_id_secret: TypedSecretKeyGroup::new(),
                bootstrap: vec!["boots".to_string()],
                bootstrap_keys: vec![TypedPublicKey::from_str(
                    "VLD0:qrxwD1-aM9xiUw4IAPVXE_4qgoIfyR4Y6MEPyaDl_GQ",
                )
                .unwrap()],
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
                public_watch_limit: 20,
                member_watch_limit: 21,
                max_watch_expiration_ms: 22,
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
            #[cfg(feature = "geolocation")]
            privacy: VeilidConfigPrivacy {
                country_code_denylist: vec![CountryCode::from_str("NZ").unwrap()],
            },
            #[cfg(feature = "virtual-network")]
            virtual_network: VeilidConfigVirtualNetwork {
                enabled: false,
                server_address: "".to_owned(),
            },
        },
    }
}

pub fn fix_veilidvaluechange() -> VeilidValueChange {
    VeilidValueChange {
        key: fix_typedrecordkey(),
        subkeys: ValueSubkeyRangeSet::new(),
        count: 5,
        value: Some(ValueData::new_with_seq(23, b"ValueData".to_vec(), fix_publickey()).unwrap()),
    }
}
