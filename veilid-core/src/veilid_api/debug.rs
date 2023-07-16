////////////////////////////////////////////////////////////////
// Debugging

use super::*;
use data_encoding::BASE64URL_NOPAD;
use network_manager::*;
use routing_table::*;

#[derive(Default, Debug)]
struct DebugCache {
    imported_routes: Vec<RouteId>,
}

static DEBUG_CACHE: Mutex<DebugCache> = Mutex::new(DebugCache {
    imported_routes: Vec::new(),
});

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

fn get_string(text: &str) -> Option<String> {
    Some(text.to_owned())
}

fn get_data(text: &str) -> Option<Vec<u8>> {
    if text.starts_with("#") {
        hex::decode(&text[1..]).ok()
    } else if text.starts_with("\"") || text.starts_with("'") {
        json::parse(text)
            .ok()?
            .as_str()
            .map(|x| x.to_owned().as_bytes().to_vec())
    } else {
        Some(text.as_bytes().to_vec())
    }
}

fn get_subkeys(text: &str) -> Option<ValueSubkeyRangeSet> {
    if let Some(n) = get_number(text) {
        Some(ValueSubkeyRangeSet::single(n.try_into().ok()?))
    } else {
        ValueSubkeyRangeSet::from_str(text).ok()
    }
}

fn get_route_id(
    rss: RouteSpecStore,
    allow_allocated: bool,
    allow_remote: bool,
) -> impl Fn(&str) -> Option<RouteId> {
    return move |text: &str| {
        if text.is_empty() {
            return None;
        }
        match RouteId::from_str(text).ok() {
            Some(key) => {
                if allow_allocated {
                    let routes = rss.list_allocated_routes(|k, _| Some(*k));
                    if routes.contains(&key) {
                        return Some(key);
                    }
                }
                if allow_remote {
                    let rroutes = rss.list_remote_routes(|k, _| Some(*k));
                    if rroutes.contains(&key) {
                        return Some(key);
                    }
                }
            }
            None => {
                if allow_allocated {
                    let routes = rss.list_allocated_routes(|k, _| Some(*k));
                    for r in routes {
                        let rkey = r.encode();
                        if rkey.starts_with(text) {
                            return Some(r);
                        }
                    }
                }
                if allow_remote {
                    let routes = rss.list_remote_routes(|k, _| Some(*k));
                    for r in routes {
                        let rkey = r.encode();
                        if rkey.starts_with(text) {
                            return Some(r);
                        }
                    }
                }
            }
        }
        None
    };
}

fn get_dht_schema(text: &str) -> Option<DHTSchema> {
    deserialize_json::<DHTSchema>(text).ok()
}

fn get_safety_selection(routing_table: RoutingTable) -> impl Fn(&str) -> Option<SafetySelection> {
    move |text| {
        let rss = routing_table.route_spec_store();
        let default_route_hop_count =
            routing_table.with_config(|c| c.network.rpc.default_route_hop_count as usize);

        if text.len() != 0 && &text[0..1] == "-" {
            // Unsafe
            let text = &text[1..];
            let seq = get_sequencing(text).unwrap_or_default();
            Some(SafetySelection::Unsafe(seq))
        } else {
            // Safe
            let mut preferred_route = None;
            let mut hop_count = default_route_hop_count;
            let mut stability = Stability::default();
            let mut sequencing = Sequencing::default();
            for x in text.split(",") {
                let x = x.trim();
                if let Some(pr) = get_route_id(rss.clone(), true, false)(x) {
                    preferred_route = Some(pr)
                }
                if let Some(n) = get_number(x) {
                    hop_count = n;
                }
                if let Some(s) = get_stability(x) {
                    stability = s;
                }
                if let Some(s) = get_sequencing(x) {
                    sequencing = s;
                }
            }
            let ss = SafetySpec {
                preferred_route,
                hop_count,
                stability,
                sequencing,
            };
            Some(SafetySelection::Safe(ss))
        }
    }
}

fn get_node_ref_modifiers(mut node_ref: NodeRef) -> impl FnOnce(&str) -> Option<NodeRef> {
    move |text| {
        for m in text.split("/") {
            if let Some(pt) = get_protocol_type(m) {
                node_ref.merge_filter(NodeRefFilter::new().with_protocol_type(pt));
            } else if let Some(at) = get_address_type(m) {
                node_ref.merge_filter(NodeRefFilter::new().with_address_type(at));
            } else if let Some(rd) = get_routing_domain(m) {
                node_ref.merge_filter(NodeRefFilter::new().with_routing_domain(rd));
            } else {
                return None;
            }
        }
        Some(node_ref)
    }
}

