#![allow(non_snake_case)]
use super::*;

#[wasm_bindgen()]
pub struct VeilidRoutingContext {
    id: u32,
}

#[wasm_bindgen()]
impl VeilidRoutingContext {
    #[wasm_bindgen(constructor)]
    pub fn new(id: u32) -> Self {
        Self { id }
    }

    pub fn createWithoutPrivacy() -> VeilidAPIResult<VeilidRoutingContext> {
        let veilid_api = get_veilid_api()?;
        let routing_context = veilid_api.routing_context();
        let id = add_routing_context(routing_context);
        Ok(VeilidRoutingContext { id })
    }

    pub async fn createWithPrivacy() -> VeilidAPIResult<VeilidRoutingContext> {
        let veilid_api = get_veilid_api()?;
        let routing_context = veilid_api.routing_context().with_privacy()?;
        let id = add_routing_context(routing_context);
        Ok(VeilidRoutingContext { id })
    }

    pub async fn createWithCustomPrivacy(
        safetySelection: SafetySelection,
    ) -> VeilidAPIResult<VeilidRoutingContext> {
        let veilid_api = get_veilid_api()?;
        let routing_context = veilid_api
            .routing_context()
            .with_custom_privacy(safetySelection)?;
        let id = add_routing_context(routing_context);
        Ok(VeilidRoutingContext { id })
    }

    pub fn createWithSequencing(sequencing: Sequencing) -> VeilidAPIResult<VeilidRoutingContext> {
        let veilid_api = get_veilid_api()?;
        let routing_context = veilid_api.routing_context().with_sequencing(sequencing);
        let id = add_routing_context(routing_context);
        Ok(VeilidRoutingContext { id })
    }

    pub async fn appMessage(&self, target_string: String, message: String) -> VeilidAPIResult<()> {
        let routing_context = {
            let rc = (*ROUTING_CONTEXTS).borrow();
            let Some(routing_context) = rc.get(&self.id) else {
                return APIResult::Err(veilid_core::VeilidAPIError::invalid_argument("routing_context_app_message", "id", self.id));
            };
            routing_context.clone()
        };

        let veilid_api = get_veilid_api()?;
        let target = veilid_api.parse_as_target(target_string).await?;
        routing_context
            .app_message(target, message.into_bytes())
            .await?;
        APIRESULT_UNDEFINED
    }

    pub async fn appCall(
        &self,
        id: u32,
        target_string: String,
        request: String,
    ) -> VeilidAPIResult<String> {
        let routing_context = {
            let rc = (*ROUTING_CONTEXTS).borrow();
            let Some(routing_context) = rc.get(&self.id) else {
                return APIResult::Err(veilid_core::VeilidAPIError::invalid_argument("routing_context_app_call", "id", self.id));
            };
            routing_context.clone()
        };

        let veilid_api = get_veilid_api()?;
        let target = veilid_api.parse_as_target(target_string).await?;
        let answer = routing_context
            .app_call(target, request.into_bytes())
            .await?;
        // let answer = data_encoding::BASE64URL_NOPAD.encode(&answer);
        let answer = String::from_utf8_lossy(&answer).into_owned();
        APIResult::Ok(answer)
    }

    pub async fn createDhtRecord(&self, schema: JsValue, kind: u32) -> VeilidAPIResult<JsValue> {
        let schema: DHTSchema = serde_wasm_bindgen::from_value(schema).unwrap();
        let crypto_kind = if kind == 0 {
            None
        } else {
            Some(veilid_core::FourCC::from(kind))
        };
        let routing_context = {
            let rc = (*ROUTING_CONTEXTS).borrow();
            let Some(routing_context) = rc.get(&self.id) else {
                return APIResult::Err(veilid_core::VeilidAPIError::invalid_argument("routing_context_create_dht_record", "id", self.id));
            };
            routing_context.clone()
        };

        let dht_record_descriptor = routing_context
            .create_dht_record(schema, crypto_kind)
            .await?;
        let out = serde_wasm_bindgen::to_value(&dht_record_descriptor).unwrap();
        APIResult::Ok(out)
    }

    pub async fn openDhtRecord(
        &self,
        key: String,
        writer: Option<String>,
    ) -> VeilidAPIResult<JsValue> {
        let key: veilid_core::TypedKey = veilid_core::deserialize_json(&key).unwrap();
        let writer: Option<veilid_core::KeyPair> =
            writer.map(|s| veilid_core::deserialize_json(&s).unwrap());
        let routing_context = {
            let rc = (*ROUTING_CONTEXTS).borrow();
            let Some(routing_context) = rc.get(&self.id) else {
                return APIResult::Err(veilid_core::VeilidAPIError::invalid_argument("routing_context_open_dht_record", "id", self.id));
            };
            routing_context.clone()
        };
        let dht_record_descriptor = routing_context.open_dht_record(key, writer).await?;
        let out = serde_wasm_bindgen::to_value(&dht_record_descriptor).unwrap();
        APIResult::Ok(out)
    }

