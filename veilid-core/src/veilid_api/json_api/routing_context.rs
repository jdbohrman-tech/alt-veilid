use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RoutingContextRequest {
    pub rc_id: u32,
    #[serde(flatten)]
    pub rc_op: RoutingContextRequestOp,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RoutingContextResponse {
    pub rc_id: u32,
    #[serde(flatten)]
    pub rc_op: RoutingContextResponseOp,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "rc_op")]
pub enum RoutingContextRequestOp {
    Release,
    WithPrivacy,
    WithCustomPrivacy {
        safety_selection: SafetySelection,
    },
    WithSequencing {
        sequencing: Sequencing,
    },
    AppCall {
        target: String,
        #[serde(with = "as_human_base64")]
        #[schemars(with = "String")]
        message: Vec<u8>,
    },
    AppMessage {
        target: String,
        #[serde(with = "as_human_base64")]
        #[schemars(with = "String")]
        message: Vec<u8>,
    },
    CreateDhtRecord {
        schema: DHTSchema,
        #[schemars(with = "Option<String>")]
        kind: Option<CryptoKind>,
    },
    OpenDhtRecord {
        #[schemars(with = "String")]
        key: TypedKey,
        #[schemars(with = "Option<String>")]
        writer: Option<KeyPair>,
    },
    CloseDhtRecord {
        #[schemars(with = "String")]
        key: TypedKey,
    },
    DeleteDhtRecord {
        #[schemars(with = "String")]
        key: TypedKey,
    },
    GetDhtValue {
        #[schemars(with = "String")]
        key: TypedKey,
        subkey: ValueSubkey,
        force_refresh: bool,
    },
    SetDhtValue {
        #[schemars(with = "String")]
        key: TypedKey,
        subkey: ValueSubkey,
        #[serde(with = "as_human_base64")]
        #[schemars(with = "String")]
        data: Vec<u8>,
    },
    WatchDhtValues {
        #[schemars(with = "String")]
        key: TypedKey,
        subkeys: ValueSubkeyRangeSet,
        expiration: Timestamp,
        count: u32,
    },
    CancelDhtWatch {
        #[schemars(with = "String")]
        key: TypedKey,
        subkeys: ValueSubkeyRangeSet,
    },
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "rc_op")]
pub enum RoutingContextResponseOp {
    InvalidId,
    Release,
    WithPrivacy {
        #[serde(flatten)]
        result: ApiResult<u32>,
    },
    WithCustomPrivacy {
        #[serde(flatten)]
        result: ApiResult<u32>,
    },
    WithSequencing {
        value: u32,
    },
    AppCall {
        #[serde(flatten)]
        #[schemars(with = "ApiResult<String>")]
        result: ApiResultWithVecU8,
    },
    AppMessage {
        #[serde(flatten)]
        result: ApiResult<()>,
    },
    CreateDhtRecord {
        #[serde(flatten)]
        result: ApiResult<Box<DHTRecordDescriptor>>,
    },
    OpenDhtRecord {
        #[serde(flatten)]
        result: ApiResult<Box<DHTRecordDescriptor>>,
    },
    CloseDhtRecord {
        #[serde(flatten)]
        result: ApiResult<()>,
    },
    DeleteDhtRecord {
        #[serde(flatten)]
        result: ApiResult<()>,
    },
    GetDhtValue {
        #[serde(flatten)]
        result: ApiResult<Option<ValueData>>,
    },
    SetDhtValue {
        #[serde(flatten)]
        result: ApiResult<Option<ValueData>>,
    },
    WatchDhtValues {
        #[serde(flatten)]
        result: ApiResult<Timestamp>,
    },
    CancelDhtWatch {
        #[serde(flatten)]
        result: ApiResult<bool>,
    },
}
