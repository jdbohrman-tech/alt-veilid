use rand::seq::SliceRandom;

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
    info!("-- test_single_out_in");
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
        let random_len = (get_random_u32() % 1000) as usize;
        let mut message = vec![1u8; random_len];
        random_bytes(&mut message);
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
            .into_result()
            .expect("should get a value")
            .expect("should get something out");

        // We should have gotten the same message
        assert_eq!(r_message, message);
        assert_eq!(r_remote_addr, remote_addr);
    }

    // Shoud have consumed everything
    assert!(net_rx.is_empty())
}

pub async fn test_one_frag_out_in() {
    info!("-- test_one_frag_out_in");
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

    let mut all_sent = HashSet::new();

    // Sending
    println!("sending");
    for _ in 0..10000 {
        let random_len = (get_random_u32() % 1000) as usize + FRAGMENT_LEN;
        let mut message = vec![1u8; random_len];
        random_bytes(&mut message);
        let remote_addr = random_sockaddr();

        // Send single message above fragmentation limit
        all_sent.insert((message.clone(), remote_addr));
        assert!(matches!(
            assbuf_out
                .split_message(message.clone(), remote_addr, sender)
                .await,
            Ok(NetworkResult::Value(()))
        ));
    }

    println!("all_sent len={}", all_sent.len());

    println!("fragments sent = {}", net_rx.len());

    drop(net_tx);

    // Receiving
    println!("receiving");

    while let Ok((frame, r_remote_addr)) = net_rx.recv_async().await {
        // Send to input
        let r_message = assbuf_in
            .insert_frame(&frame, r_remote_addr)
            .into_result()
            .expect("should get a value");

        // We should have gotten the same message
        if let Some(r_message) = r_message {
            assert!(all_sent.remove(&(r_message, r_remote_addr)));
        }
    }
    println!("all_sent len={}", all_sent.len());

    // Shoud have dropped no packets
    assert_eq!(all_sent.len(), 0);
}

pub async fn test_many_frags_out_in() {
    info!("-- test_many_frags_out_in");
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

    let mut all_sent = HashSet::new();

    // Sending
    let mut total_sent_size = 0usize;
    println!("sending");
    for _ in 0..1000 {
        let random_len = (get_random_u32() % 65536) as usize;
        total_sent_size += random_len;
        let mut message = vec![1u8; random_len];
        random_bytes(&mut message);
        let remote_addr = random_sockaddr();

        // Send single message
        all_sent.insert((message.clone(), remote_addr));
        assert!(matches!(
            assbuf_out
                .split_message(message.clone(), remote_addr, sender)
                .await,
            Ok(NetworkResult::Value(()))
        ));
    }

    println!("all_sent len={}", all_sent.len());
    println!("total_sent_size = {}", total_sent_size);
    println!("fragments sent = {}", net_rx.len());

    drop(net_tx);

    // Receiving
    println!("receiving");

    while let Ok((frame, r_remote_addr)) = net_rx.recv_async().await {
        // Send to input
        let r_message = assbuf_in
            .insert_frame(&frame, r_remote_addr)
            .into_result()
            .expect("should get a value");

        // We should have gotten the same message
        if let Some(r_message) = r_message {
            assert!(all_sent.remove(&(r_message, r_remote_addr)));
        }
    }
    println!("all_sent len={}", all_sent.len());

    // Shoud have dropped no packets
    assert_eq!(all_sent.len(), 0);
}

pub async fn test_many_frags_out_in_single_host() {
    info!("-- test_many_frags_out_in_single_host");
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

    let mut all_sent = HashSet::new();

    // Sending
    let mut total_sent_size = 0usize;
    println!("sending");
    for _ in 0..1000 {
        let random_len = (get_random_u32() % 65536) as usize;
        total_sent_size += random_len;
        let mut message = vec![1u8; random_len];
        random_bytes(&mut message);
        let remote_addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(1, 2, 3, 4), 5678));

        // Send single message
        all_sent.insert((message.clone(), remote_addr));
        assert!(matches!(
            assbuf_out
                .split_message(message.clone(), remote_addr, sender)
                .await,
            Ok(NetworkResult::Value(()))
        ));
    }

    println!("all_sent len={}", all_sent.len());
    println!("total_sent_size = {}", total_sent_size);
    println!("fragments sent = {}", net_rx.len());

    drop(net_tx);

    // Receiving
    println!("receiving");

    while let Ok((frame, r_remote_addr)) = net_rx.recv_async().await {
        // Send to input
        let r_message = assbuf_in
            .insert_frame(&frame, r_remote_addr)
            .into_result()
            .expect("should get a value");

        // We should have gotten the same message
        if let Some(r_message) = r_message {
            assert!(all_sent.remove(&(r_message, r_remote_addr)));
        }
    }
    println!("all_sent len={}", all_sent.len());

    // Shoud have dropped no packets
    assert_eq!(all_sent.len(), 0);
}

