use super::*;

use std::io;
use std::path::Path;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[macro_export]
macro_rules! assert_err {
    ($ex:expr) => {
        if let Ok(v) = $ex {
            panic!("assertion failed, expected Err(..), got {:?}", v);
        }
    };
}

#[macro_export]
macro_rules! io_error_other {
    ($msg:expr) => {
        io::Error::new(io::ErrorKind::Other, $msg.to_string())
    };
}

pub fn to_io_error_other<E: std::error::Error + Send + Sync + 'static>(x: E) -> io::Error {
    io::Error::new(io::ErrorKind::Other, x)
}

#[macro_export]
macro_rules! bail_io_error_other {
    ($msg:expr) => {
        return io::Result::Err(io::Error::new(io::ErrorKind::Other, $msg.to_string()))
    };
}

cfg_if::cfg_if! {
    if #[cfg(feature="rt-tokio")] {
        #[macro_export]
        macro_rules! asyncmutex_try_lock {
            ($x:expr) => {
                $x.try_lock().ok()
            };
        }

        #[macro_export]
        macro_rules! asyncmutex_lock_arc {
            ($x:expr) => {
                $x.clone().lock_owned().await
            };
        }

        #[macro_export]
        macro_rules! asyncmutex_try_lock_arc {
            ($x:expr) => {
                $x.clone().try_lock_owned().ok()
            };
        }

        // #[macro_export]
        // macro_rules! asyncrwlock_try_read {
        //     ($x:expr) => {
        //         $x.try_read().ok()
        //     };
        // }

        // #[macro_export]
        // macro_rules! asyncrwlock_try_write {
        //     ($x:expr) => {
        //         $x.try_write().ok()
        //     };
        // }
    } else {
        #[macro_export]
        macro_rules! asyncmutex_try_lock {
            ($x:expr) => {
                $x.try_lock()
            };
        }
        #[macro_export]
        macro_rules! asyncmutex_lock_arc {
            ($x:expr) => {
                $x.lock_arc().await
            };
        }
        #[macro_export]
        macro_rules! asyncmutex_try_lock_arc {
            ($x:expr) => {
                $x.try_lock_arc()
            };
        }

    }
}

#[macro_export]
macro_rules! asyncrwlock_try_read {
    ($x:expr) => {
        $x.try_read()
    };
}
#[macro_export]
macro_rules! asyncrwlock_try_write {
    ($x:expr) => {
        $x.try_write()
    };
}

#[macro_export]
macro_rules! asyncrwlock_try_read_arc {
    ($x:expr) => {
        $x.try_read_arc()
    };
}
#[macro_export]
macro_rules! asyncrwlock_try_write_arc {
    ($x:expr) => {
        $x.try_write_arc()
    };
}

//////////////////////////////////////////////////////////////////////////////////////////////////////////////

