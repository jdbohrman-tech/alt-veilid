use super::*;

use core::fmt;
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub struct IpAddrPort {
    addr: IpAddr,
    port: u16,
}

impl IpAddrPort {
    pub fn new(addr: IpAddr, port: u16) -> Self {
        Self { addr, port }
    }
    pub fn addr(&self) -> &IpAddr {
        &self.addr
    }
    pub fn port(&self) -> u16 {
        self.port
    }
    pub fn set_addr(&mut self, new_addr: IpAddr) {
        self.addr = new_addr;
    }
    pub fn set_port(&mut self, new_port: u16) {
        self.port = new_port;
    }

    pub fn from_socket_addr(sa: &SocketAddr) -> Self {
        match sa {
            SocketAddr::V4(v) => Self {
                addr: IpAddr::V4(*v.ip()),
                port: v.port(),
            },
            SocketAddr::V6(v) => Self {
                addr: IpAddr::V6(*v.ip()),
                port: v.port(),
            },
        }
    }
    pub fn to_socket_addr(&self) -> SocketAddr {
        match self.addr {
            IpAddr::V4(a) => SocketAddr::V4(SocketAddrV4::new(a, self.port)),
            IpAddr::V6(a) => SocketAddr::V6(SocketAddrV6::new(a, self.port, 0, 0)),
        }
    }
}

impl fmt::Display for IpAddrPort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.addr {
            IpAddr::V4(a) => write!(f, "{}:{}", a, self.port()),
            IpAddr::V6(a) => write!(f, "[{}]:{}", a, self.port()),
        }
    }
}

impl From<SocketAddrV4> for IpAddrPort {
    fn from(sock4: SocketAddrV4) -> IpAddrPort {
        Self::from_socket_addr(&SocketAddr::V4(sock4))
    }
}

impl From<SocketAddrV6> for IpAddrPort {
    fn from(sock6: SocketAddrV6) -> IpAddrPort {
        Self::from_socket_addr(&SocketAddr::V6(sock6))
    }
}
