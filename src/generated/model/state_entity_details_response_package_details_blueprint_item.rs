use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct StateEntityDetailsResponsePackageDetailsBlueprintItem {
    ///This type is defined in the Core API as `AuthConfig`. See the Core API documentation for more details.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth_template: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth_template_is_locked: Option<bool>,
    ///This type is defined in the Core API as `BlueprintDefinition`. See the Core API documentation for more details.
    #[serde(default, skip_serializing_if = "serde_json::Value::is_null")]
    pub definition: serde_json::Value,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dependant_entities: Option<Vec<String>>,
    pub name: String,
    ///This type is defined in the Core API as `BlueprintRoyaltyConfig`. See the Core API documentation for more details.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub royalty_config: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub royalty_config_is_locked: Option<bool>,
    pub version: String,
}
impl std::fmt::Display for StateEntityDetailsResponsePackageDetailsBlueprintItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
