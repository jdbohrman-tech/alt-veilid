use super::*;

use alloc::collections::btree_map::Entry;
use futures_util::stream::TryStreamExt;
use ifstructs::ifreq;
use libc::{
    close, if_indextoname, ioctl, socket, IFF_LOOPBACK, IFF_POINTOPOINT, IFF_RUNNING, IF_NAMESIZE,
    SIOCGIFFLAGS, SOCK_DGRAM,
};
use netlink_packet_route::{
    nlas::address::Nla, AddressMessage, AF_INET, AF_INET6, IFA_F_DADFAILED, IFA_F_DEPRECATED,
    IFA_F_OPTIMISTIC, IFA_F_PERMANENT, IFA_F_TEMPORARY, IFA_F_TENTATIVE,
};
use rtnetlink::{new_connection_with_socket, Handle, IpVersion};
cfg_if! {
    if #[cfg(feature="rt-async-std")] {
        use netlink_sys::{SmolSocket as RTNetLinkSocket};
    } else if #[cfg(feature="rt-tokio")] {
        use netlink_sys::{TokioSocket as RTNetLinkSocket};
    } else {
        compile_error!("needs executor implementation")
    }
}
use std::convert::TryInto;
use std::ffi::CStr;
use std::io;
use std::os::raw::c_int;
use tools::*;

fn get_interface_name(index: u32) -> EyreResult<String> {
    let mut ifnamebuf = [0u8; (IF_NAMESIZE + 1)];
    cfg_if! {
        if #[cfg(all(any(target_os = "android", target_os="linux"), any(target_arch = "arm", target_arch = "aarch64")))] {
            if unsafe { if_indextoname(index, ifnamebuf.as_mut_ptr()) }.is_null() {
                bail!("if_indextoname returned null");
            }
        } else {
            if unsafe { if_indextoname(index, ifnamebuf.as_mut_ptr() as *mut i8) }.is_null() {
                bail!("if_indextoname returned null");
            }
        }
    }

    let ifnamebuflen = ifnamebuf
        .iter()
        .position(|c| *c == 0u8)
        .ok_or_else(|| eyre!("null not found in interface name"))?;
    let ifname_str = CStr::from_bytes_with_nul(&ifnamebuf[0..=ifnamebuflen])
        .wrap_err("failed to convert interface name")?
        .to_str()
        .wrap_err("invalid characters in interface name")?;
    Ok(ifname_str.to_owned())
}

fn flags_to_address_flags(flags: u32) -> AddressFlags {
    AddressFlags {
        is_temporary: (flags & IFA_F_TEMPORARY) != 0,
        is_dynamic: (flags & IFA_F_PERMANENT) == 0,
        is_preferred: (flags
            & (IFA_F_TENTATIVE | IFA_F_DADFAILED | IFA_F_DEPRECATED | IFA_F_OPTIMISTIC))
            == 0,
    }
}

pub struct PlatformSupportNetlink {
    connection_jh: Option<MustJoinHandle<()>>,
    handle: Option<Handle>,
    default_route_interfaces: BTreeSet<u32>,
}

impl PlatformSupportNetlink {
    pub fn new() -> EyreResult<Self> {
        Ok(PlatformSupportNetlink {
            connection_jh: None,
            handle: None,
            default_route_interfaces: BTreeSet::new(),
        })
    }

    // Figure out which interfaces have default routes
    async fn refresh_default_route_interfaces(&mut self) -> EyreResult<()> {
        self.default_route_interfaces.clear();
        let mut routesv4 = self
            .handle
            .as_ref()
            .unwrap()
            .route()
            .get(IpVersion::V4)
            .execute();
        while let Some(routev4) = routesv4.try_next().await.unwrap_or_default() {
            if let Some(index) = routev4.output_interface() {
                //println!("*** ipv4 route: {:#?}", routev4);
                if routev4.header.destination_prefix_length == 0 {
                    self.default_route_interfaces.insert(index);
                }
            }
        }
        let mut routesv6 = self
            .handle
            .as_ref()
            .unwrap()
            .route()
            .get(IpVersion::V6)
            .execute();
        while let Some(routev6) = routesv6.try_next().await.unwrap_or_default() {
            if let Some(index) = routev6.output_interface() {
                //println!("*** ipv6 route: {:#?}", routev6);
                if routev6.header.destination_prefix_length == 0 {
                    self.default_route_interfaces.insert(index);
                }
            }
        }
        Ok(())
    }

    fn get_interface_flags(&self, index: u32, ifname: &str) -> EyreResult<InterfaceFlags> {
        let mut req = ifreq::from_name(ifname).wrap_err("failed to convert interface name")?;

        let sock = unsafe { socket(AF_INET as i32, SOCK_DGRAM, 0) };
        if sock < 0 {
            return Err(io::Error::last_os_error()).wrap_err("failed to create socket");
        }

        cfg_if! {
            if #[cfg(any(target_os = "android", target_env = "musl"))] {
                let res = unsafe { ioctl(sock, SIOCGIFFLAGS as i32, &mut req) };
            } else {
                let res = unsafe { ioctl(sock, SIOCGIFFLAGS, &mut req) };
            }
        }
        unsafe { close(sock) };
        if res < 0 {
            return Err(io::Error::last_os_error()).wrap_err("failed to close socket");
        }

        let flags = req.get_flags() as c_int;

