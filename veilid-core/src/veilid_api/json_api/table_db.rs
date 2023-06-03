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
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "db_op")]
pub enum TableDbResponseOp {
    Release,
}
