use serde::{Serialize, Deserialize};
use super::{AccountResourcePreferencesResponseItem, ResultSetCursorMixin};
///Account resource preferences collection
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AccountResourcePreferencesCollection {
    #[serde(flatten)]
    pub result_set_cursor_mixin: ResultSetCursorMixin,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<AccountResourcePreferencesResponseItem>,
}
impl std::fmt::Display for AccountResourcePreferencesCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for AccountResourcePreferencesCollection {
    type Target = ResultSetCursorMixin;
    fn deref(&self) -> &Self::Target {
        &self.result_set_cursor_mixin
    }
}
impl std::ops::DerefMut for AccountResourcePreferencesCollection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.result_set_cursor_mixin
    }
}
