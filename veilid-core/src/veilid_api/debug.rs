////////////////////////////////////////////////////////////////
// Debugging

use super::*;
use data_encoding::BASE64URL_NOPAD;
use hashlink::LinkedHashMap;
use network_manager::*;
use once_cell::sync::Lazy;
use routing_table::*;
use std::fmt::Write;

#[derive(Default)]
pub(crate) struct DebugCache {
    pub imported_routes: Vec<RouteId>,
    pub opened_record_contexts: Lazy<LinkedHashMap<TypedKey, RoutingContext>>,
}

pub fn format_opt_ts(ts: Option<TimestampDuration>) -> String {
    let Some(ts) = ts else {
        return "---".to_owned();
    };
    let ts = ts.as_u64();
    let secs = timestamp_to_secs(ts);
    if secs >= 1.0 {
        format!("{:.2}s", timestamp_to_secs(ts))
    } else {
        format!("{:.2}ms", timestamp_to_secs(ts) * 1000.0)
    }
}

pub fn format_opt_bps(bps: Option<ByteCount>) -> String {
    let Some(bps) = bps else {
        return "---".to_owned();
    };
    let bps = bps.as_u64();
    if bps >= 1024u64 * 1024u64 * 1024u64 {
        format!("{:.2}GB/s", (bps / (1024u64 * 1024u64)) as f64 / 1024.0)
    } else if bps >= 1024u64 * 1024u64 {
        format!("{:.2}MB/s", (bps / 1024u64) as f64 / 1024.0)
    } else if bps >= 1024u64 {
        format!("{:.2}KB/s", bps as f64 / 1024.0)
    } else {
        format!("{:.2}B/s", bps as f64)
    }
}

