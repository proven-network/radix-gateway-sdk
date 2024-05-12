use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionAccountDepositPreValidationAuthorizedDepositorBadge(
    pub serde_json::Value,
);
