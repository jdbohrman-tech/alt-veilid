use super::*;

/// Direct statement blob passed to hosting application for processing
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(target_arch = "wasm32", derive(Tsify))]
pub struct VeilidAppMessage {
    #[serde(with = "as_human_opt_string")]
    #[schemars(with = "Option<String>")]
    #[cfg_attr(target_arch = "wasm32", tsify(optional, type = "string"))]
    sender: Option<TypedKey>,

    #[cfg_attr(not(target_arch = "wasm32"), serde(with = "as_human_base64"))]
    #[schemars(with = "String")]
    #[cfg_attr(
        target_arch = "wasm32",
        serde(with = "serde_bytes"),
        tsify(type = "Uint8Array")
    )]
    message: Vec<u8>,
}

impl VeilidAppMessage {
    pub fn new(sender: Option<TypedKey>, message: Vec<u8>) -> Self {
        Self { sender, message }
    }

    /// Some(sender) if the message was sent directly, None if received via a private/safety route
    pub fn sender(&self) -> Option<&TypedKey> {
        self.sender.as_ref()
    }

    /// The content of the message to deliver to the application
    pub fn message(&self) -> &[u8] {
        &self.message
    }
}

/// Direct question blob passed to hosting application for processing to send an eventual AppReply
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(target_arch = "wasm32", derive(Tsify))]
pub struct VeilidAppCall {
    #[serde(with = "as_human_opt_string")]
    #[schemars(with = "Option<String>")]
    #[cfg_attr(target_arch = "wasm32", tsify(optional))]
    sender: Option<TypedKey>,

    #[cfg_attr(not(target_arch = "wasm32"), serde(with = "as_human_base64"))]
    #[schemars(with = "String")]
    #[cfg_attr(
        target_arch = "wasm32",
        serde(with = "serde_bytes"),
        tsify(type = "Uint8Array")
    )]
    message: Vec<u8>,

    #[serde(with = "as_human_string")]
    #[schemars(with = "String")]
    call_id: OperationId,
}

impl VeilidAppCall {
    pub fn new(sender: Option<TypedKey>, message: Vec<u8>, call_id: OperationId) -> Self {
        Self {
            sender,
            message,
            call_id,
        }
    }

    /// Some(sender) if the request was sent directly, None if received via a private/safety route
    pub fn sender(&self) -> Option<&TypedKey> {
        self.sender.as_ref()
    }
    /// The content of the request to deliver to the application
    pub fn message(&self) -> &[u8] {
        &self.message
    }

    /// The id to specify as `call_id` in the [VeilidAPI::app_call_reply] function
    pub fn id(&self) -> OperationId {
        self.call_id
    }
}
