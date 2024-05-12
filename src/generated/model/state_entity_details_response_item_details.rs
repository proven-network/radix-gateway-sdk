use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct StateEntityDetailsResponseItemDetails(pub serde_json::Value);
