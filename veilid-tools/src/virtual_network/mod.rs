//! # Virtual Network
//!
//! ## Networking abstraction layer
//!
//! Support for mocking and virtualizing network connections, as well as passing through to supported
//! networking functionality.
//!
//! The following structs are available that allow connecting to a centralized virtual
//! router to emulate a large scale network.
//!
//! * RouterClient
//! * RouterServer
//! * Machine
//!
//! Additional traits are is implemented for all shimmed APIs that have static methods
//! like `new()`, `default()`, `connect()` and `bind()` to allow optional namespacing
//! such that the structs they produce are new network router clients with their own
//! distinct IP addresses, network segments, and network characteristics as allocated
//! by the [RouterServer].
//!
//! A singleton RouterClient can be registered with this module that is used by default unless the
//! `*_with_machine` API are used to override it with another Machine instance.
//!
//! ## Available APIs
//!
//! [VirtualTcpStream]
//! [VirtualUdpSocket]
//! [VirtualTcpListener]
//! [VirtualTcpListenerStream]
//! [VirtualGateway]
//! [VirtualWsMeta]
//! [VirtualWsStream]
//!
//! Traits are implemented for [futures_util::AsyncRead] and [futures_util::AsyncWrite]
//! Conversion traits are available for use with Tokio
//!
//! ## Other modules leveraging this module
//!
//! * `veilid-core`'s network `native` and `wasm` modules
//! * This crate's `network_interfaces` module
//! * This crate's `dns_lookup` module

mod commands;
mod machine;
mod router_client;
mod router_op_table;
#[cfg(all(
    feature = "virtual-network-server",
    not(all(target_arch = "wasm32", target_os = "unknown"))
))]
mod router_server;
mod serde_io_error;
mod virtual_gateway;
mod virtual_network_error;
mod virtual_tcp_listener;
mod virtual_tcp_listener_stream;
mod virtual_tcp_stream;
mod virtual_udp_socket;

use super::*;
use commands::*;

pub use machine::*;
pub use router_client::*;
#[cfg(all(
    feature = "virtual-network-server",
    not(all(target_arch = "wasm32", target_os = "unknown"))
))]
pub use router_server::*;
pub use virtual_gateway::*;
pub use virtual_network_error::*;
pub use virtual_tcp_listener::*;
pub use virtual_tcp_listener_stream::*;
pub use virtual_tcp_stream::*;
pub use virtual_udp_socket::*;
