#![allow(non_snake_case)]
use super::*;

#[wasm_bindgen()]
pub struct VeilidRoutingContext {
    inner_routing_context: RoutingContext,
}

#[wasm_bindgen()]
impl VeilidRoutingContext {
    /// Create a new VeilidRoutingContext, without any privacy or sequencing settings.
    #[wasm_bindgen(constructor)]
    pub fn new() -> APIResult<VeilidRoutingContext> {
        let veilid_api = get_veilid_api()?;
        APIResult::Ok(VeilidRoutingContext {
            inner_routing_context: veilid_api.routing_context()?,
        })
    }

    /// Same as `new VeilidRoutingContext()` except easier to chain.
    pub fn create() -> APIResult<VeilidRoutingContext> {
        VeilidRoutingContext::new()
    }

    // --------------------------------
    // Static methods
    // --------------------------------

    /// Allocate a new private route set with default cryptography and network options.
    /// Returns a route id and a publishable 'blob' with the route encrypted with each crypto kind.
    /// Those nodes importing the blob will have their choice of which crypto kind to use.
    ///
    /// Returns a route id and 'blob' that can be published over some means (DHT or otherwise) to be imported by another Veilid node.
    pub async fn newPrivateRoute() -> APIResult<VeilidRouteBlob> {
        let veilid_api = get_veilid_api()?;

        let (route_id, blob) = veilid_api.new_private_route().await?;

        let route_blob = VeilidRouteBlob { route_id, blob };
        APIResult::Ok(route_blob)
    }

    /// Import a private route blob as a remote private route.
    ///
    /// Returns a route id that can be used to send private messages to the node creating this route.
    pub fn importRemotePrivateRoute(&self, blob: String) -> APIResult<RouteId> {
        let blob = unmarshall(blob)?;
        let veilid_api = get_veilid_api()?;
        let route_id = veilid_api.import_remote_private_route(blob)?;
        APIResult::Ok(route_id)
    }

    /// Allocate a new private route and specify a specific cryptosystem, stability and sequencing preference.
    /// Returns a route id and a publishable 'blob' with the route encrypted with each crypto kind.
    /// Those nodes importing the blob will have their choice of which crypto kind to use.
    ///
    /// Returns a route id and 'blob' that can be published over some means (DHT or otherwise) to be imported by another Veilid node.
    pub async fn newCustomPrivateRoute(
        stability: Stability,
        sequencing: Sequencing,
    ) -> APIResult<VeilidRouteBlob> {
        let veilid_api = get_veilid_api()?;

        let (route_id, blob) = veilid_api
            .new_custom_private_route(&veilid_core::VALID_CRYPTO_KINDS, stability, sequencing)
            .await?;

        let route_blob = VeilidRouteBlob { route_id, blob };
        APIResult::Ok(route_blob)
    }

    /// Release either a locally allocated or remotely imported private route.
    ///
    /// This will deactivate the route and free its resources and it can no longer be sent to or received from.
    pub fn releasePrivateRoute(route_id: String) -> APIResult<()> {
        let route_id: veilid_core::RouteId = RouteId::from_str(&route_id)?;
        let veilid_api = get_veilid_api()?;
        veilid_api.release_private_route(route_id)?;
        APIRESULT_UNDEFINED
    }

    /// Respond to an AppCall received over a VeilidUpdate::AppCall.
    ///
    /// * `call_id` - specifies which call to reply to, and it comes from a VeilidUpdate::AppCall, specifically the VeilidAppCall::id() value.
    /// * `message` - is an answer blob to be returned by the remote node's RoutingContext::app_call() function, and may be up to 32768 bytes
    pub async fn appCallReply(call_id: String, message: Box<[u8]>) -> APIResult<()> {
        let message = message.into_vec();
        let call_id = match call_id.parse() {
            Ok(v) => v,
            Err(e) => {
                return APIResult::Err(veilid_core::VeilidAPIError::invalid_argument(
                    e, "call_id", call_id,
                ))
            }
        };
        let veilid_api = get_veilid_api()?;
        veilid_api.app_call_reply(call_id, message).await?;
        APIRESULT_UNDEFINED
    }

    // --------------------------------
    // Instance methods
    // --------------------------------

    fn getRoutingContext(&self) -> APIResult<RoutingContext> {
        APIResult::Ok(self.inner_routing_context.clone())
    }

    /// Turn on sender privacy, enabling the use of safety routes. This is the default and
    /// calling this function is only necessary if you have previously disable safety or used other parameters.
    /// Returns a new instance of VeilidRoutingContext - does not mutate.
    ///
    /// Default values for hop count, stability and sequencing preferences are used.
    ///
    /// * Hop count default is dependent on config, but is set to 1 extra hop.
    /// * Stability default is to choose 'low latency' routes, preferring them over long-term reliability.
    /// * Sequencing default is to have no preference for ordered vs unordered message delivery
    ///
    /// To customize the safety selection in use, use [VeilidRoutingContext::withSafety].
    pub fn withDefaultSafety(&self) -> APIResult<VeilidRoutingContext> {
        let routing_context = self.getRoutingContext()?;
        APIResult::Ok(VeilidRoutingContext {
            inner_routing_context: routing_context.with_default_safety()?,
        })
    }