fn get_bucket_entry_state(text: &str) -> Option<BucketEntryState> {
    if text == "punished" {
        Some(BucketEntryState::Punished)
    } else if text == "dead" {
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
    if let Some(stripped_text) = text.strip_prefix('#') {
        hex::decode(stripped_text).ok()
    } else if text.starts_with('"') || text.starts_with('\'') {
        json::parse(text)
            .ok()?
            .as_str()
            .map(|x| x.to_owned().as_bytes().to_vec())
    } else {
        Some(text.as_bytes().to_vec())
    }
}

fn get_subkeys(text: &str) -> Option<ValueSubkeyRangeSet> {
    if let Some(n) = get_number::<u32>(text) {
        Some(ValueSubkeyRangeSet::single(n))
    } else {
        ValueSubkeyRangeSet::from_str(text).ok()
    }
}

fn get_dht_report_scope(text: &str) -> Option<DHTReportScope> {
    match text.to_ascii_lowercase().trim() {
        "local" => Some(DHTReportScope::Local),
        "syncget" => Some(DHTReportScope::SyncGet),
        "syncset" => Some(DHTReportScope::SyncSet),
        "updateget" => Some(DHTReportScope::UpdateGet),
        "updateset" => Some(DHTReportScope::UpdateSet),
        _ => None,
    }
}

fn get_route_id(
    registry: VeilidComponentRegistry,
    allow_allocated: bool,
    allow_remote: bool,
) -> impl Fn(&str) -> Option<RouteId> {
    move |text: &str| {
        if text.is_empty() {
            return None;
        }
        let routing_table = registry.routing_table();
        let rss = routing_table.route_spec_store();

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
    }
}

fn get_dht_schema(text: &str) -> Option<VeilidAPIResult<DHTSchema>> {
    if text.is_empty() {
        return None;
    }
    if let Ok(n) = u16::from_str(text) {
        return Some(DHTSchema::dflt(n));
    }
    Some(deserialize_json::<DHTSchema>(text))
}

fn get_safety_selection(
    registry: VeilidComponentRegistry,
) -> impl Fn(&str) -> Option<SafetySelection> {
    move |text| {
        let default_route_hop_count = registry
            .config()
            .with(|c| c.network.rpc.default_route_hop_count as usize);

        if !text.is_empty() && &text[0..1] == "-" {
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
            for x in text.split(',') {
                let x = x.trim();
                if let Some(pr) = get_route_id(registry.clone(), true, false)(x) {
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

fn get_node_ref_modifiers(node_ref: NodeRef) -> impl FnOnce(&str) -> Option<FilteredNodeRef> {
    move |text| {
        let mut node_ref = node_ref.default_filtered();
        for m in text.split('/') {
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

fn get_number<T: num_traits::Num + FromStr>(text: &str) -> Option<T> {
    T::from_str(text).ok()
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

fn get_crypto_system_version<'a>(
    crypto: &'a Crypto,
) -> impl FnOnce(&str) -> Option<CryptoSystemGuard<'a>> {
    move |text| {
        let kindstr = get_string(text)?;
        let kind = CryptoKind::from_str(&kindstr).ok()?;
        crypto.get(kind)
    }
}

fn get_dht_key_no_safety(text: &str) -> Option<TypedKey> {
    let key = if let Some(key) = get_public_key(text) {
        TypedKey::new(best_crypto_kind(), key)
    } else if let Some(key) = get_typed_key(text) {
        key
    } else {
        return None;
    };

    Some(key)
}

fn get_dht_key(
    registry: VeilidComponentRegistry,
) -> impl FnOnce(&str) -> Option<(TypedKey, Option<SafetySelection>)> {
    move |text| {
        // Safety selection
        let (text, ss) = if let Some((first, second)) = text.split_once('+') {
            let ss = get_safety_selection(registry)(second)?;
            (first, Some(ss))
        } else {
            (text, None)
        };
        if text.is_empty() {
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

fn resolve_node_ref(
    registry: VeilidComponentRegistry,
    safety_selection: SafetySelection,
) -> impl FnOnce(&str) -> SendPinBoxFuture<Option<NodeRef>> {
    move |text| {
        let text = text.to_owned();
        Box::pin(async move {
            let nr = if let Some(key) = get_public_key(&text) {
                let node_id = TypedKey::new(best_crypto_kind(), key);
                registry
                    .rpc_processor()
                    .resolve_node(node_id, safety_selection)
                    .await
                    .ok()
                    .flatten()?
            } else if let Some(node_id) = get_typed_key(&text) {
                registry
                    .rpc_processor()
                    .resolve_node(node_id, safety_selection)
                    .await
                    .ok()
                    .flatten()?
            } else {
                return None;
            };
            Some(nr)
        })
    }
}

fn resolve_filtered_node_ref(
    registry: VeilidComponentRegistry,
    safety_selection: SafetySelection,
) -> impl FnOnce(&str) -> SendPinBoxFuture<Option<FilteredNodeRef>> {
    move |text| {
        let text = text.to_owned();
        Box::pin(async move {
            let (text, mods) = text
                .split_once('/')
                .map(|x| (x.0, Some(x.1)))
                .unwrap_or((&text, None));

            let nr = if let Some(key) = get_public_key(text) {
                let node_id = TypedKey::new(best_crypto_kind(), key);
                registry
                    .rpc_processor()
                    .resolve_node(node_id, safety_selection)
                    .await
                    .ok()
                    .flatten()?
            } else if let Some(node_id) = get_typed_key(text) {
                registry
                    .rpc_processor()
                    .resolve_node(node_id, safety_selection)
                    .await
                    .ok()
                    .flatten()?
            } else {
                return None;
            };
            if let Some(mods) = mods {
                Some(get_node_ref_modifiers(nr)(mods)?)
            } else {
                Some(nr.default_filtered())
            }
        })
    }
}

fn get_node_ref(registry: VeilidComponentRegistry) -> impl FnOnce(&str) -> Option<NodeRef> {
    move |text| {
        let routing_table = registry.routing_table();
        let nr = if let Some(key) = get_public_key(text) {
            routing_table.lookup_any_node_ref(key).ok().flatten()?
        } else if let Some(node_id) = get_typed_key(text) {
            routing_table.lookup_node_ref(node_id).ok().flatten()?
        } else {
            return None;
        };
        Some(nr)
    }
}

fn get_filtered_node_ref(
    registry: VeilidComponentRegistry,
) -> impl FnOnce(&str) -> Option<FilteredNodeRef> {
    move |text| {
        let routing_table = registry.routing_table();

        // Safety selection
        let (text, seq) = if let Some((first, second)) = text.split_once('+') {
            let seq = get_sequencing(second)?;
            (first, Some(seq))
        } else {
            (text, None)
        };
        if text.is_empty() {
            return None;
        }

        let (text, mods) = text
            .split_once('/')
            .map(|x| (x.0, Some(x.1)))
            .unwrap_or((text, None));

        let nr = if let Some(key) = get_public_key(text) {
            routing_table.lookup_any_node_ref(key).ok().flatten()?
        } else if let Some(node_id) = get_typed_key(text) {
            routing_table.lookup_node_ref(node_id).ok().flatten()?
        } else {
            return None;
        };
        let nr = if let Some(mods) = mods {
            get_node_ref_modifiers(nr)(mods)?
        } else {
            nr.default_filtered()
        };
        if let Some(seq) = seq {
            Some(nr.sequencing_clone(seq))
        } else {
            Some(nr)
        }
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

fn get_published(text: &str) -> Option<bool> {
    let ptext = text.to_ascii_lowercase();
    if ptext == "published" {
        Some(true)
    } else if ptext == "current" {
        Some(false)
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

async fn async_get_debug_argument<T, G: FnOnce(&str) -> SendPinBoxFuture<Option<T>>>(
    value: &str,
    context: &str,
    argument: &str,
    getter: G,
) -> VeilidAPIResult<T> {
    let Some(val) = getter(value).await else {
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

async fn async_get_debug_argument_at<T, G: FnOnce(&str) -> SendPinBoxFuture<Option<T>>>(
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
    let Some(val) = getter(value).await else {
        apibail_invalid_argument!(context, argument, value);
    };
    Ok(val)
}

pub fn print_data(data: &[u8], truncate_len: Option<usize>) -> String {
    // check if message body is ascii printable
    let mut printable = true;
    for c in data {
        if *c < 32 || *c > 126 {
            printable = false;
            break;
        }
    }

    let (data, truncated) = if let Some(truncate_len) = truncate_len {
        if data.len() > truncate_len {
            (&data[0..truncate_len], true)
        } else {
            (data, false)
        }
    } else {
        (data, false)
    };

    let strdata = if printable {
        String::from_utf8_lossy(data).to_string()
    } else {
        let sw = shell_words::quote(String::from_utf8_lossy(data).as_ref()).to_string();
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
        let routing_table = self.core_context()?.routing_table();
        Ok(routing_table.debug_info_buckets(min_state))
    }

    async fn debug_dialinfo(&self, _args: String) -> VeilidAPIResult<String> {
        // Dump routing table dialinfo
        let routing_table = self.core_context()?.routing_table();
        Ok(routing_table.debug_info_dialinfo())
    }
    async fn debug_peerinfo(&self, args: String) -> VeilidAPIResult<String> {
        // Dump routing table peerinfo
        let args: Vec<String> = args.split_whitespace().map(|s| s.to_owned()).collect();
        let routing_table = self.core_context()?.routing_table();

        let mut ai = 0;
        let mut opt_routing_domain = None;
        let mut opt_published = None;

        while ai < args.len() {
            if let Ok(routing_domain) = get_debug_argument_at(
                &args,
                ai,
                "debug_peerinfo",
                "routing_domain",
                get_routing_domain,
            ) {
                opt_routing_domain = Some(routing_domain);
            } else if let Ok(published) =
                get_debug_argument_at(&args, ai, "debug_peerinfo", "published", get_published)
            {
                opt_published = Some(published);
            }
            ai += 1;
        }

        let routing_domain = opt_routing_domain.unwrap_or(RoutingDomain::PublicInternet);
        let published = opt_published.unwrap_or(true);

        Ok(routing_table.debug_info_peerinfo(routing_domain, published))
    }

    async fn debug_txtrecord(&self, _args: String) -> VeilidAPIResult<String> {
        // Dump routing table txt record
        let routing_table = self.core_context()?.routing_table();
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
            get_crypto_system_version(&crypto),
        )
        .unwrap_or_else(|_| crypto.best());

        // Generate a keypair
        let out = TypedKeyPair::new(vcrypto.kind(), vcrypto.generate_keypair()).to_string();
        Ok(out)
    }

    async fn debug_entries(&self, args: String) -> VeilidAPIResult<String> {
        let args: Vec<String> = args.split_whitespace().map(|s| s.to_owned()).collect();

        let mut min_state = BucketEntryState::Unreliable;
        let mut capabilities = vec![];
        let mut fastest = false;
        for arg in args {
            if let Some(ms) = get_bucket_entry_state(&arg) {
                min_state = ms;
            } else if arg == "fastest" {
                fastest = true;
            } else {
                for cap in arg.split(',') {
                    if let Ok(capfcc) = FourCC::from_str(cap) {
                        capabilities.push(capfcc);
                    } else {
                        apibail_invalid_argument!("debug_entries", "unknown", arg);
                    }
                }
            }
        }

        // Dump routing table entries
        let routing_table = self.core_context()?.routing_table();
        Ok(match fastest {
            true => routing_table.debug_info_entries_fastest(min_state, capabilities, 100000),
            false => routing_table.debug_info_entries(min_state, capabilities),
        })
    }

    async fn debug_entry(&self, args: String) -> VeilidAPIResult<String> {
        let args: Vec<String> = args.split_whitespace().map(|s| s.to_owned()).collect();
        let registry = self.core_context()?.registry();

        let node_ref = get_debug_argument_at(
            &args,
            0,
            "debug_entry",
            "node_id",
            get_node_ref(registry.clone()),
        )?;

        // Dump routing table entry
        Ok(registry.routing_table().debug_info_entry(node_ref))
    }

    async fn debug_relay(&self, args: String) -> VeilidAPIResult<String> {
        let args: Vec<String> = args.split_whitespace().map(|s| s.to_owned()).collect();
        let registry = self.core_context()?.registry();

        let relay_node = async_get_debug_argument_at(
            &args,
            0,
            "debug_relay",
            "node_id",
            resolve_node_ref(registry.clone(), SafetySelection::default()),
        )
        .await
        .ok();

        let routing_domain = get_debug_argument_at(
            &args,
            1,
            "debug_relay",
            "routing_domain",
            get_routing_domain,
        )
        .ok()
        .unwrap_or(RoutingDomain::PublicInternet);

        // Dump routing table entry
        let routing_table = registry.routing_table();
        match routing_domain {
            RoutingDomain::LocalNetwork => {
                let mut editor = routing_table.edit_local_network_routing_domain();
                if editor.set_relay_node(relay_node).commit(true).await {
                    editor.publish();
                }
            }
            RoutingDomain::PublicInternet => {
                let mut editor = routing_table.edit_public_internet_routing_domain();
                if editor.set_relay_node(relay_node).commit(true).await {
                    editor.publish();
                }
            }
        }

        Ok("Relay changed".to_owned())
    }

    async fn debug_nodeinfo(&self, _args: String) -> VeilidAPIResult<String> {
        // Dump routing table entry
        let registry = self.core_context()?.registry();
        let nodeinfo = registry.routing_table().debug_info_nodeinfo();

        // Dump core state
        let state = self.get_state().await?;

        let mut peertable = format!(
            "Recent Peers: {} (max {})\n",
            state.network.peers.len(),
            RECENT_PEERS_TABLE_SIZE
        );
        for peer in state.network.peers {
            peertable += &format!(
                "   {} | {} | {} | {} down | {} up\n",
                peer.node_ids.first().unwrap(),
                peer.peer_address,
                format_opt_ts(peer.peer_stats.latency.map(|l| l.average)),
                format_opt_bps(Some(peer.peer_stats.transfer.down.average)),
                format_opt_bps(Some(peer.peer_stats.transfer.up.average)),
            );
        }

        // Dump connection table
        let connman =
            if let Some(connection_manager) = registry.network_manager().opt_connection_manager() {
                connection_manager.debug_print().await
            } else {
                "Connection manager unavailable when detached".to_owned()
            };

        Ok(format!("{}\n{}\n{}\n", nodeinfo, peertable, connman))
    }

    async fn debug_nodeid(&self, _args: String) -> VeilidAPIResult<String> {
        // Dump routing table entry
        let registry = self.core_context()?.registry();
        let nodeid = registry.routing_table().debug_info_nodeid();
        Ok(nodeid)
    }

    async fn debug_config(&self, args: String) -> VeilidAPIResult<String> {
        let mut args = args.as_str();
        let mut config = self.config()?;
        if !args.starts_with("insecure") {
            config = config.safe_config();
        } else {
            args = &args[8..];
        }
        let args = args.trim_start();

        if args.is_empty() {
            return config.get_key_json("", true);
        }
        let (arg, rest) = args.split_once(' ').unwrap_or((args, ""));
        let rest = rest.trim_start().to_owned();

        // One argument is 'config get'
        if rest.is_empty() {
            return config.get_key_json(arg, true);
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

            let registry = self.core_context()?.registry();
            registry.network_manager().restart_network();

            Ok("Network restarted".to_owned())
        } else {
            apibail_invalid_argument!("debug_restart", "arg_1", arg);
        }
    }

    async fn debug_purge(&self, args: String) -> VeilidAPIResult<String> {
        let registry = self.core_context()?.registry();

        let args: Vec<String> = args.split_whitespace().map(|s| s.to_owned()).collect();
        if !args.is_empty() {
            if args[0] == "buckets" {
                // Must be detached
                if !matches!(
                    self.get_state().await?.attachment.state,
                    AttachmentState::Detached | AttachmentState::Detaching
                ) {
                    apibail_internal!("Must be detached to purge");
                }
                registry.routing_table().purge_buckets();
                Ok("Buckets purged".to_owned())
            } else if args[0] == "connections" {
                // Purge connection table
                let opt_connection_manager = registry.network_manager().opt_connection_manager();

                if let Some(connection_manager) = &opt_connection_manager {
                    connection_manager.shutdown().await;
                }

                // Eliminate last_connections from routing table entries
                registry.routing_table().purge_last_connections();

                if let Some(connection_manager) = &opt_connection_manager {
                    connection_manager
                        .startup()
                        .await
                        .map_err(VeilidAPIError::internal)?;
                }
                Ok("Connections purged".to_owned())
            } else if args[0] == "routes" {
                // Purge route spec store
                self.with_debug_cache(|dc| {
                    dc.imported_routes.clear();
                });
                match registry.routing_table().route_spec_store().purge().await {
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

        let registry = self.core_context()?.registry();

        let node_ref = get_debug_argument_at(
            &args,
            0,
            "debug_contact",
            "node_ref",
            get_filtered_node_ref(registry.clone()),
        )?;

        let cm = registry
            .network_manager()
            .get_node_contact_method(node_ref)
            .map_err(VeilidAPIError::internal)?;

        Ok(format!("{:#?}", cm))
    }

    async fn debug_resolve(&self, args: String) -> VeilidAPIResult<String> {
        let registry = self.core_context()?.registry();
        if !registry.attachment_manager().is_attached() {
            apibail_internal!("Must be attached first");
        };

        let args: Vec<String> = args.split_whitespace().map(|s| s.to_owned()).collect();

        let dest = async_get_debug_argument_at(
            &args,
            0,
            "debug_resolve",
            "destination",
            self.clone().get_destination(registry.clone()),
        )
        .await?;

        let routing_table = registry.routing_table();
        match &dest {
            Destination::Direct {
                node: target,
                safety_selection: _,
            } => Ok(format!(
                "Destination: {:#?}\nTarget Entry:\n{}\n",
                &dest,
                routing_table.debug_info_entry(target.unfiltered())
            )),
            Destination::Relay {
                relay,
                node: target,
                safety_selection: _,
            } => Ok(format!(
                "Destination: {:#?}\nTarget Entry:\n{}\nRelay Entry:\n{}\n",
                &dest,
                routing_table.debug_info_entry(target.clone()),
                routing_table.debug_info_entry(relay.unfiltered())
            )),
            Destination::PrivateRoute {
                private_route: _,
                safety_selection: _,
            } => Ok(format!("Destination: {:#?}", &dest)),
        }
    }

    async fn debug_ping(&self, args: String) -> VeilidAPIResult<String> {
        let registry = self.core_context()?.registry();
        if !registry.attachment_manager().is_attached() {
            apibail_internal!("Must be attached first");
        };

        let args: Vec<String> = args.split_whitespace().map(|s| s.to_owned()).collect();

        let dest = async_get_debug_argument_at(
            &args,
            0,
            "debug_ping",
            "destination",
            self.clone().get_destination(registry.clone()),
        )
        .await?;

        // Send a StatusQ
        let rpc_processor = registry.rpc_processor();
        let out = match rpc_processor
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

    async fn debug_app_message(&self, args: String) -> VeilidAPIResult<String> {
        let registry = self.core_context()?.registry();
        if !registry.attachment_manager().is_attached() {
            apibail_internal!("Must be attached first");
        };

        let (arg, rest) = args.split_once(' ').unwrap_or((&args, ""));
        let rest = rest.trim_start().to_owned();

        let dest = async_get_debug_argument(
            arg,
            "debug_app_message",
            "destination",
            self.clone().get_destination(registry.clone()),
        )
        .await?;

        let data = get_debug_argument(&rest, "debug_app_message", "data", get_data)?;
        let data_len = data.len();

        // Send an AppMessage
        let rpc_processor = registry.rpc_processor();

        let out = match rpc_processor
            .rpc_call_app_message(dest, data)
            .await
            .map_err(VeilidAPIError::internal)?
        {
            NetworkResult::Value(_) => format!("Sent {} bytes", data_len),
            r => {
                return Ok(r.to_string());
            }
        };

        Ok(out)
    }

    async fn debug_app_call(&self, args: String) -> VeilidAPIResult<String> {
        let registry = self.core_context()?.registry();
        if !registry.attachment_manager().is_attached() {
            apibail_internal!("Must be attached first");
        };

        let (arg, rest) = args.split_once(' ').unwrap_or((&args, ""));
        let rest = rest.trim_start().to_owned();

        let dest = async_get_debug_argument(
            arg,
            "debug_app_call",
            "destination",
            self.clone().get_destination(registry.clone()),
        )
        .await?;

        let data = get_debug_argument(&rest, "debug_app_call", "data", get_data)?;
        let data_len = data.len();

        // Send an AppCall
        let rpc_processor = registry.rpc_processor();

        let out = match rpc_processor
            .rpc_call_app_call(dest, data)
            .await
            .map_err(VeilidAPIError::internal)?
        {
            NetworkResult::Value(v) => format!(
                "Sent {} bytes, received: {}",
                data_len,
                print_data(&v.answer, Some(512))
            ),
            r => {
                return Ok(r.to_string());
            }
        };

        Ok(out)
    }

    async fn debug_app_reply(&self, args: String) -> VeilidAPIResult<String> {
        let registry = self.core_context()?.registry();
        if !registry.attachment_manager().is_attached() {
            apibail_internal!("Must be attached first");
        };

        let (call_id, data) = if let Some(stripped_args) = args.strip_prefix('#') {
            let (arg, rest) = stripped_args.split_once(' ').unwrap_or((&args, ""));
            let call_id =
                OperationId::new(u64::from_str_radix(arg, 16).map_err(VeilidAPIError::generic)?);
            let rest = rest.trim_start().to_owned();
            let data = get_debug_argument(&rest, "debug_app_reply", "data", get_data)?;
            (call_id, data)
        } else {
            let rpc_processor = registry.rpc_processor();

            let call_id = rpc_processor
                .get_app_call_ids()
                .first()
                .cloned()
                .ok_or_else(|| VeilidAPIError::generic("no app calls waiting"))?;
            let data = get_debug_argument(&args, "debug_app_reply", "data", get_data)?;
            (call_id, data)
        };

        let data_len = data.len();

        // Send a AppCall Reply
        self.app_call_reply(call_id, data)
            .await
            .map_err(VeilidAPIError::internal)?;

        Ok(format!("Replied with {} bytes", data_len))
    }

    async fn debug_route_allocate(&self, args: Vec<String>) -> VeilidAPIResult<String> {
        // [ord|*ord] [rel] [<count>] [in|out] [avoid_node_id]

        let registry = self.core_context()?.registry();
        let routing_table = registry.routing_table();
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

        let safety_spec = SafetySpec {
            preferred_route: None,
            hop_count,
            stability,
            sequencing,
        };

        // Allocate route
        let out =
            match rss.allocate_route(&VALID_CRYPTO_KINDS, &safety_spec, directions, &[], false) {
                Ok(v) => v.to_string(),
                Err(e) => {
                    format!("Route allocation failed: {}", e)
                }
            };

        Ok(out)
    }
    async fn debug_route_release(&self, args: Vec<String>) -> VeilidAPIResult<String> {
        // <route id>
        let registry = self.core_context()?.registry();
        let routing_table = registry.routing_table();
        let rss = routing_table.route_spec_store();

        let route_id = get_debug_argument_at(
            &args,
            1,
            "debug_route",
            "route_id",
            get_route_id(registry.clone(), true, true),
        )?;

        // Release route
        let out = match rss.release_route(route_id) {
            true => {
                // release imported
                self.with_debug_cache(|dc| {
                    for (n, ir) in dc.imported_routes.iter().enumerate() {
                        if *ir == route_id {
                            dc.imported_routes.remove(n);
                            break;
                        }
                    }
                });
                "Released".to_owned()
            }
            false => "Route does not exist".to_owned(),
        };

        Ok(out)
    }
    async fn debug_route_publish(&self, args: Vec<String>) -> VeilidAPIResult<String> {
        // <route id> [full]
        let registry = self.core_context()?.registry();
        let routing_table = registry.routing_table();
        let rss = routing_table.route_spec_store();

        let route_id = get_debug_argument_at(
            &args,
            1,
            "debug_route",
            "route_id",
            get_route_id(registry.clone(), true, false),
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
        let registry = self.core_context()?.registry();
        let routing_table = registry.routing_table();
        let rss = routing_table.route_spec_store();

        let route_id = get_debug_argument_at(
            &args,
            1,
            "debug_route",
            "route_id",
            get_route_id(registry.clone(), true, false),
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
        let registry = self.core_context()?.registry();
        let routing_table = registry.routing_table();
        let rss = routing_table.route_spec_store();

        let route_id = get_debug_argument_at(
            &args,
            1,
            "debug_route",
            "route_id",
            get_route_id(registry.clone(), true, true),
        )?;

        match rss.debug_route(&route_id) {
            Some(s) => Ok(s),
            None => Ok("Route does not exist".to_owned()),
        }
    }
    async fn debug_route_list(&self, _args: Vec<String>) -> VeilidAPIResult<String> {
        //
        let registry = self.core_context()?.registry();
        let routing_table = registry.routing_table();
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
        let registry = self.core_context()?.registry();
        let routing_table = registry.routing_table();
        let rss = routing_table.route_spec_store();

        let blob = get_debug_argument_at(&args, 1, "debug_route", "blob", get_string)?;
        let blob_dec = BASE64URL_NOPAD
            .decode(blob.as_bytes())
            .map_err(VeilidAPIError::generic)?;

        let route_id = rss
            .import_remote_private_route_blob(blob_dec)
            .map_err(VeilidAPIError::generic)?;

        let out = self.with_debug_cache(|dc| {
            let n = dc.imported_routes.len();
            let out = format!("Private route #{} imported: {}", n, route_id);
            dc.imported_routes.push(route_id);
            out
        });

        Ok(out)
    }

    async fn debug_route_test(&self, args: Vec<String>) -> VeilidAPIResult<String> {
        // <route id>
        let registry = self.core_context()?.registry();
        let routing_table = registry.routing_table();
        let rss = routing_table.route_spec_store();

        let route_id = get_debug_argument_at(
            &args,
            1,
            "debug_route",
            "route_id",
            get_route_id(registry.clone(), true, true),
        )?;

        let success = rss
            .test_route(route_id)
            .await
            .map_err(VeilidAPIError::internal)?;

        let out = match success {
            Some(true) => "SUCCESS".to_owned(),
            Some(false) => "FAILED".to_owned(),
            None => "UNTESTED".to_owned(),
        };

        Ok(out)
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
        let registry = self.core_context()?.registry();
        let storage_manager = registry.storage_manager();

        let scope = get_debug_argument_at(&args, 1, "debug_record_list", "scope", get_string)?;
        let out = match scope.as_str() {
            "local" => {
                let mut out = "Local Records:\n".to_string();
                out += &storage_manager.debug_local_records().await;
                out
            }
            "remote" => {
                let mut out = "Remote Records:\n".to_string();
                out += &storage_manager.debug_remote_records().await;
                out
            }
            "opened" => {
                let mut out = "Opened Records:\n".to_string();
                out += &storage_manager.debug_opened_records().await;
                out
            }
            "offline" => {
                let mut out = "Offline Records:\n".to_string();
                out += &storage_manager.debug_offline_records().await;
                out
            }
            _ => "Invalid scope\n".to_owned(),
        };
        Ok(out)
    }

    async fn debug_record_purge(&self, args: Vec<String>) -> VeilidAPIResult<String> {
        // <local|remote> [bytes]
        let registry = self.core_context()?.registry();
        let storage_manager = registry.storage_manager();

        let scope = get_debug_argument_at(&args, 1, "debug_record_purge", "scope", get_string)?;
        let bytes = get_debug_argument_at(&args, 2, "debug_record_purge", "bytes", get_number).ok();
        let out = match scope.as_str() {
            "local" => storage_manager.purge_local_records(bytes).await,
            "remote" => storage_manager.purge_remote_records(bytes).await,
            _ => "Invalid scope\n".to_owned(),
        };
        Ok(out)
    }

    async fn debug_record_create(&self, args: Vec<String>) -> VeilidAPIResult<String> {
        let crypto = self.crypto()?;

        let schema = get_debug_argument_at(
            &args,
            1,
            "debug_record_create",
            "dht_schema",
            get_dht_schema,
        )
        .unwrap_or_else(|_| Ok(DHTSchema::default()))?;

        let csv = get_debug_argument_at(
            &args,
            2,
            "debug_record_create",
            "kind",
            get_crypto_system_version(&crypto),
        )
        .unwrap_or_else(|_| crypto.best());

        let ss = get_debug_argument_at(
            &args,
            3,
            "debug_record_create",
            "safety_selection",
            get_safety_selection(self.core_context()?.registry()),
        )
        .ok();

        // Get routing context with optional safety
        let rc = self.routing_context()?;
        let rc = if let Some(ss) = ss {
            match rc.with_safety(ss) {
                Err(e) => return Ok(format!("Can't use safety selection: {}", e)),
                Ok(v) => v,
            }
        } else {
            rc
        };

        // Do a record create
        let record = match rc.create_dht_record(schema, None, Some(csv.kind())).await {
            Err(e) => return Ok(format!("Can't open DHT record: {}", e)),
            Ok(v) => v,
        };

        // Save routing context for record
        self.with_debug_cache(|dc| {
            dc.opened_record_contexts.insert(*record.key(), rc);
        });

        Ok(format!(
            "Created: {} {}:{}\n{:?}",
            record.key(),
            record.owner(),
            record.owner_secret().unwrap(),
            record
        ))
    }

    async fn debug_record_open(&self, args: Vec<String>) -> VeilidAPIResult<String> {
        let registry = self.core_context()?.registry();

        let (key, ss) = get_debug_argument_at(
            &args,
            1,
            "debug_record_open",
            "key",
            get_dht_key(registry.clone()),
        )?;
        let writer =
            get_debug_argument_at(&args, 2, "debug_record_open", "writer", get_keypair).ok();

        // Get routing context with optional safety
        let rc = self.routing_context()?;
        let rc = if let Some(ss) = ss {
            match rc.with_safety(ss) {
                Err(e) => return Ok(format!("Can't use safety selection: {}", e)),
                Ok(v) => v,
            }
        } else {
            rc
        };

        // Do a record open
        let record = match rc.open_dht_record(key, writer).await {
            Err(e) => return Ok(format!("Can't open DHT record: {}", e)),
            Ok(v) => v,
        };

        // Save routing context for record
        self.with_debug_cache(|dc| {
            dc.opened_record_contexts.insert(*record.key(), rc);
        });

        Ok(format!("Opened: {} : {:?}", key, record))
    }

    async fn debug_record_close(&self, args: Vec<String>) -> VeilidAPIResult<String> {
        let (key, rc) =
            self.clone()
                .get_opened_dht_record_context(&args, "debug_record_close", "key", 1)?;

        // Do a record close
        if let Err(e) = rc.close_dht_record(key).await {
            return Ok(format!("Can't close DHT record: {}", e));
        };

        Ok(format!("Closed: {:?}", key))
    }

    async fn debug_record_set(&self, args: Vec<String>) -> VeilidAPIResult<String> {
        let opt_arg_add = if args.len() >= 2 && get_dht_key_no_safety(&args[1]).is_some() {
            1
        } else {
            0
        };
        let (key, rc) =
            self.clone()
                .get_opened_dht_record_context(&args, "debug_record_set", "key", 1)?;
        let subkey = get_debug_argument_at(
            &args,
            1 + opt_arg_add,
            "debug_record_set",
            "subkey",
            get_number::<u32>,
        )?;
        let data =
            get_debug_argument_at(&args, 2 + opt_arg_add, "debug_record_set", "data", get_data)?;
        let writer = get_debug_argument_at(
            &args,
            3 + opt_arg_add,
            "debug_record_set",
            "writer",
            get_keypair,
        )
        .ok();

        // Do a record set
        let value = match rc
            .set_dht_value(key, subkey as ValueSubkey, data, writer)
            .await
        {
            Err(e) => {
                return Ok(format!("Can't set DHT value: {}", e));
            }
            Ok(v) => v,
        };
        let out = if let Some(value) = value {
            format!("Newer value found: {:?}", value)
        } else {
            "Success".to_owned()
        };
        Ok(out)
    }

    async fn debug_record_get(&self, args: Vec<String>) -> VeilidAPIResult<String> {
        let opt_arg_add = if args.len() >= 2 && get_dht_key_no_safety(&args[1]).is_some() {
            1
        } else {
            0
        };

        let (key, rc) =
            self.clone()
                .get_opened_dht_record_context(&args, "debug_record_get", "key", 1)?;
        let subkey = get_debug_argument_at(
            &args,
            1 + opt_arg_add,
            "debug_record_get",
            "subkey",
            get_number::<u32>,
        )?;
        let force_refresh = if args.len() >= 3 + opt_arg_add {
            Some(get_debug_argument_at(
                &args,
                2 + opt_arg_add,
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

        // Do a record get
        let value = match rc
            .get_dht_value(key, subkey as ValueSubkey, force_refresh)
            .await
        {
            Err(e) => {
                return Ok(format!("Can't get DHT value: {}", e));
            }
            Ok(v) => v,
        };
        let out = if let Some(value) = value {
            format!("{:?}", value)
        } else {
            "No value data returned".to_owned()
        };
        Ok(out)
    }

    async fn debug_record_delete(&self, args: Vec<String>) -> VeilidAPIResult<String> {
        let key = get_debug_argument_at(
            &args,
            1,
            "debug_record_delete",
            "key",
            get_dht_key_no_safety,
        )?;

        // Do a record delete (can use any routing context here)
        let rc = self.routing_context()?;
        match rc.delete_dht_record(key).await {
            Err(e) => return Ok(format!("Can't delete DHT record: {}", e)),
            Ok(v) => v,
        };
        Ok("DHT record deleted".to_string())
    }

    async fn debug_record_info(&self, args: Vec<String>) -> VeilidAPIResult<String> {
        let registry = self.core_context()?.registry();
        let storage_manager = registry.storage_manager();

        let key =
            get_debug_argument_at(&args, 1, "debug_record_info", "key", get_dht_key_no_safety)?;

        let subkey = get_debug_argument_at(
            &args,
            2,
            "debug_record_info",
            "subkey",
            get_number::<ValueSubkey>,
        )
        .ok();

        let out = if let Some(subkey) = subkey {
            let li = storage_manager
                .debug_local_record_subkey_info(key, subkey)
                .await;
            let ri = storage_manager
                .debug_remote_record_subkey_info(key, subkey)
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
        Ok(out)
    }

    async fn debug_record_watch(&self, args: Vec<String>) -> VeilidAPIResult<String> {
        let opt_arg_add = if args.len() >= 2 && get_dht_key_no_safety(&args[1]).is_some() {
            1
        } else {
            0
        };

        let (key, rc) =
            self.clone()
                .get_opened_dht_record_context(&args, "debug_record_watch", "key", 1)?;

        let mut rest_defaults = false;
        let subkeys = get_debug_argument_at(
            &args,
            1 + opt_arg_add,
            "debug_record_watch",
            "subkeys",
            get_subkeys,
        )
        .ok()
        .unwrap_or_else(|| {
            rest_defaults = true;
            Default::default()
        });

        let expiration = if rest_defaults {
            Default::default()
        } else {
            get_debug_argument_at(
                &args,
                2 + opt_arg_add,
                "debug_record_watch",
                "expiration",
                parse_duration,
            )
            .ok()
            .map(|dur| dur + get_timestamp())
            .unwrap_or_else(|| {
                rest_defaults = true;
                Default::default()
            })
        };
        let count = if rest_defaults {
            u32::MAX
        } else {
            get_debug_argument_at(
                &args,
                3 + opt_arg_add,
                "debug_record_watch",
                "count",
                get_number,
            )
            .ok()
            .unwrap_or_else(|| {
                rest_defaults = true;
                u32::MAX
            })
        };

        // Do a record watch
        let ts = match rc
            .watch_dht_values(key, subkeys, Timestamp::new(expiration), count)
            .await
        {
            Err(e) => {
                return Ok(format!("Can't watch DHT value: {}", e));
            }
            Ok(v) => v,
        };
        if ts.as_u64() == 0 {
            return Ok("Failed to watch value".to_owned());
        }
        Ok(format!("Success: expiration={:?}", display_ts(ts.as_u64())))
    }

    async fn debug_record_cancel(&self, args: Vec<String>) -> VeilidAPIResult<String> {
        let opt_arg_add = if args.len() >= 2 && get_dht_key_no_safety(&args[1]).is_some() {
            1
        } else {
            0
        };

        let (key, rc) =
            self.clone()
                .get_opened_dht_record_context(&args, "debug_record_watch", "key", 1)?;
        let subkeys = get_debug_argument_at(
            &args,
            1 + opt_arg_add,
            "debug_record_watch",
            "subkeys",
            get_subkeys,
        )
        .ok()
        .unwrap_or_default();

        // Do a record watch cancel
        let still_active = match rc.cancel_dht_watch(key, subkeys).await {
            Err(e) => {
                return Ok(format!("Can't cancel DHT watch: {}", e));
            }
            Ok(v) => v,
        };

        Ok(if still_active {
            "Watch partially cancelled".to_owned()
        } else {
            "Watch cancelled".to_owned()
        })
    }

    async fn debug_record_inspect(&self, args: Vec<String>) -> VeilidAPIResult<String> {
        let opt_arg_add = if args.len() >= 2 && get_dht_key_no_safety(&args[1]).is_some() {
            1
        } else {
            0
        };

        let (key, rc) =
            self.clone()
                .get_opened_dht_record_context(&args, "debug_record_watch", "key", 1)?;

        let mut rest_defaults = false;

        let scope = if rest_defaults {
            Default::default()
        } else {
            get_debug_argument_at(
                &args,
                1 + opt_arg_add,
                "debug_record_inspect",
                "scope",
                get_dht_report_scope,
            )
            .ok()
            .unwrap_or_else(|| {
                rest_defaults = true;
                Default::default()
            })
        };

        let subkeys = get_debug_argument_at(
            &args,
            2 + opt_arg_add,
            "debug_record_inspect",
            "subkeys",
            get_subkeys,
        )
        .ok()
        .unwrap_or_else(|| {
            rest_defaults = true;
            Default::default()
        });

        // Do a record inspect
        let report = match rc.inspect_dht_record(key, subkeys, scope).await {
            Err(e) => {
                return Ok(format!("Can't inspect DHT record: {}", e));
            }
            Ok(v) => v,
        };

        Ok(format!("Success: report={:?}", report))
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
        } else if command == "open" {
            self.debug_record_open(args).await
        } else if command == "close" {
            self.debug_record_close(args).await
        } else if command == "get" {
            self.debug_record_get(args).await
        } else if command == "set" {
            self.debug_record_set(args).await
        } else if command == "delete" {
            self.debug_record_delete(args).await
        } else if command == "info" {
            self.debug_record_info(args).await
        } else if command == "watch" {
            self.debug_record_watch(args).await
        } else if command == "cancel" {
            self.debug_record_cancel(args).await
        } else if command == "inspect" {
            self.debug_record_inspect(args).await
        } else {
            Ok(">>> Unknown command\n".to_owned())
        }
    }

    async fn debug_table_list(&self, _args: Vec<String>) -> VeilidAPIResult<String> {
        //
        let table_store = self.table_store()?;
        let table_names = table_store.list_all();
        let out = format!(
            "TableStore tables:\n{}",
            table_names
                .iter()
                .map(|(k, v)| format!("{} ({})", k, v))
                .collect::<Vec<String>>()
                .join("\n")
        );
        Ok(out)
    }

    fn _format_columns(columns: &[table_store::ColumnInfo]) -> String {
        let mut out = String::new();
        for (n, col) in columns.iter().enumerate() {
            //
            out += &format!("Column {}:\n", n);
            out += &format!("  Key Count: {}\n", col.key_count);
        }
        out
    }

    async fn debug_table_info(&self, args: Vec<String>) -> VeilidAPIResult<String> {
        //
        let table_store = self.table_store()?;

        let table_name = get_debug_argument_at(&args, 1, "debug_table_info", "name", get_string)?;

        let Some(info) = table_store.info(&table_name).await? else {
            return Ok(format!("Table '{}' does not exist", table_name));
        };

        let info_str = format!(
            "Table Name: {}\n\
            Column Count: {}\n\
            IO Stats (since previous query):\n{}\n\
            IO Stats (overall):\n{}\n\
            Columns:\n{}\n",
            info.table_name,
            info.column_count,
            indent::indent_all_by(4, format!("{:#?}", info.io_stats_since_previous)),
            indent::indent_all_by(4, format!("{:#?}", info.io_stats_overall)),
            Self::_format_columns(&info.columns),
        );

        let out = format!("Table info for '{}':\n{}", table_name, info_str);
        Ok(out)
    }

    async fn debug_table(&self, args: String) -> VeilidAPIResult<String> {
        let args: Vec<String> =
            shell_words::split(&args).map_err(|e| VeilidAPIError::parse_error(e, args))?;

        let command = get_debug_argument_at(&args, 0, "debug_table", "command", get_string)?;

        if command == "list" {
            self.debug_table_list(args).await
        } else if command == "info" {
            self.debug_table_info(args).await
        } else {
            Ok(">>> Unknown command\n".to_owned())
        }
    }

    async fn debug_punish_list(&self, _args: Vec<String>) -> VeilidAPIResult<String> {
        //
        let registry = self.core_context()?.registry();
        let network_manager = registry.network_manager();
        let address_filter = network_manager.address_filter();

        let out = format!("Address filter punishments:\n{:#?}", address_filter);
        Ok(out)
    }

    async fn debug_punish_clear(&self, _args: Vec<String>) -> VeilidAPIResult<String> {
        //
        let registry = self.core_context()?.registry();
        let network_manager = registry.network_manager();
        let address_filter = network_manager.address_filter();

        address_filter.clear_punishments();

        Ok("Address Filter punishments cleared\n".to_owned())
    }

    async fn debug_punish(&self, args: String) -> VeilidAPIResult<String> {
        let args: Vec<String> =
            shell_words::split(&args).map_err(|e| VeilidAPIError::parse_error(e, args))?;

        let command = get_debug_argument_at(&args, 0, "debug_punish", "command", get_string)?;

        if command == "list" {
            self.debug_punish_list(args).await
        } else if command == "clear" {
            self.debug_punish_clear(args).await
        } else {
            Ok(">>> Unknown command\n".to_owned())
        }
    }

    /// Get the help text for 'internal debug' commands.
    pub async fn debug_help(&self, _args: String) -> VeilidAPIResult<String> {
        Ok(r#"Node Information:
    nodeid  - display a node's id(s)
    nodeinfo - display detailed information about this node
    dialinfo - display the dialinfo in the routing domains of this node
    peerinfo [routingdomain] [published|current] - display the full PeerInfo for a routing domain of this node
    uptime - display node uptime

Routing:
    buckets [dead|reliable] - Display the routing table bucket statistics (default is only non-dead nodes)
    entries [dead|reliable] [<capabilities>] - Display the index of nodes in the routing table
    entry <node> - Display all the details about a particular node in the routing table
    contact <node>[+<sequencing>][<modifiers>] - Explain what mechanism would be used to contact a particular node
    resolve <destination> - Search the network for a particular node or private route
    relay <relay> [public|local] - Change the relay in use for this node
    punish list - List all punishments this node has assigned to other nodes / networks
           clear - Clear all punishments from this node
    route allocate [<sequencing>] [rel] [<count>] [in|out] - Allocate a route
          release <route> - Release a route
          publish <route> [full] - Publish a route 'blob' that can be imported on another machine
          unpublish <route> - Mark a route as 'no longer published'
          print <route> - Display details about a route
          list - List allocated routes
          import <blob> - Import a remote route blob generated by another node's 'publish' command.
          test <route> - Test an allocated or imported remote route

Utilities:
    config [insecure] [configkey [new value]] - Display or temporarily change the node config
                                                (most values should not be changed this way, careful!)
    txtrecord - Generate a TXT record for making this node into a bootstrap node capable of DNS bootstrap
    keypair [cryptokind] - Generate and display a random public/private keypair
    purge <buckets|connections|routes> - Throw away the node's routing table, connections, or routes
    attach - Attach the node to the network if it is detached
    detach - Detach the node from the network if it is attached
    restart network - Restart the low level network

RPC Operations:
    ping <destination> - Send a 'Status' RPC question to a destination node and display the returned ping status
    appmessage <destination> <data> - Send an 'App Message' RPC statement to a destination node
    appcall <destination> <data> - Send a 'App Call' RPC question to a destination node and display the answer
    appreply [#id] <data> - Reply to an 'App Call' RPC received by this node

DHT Operations:
    record list <local|remote|opened|offline> - display the dht records in the store
           purge <local|remote> [bytes] - clear all dht records optionally down to some total size
           create <dhtschema> [<cryptokind> [<safety>]] - create a new dht record
           open <key>[+<safety>] [<writer>] - open an existing dht record
           close [<key>] - close an opened/created dht record
           set [<key>] <subkey> <data> - write a value to a dht record subkey
           get [<key>] <subkey> [force] - read a value from a dht record subkey
           delete <key> - delete the local copy of a dht record (not from the network)
           info [<key>] [subkey] - display information about a dht record or subkey
           watch [<key>] [<subkeys> [<expiration> [<count>]]] - watch a record for changes
           cancel [<key>] [<subkeys>] - cancel a dht record watch
           inspect [<key>] [<scope> [<subkeys>]] - display a dht record's subkey status

TableDB Operations:
    table list - list the names of all the tables in the TableDB

--------------------------------------------------------------------
<key> is: VLD0:GsgXCRPrzSK6oBNgxhNpm-rTYFd02R0ySx6j9vbQBG4
    * also <node>, <relay>, <target>, <route>
<capabilities> is: a list of FourCC codes: ROUT,SGNL,RLAY,DIAL,DHTV,DHTW,APPM etc.
<configkey> is: dot path like network.protocol.udp.enabled
<destination> is:
    * direct:  <node>[+<safety>][<modifiers>]
    * relay:   <relay>@<target>[+<safety>][<modifiers>]
    * private: #<id>[+<safety>]
<sequencing> is:
    * prefer_ordered: ord
    * ensure_ordered: *ord
<safety> is:
    * unsafe: -<sequencing>
    * safe: [route][,<sequencing>][,rel][,<count>]
<modifiers> is: [/<protocoltype>][/<addresstype>][/<routingdomain>]
<protocoltype> is: udp|tcp|ws|wss
<addresstype> is: ipv4|ipv6
<routingdomain> is: public|local
<cryptokind> is: VLD0
<dhtschema> is:
    * a single-quoted json dht schema, or
    * an integer number for a DFLT schema subkey count.
    default is '{"kind":"DFLT","o_cnt":1}'
<scope> is: local, syncget, syncset, updateget, updateset
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

    /// Get node uptime info.
    pub async fn debug_uptime(&self, _args: String) -> VeilidAPIResult<String> {
        let mut result = String::new();

        writeln!(result, "Uptime...").ok();

        let state = self.get_state().await?;

        let uptime = state.attachment.uptime;
        writeln!(result, "  since launch: {uptime}").ok();

        if let Some(attached_uptime) = state.attachment.attached_uptime {
            writeln!(result, "  since attachment: {attached_uptime}").ok();
        }

        Ok(result)
    }

    /// Execute an 'internal debug command'.
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
            } else if arg == "nodeid" {
                self.debug_nodeid(rest).await
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
            } else if arg == "appmessage" {
                self.debug_app_message(rest).await
            } else if arg == "appcall" {
                self.debug_app_call(rest).await
            } else if arg == "appreply" {
                self.debug_app_reply(rest).await
            } else if arg == "resolve" {
                self.debug_resolve(rest).await
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
            } else if arg == "punish" {
                self.debug_punish(rest).await
            } else if arg == "table" {
                self.debug_table(rest).await
            } else if arg == "uptime" {
                self.debug_uptime(rest).await
            } else {
                Err(VeilidAPIError::generic("Unknown debug command"))
            }
        };
        res
    }

    fn get_destination(
        self,
        registry: VeilidComponentRegistry,
    ) -> impl FnOnce(&str) -> SendPinBoxFuture<Option<Destination>> {
        move |text| {
            let text = text.to_owned();
            Box::pin(async move {
                // Safety selection
                let (text, ss) = if let Some((first, second)) = text.split_once('+') {
                    let ss = get_safety_selection(registry.clone())(second)?;
                    (first, Some(ss))
                } else {
                    (text.as_str(), None)
                };
                if text.is_empty() {
                    return None;
                }
                if &text[0..1] == "#" {
                    let routing_table = registry.routing_table();
                    let rss = routing_table.route_spec_store();

                    // Private route
                    let text = &text[1..];

                    let private_route = if let Some(prid) =
                        get_route_id(registry.clone(), false, true)(text)
                    {
                        rss.best_remote_private_route(&prid)?
                    } else {
                        let n = get_number(text)?;

                        self.with_debug_cache(|dc| {
                            let prid = *dc.imported_routes.get(n)?;
                            let Some(private_route) = rss.best_remote_private_route(&prid) else {
                                // Remove imported route
                                dc.imported_routes.remove(n);
                                info!("removed dead imported route {}", n);
                                return None;
                            };
                            Some(private_route)
                        })?
                    };

                    Some(Destination::private_route(
                        private_route,
                        ss.unwrap_or(SafetySelection::Unsafe(Sequencing::default())),
                    ))
                } else if let Some((first, second)) = text.split_once('@') {
                    // Relay
                    let relay_nr =
                        resolve_filtered_node_ref(registry.clone(), ss.unwrap_or_default())(second)
                            .await?;
                    let target_nr = get_node_ref(registry.clone())(first)?;

                    let mut d = Destination::relay(relay_nr, target_nr);
                    if let Some(ss) = ss {
                        d = d.with_safety(ss)
                    }

                    Some(d)
                } else {
                    // Direct
                    let target_nr =
                        resolve_filtered_node_ref(registry.clone(), ss.unwrap_or_default())(text)
                            .await?;

                    let mut d = Destination::direct(target_nr);
                    if let Some(ss) = ss {
                        d = d.with_safety(ss)
                    }

                    Some(d)
                }
            })
        }
    }

    fn get_opened_dht_record_context(
        self,
        args: &[String],
        context: &str,
        key: &str,
        arg: usize,
    ) -> VeilidAPIResult<(TypedKey, RoutingContext)> {
        let key = match get_debug_argument_at(args, arg, context, key, get_dht_key_no_safety)
            .ok()
            .or_else(|| {
                // If unspecified, use the most recent key opened or created
                self.with_debug_cache(|dc| dc.opened_record_contexts.back().map(|kv| kv.0).copied())
            }) {
            Some(k) => k,
            None => {
                apibail_missing_argument!("no keys are opened", "key");
            }
        };

        // Get routing context for record

        let Some(rc) = self.with_debug_cache(|dc| dc.opened_record_contexts.get(&key).cloned())
        else {
            apibail_missing_argument!("key is not opened", "key");
        };

        Ok((key, rc))
    }
}

const DEFAULT_INDENT: usize = 4;
pub fn indent_string<S: ToString>(s: &S) -> String {
    indent_by(DEFAULT_INDENT, s.to_string())
}
pub fn indent_all_string<S: ToString>(s: &S) -> String {
    indent_all_by(DEFAULT_INDENT, s.to_string())
}

pub trait ToMultilineString {
    fn to_multiline_string(&self) -> String;
}

impl<T> ToMultilineString for Vec<T>
where
    T: fmt::Display,
{
    fn to_multiline_string(&self) -> String {
        let mut out = String::new();
        for x in self {
            out += &x.to_string();
            out += "\n";
        }
        out
    }
}

pub trait StripTrailingNewline {
    fn strip_trailing_newline(&self) -> &str;
}

impl<T: AsRef<str>> StripTrailingNewline for T {
    fn strip_trailing_newline(&self) -> &str {
        self.as_ref()
            .strip_suffix("\r\n")
            .or(self.as_ref().strip_suffix("\n"))
            .unwrap_or(self.as_ref())
    }
}