    pub async fn closeDhtRecord(&self, key: String) -> VeilidAPIResult<()> {
        let key: veilid_core::TypedKey = veilid_core::deserialize_json(&key).unwrap();
        let routing_context = {
            let rc = (*ROUTING_CONTEXTS).borrow();
            let Some(routing_context) = rc.get(&self.id) else {
                return APIResult::Err(veilid_core::VeilidAPIError::invalid_argument("routing_context_close_dht_record", "id", self.id));
            };
            routing_context.clone()
        };
        routing_context.close_dht_record(key).await?;
        APIRESULT_UNDEFINED
    }

    pub async fn deleteDhtRecord(&self, key: String) -> VeilidAPIResult<()> {
        let key: veilid_core::TypedKey = veilid_core::deserialize_json(&key).unwrap();
        let routing_context = {
            let rc = (*ROUTING_CONTEXTS).borrow();
            let Some(routing_context) = rc.get(&self.id) else {
                return APIResult::Err(veilid_core::VeilidAPIError::invalid_argument("routing_context_delete_dht_record", "id", self.id));
            };
            routing_context.clone()
        };
        routing_context.delete_dht_record(key).await?;
        APIRESULT_UNDEFINED
    }

    pub async fn getDhtValue(
        &self,
        key: String,
        subKey: u32,
        forceRefresh: bool,
    ) -> VeilidAPIResult<JsValue> {
        let key: veilid_core::TypedKey = veilid_core::deserialize_json(&key).unwrap();
        let routing_context = {
            let rc = (*ROUTING_CONTEXTS).borrow();
            let Some(routing_context) = rc.get(&self.id) else {
                    return APIResult::Err(veilid_core::VeilidAPIError::invalid_argument("routing_context_get_dht_value", "id", self.id));
                };
            routing_context.clone()
        };
        let res = routing_context
            .get_dht_value(key, subKey, forceRefresh)
            .await?;
        let out = serde_wasm_bindgen::to_value(&res).unwrap();
        APIResult::Ok(out)
    }

    pub async fn setDhtValue(
        &self,
        key: String,
        subKey: u32,
        data: String,
    ) -> VeilidAPIResult<JsValue> {
        let key: veilid_core::TypedKey = veilid_core::deserialize_json(&key).unwrap();
        let data: Vec<u8> = data_encoding::BASE64URL_NOPAD
            .decode(&data.as_bytes())
            .unwrap();

        let routing_context = {
            let rc = (*ROUTING_CONTEXTS).borrow();
            let Some(routing_context) = rc.get(&self.id) else {
                    return APIResult::Err(veilid_core::VeilidAPIError::invalid_argument("routing_context_set_dht_value", "id", self.id));
                };
            routing_context.clone()
        };
        let res = routing_context.set_dht_value(key, subKey, data).await?;
        let out = serde_wasm_bindgen::to_value(&res).unwrap();
        APIResult::Ok(out)
    }

    // pub async fn watchDhtValues(
    //     &self,
    //     key: String,
    //     subKeys: ValueSubkeyRangeSet,
    //     expiration: Timestamp,
    //     count: u32,
    // ) -> VeilidAPIResult<String> {
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

#[wasm_bindgen()]
pub async fn newPrivateRoute() -> VeilidAPIResult<JsValue> {
    let veilid_api = get_veilid_api()?;

    let (route_id, blob) = veilid_api.new_private_route().await?;

    let route_blob = VeilidRouteBlob { route_id, blob };
    let out = serde_wasm_bindgen::to_value(&route_blob).unwrap();
    APIResult::Ok(out)
}

#[wasm_bindgen()]
pub async fn newCustomPrivateRoute(
    stability: Stability,
    sequencing: Sequencing,
) -> VeilidAPIResult<JsValue> {
    let veilid_api = get_veilid_api()?;

    let (route_id, blob) = veilid_api
        .new_custom_private_route(&veilid_core::VALID_CRYPTO_KINDS, stability, sequencing)
        .await?;

    let route_blob = VeilidRouteBlob { route_id, blob };
    let out = serde_wasm_bindgen::to_value(&route_blob).unwrap();
    APIResult::Ok(out)
}

#[wasm_bindgen()]
pub async fn releasePrivateRoute(routeId: String) -> VeilidAPIResult<()> {
    let route_id: veilid_core::RouteId = veilid_core::deserialize_json(&routeId).unwrap();
    let veilid_api = get_veilid_api()?;
    veilid_api.release_private_route(route_id)?;
    APIRESULT_UNDEFINED
}

#[wasm_bindgen()]
pub async fn appCallReply(callId: String, message: String) -> VeilidAPIResult<()> {
    let call_id = match callId.parse() {
        Ok(v) => v,
        Err(e) => {
            return APIResult::Err(veilid_core::VeilidAPIError::invalid_argument(
                e, "call_id", callId,
            ))
        }
    };
    let veilid_api = get_veilid_api()?;
    veilid_api
        .app_call_reply(call_id, message.into_bytes())
        .await?;
    APIRESULT_UNDEFINED
}
