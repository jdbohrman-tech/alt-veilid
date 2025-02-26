use super::*;
use async_io::Async;
use std::io;

cfg_if! {
    if #[cfg(feature="rt-async-std")] {
        pub use async_std::net::{TcpStream, TcpListener, UdpSocket};
    } else if #[cfg(feature="rt-tokio")] {
        pub use tokio::net::{TcpStream, TcpListener, UdpSocket};
        pub use tokio_util::compat::*;
    } else {
        compile_error!("needs executor implementation");
    }
}

use socket2::{Domain, Protocol, SockAddr, Socket, Type};

//////////////////////////////////////////////////////////////////////////////////////////

pub fn bind_async_udp_socket(local_address: SocketAddr) -> io::Result<Option<UdpSocket>> {
    let Some(socket) = new_bound_default_socket2_udp(local_address)? else {
        return Ok(None);
    };

    // Make an async UdpSocket from the socket2 socket
    let std_udp_socket: std::net::UdpSocket = socket.into();
    cfg_if! {
        if #[cfg(feature="rt-async-std")] {
            let udp_socket = UdpSocket::from(std_udp_socket);
        } else if #[cfg(feature="rt-tokio")] {
            std_udp_socket.set_nonblocking(true)?;
            let udp_socket = UdpSocket::from_std(std_udp_socket)?;
        } else {
            compile_error!("needs executor implementation");
        }
    }
    Ok(Some(udp_socket))
}

pub fn bind_async_tcp_listener(local_address: SocketAddr) -> io::Result<Option<TcpListener>> {
    // Create a default non-shared socket and bind it
    let Some(socket) = new_bound_default_socket2_tcp(local_address)? else {
        return Ok(None);
    };

    // Drop the socket so we can make another shared socket in its place
    drop(socket);

    // Create a shared socket and bind it now we have determined the port is free
    let Some(socket) = new_bound_shared_socket2_tcp(local_address)? else {
        return Ok(None);
    };

    // Listen on the socket
    if socket.listen(128).is_err() {
        return Ok(None);
    }

    // Make an async tcplistener from the socket2 socket
    let std_listener: std::net::TcpListener = socket.into();
    cfg_if! {
        if #[cfg(feature="rt-async-std")] {
            let listener = TcpListener::from(std_listener);
        } else if #[cfg(feature="rt-tokio")] {
            std_listener.set_nonblocking(true)?;
            let listener = TcpListener::from_std(std_listener)?;
        } else {
            compile_error!("needs executor implementation");
        }
    }
    Ok(Some(listener))
}

pub async fn connect_async_tcp_stream(
    local_address: Option<SocketAddr>,
    remote_address: SocketAddr,
    timeout_ms: u32,
) -> io::Result<TimeoutOr<TcpStream>> {
    let socket = match local_address {
        Some(a) => {
            new_bound_shared_socket2_tcp(a)?.ok_or(io::Error::from(io::ErrorKind::AddrInUse))?
        }
        None => new_default_socket2_tcp(domain_for_address(remote_address))?,
    };

    // Non-blocking connect to remote address
    nonblocking_connect(socket, remote_address, timeout_ms).await
}

pub fn set_tcp_stream_linger(
    tcp_stream: &TcpStream,
    linger: Option<core::time::Duration>,
) -> io::Result<()> {
    #[cfg(all(feature = "rt-async-std", unix))]
    {
        // async-std does not directly support linger on TcpStream yet
        use std::os::fd::{AsRawFd, FromRawFd, IntoRawFd};
        unsafe {
            let s = socket2::Socket::from_raw_fd(tcp_stream.as_raw_fd());
            let res = s.set_linger(linger);
            let _ = s.into_raw_fd();
            res
        }
    }
    #[cfg(all(feature = "rt-async-std", windows))]
    {
        // async-std does not directly support linger on TcpStream yet
        use std::os::windows::io::{AsRawSocket, FromRawSocket, IntoRawSocket};
        unsafe {
            let s = socket2::Socket::from_raw_socket(tcp_stream.as_raw_socket());
            let res = s.set_linger(linger);
            let _ = s.into_raw_socket();
            res
        }
    }
    #[cfg(not(feature = "rt-async-std"))]
    tcp_stream.set_linger(linger)
}

cfg_if! {
    if #[cfg(feature="rt-async-std")] {
        pub type ReadHalf = futures_util::io::ReadHalf<TcpStream>;
        pub type WriteHalf = futures_util::io::WriteHalf<TcpStream>;
    } else if #[cfg(feature="rt-tokio")] {
        pub type ReadHalf = tokio::net::tcp::OwnedReadHalf;
        pub type WriteHalf = tokio::net::tcp::OwnedWriteHalf;
    } else {
        compile_error!("needs executor implementation");
    }
}

