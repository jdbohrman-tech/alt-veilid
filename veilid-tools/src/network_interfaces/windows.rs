#![cfg(target_os = "windows")]

// Copyright 2018 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under the MIT license <LICENSE-MIT
// http://opensource.org/licenses/MIT> or the Modified BSD license <LICENSE-BSD
// https://opensource.org/licenses/BSD-3-Clause>, at your option. This file may not be copied,
// modified, or distributed except according to those terms. Please review the Licences for the
// specific language governing permissions and limitations relating to use of the SAFE Network
// Software.

use super::*;

use libc::{self, c_ulong, c_void, size_t};
use std::ffi::CStr;
use std::{io, ptr};
use winapi::shared::ifdef::IfOperStatusUp;
use winapi::shared::ipifcons::{IF_TYPE_SOFTWARE_LOOPBACK, IF_TYPE_TUNNEL};
use winapi::shared::nldef::{
    IpDadStatePreferred, IpPrefixOriginDhcp, IpSuffixOriginDhcp, IpSuffixOriginRandom,
};
use winapi::shared::winerror::{ERROR_BUFFER_OVERFLOW, ERROR_SUCCESS};
use winapi::um::iphlpapi::GetAdaptersAddresses;
use winapi::um::iptypes::{
    GAA_FLAG_INCLUDE_GATEWAYS, GAA_FLAG_INCLUDE_PREFIX, GAA_FLAG_SKIP_ANYCAST,
    GAA_FLAG_SKIP_DNS_SERVER, GAA_FLAG_SKIP_FRIENDLY_NAME, GAA_FLAG_SKIP_MULTICAST,
    IP_ADAPTER_ADDRESSES, IP_ADAPTER_PREFIX, IP_ADAPTER_UNICAST_ADDRESS,
};

pub struct PlatformSupportWindows {}

impl PlatformSupportWindows {
    pub fn new() -> Self {
        PlatformSupportWindows {}
    }

    fn get_interface_flags(intf: &IpAdapterAddresses) -> InterfaceFlags {
        InterfaceFlags {
            is_loopback: intf.get_flag_loopback(),
            is_running: intf.get_flag_running(),
            is_point_to_point: intf.get_flag_point_to_point(),
            has_default_route: intf.get_has_default_route(),
        }
    }

    fn get_address_flags(addr: *const IP_ADAPTER_UNICAST_ADDRESS) -> AddressFlags {
        let ds = unsafe { (*addr).DadState };
        let po = unsafe { (*addr).PrefixOrigin };
        let so = unsafe { (*addr).SuffixOrigin };
        AddressFlags {
            is_temporary: so == IpSuffixOriginRandom,
            is_dynamic: po == IpPrefixOriginDhcp || so == IpSuffixOriginDhcp,
            is_preferred: ds == IpDadStatePreferred,
        }
    }

