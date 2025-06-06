use super::*;

/// Direct statement blob passed to hosting application for processing.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(all(target_arch = "wasm32", target_os = "unknown"), derive(Tsify))]
#[must_use]
pub struct VeilidAppMessage {
    #[serde(with = "as_human_opt_string")]
    #[schemars(with = "Option<String>")]
    #[cfg_attr(
        all(target_arch = "wasm32", target_os = "unknown"),
        tsify(optional, type = "string")
    )]
    sender: Option<TypedNodeId>,

    #[serde(with = "as_human_opt_string")]
    #[schemars(with = "Option<String>")]
    #[cfg_attr(
        all(target_arch = "wasm32", target_os = "unknown"),
        tsify(optional, type = "string")
    )]
    route_id: Option<RouteId>,

    #[cfg_attr(
        not(all(target_arch = "wasm32", target_os = "unknown")),
        serde(with = "as_human_base64")
    )]
    #[schemars(with = "String")]
    #[cfg_attr(
        all(target_arch = "wasm32", target_os = "unknown"),
        serde(with = "serde_bytes"),
        tsify(type = "Uint8Array")
    )]
    message: Vec<u8>,
}

impl VeilidAppMessage {
    pub fn new(sender: Option<TypedNodeId>, route_id: Option<RouteId>, message: Vec<u8>) -> Self {
        Self {
            sender,
            route_id,
            message,
        }
    }

    /// Some(sender) if the message was sent directly, None if received via a private/safety route.
    #[must_use]
    pub fn sender(&self) -> Option<&TypedNodeId> {
        self.sender.as_ref()
    }

    /// Some(route_id) if the message was received over a private route, None if received only a safety route or directly.
    #[must_use]
    pub fn route_id(&self) -> Option<&RouteId> {
        self.route_id.as_ref()
    }

    /// The content of the message to deliver to the application.
    #[must_use]
    pub fn message(&self) -> &[u8] {
        &self.message
    }
}

/// Direct question blob passed to hosting application for processing to send an eventual AppReply.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(all(target_arch = "wasm32", target_os = "unknown"), derive(Tsify))]
#[must_use]
pub struct VeilidAppCall {
    #[serde(with = "as_human_opt_string")]
    #[schemars(with = "Option<String>")]
    #[cfg_attr(all(target_arch = "wasm32", target_os = "unknown"), tsify(optional))]
    sender: Option<TypedNodeId>,

    #[serde(with = "as_human_opt_string")]
    #[schemars(with = "Option<String>")]
    #[cfg_attr(
        all(target_arch = "wasm32", target_os = "unknown"),
        tsify(optional, type = "string")
    )]
    route_id: Option<RouteId>,

    #[cfg_attr(
        not(all(target_arch = "wasm32", target_os = "unknown")),
        serde(with = "as_human_base64")
    )]
    #[schemars(with = "String")]
    #[cfg_attr(
        all(target_arch = "wasm32", target_os = "unknown"),
        serde(with = "serde_bytes"),
        tsify(type = "Uint8Array")
    )]
    message: Vec<u8>,

    #[serde(with = "as_human_string")]
    #[schemars(with = "String")]
    call_id: OperationId,
}

impl VeilidAppCall {
    pub fn new(
        sender: Option<TypedNodeId>,
        route_id: Option<RouteId>,
        message: Vec<u8>,
        call_id: OperationId,
    ) -> Self {
        Self {
            sender,
            route_id,
            message,
            call_id,
        }
    }

    /// Some(sender) if the request was sent directly, None if received via a private/safety route.
    #[must_use]
    pub fn sender(&self) -> Option<&TypedNodeId> {
        self.sender.as_ref()
    }

    /// Some(route_id) if the request was received over a private route, None if received only a safety route or directly.
    #[must_use]
    pub fn route_id(&self) -> Option<&RouteId> {
        self.route_id.as_ref()
    }

    /// The content of the request to deliver to the application.
    #[must_use]
    pub fn message(&self) -> &[u8] {
        &self.message
    }

    /// The id to specify as `call_id` in the [VeilidAPI::app_call_reply] function.
    pub fn id(&self) -> OperationId {
        self.call_id
    }
}
