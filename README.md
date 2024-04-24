# Rust API client for keycloak_rs

This is a REST API reference for the Keycloak Admin REST API.


## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 1.0
- Package version: 24.0.3
- Generator version: 7.6.0-SNAPSHOT
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `keycloak_rs` and add the following to `Cargo.toml` under `[dependencies]`:

```
keycloak_rs = { path = "./keycloak_rs" }
```

## Documentation for API Endpoints

All URIs are relative to *http://localhost*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*UsersApi* | [**admin_realms_realm_users_count_get**](docs/UsersApi.md#admin_realms_realm_users_count_get) | **GET** /admin/realms/{realm}/users/count | Returns the number of users that match the given criteria.
*UsersApi* | [**admin_realms_realm_users_get**](docs/UsersApi.md#admin_realms_realm_users_get) | **GET** /admin/realms/{realm}/users | Get users Returns a stream of users, filtered according to query parameters.
*UsersApi* | [**admin_realms_realm_users_post**](docs/UsersApi.md#admin_realms_realm_users_post) | **POST** /admin/realms/{realm}/users | Create a new user Username must be unique.
*UsersApi* | [**admin_realms_realm_users_profile_get**](docs/UsersApi.md#admin_realms_realm_users_profile_get) | **GET** /admin/realms/{realm}/users/profile | 
*UsersApi* | [**admin_realms_realm_users_profile_metadata_get**](docs/UsersApi.md#admin_realms_realm_users_profile_metadata_get) | **GET** /admin/realms/{realm}/users/profile/metadata | 
*UsersApi* | [**admin_realms_realm_users_profile_put**](docs/UsersApi.md#admin_realms_realm_users_profile_put) | **PUT** /admin/realms/{realm}/users/profile | 
*UsersApi* | [**admin_realms_realm_users_user_id_configured_user_storage_credential_types_get**](docs/UsersApi.md#admin_realms_realm_users_user_id_configured_user_storage_credential_types_get) | **GET** /admin/realms/{realm}/users/{user_id}/configured-user-storage-credential-types | Return credential types, which are provided by the user storage where user is stored.
*UsersApi* | [**admin_realms_realm_users_user_id_consents_client_delete**](docs/UsersApi.md#admin_realms_realm_users_user_id_consents_client_delete) | **DELETE** /admin/realms/{realm}/users/{user_id}/consents/{client} | Revoke consent and offline tokens for particular client from user
*UsersApi* | [**admin_realms_realm_users_user_id_consents_get**](docs/UsersApi.md#admin_realms_realm_users_user_id_consents_get) | **GET** /admin/realms/{realm}/users/{user_id}/consents | Get consents granted by the user
*UsersApi* | [**admin_realms_realm_users_user_id_credentials_credential_id_delete**](docs/UsersApi.md#admin_realms_realm_users_user_id_credentials_credential_id_delete) | **DELETE** /admin/realms/{realm}/users/{user_id}/credentials/{credentialId} | Remove a credential for a user
*UsersApi* | [**admin_realms_realm_users_user_id_credentials_credential_id_move_after_new_previous_credential_id_post**](docs/UsersApi.md#admin_realms_realm_users_user_id_credentials_credential_id_move_after_new_previous_credential_id_post) | **POST** /admin/realms/{realm}/users/{user_id}/credentials/{credentialId}/moveAfter/{newPreviousCredentialId} | Move a credential to a position behind another credential
*UsersApi* | [**admin_realms_realm_users_user_id_credentials_credential_id_move_to_first_post**](docs/UsersApi.md#admin_realms_realm_users_user_id_credentials_credential_id_move_to_first_post) | **POST** /admin/realms/{realm}/users/{user_id}/credentials/{credentialId}/moveToFirst | Move a credential to a first position in the credentials list of the user
*UsersApi* | [**admin_realms_realm_users_user_id_credentials_credential_id_user_label_put**](docs/UsersApi.md#admin_realms_realm_users_user_id_credentials_credential_id_user_label_put) | **PUT** /admin/realms/{realm}/users/{user_id}/credentials/{credentialId}/userLabel | Update a credential label for a user
*UsersApi* | [**admin_realms_realm_users_user_id_credentials_get**](docs/UsersApi.md#admin_realms_realm_users_user_id_credentials_get) | **GET** /admin/realms/{realm}/users/{user_id}/credentials | 
*UsersApi* | [**admin_realms_realm_users_user_id_delete**](docs/UsersApi.md#admin_realms_realm_users_user_id_delete) | **DELETE** /admin/realms/{realm}/users/{user_id} | Delete the user
*UsersApi* | [**admin_realms_realm_users_user_id_disable_credential_types_put**](docs/UsersApi.md#admin_realms_realm_users_user_id_disable_credential_types_put) | **PUT** /admin/realms/{realm}/users/{user_id}/disable-credential-types | Disable all credentials for a user of a specific type
*UsersApi* | [**admin_realms_realm_users_user_id_execute_actions_email_put**](docs/UsersApi.md#admin_realms_realm_users_user_id_execute_actions_email_put) | **PUT** /admin/realms/{realm}/users/{user_id}/execute-actions-email | Send an email to the user with a link they can click to execute particular actions.
*UsersApi* | [**admin_realms_realm_users_user_id_federated_identity_get**](docs/UsersApi.md#admin_realms_realm_users_user_id_federated_identity_get) | **GET** /admin/realms/{realm}/users/{user_id}/federated-identity | Get social logins associated with the user
*UsersApi* | [**admin_realms_realm_users_user_id_federated_identity_provider_delete**](docs/UsersApi.md#admin_realms_realm_users_user_id_federated_identity_provider_delete) | **DELETE** /admin/realms/{realm}/users/{user_id}/federated-identity/{provider} | Remove a social login provider from user
*UsersApi* | [**admin_realms_realm_users_user_id_federated_identity_provider_post**](docs/UsersApi.md#admin_realms_realm_users_user_id_federated_identity_provider_post) | **POST** /admin/realms/{realm}/users/{user_id}/federated-identity/{provider} | Add a social login provider to the user
*UsersApi* | [**admin_realms_realm_users_user_id_get**](docs/UsersApi.md#admin_realms_realm_users_user_id_get) | **GET** /admin/realms/{realm}/users/{user_id} | Get representation of the user
*UsersApi* | [**admin_realms_realm_users_user_id_groups_count_get**](docs/UsersApi.md#admin_realms_realm_users_user_id_groups_count_get) | **GET** /admin/realms/{realm}/users/{user_id}/groups/count | 
*UsersApi* | [**admin_realms_realm_users_user_id_groups_get**](docs/UsersApi.md#admin_realms_realm_users_user_id_groups_get) | **GET** /admin/realms/{realm}/users/{user_id}/groups | 
*UsersApi* | [**admin_realms_realm_users_user_id_groups_group_id_delete**](docs/UsersApi.md#admin_realms_realm_users_user_id_groups_group_id_delete) | **DELETE** /admin/realms/{realm}/users/{user_id}/groups/{groupId} | 
*UsersApi* | [**admin_realms_realm_users_user_id_groups_group_id_put**](docs/UsersApi.md#admin_realms_realm_users_user_id_groups_group_id_put) | **PUT** /admin/realms/{realm}/users/{user_id}/groups/{groupId} | 
*UsersApi* | [**admin_realms_realm_users_user_id_impersonation_post**](docs/UsersApi.md#admin_realms_realm_users_user_id_impersonation_post) | **POST** /admin/realms/{realm}/users/{user_id}/impersonation | Impersonate the user
*UsersApi* | [**admin_realms_realm_users_user_id_logout_post**](docs/UsersApi.md#admin_realms_realm_users_user_id_logout_post) | **POST** /admin/realms/{realm}/users/{user_id}/logout | Remove all user sessions associated with the user Also send notification to all clients that have an admin URL to invalidate the sessions for the particular user.
*UsersApi* | [**admin_realms_realm_users_user_id_offline_sessions_client_uuid_get**](docs/UsersApi.md#admin_realms_realm_users_user_id_offline_sessions_client_uuid_get) | **GET** /admin/realms/{realm}/users/{user_id}/offline-sessions/{clientUuid} | Get offline sessions associated with the user and client
*UsersApi* | [**admin_realms_realm_users_user_id_put**](docs/UsersApi.md#admin_realms_realm_users_user_id_put) | **PUT** /admin/realms/{realm}/users/{user_id} | Update the user
*UsersApi* | [**admin_realms_realm_users_user_id_reset_password_email_put**](docs/UsersApi.md#admin_realms_realm_users_user_id_reset_password_email_put) | **PUT** /admin/realms/{realm}/users/{user_id}/reset-password-email | Send an email to the user with a link they can click to reset their password.
*UsersApi* | [**admin_realms_realm_users_user_id_reset_password_put**](docs/UsersApi.md#admin_realms_realm_users_user_id_reset_password_put) | **PUT** /admin/realms/{realm}/users/{user_id}/reset-password | Set up a new password for the user.
*UsersApi* | [**admin_realms_realm_users_user_id_send_verify_email_put**](docs/UsersApi.md#admin_realms_realm_users_user_id_send_verify_email_put) | **PUT** /admin/realms/{realm}/users/{user_id}/send-verify-email | Send an email-verification email to the user An email contains a link the user can click to verify their email address.
*UsersApi* | [**admin_realms_realm_users_user_id_sessions_get**](docs/UsersApi.md#admin_realms_realm_users_user_id_sessions_get) | **GET** /admin/realms/{realm}/users/{user_id}/sessions | Get sessions associated with the user


## Documentation For Models

 - [CredentialRepresentation](docs/CredentialRepresentation.md)
 - [FederatedIdentityRepresentation](docs/FederatedIdentityRepresentation.md)
 - [GroupRepresentation](docs/GroupRepresentation.md)
 - [SocialLinkRepresentation](docs/SocialLinkRepresentation.md)
 - [UnmanagedAttributePolicy](docs/UnmanagedAttributePolicy.md)
 - [UpAttribute](docs/UpAttribute.md)
 - [UpAttributePermissions](docs/UpAttributePermissions.md)
 - [UpAttributeRequired](docs/UpAttributeRequired.md)
 - [UpAttributeSelector](docs/UpAttributeSelector.md)
 - [UpConfig](docs/UpConfig.md)
 - [UpGroup](docs/UpGroup.md)
 - [UserConsentRepresentation](docs/UserConsentRepresentation.md)
 - [UserProfileAttributeGroupMetadata](docs/UserProfileAttributeGroupMetadata.md)
 - [UserProfileAttributeMetadata](docs/UserProfileAttributeMetadata.md)
 - [UserProfileMetadata](docs/UserProfileMetadata.md)
 - [UserRepresentation](docs/UserRepresentation.md)
 - [UserSessionRepresentation](docs/UserSessionRepresentation.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author