    pub async fn get_interfaces(
        &mut self,
        interfaces: &mut BTreeMap<String, NetworkInterface>,
    ) -> io::Result<()> {
        // Iterate all the interfaces
        let windows_interfaces = WindowsInterfaces::new()?;
        for windows_interface in windows_interfaces.iter() {
            // Get name
            let intf_name = windows_interface.name();

            // Get flags
            let flags = Self::get_interface_flags(&windows_interface);

            let mut network_interface = NetworkInterface::new(intf_name.clone(), flags);

            // Go through all addresses and add them if appropriate
            for addr in windows_interface.unicast_addresses() {
                let intf_addr = match sockaddr_tools::to_ipaddr(addr.Address.lpSockaddr) {
                    None => continue,
                    Some(IpAddr::V4(ipv4_addr)) => {
                        let mut item_netmask = Ipv4Addr::new(0, 0, 0, 0);
                        let mut item_broadcast = None;

                        // Search prefixes for a prefix matching addr
                        'prefixloopv4: for prefix in windows_interface.prefixes() {
                            let ipprefix = sockaddr_tools::to_ipaddr(prefix.Address.lpSockaddr);
                            match ipprefix {
                                Some(IpAddr::V4(ref a)) => {
                                    let mut netmask: [u8; 4] = [0; 4];
                                    for (n, netmask_elt) in netmask
                                        .iter_mut()
                                        .enumerate()
                                        .take((prefix.PrefixLength as usize + 7) / 8)
                                    {
                                        let x_byte = ipv4_addr.octets()[n];
                                        let y_byte = a.octets()[n];
                                        for m in 0..8 {
                                            if (n * 8) + m > prefix.PrefixLength as usize {
                                                break;
                                            }
                                            let bit = 1 << m;
                                            if (x_byte & bit) == (y_byte & bit) {
                                                *netmask_elt |= bit;
                                            } else {
                                                continue 'prefixloopv4;
                                            }
                                        }
                                    }
                                    item_netmask = Ipv4Addr::new(
                                        netmask[0], netmask[1], netmask[2], netmask[3],
                                    );
                                    let mut broadcast: [u8; 4] = ipv4_addr.octets();
                                    for n in 0..4 {
                                        broadcast[n] |= !netmask[n];
                                    }
                                    item_broadcast = Some(Ipv4Addr::new(
                                        broadcast[0],
                                        broadcast[1],
                                        broadcast[2],
                                        broadcast[3],
                                    ));
                                    break 'prefixloopv4;
                                }
                                _ => continue,
                            };
                        }
                        IfAddr::V4(Ifv4Addr {
                            ip: ipv4_addr,
                            netmask: item_netmask,
                            broadcast: item_broadcast,
                        })
                    }
                    Some(IpAddr::V6(ipv6_addr)) => {
                        let mut item_netmask = Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0);
                        // Search prefixes for a prefix matching addr
                        'prefixloopv6: for prefix in windows_interface.prefixes() {
                            let ipprefix = sockaddr_tools::to_ipaddr(prefix.Address.lpSockaddr);
                            match ipprefix {
                                Some(IpAddr::V6(ref a)) => {
                                    // Iterate the bits in the prefix, if they all match this prefix
                                    // is the right one, else try the next prefix
                                    let mut netmask: [u16; 8] = [0; 8];
                                    for (n, netmask_elt) in netmask
                                        .iter_mut()
                                        .enumerate()
                                        .take((prefix.PrefixLength as usize + 15) / 16)
                                    {
                                        let x_word = ipv6_addr.segments()[n];
                                        let y_word = a.segments()[n];
                                        for m in 0..16 {
                                            if (n * 16) + m > prefix.PrefixLength as usize {
                                                break;
                                            }
                                            let bit = 1 << m;
                                            if (x_word & bit) == (y_word & bit) {
                                                *netmask_elt |= bit;
                                            } else {
                                                continue 'prefixloopv6;
                                            }
                                        }
                                    }
                                    item_netmask = Ipv6Addr::new(
                                        netmask[0], netmask[1], netmask[2], netmask[3], netmask[4],
                                        netmask[5], netmask[6], netmask[7],
                                    );
                                    break 'prefixloopv6;
                                }
                                _ => continue,
                            };
                        }
                        IfAddr::V6(Ifv6Addr {
                            ip: ipv6_addr,
                            netmask: item_netmask,
                            broadcast: None,
                        })
                    }
                };

                let address_flags = Self::get_address_flags(addr);

                network_interface
                    .addrs
                    .push(InterfaceAddress::new(intf_addr, address_flags))
            }

            interfaces.insert(intf_name, network_interface);
        }

        Ok(())
    }
}

#[repr(C)]
pub struct IpAdapterAddresses {
    data: *const IP_ADAPTER_ADDRESSES,
}

impl IpAdapterAddresses {
    #[allow(unsafe_code)]
    pub fn name(&self) -> String {
        unsafe { CStr::from_ptr((*self.data).AdapterName) }
            .to_string_lossy()
            .into_owned()
    }

    pub fn prefixes(&self) -> PrefixesIterator {
        PrefixesIterator {
            _phantom: std::marker::PhantomData {},
            next: unsafe { (*self.data).FirstPrefix },
        }
    }

