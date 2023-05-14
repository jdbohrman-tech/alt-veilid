use super::*;

/// Direct statement blob passed to hosting application for processing
#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, RkyvArchive, RkyvSerialize, RkyvDeserialize,
)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct VeilidAppMessage {
    /// Some(sender) if the message was sent directly, None if received via a private/safety route
    #[serde(with = "opt_json_as_string")]
    sender: Option<TypedKey>, xxx continue propagating this publickey->typedkey and get all the FFI done
    /// The content of the message to deliver to the application
    #[serde(with = "json_as_base64")]
    message: Vec<u8>,
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
#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, RkyvArchive, RkyvSerialize, RkyvDeserialize,
)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct VeilidAppCall {
    /// Some(sender) if the request was sent directly, None if received via a private/safety route
    #[serde(with = "opt_json_as_string")]
    sender: Option<TypedKey>,
    /// The content of the request to deliver to the application
    #[serde(with = "json_as_base64")]
    message: Vec<u8>,
    /// The id to reply to
    #[serde(with = "json_as_string")]
    id: OperationId,
}

impl VeilidAppCall {
    pub fn new(sender: Option<TypedKey>, message: Vec<u8>, id: OperationId) -> Self {
        Self {
            sender,
            message,
            id,
        }
    }

    pub fn sender(&self) -> Option<&TypedKey> {
        self.sender.as_ref()
    }
    pub fn message(&self) -> &[u8] {
        &self.message
    }
    pub fn id(&self) -> OperationId {
        self.id
    }
}
