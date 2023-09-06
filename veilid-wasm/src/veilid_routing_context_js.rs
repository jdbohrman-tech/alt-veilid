#![allow(non_snake_case)]
use super::*;

#[wasm_bindgen()]
pub struct VeilidRoutingContext {
    inner_routing_context: Option<RoutingContext>,
}

#[wasm_bindgen()]
impl VeilidRoutingContext {
    /// Don't use this constructor directly.
    /// Use one of the `VeilidRoutingContext.create___()` factory methods instead.
    /// @deprecated
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner_routing_context: None,
        }
    }

    // --------------------------------
    // Constructor factories
    // --------------------------------

    /// Get a new RoutingContext object to use to send messages over the Veilid network.
    pub fn createWithoutPrivacy() -> APIResult<VeilidRoutingContext> {
        let veilid_api = get_veilid_api()?;
        let routing_context = veilid_api.routing_context();
        Ok(VeilidRoutingContext {
            inner_routing_context: Some(routing_context),
        })
    }

    /// Turn on sender privacy, enabling the use of safety routes.
    ///
    /// Default values for hop count, stability and sequencing preferences are used.
    ///
    /// Hop count default is dependent on config, but is set to 1 extra hop.
    /// Stability default is to choose 'low latency' routes, preferring them over long-term reliability.
    /// Sequencing default is to have no preference for ordered vs unordered message delivery
    /// To modify these defaults, use `VeilidRoutingContext.createWithCustomPrivacy()`.
    pub fn createWithPrivacy() -> APIResult<VeilidRoutingContext> {
        let veilid_api = get_veilid_api()?;
        let routing_context = veilid_api.routing_context().with_privacy()?;
        Ok(VeilidRoutingContext {
            inner_routing_context: Some(routing_context),
        })
    }

    /// Turn on privacy using a custom `SafetySelection`
    pub fn createWithCustomPrivacy(
        safety_selection: SafetySelection,
    ) -> APIResult<VeilidRoutingContext> {
        let veilid_api = get_veilid_api()?;
        let routing_context = veilid_api
            .routing_context()
            .with_custom_privacy(safety_selection)?;
        Ok(VeilidRoutingContext {
            inner_routing_context: Some(routing_context),
        })
    }

    /// Use a specified `Sequencing` preference, with or without privacy.
    pub fn createWithSequencing(sequencing: Sequencing) -> APIResult<VeilidRoutingContext> {
        let veilid_api = get_veilid_api()?;
        let routing_context = veilid_api.routing_context().with_sequencing(sequencing);
        Ok(VeilidRoutingContext {
            inner_routing_context: Some(routing_context),
        })
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
        let route_id: veilid_core::RouteId = veilid_core::deserialize_json(&route_id).unwrap();
        let veilid_api = get_veilid_api()?;
        veilid_api.release_private_route(route_id)?;
        APIRESULT_UNDEFINED
    }

    /// Respond to an AppCall received over a VeilidUpdate::AppCall.
    ///
    /// * `call_id` - specifies which call to reply to, and it comes from a VeilidUpdate::AppCall, specifically the VeilidAppCall::id() value.
    /// * `message` - is an answer blob to be returned by the remote node's RoutingContext::app_call() function, and may be up to 32768 bytes
    pub async fn appCallReply(call_id: String, message: String) -> APIResult<()> {
        let message = unmarshall(message);
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
        let Some(routing_context) = &self.inner_routing_context else {
            return APIResult::Err(veilid_core::VeilidAPIError::generic("Unable to getRoutingContext instance. inner_routing_context is None."));
        };
        APIResult::Ok(routing_context.clone())
    }

    /// App-level unidirectional message that does not expect any value to be returned.
    ///
    /// Veilid apps may use this for arbitrary message passing.
    ///
    /// @param {string} target - can be either a direct node id or a private route.
    /// @param {string} message - an arbitrary message blob of up to `32768` bytes.
    #[wasm_bindgen(skip_jsdoc)]
    pub async fn appMessage(&self, target_string: String, message: String) -> APIResult<()> {
        let routing_context = self.getRoutingContext()?;
        let message = unmarshall(message);

        let veilid_api = get_veilid_api()?;
        let target = veilid_api.parse_as_target(target_string).await?;
        routing_context.app_message(target, message).await?;
        APIRESULT_UNDEFINED
    }

    /// App-level bidirectional call that expects a response to be returned.
    ///
    /// Veilid apps may use this for arbitrary message passing.
    ///
    /// @param {string} target_string - can be either a direct node id or a private route, base64Url encoded.
    /// @param {string} message - an arbitrary message blob of up to `32768` bytes, base64Url encoded.
    /// @returns an answer blob of up to `32768` bytes, base64Url encoded.
    #[wasm_bindgen(skip_jsdoc)]
    pub async fn appCall(&self, target_string: String, request: String) -> APIResult<String> {
        let request: Vec<u8> = unmarshall(request);
        let routing_context = self.getRoutingContext()?;

        let veilid_api = get_veilid_api()?;
        let target = veilid_api.parse_as_target(target_string).await?;
        let answer = routing_context.app_call(target, request).await?;
        let answer = marshall(&answer);
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
        let key = TypedKey::from_str(&key).unwrap();
        let writer = match writer {
            Some(writer) => Some(KeyPair::from_str(&writer).unwrap()),
            _ => None,
        };

        let routing_context = self.getRoutingContext()?;
        let dht_record_descriptor = routing_context.open_dht_record(key, writer).await?;
        APIResult::Ok(dht_record_descriptor)
    }

    /// Closes a DHT record at a specific key that was opened with create_dht_record or open_dht_record.
    ///
    /// Closing a record allows you to re-open it with a different routing context
    pub async fn closeDhtRecord(&self, key: String) -> APIResult<()> {
        let key = TypedKey::from_str(&key).unwrap();
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
        let key = TypedKey::from_str(&key).unwrap();
        let routing_context = self.getRoutingContext()?;
        routing_context.delete_dht_record(key).await?;
        APIRESULT_UNDEFINED
    }

    /// Gets the latest value of a subkey.
    ///
    /// May pull the latest value from the network, but by settings 'force_refresh' you can force a network data refresh.
    ///
    /// Returns `undefined` if the value subkey has not yet been set.
    /// Returns base64Url encoded `data` if the value subkey has valid data.
    pub async fn getDhtValue(
        &self,
        key: String,
        subKey: u32,
        forceRefresh: bool,
    ) -> APIResult<Option<ValueData>> {
        let key = TypedKey::from_str(&key).unwrap();
        let routing_context = self.getRoutingContext()?;
        let res = routing_context
            .get_dht_value(key, subKey, forceRefresh)
            .await?;
        APIResult::Ok(res)
    }

    /// Pushes a changed subkey value to the network
    ///
    /// Returns `undefined` if the value was successfully put.
    /// Returns base64Url encoded `data` if the value put was older than the one available on the network.
    pub async fn setDhtValue(
        &self,
        key: String,
        subKey: u32,
        data: String,
    ) -> APIResult<Option<ValueData>> {
        let key = TypedKey::from_str(&key).unwrap();
        let data: Vec<u8> = data_encoding::BASE64URL_NOPAD
            .decode(&data.as_bytes())
            .unwrap();

        let routing_context = self.getRoutingContext()?;
        let res = routing_context.set_dht_value(key, subKey, data).await?;
        APIResult::Ok(res)
    }

    // pub async fn watchDhtValues(
    //     &self,
    //     key: String,
    //     subKeys: ValueSubkeyRangeSet,
    //     expiration: Timestamp,
    //     count: u32,
    // ) -> APIResult<String> {
    //     let key: veilid_core::TypedKey = veilid_core::deserialize_json(&key).unwrap();
    //     let subkeys: veilid_core::ValueSubkeyRangeSet =
    //         veilid_core::deserialize_json(&subkeys).unwrap();
    //     let expiration = veilid_core::Timestamp::from_str(&expiration).unwrap();

    //     let routing_context = {
    //         let rc = (*ROUTING_CONTEXTS).borrow();
    //         let Some(routing_context) = rc.get(&id) else {
    //             return APIResult::Err(veilid_core::VeilidAPIError::invalid_argument("routing_context_watch_dht_values", "id", self.id));
    //         };
    //         routing_context.clone()
    //     };
    //     let res = routing_context
    //         .watch_dht_values(key, subkeys, expiration, count)
    //         .await?;
    //     APIResult::Ok(res.to_string())
    // }

    // pub async fn cancelDhtWatch(id: u32, key: String, subkeys: String) -> Promise {
    //     let key: veilid_core::TypedKey = veilid_core::deserialize_json(&key).unwrap();
    //     let subkeys: veilid_core::ValueSubkeyRangeSet =
    //         veilid_core::deserialize_json(&subkeys).unwrap();

    //     let routing_context = {
    //         let rc = (*ROUTING_CONTEXTS).borrow();
    //         let Some(routing_context) = rc.get(&id) else {
    //                 return APIResult::Err(veilid_core::VeilidAPIError::invalid_argument("routing_context_cancel_dht_watch", "id", self.id));
    //             };
    //         routing_context.clone()
    //     };
    //     let res = routing_context.cancel_dht_watch(key, subkeys).await?;
    //     APIResult::Ok(res)
    // }
}
