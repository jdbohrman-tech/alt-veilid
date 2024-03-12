use super::*;

///////////////////////////////////////////////////////////////////////////////////////

/// Valid destinations for a message sent over a routing context
#[derive(Clone, Debug, Eq, PartialEq, Hash, Copy, PartialOrd, Ord)]
pub enum Target {
    /// Node by its public key
    NodeId(TypedKey),
    /// Remote private route by its id
    PrivateRoute(RouteId),
}

pub struct RoutingContextInner {}

pub struct RoutingContextUnlockedInner {
    /// Safety routing requirements
    safety_selection: SafetySelection,
}

/// Routing contexts are the way you specify the communication preferences for Veilid.
///
/// By default routing contexts have 'safety routing' enabled which offers sender privacy.
/// privacy. To disable this and send RPC operations straight from the node use [RoutingContext::with_safety()] with a [SafetySelection::Unsafe] parameter.
/// To enable receiver privacy, you should send to a private route RouteId that you have imported, rather than directly to a NodeId.
///
#[derive(Clone)]
pub struct RoutingContext {
    /// Veilid API handle
    api: VeilidAPI,
    inner: Arc<Mutex<RoutingContextInner>>,
    unlocked_inner: Arc<RoutingContextUnlockedInner>,
}

impl fmt::Debug for RoutingContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RoutingContext")
            .field("ptr", &format!("{:p}", Arc::as_ptr(&self.unlocked_inner)))
            .field("safety_selection", &self.unlocked_inner.safety_selection)
            .finish()
    }
}

impl RoutingContext {
    ////////////////////////////////////////////////////////////////

    pub(super) fn try_new(api: VeilidAPI) -> VeilidAPIResult<Self> {
        let config = api.config()?;
        let c = config.get();

        Ok(Self {
            api,
            inner: Arc::new(Mutex::new(RoutingContextInner {})),
            unlocked_inner: Arc::new(RoutingContextUnlockedInner {
                safety_selection: SafetySelection::Safe(SafetySpec {
                    preferred_route: None,
                    hop_count: c.network.rpc.default_route_hop_count as usize,
                    stability: Stability::default(),
                    sequencing: Sequencing::PreferOrdered,
                }),
            }),
        })
    }

    /// Turn on sender privacy, enabling the use of safety routes. This is the default and
    /// calling this function is only necessary if you have previously disable safety or used other parameters.
    ///
    /// Default values for hop count, stability and sequencing preferences are used.
    ///
    /// * Hop count default is dependent on config, but is set to 1 extra hop.
    /// * Stability default is to choose 'low latency' routes, preferring them over long-term reliability.
    /// * Sequencing default is to prefer ordered before unordered message delivery
    ///
    /// To customize the safety selection in use, use [RoutingContext::with_safety()].
    #[instrument(target = "veilid_api", level = "debug", ret, err)]
    pub fn with_default_safety(self) -> VeilidAPIResult<Self> {
        event!(target: "veilid_api", Level::DEBUG, 
            "RoutingContext::with_default_safety(self: {:?})", self);

        let config = self.api.config()?;
        let c = config.get();

        self.with_safety(SafetySelection::Safe(SafetySpec {
            preferred_route: None,
            hop_count: c.network.rpc.default_route_hop_count as usize,
            stability: Stability::default(),
            sequencing: Sequencing::PreferOrdered,
        }))
    }

    /// Use a custom [SafetySelection]. Can be used to disable safety via [SafetySelection::Unsafe]
    #[instrument(target = "veilid_api", level = "debug", ret, err)]
    pub fn with_safety(self, safety_selection: SafetySelection) -> VeilidAPIResult<Self> {
        event!(target: "veilid_api", Level::DEBUG, 
            "RoutingContext::with_safety(self: {:?}, safety_selection: {:?})", self, safety_selection);

        Ok(Self {
            api: self.api.clone(),
            inner: Arc::new(Mutex::new(RoutingContextInner {})),
            unlocked_inner: Arc::new(RoutingContextUnlockedInner { safety_selection }),
        })
    }

