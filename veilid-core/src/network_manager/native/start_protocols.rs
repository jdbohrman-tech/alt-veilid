use super::*;
use lazy_static::*;

lazy_static! {
    static ref BAD_PORTS: BTreeSet<u16> = BTreeSet::from([
        1,    // tcpmux
        7,    // echo
        9,    // discard
        11,   // systat
        13,   // daytime
        15,   // netstat
        17,   // qotd
        19,   // chargen
        20,   // ftp data
        21,   // ftp access
        22,   // ssh
        23,   // telnet
        25,   // smtp
        37,   // time
        42,   // name
        43,   // nicname
        53,   // domain
        77,   // priv-rjs
        79,   // finger
        87,   // ttylink
        95,   // supdup
        101,  // hostriame
        102,  // iso-tsap
        103,  // gppitnp
        104,  // acr-nema
        109,  // pop2
        110,  // pop3
        111,  // sunrpc
        113,  // auth
        115,  // sftp
        117,  // uucp-path
        119,  // nntp
        123,  // NTP
        135,  // loc-srv /epmap
        139,  // netbios
        143,  // imap2
        179,  // BGP
        389,  // ldap
        427,  // SLP (Also used by Apple Filing Protocol)
        465,  // smtp+ssl
        512,  // print / exec
        513,  // login
        514,  // shell
        515,  // printer
        526,  // tempo
        530,  // courier
        531,  // chat
        532,  // netnews
        540,  // uucp
        548,  // AFP (Apple Filing Protocol)
        556,  // remotefs
        563,  // nntp+ssl
        587,  // smtp (rfc6409)
        601,  // syslog-conn (rfc3195)
        636,  // ldap+ssl
        993,  // ldap+ssl
        995,  // pop3+ssl
        2049, // nfs
        3659, // apple-sasl / PasswordServer
        4045, // lockd
        6000, // X11
        6665, // Alternate IRC [Apple addition]
        6666, // Alternate IRC [Apple addition]
        6667, // Standard IRC [Apple addition]
        6668, // Alternate IRC [Apple addition]
        6669, // Alternate IRC [Apple addition]
        6697, // IRC + TLS
    ]);
}

pub(super) struct NetworkBindSet {
    pub port: u16,
    pub addrs: Vec<IpAddr>,
    pub search: bool,
}

impl Network {
    /////////////////////////////////////////////////////

    // Returns a port, a set of ip addresses to bind to, and a
    // bool specifying if multiple ports should be tried
    async fn convert_listen_address_to_bind_set(
        &self,
        listen_address: String,
    ) -> EyreResult<NetworkBindSet> {
        if listen_address.is_empty() {
            // If listen address is empty, start with port 5150 and iterate
            let ip_addrs = available_unspecified_addresses();
            Ok(NetworkBindSet {
                port: 5150,
                addrs: ip_addrs,
                search: true,
            })
        } else {
            // If no address is specified, but the port is, use ipv4 and ipv6 unspecified
            // If the address is specified, only use the specified port and fail otherwise
            let sockaddrs =
                listen_address_to_socket_addrs(&listen_address).map_err(|e| eyre!("{}", e))?;
            if sockaddrs.is_empty() {
                bail!("No valid listen address: {}", listen_address);
            }
            let port = sockaddrs[0].port();
            if port == 0 {
                Ok(NetworkBindSet {
                    port: 5150,
                    addrs: sockaddrs.iter().map(|s| s.ip()).collect(),
                    search: true,
                })
            } else {
                Ok(NetworkBindSet {
                    port,
                    addrs: sockaddrs.iter().map(|s| s.ip()).collect(),
                    search: false,
                })
            }
        }
    }

