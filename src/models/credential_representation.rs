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
pub struct CredentialRepresentation {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "userLabel", skip_serializing_if = "Option::is_none")]
    pub user_label: Option<String>,
    #[serde(rename = "createdDate", skip_serializing_if = "Option::is_none")]
    pub created_date: Option<i64>,
    #[serde(rename = "secretData", skip_serializing_if = "Option::is_none")]
    pub secret_data: Option<String>,
    #[serde(rename = "credentialData", skip_serializing_if = "Option::is_none")]
    pub credential_data: Option<String>,
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "temporary", skip_serializing_if = "Option::is_none")]
    pub temporary: Option<bool>,
    #[serde(rename = "device", skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    #[serde(rename = "hashedSaltedValue", skip_serializing_if = "Option::is_none")]
    pub hashed_salted_value: Option<String>,
    #[serde(rename = "salt", skip_serializing_if = "Option::is_none")]
    pub salt: Option<String>,
    #[serde(rename = "hashIterations", skip_serializing_if = "Option::is_none")]
    pub hash_iterations: Option<i32>,
    #[serde(rename = "counter", skip_serializing_if = "Option::is_none")]
    pub counter: Option<i32>,
    #[serde(rename = "algorithm", skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,
    #[serde(rename = "digits", skip_serializing_if = "Option::is_none")]
    pub digits: Option<i32>,
    #[serde(rename = "period", skip_serializing_if = "Option::is_none")]
    pub period: Option<i32>,
    #[serde(rename = "config", skip_serializing_if = "Option::is_none")]
    pub config: Option<std::collections::HashMap<String, Vec<String>>>,
}

impl CredentialRepresentation {
    pub fn new() -> CredentialRepresentation {
        CredentialRepresentation {
            id: None,
            r#type: None,
            user_label: None,
            created_date: None,
            secret_data: None,
            credential_data: None,
            priority: None,
            value: None,
            temporary: None,
            device: None,
            hashed_salted_value: None,
            salt: None,
            hash_iterations: None,
            counter: None,
            algorithm: None,
            digits: None,
            period: None,
            config: None,
        }
    }
}