pub fn async_tcp_listener_incoming(
    tcp_listener: TcpListener,
) -> Pin<Box<impl futures_util::stream::Stream<Item = std::io::Result<TcpStream>> + Send>> {
    cfg_if! {
        if #[cfg(feature="rt-async-std")] {
            Box::pin(tcp_listener.into_incoming())
        } else if #[cfg(feature="rt-tokio")] {
            Box::pin(tokio_stream::wrappers::TcpListenerStream::new(tcp_listener))
        } else {
            compile_error!("needs executor implementation");
        }
    }
}

pub fn split_async_tcp_stream(tcp_stream: TcpStream) -> (ReadHalf, WriteHalf) {
    cfg_if! {
        if #[cfg(feature="rt-async-std")] {
            use futures_util::AsyncReadExt;
            tcp_stream.split()
        } else if #[cfg(feature="rt-tokio")] {
            tcp_stream.into_split()
        } else {
            compile_error!("needs executor implementation");
        }
    }
}

//////////////////////////////////////////////////////////////////////////////////////////

fn new_default_udp_socket(domain: core::ffi::c_int) -> io::Result<Socket> {
    let domain = Domain::from(domain);
    let socket = Socket::new(domain, Type::DGRAM, Some(Protocol::UDP))?;
    if domain == Domain::IPV6 {
        socket.set_only_v6(true)?;
    }

    Ok(socket)
}

fn new_bound_default_socket2_udp(local_address: SocketAddr) -> io::Result<Option<Socket>> {
    let domain = domain_for_address(local_address);
    let socket = new_default_udp_socket(domain)?;
    let socket2_addr = SockAddr::from(local_address);

    if socket.bind(&socket2_addr).is_err() {
        return Ok(None);
    }

    Ok(Some(socket))
}

pub fn new_default_socket2_tcp(domain: core::ffi::c_int) -> io::Result<Socket> {
    let domain = Domain::from(domain);
    let socket = Socket::new(domain, Type::STREAM, Some(Protocol::TCP))?;
    socket.set_linger(Some(core::time::Duration::from_secs(0)))?;
    socket.set_nodelay(true)?;
    if domain == Domain::IPV6 {
        socket.set_only_v6(true)?;
    }
    Ok(socket)
}

fn new_shared_socket2_tcp(domain: core::ffi::c_int) -> io::Result<Socket> {
    let domain = Domain::from(domain);
    let socket = Socket::new(domain, Type::STREAM, Some(Protocol::TCP))?;
    socket.set_linger(Some(core::time::Duration::from_secs(0)))?;
    socket.set_nodelay(true)?;
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

fn new_bound_default_socket2_tcp(local_address: SocketAddr) -> io::Result<Option<Socket>> {
    let domain = domain_for_address(local_address);
    let socket = new_default_socket2_tcp(domain)?;
    let socket2_addr = SockAddr::from(local_address);
    if socket.bind(&socket2_addr).is_err() {
        return Ok(None);
    }

    Ok(Some(socket))
}

fn new_bound_shared_socket2_tcp(local_address: SocketAddr) -> io::Result<Option<Socket>> {
    // Create the reuseaddr/reuseport socket now that we've asserted the port is free
    let domain = domain_for_address(local_address);
    let socket = new_shared_socket2_tcp(domain)?;
    let socket2_addr = SockAddr::from(local_address);
    if socket.bind(&socket2_addr).is_err() {
        return Ok(None);
    }

    Ok(Some(socket))
}

// Non-blocking connect is tricky when you want to start with a prepared socket
// Errors should not be logged as they are valid conditions for this function
async fn nonblocking_connect(
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
            compile_error!("needs executor implementation");
        }
    }
}

#[must_use]
pub fn domain_for_address(address: SocketAddr) -> core::ffi::c_int {
    socket2::Domain::for_address(address).into()
}

// Run operations on underlying socket
cfg_if! {
    if #[cfg(unix)] {
        pub fn socket2_operation<S: std::os::fd::AsRawFd, F: FnOnce(&mut socket2::Socket) -> R, R>(
            s: &S,
            callback: F,
        ) -> R {
            use std::os::fd::{FromRawFd, IntoRawFd};
            let mut s = unsafe { socket2::Socket::from_raw_fd(s.as_raw_fd()) };
            let res = callback(&mut s);
            let _ = s.into_raw_fd();
            res
        }
    } else if #[cfg(windows)] {
        pub fn socket2_operation<
            S: std::os::windows::io::AsRawSocket,
            F: FnOnce(&mut socket2::Socket) -> R,
            R,
        >(
            s: &S,
            callback: F,
        ) -> R {
            use std::os::windows::io::{FromRawSocket, IntoRawSocket};
            let mut s = unsafe { socket2::Socket::from_raw_socket(s.as_raw_socket()) };
            let res = callback(&mut s);
            let _ = s.into_raw_socket();
            res
        }
    } else {
        #[compile_error("unimplemented")]
    }
}
