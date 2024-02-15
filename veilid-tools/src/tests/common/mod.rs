pub mod test_async_tag_lock;
pub mod test_host_interface;

#[allow(dead_code)]
pub static DEFAULT_LOG_IGNORE_LIST: [&str; 21] = [
    "mio",
    "h2",
    "hyper",
    "tower",
    "tonic",
    "tokio",
    "runtime",
    "tokio_util",
    "want",
    "serial_test",
    "async_std",
    "async_io",
    "polling",
    "rustls",
    "async_tungstenite",
    "tungstenite",
    "netlink_proto",
    "netlink_sys",
    "hickory_resolver",
    "hickory_proto",
    "attohttpc",
];
