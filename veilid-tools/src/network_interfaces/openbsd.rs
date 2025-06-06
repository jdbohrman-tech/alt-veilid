#![cfg(target_os = "openbsd")]
#![allow(non_camel_case_types)]
use super::*;

use libc::{
    c_short, close, freeifaddrs, getifaddrs, if_nametoindex, ifaddrs, ioctl, pid_t, sockaddr,
    sockaddr_in6, socket, sysctl, time_t, AF_INET6, CTL_NET, IFF_BROADCAST, IFF_LOOPBACK,
    IFF_POINTOPOINT, IFF_RUNNING, IFNAMSIZ, NET_RT_FLAGS, PF_ROUTE, SOCK_DGRAM,
};
use sockaddr_tools::SockAddr;
use std::ffi::CStr;
use std::io;
use std::os::raw::{c_int, c_uchar, c_ulong, c_ushort, c_void};

const SIOCGIFAFLAG_IN6: c_ulong = 0xC1206949;
const SIOCGIFALIFETIME_IN6: c_ulong = 0xC1206951;
const IN6_IFF_TENTATIVE: c_ushort = 0x0002;
const IN6_IFF_DUPLICATED: c_ushort = 0x0004;
const IN6_IFF_DETACHED: c_ushort = 0x0008;
const IN6_IFF_AUTOCONF: c_ushort = 0x0040;
const IN6_IFF_TEMPORARY: c_ushort = 0x0080;
const IN6_IFF_DEPRECATED: c_ushort = 0x0010;
const IN6_IFF_DYNAMIC: c_ushort = 0x0100;
const IN6_IFF_SECURED: c_ushort = 0x0400;
const RTAX_DST: c_int = 0;
const RTAX_GATEWAY: c_int = 1;
const RTAX_MAX: c_int = 15;
const RTA_DST: c_int = 1;
const RTA_GATEWAY: c_int = 2;
const RTF_GATEWAY: c_int = 2;

macro_rules! set_name {
    ($name_field:expr, $name_str:expr) => {{
        let name_c = &::std::ffi::CString::new($name_str.to_owned()).map_err(|_| {
            ::std::io::Error::new(
                ::std::io::ErrorKind::InvalidInput,
                "malformed interface name",
            )
        })?;
        let name_slice = name_c.as_bytes_with_nul();
        if name_slice.len() > IFNAMSIZ {
            return Err(io::Error::new(::std::io::ErrorKind::InvalidInput, ""));
        }
        $name_field[..name_slice.len()].clone_from_slice(name_slice);

        Ok(())
    }};
}