fn get_destination(routing_table: RoutingTable) -> impl FnOnce(&str) -> Option<Destination> {
    move |text| {
        // Safety selection
        let (text, ss) = if let Some((first, second)) = text.split_once('+') {
            let ss = get_safety_selection(routing_table.clone())(second)?;
            (first, Some(ss))
        } else {
            (text, None)
        };
        if text.len() == 0 {
            return None;
        }
        if &text[0..1] == "#" {
            let rss = routing_table.route_spec_store();

            // Private route
            let text = &text[1..];

            let private_route = if let Some(prid) = get_route_id(rss.clone(), false, true)(text) {
                let Some(private_route) = rss.best_remote_private_route(&prid) else {
                    return None;
                };
                private_route
            } else {
                let mut dc = DEBUG_CACHE.lock();
                let n = get_number(text)?;
                let prid = dc.imported_routes.get(n)?.clone();
                let Some(private_route) = rss.best_remote_private_route(&prid) else {
                    // Remove imported route
                    dc.imported_routes.remove(n);
                    info!("removed dead imported route {}", n);
                    return None;
                };
                private_route
            };

            Some(Destination::private_route(
                private_route,
                ss.unwrap_or(SafetySelection::Unsafe(Sequencing::default())),
            ))
        } else {
            let (text, mods) = text
                .split_once('/')
                .map(|x| (x.0, Some(x.1)))
                .unwrap_or((text, None));
            if let Some((first, second)) = text.split_once('@') {
                // Relay
                let mut relay_nr = get_node_ref(routing_table.clone())(second)?;
                let target_nr = get_node_ref(routing_table)(first)?;

                if let Some(mods) = mods {
                    relay_nr = get_node_ref_modifiers(relay_nr)(mods)?;
                }

                let mut d = Destination::relay(relay_nr, target_nr);
                if let Some(ss) = ss {
                    d = d.with_safety(ss)
                }

                Some(d)
            } else {
                // Direct
                let mut target_nr = get_node_ref(routing_table)(text)?;

                if let Some(mods) = mods {
                    target_nr = get_node_ref_modifiers(target_nr)(mods)?;
                }

                let mut d = Destination::direct(target_nr);
                if let Some(ss) = ss {
                    d = d.with_safety(ss)
                }

                Some(d)
            }
        }
    }
}

fn get_number(text: &str) -> Option<usize> {
    usize::from_str(text).ok()
}
fn get_typed_key(text: &str) -> Option<TypedKey> {
    TypedKey::from_str(text).ok()
}
fn get_public_key(text: &str) -> Option<PublicKey> {
    PublicKey::from_str(text).ok()
}
fn get_keypair(text: &str) -> Option<KeyPair> {
    KeyPair::from_str(text).ok()
}

fn get_crypto_system_version(crypto: Crypto) -> impl FnOnce(&str) -> Option<CryptoSystemVersion> {
    move |text| {
        let kindstr = get_string(text)?;
        let kind = CryptoKind::from_str(&kindstr).ok()?;
        crypto.get(kind)
    }
}

fn get_dht_key(
    routing_table: RoutingTable,
) -> impl FnOnce(&str) -> Option<(TypedKey, Option<SafetySelection>)> {
    move |text| {
        // Safety selection
        let (text, ss) = if let Some((first, second)) = text.split_once('+') {
            let ss = get_safety_selection(routing_table.clone())(second)?;
            (first, Some(ss))
        } else {
            (text, None)
        };
        if text.len() == 0 {
            return None;
        }

        let key = if let Some(key) = get_public_key(text) {
            TypedKey::new(best_crypto_kind(), key)
        } else if let Some(key) = get_typed_key(text) {
            key
        } else {
            return None;
        };

        Some((key, ss))
    }
}