pub async fn test_many_frags_with_drops() {
    info!("-- test_many_frags_with_drops");
    let assbuf_out = AssemblyBuffer::new();
    let assbuf_in = AssemblyBuffer::new();
    let (net_tx, net_rx) = flume::unbounded();

    let first = Arc::new(AtomicBool::new(true));

    let sender = |framed_chunk: Vec<u8>, remote_addr: SocketAddr| {
        let net_tx = net_tx.clone();
        let first = first.clone();
        async move {
            // Send only first packet, drop rest
            if first.swap(false, Ordering::AcqRel) {
                net_tx
                    .send_async((framed_chunk, remote_addr))
                    .await
                    .expect("should send");
            }
            Ok(NetworkResult::value(()))
        }
    };

    let mut all_sent = HashSet::new();

    // Sending
    let mut total_sent_size = 0usize;
    let mut total_fragged = 0usize;
    println!("sending");
    for _ in 0..1000 {
        let random_len = (get_random_u32() % 65536) as usize;
        if random_len > FRAGMENT_LEN {
            total_fragged += 1;
        }
        total_sent_size += random_len;
        let mut message = vec![1u8; random_len];
        random_bytes(&mut message);
        let remote_addr = random_sockaddr();

        // Send single message
        all_sent.insert((message.clone(), remote_addr));
        assert!(matches!(
            assbuf_out
                .split_message(message.clone(), remote_addr, sender)
                .await,
            Ok(NetworkResult::Value(()))
        ));

        first.store(true, Ordering::Release);
    }

    println!("all_sent len={}", all_sent.len());
    println!("total_sent_size = {}", total_sent_size);
    println!("fragments sent = {}", net_rx.len());
    println!("total_fragged = {}", total_fragged);
    drop(net_tx);

    // Receiving
    println!("receiving");

    while let Ok((frame, r_remote_addr)) = net_rx.recv_async().await {
        // Send to input
        let r_message = assbuf_in
            .insert_frame(&frame, r_remote_addr)
            .into_result()
            .expect("should get a value");

        // We should have gotten the same message
        if let Some(r_message) = r_message {
            assert!(all_sent.remove(&(r_message, r_remote_addr)));
        }
    }
    println!("all_sent len={}", all_sent.len());

    // Shoud have dropped all fragged packets
    assert_eq!(all_sent.len(), total_fragged);
}

pub async fn test_many_frags_reordered() {
    info!("-- test_many_frags_reordered");
    let assbuf_out = AssemblyBuffer::new();
    let assbuf_in = AssemblyBuffer::new();
    let (net_tx, net_rx) = flume::unbounded();

    let reorder_buffer = Arc::new(Mutex::new(Vec::new()));
    let sender = |framed_chunk: Vec<u8>, remote_addr: SocketAddr| {
        let reorder_buffer = reorder_buffer.clone();
        async move {
            reorder_buffer.lock().push((framed_chunk, remote_addr));
            Ok(NetworkResult::Value(()))
        }
    };

    let mut all_sent = HashSet::new();

    // Sending
    let mut total_sent_size = 0usize;
    let mut rng = rand::thread_rng();
    println!("sending");
    for _ in 0..1000 {
        let random_len = (get_random_u32() % 65536) as usize;
        total_sent_size += random_len;
        let mut message = vec![1u8; random_len];
        random_bytes(&mut message);
        let remote_addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(1, 2, 3, 4), 5678));

        // Send single message
        all_sent.insert((message.clone(), remote_addr));
        assert!(matches!(
            assbuf_out
                .split_message(message.clone(), remote_addr, sender)
                .await,
            Ok(NetworkResult::Value(()))
        ));

        // Shuffle fragments
        let items = {
            let mut rbinner = reorder_buffer.lock();
            rbinner.shuffle(&mut rng);
            let items = rbinner.clone();
            rbinner.clear();
            items
        };
        for p in items {
            net_tx.send_async(p).await.expect("should send");
        }
    }

    println!("all_sent len={}", all_sent.len());
    println!("total_sent_size = {}", total_sent_size);
    println!("fragments sent = {}", net_rx.len());

    drop(net_tx);

    // Receiving
    println!("receiving");

    while let Ok((frame, r_remote_addr)) = net_rx.recv_async().await {
        // Send to input
        let r_message = assbuf_in
            .insert_frame(&frame, r_remote_addr)
            .into_result()
            .expect("should get a value");

        // We should have gotten the same message
        if let Some(r_message) = r_message {
            assert!(all_sent.remove(&(r_message, r_remote_addr)));
        }
    }
    println!("all_sent len={}", all_sent.len());

    // Shoud have dropped no packets
    assert_eq!(all_sent.len(), 0);
}

pub async fn test_all() {
    test_single_out_in().await;
    test_one_frag_out_in().await;
    test_many_frags_out_in().await;
    test_many_frags_out_in_single_host().await;
    test_many_frags_with_drops().await;
    test_many_frags_reordered().await;
}