    /// Use a specified [Sequencing] preference, with or without privacy
    #[instrument(target = "veilid_api", level = "debug", ret)]
    pub fn with_sequencing(self, sequencing: Sequencing) -> Self {
        event!(target: "veilid_api", Level::DEBUG, 
            "RoutingContext::with_sequencing(self: {:?}, sequencing: {:?})", self, sequencing);

        Self {
            api: self.api.clone(),
            inner: Arc::new(Mutex::new(RoutingContextInner {})),
            unlocked_inner: Arc::new(RoutingContextUnlockedInner {
                safety_selection: match self.unlocked_inner.safety_selection {
                    SafetySelection::Unsafe(_) => SafetySelection::Unsafe(sequencing),
                    SafetySelection::Safe(safety_spec) => SafetySelection::Safe(SafetySpec {
                        preferred_route: safety_spec.preferred_route,
                        hop_count: safety_spec.hop_count,
                        stability: safety_spec.stability,
                        sequencing,
                    }),
                },
            }),
        }
    }

    /// Get the safety selection in use on this routing context
    pub fn safety(&self) -> SafetySelection {
        self.unlocked_inner.safety_selection
    }

    fn sequencing(&self) -> Sequencing {
        match self.unlocked_inner.safety_selection {
            SafetySelection::Unsafe(sequencing) => sequencing,
            SafetySelection::Safe(safety_spec) => safety_spec.sequencing,
        }
    }

    /// Get the [VeilidAPI] object that created this [RoutingContext]
    pub fn api(&self) -> VeilidAPI {
        self.api.clone()
    }

    #[instrument(target = "veilid_api", level = "debug", ret, err)]
    async fn get_destination(&self, target: Target) -> VeilidAPIResult<rpc_processor::Destination> {
        event!(target: "veilid_api", Level::DEBUG, 
            "RoutingContext::get_destination(self: {:?}, target: {:?})", self, target);

        let rpc_processor = self.api.rpc_processor()?;
        rpc_processor
            .resolve_target_to_destination(target, self.unlocked_inner.safety_selection)
            .await
            .map_err(VeilidAPIError::invalid_target)
    }

    ////////////////////////////////////////////////////////////////
    // App-level Messaging

    /// App-level bidirectional call that expects a response to be returned.
    ///
    /// Veilid apps may use this for arbitrary message passing.
    ///
    /// * `target` - can be either a direct node id or a private route
    /// * `message` - an arbitrary message blob of up to 32768 bytes
    ///
    /// Returns an answer blob of up to 32768 bytes
    #[instrument(target = "veilid_api", level = "debug", ret, err)]
    pub async fn app_call(&self, target: Target, message: Vec<u8>) -> VeilidAPIResult<Vec<u8>> {
        event!(target: "veilid_api", Level::DEBUG, 
            "RoutingContext::app_call(self: {:?}, target: {:?}, message: {:?})", self, target, message);

        let rpc_processor = self.api.rpc_processor()?;

        // Get destination
        let dest = self.get_destination(target).await?;

        // Send app message
        let answer = match rpc_processor.rpc_call_app_call(dest, message).await {
            Ok(NetworkResult::Value(v)) => v,
            Ok(NetworkResult::Timeout) => apibail_timeout!(),
            Ok(NetworkResult::ServiceUnavailable(e)) => apibail_invalid_target!(e),
            Ok(NetworkResult::NoConnection(e)) | Ok(NetworkResult::AlreadyExists(e)) => {
                apibail_no_connection!(e);
            }

            Ok(NetworkResult::InvalidMessage(message)) => {
                apibail_generic!(message);
            }
            Err(e) => return Err(e.into()),
        };

        Ok(answer.answer)
    }

    /// App-level unidirectional message that does not expect any value to be returned.
    ///
    /// Veilid apps may use this for arbitrary message passing.
    ///
    /// * `target` - can be either a direct node id or a private route
    /// * `message` - an arbitrary message blob of up to 32768 bytes
    #[instrument(target = "veilid_api", level = "debug", ret, err)]
    pub async fn app_message(&self, target: Target, message: Vec<u8>) -> VeilidAPIResult<()> {
        event!(target: "veilid_api", Level::DEBUG, 
            "RoutingContext::app_message(self: {:?}, target: {:?}, message: {:?})", self, target, message);

        let rpc_processor = self.api.rpc_processor()?;

        // Get destination
        let dest = self.get_destination(target).await?;

        // Send app message
        match rpc_processor.rpc_call_app_message(dest, message).await {
            Ok(NetworkResult::Value(())) => {}
            Ok(NetworkResult::Timeout) => apibail_timeout!(),
            Ok(NetworkResult::ServiceUnavailable(e)) => apibail_invalid_target!(e),
            Ok(NetworkResult::NoConnection(e)) | Ok(NetworkResult::AlreadyExists(e)) => {
                apibail_no_connection!(e);
            }
            Ok(NetworkResult::InvalidMessage(message)) => {
                apibail_generic!(message);
            }
            Err(e) => return Err(e.into()),
        };

        Ok(())
    }

