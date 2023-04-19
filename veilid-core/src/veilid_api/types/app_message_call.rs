use super::*;

/// Direct statement blob passed to hosting application for processing
#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, RkyvArchive, RkyvSerialize, RkyvDeserialize,
)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct VeilidAppMessage {
    /// Some(sender) if the message was sent directly, None if received via a private/safety route
    #[serde(with = "opt_json_as_string")]
    pub sender: Option<PublicKey>,
    /// The content of the message to deliver to the application
    #[serde(with = "json_as_base64")]
    pub message: Vec<u8>,
}

/// Direct question blob passed to hosting application for processing to send an eventual AppReply
#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, RkyvArchive, RkyvSerialize, RkyvDeserialize,
)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct VeilidAppCall {
    /// Some(sender) if the request was sent directly, None if received via a private/safety route
    #[serde(with = "opt_json_as_string")]
    pub sender: Option<PublicKey>,
    /// The content of the request to deliver to the application
    #[serde(with = "json_as_base64")]
    pub message: Vec<u8>,
    /// The id to reply to
    #[serde(with = "json_as_string")]
    pub id: OperationId,
}
