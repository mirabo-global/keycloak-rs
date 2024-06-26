/*
 * Keycloak Admin REST API
 *
 * This is a REST API reference for the Keycloak Admin REST API.
 *
 * The version of the OpenAPI document: 1.0
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserProfileAttributeMetadata {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "required", skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(rename = "readOnly", skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(rename = "annotations", skip_serializing_if = "Option::is_none")]
    pub annotations: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "validators", skip_serializing_if = "Option::is_none")]
    pub validators: Option<
        std::collections::HashMap<String, std::collections::HashMap<String, serde_json::Value>>,
    >,
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(rename = "multivalued", skip_serializing_if = "Option::is_none")]
    pub multivalued: Option<bool>,
}

impl UserProfileAttributeMetadata {
    pub fn new() -> UserProfileAttributeMetadata {
        UserProfileAttributeMetadata {
            name: None,
            display_name: None,
            required: None,
            read_only: None,
            annotations: None,
            validators: None,
            group: None,
            multivalued: None,
        }
    }
}
