use super::*;

mod routing_context;
pub use routing_context::*;

mod table_db;
pub use table_db::*;

mod crypto_system;
pub use crypto_system::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Request {
    /// Operation Id (pairs with Response)
    id: String,
    /// The request operation variant
    #[serde(flatten)]
    op: RequestOp,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Response {
    /// Operation Id (pairs with Request)
    id: String,
    /// The response operation variant
    #[serde(flatten)]
    op: ResponseOp,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "op")]
pub enum RequestOp {
    GetState,
    Attach,
    Detach,
    NewPrivateRoute,
    NewCustomPrivateRoute {
        #[schemars(with = "Vec<String>")]
        crypto_kinds: Vec<CryptoKind>,
        #[serde(default)]
        stability: Stability,
        #[serde(default)]
        sequencing: Sequencing,
    },
    ImportRemotePrivateRoute {
        #[serde(
            serialize_with = "json_as_base64::serialize",
            deserialize_with = "json_as_base64::deserialize"
        )]
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
        #[serde(
            serialize_with = "json_as_base64::serialize",
            deserialize_with = "json_as_base64::deserialize"
        )]
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
    // Crypto
    GetCryptoSystem {
        #[schemars(with = "String")]
        crypto_kind: CryptoKind,
    },
    BestCryptoSystem,
    CryptoSystem(CryptoSystemRequest),
    VerifySignatures {
        #[schemars(with = "Vec<String>")]
        node_ids: Vec<TypedKey>,
        #[serde(
            serialize_with = "json_as_base64::serialize",
            deserialize_with = "json_as_base64::deserialize"
        )]
        #[schemars(with = "String")]
        data: Vec<u8>,
        #[schemars(with = "Vec<String>")]
        signatures: Vec<TypedSignature>,
    },
    GenerateSignatures {
        #[serde(
            serialize_with = "json_as_base64::serialize",
            deserialize_with = "json_as_base64::deserialize"
        )]
        #[schemars(with = "String")]
        data: Vec<u8>,
        #[schemars(with = "Vec<String>")]
        key_pairs: Vec<TypedKeyPair>,
    },
    GenerateKeyPair {
        #[schemars(with = "String")]
        crypto_kind: CryptoKind,
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
    #[serde(
        serialize_with = "json_as_base64::serialize",
        deserialize_with = "json_as_base64::deserialize"
    )]
    #[schemars(with = "String")]
    blob: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "op")]
pub enum ResponseOp {
    GetState {
        #[serde(flatten)]
        result: ApiResult<VeilidState>,
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
    ImportRemotePrivateRoute,
    ReleasePrivateRoute,
    AppCallReply,
    // Routing Context
    NewRoutingContext,
    RoutingContext(RoutingContextResponse),
    // TableDb
    OpenTableDb,
    DeleteTableDb,
    TableDb(TableDbResponse),
    // Crypto
    GetCryptoSystem,
    BestCryptoSystem,
    CryptoSystem(CryptoSystemResponse),
    VerifySignatures,
    GenerateSignatures,
    GenerateKeyPair,
    // Misc
    Now,
    Debug,
    VeilidVersionString,
    VeilidVersion,
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

pub fn emit_schemas(out: &mut HashMap<String, String>) {
    let schema_request = schema_for!(Request);
    let schema_response = schema_for!(Response);

    out.insert(
        "Request".to_owned(),
        serde_json::to_string_pretty(&schema_request).unwrap(),
    );

    out.insert(
        "Response".to_owned(),
        serde_json::to_string_pretty(&schema_response).unwrap(),
    );
}
