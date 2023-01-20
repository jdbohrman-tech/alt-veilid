use super::*;

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {

    } else {
        use std::net::{TcpListener, UdpSocket};
    }
}

#[derive(ThisError, Debug, Clone, PartialEq, Eq)]
pub enum BumpPortError {
    #[error("Unsupported architecture")]
    Unsupported,
    #[error("Failure: {0}")]
    Failed(String),
}

pub enum BumpPortType {
    UDP,
    TCP,
}

pub fn tcp_port_available(addr: &SocketAddr) -> bool {
    cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            true
        } else {
            match TcpListener::bind(addr) {
                Ok(_) => true,
                Err(_) => false,
            }
        }
    }
}

pub fn udp_port_available(addr: &SocketAddr) -> bool {
    cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            true
        } else {
            match UdpSocket::bind(addr) {
                Ok(_) => true,
                Err(_) => false,
            }
        }
    }
}

pub fn bump_port(addr: &mut SocketAddr, bpt: BumpPortType) -> Result<bool, BumpPortError> {
    cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            Err(BumpPortError::Unsupported)
        }
        else
        {
            let mut bumped = false;
            let mut port = addr.port();
            let mut addr_bump = addr.clone();
            loop {

                if match bpt {
                    BumpPortType::TCP => tcp_port_available(&addr_bump),
                    BumpPortType::UDP => udp_port_available(&addr_bump),
                } {
                    *addr = addr_bump;
                    return Ok(bumped);
                }
                if port == u16::MAX {
                    break;
                }
                port += 1;
                addr_bump.set_port(port);
                bumped = true;
            }

            Err(BumpPortError::Failure("no ports remaining".to_owned()))
        }
    }
}

pub fn bump_port_string(addr: &mut String, bpt: BumpPortType) -> Result<bool, BumpPortError> {
    cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            return Err(BumpPortError::Unsupported);
        }
        else
        {
            let savec: Vec<SocketAddr> = addr
                .to_socket_addrs()
                .map_err(|x| BumpPortError::Failure(format!("failed to resolve socket address: {}", x)))?
                .collect();

            if savec.len() == 0 {
                return Err(BumpPortError::Failure("No socket addresses resolved".to_owned()));
            }
            let mut sa = savec.first().unwrap().clone();

            if !bump_port(&mut sa, bpt)? {
                return Ok(false);
            }

            *addr = sa.to_string();

            Ok(true)
        }
    }
}
