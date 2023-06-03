use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RoutingContextRequest {
    rc_id: String,
    #[serde(flatten)]
    rc_op: RoutingContextRequestOp,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RoutingContextResponse {
    rc_id: String,
    #[serde(flatten)]
    rc_op: RoutingContextResponseOp,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "rc_op")]
pub enum RoutingContextRequestOp {
    Release,
    WithPrivacy,
    WithCustomPrivacy,
    WithSequencing,
    AppCall,
    AppMessage,
    CreateDhtRecord,
    OpenDhtRecord,
    CloseDhtRecord,
    DeleteDhtRecord,
    GetDhtValue,
    SetDhtValue,
    WatchDhtValues,
    CancelDhtWatch,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "rc_op")]
pub enum RoutingContextResponseOp {
    Release,
    WithPrivacy,
    WithCustomPrivacy,
    WithSequencing,
    AppCall,
    AppMessage,
    CreateDhtRecord,
    OpenDhtRecord,
    CloseDhtRecord,
    DeleteDhtRecord,
    GetDhtValue,
    SetDhtValue,
    WatchDhtValues,
    CancelDhtWatch,
}
