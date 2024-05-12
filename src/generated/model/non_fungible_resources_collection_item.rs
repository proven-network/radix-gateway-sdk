use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct NonFungibleResourcesCollectionItem(pub serde_json::Value);