    ///////////////////////////////////
    /// DHT Records

    /// Creates a new DHT record a specified crypto kind and schema
    ///
    /// The record is considered 'open' after the create operation succeeds.
    ///
    /// Returns the newly allocated DHT record's key if successful.    
    #[instrument(target = "veilid_api", level = "debug", ret, err)]
    pub async fn create_dht_record(
        &self,
        schema: DHTSchema,
        kind: Option<CryptoKind>,
    ) -> VeilidAPIResult<DHTRecordDescriptor> {
        event!(target: "veilid_api", Level::DEBUG, 
            "RoutingContext::create_dht_record(self: {:?}, schema: {:?}, kind: {:?})", self, schema, kind);
        schema.validate()?;

        let kind = kind.unwrap_or(best_crypto_kind());
        Crypto::validate_crypto_kind(kind)?;
        let storage_manager = self.api.storage_manager()?;
        storage_manager
            .create_record(kind, schema, self.unlocked_inner.safety_selection)
            .await
    }

    /// Opens a DHT record at a specific key
    ///
    /// Associates a 'default_writer' secret if one is provided to provide writer capability. The
    /// writer can be overridden if specified here via the set_dht_value writer.
    ///
    /// Records may only be opened or created. If a record is re-opened it will use the new writer and routing context
    /// ignoring the settings of the last time it was opened. This allows one to open a record a second time
    /// without first closing it, which will keep the active 'watches' on the record but change the default writer or
    /// safety selection.
    ///
    /// Returns the DHT record descriptor for the opened record if successful
    #[instrument(target = "veilid_api", level = "debug", ret, err)]
    pub async fn open_dht_record(
        &self,
        key: TypedKey,
        default_writer: Option<KeyPair>,
    ) -> VeilidAPIResult<DHTRecordDescriptor> {
        event!(target: "veilid_api", Level::DEBUG, 
            "RoutingContext::open_dht_record(self: {:?}, key: {:?}, default_writer: {:?})", self, key, default_writer);

        Crypto::validate_crypto_kind(key.kind)?;
        let storage_manager = self.api.storage_manager()?;
        storage_manager
            .open_record(key, default_writer, self.unlocked_inner.safety_selection)
            .await
    }

    /// Closes a DHT record at a specific key that was opened with create_dht_record or open_dht_record.
    ///
    /// Closing a record allows you to re-open it with a different routing context
    #[instrument(target = "veilid_api", level = "debug", ret, err)]
    pub async fn close_dht_record(&self, key: TypedKey) -> VeilidAPIResult<()> {
        event!(target: "veilid_api", Level::DEBUG, 
            "RoutingContext::close_dht_record(self: {:?}, key: {:?})", self, key);

        Crypto::validate_crypto_kind(key.kind)?;
        let storage_manager = self.api.storage_manager()?;
        storage_manager.close_record(key).await
    }

    /// Deletes a DHT record at a specific key
    ///
    /// If the record is opened, it must be closed before it is deleted.
    /// Deleting a record does not delete it from the network, but will remove the storage of the record
    /// locally, and will prevent its value from being refreshed on the network by this node.
    #[instrument(target = "veilid_api", level = "debug", ret, err)]
    pub async fn delete_dht_record(&self, key: TypedKey) -> VeilidAPIResult<()> {
        event!(target: "veilid_api", Level::DEBUG, 
            "RoutingContext::delete_dht_record(self: {:?}, key: {:?})", self, key);

        Crypto::validate_crypto_kind(key.kind)?;
        let storage_manager = self.api.storage_manager()?;
        storage_manager.delete_record(key).await
    }

    /// Gets the latest value of a subkey
    ///
    /// May pull the latest value from the network, but by setting 'force_refresh' you can force a network data refresh
    ///
    /// Returns `None` if the value subkey has not yet been set
    /// Returns `Some(data)` if the value subkey has valid data
    #[instrument(target = "veilid_api", level = "debug", ret, err)]
    pub async fn get_dht_value(
        &self,
        key: TypedKey,
        subkey: ValueSubkey,
        force_refresh: bool,
    ) -> VeilidAPIResult<Option<ValueData>> {
        event!(target: "veilid_api", Level::DEBUG, 
            "RoutingContext::get_dht_value(self: {:?}, key: {:?}, subkey: {:?}, force_refresh: {:?})", self, key, subkey, force_refresh);

        Crypto::validate_crypto_kind(key.kind)?;
        let storage_manager = self.api.storage_manager()?;
        storage_manager.get_value(key, subkey, force_refresh).await
    }

