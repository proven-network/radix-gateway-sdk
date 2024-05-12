use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct PublicKey(pub serde_json::Value);
