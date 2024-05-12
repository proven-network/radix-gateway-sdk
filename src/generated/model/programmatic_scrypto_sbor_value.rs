use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct ProgrammaticScryptoSborValue(pub serde_json::Value);
