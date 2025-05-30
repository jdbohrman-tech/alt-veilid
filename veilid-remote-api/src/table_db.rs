use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TableDbRequest {
    pub db_id: u32,
    #[serde(flatten)]
    pub db_op: TableDbRequestOp,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TableDbResponse {
    pub db_id: u32,
    #[serde(flatten)]
    pub db_op: TableDbResponseOp,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "db_op")]
pub enum TableDbRequestOp {
    Release,
    GetColumnCount,
    GetKeys {
        col: u32,
    },
    Transact,
    Store {
        col: u32,
        #[serde(with = "as_human_base64")]
        #[schemars(with = "String")]
        key: Vec<u8>,
        #[serde(with = "as_human_base64")]
        #[schemars(with = "String")]
        value: Vec<u8>,
    },
    Load {
        col: u32,
        #[serde(with = "as_human_base64")]
        #[schemars(with = "String")]
        key: Vec<u8>,
    },
    Delete {
        col: u32,
        #[serde(with = "as_human_base64")]
        #[schemars(with = "String")]
        key: Vec<u8>,
    },
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "db_op")]
pub enum TableDbResponseOp {
    InvalidId,
    Release,
    GetColumnCount {
        #[serde(flatten)]
        result: ApiResult<u32>,
    },
    GetKeys {
        #[serde(flatten)]
        #[schemars(with = "ApiResult<Vec<String>>")]
        result: ApiResultWithVecVecU8,
    },
    Transact {
        value: u32,
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
    pub tx_id: u32,
    #[serde(flatten)]
    pub tx_op: TableDbTransactionRequestOp,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TableDbTransactionResponse {
    pub tx_id: u32,
    #[serde(flatten)]
    pub tx_op: TableDbTransactionResponseOp,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "tx_op")]
pub enum TableDbTransactionRequestOp {
    Commit,
    Rollback,
    Store {
        col: u32,
        #[serde(with = "as_human_base64")]
        #[schemars(with = "String")]
        key: Vec<u8>,
        #[serde(with = "as_human_base64")]
        #[schemars(with = "String")]
        value: Vec<u8>,
    },
    Delete {
        col: u32,
        #[serde(with = "as_human_base64")]
        #[schemars(with = "String")]
        key: Vec<u8>,
    },
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "tx_op")]
pub enum TableDbTransactionResponseOp {
    InvalidId,
    Commit {
        #[serde(flatten)]
        result: ApiResult<()>,
    },
    Rollback {},
    Store {
        #[serde(flatten)]
        result: ApiResult<()>,
    },
    Delete {
        #[serde(flatten)]
        result: ApiResult<()>,
    },
}
