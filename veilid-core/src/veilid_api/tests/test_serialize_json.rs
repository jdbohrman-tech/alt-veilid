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

pub async fn test_all() {
    test_round_trip_peerinfo().await;
    test_alignedu64().await;
    test_fourcc().await;
    test_safetyspec().await;
}
