use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountAuthorizedDepositorsResponseItem(pub serde_json::Value);
