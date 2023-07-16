use super::*;

pub async fn test_routingtable_buckets_round_trip() {
    let original = mock_routing_table();
    let copy = mock_routing_table();
    original.init().await.unwrap();
    copy.init().await.unwrap();

    // Add lots of routes to `original` here to exercise all various types.

    let (serialized_bucket_map, all_entry_bytes) = original.serialized_buckets();

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
    let mut tks = TypedKeyGroup::new();
    tks.add(TypedKey::new(
        CRYPTO_KIND_VLD0,
        CryptoKey::new([
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ]),
    ));
    let pi: PeerInfo = PeerInfo::new(
        tks,
        SignedNodeInfo::Direct(SignedDirectNodeInfo::new(
            NodeInfo::new(
                NetworkClass::OutboundOnly,
                ProtocolTypeSet::new(),
                AddressTypeSet::new(),
                vec![0],
                vec![CRYPTO_KIND_VLD0],
                PUBLIC_INTERNET_CAPABILITIES.to_vec(),
                vec![],
            ),
            Timestamp::new(0),
            Vec::new(),
        )),
    );
    let s = serialize_json(&pi);
    let pi2 = deserialize_json(&s).expect("Should deserialize");
    let s2 = serialize_json(&pi2);

    assert_eq!(pi, pi2);
    assert_eq!(s, s2);
}

pub async fn test_all() {
    test_routingtable_buckets_round_trip().await;
    test_round_trip_peerinfo().await;
}
