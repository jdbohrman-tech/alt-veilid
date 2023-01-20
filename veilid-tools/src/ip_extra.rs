//
// This file really shouldn't be necessary, but 'ip' isn't a stable feature
//

use super::*;

use core::hash::*;

#[derive(Copy, PartialEq, Eq, Clone, Hash, Debug)]
pub enum Ipv6MulticastScope {
    InterfaceLocal,
    LinkLocal,
    RealmLocal,
    AdminLocal,
    SiteLocal,
    OrganizationLocal,
    Global,
}

pub fn ipaddr_is_unspecified(addr: &IpAddr) -> bool {
    match addr {
        IpAddr::V4(ip) => ipv4addr_is_unspecified(ip),
        IpAddr::V6(ip) => ipv6addr_is_unspecified(ip),
    }
}

pub fn ipaddr_is_loopback(addr: &IpAddr) -> bool {
    match addr {
        IpAddr::V4(ip) => ipv4addr_is_loopback(ip),
        IpAddr::V6(ip) => ipv6addr_is_loopback(ip),
    }
}

pub fn ipaddr_is_global(addr: &IpAddr) -> bool {
    match addr {
        IpAddr::V4(ip) => ipv4addr_is_global(ip),
        IpAddr::V6(ip) => ipv6addr_is_global(ip),
    }
}

pub fn ipaddr_is_multicast(addr: &IpAddr) -> bool {
    match addr {
        IpAddr::V4(ip) => ipv4addr_is_multicast(ip),
        IpAddr::V6(ip) => ipv6addr_is_multicast(ip),
    }
}

pub fn ipaddr_is_documentation(addr: &IpAddr) -> bool {
    match addr {
        IpAddr::V4(ip) => ipv4addr_is_documentation(ip),
        IpAddr::V6(ip) => ipv6addr_is_documentation(ip),
    }
}

pub fn ipv4addr_is_unspecified(addr: &Ipv4Addr) -> bool {
    addr.octets() == [0u8, 0u8, 0u8, 0u8]
}

pub fn ipv4addr_is_loopback(addr: &Ipv4Addr) -> bool {
    addr.octets()[0] == 127
}

pub fn ipv4addr_is_private(addr: &Ipv4Addr) -> bool {
    match addr.octets() {
        [10, ..] => true,
        [172, b, ..] if (16..=31).contains(&b) => true,
        [192, 168, ..] => true,
        _ => false,
    }
}

pub fn ipv4addr_is_link_local(addr: &Ipv4Addr) -> bool {
    matches!(addr.octets(), [169, 254, ..])
}

pub fn ipv4addr_is_global(addr: &Ipv4Addr) -> bool {
    // check if this address is 192.0.0.9 or 192.0.0.10. These addresses are the only two
    // globally routable addresses in the 192.0.0.0/24 range.
    if u32::from(*addr) == 0xc0000009 || u32::from(*addr) == 0xc000000a {
        return true;
    }
    !ipv4addr_is_private(addr)
        && !ipv4addr_is_loopback(addr)
        && !ipv4addr_is_link_local(addr)
        && !ipv4addr_is_broadcast(addr)
        && !ipv4addr_is_documentation(addr)
        && !ipv4addr_is_shared(addr)
        && !ipv4addr_is_ietf_protocol_assignment(addr)
        && !ipv4addr_is_reserved(addr)
        && !ipv4addr_is_benchmarking(addr)
        // Make sure the address is not in 0.0.0.0/8
        && addr.octets()[0] != 0
}

pub fn ipv4addr_is_shared(addr: &Ipv4Addr) -> bool {
    addr.octets()[0] == 100 && (addr.octets()[1] & 0b1100_0000 == 0b0100_0000)
}

pub fn ipv4addr_is_ietf_protocol_assignment(addr: &Ipv4Addr) -> bool {
    addr.octets()[0] == 192 && addr.octets()[1] == 0 && addr.octets()[2] == 0
}

pub fn ipv4addr_is_benchmarking(addr: &Ipv4Addr) -> bool {
    addr.octets()[0] == 198 && (addr.octets()[1] & 0xfe) == 18
}

pub fn ipv4addr_is_reserved(addr: &Ipv4Addr) -> bool {
    addr.octets()[0] & 240 == 240 && !addr.is_broadcast()
}

pub fn ipv4addr_is_multicast(addr: &Ipv4Addr) -> bool {
    addr.octets()[0] >= 224 && addr.octets()[0] <= 239
}

pub fn ipv4addr_is_broadcast(addr: &Ipv4Addr) -> bool {
    addr.octets() == [255u8, 255u8, 255u8, 255u8]
}

pub fn ipv4addr_is_documentation(addr: &Ipv4Addr) -> bool {
    matches!(
        addr.octets(),
        [192, 0, 2, _] | [198, 51, 100, _] | [203, 0, 113, _]
    )
}

pub fn ipv6addr_is_unspecified(addr: &Ipv6Addr) -> bool {
    addr.segments() == [0, 0, 0, 0, 0, 0, 0, 0]
}

pub fn ipv6addr_is_loopback(addr: &Ipv6Addr) -> bool {
    addr.segments() == [0, 0, 0, 0, 0, 0, 0, 1]
}

