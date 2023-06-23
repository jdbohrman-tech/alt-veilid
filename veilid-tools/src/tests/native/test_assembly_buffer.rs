use crate::*;

fn random_sockaddr() -> SocketAddr {
    if get_random_u32() & 1 == 0 {
        let mut addr = [0u8; 16];
        random_bytes(&mut addr);
        let port = get_random_u32() as u16;
        SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::from(addr), port, 0, 0))
    } else {
        let mut addr = [0u8; 4];
        random_bytes(&mut addr);
        let port = get_random_u32() as u16;
        SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::from(addr), port))
    }
}

pub async fn test_single_out_in() {
    let assbuf_out = AssemblyBuffer::new();
    let assbuf_in = AssemblyBuffer::new();
    let (net_tx, net_rx) = flume::unbounded();
    let sender = |framed_chunk: Vec<u8>, remote_addr: SocketAddr| {
        let net_tx = net_tx.clone();
        async move {
            net_tx
                .send_async((framed_chunk, remote_addr))
                .await
                .expect("should send");
            Ok(NetworkResult::value(()))
        }
    };

    for _ in 0..1000 {
        let message = vec![1u8; 1000];
        let remote_addr = random_sockaddr();

        // Send single message below fragmentation limit
        assert!(matches!(
            assbuf_out
                .split_message(message.clone(), remote_addr, sender)
                .await,
            Ok(NetworkResult::Value(()))
        ));

        // Ensure we didn't fragment
        let (frame, r_remote_addr) = net_rx.recv_async().await.expect("should recv");

        // Send to input
        let r_message = assbuf_in
            .insert_frame(&frame, r_remote_addr)
            .expect("should get one out");

        // We should have gotten the same message
        assert_eq!(r_message, message);
        assert_eq!(r_remote_addr, remote_addr);
    }

    // Shoud have consumed everything
    assert!(net_rx.is_empty())
}

pub async fn test_all() {
    test_single_out_in().await;
}