    /// Use a custom [SafetySelection]. Can be used to disable safety via [SafetySelection::Unsafe]
    /// Returns a new instance of VeilidRoutingContext - does not mutate.
    pub fn withSafety(&self, safety_selection: SafetySelection) -> APIResult<VeilidRoutingContext> {
        let routing_context = self.getRoutingContext()?;
        APIResult::Ok(VeilidRoutingContext {
            inner_routing_context: routing_context.with_safety(safety_selection)?,
        })
    }

    /// Use a specified `Sequencing` preference.
    /// Returns a new instance of VeilidRoutingContext - does not mutate.
    pub fn withSequencing(&self, sequencing: Sequencing) -> APIResult<VeilidRoutingContext> {
        let routing_context = self.getRoutingContext()?;
        APIResult::Ok(VeilidRoutingContext {
            inner_routing_context: routing_context.with_sequencing(sequencing),
        })
    }

    /// Get the safety selection in use on this routing context
    /// @returns the SafetySelection currently in use if successful.
    pub fn safety(&self) -> APIResult<SafetySelection> {
        let routing_context = self.getRoutingContext()?;

        let safety_selection = routing_context.safety();
        APIResult::Ok(safety_selection)
    }
    /// App-level unidirectional message that does not expect any value to be returned.
    ///
    /// Veilid apps may use this for arbitrary message passing.
    ///
    /// @param {string} target - can be either a direct node id or a private route.
    /// @param {string} message - an arbitrary message blob of up to `32768` bytes.
    #[wasm_bindgen(skip_jsdoc)]
    pub async fn appMessage(&self, target_string: String, message: Box<[u8]>) -> APIResult<()> {
        let routing_context = self.getRoutingContext()?;
        let message = message.into_vec();
        let veilid_api = get_veilid_api()?;
        let target = veilid_api.parse_as_target(target_string).await?;
        routing_context.app_message(target, message).await?;
        APIRESULT_UNDEFINED
    }

    /// App-level bidirectional call that expects a response to be returned.
    ///
    /// Veilid apps may use this for arbitrary message passing.
    ///
    /// @param {string} target_string - can be either a direct node id or a private route.
    /// @param {Uint8Array} message - an arbitrary message blob of up to `32768` bytes.
    /// @returns {Uint8Array} an answer blob of up to `32768` bytes.
    #[wasm_bindgen(skip_jsdoc)]
    pub async fn appCall(
        &self,
        target_string: String,
        request: Box<[u8]>,
    ) -> APIResult<Uint8Array> {
        let request: Vec<u8> = request.into_vec();
        let routing_context = self.getRoutingContext()?;

        let veilid_api = get_veilid_api()?;
        let target = veilid_api.parse_as_target(target_string).await?;
        let answer = routing_context.app_call(target, request).await?;
        let answer = Uint8Array::from(answer.as_slice());
        APIResult::Ok(answer)
    }

    /// DHT Records Creates a new DHT record a specified crypto kind and schema
    ///
    /// The record is considered 'open' after the create operation succeeds.
    ///
    /// @returns the newly allocated DHT record's key if successful.
    pub async fn createDhtRecord(
        &self,
        schema: DHTSchema,
        kind: String,
    ) -> APIResult<DHTRecordDescriptor> {
        let crypto_kind = if kind.is_empty() {
            None
        } else {
            Some(veilid_core::FourCC::from_str(&kind)?)
        };
        let routing_context = self.getRoutingContext()?;

        let dht_record_descriptor = routing_context
            .create_dht_record(schema, crypto_kind)
            .await?;
        APIResult::Ok(dht_record_descriptor)
    }

    /// Opens a DHT record at a specific key.
    ///
    /// Associates a secret if one is provided to provide writer capability. Records may only be opened or created. To re-open with a different routing context, first close the value.
    ///
    /// @returns the DHT record descriptor for the opened record if successful.
    /// @param {string} writer - Stringified key pair, in the form of `key:secret` where `key` and `secret` are base64Url encoded.
    /// @param {string} key - key of the DHT record.
    #[wasm_bindgen(skip_jsdoc)]
    pub async fn openDhtRecord(
        &self,
        key: String,
        writer: Option<String>,
    ) -> APIResult<DHTRecordDescriptor> {
        let key = TypedKey::from_str(&key)?;
        let writer = writer
            .map(|writer| KeyPair::from_str(&writer))
            .map_or(APIResult::Ok(None), |r| r.map(Some))?;

        let routing_context = self.getRoutingContext()?;
        let dht_record_descriptor = routing_context.open_dht_record(key, writer).await?;
        APIResult::Ok(dht_record_descriptor)
    }

