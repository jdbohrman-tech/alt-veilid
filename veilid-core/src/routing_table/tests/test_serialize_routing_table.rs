use crate::*;

const SERIALIZED_PEERINFO: &str = r###"{"node_ids":["FAKE:eFOfgm_FNZBsTRi7KAESNwYFAUGgX2uDrTRWAL8ucjM"],"signed_node_info":{"Direct":{"node_info":{"network_class":"InboundCapable","outbound_protocols":1,"address_types":3,"envelope_support":[0],"crypto_support":[[86,76,68,48]],"dial_info_detail_list":[{"class":"Direct","dial_info":{"kind":"UDP","socket_address":{"address":{"IPV4":"1.2.3.4"},"port":5150}}},{"class":"Direct","dial_info":{"kind":"UDP","socket_address":{"address":{"IPV6":"bad:cafe::1"},"port":5150}}},{"class":"Direct","dial_info":{"kind":"TCP","socket_address":{"address":{"IPV4":"5.6.7.8"},"port":5150}}},{"class":"Direct","dial_info":{"kind":"TCP","socket_address":{"address":{"IPV6":"bad:cafe::1"},"port":5150}}},{"class":"Direct","dial_info":{"kind":"WS","socket_address":{"address":{"IPV4":"9.10.11.12"},"port":5150},"request":"bootstrap-1.dev.veilid.net:5150/ws"}},{"class":"Direct","dial_info":{"kind":"WS","socket_address":{"address":{"IPV6":"bad:cafe::1"},"port":5150},"request":"bootstrap-1.dev.veilid.net:5150/ws"}}]},"timestamp":1685058646770389,"signatures":[]}}}"###;

fn fake_routing_table() -> routing_table::RoutingTable {
    let veilid_config = VeilidConfig::new();
    let block_store = BlockStore::new(veilid_config.clone());
    let protected_store = ProtectedStore::new(veilid_config.clone());
    let table_store = TableStore::new(veilid_config.clone(), protected_store.clone());
    let crypto = Crypto::new(veilid_config.clone(), table_store.clone());
    let storage_manager = storage_manager::StorageManager::new(
        veilid_config.clone(),
        crypto.clone(),
        protected_store.clone(),
        table_store.clone(),
        block_store.clone(),
    );
    let network_manager = network_manager::NetworkManager::new(
        veilid_config.clone(),
        storage_manager,
        protected_store.clone(),
        table_store.clone(),
        block_store.clone(),
        crypto.clone(),
    );
    routing_table::RoutingTable::new(network_manager)
}

pub async fn test_routingtable_buckets_round_trip() {
    let original = fake_routing_table();
    let copy = fake_routing_table();
    original.init().await.unwrap();
    copy.init().await.unwrap();

    // Add lots of routes to `original` here to exercise all various types.

    let (serialized_bucket_map, all_entry_bytes) = original.serialized_buckets().unwrap();

    copy.populate_routing_table(
        &mut copy.inner.write(),
        serialized_bucket_map,
        all_entry_bytes,
    )
    .unwrap();

    // Wrap to close lifetime of 'inner' which is borrowed here so terminate() can succeed
    // (it also .write() locks routing table inner)
    {
        let original_inner = &*original.inner.read();
        let copy_inner = &*copy.inner.read();

        let routing_table_keys: Vec<_> = original_inner.buckets.keys().clone().collect();
        let copy_keys: Vec<_> = copy_inner.buckets.keys().clone().collect();

        assert_eq!(routing_table_keys.len(), copy_keys.len());

        for crypto in routing_table_keys {
            // The same keys are present in the original and copy RoutingTables.
            let original_buckets = original_inner.buckets.get(&crypto).unwrap();
            let copy_buckets = copy_inner.buckets.get(&crypto).unwrap();

            // Recurse into RoutingTable.inner.buckets
            for (left_buckets, right_buckets) in original_buckets.iter().zip(copy_buckets.iter()) {
                // Recurse into RoutingTable.inner.buckets.entries
                for ((left_crypto, left_entries), (right_crypto, right_entries)) in
                    left_buckets.entries().zip(right_buckets.entries())
                {
                    assert_eq!(left_crypto, right_crypto);

                    assert_eq!(
                        format!("{:?}", left_entries),
                        format!("{:?}", right_entries)
                    );
                }
            }
        }
    }

    // Even if these are mocks, we should still practice good hygiene.
    original.terminate().await;
    copy.terminate().await;
}

pub async fn test_round_trip_peerinfo() {
    let pi: routing_table::PeerInfo = deserialize_json(SERIALIZED_PEERINFO).unwrap();

    let back = serialize_json(pi);

    assert_eq!(SERIALIZED_PEERINFO, back);
}

pub async fn test_all() {
    test_routingtable_buckets_round_trip().await;
    test_round_trip_peerinfo().await;
}
