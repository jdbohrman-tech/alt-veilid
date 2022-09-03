////////////////////////////////////////////////////////////////
// Debugging

use super::*;
use routing_table::*;

fn get_bucket_entry_state(text: &str) -> Option<BucketEntryState> {
    if text == "dead" {
        Some(BucketEntryState::Dead)
    } else if text == "reliable" {
        Some(BucketEntryState::Reliable)
    } else if text == "unreliable" {
        Some(BucketEntryState::Unreliable)
    } else {
        None
    }
}
fn get_number(text: &str) -> Option<usize> {
    usize::from_str(text).ok()
}
fn get_dht_key(text: &str) -> Option<DHTKey> {
    DHTKey::try_decode(text).ok()
}
fn get_protocol_type(text: &str) -> Option<ProtocolType> {
    let lctext = text.to_ascii_lowercase();
    if lctext == "udp" {
        Some(ProtocolType::UDP)
    } else if lctext == "tcp" {
        Some(ProtocolType::TCP)
    } else if lctext == "ws" {
        Some(ProtocolType::WS)
    } else if lctext == "wss" {
        Some(ProtocolType::WSS)
    } else {
        None
    }
}
fn get_address_type(text: &str) -> Option<AddressType> {
    let lctext = text.to_ascii_lowercase();
    if lctext == "ipv4" {
        Some(AddressType::IPV4)
    } else if lctext == "ipv6" {
        Some(AddressType::IPV6)
    } else {
        None
    }
}
fn get_routing_domain(text: &str) -> Option<RoutingDomain> {
    let lctext = text.to_ascii_lowercase();
    if "publicinternet".starts_with(&lctext) {
        Some(RoutingDomain::PublicInternet)
    } else if "localnetwork".starts_with(&lctext) {
        Some(RoutingDomain::LocalNetwork)
    } else {
        None
    }
}

fn get_debug_argument<T, G: FnOnce(&str) -> Option<T>>(
    value: &str,
    context: &str,
    argument: &str,
    getter: G,
) -> Result<T, VeilidAPIError> {
    if let Some(val) = getter(value) {
        Ok(val)
    } else {
        Err(VeilidAPIError::InvalidArgument {
            context: context.to_owned(),
            argument: argument.to_owned(),
            value: value.to_owned(),
        })
    }
}
fn get_debug_argument_at<T, G: FnOnce(&str) -> Option<T>>(
    debug_args: &[String],
    pos: usize,
    context: &str,
    argument: &str,
    getter: G,
) -> Result<T, VeilidAPIError> {
    if pos >= debug_args.len() {
        return Err(VeilidAPIError::MissingArgument {
            context: context.to_owned(),
            argument: argument.to_owned(),
        });
    }
    let value = &debug_args[pos];
    if let Some(val) = getter(value) {
        Ok(val)
    } else {
        Err(VeilidAPIError::InvalidArgument {
            context: context.to_owned(),
            argument: argument.to_owned(),
            value: value.to_owned(),
        })
    }
}

impl VeilidAPI {
    async fn debug_buckets(&self, args: String) -> Result<String, VeilidAPIError> {
        let args: Vec<String> = args.split_whitespace().map(|s| s.to_owned()).collect();
        let mut min_state = BucketEntryState::Unreliable;
        if args.len() == 1 {
            min_state = get_debug_argument(
                &args[0],
                "debug_buckets",
                "min_state",
                get_bucket_entry_state,
            )?;
        }
        // Dump routing table bucket info
        let routing_table = self.network_manager()?.routing_table();
        Ok(routing_table.debug_info_buckets(min_state))
    }

    async fn debug_dialinfo(&self, _args: String) -> Result<String, VeilidAPIError> {
        // Dump routing table dialinfo
        let routing_table = self.network_manager()?.routing_table();
        Ok(routing_table.debug_info_dialinfo())
    }

    async fn debug_txtrecord(&self, _args: String) -> Result<String, VeilidAPIError> {
        // Dump routing table txt record
        let routing_table = self.network_manager()?.routing_table();
        Ok(routing_table.debug_info_txtrecord().await)
    }

    async fn debug_entries(&self, args: String) -> Result<String, VeilidAPIError> {
        let args: Vec<String> = args.split_whitespace().map(|s| s.to_owned()).collect();

        let mut min_state = BucketEntryState::Unreliable;
        let mut limit = 20;
        for arg in args {
            if let Some(ms) = get_bucket_entry_state(&arg) {
                min_state = ms;
            } else if let Some(lim) = get_number(&arg) {
                limit = lim;
            } else {
                return Err(VeilidAPIError::InvalidArgument {
                    context: "debug_entries".to_owned(),
                    argument: "unknown".to_owned(),
                    value: arg,
                });
            }
        }

        // Dump routing table entries
        let routing_table = self.network_manager()?.routing_table();
        Ok(routing_table.debug_info_entries(limit, min_state))
    }

