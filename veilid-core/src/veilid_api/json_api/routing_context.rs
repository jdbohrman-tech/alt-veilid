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
    WithCustomPrivacy {
        stability: Stability,
    },
    WithSequencing {
        sequencing: Sequencing,
    },
    AppCall {
        target: String,
        #[serde(with = "json_as_base64")]
        #[schemars(with = "String")]
        request: Vec<u8>,
    },
    AppMessage {
        target: String,
        #[serde(with = "json_as_base64")]
        #[schemars(with = "String")]
        message: Vec<u8>,
    },
    CreateDhtRecord {
        #[schemars(with = "String")]
        kind: CryptoKind,
        schema: DHTSchema,
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
        #[serde(with = "json_as_base64")]
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
    Release,
    WithPrivacy {
        value: String,
    },
    WithCustomPrivacy {
        value: String,
    },
    WithSequencing {
        value: String,
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
        result: ApiResult<DHTRecordDescriptor>,
    },
    OpenDhtRecord {
        #[serde(flatten)]
        result: ApiResult<DHTRecordDescriptor>,
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
