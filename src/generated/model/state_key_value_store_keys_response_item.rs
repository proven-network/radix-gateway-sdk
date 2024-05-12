use serde::{Serialize, Deserialize};
use super::ScryptoSborValue;
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct StateKeyValueStoreKeysResponseItem {
    pub key: ScryptoSborValue,
    ///The most recent state version underlying object was modified at.
    pub last_updated_at_state_version: i64,
}
impl std::fmt::Display for StateKeyValueStoreKeysResponseItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