    /// Pushes a changed subkey value to the network
    /// The DHT record must first by opened via open_dht_record or create_dht_record.
    ///
    /// The writer, if specified, will override the 'default_writer' specified when the record is opened.
    ///
    /// Returns `None` if the value was successfully put
    /// Returns `Some(data)` if the value put was older than the one available on the network
    #[instrument(target = "veilid_api", level = "debug", ret, err)]
    pub async fn set_dht_value(
        &self,
        key: TypedKey,
        subkey: ValueSubkey,
        data: Vec<u8>,
        writer: Option<KeyPair>,
    ) -> VeilidAPIResult<Option<ValueData>> {
        event!(target: "veilid_api", Level::DEBUG, 
            "RoutingContext::set_dht_value(self: {:?}, key: {:?}, subkey: {:?}, data: {:?}, writer: {:?})", self, key, subkey, data, writer);

        Crypto::validate_crypto_kind(key.kind)?;
        let storage_manager = self.api.storage_manager()?;
        storage_manager.set_value(key, subkey, data, writer).await
    }

    /// Add or update a watch to a DHT value that informs the user via an VeilidUpdate::ValueChange callback when the record has subkeys change.
    /// One remote node will be selected to perform the watch and it will offer an expiration time based on a suggestion, and make an attempt to
    /// continue to report changes via the callback. Nodes that agree to doing watches will be put on our 'ping' list to ensure they are still around
    /// otherwise the watch will be cancelled and will have to be re-watched.
    ///
    /// There is only one watch permitted per record. If a change to a watch is desired, the previous one will be overwritten.
    /// * `key` is the record key to watch. it must first be opened for reading or writing.
    /// * `subkeys` is the the range of subkeys to watch. The range must not exceed 512 discrete non-overlapping or adjacent subranges. If no range is specified, this is equivalent to watching the entire range of subkeys.
    /// * `expiration` is the desired timestamp of when to automatically terminate the watch, in microseconds. If this value is less than `network.rpc.timeout_ms` milliseconds in the future, this function will return an error immediately.
    /// * `count` is the number of times the watch will be sent, maximum. A zero value here is equivalent to a cancellation.
    ///
    /// Returns a timestamp of when the watch will expire. All watches are guaranteed to expire at some point in the future,
    /// and the returned timestamp will be no later than the requested expiration, but -may- be before the requested expiration.
    /// If the returned timestamp is zero it indicates that the watch creation or update has failed. In the case of a faild update, the watch is considered cancelled.
    ///
    /// DHT watches are accepted with the following conditions:
    /// * First-come first-served basis for arbitrary unauthenticated readers, up to network.dht.public_watch_limit per record
    /// * If a member (either the owner or a SMPL schema member) has opened the key for writing (even if no writing is performed) then the watch will be signed and guaranteed network.dht.member_watch_limit per writer
    ///
    /// Members can be specified via the SMPL schema and do not need to allocate writable subkeys in order to offer a member watch capability.
    #[instrument(target = "veilid_api", level = "debug", ret, err)]
    pub async fn watch_dht_values(
        &self,
        key: TypedKey,
        subkeys: ValueSubkeyRangeSet,
        expiration: Timestamp,
        count: u32,
    ) -> VeilidAPIResult<Timestamp> {
        event!(target: "veilid_api", Level::DEBUG, 
            "RoutingContext::watch_dht_values(self: {:?}, key: {:?}, subkeys: {:?}, expiration: {:?}, count: {:?})", self, key, subkeys, expiration, count);

        Crypto::validate_crypto_kind(key.kind)?;
        let storage_manager = self.api.storage_manager()?;
        storage_manager
            .watch_values(key, subkeys, expiration, count)
            .await
    }