pub fn ipv6addr_is_global(addr: &Ipv6Addr) -> bool {
    match ipv6addr_multicast_scope(addr) {
        Some(Ipv6MulticastScope::Global) => true,
        None => ipv6addr_is_unicast_global(addr),
        _ => false,
    }
}

pub fn ipv6addr_is_unique_local(addr: &Ipv6Addr) -> bool {
    (addr.segments()[0] & 0xfe00) == 0xfc00
}

pub fn ipv6addr_is_unicast_link_local_strict(addr: &Ipv6Addr) -> bool {
    addr.segments()[0] == 0xfe80
        && addr.segments()[1] == 0
        && addr.segments()[2] == 0
        && addr.segments()[3] == 0
}

pub fn ipv6addr_is_unicast_link_local(addr: &Ipv6Addr) -> bool {
    (addr.segments()[0] & 0xffc0) == 0xfe80
}

pub fn ipv6addr_is_unicast_site_local(addr: &Ipv6Addr) -> bool {
    (addr.segments()[0] & 0xffc0) == 0xfec0
}

pub fn ipv6addr_is_documentation(addr: &Ipv6Addr) -> bool {
    (addr.segments()[0] == 0x2001) && (addr.segments()[1] == 0xdb8)
}

pub fn ipv6addr_is_unicast_global(addr: &Ipv6Addr) -> bool {
    !ipv6addr_is_multicast(addr)
        && !ipv6addr_is_loopback(addr)
        && !ipv6addr_is_unicast_link_local(addr)
        && !ipv6addr_is_unique_local(addr)
        && !ipv6addr_is_unspecified(addr)
        && !ipv6addr_is_documentation(addr)
}

pub fn ipv6addr_multicast_scope(addr: &Ipv6Addr) -> Option<Ipv6MulticastScope> {
    if ipv6addr_is_multicast(addr) {
        match addr.segments()[0] & 0x000f {
            1 => Some(Ipv6MulticastScope::InterfaceLocal),
            2 => Some(Ipv6MulticastScope::LinkLocal),
            3 => Some(Ipv6MulticastScope::RealmLocal),
            4 => Some(Ipv6MulticastScope::AdminLocal),
            5 => Some(Ipv6MulticastScope::SiteLocal),
            8 => Some(Ipv6MulticastScope::OrganizationLocal),
            14 => Some(Ipv6MulticastScope::Global),
            _ => None,
        }
    } else {
        None
    }
}

pub fn ipv6addr_is_multicast(addr: &Ipv6Addr) -> bool {
    (addr.segments()[0] & 0xff00) == 0xff00
}

// Converts an ip to a ip block by applying a netmask
// to the host part of the ip address
// ipv4 addresses are treated as single hosts
// ipv6 addresses are treated as prefix allocated blocks
pub fn ip_to_ipblock(ip6_prefix_size: usize, addr: IpAddr) -> IpAddr {
    match addr {
        IpAddr::V4(_) => addr,
        IpAddr::V6(v6) => {
            let mut hostlen = 128usize.saturating_sub(ip6_prefix_size);
            let mut out = v6.octets();
            for i in (0..16).rev() {
                if hostlen >= 8 {
                    out[i] = 0xFF;
                    hostlen -= 8;
                } else {
                    out[i] |= !(0xFFu8 << hostlen);
                    break;
                }
            }
            IpAddr::V6(Ipv6Addr::from(out))
        }
    }
}

pub fn ipaddr_apply_netmask(addr: IpAddr, netmask: IpAddr) -> IpAddr {
    match addr {
        IpAddr::V4(v4) => {
            let v4mask = match netmask {
                IpAddr::V4(v4mask) => v4mask,
                IpAddr::V6(_) => {
                    panic!("netmask doesn't match ipv4 address");
                }
            };
            let v4 = v4.octets();
            let v4mask = v4mask.octets();
            IpAddr::V4(Ipv4Addr::new(
                v4[0] & v4mask[0],
                v4[1] & v4mask[1],
                v4[2] & v4mask[2],
                v4[3] & v4mask[3],
            ))
        }
        IpAddr::V6(v6) => {
            let v6mask = match netmask {
                IpAddr::V4(_) => {
                    panic!("netmask doesn't match ipv6 address");
                }
                IpAddr::V6(v6mask) => v6mask,
            };
            let v6 = v6.segments();
            let v6mask = v6mask.segments();
            IpAddr::V6(Ipv6Addr::new(
                v6[0] & v6mask[0],
                v6[1] & v6mask[1],
                v6[2] & v6mask[2],
                v6[3] & v6mask[3],
                v6[4] & v6mask[4],
                v6[5] & v6mask[5],
                v6[6] & v6mask[6],
                v6[7] & v6mask[7],
            ))
        }
    }
}

pub fn ipaddr_in_network(addr: IpAddr, netaddr: IpAddr, netmask: IpAddr) -> bool {
    if addr.is_ipv4() && !netaddr.is_ipv4() {
        return false;
    }
    if addr.is_ipv6() && !netaddr.is_ipv6() {
        return false;
    }
    ipaddr_apply_netmask(netaddr, netmask) == ipaddr_apply_netmask(addr, netmask)
}
