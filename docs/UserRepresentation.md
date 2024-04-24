# UserRepresentation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [optional]
**username** | Option<**String**> |  | [optional]
**first_name** | Option<**String**> |  | [optional]
**last_name** | Option<**String**> |  | [optional]
**email** | Option<**String**> |  | [optional]
**email_verified** | Option<**bool**> |  | [optional]
**attributes** | Option<[**std::collections::HashMap<String, Vec<String>>**](Vec.md)> |  | [optional]
**user_profile_metadata** | Option<[**models::UserProfileMetadata**](UserProfileMetadata.md)> |  | [optional]
**param_self** | Option<**String**> |  | [optional]
**origin** | Option<**String**> |  | [optional]
**created_timestamp** | Option<**i64**> |  | [optional]
**enabled** | Option<**bool**> |  | [optional]
**totp** | Option<**bool**> |  | [optional]
**federation_link** | Option<**String**> |  | [optional]
**service_account_client_id** | Option<**String**> |  | [optional]
**credentials** | Option<[**Vec<models::CredentialRepresentation>**](CredentialRepresentation.md)> |  | [optional]
**disableable_credential_types** | Option<**Vec<String>**> |  | [optional]
**required_actions** | Option<**Vec<String>**> |  | [optional]
**federated_identities** | Option<[**Vec<models::FederatedIdentityRepresentation>**](FederatedIdentityRepresentation.md)> |  | [optional]
**realm_roles** | Option<**Vec<String>**> |  | [optional]
**client_roles** | Option<[**std::collections::HashMap<String, Vec<String>>**](Vec.md)> |  | [optional]
**client_consents** | Option<[**Vec<models::UserConsentRepresentation>**](UserConsentRepresentation.md)> |  | [optional]
**not_before** | Option<**i32**> |  | [optional]
**application_roles** | Option<[**std::collections::HashMap<String, Vec<String>>**](Vec.md)> |  | [optional]
**social_links** | Option<[**Vec<models::SocialLinkRepresentation>**](SocialLinkRepresentation.md)> |  | [optional]
**groups** | Option<**Vec<String>**> |  | [optional]
**access** | Option<**std::collections::HashMap<String, bool>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