    /// Cancels a watch early
    ///
    /// This is a convenience function that cancels watching all subkeys in a range. The subkeys specified here
    /// are subtracted from the watched subkey range. If no range is specified, this is equivalent to cancelling the entire range of subkeys.
    /// Only the subkey range is changed, the expiration and count remain the same.
    /// If no subkeys remain, the watch is entirely cancelled and will receive no more updates.
    /// Returns Ok(true) if there is any remaining watch for this record
    /// Returns Ok(false) if the entire watch has been cancelled
    #[instrument(target = "veilid_api", level = "debug", ret, err)]
    pub async fn cancel_dht_watch(
        &self,
        key: TypedKey,
        subkeys: ValueSubkeyRangeSet,
    ) -> VeilidAPIResult<bool> {
        event!(target: "veilid_api", Level::DEBUG, 
            "RoutingContext::cancel_dht_watch(self: {:?}, key: {:?}, subkeys: {:?}", self, key, subkeys);

        Crypto::validate_crypto_kind(key.kind)?;
        let storage_manager = self.api.storage_manager()?;
        storage_manager.cancel_watch_values(key, subkeys).await
    }

    /// Inspects a DHT record for subkey state.
    /// This is useful for checking if you should push new subkeys to the network, or retrieve the current state of a record from the network
    /// to see what needs updating locally.
    ///
    /// * `key` is the record key to watch. it must first be opened for reading or writing.
    /// * `subkeys` is the the range of subkeys to inspect. The range must not exceed 512 discrete non-overlapping or adjacent subranges.
    ///    If no range is specified, this is equivalent to inspecting the entire range of subkeys. In total, the list of subkeys returned will be truncated at 512 elements.
    /// * `scope` is what kind of range the inspection has:
    ///
    ///   - DHTReportScope::Local
    ///     Results will be only for a locally stored record.
    ///     Useful for seeing what subkeys you have locally and which ones have not been retrieved
    ///
    ///   - DHTReportScope::SyncGet
    ///     Return the local sequence numbers and the network sequence numbers with GetValue fanout parameters
    ///     Provides an independent view of both the local sequence numbers and the network sequence numbers for nodes that
    ///     would be reached as if the local copy did not exist locally.
    ///     Useful for determining if the current local copy should be updated from the network.
    ///
    ///   - DHTReportScope::SyncSet
    ///     Return the local sequence numbers and the network sequence numbers with SetValue fanout parameters
    ///     Provides an independent view of both the local sequence numbers and the network sequence numbers for nodes that
    ///     would be reached as if the local copy did not exist locally.
    ///     Useful for determining if the unchanged local copy should be pushed to the network.
    ///
    ///   - DHTReportScope::UpdateGet
    ///     Return the local sequence numbers and the network sequence numbers with GetValue fanout parameters
    ///     Provides an view of both the local sequence numbers and the network sequence numbers for nodes that
    ///     would be reached as if a GetValue operation were being performed, including accepting newer values from the network.
    ///     Useful for determining which subkeys would change with a GetValue operation
    ///
    ///   - DHTReportScope::UpdateSet
    ///     Return the local sequence numbers and the network sequence numbers with SetValue fanout parameters
    ///     Provides an view of both the local sequence numbers and the network sequence numbers for nodes that
    ///     would be reached as if a SetValue operation were being performed, including accepting newer values from the network.
    ///     This simulates a SetValue with the initial sequence number incremented by 1, like a real SetValue would when updating.
    ///     Useful for determine which subkeys would change with an SetValue operation
    ///
    /// Returns a DHTRecordReport with the subkey ranges that were returned that overlapped the schema, and sequence numbers for each of the subkeys in the range.
    #[instrument(target = "veilid_api", level = "debug", ret, err)]
    pub async fn inspect_dht_record(
        &self,
        key: TypedKey,
        subkeys: ValueSubkeyRangeSet,
        scope: DHTReportScope,
    ) -> VeilidAPIResult<DHTRecordReport> {
        event!(target: "veilid_api", Level::DEBUG, 
            "RoutingContext::inspect_dht_record(self: {:?}, key: {:?}, subkeys: {:?}, scope: {:?})", self, key, subkeys, scope);

        Crypto::validate_crypto_kind(key.kind)?;
        let storage_manager = self.api.storage_manager()?;
        storage_manager.inspect_record(key, subkeys, scope).await
    }

    ///////////////////////////////////
    /// Block Store

    #[cfg(feature = "unstable-blockstore")]
    #[instrument(target = "veilid_api", level = "debug", ret, err)]
    pub async fn find_block(&self, _block_id: PublicKey) -> VeilidAPIResult<Vec<u8>> {
        panic!("unimplemented");
    }

    #[cfg(feature = "unstable-blockstore")]
    #[instrument(target = "veilid_api", level = "debug", ret, err)]
    pub async fn supply_block(&self, _block_id: PublicKey) -> VeilidAPIResult<bool> {
        panic!("unimplemented");
    }
}