    // Add local dial info to preferred local address table
    fn add_preferred_local_address(inner: &mut NetworkInner, pa: PeerAddress) {
        let key = (pa.protocol_type(), pa.address_type());
        let sa = pa.socket_addr();
        let unspec_sa = match sa {
            SocketAddr::V4(a) => SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, a.port())),
            SocketAddr::V6(a) => {
                SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, a.port(), 0, 0))
            }
        };
        inner.preferred_local_addresses.insert(key, unspec_sa);
    }

    /////////////////////////////////////////////////////

    pub(super) async fn bind_udp_protocol_handlers(
        &self,
        editor_public_internet: &mut RoutingDomainEditor,
        editor_local_network: &mut RoutingDomainEditor,
    ) -> EyreResult<()> {
        log_net!("UDP: binding protocol handlers");
        let routing_table = self.routing_table();
        let (listen_address, public_address, detect_address_changes) = {
            let c = self.config.get();
            (
                c.network.protocol.udp.listen_address.clone(),
                c.network.protocol.udp.public_address.clone(),
                c.network.detect_address_changes,
            )
        };

        // Get the binding parameters from the user-specified listen address
        let bind_set = self
            .convert_listen_address_to_bind_set(listen_address.clone())
            .await?;

        // Now create udp inbound sockets for whatever interfaces we're listening on
        if bind_set.search {
            info!(
                "UDP: searching for free port starting with {} on {:?}",
                bind_set.port, bind_set.addrs
            );
        } else {
            info!(
                "UDP: binding protocol handlers at port {} on {:?}",
                bind_set.port, bind_set.addrs
            );
        }

        let mut local_dial_info_list = self.create_udp_protocol_handlers(bind_set).await?;
        local_dial_info_list.sort();

        let mut static_public = false;

        log_net!(
            "UDP: protocol handlers bound to {:#?}",
            local_dial_info_list
        );

        // Add static public dialinfo if it's configured
        if let Some(public_address) = public_address.as_ref() {
            // Resolve statically configured public dialinfo
            let mut public_sockaddrs = public_address
                .to_socket_addrs()
                .wrap_err(format!("Unable to resolve address: {}", public_address))?;

            // Add all resolved addresses as public dialinfo
            for pdi_addr in &mut public_sockaddrs {
                let pdi = DialInfo::udp_from_socketaddr(pdi_addr);

                // Register the public address
                editor_public_internet.register_dial_info(pdi.clone(), DialInfoClass::Direct)?;
                static_public = true;

                // See if this public address is also a local interface address we haven't registered yet
                let is_interface_address = (|| {
                    for ip_addr in self.get_stable_interface_addresses() {
                        if pdi_addr.ip() == ip_addr {
                            return true;
                        }
                    }
                    false
                })();

                if !local_dial_info_list.contains(&pdi) && is_interface_address {
                    editor_local_network.register_dial_info(
                        DialInfo::udp_from_socketaddr(pdi_addr),
                        DialInfoClass::Direct,
                    )?;
                }
            }
        }

        // Register local dial info
        for di in &local_dial_info_list {
            // If the local interface address is global, then register global dial info
            // if no other public address is specified
            if !detect_address_changes
                && public_address.is_none()
                && routing_table.ensure_dial_info_is_valid(RoutingDomain::PublicInternet, di)
            {
                editor_public_internet.register_dial_info(di.clone(), DialInfoClass::Direct)?;
                static_public = true;
            }

            // Register interface dial info as well since the address is on the local interface
            editor_local_network.register_dial_info(di.clone(), DialInfoClass::Direct)?;
        }

        {
            let mut inner = self.inner.lock();
            if static_public {
                inner.static_public_dialinfo.insert(ProtocolType::UDP);
            }
            for ldi in local_dial_info_list {
                Self::add_preferred_local_address(&mut inner, ldi.peer_address());
            }
        }

        // Now create tasks for udp listeners
        self.create_udp_listener_tasks().await
    }

    pub(super) async fn start_ws_listeners(
        &self,
        editor_public_internet: &mut RoutingDomainEditor,
        editor_local_network: &mut RoutingDomainEditor,
    ) -> EyreResult<()> {
        log_net!("WS: binding protocol handlers");
        let routing_table = self.routing_table();
        let (listen_address, url, path, detect_address_changes) = {
            let c = self.config.get();
            (
                c.network.protocol.ws.listen_address.clone(),
                c.network.protocol.ws.url.clone(),
                c.network.protocol.ws.path.clone(),
                c.network.detect_address_changes,
            )
        };

        // Get the binding parameters from the user-specified listen address
        let bind_set = self
            .convert_listen_address_to_bind_set(listen_address.clone())
            .await?;

        if bind_set.search {
            info!(
                "WS: searching for free port starting with {} on {:?}",
                bind_set.port, bind_set.addrs
            );
        } else {
            info!(
                "WS: binding protocol handlers at port {} on {:?}",
                bind_set.port, bind_set.addrs
            );
        }
        let socket_addresses = self
            .start_tcp_listener(
                bind_set,
                false,
                Box::new(|c, t| Box::new(WebsocketProtocolHandler::new(c, t))),
            )
            .await?;
        log_net!("WS: protocol handlers started on {:#?}", socket_addresses);

        let mut static_public = false;
        let mut registered_addresses: HashSet<IpAddr> = HashSet::new();

        // Add static public dialinfo if it's configured
        if let Some(url) = url.as_ref() {
            let mut split_url = SplitUrl::from_str(url).wrap_err("couldn't split url")?;
            if split_url.scheme.to_ascii_lowercase() != "ws" {
                bail!("WS URL must use 'ws://' scheme");
            }
            split_url.scheme = "ws".to_owned();

            // Resolve static public hostnames
            let global_socket_addrs = split_url
                .host_port(80)
                .to_socket_addrs()
                .wrap_err("failed to resolve ws url")?;

            for gsa in global_socket_addrs {
                let pdi = DialInfo::try_ws(SocketAddress::from_socket_addr(gsa), url.clone())
                    .wrap_err("try_ws failed")?;

                editor_public_internet.register_dial_info(pdi.clone(), DialInfoClass::Direct)?;
                static_public = true;

                // See if this public address is also a local interface address
                if !registered_addresses.contains(&gsa.ip())
                    && self.is_stable_interface_address(gsa.ip())
                {
                    editor_local_network.register_dial_info(pdi, DialInfoClass::Direct)?;
                }

                registered_addresses.insert(gsa.ip());
            }
        }

        for socket_address in &socket_addresses {
            // Skip addresses we already did
            if registered_addresses.contains(&socket_address.ip_addr()) {
                continue;
            }
            // Build dial info request url
            let local_url = format!("ws://{}/{}", socket_address, path);
            let local_di =
                DialInfo::try_ws(*socket_address, local_url).wrap_err("try_ws failed")?;

            if !detect_address_changes
                && url.is_none()
                && routing_table.ensure_dial_info_is_valid(RoutingDomain::PublicInternet, &local_di)
            {
                // Register public dial info
                editor_public_internet
                    .register_dial_info(local_di.clone(), DialInfoClass::Direct)?;
                static_public = true;
            }

            // Register local dial info
            editor_local_network.register_dial_info(local_di, DialInfoClass::Direct)?;
        }

        let mut inner = self.inner.lock();
        if static_public {
            inner.static_public_dialinfo.insert(ProtocolType::WS);
        }
        for sa in socket_addresses {
            Self::add_preferred_local_address(&mut inner, PeerAddress::new(sa, ProtocolType::WS));
        }

        Ok(())
    }

    pub(super) async fn start_wss_listeners(
        &self,
        editor_public_internet: &mut RoutingDomainEditor,
        editor_local_network: &mut RoutingDomainEditor,
    ) -> EyreResult<()> {
        log_net!("WSS: binding protocol handlers");

        let (listen_address, url, _detect_address_changes) = {
            let c = self.config.get();
            (
                c.network.protocol.wss.listen_address.clone(),
                c.network.protocol.wss.url.clone(),
                c.network.detect_address_changes,
            )
        };

        // Get the binding parameters from the user-specified listen address
        let bind_set = self
            .convert_listen_address_to_bind_set(listen_address.clone())
            .await?;

        if bind_set.search {
            info!(
                "WSS: searching for free port starting with {} on {:?}",
                bind_set.port, bind_set.addrs
            );
        } else {
            info!(
                "WSS: binding protocol handlers at port {} on {:?}",
                bind_set.port, bind_set.addrs
            );
        }

        let socket_addresses = self
            .start_tcp_listener(
                bind_set,
                true,
                Box::new(|c, t| Box::new(WebsocketProtocolHandler::new(c, t))),
            )
            .await?;
        log_net!("WSS: protocol handlers started on {:#?}", socket_addresses);

        // NOTE: No interface dial info for WSS, as there is no way to connect to a local dialinfo via TLS
        // If the hostname is specified, it is the public dialinfo via the URL. If no hostname
        // is specified, then TLS won't validate, so no local dialinfo is possible.
        // This is not the case with unencrypted websockets, which can be specified solely by an IP address

        let mut static_public = false;
        let mut registered_addresses: HashSet<IpAddr> = HashSet::new();

        // Add static public dialinfo if it's configured
        if let Some(url) = url.as_ref() {
            // Add static public dialinfo if it's configured
            let mut split_url = SplitUrl::from_str(url)?;
            if split_url.scheme.to_ascii_lowercase() != "wss" {
                bail!("WSS URL must use 'wss://' scheme");
            }
            split_url.scheme = "wss".to_owned();

            // Resolve static public hostnames
            let global_socket_addrs = split_url
                .host_port(443)
                .to_socket_addrs()
                .wrap_err("failed to resolve wss url")?;
            for gsa in global_socket_addrs {
                let pdi = DialInfo::try_wss(SocketAddress::from_socket_addr(gsa), url.clone())
                    .wrap_err("try_wss failed")?;

                editor_public_internet.register_dial_info(pdi.clone(), DialInfoClass::Direct)?;
                static_public = true;

                // See if this public address is also a local interface address
                if !registered_addresses.contains(&gsa.ip())
                    && self.is_stable_interface_address(gsa.ip())
                {
                    editor_local_network.register_dial_info(pdi, DialInfoClass::Direct)?;
                }

                registered_addresses.insert(gsa.ip());
            }
        } else {
            bail!("WSS URL must be specified due to TLS requirements");
        }

        let mut inner = self.inner.lock();
        if static_public {
            inner.static_public_dialinfo.insert(ProtocolType::WSS);
        }
        for sa in socket_addresses {
            Self::add_preferred_local_address(&mut inner, PeerAddress::new(sa, ProtocolType::WSS));
        }

        Ok(())
    }

    pub(super) async fn start_tcp_listeners(
        &self,
        editor_public_internet: &mut RoutingDomainEditor,
        editor_local_network: &mut RoutingDomainEditor,
    ) -> EyreResult<()> {
        log_net!("TCP: binding protocol handlers");

        let routing_table = self.routing_table();
        let (listen_address, public_address, detect_address_changes) = {
            let c = self.config.get();
            (
                c.network.protocol.tcp.listen_address.clone(),
                c.network.protocol.tcp.public_address.clone(),
                c.network.detect_address_changes,
            )
        };

        // Get the binding parameters from the user-specified listen address
        let bind_set = self
            .convert_listen_address_to_bind_set(listen_address.clone())
            .await?;

        if bind_set.search {
            info!(
                "TCP: searching for free port starting with {} on {:?}",
                bind_set.port, bind_set.addrs
            );
        } else {
            info!(
                "TCP: binding protocol handlers at port {} on {:?}",
                bind_set.port, bind_set.addrs
            );
        }
        let socket_addresses = self
            .start_tcp_listener(
                bind_set,
                false,
                Box::new(|c, _| Box::new(RawTcpProtocolHandler::new(c))),
            )
            .await?;
        log_net!("TCP: protocol handlers started on {:#?}", socket_addresses);

        let mut static_public = false;
        let mut registered_addresses: HashSet<IpAddr> = HashSet::new();

        // Add static public dialinfo if it's configured
        if let Some(public_address) = public_address.as_ref() {
            // Resolve statically configured public dialinfo
            let mut public_sockaddrs = public_address
                .to_socket_addrs()
                .wrap_err("failed to resolve tcp address")?;

            // Add all resolved addresses as public dialinfo
            for pdi_addr in &mut public_sockaddrs {
                // Skip addresses we already did
                if registered_addresses.contains(&pdi_addr.ip()) {
                    continue;
                }
                let pdi = DialInfo::tcp_from_socketaddr(pdi_addr);

                editor_public_internet.register_dial_info(pdi.clone(), DialInfoClass::Direct)?;
                static_public = true;

                // See if this public address is also a local interface address
                if self.is_stable_interface_address(pdi_addr.ip()) {
                    editor_local_network.register_dial_info(pdi, DialInfoClass::Direct)?;
                }
            }
        }

        for socket_address in &socket_addresses {
            let di = DialInfo::tcp(*socket_address);

            // Register global dial info if no public address is specified
            if !detect_address_changes
                && public_address.is_none()
                && routing_table.ensure_dial_info_is_valid(RoutingDomain::PublicInternet, &di)
            {
                editor_public_internet.register_dial_info(di.clone(), DialInfoClass::Direct)?;
                static_public = true;
            }
            // Register interface dial info
            editor_local_network.register_dial_info(di.clone(), DialInfoClass::Direct)?;
            registered_addresses.insert(socket_address.ip_addr());
        }

        let mut inner = self.inner.lock();

        if static_public {
            inner.static_public_dialinfo.insert(ProtocolType::TCP);
        }
        for sa in socket_addresses {
            Self::add_preferred_local_address(&mut inner, PeerAddress::new(sa, ProtocolType::TCP));
        }

        Ok(())
    }
}
