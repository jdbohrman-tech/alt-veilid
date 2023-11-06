use super::*;

mod routing_context;
pub use routing_context::*;

mod table_db;
pub use table_db::*;

mod crypto_system;
pub use crypto_system::*;

mod process;
pub use process::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Request {
    /// Operation Id (pairs with Response, or empty if unidirectional)
    #[serde(default)]
    pub id: u32,
    /// The request operation variant
    #[serde(flatten)]
    pub op: RequestOp,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type")]
pub enum RecvMessage {
    Response(Response),
    Update(VeilidUpdate),
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Response {
    /// Operation Id (pairs with Request, or empty if unidirectional)
    #[serde(default)]
    pub id: u32,
    /// The response operation variant
    #[serde(flatten)]
    pub op: ResponseOp,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "op")]
pub enum RequestOp {
    Control {
        args: Vec<String>,
    },
    GetState,
    Attach,
    Detach,
    NewPrivateRoute,
    NewCustomPrivateRoute {
        #[schemars(with = "Vec<String>")]
        kinds: Vec<CryptoKind>,
        #[serde(default)]
        stability: Stability,
        #[serde(default)]
        sequencing: Sequencing,
    },
    ImportRemotePrivateRoute {
        #[serde(with = "as_human_base64")]
        #[schemars(with = "String")]
        blob: Vec<u8>,
    },
    ReleasePrivateRoute {
        #[schemars(with = "String")]
        route_id: RouteId,
    },
    AppCallReply {
        #[schemars(with = "String")]
        call_id: OperationId,
        #[serde(with = "as_human_base64")]
        #[schemars(with = "String")]
        message: Vec<u8>,
    },
    // Routing Context
    NewRoutingContext,
    RoutingContext(RoutingContextRequest),
    // TableDb
    OpenTableDb {
        name: String,
        column_count: u32,
    },
    DeleteTableDb {
        name: String,
    },
    TableDb(TableDbRequest),
    TableDbTransaction(TableDbTransactionRequest),
    // Crypto
    GetCryptoSystem {
        #[schemars(with = "String")]
        kind: CryptoKind,
    },
    BestCryptoSystem,
    CryptoSystem(CryptoSystemRequest),
    VerifySignatures {
        #[schemars(with = "Vec<String>")]
        node_ids: Vec<TypedKey>,
        #[serde(with = "as_human_base64")]
        #[schemars(with = "String")]
        data: Vec<u8>,
        #[schemars(with = "Vec<String>")]
        signatures: Vec<TypedSignature>,
    },
    GenerateSignatures {
        #[serde(with = "as_human_base64")]
        #[schemars(with = "String")]
        data: Vec<u8>,
        #[schemars(with = "Vec<String>")]
        key_pairs: Vec<TypedKeyPair>,
    },
    GenerateKeyPair {
        #[schemars(with = "String")]
        kind: CryptoKind,
    },
    // Misc
    Now,
    Debug {
        command: String,
    },
    VeilidVersionString,
    VeilidVersion,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct NewPrivateRouteResult {
    #[schemars(with = "String")]
    route_id: RouteId,
    #[serde(with = "as_human_base64")]
    #[schemars(with = "String")]
    blob: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "op")]
pub enum ResponseOp {
    Control {
        #[serde(flatten)]
        result: ApiResult<String>,
    },
    GetState {
        #[serde(flatten)]
        result: ApiResult<Box<VeilidState>>,
    },
    Attach {
        #[serde(flatten)]
        result: ApiResult<()>,
    },
    Detach {
        #[serde(flatten)]
        result: ApiResult<()>,
    },
    NewPrivateRoute {
        #[serde(flatten)]
        result: ApiResult<NewPrivateRouteResult>,
    },
    NewCustomPrivateRoute {
        #[serde(flatten)]
        result: ApiResult<NewPrivateRouteResult>,
    },
    ImportRemotePrivateRoute {
        #[serde(flatten)]
        #[schemars(with = "ApiResult<String>")]
        result: ApiResultWithString<RouteId>,
    },
    ReleasePrivateRoute {
        #[serde(flatten)]
        result: ApiResult<()>,
    },
    AppCallReply {
        #[serde(flatten)]
        result: ApiResult<()>,
    },
    // Routing Context
    NewRoutingContext {
        #[serde(flatten)]
        result: ApiResult<u32>,
    },
    RoutingContext(Box<RoutingContextResponse>),
    // TableDb
    OpenTableDb {
        #[serde(flatten)]
        result: ApiResult<u32>,
    },
    DeleteTableDb {
        #[serde(flatten)]
        result: ApiResult<bool>,
    },
    TableDb(TableDbResponse),
    TableDbTransaction(TableDbTransactionResponse),
    // Crypto
    GetCryptoSystem {
        #[serde(flatten)]
        result: ApiResult<u32>,
    },
    BestCryptoSystem {
        #[serde(flatten)]
        result: ApiResult<u32>,
    },
    CryptoSystem(CryptoSystemResponse),
    VerifySignatures {
        #[serde(flatten)]
        #[schemars(with = "ApiResult<Vec<String>>")]
        result: ApiResultWithVecString<TypedKeyGroup>,
    },
    GenerateSignatures {
        #[serde(flatten)]
        #[schemars(with = "ApiResult<Vec<String>>")]
        result: ApiResultWithVecString<Vec<TypedSignature>>,
    },
    GenerateKeyPair {
        #[serde(flatten)]
        #[schemars(with = "ApiResult<String>")]
        result: ApiResultWithString<TypedKeyPair>,
    },
    // Misc
    Now {
        #[schemars(with = "String")]
        value: Timestamp,
    },
    Debug {
        #[serde(flatten)]
        result: ApiResult<String>,
    },
    VeilidVersionString {
        value: String,
    },
    VeilidVersion {
        major: u32,
        minor: u32,
        patch: u32,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum ApiResult<T>
where
    T: Clone + fmt::Debug + JsonSchema,
{
    Ok { value: T },
    Err { error: VeilidAPIError },
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum ApiResultWithString<T>
where
    T: Clone + fmt::Debug,
{
    Ok {
        #[schemars(with = "String")]
        value: T,
    },
    Err {
        error: VeilidAPIError,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum ApiResultWithVecU8 {
    Ok {
        #[serde(with = "as_human_base64")]
        #[schemars(with = "String")]
        value: Vec<u8>,
    },
    Err {
        error: VeilidAPIError,
    },
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(transparent)]
pub struct VecU8 {
    #[serde(with = "as_human_base64")]
    #[schemars(with = "String")]
    value: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum ApiResultWithVecVecU8 {
    Ok {
        #[schemars(with = "Vec<String>")]
        value: Vec<VecU8>,
    },
    Err {
        error: VeilidAPIError,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum ApiResultWithVecString<T>
where
    T: Clone + fmt::Debug,
{
    Ok {
        #[schemars(with = "Vec<String>")]
        value: T,
    },
    Err {
        error: VeilidAPIError,
    },
}

pub fn emit_schemas(out: &mut HashMap<String, String>) {
    let schema_request = schema_for!(Request);
    let schema_recv_message = schema_for!(RecvMessage);

    out.insert(
        "Request".to_owned(),
        serde_json::to_string_pretty(&schema_request).unwrap(),
    );

    out.insert(
        "RecvMessage".to_owned(),
        serde_json::to_string_pretty(&schema_recv_message).unwrap(),
    );
}
