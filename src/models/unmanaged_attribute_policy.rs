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

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum UnmanagedAttributePolicy {
    #[serde(rename = "ENABLED")]
    Enabled,
    #[serde(rename = "ADMIN_VIEW")]
    AdminView,
    #[serde(rename = "ADMIN_EDIT")]
    AdminEdit,

}

impl ToString for UnmanagedAttributePolicy {
    fn to_string(&self) -> String {
        match self {
            Self::Enabled => String::from("ENABLED"),
            Self::AdminView => String::from("ADMIN_VIEW"),
            Self::AdminEdit => String::from("ADMIN_EDIT"),
        }
    }
}

impl Default for UnmanagedAttributePolicy {
    fn default() -> UnmanagedAttributePolicy {
        Self::Enabled
    }
}

