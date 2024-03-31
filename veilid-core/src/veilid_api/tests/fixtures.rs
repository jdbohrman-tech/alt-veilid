use crate::*;

// Fixtures used by various tests

pub fn fix_latencystats() -> LatencyStats {
    LatencyStats {
        fastest: AlignedU64::from(1234),
        average: AlignedU64::from(2345),
        slowest: AlignedU64::from(3456),
    }
}

pub fn fix_transferstats() -> TransferStats {
    TransferStats {
        total: AlignedU64::from(1_000_000),
        maximum: AlignedU64::from(3456),
        average: AlignedU64::from(2345),
        minimum: AlignedU64::from(1234),
    }
}

pub fn fix_transferstatsdownup() -> TransferStatsDownUp {
    TransferStatsDownUp {
        down: fix_transferstats(),
        up: fix_transferstats(),
    }
}

pub fn fix_rpcstats() -> RPCStats {
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

pub fn fix_peerstats() -> PeerStats {
    PeerStats {
        time_added: AlignedU64::from(1685569176894),
        rpc_stats: fix_rpcstats(),
        latency: Some(fix_latencystats()),
        transfer: fix_transferstatsdownup(),
    }
}

pub fn fix_cryptokey() -> CryptoKey {
    let mut fake_key = [0u8; CRYPTO_KEY_LENGTH];
    random_bytes(&mut fake_key);
    CryptoKey::new(fake_key)
}

pub fn fix_typedkey() -> TypedKey {
    let mut fake_key = [0u8; CRYPTO_KEY_LENGTH];
    random_bytes(&mut fake_key);
    TypedKey {
        kind: FourCC::from_str("FAKE").unwrap(),
        value: fix_cryptokey(),
    }
}

pub fn fix_peertabledata() -> PeerTableData {
    PeerTableData {
        node_ids: vec![fix_typedkey()],
        peer_address: "123 Main St.".to_string(),
        peer_stats: fix_peerstats(),
    }
}

pub fn fix_veilidconfiginner() -> VeilidConfigInner {
    VeilidConfigInner {
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
                node_id: TypedKeyGroup::new(),
                node_id_secret: TypedSecretGroup::new(),
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
        },
    }
}

pub fn fix_veilidvaluechange() -> VeilidValueChange {
    VeilidValueChange {
        key: fix_typedkey(),
        subkeys: ValueSubkeyRangeSet::new(),
        count: 5,
        value: Some(ValueData::new_with_seq(23, b"ValueData".to_vec(), fix_cryptokey()).unwrap()),
    }
}
