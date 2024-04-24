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
pub struct UpAttributePermissions {
    #[serde(rename = "view", skip_serializing_if = "Option::is_none")]
    pub view: Option<Vec<String>>,
    #[serde(rename = "edit", skip_serializing_if = "Option::is_none")]
    pub edit: Option<Vec<String>>,
}

impl UpAttributePermissions {
    pub fn new() -> UpAttributePermissions {
        UpAttributePermissions {
            view: None,
            edit: None,
        }
    }
}