fn get_node_ref(routing_table: RoutingTable) -> impl FnOnce(&str) -> Option<NodeRef> {
    move |text| {
        let (text, mods) = text
            .split_once('/')
            .map(|x| (x.0, Some(x.1)))
            .unwrap_or((text, None));

        let mut nr = if let Some(key) = get_public_key(text) {
            routing_table.lookup_any_node_ref(key).ok().flatten()?
        } else if let Some(key) = get_typed_key(text) {
            routing_table.lookup_node_ref(key).ok().flatten()?
        } else {
            return None;
        };
        if let Some(mods) = mods {
            nr = get_node_ref_modifiers(nr)(mods)?;
        }
        Some(nr)
    }
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
fn get_sequencing(text: &str) -> Option<Sequencing> {
    let seqtext = text.to_ascii_lowercase();
    if seqtext == "np" {
        Some(Sequencing::NoPreference)
    } else if seqtext == "ord" {
        Some(Sequencing::PreferOrdered)
    } else if seqtext == "*ord" {
        Some(Sequencing::EnsureOrdered)
    } else {
        None
    }
}
fn get_stability(text: &str) -> Option<Stability> {
    let sttext = text.to_ascii_lowercase();
    if sttext == "ll" {
        Some(Stability::LowLatency)
    } else if sttext == "rel" {
        Some(Stability::Reliable)
    } else {
        None
    }
}
fn get_direction_set(text: &str) -> Option<DirectionSet> {
    let dstext = text.to_ascii_lowercase();
    if dstext == "in" {
        Some(Direction::Inbound.into())
    } else if dstext == "out" {
        Some(Direction::Outbound.into())
    } else if dstext == "inout" {
        Some(DirectionSet::all())
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
) -> VeilidAPIResult<T> {
    let Some(val) = getter(value) else {
        apibail_invalid_argument!(context, argument, value);
    };
    Ok(val)
}
fn get_debug_argument_at<T, G: FnOnce(&str) -> Option<T>>(
    debug_args: &[String],
    pos: usize,
    context: &str,
    argument: &str,
    getter: G,
) -> VeilidAPIResult<T> {
    if pos >= debug_args.len() {
        apibail_missing_argument!(context, argument);
    }
    let value = &debug_args[pos];
    let Some(val) = getter(value) else {
        apibail_invalid_argument!(context, argument, value);
    };
    Ok(val)
}

pub fn print_data(data: &[u8], truncate_len: Option<usize>) -> String {
    // check is message body is ascii printable
    let mut printable = true;
    for c in data {
        if *c < 32 || *c > 126 {
            printable = false;
            break;
        }
    }

    let (data, truncated) = if truncate_len.is_some() && data.len() > truncate_len.unwrap() {
        (&data[0..64], true)
    } else {
        (&data[..], false)
    };

    let strdata = if printable {
        format!("{}", String::from_utf8_lossy(&data).to_string())
    } else {
        let sw = shell_words::quote(&String::from_utf8_lossy(&data).to_string()).to_string();
        let h = hex::encode(data);
        if h.len() < sw.len() {
            h
        } else {
            sw
        }
    };
    if truncated {
        format!("{}...", strdata)
    } else {
        strdata
    }
}

impl VeilidAPI {
    async fn debug_buckets(&self, args: String) -> VeilidAPIResult<String> {
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

    async fn debug_dialinfo(&self, _args: String) -> VeilidAPIResult<String> {
        // Dump routing table dialinfo
        let routing_table = self.network_manager()?.routing_table();
        Ok(routing_table.debug_info_dialinfo())
    }
    async fn debug_peerinfo(&self, args: String) -> VeilidAPIResult<String> {
        // Dump routing table peerinfo
        let args: Vec<String> = args.split_whitespace().map(|s| s.to_owned()).collect();
        let routing_table = self.network_manager()?.routing_table();

        let routing_domain = get_debug_argument_at(
            &args,
            0,
            "debug_peerinfo",
            "routing_domain",
            get_routing_domain,
        )
        .ok()
        .unwrap_or(RoutingDomain::PublicInternet);

        Ok(routing_table.debug_info_peerinfo(routing_domain))
    }

    async fn debug_txtrecord(&self, _args: String) -> VeilidAPIResult<String> {
        // Dump routing table txt record
        let routing_table = self.network_manager()?.routing_table();
        Ok(routing_table.debug_info_txtrecord().await)
    }

    async fn debug_keypair(&self, args: String) -> VeilidAPIResult<String> {
        let args: Vec<String> = args.split_whitespace().map(|s| s.to_owned()).collect();
        let crypto = self.crypto()?;

        let vcrypto = get_debug_argument_at(
            &args,
            0,
            "debug_keypair",
            "kind",
            get_crypto_system_version(crypto.clone()),
        )
        .unwrap_or_else(|_| crypto.best());

        // Generate a keypair
        let out = TypedKeyPair::new(vcrypto.kind(), vcrypto.generate_keypair()).to_string();
        Ok(out)
    }

    async fn debug_entries(&self, args: String) -> VeilidAPIResult<String> {
        let args: Vec<String> = args.split_whitespace().map(|s| s.to_owned()).collect();

        let mut min_state = BucketEntryState::Unreliable;
        for arg in args {
            if let Some(ms) = get_bucket_entry_state(&arg) {
                min_state = ms;
            } else {
                apibail_invalid_argument!("debug_entries", "unknown", arg);
            }
        }

        // Dump routing table entries
        let routing_table = self.network_manager()?.routing_table();
        Ok(routing_table.debug_info_entries(min_state))
    }

    async fn debug_entry(&self, args: String) -> VeilidAPIResult<String> {
        let args: Vec<String> = args.split_whitespace().map(|s| s.to_owned()).collect();
        let routing_table = self.network_manager()?.routing_table();

        let node_ref = get_debug_argument_at(
            &args,
            0,
            "debug_entry",
            "node_id",
            get_node_ref(routing_table),
        )?;

        // Dump routing table entry
        let routing_table = self.network_manager()?.routing_table();
        Ok(routing_table.debug_info_entry(node_ref))
    }

    async fn debug_relay(&self, args: String) -> VeilidAPIResult<String> {
        let args: Vec<String> = args.split_whitespace().map(|s| s.to_owned()).collect();
        let routing_table = self.network_manager()?.routing_table();

        let relay_node = get_debug_argument_at(
            &args,
            0,
            "debug_relay",
            "node_id",
            get_node_ref(routing_table),
        )?;

        let routing_domain = get_debug_argument_at(
            &args,
            0,
            "debug_relay",
            "routing_domain",
            get_routing_domain,
        )
        .ok()
        .unwrap_or(RoutingDomain::PublicInternet);

        // Dump routing table entry
        let routing_table = self.network_manager()?.routing_table();
        routing_table
            .edit_routing_domain(routing_domain)
            .set_relay_node(relay_node)
            .commit();
        Ok("Relay changed".to_owned())
    }

    async fn debug_nodeinfo(&self, _args: String) -> VeilidAPIResult<String> {
        // Dump routing table entry
        let routing_table = self.network_manager()?.routing_table();
        Ok(routing_table.debug_info_nodeinfo())
    }

    async fn debug_config(&self, args: String) -> VeilidAPIResult<String> {
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

    async fn debug_restart(&self, args: String) -> VeilidAPIResult<String> {
        let args = args.trim_start();
        if args.is_empty() {
            apibail_missing_argument!("debug_restart", "arg_0");
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
            apibail_invalid_argument!("debug_restart", "arg_1", arg);
        }
    }

    async fn debug_purge(&self, args: String) -> VeilidAPIResult<String> {
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

                // Eliminate last_connections from routing table entries
                self.network_manager()?
                    .routing_table()
                    .purge_last_connections();

                connection_manager.startup().await;

                Ok("Connections purged".to_owned())
            } else if args[0] == "routes" {
                // Purge route spec store
                {
                    let mut dc = DEBUG_CACHE.lock();
                    dc.imported_routes.clear();
                }
                let rss = self.network_manager()?.routing_table().route_spec_store();
                match rss.purge().await {
                    Ok(_) => Ok("Routes purged".to_owned()),
                    Err(e) => Ok(format!("Routes purged but failed to save: {}", e)),
                }
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

    async fn debug_attach(&self, _args: String) -> VeilidAPIResult<String> {
        if !matches!(
            self.get_state().await?.attachment.state,
            AttachmentState::Detached
        ) {
            apibail_internal!("Not detached");
        }

        self.attach().await?;

        Ok("Attached".to_owned())
    }

    async fn debug_detach(&self, _args: String) -> VeilidAPIResult<String> {
        if matches!(
            self.get_state().await?.attachment.state,
            AttachmentState::Detaching
        ) {
            apibail_internal!("Not attached");
        };

        self.detach().await?;

        Ok("Detached".to_owned())
    }

    async fn debug_contact(&self, args: String) -> VeilidAPIResult<String> {
        let args: Vec<String> = args.split_whitespace().map(|s| s.to_owned()).collect();

        let network_manager = self.network_manager()?;
        let routing_table = network_manager.routing_table();

        let node_ref = get_debug_argument_at(
            &args,
            0,
            "debug_contact",
            "node_ref",
            get_node_ref(routing_table),
        )?;

        let cm = network_manager
            .get_node_contact_method(node_ref)
            .map_err(VeilidAPIError::internal)?;

        Ok(format!("{:#?}", cm))
    }

    async fn debug_ping(&self, args: String) -> VeilidAPIResult<String> {
        let netman = self.network_manager()?;
        let routing_table = netman.routing_table();
        let rpc = netman.rpc_processor();

        let args: Vec<String> = args.split_whitespace().map(|s| s.to_owned()).collect();

        let dest = get_debug_argument_at(
            &args,
            0,
            "debug_ping",
            "destination",
            get_destination(routing_table),
        )?;

        // Dump routing table entry
        let out = match rpc
            .rpc_call_status(dest)
            .await
            .map_err(VeilidAPIError::internal)?
        {
            NetworkResult::Value(v) => v,
            r => {
                return Ok(r.to_string());
            }
        };

        Ok(format!("{:#?}", out))
    }

    async fn debug_route_allocate(&self, args: Vec<String>) -> VeilidAPIResult<String> {
        // [ord|*ord] [rel] [<count>] [in|out] [avoid_node_id]

        let netman = self.network_manager()?;
        let routing_table = netman.routing_table();
        let rss = routing_table.route_spec_store();
        let config = self.config().unwrap();
        let default_route_hop_count = {
            let c = config.get();
            c.network.rpc.default_route_hop_count as usize
        };

        let mut ai = 1;
        let mut sequencing = Sequencing::default();
        let mut stability = Stability::default();
        let mut hop_count = default_route_hop_count;
        let mut directions = DirectionSet::all();

        while ai < args.len() {
            if let Ok(seq) =
                get_debug_argument_at(&args, ai, "debug_route", "sequencing", get_sequencing)
            {
                sequencing = seq;
            } else if let Ok(sta) =
                get_debug_argument_at(&args, ai, "debug_route", "stability", get_stability)
            {
                stability = sta;
            } else if let Ok(hc) =
                get_debug_argument_at(&args, ai, "debug_route", "hop_count", get_number)
            {
                hop_count = hc;
            } else if let Ok(ds) =
                get_debug_argument_at(&args, ai, "debug_route", "direction_set", get_direction_set)
            {
                directions = ds;
            } else {
                return Ok(format!("Invalid argument specified: {}", args[ai]));
            }
            ai += 1;
        }

        // Allocate route
        let out = match rss.allocate_route(
            &VALID_CRYPTO_KINDS,
            stability,
            sequencing,
            hop_count,
            directions,
            &[],
        ) {
            Ok(Some(v)) => format!("{}", v),
            Ok(None) => format!("<unavailable>"),
            Err(e) => {
                format!("Route allocation failed: {}", e)
            }
        };

        Ok(out)
    }
    async fn debug_route_release(&self, args: Vec<String>) -> VeilidAPIResult<String> {
        // <route id>
        let netman = self.network_manager()?;
        let routing_table = netman.routing_table();
        let rss = routing_table.route_spec_store();

        let route_id = get_debug_argument_at(
            &args,
            1,
            "debug_route",
            "route_id",
            get_route_id(rss.clone(), true, true),
        )?;

        // Release route
        let out = match rss.release_route(route_id) {
            true => {
                // release imported
                let mut dc = DEBUG_CACHE.lock();
                for (n, ir) in dc.imported_routes.iter().enumerate() {
                    if *ir == route_id {
                        dc.imported_routes.remove(n);
                        break;
                    }
                }
                "Released".to_owned()
            }
            false => "Route does not exist".to_owned(),
        };

        Ok(out)
    }
    async fn debug_route_publish(&self, args: Vec<String>) -> VeilidAPIResult<String> {
        // <route id> [full]
        let netman = self.network_manager()?;
        let routing_table = netman.routing_table();
        let rss = routing_table.route_spec_store();

        let route_id = get_debug_argument_at(
            &args,
            1,
            "debug_route",
            "route_id",
            get_route_id(rss.clone(), true, false),
        )?;
        let full = {
            if args.len() > 2 {
                let full_val = get_debug_argument_at(&args, 2, "debug_route", "full", get_string)?
                    .to_ascii_lowercase();
                if full_val == "full" {
                    true
                } else {
                    apibail_invalid_argument!("debug_route", "full", full_val);
                }
            } else {
                false
            }
        };

        // Publish route
        let out = match rss.assemble_private_routes(&route_id, Some(!full)) {
            Ok(private_routes) => {
                if let Err(e) = rss.mark_route_published(&route_id, true) {
                    return Ok(format!("Couldn't mark route published: {}", e));
                }
                // Convert to blob
                let blob_data = RouteSpecStore::private_routes_to_blob(&private_routes)
                    .map_err(VeilidAPIError::internal)?;
                let out = BASE64URL_NOPAD.encode(&blob_data);
                info!(
                    "Published route {} as {} bytes:\n{}",
                    route_id.encode(),
                    blob_data.len(),
                    out
                );
                format!("Published route {}", route_id.encode())
            }
            Err(e) => {
                format!("Couldn't assemble private route: {}", e)
            }
        };

        Ok(out)
    }
    async fn debug_route_unpublish(&self, args: Vec<String>) -> VeilidAPIResult<String> {
        // <route id>
        let netman = self.network_manager()?;
        let routing_table = netman.routing_table();
        let rss = routing_table.route_spec_store();

        let route_id = get_debug_argument_at(
            &args,
            1,
            "debug_route",
            "route_id",
            get_route_id(rss.clone(), true, false),
        )?;

        // Unpublish route
        let out = if let Err(e) = rss.mark_route_published(&route_id, false) {
            return Ok(format!("Couldn't mark route unpublished: {}", e));
        } else {
            "Route unpublished".to_owned()
        };
        Ok(out)
    }
    async fn debug_route_print(&self, args: Vec<String>) -> VeilidAPIResult<String> {
        // <route id>
        let netman = self.network_manager()?;
        let routing_table = netman.routing_table();
        let rss = routing_table.route_spec_store();

        let route_id = get_debug_argument_at(
            &args,
            1,
            "debug_route",
            "route_id",
            get_route_id(rss.clone(), true, true),
        )?;

        match rss.debug_route(&route_id) {
            Some(s) => Ok(s),
            None => Ok("Route does not exist".to_owned()),
        }
    }
    async fn debug_route_list(&self, _args: Vec<String>) -> VeilidAPIResult<String> {
        //
        let netman = self.network_manager()?;
        let routing_table = netman.routing_table();
        let rss = routing_table.route_spec_store();

        let routes = rss.list_allocated_routes(|k, _| Some(*k));
        let mut out = format!("Allocated Routes: (count = {}):\n", routes.len());
        for r in routes {
            out.push_str(&format!("{}\n", r.encode()));
        }

        let remote_routes = rss.list_remote_routes(|k, _| Some(*k));
        out.push_str(&format!(
            "Remote Routes: (count = {}):\n",
            remote_routes.len()
        ));
        for r in remote_routes {
            out.push_str(&format!("{}\n", r.encode()));
        }

        Ok(out)
    }
    async fn debug_route_import(&self, args: Vec<String>) -> VeilidAPIResult<String> {
        // <blob>

        let blob = get_debug_argument_at(&args, 1, "debug_route", "blob", get_string)?;
        let blob_dec = BASE64URL_NOPAD
            .decode(blob.as_bytes())
            .map_err(VeilidAPIError::generic)?;
        let rss = self.routing_table()?.route_spec_store();
        let route_id = rss
            .import_remote_private_route(blob_dec)
            .map_err(VeilidAPIError::generic)?;

        let mut dc = DEBUG_CACHE.lock();
        let n = dc.imported_routes.len();
        let out = format!("Private route #{} imported: {}", n, route_id);
        dc.imported_routes.push(route_id);

        return Ok(out);
    }

    async fn debug_route_test(&self, args: Vec<String>) -> VeilidAPIResult<String> {
        // <route id>
        let netman = self.network_manager()?;
        let routing_table = netman.routing_table();
        let rss = routing_table.route_spec_store();

        let route_id = get_debug_argument_at(
            &args,
            1,
            "debug_route",
            "route_id",
            get_route_id(rss.clone(), true, true),
        )?;

        let success = rss
            .test_route(route_id)
            .await
            .map_err(VeilidAPIError::internal)?;

        let out = if success {
            "SUCCESS".to_owned()
        } else {
            "FAILED".to_owned()
        };

        return Ok(out);
    }

    async fn debug_route(&self, args: String) -> VeilidAPIResult<String> {
        let args: Vec<String> = args.split_whitespace().map(|s| s.to_owned()).collect();

        let command = get_debug_argument_at(&args, 0, "debug_route", "command", get_string)?;

        if command == "allocate" {
            self.debug_route_allocate(args).await
        } else if command == "release" {
            self.debug_route_release(args).await
        } else if command == "publish" {
            self.debug_route_publish(args).await
        } else if command == "unpublish" {
            self.debug_route_unpublish(args).await
        } else if command == "print" {
            self.debug_route_print(args).await
        } else if command == "list" {
            self.debug_route_list(args).await
        } else if command == "import" {
            self.debug_route_import(args).await
        } else if command == "test" {
            self.debug_route_test(args).await
        } else {
            Ok(">>> Unknown command\n".to_owned())
        }
    }

    async fn debug_record_list(&self, args: Vec<String>) -> VeilidAPIResult<String> {
        // <local|remote>
        let storage_manager = self.storage_manager()?;

        let scope = get_debug_argument_at(&args, 1, "debug_record_list", "scope", get_string)?;
        let out = match scope.as_str() {
            "local" => {
                let mut out = format!("Local Records:\n");
                out += &storage_manager.debug_local_records().await;
                out
            }
            "remote" => {
                let mut out = format!("Remote Records:\n");
                out += &storage_manager.debug_remote_records().await;
                out
            }
            _ => "Invalid scope\n".to_owned(),
        };
        return Ok(out);
    }

    async fn debug_record_purge(&self, args: Vec<String>) -> VeilidAPIResult<String> {
        // <local|remote> [bytes]
        let storage_manager = self.storage_manager()?;

        let scope = get_debug_argument_at(&args, 1, "debug_record_purge", "scope", get_string)?;
        let bytes = get_debug_argument_at(&args, 2, "debug_record_purge", "bytes", get_number).ok();
        let out = match scope.as_str() {
            "local" => storage_manager.purge_local_records(bytes).await,
            "remote" => storage_manager.purge_remote_records(bytes).await,
            _ => "Invalid scope\n".to_owned(),
        };
        return Ok(out);
    }

    async fn debug_record_create(&self, args: Vec<String>) -> VeilidAPIResult<String> {
        let netman = self.network_manager()?;
        let routing_table = netman.routing_table();
        let crypto = self.crypto()?;

        let schema = get_debug_argument_at(
            &args,
            1,
            "debug_record_create",
            "dht_schema",
            get_dht_schema,
        )
        .unwrap_or_else(|_| DHTSchema::dflt(1));

        let csv = get_debug_argument_at(
            &args,
            2,
            "debug_record_create",
            "kind",
            get_crypto_system_version(crypto.clone()),
        )
        .unwrap_or_else(|_| crypto.best());

        let ss = get_debug_argument_at(
            &args,
            3,
            "debug_record_create",
            "safety_selection",
            get_safety_selection(routing_table),
        )
        .ok();

        // Get routing context with optional privacy
        let rc = self.routing_context();
        let rc = if let Some(ss) = ss {
            let rcp = match rc.with_custom_privacy(ss) {
                Err(e) => return Ok(format!("Can't use safety selection: {}", e)),
                Ok(v) => v,
            };
            rcp
        } else {
            rc
        };

        // Do a record get
        let record = match rc.create_dht_record(schema, Some(csv.kind())).await {
            Err(e) => return Ok(format!("Can't open DHT record: {}", e)),
            Ok(v) => v,
        };
        match rc.close_dht_record(*record.key()).await {
            Err(e) => return Ok(format!("Can't close DHT record: {}", e)),
            Ok(v) => v,
        };
        debug!("DHT Record Created:\n{:#?}", record);
        return Ok(format!("{:?}", record));
    }

    async fn debug_record_get(&self, args: Vec<String>) -> VeilidAPIResult<String> {
        let netman = self.network_manager()?;
        let routing_table = netman.routing_table();

        let (key, ss) = get_debug_argument_at(
            &args,
            1,
            "debug_record_get",
            "key",
            get_dht_key(routing_table),
        )?;
        let subkey = get_debug_argument_at(&args, 2, "debug_record_get", "subkey", get_number)?;
        let force_refresh = if args.len() >= 4 {
            Some(get_debug_argument_at(
                &args,
                3,
                "debug_record_get",
                "force_refresh",
                get_string,
            )?)
        } else {
            None
        };

        let force_refresh = if let Some(force_refresh) = force_refresh {
            if &force_refresh == "force" {
                true
            } else {
                return Ok(format!("Unknown force: {}", force_refresh));
            }
        } else {
            false
        };

        // Get routing context with optional privacy
        let rc = self.routing_context();
        let rc = if let Some(ss) = ss {
            let rcp = match rc.with_custom_privacy(ss) {
                Err(e) => return Ok(format!("Can't use safety selection: {}", e)),
                Ok(v) => v,
            };
            rcp
        } else {
            rc
        };

        // Do a record get
        let _record = match rc.open_dht_record(key, None).await {
            Err(e) => return Ok(format!("Can't open DHT record: {}", e)),
            Ok(v) => v,
        };
        let value = match rc
            .get_dht_value(key, subkey as ValueSubkey, force_refresh)
            .await
        {
            Err(e) => {
                match rc.close_dht_record(key).await {
                    Err(e) => {
                        return Ok(format!(
                            "Can't get DHT value and can't close DHT record: {}",
                            e
                        ))
                    }
                    Ok(v) => v,
                };
                return Ok(format!("Can't get DHT value: {}", e));
            }
            Ok(v) => v,
        };
        let out = if let Some(value) = value {
            format!("{:?}", value)
        } else {
            "No value data returned".to_owned()
        };
        match rc.close_dht_record(key).await {
            Err(e) => return Ok(format!("Can't close DHT record: {}", e)),
            Ok(v) => v,
        };
        return Ok(out);
    }

    async fn debug_record_set(&self, args: Vec<String>) -> VeilidAPIResult<String> {
        let netman = self.network_manager()?;
        let routing_table = netman.routing_table();

        let (key, ss) = get_debug_argument_at(
            &args,
            1,
            "debug_record_set",
            "key",
            get_dht_key(routing_table),
        )?;
        let subkey = get_debug_argument_at(&args, 2, "debug_record_set", "subkey", get_number)?;
        let writer = get_debug_argument_at(&args, 3, "debug_record_set", "writer", get_keypair)?;
        let data = get_debug_argument_at(&args, 4, "debug_record_set", "data", get_data)?;

        // Get routing context with optional privacy
        let rc = self.routing_context();
        let rc = if let Some(ss) = ss {
            let rcp = match rc.with_custom_privacy(ss) {
                Err(e) => return Ok(format!("Can't use safety selection: {}", e)),
                Ok(v) => v,
            };
            rcp
        } else {
            rc
        };

        // Do a record get
        let _record = match rc.open_dht_record(key, Some(writer)).await {
            Err(e) => return Ok(format!("Can't open DHT record: {}", e)),
            Ok(v) => v,
        };
        let value = match rc.set_dht_value(key, subkey as ValueSubkey, data).await {
            Err(e) => {
                match rc.close_dht_record(key).await {
                    Err(e) => {
                        return Ok(format!(
                            "Can't set DHT value and can't close DHT record: {}",
                            e
                        ))
                    }
                    Ok(v) => v,
                };
                return Ok(format!("Can't set DHT value: {}", e));
            }
            Ok(v) => v,
        };
        let out = if let Some(value) = value {
            format!("{:?}", value)
        } else {
            "No value data returned".to_owned()
        };
        match rc.close_dht_record(key).await {
            Err(e) => return Ok(format!("Can't close DHT record: {}", e)),
            Ok(v) => v,
        };
        return Ok(out);
    }

    async fn debug_record_delete(&self, args: Vec<String>) -> VeilidAPIResult<String> {
        let key = get_debug_argument_at(&args, 1, "debug_record_delete", "key", get_typed_key)?;

        // Do a record delete
        let rc = self.routing_context();
        match rc.delete_dht_record(key).await {
            Err(e) => return Ok(format!("Can't delete DHT record: {}", e)),
            Ok(v) => v,
        };
        Ok(format!("DHT record deleted"))
    }

    async fn debug_record_info(&self, args: Vec<String>) -> VeilidAPIResult<String> {
        let storage_manager = self.storage_manager()?;

        let key = get_debug_argument_at(&args, 1, "debug_record_info", "key", get_typed_key)?;

        let subkey =
            get_debug_argument_at(&args, 2, "debug_record_info", "subkey", get_number).ok();

        let out = if let Some(subkey) = subkey {
            let li = storage_manager
                .debug_local_record_subkey_info(key, subkey as ValueSubkey)
                .await;
            let ri = storage_manager
                .debug_remote_record_subkey_info(key, subkey as ValueSubkey)
                .await;
            format!(
                "Local Subkey Info:\n{}\n\nRemote Subkey Info:\n{}\n",
                li, ri
            )
        } else {
            let li = storage_manager.debug_local_record_info(key).await;
            let ri = storage_manager.debug_remote_record_info(key).await;
            format!("Local Info:\n{}\n\nRemote Info:\n{}\n", li, ri)
        };
        return Ok(out);
    }

    async fn debug_record(&self, args: String) -> VeilidAPIResult<String> {
        let args: Vec<String> =
            shell_words::split(&args).map_err(|e| VeilidAPIError::parse_error(e, args))?;

        let command = get_debug_argument_at(&args, 0, "debug_record", "command", get_string)?;

        if command == "list" {
            self.debug_record_list(args).await
        } else if command == "purge" {
            self.debug_record_purge(args).await
        } else if command == "create" {
            self.debug_record_create(args).await
        } else if command == "get" {
            self.debug_record_get(args).await
        } else if command == "set" {
            self.debug_record_set(args).await
        } else if command == "delete" {
            self.debug_record_delete(args).await
        } else if command == "info" {
            self.debug_record_info(args).await
        } else {
            Ok(">>> Unknown command\n".to_owned())
        }
    }

    pub async fn debug_help(&self, _args: String) -> VeilidAPIResult<String> {
        Ok(r#"buckets [dead|reliable]
dialinfo
peerinfo [routingdomain]
entries [dead|reliable]
entry <node>
nodeinfo
config [configkey [new value]]
txtrecord
keypair
purge <buckets|connections|routes>
attach
detach
restart network
contact <node>[<modifiers>]
ping <destination>
relay <relay> [public|local]
route allocate [ord|*ord] [rel] [<count>] [in|out]
      release <route>
      publish <route> [full]
      unpublish <route>
      print <route>
      list
      import <blob>
      test <route>
record list <local|remote>
       purge <local|remote> [bytes]
       create <dhtschema> [<cryptokind> [<safety>]]
       set <key>[+<safety>] <subkey> <writer> <data> 
       get <key>[+<safety>] <subkey> [force]
       delete <key>
       info <key> [subkey]
--------------------------------------------------------------------
<key> is: VLD0:GsgXCRPrzSK6oBNgxhNpm-rTYFd02R0ySx6j9vbQBG4
    * also <node>, <relay>, <target>, <route>
<configkey> is: dot path like network.protocol.udp.enabled
<destination> is:
    * direct:  <node>[+<safety>][<modifiers>]
    * relay:   <relay>@<target>[+<safety>][<modifiers>]
    * private: #<id>[+<safety>]
<safety> is:
    * unsafe: -[ord|*ord]
    * safe: [route][,ord|*ord][,rel][,<count>]
<modifiers> is: [/<protocoltype>][/<addresstype>][/<routingdomain>]
<protocoltype> is: udp|tcp|ws|wss
<addresstype> is: ipv4|ipv6
<routingdomain> is: public|local
<cryptokind> is: VLD0
<dhtschema> is: a json dht schema, default is '{"kind":"DFLT","o_cnt":1}'
<subkey> is: a number: 2
<subkeys> is: 
    * a number: 2
    * a comma-separated inclusive range list: 1..=3,5..=8
<data> is:
    * a single-word string: foobar
    * a shell-quoted string: "foo\nbar\n"
    * a '#' followed by hex data: #12AB34CD...
"#
        .to_owned())
    }

    pub async fn debug(&self, args: String) -> VeilidAPIResult<String> {
        let res = {
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
            } else if arg == "peerinfo" {
                self.debug_peerinfo(rest).await
            } else if arg == "txtrecord" {
                self.debug_txtrecord(rest).await
            } else if arg == "keypair" {
                self.debug_keypair(rest).await
            } else if arg == "entries" {
                self.debug_entries(rest).await
            } else if arg == "entry" {
                self.debug_entry(rest).await
            } else if arg == "relay" {
                self.debug_relay(rest).await
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
            } else if arg == "route" {
                self.debug_route(rest).await
            } else if arg == "record" {
                self.debug_record(rest).await
            } else {
                Err(VeilidAPIError::generic("Unknown server debug command"))
            }
        };
        res
    }
}