    /// Closes a DHT record at a specific key that was opened with create_dht_record or open_dht_record.
    ///
    /// Closing a record allows you to re-open it with a different routing context
    pub async fn closeDhtRecord(&self, key: String) -> APIResult<()> {
        let key = TypedKey::from_str(&key)?;
        let routing_context = self.getRoutingContext()?;
        routing_context.close_dht_record(key).await?;
        APIRESULT_UNDEFINED
    }

    /// Deletes a DHT record at a specific key
    ///
    /// If the record is opened, it must be closed before it is deleted.
    /// Deleting a record does not delete it from the network, but will remove the storage of the record locally,
    /// and will prevent its value from being refreshed on the network by this node.
    pub async fn deleteDhtRecord(&self, key: String) -> APIResult<()> {
        let key = TypedKey::from_str(&key)?;
        let routing_context = self.getRoutingContext()?;
        routing_context.delete_dht_record(key).await?;
        APIRESULT_UNDEFINED
    }

    /// Gets the latest value of a subkey.
    ///
    /// May pull the latest value from the network, but by settings 'force_refresh' you can force a network data refresh.
    ///
    /// Returns `undefined` if the value subkey has not yet been set.
    /// Returns a Uint8Array of `data` if the value subkey has valid data.
    pub async fn getDhtValue(
        &self,
        key: String,
        subkey: u32,
        forceRefresh: bool,
    ) -> APIResult<Option<ValueData>> {
        let key = TypedKey::from_str(&key)?;
        let routing_context = self.getRoutingContext()?;
        let res = routing_context
            .get_dht_value(key, subkey, forceRefresh)
            .await?;
        APIResult::Ok(res)
    }

    /// Pushes a changed subkey value to the network
    ///
    /// Returns `undefined` if the value was successfully put.
    /// Returns a Uint8Array of `data` if the value put was older than the one available on the network.
    pub async fn setDhtValue(
        &self,
        key: String,
        subkey: u32,
        data: Box<[u8]>,
        writer: Option<String>,
    ) -> APIResult<Option<ValueData>> {
        let key = TypedKey::from_str(&key)?;
        let data = data.into_vec();
        let writer = writer
            .map(|writer| KeyPair::from_str(&writer))
            .map_or(APIResult::Ok(None), |r| r.map(Some))?;

        let routing_context = self.getRoutingContext()?;
        let res = routing_context
            .set_dht_value(key, subkey, data, writer)
            .await?;
        APIResult::Ok(res)
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
    pub async fn watchDhtValues(
        &self,
        key: String,
        subkeys: Option<ValueSubkeyRangeSet>,
        expiration: Option<String>,
        count: Option<u32>,
    ) -> APIResult<String> {
        let key = TypedKey::from_str(&key)?;
        let subkeys = subkeys.unwrap_or_default();
        let expiration = if let Some(expiration) = expiration {
            veilid_core::Timestamp::from_str(&expiration).map_err(VeilidAPIError::generic)?
        } else {
            veilid_core::Timestamp::default()
        };
        let count = count.unwrap_or(u32::MAX);

        let routing_context = self.getRoutingContext()?;
        let res = routing_context
            .watch_dht_values(key, subkeys, expiration, count)
            .await?;
        APIResult::Ok(res.to_string())
    }

    /// Cancels a watch early
    ///
    /// This is a convenience function that cancels watching all subkeys in a range. The subkeys specified here
    /// are subtracted from the watched subkey range. If no range is specified, this is equivalent to cancelling the entire range of subkeys.
    /// Only the subkey range is changed, the expiration and count remain the same.
    /// If no subkeys remain, the watch is entirely cancelled and will receive no more updates.
    /// Returns true if there is any remaining watch for this record
    /// Returns false if the entire watch has been cancelled
    pub async fn cancelDhtWatch(
        &self,
        key: String,
        subkeys: Option<ValueSubkeyRangeSet>,
    ) -> APIResult<bool> {
        let key = TypedKey::from_str(&key)?;
        let subkeys = subkeys.unwrap_or_default();

        let routing_context = self.getRoutingContext()?;
        let res = routing_context.cancel_dht_watch(key, subkeys).await?;
        APIResult::Ok(res)
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
    pub async fn inspectDhtRecord(
        &self,
        key: String,
        subkeys: Option<ValueSubkeyRangeSet>,
        scope: Option<DHTReportScope>,
    ) -> APIResult<DHTRecordReport> {
        let key = TypedKey::from_str(&key)?;
        let subkeys = subkeys.unwrap_or_default();
        let scope = scope.unwrap_or_default();

        let routing_context = self.getRoutingContext()?;
        let res = routing_context
            .inspect_dht_record(key, subkeys, scope)
            .await?;
        APIResult::Ok(res)
    }
}
