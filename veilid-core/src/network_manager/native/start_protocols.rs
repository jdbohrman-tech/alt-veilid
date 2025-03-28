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
    #[instrument(level = "trace", skip_all)]
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
    pub(super) fn set_preferred_local_address(inner: &mut NetworkInner, pa: PeerAddress) {
        let key = (pa.protocol_type(), pa.address_type());
        let sa = pa.socket_addr();
        // let unspec_sa = match sa {
        //     SocketAddr::V4(a) => SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, a.port())),
        //     SocketAddr::V6(a) => {
        //         SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, a.port(), 0, 0))
        //     }
        // };
        inner.preferred_local_addresses.entry(key).or_insert(sa);
    }

    /////////////////////////////////////////////////////

    #[instrument(level = "trace", skip_all)]
    pub(super) async fn bind_udp_protocol_handlers(&self) -> EyreResult<StartupDisposition> {
        veilid_log!(self trace "UDP: binding protocol handlers");
        let (listen_address, public_address, detect_address_changes) = self.config().with(|c| {
            (
                c.network.protocol.udp.listen_address.clone(),
                c.network.protocol.udp.public_address.clone(),
                c.network.detect_address_changes,
            )
        });

        // Get the binding parameters from the user-specified listen address
        let bind_set = self
            .convert_listen_address_to_bind_set(listen_address.clone())
            .await?;

        // Now create udp inbound sockets for whatever interfaces we're listening on
        if bind_set.search {
            veilid_log!(self info
                "UDP: searching for free port starting with {} on {:?}",
                bind_set.port, bind_set.addrs
            );
        } else {
            veilid_log!(self info
                "UDP: binding protocol handlers at port {} on {:?}",
                bind_set.port, bind_set.addrs
            );
        }

        if !self.create_udp_protocol_handlers(bind_set).await? {
            return Ok(StartupDisposition::BindRetry);
        };

        // Now create tasks for udp listeners
        self.create_udp_listener_tasks().await?;

        {
            let mut inner = self.inner.lock();
            if public_address.is_some() && !detect_address_changes {
                inner.static_public_dial_info.insert(ProtocolType::UDP);
            }
        }

        Ok(StartupDisposition::Success)
    }

    #[instrument(level = "trace", skip_all)]
    pub(super) async fn register_udp_dial_info(
        &self,
        editor_public_internet: &mut RoutingDomainEditorPublicInternet<'_>,
        editor_local_network: &mut RoutingDomainEditorLocalNetwork<'_>,
    ) -> EyreResult<()> {
        veilid_log!(self trace "UDP: registering dial info");

        let (public_address, detect_address_changes) = self.config().with(|c| {
            (
                c.network.protocol.udp.public_address.clone(),
                c.network.detect_address_changes,
            )
        });

        let local_dial_info_list = {
            let mut out = vec![];
            if let Some(bound_addresses) = {
                let inner = self.inner.lock();
                inner
                    .bound_address_per_protocol
                    .get(&ProtocolType::UDP)
                    .cloned()
            } {
                for addr in bound_addresses {
                    let idi_addrs = self.translate_unspecified_address(addr);
                    for idi_addr in idi_addrs {
                        out.push(DialInfo::udp_from_socketaddr(idi_addr));
                    }
                }
            }
            out.sort();
            out.dedup();
            out
        };

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
                editor_public_internet.add_dial_info(pdi.clone(), DialInfoClass::Direct);

                // See if this public address is also a local interface address we haven't registered yet
                if self.is_stable_interface_address(pdi_addr.ip()) {
                    editor_local_network.add_dial_info(
                        DialInfo::udp_from_socketaddr(pdi_addr),
                        DialInfoClass::Direct,
                    );
                }
            }
        }

        // Register local dial info
        for di in &local_dial_info_list {
            // If the local interface address is global, then register global dial info
            // if no other public address is specified
            if !detect_address_changes && public_address.is_none() && di.address().is_global() {
                editor_public_internet.add_dial_info(di.clone(), DialInfoClass::Direct);
            }

            // Register interface dial info as well since the address is on the local interface
            editor_local_network.add_dial_info(di.clone(), DialInfoClass::Direct);
        }

        Ok(())
    }

    #[instrument(level = "trace", skip_all)]
    pub(super) async fn start_ws_listeners(&self) -> EyreResult<StartupDisposition> {
        veilid_log!(self trace "WS: binding protocol handlers");
        let (listen_address, url, detect_address_changes) = self.config().with(|c| {
            (
                c.network.protocol.ws.listen_address.clone(),
                c.network.protocol.ws.url.clone(),
                c.network.detect_address_changes,
            )
        });

        // Get the binding parameters from the user-specified listen address
        let bind_set = self
            .convert_listen_address_to_bind_set(listen_address.clone())
            .await?;

        if bind_set.search {
            veilid_log!(self info
                "WS: searching for free port starting with {} on {:?}",
                bind_set.port, bind_set.addrs
            );
        } else {
            veilid_log!(self info
                "WS: binding protocol handlers at port {} on {:?}",
                bind_set.port, bind_set.addrs
            );
        }
        if !self
            .start_tcp_listener(
                bind_set,
                false,
                ProtocolType::WS,
                Box::new(|r, t| Box::new(WebsocketProtocolHandler::new(r, t))),
            )
            .await?
        {
            return Ok(StartupDisposition::BindRetry);
        }

        {
            let mut inner = self.inner.lock();
            if url.is_some() && !detect_address_changes {
                inner.static_public_dial_info.insert(ProtocolType::WS);
            }
        }

        Ok(StartupDisposition::Success)
    }

    #[instrument(level = "trace", skip_all)]
    pub(super) async fn register_ws_dial_info(
        &self,
        editor_public_internet: &mut RoutingDomainEditorPublicInternet<'_>,
        editor_local_network: &mut RoutingDomainEditorLocalNetwork<'_>,
    ) -> EyreResult<()> {
        veilid_log!(self trace "WS: registering dial info");
        let (url, path, detect_address_changes) = self.config().with(|c| {
            (
                c.network.protocol.ws.url.clone(),
                c.network.protocol.ws.path.clone(),
                c.network.detect_address_changes,
            )
        });

        let mut registered_addresses: HashSet<IpAddr> = HashSet::new();

        let socket_addresses = {
            let mut out = vec![];
            if let Some(bound_addresses) = {
                let inner = self.inner.lock();
                inner
                    .bound_address_per_protocol
                    .get(&ProtocolType::WS)
                    .cloned()
            } {
                for addr in bound_addresses {
                    for idi_addr in self
                        .translate_unspecified_address(addr)
                        .into_iter()
                        .map(SocketAddress::from_socket_addr)
                    {
                        out.push(idi_addr);
                    }
                }
            }
            out.sort();
            out.dedup();
            out
        };

        // Add static public dialinfo if it's configured
        if let Some(url) = url.as_ref() {
            let mut split_url = SplitUrl::from_str(url).wrap_err("couldn't split url")?;
            if !split_url.scheme.eq_ignore_ascii_case("ws") {
                bail!("WS URL must use 'ws://' scheme");
            }
            "ws".clone_into(&mut split_url.scheme);

            // Resolve static public hostnames
            let global_socket_addrs = split_url
                .host_port(80)
                .to_socket_addrs()
                .wrap_err("failed to resolve ws url")?;

            for gsa in global_socket_addrs {
                let pdi = DialInfo::try_ws(SocketAddress::from_socket_addr(gsa), url.clone())
                    .wrap_err("try_ws failed")?;

                editor_public_internet.add_dial_info(pdi.clone(), DialInfoClass::Direct);

                // See if this public address is also a local interface address
                if !registered_addresses.contains(&gsa.ip())
                    && self.is_stable_interface_address(gsa.ip())
                {
                    editor_local_network.add_dial_info(pdi, DialInfoClass::Direct);
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

            if !detect_address_changes && url.is_none() && local_di.address().is_global() {
                // Register public dial info
                editor_public_internet.add_dial_info(local_di.clone(), DialInfoClass::Direct);
            }

            // Register local dial info
            editor_local_network.add_dial_info(local_di, DialInfoClass::Direct);
        }

        Ok(())
    }

    #[instrument(level = "trace", skip_all)]
    pub(super) async fn start_wss_listeners(&self) -> EyreResult<StartupDisposition> {
        veilid_log!(self trace "WSS: binding protocol handlers");

        let (listen_address, url, detect_address_changes) = self.config().with(|c| {
            (
                c.network.protocol.wss.listen_address.clone(),
                c.network.protocol.wss.url.clone(),
                c.network.detect_address_changes,
            )
        });

        // Get the binding parameters from the user-specified listen address
        let bind_set = self
            .convert_listen_address_to_bind_set(listen_address.clone())
            .await?;

        if bind_set.search {
            veilid_log!(self info
                "WSS: searching for free port starting with {} on {:?}",
                bind_set.port, bind_set.addrs
            );
        } else {
            veilid_log!(self info
                "WSS: binding protocol handlers at port {} on {:?}",
                bind_set.port, bind_set.addrs
            );
        }

        if !self
            .start_tcp_listener(
                bind_set,
                true,
                ProtocolType::WSS,
                Box::new(|r, t| Box::new(WebsocketProtocolHandler::new(r, t))),
            )
            .await?
        {
            return Ok(StartupDisposition::BindRetry);
        }

        {
            let mut inner = self.inner.lock();
            if url.is_some() && !detect_address_changes {
                inner.static_public_dial_info.insert(ProtocolType::WSS);
            }
        }

        Ok(StartupDisposition::Success)
    }

    #[instrument(level = "trace", skip_all)]
    pub(super) async fn register_wss_dial_info(
        &self,
        editor_public_internet: &mut RoutingDomainEditorPublicInternet<'_>,
        editor_local_network: &mut RoutingDomainEditorLocalNetwork<'_>,
    ) -> EyreResult<()> {
        veilid_log!(self trace "WSS: registering dialinfo");

        let (url, _detect_address_changes) = self.config().with(|c| {
            (
                c.network.protocol.wss.url.clone(),
                c.network.detect_address_changes,
            )
        });

        // NOTE: No interface dial info for WSS, as there is no way to connect to a local dialinfo via TLS
        // If the hostname is specified, it is the public dialinfo via the URL. If no hostname
        // is specified, then TLS won't validate, so no local dialinfo is possible.
        // This is not the case with unencrypted websockets, which can be specified solely by an IP address

        let mut registered_addresses: HashSet<IpAddr> = HashSet::new();

        // Add static public dialinfo if it's configured
        if let Some(url) = url.as_ref() {
            // Add static public dialinfo if it's configured
            let mut split_url = SplitUrl::from_str(url)?;
            if !split_url.scheme.eq_ignore_ascii_case("wss") {
                bail!("WSS URL must use 'wss://' scheme");
            }
            "wss".clone_into(&mut split_url.scheme);

            // Resolve static public hostnames
            let global_socket_addrs = split_url
                .host_port(443)
                .to_socket_addrs()
                .wrap_err("failed to resolve wss url")?;
            for gsa in global_socket_addrs {
                let pdi = DialInfo::try_wss(SocketAddress::from_socket_addr(gsa), url.clone())
                    .wrap_err("try_wss failed")?;

                editor_public_internet.add_dial_info(pdi.clone(), DialInfoClass::Direct);

                // See if this public address is also a local interface address
                if !registered_addresses.contains(&gsa.ip())
                    && self.is_stable_interface_address(gsa.ip())
                {
                    editor_local_network.add_dial_info(pdi, DialInfoClass::Direct);
                }

                registered_addresses.insert(gsa.ip());
            }
        } else {
            bail!("WSS URL must be specified due to TLS requirements");
        }

        Ok(())
    }

    #[instrument(level = "trace", skip_all)]
    pub(super) async fn start_tcp_listeners(&self) -> EyreResult<StartupDisposition> {
        veilid_log!(self trace "TCP: binding protocol handlers");

        let (listen_address, public_address, detect_address_changes) = self.config().with(|c| {
            (
                c.network.protocol.tcp.listen_address.clone(),
                c.network.protocol.tcp.public_address.clone(),
                c.network.detect_address_changes,
            )
        });

        // Get the binding parameters from the user-specified listen address
        let bind_set = self
            .convert_listen_address_to_bind_set(listen_address.clone())
            .await?;

        if bind_set.search {
            veilid_log!(self info
                "TCP: searching for free port starting with {} on {:?}",
                bind_set.port, bind_set.addrs
            );
        } else {
            veilid_log!(self info
                "TCP: binding protocol handlers at port {} on {:?}",
                bind_set.port, bind_set.addrs
            );
        }
        if !self
            .start_tcp_listener(
                bind_set,
                false,
                ProtocolType::TCP,
                Box::new(|c, _| Box::new(RawTcpProtocolHandler::new(c))),
            )
            .await?
        {
            return Ok(StartupDisposition::BindRetry);
        }

        {
            let mut inner = self.inner.lock();
            if public_address.is_some() && !detect_address_changes {
                inner.static_public_dial_info.insert(ProtocolType::TCP);
            }
        }

        Ok(StartupDisposition::Success)
    }

    #[instrument(level = "trace", skip_all)]
    pub(super) async fn register_tcp_dial_info(
        &self,
        editor_public_internet: &mut RoutingDomainEditorPublicInternet<'_>,
        editor_local_network: &mut RoutingDomainEditorLocalNetwork<'_>,
    ) -> EyreResult<()> {
        veilid_log!(self trace "TCP: registering dialinfo");

        let (public_address, detect_address_changes) = self.config().with(|c| {
            (
                c.network.protocol.tcp.public_address.clone(),
                c.network.detect_address_changes,
            )
        });

        let mut registered_addresses: HashSet<IpAddr> = HashSet::new();

        let socket_addresses = {
            let mut out = vec![];
            if let Some(bound_addresses) = {
                let inner = self.inner.lock();
                inner
                    .bound_address_per_protocol
                    .get(&ProtocolType::TCP)
                    .cloned()
            } {
                for addr in bound_addresses {
                    for idi_addr in self
                        .translate_unspecified_address(addr)
                        .into_iter()
                        .map(SocketAddress::from_socket_addr)
                    {
                        out.push(idi_addr);
                    }
                }
            }
            out.sort();
            out.dedup();
            out
        };

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

                editor_public_internet.add_dial_info(pdi.clone(), DialInfoClass::Direct);

                // See if this public address is also a local interface address
                if self.is_stable_interface_address(pdi_addr.ip()) {
                    editor_local_network.add_dial_info(pdi, DialInfoClass::Direct);
                }
            }
        }

        for socket_address in &socket_addresses {
            let di = DialInfo::tcp(*socket_address);

            // Register global dial info if no public address is specified
            if !detect_address_changes && public_address.is_none() && di.address().is_global() {
                editor_public_internet.add_dial_info(di.clone(), DialInfoClass::Direct);
            }
            // Register interface dial info
            editor_local_network.add_dial_info(di.clone(), DialInfoClass::Direct);
            registered_addresses.insert(socket_address.ip_addr());
        }

        Ok(())
    }
}
