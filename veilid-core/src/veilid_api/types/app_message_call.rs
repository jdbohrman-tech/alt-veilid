use super::*;

/// Direct statement blob passed to hosting application for processing
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct VeilidAppMessage {
    /// Some(sender) if the message was sent directly, None if received via a private/safety route
    #[serde(with = "as_human_opt_string")]
    #[schemars(with = "Option<String>")]
    pub sender: Option<TypedKey>,

    /// The content of the message to deliver to the application
    #[serde(with = "as_human_base64")]
    #[schemars(with = "String")]
    pub message: Vec<u8>,
}

impl VeilidAppMessage {
    pub fn new(sender: Option<TypedKey>, message: Vec<u8>) -> Self {
        Self { sender, message }
    }

    pub fn sender(&self) -> Option<&TypedKey> {
        self.sender.as_ref()
    }
    pub fn message(&self) -> &[u8] {
        &self.message
    }
}

/// Direct question blob passed to hosting application for processing to send an eventual AppReply
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct VeilidAppCall {
    /// Some(sender) if the request was sent directly, None if received via a private/safety route
    #[serde(with = "as_human_opt_string")]
    #[schemars(with = "Option<String>")]
    sender: Option<TypedKey>,

    /// The content of the request to deliver to the application
    #[serde(with = "as_human_base64")]
    #[schemars(with = "String")]
    message: Vec<u8>,

    /// The id to reply to
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

    pub fn sender(&self) -> Option<&TypedKey> {
        self.sender.as_ref()
    }
    pub fn message(&self) -> &[u8] {
        &self.message
    }
    pub fn id(&self) -> OperationId {
        self.call_id
    }
}
