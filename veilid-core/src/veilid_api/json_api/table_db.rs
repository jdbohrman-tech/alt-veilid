use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TableDbRequest {
    db_id: String,
    #[serde(flatten)]
    db_op: TableDbRequestOp,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TableDbResponse {
    db_id: String,
    #[serde(flatten)]
    db_op: TableDbResponseOp,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "db_op")]
pub enum TableDbRequestOp {
    Release,
    GetColumnCount,
    GetKeys {
        col: i32,
    },
    Transact,
    Store {
        col: i32,
        #[serde(with = "json_as_base64")]
        #[schemars(with = "String")]
        key: Vec<u8>,
        #[serde(with = "json_as_base64")]
        #[schemars(with = "String")]
        value: Vec<u8>,
    },
    Load {
        col: i32,
        #[serde(with = "json_as_base64")]
        #[schemars(with = "String")]
        key: Vec<u8>,
    },
    Delete {
        col: i32,
        #[serde(with = "json_as_base64")]
        #[schemars(with = "String")]
        key: Vec<u8>,
    },
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "db_op")]
pub enum TableDbResponseOp {
    Release,
    GetColumnCount {
        value: i32,
    },
    GetKeys {
        #[serde(flatten)]
        #[schemars(with = "ApiResult<Vec<String>>")]
        result: ApiResultWithVecVecU8,
    },
    Transact {
        value: String,
    },
    Store {
        #[serde(flatten)]
        result: ApiResult<()>,
    },
    Load {
        #[serde(flatten)]
        #[schemars(with = "ApiResult<Option<String>>")]
        result: ApiResult<Option<VecU8>>,
    },
    Delete {
        #[serde(flatten)]
        #[schemars(with = "ApiResult<Option<String>>")]
        result: ApiResult<Option<VecU8>>,
    },
}

//////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TableDbTransactionRequest {
    tx_id: String,
    #[serde(flatten)]
    tx_op: TableDbTransactionRequestOp,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TableDbTransactionResponse {
    tx_id: String,
    #[serde(flatten)]
    tx_op: TableDbTransactionResponseOp,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "tx_op")]
pub enum TableDbTransactionRequestOp {
    Commit,
    Rollback,
    Store {
        col: i32,
        #[serde(with = "json_as_base64")]
        #[schemars(with = "String")]
        key: Vec<u8>,
        #[serde(with = "json_as_base64")]
        #[schemars(with = "String")]
        value: Vec<u8>,
    },
    Delete {
        col: i32,
        #[serde(with = "json_as_base64")]
        #[schemars(with = "String")]
        key: Vec<u8>,
    },
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "tx_op")]
pub enum TableDbTransactionResponseOp {
    Commit {
        #[serde(flatten)]
        result: ApiResult<()>,
    },
    Rollback {
        #[serde(flatten)]
        result: ApiResult<()>,
    },
    Store {
        #[serde(flatten)]
        result: ApiResult<()>,
    },
    Delete {
        #[serde(flatten)]
        result: ApiResult<()>,
    },
}