    pub fn unicast_addresses(&self) -> UnicastAddressesIterator {
        UnicastAddressesIterator {
            _phantom: std::marker::PhantomData {},
            next: unsafe { (*self.data).FirstUnicastAddress },
        }
    }

    pub fn get_flag_loopback(&self) -> bool {
        unsafe { (*self.data).IfType == IF_TYPE_SOFTWARE_LOOPBACK }
    }
    pub fn get_flag_running(&self) -> bool {
        unsafe { (*self.data).OperStatus == IfOperStatusUp }
    }
    pub fn get_flag_point_to_point(&self) -> bool {
        unsafe { (*self.data).IfType == IF_TYPE_TUNNEL }
    }
    pub fn get_has_default_route(&self) -> bool {
        unsafe { !(*self.data).FirstGatewayAddress.is_null() }
    }
}

struct WindowsInterfaces {
    data: *const IP_ADAPTER_ADDRESSES,
}

impl WindowsInterfaces {
    pub fn new() -> io::Result<Self> {
        let mut buffersize: c_ulong = 16384;
        let mut ifaddrs: *mut IP_ADAPTER_ADDRESSES;

        loop {
            unsafe {
                ifaddrs = libc::malloc(buffersize as size_t) as *mut IP_ADAPTER_ADDRESSES;
                if ifaddrs.is_null() {
                    panic!("Failed to allocate buffer in get_if_addrs()");
                }

                let retcode = GetAdaptersAddresses(
                    0,
                    GAA_FLAG_SKIP_ANYCAST
                        | GAA_FLAG_SKIP_MULTICAST
                        | GAA_FLAG_SKIP_DNS_SERVER
                        | GAA_FLAG_INCLUDE_PREFIX
                        | GAA_FLAG_SKIP_FRIENDLY_NAME
                        | GAA_FLAG_INCLUDE_GATEWAYS,
                    ptr::null_mut(),
                    ifaddrs,
                    &mut buffersize,
                );

                match retcode {
                    ERROR_SUCCESS => break,
                    ERROR_BUFFER_OVERFLOW => {
                        libc::free(ifaddrs as *mut c_void);
                        buffersize *= 2;
                        continue;
                    }
                    _ => return Err(io::Error::last_os_error()),
                }
            }
        }

        Ok(Self { data: ifaddrs })
    }

    pub fn iter(&self) -> WindowsInterfacesIterator<'_> {
        WindowsInterfacesIterator {
            next: self.data,
            _phantom: std::marker::PhantomData {},
        }
    }
}

impl Drop for WindowsInterfaces {
    fn drop(&mut self) {
        unsafe {
            libc::free(self.data as *mut c_void);
        }
    }
}

pub struct WindowsInterfacesIterator<'a> {
    next: *const IP_ADAPTER_ADDRESSES,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> Iterator for WindowsInterfacesIterator<'a> {
    type Item = IpAdapterAddresses;

    #[allow(unsafe_code)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.next.is_null() {
            return None;
        };

        Some(unsafe {
            let result = &*self.next;
            self.next = (*self.next).Next;

            IpAdapterAddresses { data: result }
        })
    }
}

pub struct PrefixesIterator<'a> {
    _phantom: std::marker::PhantomData<&'a u8>,
    next: *const IP_ADAPTER_PREFIX,
}

impl<'a> Iterator for PrefixesIterator<'a> {
    type Item = &'a IP_ADAPTER_PREFIX;

    #[allow(unsafe_code)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.next.is_null() {
            return None;
        };

        Some(unsafe {
            let result = &*self.next;
            self.next = (*self.next).Next;

            result
        })
    }
}

pub struct UnicastAddressesIterator<'a> {
    _phantom: std::marker::PhantomData<&'a u8>,
    next: *const IP_ADAPTER_UNICAST_ADDRESS,
}

impl<'a> Iterator for UnicastAddressesIterator<'a> {
    type Item = &'a IP_ADAPTER_UNICAST_ADDRESS;

    #[allow(unsafe_code)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.next.is_null() {
            return None;
        };

        Some(unsafe {
            let result = &*self.next;
            self.next = (*self.next).Next;

            result
        })
    }
}
