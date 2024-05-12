use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct FungibleResourcesCollectionItem(pub serde_json::Value);