macro_rules! round_up {
    ($a:expr) => {
        if $a > 0 {
            1 + (($a - 1) | 3)
        } else {
            4
        }
    };
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
struct rt_msghdr {
    rtm_msglen: c_ushort,
    rtm_version: c_uchar,
    rtm_type: c_uchar,
    rtm_hdrlen: c_ushort,
    rtm_index: c_ushort,
    rtm_tableid: c_ushort,
    rtm_pri: c_uchar,
    rtm_mpls: c_uchar,
    rtm_addrs: c_int,
    rtm_flags: c_int,
    rtm_fmask: c_int,
    rtm_pid: pid_t,
    rtm_seq: c_int,
    rtm_errno: c_int,
    rtm_inits: u32,
    rtm_rmx: rt_metrics,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
struct rt_metrics {
    rmx_pksent: u64,
    rmx_expire: i64,
    rmx_locks: u32,
    rmx_mtu: u32,
    rmx_refcnt: u32,
    rmx_hopcount: i32,
    rmx_recvpipe: u32,
    rmx_sendpipe: u32,
    rmx_ssthresh: u32,
    rmx_rtt: u32,
    rmx_rttvar: u32,
    rmx_pad: u32,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
struct in6_addrlifetime {
    ia6t_expire: time_t,
    ia6t_preferred: time_t,
    ia6t_vltime: u32,
    ia6t_pltime: u32,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
struct in6_ifstat {
    ifs6_in_receive: u64,
    ifs6_in_hdrerr: u64,
    ifs6_in_toobig: u64,
    ifs6_in_noroute: u64,
    ifs6_in_addrerr: u64,
    ifs6_in_protounknown: u64,
    ifs6_in_truncated: u64,
    ifs6_in_discard: u64,
    ifs6_in_deliver: u64,
    ifs6_out_forward: u64,
    ifs6_out_request: u64,
    ifs6_out_discard: u64,
    ifs6_out_fragok: u64,
    ifs6_out_fragfail: u64,
    ifs6_out_fragcreat: u64,
    ifs6_reass_reqd: u64,
    ifs6_reass_ok: u64,
    ifs6_reass_fail: u64,
    ifs6_in_mcast: u64,
    ifs6_out_mcast: u64,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
struct icmp6_ifstat {
    ifs6_in_msg: u64,
    ifs6_in_error: u64,
    ifs6_in_dstunreach: u64,
    ifs6_in_adminprohib: u64,
    ifs6_in_timeexceed: u64,
    ifs6_in_paramprob: u64,
    ifs6_in_pkttoobig: u64,
    ifs6_in_echo: u64,
    ifs6_in_echoreply: u64,
    ifs6_in_routersolicit: u64,
    ifs6_in_routeradvert: u64,
    ifs6_in_neighborsolicit: u64,
    ifs6_in_neighboradvert: u64,
    ifs6_in_redirect: u64,
    ifs6_in_mldquery: u64,
    ifs6_in_mldreport: u64,
    ifs6_in_mlddone: u64,
    ifs6_out_msg: u64,
    ifs6_out_error: u64,
    ifs6_out_dstunreach: u64,
    ifs6_out_adminprohib: u64,
    ifs6_out_timeexceed: u64,
    ifs6_out_paramprob: u64,
    ifs6_out_pkttoobig: u64,
    ifs6_out_echo: u64,
    ifs6_out_echoreply: u64,
    ifs6_out_routersolicit: u64,
    ifs6_out_routeradvert: u64,
    ifs6_out_neighborsolicit: u64,
    ifs6_out_neighboradvert: u64,
    ifs6_out_redirect: u64,
    ifs6_out_mldquery: u64,
    ifs6_out_mldreport: u64,
    ifs6_out_mlddone: u64,
}

#[derive(Clone, Copy)]
#[repr(C)]
union IfrIfru {
    ifru_addr: sockaddr_in6,
    ifru_dstaddr: sockaddr_in6,
    ifru_flags: c_short,
    ifru_flags6: c_int,
    ifru_metric: c_int,
    ifru_data: *mut c_uchar, // caddr_t
    ifru_lifetime: in6_addrlifetime,
    ifru_stat: in6_ifstat,
    ifru_icmp6stat: icmp6_ifstat,
}

#[derive(Clone)]
#[repr(C)]
struct in6_ifreq {
    ifr_name: [c_uchar; IFNAMSIZ],
    ifr_ifru: IfrIfru,
}

impl in6_ifreq {
    pub fn from_name(name: &str) -> io::Result<Self> {
        let mut req: in6_ifreq = unsafe { mem::zeroed() };
        req.set_name(name)?;
        Ok(req)
    }
    pub fn set_name(&mut self, name: &str) -> io::Result<()> {
        set_name!(self.ifr_name, name)
    }
    pub fn set_addr(&mut self, addr: sockaddr_in6) {
        self.ifr_ifru.ifru_addr = addr;
    }
    pub fn get_flags6(&self) -> c_ushort {
        unsafe { self.ifr_ifru.ifru_flags6 as c_ushort }
    }
    pub fn get_ia6t_expire(&self) -> time_t {
        unsafe { self.ifr_ifru.ifru_lifetime.ia6t_expire as time_t }
    }
}

pub fn do_broadcast(ifaddr: &ifaddrs) -> Option<IpAddr> {
    sockaddr_tools::to_ipaddr(ifaddr.ifa_dstaddr)
}

///////////////////////////////////////////////////

pub struct IfAddrs {
    inner: *mut ifaddrs,
}

impl IfAddrs {
    pub fn new() -> io::Result<Self> {
        let mut ifaddrs = mem::MaybeUninit::uninit();

        unsafe {
            if -1 == getifaddrs(ifaddrs.as_mut_ptr()) {
                return Err(io::Error::last_os_error());
            }
            Ok(Self {
                inner: ifaddrs.assume_init(),
            })
        }
    }

    pub fn iter(&self) -> IfAddrsIterator {
        IfAddrsIterator { next: self.inner }
    }
}

impl Drop for IfAddrs {
    #[allow(unsafe_code)]
    fn drop(&mut self) {
        unsafe {
            freeifaddrs(self.inner);
        }
    }
}

pub struct IfAddrsIterator {
    next: *mut ifaddrs,
}

impl Iterator for IfAddrsIterator {
    type Item = ifaddrs;

    #[allow(unsafe_code)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.next.is_null() {
            return None;
        };

        Some(unsafe {
            let result = *self.next;
            self.next = (*self.next).ifa_next;

            result
        })
    }
}

///////////////////////////////////////////////////

pub struct PlatformSupportOpenBSD {
    default_route_interfaces: BTreeSet<u32>,
}

impl PlatformSupportOpenBSD {
    pub fn new() -> Self {
        PlatformSupportOpenBSD {
            default_route_interfaces: BTreeSet::new(),
        }
    }

    fn refresh_default_route_interfaces(&mut self) {
        self.default_route_interfaces.clear();

        let mut mib = [CTL_NET, PF_ROUTE, 0, 0, NET_RT_FLAGS, RTF_GATEWAY];
        let mut sa_tab: [*const sockaddr; RTAX_MAX as usize] =
            [std::ptr::null(); RTAX_MAX as usize];
        let mut rt_buf_len = 0usize;

        // Get memory size for mib result
        if unsafe {
            sysctl(
                mib.as_mut_ptr(),
                mib.len() as u32,
                std::ptr::null_mut(),
                &mut rt_buf_len as *mut usize,
                std::ptr::null_mut(),
                0,
            )
        } < 0
        {
            error!("Unable to get memory size for routing table");
            return;
        }

        // Allocate a buffer
        let mut rt_buf = vec![0u8; rt_buf_len];

        // Get mib result
        if unsafe {
            sysctl(
                mib.as_mut_ptr(),
                mib.len() as u32,
                rt_buf.as_mut_ptr() as *mut c_void,
                &mut rt_buf_len as *mut usize,
                std::ptr::null_mut(),
                0,
            )
        } < 0
        {
            error!("Unable to get memory size for routing table");
            return;
        }

        // Process each routing message
        let mut mib_ptr = rt_buf.as_ptr();
        let mib_end = unsafe { mib_ptr.add(rt_buf_len) };
        while mib_ptr < mib_end {
            let rt = mib_ptr as *const rt_msghdr;
            let mut sa = unsafe { rt.add(1) } as *const sockaddr;
            let rtm_addrs = unsafe { (*rt).rtm_addrs };
            let intf_index = unsafe { (*rt).rtm_index } as u32;

            // Fill in sockaddr table
            (0..(RTAX_MAX as usize)).for_each(|i| {
                if rtm_addrs & (1 << i) != 0 {
                    sa_tab[i] = sa;
                    sa = unsafe {
                        let sa_len = (*sa).sa_len;
                        sa = ((sa as *const u8).add(round_up!(sa_len as usize))) as *const sockaddr;
                        sa
                    };
                }
            });

            // Advance the pointer before potential continue on match fail below
            mib_ptr = unsafe { mib_ptr.add((*rt).rtm_msglen.into()) };

            // Look for gateways
            if rtm_addrs & (RTA_DST | RTA_GATEWAY) == (RTA_DST | RTA_GATEWAY) {
                // Only interested in AF_INET and AF_INET6 address families
                // SockAddr::new() takes care of this for us
                let saddr_dst = match SockAddr::new(sa_tab[RTAX_DST as usize]) {
                    Some(a) => a,
                    None => continue,
                };

                // we want to identify if gw addr is from a loopback if
                let saddr_src = match SockAddr::new(sa_tab[RTAX_GATEWAY as usize]) {
                    Some(a) => a,
                    None => continue,
                };

                let src_ipaddr = match saddr_src.as_ipaddr() {
                    Some(a) => a,
                    None => continue,
                };

                let _saddr_gateway = match SockAddr::new(sa_tab[RTAX_GATEWAY as usize]) {
                    Some(a) => a,
                    None => continue,
                };

                // Look for default gateways
                let dst_ipaddr = match saddr_dst.as_ipaddr() {
                    Some(a) => a,
                    None => continue,
                };

                if dst_ipaddr.is_unspecified() && !src_ipaddr.is_loopback() {
                    self.default_route_interfaces.insert(intf_index);
                }
            }
        }
    }

    fn get_interface_flags(&self, index: u32, flags: c_int) -> InterfaceFlags {
        InterfaceFlags {
            is_loopback: (flags & IFF_LOOPBACK) != 0,
            is_running: (flags & IFF_RUNNING) != 0,
            is_point_to_point: (flags & IFF_POINTOPOINT) != 0,
            has_default_route: self.default_route_interfaces.contains(&index),
        }
    }

    fn get_address_flags(ifname: &str, addr: sockaddr_in6) -> io::Result<AddressFlags> {
        let sock = unsafe { socket(AF_INET6, SOCK_DGRAM, 0) };
        if sock < 0 {
            return Err(io::Error::last_os_error());
        }

        let mut req = in6_ifreq::from_name(ifname).unwrap();
        req.set_addr(addr);

        let res = unsafe { ioctl(sock, SIOCGIFAFLAG_IN6, &mut req) };
        if res < 0 {
            unsafe { close(sock) };
            return Err(io::Error::last_os_error());
        }
        let flags = req.get_flags6();

        let mut req = in6_ifreq::from_name(ifname).unwrap();
        req.set_addr(addr);

        let res = unsafe { ioctl(sock, SIOCGIFALIFETIME_IN6, &mut req) };
        unsafe { close(sock) };
        if res < 0 {
            return Err(io::Error::last_os_error());
        }
        let expire = req.get_ia6t_expire();

        let is_auto_generated_random_address =
            flags & (IN6_IFF_SECURED | IN6_IFF_AUTOCONF) == (IN6_IFF_SECURED | IN6_IFF_AUTOCONF);

        let is_temporary =
            (flags & IN6_IFF_TEMPORARY) != 0 || (expire != 0 && is_auto_generated_random_address);
        let is_dynamic = (flags & (IN6_IFF_DYNAMIC | IN6_IFF_AUTOCONF)) != 0;
        let is_preferred = (flags
            & (IN6_IFF_TENTATIVE | IN6_IFF_DUPLICATED | IN6_IFF_DETACHED | IN6_IFF_DEPRECATED))
            == 0;

        Ok(AddressFlags {
            is_temporary,
            is_dynamic,
            is_preferred,
        })
    }

    #[expect(clippy::unused_async)]
    pub async fn get_interfaces(
        &mut self,
        interfaces: &mut BTreeMap<String, NetworkInterface>,
    ) -> io::Result<()> {
        self.refresh_default_route_interfaces();

        // Ask for all the addresses we have
        let ifaddrs = IfAddrs::new()?;
        for ifaddr in ifaddrs.iter() {
            // Get the interface name
            let ifname = unsafe { CStr::from_ptr(ifaddr.ifa_name) }
                .to_string_lossy()
                .into_owned();

            // Get the interface index
            let ifindex = unsafe { if_nametoindex(ifaddr.ifa_name) };

            // Map the name to a NetworkInterface
            if !interfaces.contains_key(&ifname) {
                // If we have no NetworkInterface yet, make one
                let flags = self.get_interface_flags(ifindex, ifaddr.ifa_flags as c_int);
                interfaces.insert(ifname.clone(), NetworkInterface::new(ifname.clone(), flags));
            }
            let intf = interfaces.get_mut(&ifname).unwrap();

            let mut address_flags = AddressFlags::default();

            let intf_addr = match sockaddr_tools::to_ipaddr(ifaddr.ifa_addr) {
                None => continue,
                Some(IpAddr::V4(ipv4_addr)) => {
                    let netmask = match sockaddr_tools::to_ipaddr(ifaddr.ifa_netmask) {
                        Some(IpAddr::V4(netmask)) => netmask,
                        _ => Ipv4Addr::new(0, 0, 0, 0),
                    };
                    let broadcast = if (ifaddr.ifa_flags & (IFF_BROADCAST as u32)) != 0 {
                        match do_broadcast(&ifaddr) {
                            Some(IpAddr::V4(broadcast)) => Some(broadcast),
                            _ => None,
                        }
                    } else {
                        None
                    };

                    IfAddr::V4(Ifv4Addr {
                        ip: ipv4_addr,
                        netmask,
                        broadcast,
                    })
                }
                Some(IpAddr::V6(ipv6_addr)) => {
                    let netmask = match sockaddr_tools::to_ipaddr(ifaddr.ifa_netmask) {
                        Some(IpAddr::V6(netmask)) => netmask,
                        _ => Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0),
                    };

                    // Get address flags for ipv6
                    address_flags = match Self::get_address_flags(
                        &ifname,
                        SockAddr::new(ifaddr.ifa_addr).unwrap().sa_in6(),
                    ) {
                        Ok(v) => v,
                        Err(e) => {
                            // debug!("failed to get address flags for ifname={}, ifaddr={:?} : {}", ifname, ifaddr.ifa_addr, e);
                            continue;
                        }
                    };

                    IfAddr::V6(Ifv6Addr {
                        ip: ipv6_addr,
                        netmask,
                        broadcast: None,
                    })
                }
            };

            // Add to the list
            intf.addrs
                .push(InterfaceAddress::new(intf_addr, address_flags));
        }

        Ok(())
    }
}