cfg_if! {
    if #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))] {
        #[must_use]
        pub fn get_concurrency() -> u32 {
            std::thread::available_parallelism()
                .map(|x| x.get())
                .unwrap_or_else(|e| {
                    warn!("unable to get concurrency defaulting to single core: {}", e);
                    1
                }) as u32
        }
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn split_port(name: &str) -> Result<(String, Option<u16>), String> {
    if let Some(split) = name.rfind(':') {
        let hoststr = &name[0..split];
        let portstr = &name[split + 1..];
        let port: u16 = portstr
            .parse::<u16>()
            .map_err(|e| format!("invalid port: {}", e))?;

        Ok((hoststr.to_string(), Some(port)))
    } else {
        Ok((name.to_string(), None))
    }
}

#[must_use]
pub fn prepend_slash(s: String) -> String {
    if s.starts_with('/') {
        return s;
    }
    let mut out = "/".to_owned();
    out.push_str(s.as_str());
    out
}

#[must_use]
pub fn timestamp_to_secs(ts: u64) -> f64 {
    ts as f64 / 1000000.0f64
}

#[must_use]
pub fn secs_to_timestamp(secs: f64) -> u64 {
    (secs * 1000000.0f64) as u64
}

#[must_use]
pub fn ms_to_us(ms: u32) -> u64 {
    (ms as u64) * 1000u64
}

pub fn us_to_ms(us: u64) -> Result<u32, String> {
    u32::try_from(us / 1000u64).map_err(|e| format!("could not convert microseconds: {}", e))
}

// Calculate retry attempt with logarhythmic falloff
#[must_use]
pub fn retry_falloff_log(
    last_us: u64,
    cur_us: u64,
    interval_start_us: u64,
    interval_max_us: u64,
    interval_multiplier_us: f64,
) -> bool {
    //
    if cur_us < interval_start_us {
        // Don't require a retry within the first 'interval_start_us' microseconds of the reliable time period
        false
    } else if cur_us >= last_us + interval_max_us {
        // Retry at least every 'interval_max_us' microseconds
        true
    } else {
        // Exponential falloff between 'interval_start_us' and 'interval_max_us' microseconds
        last_us <= secs_to_timestamp(timestamp_to_secs(cur_us) / interval_multiplier_us)
    }
}

pub fn try_at_most_n_things<T, I, C, R>(max: usize, things: I, closure: C) -> Option<R>
where
    I: IntoIterator<Item = T>,
    C: Fn(T) -> Option<R>,
{
    let mut fails = 0usize;
    for thing in things.into_iter() {
        if let Some(r) = closure(thing) {
            return Some(r);
        }
        fails += 1;
        if fails >= max {
            break;
        }
    }
    None
}

pub async fn async_try_at_most_n_things<T, I, C, R, F>(
    max: usize,
    things: I,
    closure: C,
) -> Option<R>
where
    I: IntoIterator<Item = T>,
    C: Fn(T) -> F,
    F: Future<Output = Option<R>>,
{
    let mut fails = 0usize;
    for thing in things.into_iter() {
        if let Some(r) = closure(thing).await {
            return Some(r);
        }
        fails += 1;
        if fails >= max {
            break;
        }
    }
    None
}

pub trait CmpAssign {
    fn min_assign(&mut self, other: Self);
    fn max_assign(&mut self, other: Self);
}

impl<T> CmpAssign for T
where
    T: core::cmp::Ord,
{
    fn min_assign(&mut self, other: Self) {
        if &other < self {
            *self = other;
        }
    }
    fn max_assign(&mut self, other: Self) {
        if &other > self {
            *self = other;
        }
    }
}

#[must_use]
pub fn compatible_unspecified_socket_addr(socket_addr: &SocketAddr) -> SocketAddr {
    match socket_addr {
        SocketAddr::V4(_) => SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0),
        SocketAddr::V6(_) => SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)), 0),
    }
}

cfg_if! {
    if #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))] {
        use std::net::UdpSocket;

        static IPV6_IS_SUPPORTED: Mutex<Option<bool>> = Mutex::new(None);

        pub fn is_ipv6_supported() -> bool {
            let mut opt_supp = IPV6_IS_SUPPORTED.lock();
            if let Some(supp) = *opt_supp {
                return supp;
            }
            // Not exhaustive but for our use case it should be sufficient. If no local ports are available for binding, Veilid isn't going to work anyway :P
            let supp = UdpSocket::bind(SocketAddrV6::new(Ipv6Addr::LOCALHOST, 0, 0, 0)).is_ok();
            *opt_supp = Some(supp);
            supp
        }
    }
}

#[must_use]
pub fn available_unspecified_addresses() -> Vec<IpAddr> {
    if is_ipv6_supported() {
        vec![
            IpAddr::V4(Ipv4Addr::UNSPECIFIED),
            IpAddr::V6(Ipv6Addr::UNSPECIFIED),
        ]
    } else {
        vec![IpAddr::V4(Ipv4Addr::UNSPECIFIED)]
    }
}