    async fn debug_entry(&self, args: String) -> Result<String, VeilidAPIError> {
        let args: Vec<String> = args.split_whitespace().map(|s| s.to_owned()).collect();

        let node_id = get_debug_argument_at(&args, 0, "debug_entry", "node_id", get_dht_key)?;

        // Dump routing table entry
        let routing_table = self.network_manager()?.routing_table();
        Ok(routing_table.debug_info_entry(node_id))
    }

    async fn debug_nodeinfo(&self, _args: String) -> Result<String, VeilidAPIError> {
        // Dump routing table entry
        let routing_table = self.network_manager()?.routing_table();
        Ok(routing_table.debug_info_nodeinfo())
    }

    async fn debug_config(&self, args: String) -> Result<String, VeilidAPIError> {
        let config = self.config()?;
        let args = args.trim_start();
        if args.is_empty() {
            return config.get_key_json("");
        }
        let (arg, rest) = args.split_once(' ').unwrap_or((args, ""));
        let rest = rest.trim_start().to_owned();

        // One argument is 'config get'
        if rest.is_empty() {
            return config.get_key_json(arg);
        }

        // More than one argument is 'config set'

        // Must be detached
        if !matches!(
            self.get_state().await?.attachment.state,
            AttachmentState::Detached
        ) {
            apibail_internal!("Must be detached to change config");
        }

        // Change the config key
        config.set_key_json(arg, &rest)?;
        Ok("Config value set".to_owned())
    }

    async fn debug_restart(&self, args: String) -> Result<String, VeilidAPIError> {
        let args = args.trim_start();
        if args.is_empty() {
            return Err(VeilidAPIError::missing_argument("debug_restart", "arg_0"));
        }
        let (arg, _rest) = args.split_once(' ').unwrap_or((args, ""));
        // let rest = rest.trim_start().to_owned();

        if arg == "network" {
            // Must be attached
            if matches!(
                self.get_state().await?.attachment.state,
                AttachmentState::Detached
            ) {
                apibail_internal!("Must be attached to restart network");
            }

            let netman = self.network_manager()?;
            netman.net().restart_network();

            Ok("Network restarted".to_owned())
        } else {
            Err(VeilidAPIError::invalid_argument(
                "debug_restart",
                "arg_1",
                arg,
            ))
        }
    }

    async fn debug_purge(&self, args: String) -> Result<String, VeilidAPIError> {
        let args: Vec<String> = args.split_whitespace().map(|s| s.to_owned()).collect();
        if !args.is_empty() {
            if args[0] == "buckets" {
                // Must be detached
                if matches!(
                    self.get_state().await?.attachment.state,
                    AttachmentState::Detached | AttachmentState::Detaching
                ) {
                    apibail_internal!("Must be detached to purge");
                }
                self.network_manager()?.routing_table().purge_buckets();
                Ok("Buckets purged".to_owned())
            } else if args[0] == "connections" {
                // Purge connection table
                let connection_manager = self.network_manager()?.connection_manager();
                connection_manager.shutdown().await;
                connection_manager.startup().await;

                // Eliminate last_connections from routing table entries
                self.network_manager()?
                    .routing_table()
                    .purge_last_connections();

                Ok("Connections purged".to_owned())
            } else {
                Err(VeilidAPIError::InvalidArgument {
                    context: "debug_purge".to_owned(),
                    argument: "parameter".to_owned(),
                    value: args[0].clone(),
                })
            }
        } else {
            Err(VeilidAPIError::MissingArgument {
                context: "debug_purge".to_owned(),
                argument: "parameter".to_owned(),
            })
        }
    }

    async fn debug_attach(&self, _args: String) -> Result<String, VeilidAPIError> {
        if !matches!(
            self.get_state().await?.attachment.state,
            AttachmentState::Detached
        ) {
            apibail_internal!("Not detached");
        }

        self.attach().await?;

        Ok("Attached".to_owned())
    }

    async fn debug_detach(&self, _args: String) -> Result<String, VeilidAPIError> {
        if matches!(
            self.get_state().await?.attachment.state,
            AttachmentState::Detaching
        ) {
            apibail_internal!("Not attached");
        };

        self.detach().await?;

        Ok("Detached".to_owned())
    }

