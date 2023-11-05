use crate::*;
use async_io::Async;
use std::io;

cfg_if! {
    if #[cfg(feature="rt-async-std")] {
        pub use async_std::net::{TcpStream, TcpListener, Shutdown, UdpSocket};
    } else if #[cfg(feature="rt-tokio")] {
        pub use tokio::net::{TcpStream, TcpListener, UdpSocket};
        pub use tokio_util::compat::*;
    } else {
        compile_error!("needs executor implementation")
    }
}

use socket2::{Domain, Protocol, SockAddr, Socket, Type};

cfg_if! {
    if #[cfg(windows)] {
        use winapi::shared::ws2def::{ SOL_SOCKET, SO_EXCLUSIVEADDRUSE};
        use winapi::um::winsock2::{SOCKET_ERROR, setsockopt};
        use winapi::ctypes::c_int;
        use std::os::windows::io::AsRawSocket;

        fn set_exclusiveaddruse(socket: &Socket) -> io::Result<()> {
            unsafe {
                let optval:c_int = 1;
                if setsockopt(socket.as_raw_socket().try_into().unwrap(), SOL_SOCKET, SO_EXCLUSIVEADDRUSE, (&optval as *const c_int).cast(),
                    std::mem::size_of::<c_int>() as c_int) == SOCKET_ERROR {
                    return Err(io::Error::last_os_error());
                }
                Ok(())
            }
        }
    }
}

#[instrument(level = "trace", ret)]
pub fn new_unbound_shared_udp_socket(domain: Domain) -> io::Result<Socket> {
    let socket = Socket::new(domain, Type::DGRAM, Some(Protocol::UDP))?;
    if domain == Domain::IPV6 {
        socket.set_only_v6(true)?;
    }
    socket.set_reuse_address(true)?;

    cfg_if! {
        if #[cfg(unix)] {
            socket.set_reuse_port(true)?;
        }
    }
    Ok(socket)
}

#[instrument(level = "trace", ret)]
pub fn new_bound_shared_udp_socket(local_address: SocketAddr) -> io::Result<Socket> {
    let domain = Domain::for_address(local_address);
    let socket = new_unbound_shared_udp_socket(domain)?;
    let socket2_addr = SockAddr::from(local_address);
    socket.bind(&socket2_addr)?;

    log_net!("created bound shared udp socket on {:?}", &local_address);

    Ok(socket)
}

#[instrument(level = "trace", ret)]
pub fn new_bound_first_udp_socket(local_address: SocketAddr) -> io::Result<Socket> {
    let domain = Domain::for_address(local_address);
    let socket = Socket::new(domain, Type::DGRAM, Some(Protocol::UDP))?;
    if domain == Domain::IPV6 {
        socket.set_only_v6(true)?;
    }
    // Bind the socket -first- before turning on 'reuse address' this way it will
    // fail if the port is already taken
    let socket2_addr = SockAddr::from(local_address);

    // On windows, do SO_EXCLUSIVEADDRUSE before the bind to ensure the port is fully available
    cfg_if! {
        if #[cfg(windows)] {
            set_exclusiveaddruse(&socket)?;
        }
    }

    socket.bind(&socket2_addr)?;

    // Set 'reuse address' so future binds to this port will succeed
    // This does not work on Windows, where reuse options can not be set after the bind
    cfg_if! {
        if #[cfg(unix)] {
            socket
                .set_reuse_address(true)?;
            socket.set_reuse_port(true)?;
        }
    }
    log_net!("created bound first udp socket on {:?}", &local_address);

    Ok(socket)
}

#[instrument(level = "trace", ret)]
pub fn new_unbound_tcp_socket(domain: Domain) -> io::Result<Socket> {
    let socket = Socket::new(domain, Type::STREAM, Some(Protocol::TCP))?;
    if let Err(e) = socket.set_nodelay(true) {
        log_net!(error "Couldn't set TCP nodelay: {}", e);
    }
    if domain == Domain::IPV6 {
        socket.set_only_v6(true)?;
    }
    Ok(socket)
}