pub fn listen_address_to_socket_addrs(listen_address: &str) -> Result<Vec<SocketAddr>, String> {
    // If no address is specified, but the port is, use ipv4 and ipv6 unspecified
    // If the address is specified, only use the specified port and fail otherwise

    let ip_addrs = available_unspecified_addresses();

    Ok(if let Some(portstr) = listen_address.strip_prefix(':') {
        let port = portstr
            .parse::<u16>()
            .map_err(|e| format!("Invalid port format in udp listen address: {}", e))?;
        ip_addrs.iter().map(|a| SocketAddr::new(*a, port)).collect()
    } else if let Ok(port) = listen_address.parse::<u16>() {
        ip_addrs.iter().map(|a| SocketAddr::new(*a, port)).collect()
    } else {
        let listen_address_with_port = if listen_address.contains(':') {
            listen_address.to_string()
        } else {
            format!("{}:0", listen_address)
        };
        cfg_if! {
            if #[cfg(all(target_arch = "wasm32", target_os = "unknown"))] {
                use core::str::FromStr;
                vec![SocketAddr::from_str(&listen_address_with_port).map_err(|e| format!("Unable to parse address: {}",e))?]
            } else {
                listen_address_with_port
                    .to_socket_addrs()
                    .map_err(|e| format!("Unable to resolve address: {}", e))?
                    .collect()
            }
        }
    })
}

/// Dedup, but doesn't require a sorted vec, and keeps the element order
pub trait RemoveDuplicates<T: PartialEq + Clone> {
    fn remove_duplicates(&mut self);
}

impl<T: PartialEq + Clone> RemoveDuplicates<T> for Vec<T> {
    fn remove_duplicates(&mut self) {
        let mut already_seen = Vec::new();
        self.retain(|item| match already_seen.contains(item) {
            true => false,
            _ => {
                already_seen.push(item.clone());
                true
            }
        })
    }
}

cfg_if::cfg_if! {
    if #[cfg(unix)] {
        use std::os::unix::fs::MetadataExt;
        use std::os::unix::prelude::PermissionsExt;
        use nix::unistd::{Uid, Gid};

        pub fn ensure_file_private_owner<P:AsRef<Path>>(path: P) -> Result<(), String>
        {
            let path = path.as_ref();
            if !path.is_file() {
                return Ok(());
            }

            let uid = Uid::effective();
            let gid = Gid::effective();
            let meta = std::fs::metadata(path).map_err(|e| format!("unable to get metadata for path: {}", e))?;

            if meta.mode() != 0o600 {
                std::fs::set_permissions(path,std::fs::Permissions::from_mode(0o600)).map_err(|e| format!("unable to set correct permissions on path: {}", e))?;
            }
            if meta.uid() != uid.as_raw() || meta.gid() != gid.as_raw() {
                return Err("path has incorrect owner/group".to_owned());
            }
            Ok(())
        }

        pub fn ensure_directory_private_owner<P:AsRef<Path>>(path: P, group_read: bool) -> Result<(), String>
        {
            let path = path.as_ref();
            if !path.is_dir() {
                return Ok(());
            }

            let uid = Uid::effective();
            let gid = Gid::effective();
            let meta = std::fs::metadata(path).map_err(|e| format!("unable to get metadata for path: {}", e))?;

            let perm = if group_read {
                0o750
            } else {
                0o700
            };

            if meta.mode() != perm {
                std::fs::set_permissions(path,std::fs::Permissions::from_mode(perm)).map_err(|e| format!("unable to set correct permissions on path: {}", e))?;
            }
            if meta.uid() != uid.as_raw() || meta.gid() != gid.as_raw() {
                return Err("path has incorrect owner/group".to_owned());
            }
            Ok(())
        }
    } else if #[cfg(windows)] {
        //use std::os::windows::fs::MetadataExt;
        //use windows_permissions::*;

        pub fn ensure_file_private_owner<P:AsRef<Path>>(path: P) -> Result<(), String>
        {
            let path = path.as_ref();
            if !path.is_file() {
                return Ok(());
            }

            Ok(())
        }

        pub fn ensure_directory_private_owner<P:AsRef<Path>>(path: P, _group_read: bool) -> Result<(), String>
        {
            let path = path.as_ref();
            if !path.is_dir() {
                return Ok(());
            }

            Ok(())
        }

    } else {
        pub fn ensure_file_private_owner<P:AsRef<Path>>(path: P) -> Result<(), String>
        {
            let path = path.as_ref();
            if !path.is_file() {
                return Ok(());
            }

            Ok(())
        }

        pub fn ensure_directory_private_owner<P:AsRef<Path>>(path: P, _group_read: bool) -> Result<(), String>
        {
            let path = path.as_ref();
            if !path.is_dir() {
                return Ok(());
            }

            Ok(())
        }
    }
}

