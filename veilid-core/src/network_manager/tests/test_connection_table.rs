use super::*;

use super::connection_table::*;
use crate::tests::mock_registry;

pub async fn test_add_get_remove() {
    let registry = mock_registry::init("").await;

    let table = ConnectionTable::new(registry.clone());

    let a1 = Flow::new_no_local(PeerAddress::new(
        SocketAddress::new(Address::IPV4(Ipv4Addr::new(192, 168, 0, 1)), 8080),
        ProtocolType::TCP,
    ));
    let a2 = a1;
    let a3 = Flow::new(
        PeerAddress::new(
            SocketAddress::new(Address::IPV6(Ipv6Addr::new(191, 0, 0, 0, 0, 0, 0, 1)), 8090),
            ProtocolType::TCP,
        ),
        SocketAddress::from_socket_addr(SocketAddr::V6(SocketAddrV6::new(
            Ipv6Addr::new(191, 0, 0, 0, 0, 0, 0, 1),
            8080,
            0,
            0,
        ))),
    );
    let a4 = Flow::new(
        PeerAddress::new(
            SocketAddress::new(Address::IPV6(Ipv6Addr::new(192, 0, 0, 0, 0, 0, 0, 1)), 8090),
            ProtocolType::TCP,
        ),
        SocketAddress::from_socket_addr(SocketAddr::V6(SocketAddrV6::new(
            Ipv6Addr::new(192, 0, 0, 0, 0, 0, 0, 1),
            8080,
            0,
            0,
        ))),
    );
    let a5 = Flow::new(
        PeerAddress::new(
            SocketAddress::new(Address::IPV6(Ipv6Addr::new(192, 0, 0, 0, 0, 0, 0, 1)), 8090),
            ProtocolType::WSS,
        ),
        SocketAddress::from_socket_addr(SocketAddr::V6(SocketAddrV6::new(
            Ipv6Addr::new(193, 0, 0, 0, 0, 0, 0, 1),
            8080,
            0,
            0,
        ))),
    );

    let c1 = NetworkConnection::dummy(registry.clone(), 1.into(), a1);
    let c1b = NetworkConnection::dummy(registry.clone(), 10.into(), a1);
    let c1h = c1.get_handle();
    let c2 = NetworkConnection::dummy(registry.clone(), 2.into(), a2);
    let c3 = NetworkConnection::dummy(registry.clone(), 3.into(), a3);
    let c4 = NetworkConnection::dummy(registry.clone(), 4.into(), a4);
    let c5 = NetworkConnection::dummy(registry.clone(), 5.into(), a5);

    assert_eq!(a1, c2.flow());
    assert_ne!(a3, c4.flow());
    assert_ne!(a4, c5.flow());

    assert_eq!(table.connection_count(), 0);
    assert_eq!(table.peek_connection_by_flow(a1), None);
    table.add_connection(c1).unwrap();
    assert!(table.add_connection(c1b).is_err());

    assert_eq!(table.connection_count(), 1);
    assert!(table.remove_connection_by_id(4.into()).is_none());
    assert!(table.remove_connection_by_id(5.into()).is_none());
    assert_eq!(table.connection_count(), 1);
    assert_eq!(table.peek_connection_by_flow(a1), Some(c1h.clone()));
    assert_eq!(table.peek_connection_by_flow(a1), Some(c1h.clone()));
    assert_eq!(table.connection_count(), 1);
    assert_err!(table.add_connection(c2));
    assert_eq!(table.connection_count(), 1);
    assert_eq!(table.peek_connection_by_flow(a1), Some(c1h.clone()));
    assert_eq!(table.peek_connection_by_flow(a1), Some(c1h.clone()));
    assert_eq!(table.connection_count(), 1);
    assert_eq!(
        table
            .remove_connection_by_id(1.into())
            .map(|c| c.flow())
            .unwrap(),
        a1
    );
    assert_eq!(table.connection_count(), 0);
    assert!(table.remove_connection_by_id(2.into()).is_none());
    assert_eq!(table.connection_count(), 0);
    assert_eq!(table.peek_connection_by_flow(a2), None);
    assert_eq!(table.peek_connection_by_flow(a1), None);
    assert_eq!(table.connection_count(), 0);
    let c1 = NetworkConnection::dummy(registry.clone(), 6.into(), a1);
    table.add_connection(c1).unwrap();
    let c2 = NetworkConnection::dummy(registry.clone(), 7.into(), a2);
    assert_err!(table.add_connection(c2));
    table.add_connection(c3).unwrap();
    table.add_connection(c4).unwrap();
    assert_eq!(table.connection_count(), 3);
    assert_eq!(
        table
            .remove_connection_by_id(6.into())
            .map(|c| c.flow())
            .unwrap(),
        a2
    );
    assert_eq!(
        table
            .remove_connection_by_id(3.into())
            .map(|c| c.flow())
            .unwrap(),
        a3
    );
    assert_eq!(
        table
            .remove_connection_by_id(4.into())
            .map(|c| c.flow())
            .unwrap(),
        a4
    );
    assert_eq!(table.connection_count(), 0);

    mock_registry::terminate(registry).await;
}

pub async fn test_all() {
    test_add_get_remove().await;
}
