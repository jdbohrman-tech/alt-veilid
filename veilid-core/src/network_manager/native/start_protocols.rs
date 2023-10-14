use super::sockets::*;
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

impl Network {
    /////////////////////////////////////////////////////
    // Support for binding first on ports to ensure nobody binds ahead of us
    // or two copies of the app don't accidentally collide. This is tricky
    // because we use 'reuseaddr/port' and we can accidentally bind in front of ourselves :P

    fn bind_first_udp_port(&self, udp_port: u16) -> bool {
        let mut inner = self.inner.lock();
        if inner.bound_first_udp.contains_key(&udp_port) {
            return true;
        }
        // If the address is specified, only use the specified port and fail otherwise
        let mut bound_first_socket_v4 = None;
        let mut bound_first_socket_v6 = None;
        if let Ok(bfs4) =
            new_bound_first_udp_socket(SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), udp_port))
        {
            if let Ok(bfs6) = new_bound_first_udp_socket(SocketAddr::new(
                IpAddr::V6(Ipv6Addr::UNSPECIFIED),
                udp_port,
            )) {
                bound_first_socket_v4 = Some(bfs4);
                bound_first_socket_v6 = Some(bfs6);
            }
        }
        if let (Some(bfs4), Some(bfs6)) = (bound_first_socket_v4, bound_first_socket_v6) {
            cfg_if! {
                if #[cfg(windows)] {
                    // On windows, drop the socket. This is a race condition, but there's
                    // no way around it. This isn't for security anyway, it's to prevent multiple copies of the
                    // app from binding on the same port.
                    drop(bfs4);
                    drop(bfs6);
                    inner.bound_first_udp.insert(udp_port, None);
                } else {
                    inner.bound_first_udp.insert(udp_port, Some((bfs4, bfs6)));
                }
            }
            true
        } else {
            false
        }
    }

    fn bind_first_tcp_port(&self, tcp_port: u16) -> bool {
        let mut inner = self.inner.lock();
        if inner.bound_first_tcp.contains_key(&tcp_port) {
            return true;
        }
        // If the address is specified, only use the specified port and fail otherwise
        let mut bound_first_socket_v4 = None;
        let mut bound_first_socket_v6 = None;
        if let Ok(bfs4) =
            new_bound_first_tcp_socket(SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), tcp_port))
        {
            if let Ok(bfs6) = new_bound_first_tcp_socket(SocketAddr::new(
                IpAddr::V6(Ipv6Addr::UNSPECIFIED),
                tcp_port,
            )) {
                bound_first_socket_v4 = Some(bfs4);
                bound_first_socket_v6 = Some(bfs6);
            }
        }
        if let (Some(bfs4), Some(bfs6)) = (bound_first_socket_v4, bound_first_socket_v6) {
            cfg_if! {
                if #[cfg(windows)] {
                    // On windows, drop the socket. This is a race condition, but there's
                    // no way around it. This isn't for security anyway, it's to prevent multiple copies of the
                    // app from binding on the same port.
                    drop(bfs4);
                    drop(bfs6);
                    inner.bound_first_tcp.insert(tcp_port, None);
                } else {
                    inner.bound_first_tcp.insert(tcp_port, Some((bfs4, bfs6)));
                }
            }
            true
        } else {
            false
        }
    }

    pub(super) fn free_bound_first_ports(&self) {
        let mut inner = self.inner.lock();
        inner.bound_first_udp.clear();
        inner.bound_first_tcp.clear();
    }

    /////////////////////////////////////////////////////

    fn find_available_udp_port(&self, start_port: u16) -> EyreResult<u16> {
        // If the address is empty, iterate ports until we find one we can use.
        let mut udp_port = start_port;
        loop {
            if BAD_PORTS.contains(&udp_port) {
                continue;
            }
            if self.bind_first_udp_port(udp_port) {
                break;
            }
            if udp_port == 65535 {
                bail!("Could not find free udp port to listen on");
            }
            udp_port += 1;
        }
        Ok(udp_port)
    }

    fn find_available_tcp_port(&self, start_port: u16) -> EyreResult<u16> {
        // If the address is empty, iterate ports until we find one we can use.
        let mut tcp_port = start_port;
        loop {
            if BAD_PORTS.contains(&tcp_port) {
                continue;
            }
            if self.bind_first_tcp_port(tcp_port) {
                break;
            }
            if tcp_port == 65535 {
                bail!("Could not find free tcp port to listen on");
            }
            tcp_port += 1;
        }
        Ok(tcp_port)
    }

    async fn allocate_udp_port(&self, listen_address: String) -> EyreResult<(u16, Vec<IpAddr>)> {
        if listen_address.is_empty() {
            // If listen address is empty, find us a port iteratively
            let port = self.find_available_udp_port(5150)?;
            let ip_addrs = vec![
                IpAddr::V4(Ipv4Addr::UNSPECIFIED),
                IpAddr::V6(Ipv6Addr::UNSPECIFIED),
            ];
            Ok((port, ip_addrs))
        } else {
            // If no address is specified, but the port is, use ipv4 and ipv6 unspecified
            // If the address is specified, only use the specified port and fail otherwise
            let sockaddrs =
                listen_address_to_socket_addrs(&listen_address).map_err(|e| eyre!("{}", e))?;
            if sockaddrs.is_empty() {
                bail!("No valid listen address: {}", listen_address);
            }
            let port = sockaddrs[0].port();

            Ok((port, sockaddrs.iter().map(|s| s.ip()).collect()))
        }
    }

    async fn allocate_tcp_port(&self, listen_address: String) -> EyreResult<(u16, Vec<IpAddr>)> {
        if listen_address.is_empty() {
            // If listen address is empty, find us a port iteratively
            let port = self.find_available_tcp_port(5150)?;
            let ip_addrs = vec![
                IpAddr::V4(Ipv4Addr::UNSPECIFIED),
                IpAddr::V6(Ipv6Addr::UNSPECIFIED),
            ];
            Ok((port, ip_addrs))
        } else {
            // If no address is specified, but the port is, use ipv4 and ipv6 unspecified
            // If the address is specified, only use the specified port and fail otherwise
            let sockaddrs =
                listen_address_to_socket_addrs(&listen_address).map_err(|e| eyre!("{}", e))?;
            if sockaddrs.is_empty() {
                bail!("No valid listen address: {}", listen_address);
            }
            let port = sockaddrs[0].port();

            let mut attempts = 10;
            let mut success = false;
            while attempts >= 0 {
                if self.bind_first_tcp_port(port) {
                    success = true;
                    break;
                }
                attempts -= 1;

                // Wait 5 seconds before trying again
                log_net!(debug
                    "Binding TCP port at {} failed, waiting. Attempts remaining = {}",
                    port, attempts
                );
                sleep(5000).await
            }
            if !success {
                bail!("Could not find free tcp port to listen on");
            }
            Ok((port, sockaddrs.iter().map(|s| s.ip()).collect()))
        }
    }

    /////////////////////////////////////////////////////

    pub(super) async fn start_udp_listeners(
        &self,
        editor_public_internet: &mut RoutingDomainEditor,
        editor_local_network: &mut RoutingDomainEditor,
    ) -> EyreResult<()> {
        trace!("starting udp listeners");
        let routing_table = self.routing_table();
        let (listen_address, public_address, detect_address_changes) = {
            let c = self.config.get();
            (
                c.network.protocol.udp.listen_address.clone(),
                c.network.protocol.udp.public_address.clone(),
                c.network.detect_address_changes,
            )
        };

        // Pick out UDP port we're going to use everywhere
        // Keep sockets around until the end of this function
        // to keep anyone else from binding in front of us
        let (udp_port, ip_addrs) = self.allocate_udp_port(listen_address.clone()).await?;

        // Save the bound udp port for use later on
        self.inner.lock().udp_port = udp_port;

        // First, create outbound sockets
        // (unlike tcp where we create sockets for every connection)
        // and we'll add protocol handlers for them too
        self.create_udp_outbound_sockets().await?;

        // Now create udp inbound sockets for whatever interfaces we're listening on
        info!(
            "UDP: starting listeners on port {} at {:?}",
            udp_port, ip_addrs
        );
        let local_dial_info_list = self.create_udp_inbound_sockets(ip_addrs, udp_port).await?;
        let mut static_public = false;

        trace!("UDP: listener started on {:#?}", local_dial_info_list);

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
                if !detect_address_changes {
                    editor_public_internet
                        .register_dial_info(pdi.clone(), DialInfoClass::Direct)?;
                    static_public = true;
                }

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

        if static_public {
            self.inner
                .lock()
                .static_public_dialinfo
                .insert(ProtocolType::UDP);
        }

        // Now create tasks for udp listeners
        self.create_udp_listener_tasks().await
    }

    pub(super) async fn start_ws_listeners(
        &self,
        editor_public_internet: &mut RoutingDomainEditor,
        editor_local_network: &mut RoutingDomainEditor,
    ) -> EyreResult<()> {
        trace!("starting ws listeners");
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

        // Pick out TCP port we're going to use everywhere
        // Keep sockets around until the end of this function
        // to keep anyone else from binding in front of us
        let (ws_port, ip_addrs) = self.allocate_tcp_port(listen_address.clone()).await?;

        // Save the bound ws port for use later on
        self.inner.lock().ws_port = ws_port;

        info!(
            "WS: starting listener on port {} at {:?}",
            ws_port, ip_addrs
        );
        let socket_addresses = self
            .start_tcp_listener(
                ip_addrs,
                ws_port,
                false,
                Box::new(|c, t| Box::new(WebsocketProtocolHandler::new(c, t))),
            )
            .await?;
        trace!("WS: listener started on {:#?}", socket_addresses);

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

                if !detect_address_changes {
                    editor_public_internet
                        .register_dial_info(pdi.clone(), DialInfoClass::Direct)?;
                    static_public = true;
                }

                // See if this public address is also a local interface address
                if !registered_addresses.contains(&gsa.ip())
                    && self.is_stable_interface_address(gsa.ip())
                {
                    editor_local_network.register_dial_info(pdi, DialInfoClass::Direct)?;
                }

                registered_addresses.insert(gsa.ip());
            }
        }

        for socket_address in socket_addresses {
            // Skip addresses we already did
            if registered_addresses.contains(&socket_address.ip_addr()) {
                continue;
            }
            // Build dial info request url
            let local_url = format!("ws://{}/{}", socket_address, path);
            let local_di = DialInfo::try_ws(socket_address, local_url).wrap_err("try_ws failed")?;

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

        if static_public {
            self.inner
                .lock()
                .static_public_dialinfo
                .insert(ProtocolType::WS);
        }

        Ok(())
    }

    pub(super) async fn start_wss_listeners(
        &self,
        editor_public_internet: &mut RoutingDomainEditor,
        editor_local_network: &mut RoutingDomainEditor,
    ) -> EyreResult<()> {
        trace!("starting wss listeners");

        let (listen_address, url, detect_address_changes) = {
            let c = self.config.get();
            (
                c.network.protocol.wss.listen_address.clone(),
                c.network.protocol.wss.url.clone(),
                c.network.detect_address_changes,
            )
        };

        // Pick out TCP port we're going to use everywhere
        // Keep sockets around until the end of this function
        // to keep anyone else from binding in front of us
        let (wss_port, ip_addrs) = self.allocate_tcp_port(listen_address.clone()).await?;

        // Save the bound wss port for use later on
        self.inner.lock().wss_port = wss_port;

        info!(
            "WSS: starting listener on port {} at {:?}",
            wss_port, ip_addrs
        );
        let socket_addresses = self
            .start_tcp_listener(
                ip_addrs,
                wss_port,
                true,
                Box::new(|c, t| Box::new(WebsocketProtocolHandler::new(c, t))),
            )
            .await?;
        trace!("WSS: listener started on {:#?}", socket_addresses);

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

                if !detect_address_changes {
                    editor_public_internet
                        .register_dial_info(pdi.clone(), DialInfoClass::Direct)?;
                    static_public = true;
                }

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

        if static_public {
            self.inner
                .lock()
                .static_public_dialinfo
                .insert(ProtocolType::WSS);
        }

        Ok(())
    }

    pub(super) async fn start_tcp_listeners(
        &self,
        editor_public_internet: &mut RoutingDomainEditor,
        editor_local_network: &mut RoutingDomainEditor,
    ) -> EyreResult<()> {
        trace!("starting tcp listeners");

        let routing_table = self.routing_table();
        let (listen_address, public_address, detect_address_changes) = {
            let c = self.config.get();
            (
                c.network.protocol.tcp.listen_address.clone(),
                c.network.protocol.tcp.public_address.clone(),
                c.network.detect_address_changes,
            )
        };

        // Pick out TCP port we're going to use everywhere
        // Keep sockets around until the end of this function
        // to keep anyone else from binding in front of us
        let (tcp_port, ip_addrs) = self.allocate_tcp_port(listen_address.clone()).await?;

        // Save the bound tcp port for use later on
        self.inner.lock().tcp_port = tcp_port;

        info!(
            "TCP: starting listener on port {} at {:?}",
            tcp_port, ip_addrs
        );
        let socket_addresses = self
            .start_tcp_listener(
                ip_addrs,
                tcp_port,
                false,
                Box::new(|c, _| Box::new(RawTcpProtocolHandler::new(c))),
            )
            .await?;
        trace!("TCP: listener started on {:#?}", socket_addresses);

        let mut static_public = false;
        let mut registered_addresses: HashSet<IpAddr> = HashSet::new();

        for socket_address in socket_addresses {
            let di = DialInfo::tcp(socket_address);

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

                if !detect_address_changes {
                    editor_public_internet
                        .register_dial_info(pdi.clone(), DialInfoClass::Direct)?;
                    static_public = true;
                }

                // See if this public address is also a local interface address
                if self.is_stable_interface_address(pdi_addr.ip()) {
                    editor_local_network.register_dial_info(pdi, DialInfoClass::Direct)?;
                }
            }
        }

        if static_public {
            self.inner
                .lock()
                .static_public_dialinfo
                .insert(ProtocolType::TCP);
        }

        Ok(())
    }
}
