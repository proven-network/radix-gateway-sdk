use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct MetadataTypedValue(pub serde_json::Value);