#[repr(C, align(8))]
struct AlignToEight([u8; 8]);

/// # Safety
/// Ensure you immediately initialize this vector as it could contain sensitive data
#[must_use]
pub unsafe fn aligned_8_u8_vec_uninit(n_bytes: usize) -> Vec<u8> {
    let n_units = n_bytes.div_ceil(mem::size_of::<AlignToEight>());
    let mut aligned: Vec<AlignToEight> = Vec::with_capacity(n_units);
    let ptr = aligned.as_mut_ptr();
    let cap_units = aligned.capacity();
    mem::forget(aligned);

    Vec::from_raw_parts(
        ptr as *mut u8,
        n_bytes,
        cap_units * mem::size_of::<AlignToEight>(),
    )
}

/// # Safety
/// Ensure you immediately initialize this vector as it could contain sensitive data
#[must_use]
pub unsafe fn unaligned_u8_vec_uninit(n_bytes: usize) -> Vec<u8> {
    let mut unaligned: Vec<u8> = Vec::with_capacity(n_bytes);
    let ptr = unaligned.as_mut_ptr();
    mem::forget(unaligned);

    Vec::from_raw_parts(ptr, n_bytes, n_bytes)
}

#[must_use]
pub fn debug_backtrace() -> String {
    let bt = backtrace::Backtrace::new();
    format!("{:?}", bt)
}

pub fn debug_print_backtrace() {
    if is_debug_backtrace_enabled() {
        debug!("{}", debug_backtrace());
    }
}

#[must_use]
pub fn is_debug_backtrace_enabled() -> bool {
    cfg_if! {
        if #[cfg(debug_assertions)] {
            cfg_if! {
                if #[cfg(all(target_arch = "wasm32", target_os = "unknown"))] {
                    let rbenv = get_wasm_global_string_value("RUST_BACKTRACE").unwrap_or_default();
                }
                else
                {
                    let rbenv = std::env::var("RUST_BACKTRACE").unwrap_or_default();
                }
            }
            rbenv == "1" || rbenv == "full"
        } else {
            false
        }
    }
}

#[track_caller]
pub fn debug_duration<R, F: Future<Output = R>, T: FnOnce() -> F>(
    f: T,
    opt_timeout_us: Option<u64>,
) -> impl Future<Output = R> {
    let location = core::panic::Location::caller();
    async move {
        let t1 = get_timestamp();
        let out = f().await;
        let t2 = get_timestamp();
        let duration_us = t2 - t1;
        if let Some(timeout_us) = opt_timeout_us {
            if duration_us > timeout_us {
                #[cfg(not(feature = "debug-duration-timeout"))]
                debug!(
                    "Excessive duration: {}\n{:?}",
                    display_duration(duration_us),
                    backtrace::Backtrace::new()
                );
                #[cfg(feature = "debug-duration-timeout")]
                panic!(format!(
                    "Duration panic timeout exceeded: {}",
                    display_duration(duration_us)
                ));
            }
        } else {
            debug!("Duration: {} = {}", location, display_duration(duration_us),);
        }
        out
    }
}

pub fn type_name_of_val<T: ?Sized>(_val: &T) -> &'static str {
    std::any::type_name::<T>()
}

pub fn map_to_string<X: ToString>(arg: X) -> String {
    arg.to_string()
}

//////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct DebugGuard {
    name: &'static str,
    counter: &'static AtomicUsize,
}

impl DebugGuard {
    pub fn new(name: &'static str, counter: &'static AtomicUsize) -> Self {
        let c = counter.fetch_add(1, Ordering::SeqCst);
        eprintln!("{} entered: {}", name, c + 1);
        Self { name, counter }
    }
}

impl Drop for DebugGuard {
    fn drop(&mut self) {
        let c = self.counter.fetch_sub(1, Ordering::SeqCst);
        eprintln!("{} exited: {}", self.name, c - 1);
    }
}