    async fn debug_contact(&self, args: String) -> Result<String, VeilidAPIError> {
        let args: Vec<String> = args.split_whitespace().map(|s| s.to_owned()).collect();

        let node_id = get_debug_argument_at(&args, 0, "debug_contact", "node_id", get_dht_key)?;

        let network_manager = self.network_manager()?;
        let routing_table = network_manager.routing_table();

        let mut nr = match routing_table.lookup_node_ref(node_id) {
            Some(nr) => nr,
            None => return Ok("Node id not found in routing table".to_owned()),
        };

        if args.len() >= 2 {
            let pt = get_debug_argument_at(
                &args,
                1,
                "debug_contact",
                "protocol_type",
                get_protocol_type,
            )?;
            nr.merge_filter(DialInfoFilter::all().with_protocol_type(pt));
            if args.len() >= 3 {
                let at = get_debug_argument_at(
                    &args,
                    2,
                    "debug_contact",
                    "address_type",
                    get_address_type,
                )?;
                nr.merge_filter(DialInfoFilter::all().with_address_type(at));
            }
        }

        let cm = network_manager.get_contact_method(nr);

        Ok(format!("{:#?}", cm))
    }

    async fn debug_ping(&self, args: String) -> Result<String, VeilidAPIError> {
        let args: Vec<String> = args.split_whitespace().map(|s| s.to_owned()).collect();

        let node_id = get_debug_argument_at(&args, 0, "debug_ping", "node_id", get_dht_key)?;

        let routing_table = self.network_manager()?.routing_table();
        let mut nr = match routing_table.lookup_node_ref(node_id) {
            Some(nr) => nr,
            None => return Ok("Node id not found in routing table".to_owned()),
        };

        let mut ai = 1;
        let mut routing_domain = None;
        while ai < args.len() {
            if let Ok(pt) =
                get_debug_argument_at(&args, ai, "debug_ping", "protocol_type", get_protocol_type)
            {
                nr.merge_filter(DialInfoFilter::all().with_protocol_type(pt));
            } else if let Ok(at) =
                get_debug_argument_at(&args, ai, "debug_ping", "address_type", get_address_type)
            {
                nr.merge_filter(DialInfoFilter::all().with_address_type(at));
            } else if let Ok(rd) = get_debug_argument_at(
                &args,
                ai,
                "debug_ping",
                "routing_domain",
                get_routing_domain,
            ) {
                if routing_domain.is_none() {
                    routing_domain = Some(rd);
                } else {
                    return Ok("Multiple routing domains specified".to_owned());
                }
            } else {
                return Ok(format!("Invalid argument specified: {}", args[ai]));
            }
            ai += 1;
        }

        let rpc = self.network_manager()?.rpc_processor();

        // Dump routing table entry
        let out = match rpc
            .rpc_call_status(routing_domain, nr)
            .await
            .map_err(VeilidAPIError::internal)?
        {
            NetworkResult::Value(v) => v,
            NetworkResult::Timeout => {
                return Ok("Timeout".to_owned());
            }
            NetworkResult::NoConnection(e) => {
                return Ok(format!("NoConnection({})", e));
            }
            NetworkResult::InvalidMessage(e) => {
                return Ok(format!("InvalidMessage({})", e));
            }
        };

        Ok(format!("{:#?}", out))
    }

    pub async fn debug_help(&self, _args: String) -> Result<String, VeilidAPIError> {
        Ok(r#">>> Debug commands:
        help
        buckets [dead|reliable]
        dialinfo
        entries [dead|reliable] [limit]
        entry <node_id>
        nodeinfo
        config [key [new value]]
        purge <buckets|connections>
        attach
        detach
        restart network
        ping <node_id> [protocol_type][address_type][routing_domain]
        contact <node_id> [protocol_type [address_type]]
    "#
        .to_owned())
    }

    pub async fn debug(&self, args: String) -> Result<String, VeilidAPIError> {
        let args = args.trim_start();
        if args.is_empty() {
            // No arguments runs help command
            return self.debug_help("".to_owned()).await;
        }
        let (arg, rest) = args.split_once(' ').unwrap_or((args, ""));
        let rest = rest.trim_start().to_owned();

        if arg == "help" {
            self.debug_help(rest).await
        } else if arg == "buckets" {
            self.debug_buckets(rest).await
        } else if arg == "dialinfo" {
            self.debug_dialinfo(rest).await
        } else if arg == "txtrecord" {
            self.debug_txtrecord(rest).await
        } else if arg == "entries" {
            self.debug_entries(rest).await
        } else if arg == "entry" {
            self.debug_entry(rest).await
        } else if arg == "ping" {
            self.debug_ping(rest).await
        } else if arg == "contact" {
            self.debug_contact(rest).await
        } else if arg == "nodeinfo" {
            self.debug_nodeinfo(rest).await
        } else if arg == "purge" {
            self.debug_purge(rest).await
        } else if arg == "attach" {
            self.debug_attach(rest).await
        } else if arg == "detach" {
            self.debug_detach(rest).await
        } else if arg == "config" {
            self.debug_config(rest).await
        } else if arg == "restart" {
            self.debug_restart(rest).await
        } else {
            Ok(">>> Unknown command\n".to_owned())
        }
    }
}
