use super::*;
use serde::*;
use std::io;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(transparent)]
pub struct MessageId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(transparent)]
pub struct SocketId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(transparent)]
pub struct GatewayId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum VirtualAddressType {
    IPV6,
    IPV4,
}

impl fmt::Display for VirtualAddressType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VirtualAddressType::IPV6 => write!(f, "IPV6"),
            VirtualAddressType::IPV4 => write!(f, "IPV4"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum VirtualProtocolType {
    UDP,
    TCP,
}

impl fmt::Display for VirtualProtocolType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VirtualProtocolType::UDP => write!(f, "UDP"),
            VirtualProtocolType::TCP => write!(f, "TCP"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum ServerProcessorRequest {
    AllocateMachine {
        profile: String,
    },
    ReleaseMachine {
        machine_id: MachineId,
    },
    GetInterfaces {
        machine_id: MachineId,
    },
    TcpConnect {
        machine_id: MachineId,
        local_address: Option<SocketAddr>,
        remote_address: SocketAddr,
        timeout_ms: u32,
        options: VirtualTcpOptions,
    },
    TcpBind {
        machine_id: MachineId,
        local_address: Option<SocketAddr>,
        options: VirtualTcpOptions,
    },
    TcpAccept {
        machine_id: MachineId,
        listen_socket_id: SocketId,
    },
    TcpShutdown {
        machine_id: MachineId,
        socket_id: SocketId,
    },
    UdpBind {
        machine_id: MachineId,
        local_address: Option<SocketAddr>,
        options: VirtualUdpOptions,
    },
    Send {
        machine_id: MachineId,
        socket_id: SocketId,
        data: Vec<u8>,
    },
    SendTo {
        machine_id: MachineId,
        socket_id: SocketId,
        remote_address: SocketAddr,
        data: Vec<u8>,
    },
    Recv {
        machine_id: MachineId,
        socket_id: SocketId,
        len: u32,
    },
    RecvFrom {
        machine_id: MachineId,
        socket_id: SocketId,
        len: u32,
    },
    GetRoutedLocalAddress {
        machine_id: MachineId,
        address_type: VirtualAddressType,
    },
    FindGateway {
        machine_id: MachineId,
    },
    GetExternalAddress {
        gateway_id: GatewayId,
    },
    AddPort {
        gateway_id: GatewayId,
        protocol: VirtualProtocolType,
        external_port: Option<u16>,
        local_address: SocketAddr,
        lease_duration_ms: u32,
        description: String,
    },
    RemovePort {
        gateway_id: GatewayId,
        protocol: VirtualProtocolType,
        external_port: u16,
    },
    TXTQuery {
        name: String,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct ServerProcessorMessage {
    pub message_id: MessageId,
    pub request: ServerProcessorRequest,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum ServerProcessorCommand {
    Message(ServerProcessorMessage),
    CloseSocket {
        machine_id: MachineId,
        socket_id: SocketId,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum ServerProcessorReplyValue {
    AllocateMachine {
        machine_id: MachineId,
    },
    ReleaseMachine,
    GetInterfaces {
        interfaces: BTreeMap<String, NetworkInterface>,
    },
    TcpConnect {
        socket_id: SocketId,
        local_address: SocketAddr,
    },
    TcpBind {
        socket_id: SocketId,
        local_address: SocketAddr,
    },
    TcpAccept {
        socket_id: SocketId,
        address: SocketAddr,
    },
    TcpShutdown,
    UdpBind {
        socket_id: SocketId,
        local_address: SocketAddr,
    },
    Send {
        len: u32,
    },
    SendTo {
        len: u32,
    },
    Recv {
        data: Vec<u8>,
    },
    RecvFrom {
        remote_address: SocketAddr,
        data: Vec<u8>,
    },
    GetRoutedLocalAddress {
        address: IpAddr,
    },
    FindGateway {
        opt_gateway_id: Option<GatewayId>,
    },
    GetExternalAddress {
        address: IpAddr,
    },
    AddPort {
        external_port: u16,
    },
    RemovePort,
    TXTQuery {
        result: Vec<String>,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum ServerProcessorReplyStatus {
    Value(ServerProcessorReplyValue),
    InvalidMachineId,
    InvalidSocketId,
    MissingProfile,
    ProfileComplete,
    IoError(#[serde(with = "serde_io_error::SerdeIoErrorKindDef")] io::ErrorKind),
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct ServerProcessorReply {
    pub message_id: MessageId,
    pub status: ServerProcessorReplyStatus,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum ServerProcessorEvent {
    Reply(ServerProcessorReply),
    // DeadSocket {
    //     machine_id: MachineId,
    //     socket_id: SocketId,
    // },
}