        Ok(InterfaceFlags {
            is_loopback: (flags & IFF_LOOPBACK) != 0,
            is_running: (flags & IFF_RUNNING) != 0,
            is_point_to_point: (flags & IFF_POINTOPOINT) != 0,
            has_default_route: self.default_route_interfaces.contains(&index),
        })
    }

    fn process_address_message_v4(msg: AddressMessage) -> Option<InterfaceAddress> {
        // Get ip address
        let ip = msg.nlas.iter().find_map(|nla| {
            if let Nla::Address(a) = nla {
                let a: Option<[u8; 4]> = a.clone().try_into().ok();
                a.map(Ipv4Addr::from)
            } else {
                None
            }
        })?;

        // get netmask
        let plen = msg.header.prefix_len as i16;
        let mut netmask = [0u8; 4];
        get_netmask_from_prefix_length_v4(&mut netmask, plen);
        let netmask = Ipv4Addr::from(netmask);

        // get broadcast address
        let broadcast = msg.nlas.iter().find_map(|nla| {
            if let Nla::Broadcast(b) = nla {
                let b: Option<[u8; 4]> = b.clone().try_into().ok();
                b.map(Ipv4Addr::from)
            } else {
                None
            }
        });

        // get address flags
        let flags = msg
            .nlas
            .iter()
            .find_map(|nla| {
                if let Nla::Flags(f) = nla {
                    Some(*f)
                } else {
                    None
                }
            })
            .unwrap_or(msg.header.flags as u32);

        Some(InterfaceAddress::new(
            IfAddr::V4(Ifv4Addr {
                ip,
                /// The netmask of the interface.
                netmask,
                /// The broadcast address of the interface.
                broadcast,
            }),
            flags_to_address_flags(flags),
        ))
    }

    fn process_address_message_v6(msg: AddressMessage) -> Option<InterfaceAddress> {
        // Get ip address
        let ip = msg.nlas.iter().find_map(|nla| {
            if let Nla::Address(a) = nla {
                let a: Option<[u8; 16]> = a.clone().try_into().ok();
                a.map(Ipv6Addr::from)
            } else {
                None
            }
        })?;

        // get netmask
        let plen = msg.header.prefix_len as i16;
        let mut netmask = [0u8; 16];
        get_netmask_from_prefix_length_v6(&mut netmask, plen);
        let netmask = Ipv6Addr::from(netmask);

        // get address flags
        let flags = msg
            .nlas
            .iter()
            .find_map(|nla| {
                if let Nla::Flags(f) = nla {
                    Some(*f)
                } else {
                    None
                }
            })
            .unwrap_or(msg.header.flags as u32);

        // Skip addresses going through duplicate address detection, or ones that have failed it
        if ((flags & IFA_F_TENTATIVE) != 0) || ((flags & IFA_F_DADFAILED) != 0) {
            return None;
        }

        Some(InterfaceAddress::new(
            IfAddr::V6(Ifv6Addr {
                ip,
                /// The netmask of the interface.
                netmask,
                /// The broadcast address of the interface.
                broadcast: None,
            }),
            flags_to_address_flags(flags),
        ))
    }

    async fn get_interfaces_internal(
        &mut self,
        interfaces: &mut BTreeMap<String, NetworkInterface>,
    ) -> EyreResult<()> {
        // Refresh the routes
        self.refresh_default_route_interfaces().await?;

        // Ask for all the addresses we have
        let mut names = BTreeMap::<u32, String>::new();
        let mut addresses = self.handle.as_ref().unwrap().address().get().execute();
        while let Some(msg) = addresses
            .try_next()
            .await
            .wrap_err("failed to iterate interface addresses")?
        {
            // Have we seen this interface index yet?
            // Get the name from the index, cached, if we can
            let ifname = match names.entry(msg.header.index) {
                Entry::Vacant(v) => {
                    // If not, get the name for the index if we can
                    let ifname = match get_interface_name(msg.header.index) {
                        Ok(v) => v,
                        Err(e) => {
                            log_net!(warn
                                "couldn't get interface name for index {}: {}",
                                msg.header.index,
                                e
                            );
                            continue;
                        }
                    };
                    v.insert(ifname).clone()
                }
                Entry::Occupied(o) => o.get().clone(),
            };

            // Map the name to a NetworkInterface
            if !interfaces.contains_key(&ifname) {
                // If we have no NetworkInterface yet, make one
                let flags = self.get_interface_flags(msg.header.index, &ifname)?;
                interfaces.insert(ifname.clone(), NetworkInterface::new(ifname.clone(), flags));
            }
            let intf = interfaces.get_mut(&ifname).unwrap();

            // Process the address
            let intf_addr = match msg.header.family as u16 {
                AF_INET => match Self::process_address_message_v4(msg) {
                    Some(ia) => ia,
                    None => {
                        continue;
                    }
                },
                AF_INET6 => match Self::process_address_message_v6(msg) {
                    Some(ia) => ia,
                    None => {
                        continue;
                    }
                },
                _ => {
                    continue;
                }
            };

            intf.addrs.push(intf_addr);
        }

        Ok(())
    }

    pub async fn get_interfaces(
        &mut self,
        interfaces: &mut BTreeMap<String, NetworkInterface>,
    ) -> EyreResult<()> {
        // Get the netlink connection
        let (connection, handle, _) = new_connection_with_socket::<RTNetLinkSocket>()
            .wrap_err("failed to create rtnetlink socket")?;

        // Spawn a connection handler
        let connection_jh = spawn(connection);

        // Save the connection
        self.connection_jh = Some(connection_jh);
        self.handle = Some(handle);

        // Do the work
        let out = self.get_interfaces_internal(interfaces).await;

        // Clean up connection
        drop(self.handle.take());
        self.connection_jh.take().unwrap().abort().await;

        out
    }
}