#[instrument(level = "trace", ret)]
pub fn new_unbound_shared_tcp_socket(domain: Domain) -> io::Result<Socket> {
    let socket = Socket::new(domain, Type::STREAM, Some(Protocol::TCP))?;
    // if let Err(e) = socket.set_linger(Some(core::time::Duration::from_secs(0))) {
    //     log_net!(error "Couldn't set TCP linger: {}", e);
    // }
    if let Err(e) = socket.set_nodelay(true) {
        log_net!(error "Couldn't set TCP nodelay: {}", e);
    }
    if domain == Domain::IPV6 {
        socket.set_only_v6(true)?;
    }
    socket.set_reuse_address(true)?;
    cfg_if! {
        if #[cfg(unix)] {
            socket.set_reuse_port(true)?;
        }
    }

    Ok(socket)
}

#[instrument(level = "trace", ret)]
pub fn new_bound_shared_tcp_socket(local_address: SocketAddr) -> io::Result<Socket> {
    let domain = Domain::for_address(local_address);
    let socket = new_unbound_shared_tcp_socket(domain)?;
    let socket2_addr = SockAddr::from(local_address);
    socket.bind(&socket2_addr)?;

    log_net!("created bound shared tcp socket on {:?}", &local_address);

    Ok(socket)
}

#[instrument(level = "trace", ret)]
pub fn new_bound_first_tcp_socket(local_address: SocketAddr) -> io::Result<Socket> {
    let domain = Domain::for_address(local_address);

    let socket = Socket::new(domain, Type::STREAM, Some(Protocol::TCP))?;
    // if let Err(e) = socket.set_linger(Some(core::time::Duration::from_secs(0))) {
    //     log_net!(error "Couldn't set TCP linger: {}", e);
    // }
    if let Err(e) = socket.set_nodelay(true) {
        log_net!(error "Couldn't set TCP nodelay: {}", e);
    }
    if domain == Domain::IPV6 {
        socket.set_only_v6(true)?;
    }

    // On windows, do SO_EXCLUSIVEADDRUSE before the bind to ensure the port is fully available
    cfg_if! {
        if #[cfg(windows)] {
            set_exclusiveaddruse(&socket)?;
        }
    }

    // Bind the socket -first- before turning on 'reuse address' this way it will
    // fail if the port is already taken
    let socket2_addr = SockAddr::from(local_address);
    socket.bind(&socket2_addr)?;

    // Set 'reuse address' so future binds to this port will succeed
    // This does not work on Windows, where reuse options can not be set after the bind
    cfg_if! {
        if #[cfg(unix)] {
        socket
            .set_reuse_address(true)?;
        socket.set_reuse_port(true)?;
        }
    }
    log_net!("created bound first tcp socket on {:?}", &local_address);

    Ok(socket)
}

// Non-blocking connect is tricky when you want to start with a prepared socket
// Errors should not be logged as they are valid conditions for this function
#[instrument(level = "trace", ret)]
pub async fn nonblocking_connect(
    socket: Socket,
    addr: SocketAddr,
    timeout_ms: u32,
) -> io::Result<TimeoutOr<TcpStream>> {
    // Set for non blocking connect
    socket.set_nonblocking(true)?;

    // Make socket2 SockAddr
    let socket2_addr = socket2::SockAddr::from(addr);

    // Connect to the remote address
    match socket.connect(&socket2_addr) {
        Ok(()) => Ok(()),
        #[cfg(unix)]
        Err(err) if err.raw_os_error() == Some(libc::EINPROGRESS) => Ok(()),
        Err(err) if err.kind() == std::io::ErrorKind::WouldBlock => Ok(()),
        Err(e) => Err(e),
    }?;
    let async_stream = Async::new(std::net::TcpStream::from(socket))?;

    // The stream becomes writable when connected
    timeout_or_try!(timeout(timeout_ms, async_stream.writable())
        .await
        .into_timeout_or()
        .into_result()?);

    // Check low level error
    let async_stream = match async_stream.get_ref().take_error()? {
        None => Ok(async_stream),
        Some(err) => Err(err),
    }?;

    // Convert back to inner and then return async version
    cfg_if! {
        if #[cfg(feature="rt-async-std")] {
            Ok(TimeoutOr::value(TcpStream::from(async_stream.into_inner()?)))
        } else if #[cfg(feature="rt-tokio")] {
            Ok(TimeoutOr::value(TcpStream::from_std(async_stream.into_inner()?)?))
        } else {
            compile_error!("needs executor implementation")
        }
    }
}
