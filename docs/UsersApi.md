# \UsersApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**admin_realms_realm_users_count_get**](UsersApi.md#admin_realms_realm_users_count_get) | **GET** /admin/realms/{realm}/users/count | Returns the number of users that match the given criteria.
[**admin_realms_realm_users_get**](UsersApi.md#admin_realms_realm_users_get) | **GET** /admin/realms/{realm}/users | Get users Returns a stream of users, filtered according to query parameters.
[**admin_realms_realm_users_post**](UsersApi.md#admin_realms_realm_users_post) | **POST** /admin/realms/{realm}/users | Create a new user Username must be unique.
[**admin_realms_realm_users_profile_get**](UsersApi.md#admin_realms_realm_users_profile_get) | **GET** /admin/realms/{realm}/users/profile | 
[**admin_realms_realm_users_profile_metadata_get**](UsersApi.md#admin_realms_realm_users_profile_metadata_get) | **GET** /admin/realms/{realm}/users/profile/metadata | 
[**admin_realms_realm_users_profile_put**](UsersApi.md#admin_realms_realm_users_profile_put) | **PUT** /admin/realms/{realm}/users/profile | 
[**admin_realms_realm_users_user_id_configured_user_storage_credential_types_get**](UsersApi.md#admin_realms_realm_users_user_id_configured_user_storage_credential_types_get) | **GET** /admin/realms/{realm}/users/{user_id}/configured-user-storage-credential-types | Return credential types, which are provided by the user storage where user is stored.
[**admin_realms_realm_users_user_id_consents_client_delete**](UsersApi.md#admin_realms_realm_users_user_id_consents_client_delete) | **DELETE** /admin/realms/{realm}/users/{user_id}/consents/{client} | Revoke consent and offline tokens for particular client from user
[**admin_realms_realm_users_user_id_consents_get**](UsersApi.md#admin_realms_realm_users_user_id_consents_get) | **GET** /admin/realms/{realm}/users/{user_id}/consents | Get consents granted by the user
[**admin_realms_realm_users_user_id_credentials_credential_id_delete**](UsersApi.md#admin_realms_realm_users_user_id_credentials_credential_id_delete) | **DELETE** /admin/realms/{realm}/users/{user_id}/credentials/{credentialId} | Remove a credential for a user
[**admin_realms_realm_users_user_id_credentials_credential_id_move_after_new_previous_credential_id_post**](UsersApi.md#admin_realms_realm_users_user_id_credentials_credential_id_move_after_new_previous_credential_id_post) | **POST** /admin/realms/{realm}/users/{user_id}/credentials/{credentialId}/moveAfter/{newPreviousCredentialId} | Move a credential to a position behind another credential
[**admin_realms_realm_users_user_id_credentials_credential_id_move_to_first_post**](UsersApi.md#admin_realms_realm_users_user_id_credentials_credential_id_move_to_first_post) | **POST** /admin/realms/{realm}/users/{user_id}/credentials/{credentialId}/moveToFirst | Move a credential to a first position in the credentials list of the user
[**admin_realms_realm_users_user_id_credentials_credential_id_user_label_put**](UsersApi.md#admin_realms_realm_users_user_id_credentials_credential_id_user_label_put) | **PUT** /admin/realms/{realm}/users/{user_id}/credentials/{credentialId}/userLabel | Update a credential label for a user
[**admin_realms_realm_users_user_id_credentials_get**](UsersApi.md#admin_realms_realm_users_user_id_credentials_get) | **GET** /admin/realms/{realm}/users/{user_id}/credentials | 
[**admin_realms_realm_users_user_id_delete**](UsersApi.md#admin_realms_realm_users_user_id_delete) | **DELETE** /admin/realms/{realm}/users/{user_id} | Delete the user
[**admin_realms_realm_users_user_id_disable_credential_types_put**](UsersApi.md#admin_realms_realm_users_user_id_disable_credential_types_put) | **PUT** /admin/realms/{realm}/users/{user_id}/disable-credential-types | Disable all credentials for a user of a specific type
[**admin_realms_realm_users_user_id_execute_actions_email_put**](UsersApi.md#admin_realms_realm_users_user_id_execute_actions_email_put) | **PUT** /admin/realms/{realm}/users/{user_id}/execute-actions-email | Send an email to the user with a link they can click to execute particular actions.
[**admin_realms_realm_users_user_id_federated_identity_get**](UsersApi.md#admin_realms_realm_users_user_id_federated_identity_get) | **GET** /admin/realms/{realm}/users/{user_id}/federated-identity | Get social logins associated with the user
[**admin_realms_realm_users_user_id_federated_identity_provider_delete**](UsersApi.md#admin_realms_realm_users_user_id_federated_identity_provider_delete) | **DELETE** /admin/realms/{realm}/users/{user_id}/federated-identity/{provider} | Remove a social login provider from user
[**admin_realms_realm_users_user_id_federated_identity_provider_post**](UsersApi.md#admin_realms_realm_users_user_id_federated_identity_provider_post) | **POST** /admin/realms/{realm}/users/{user_id}/federated-identity/{provider} | Add a social login provider to the user
[**admin_realms_realm_users_user_id_get**](UsersApi.md#admin_realms_realm_users_user_id_get) | **GET** /admin/realms/{realm}/users/{user_id} | Get representation of the user
[**admin_realms_realm_users_user_id_groups_count_get**](UsersApi.md#admin_realms_realm_users_user_id_groups_count_get) | **GET** /admin/realms/{realm}/users/{user_id}/groups/count | 
[**admin_realms_realm_users_user_id_groups_get**](UsersApi.md#admin_realms_realm_users_user_id_groups_get) | **GET** /admin/realms/{realm}/users/{user_id}/groups | 
[**admin_realms_realm_users_user_id_groups_group_id_delete**](UsersApi.md#admin_realms_realm_users_user_id_groups_group_id_delete) | **DELETE** /admin/realms/{realm}/users/{user_id}/groups/{groupId} | 
[**admin_realms_realm_users_user_id_groups_group_id_put**](UsersApi.md#admin_realms_realm_users_user_id_groups_group_id_put) | **PUT** /admin/realms/{realm}/users/{user_id}/groups/{groupId} | 
[**admin_realms_realm_users_user_id_impersonation_post**](UsersApi.md#admin_realms_realm_users_user_id_impersonation_post) | **POST** /admin/realms/{realm}/users/{user_id}/impersonation | Impersonate the user
[**admin_realms_realm_users_user_id_logout_post**](UsersApi.md#admin_realms_realm_users_user_id_logout_post) | **POST** /admin/realms/{realm}/users/{user_id}/logout | Remove all user sessions associated with the user Also send notification to all clients that have an admin URL to invalidate the sessions for the particular user.
[**admin_realms_realm_users_user_id_offline_sessions_client_uuid_get**](UsersApi.md#admin_realms_realm_users_user_id_offline_sessions_client_uuid_get) | **GET** /admin/realms/{realm}/users/{user_id}/offline-sessions/{clientUuid} | Get offline sessions associated with the user and client
[**admin_realms_realm_users_user_id_put**](UsersApi.md#admin_realms_realm_users_user_id_put) | **PUT** /admin/realms/{realm}/users/{user_id} | Update the user
[**admin_realms_realm_users_user_id_reset_password_email_put**](UsersApi.md#admin_realms_realm_users_user_id_reset_password_email_put) | **PUT** /admin/realms/{realm}/users/{user_id}/reset-password-email | Send an email to the user with a link they can click to reset their password.
[**admin_realms_realm_users_user_id_reset_password_put**](UsersApi.md#admin_realms_realm_users_user_id_reset_password_put) | **PUT** /admin/realms/{realm}/users/{user_id}/reset-password | Set up a new password for the user.
[**admin_realms_realm_users_user_id_send_verify_email_put**](UsersApi.md#admin_realms_realm_users_user_id_send_verify_email_put) | **PUT** /admin/realms/{realm}/users/{user_id}/send-verify-email | Send an email-verification email to the user An email contains a link the user can click to verify their email address.
[**admin_realms_realm_users_user_id_sessions_get**](UsersApi.md#admin_realms_realm_users_user_id_sessions_get) | **GET** /admin/realms/{realm}/users/{user_id}/sessions | Get sessions associated with the user



## admin_realms_realm_users_count_get

> i32 admin_realms_realm_users_count_get(realm, email, email_verified, enabled, first_name, last_name, q, search, username)
Returns the number of users that match the given criteria.

It can be called in three different ways. 1. Don’t specify any criteria and pass {@code null}. The number of all users within that realm will be returned. <p> 2. If {@code search} is specified other criteria such as {@code last} will be ignored even though you set them. The {@code search} string will be matched against the first and last name, the username and the email of a user. <p> 3. If {@code search} is unspecified but any of {@code last}, {@code first}, {@code email} or {@code username} those criteria are matched against their respective fields on a user entity. Combined with a logical and.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**realm** | **String** | realm name (not id!) | [required] |
**email** | Option<**String**> | email filter |  |
**email_verified** | Option<**bool**> |  |  |
**enabled** | Option<**bool**> | Boolean representing if user is enabled or not |  |
**first_name** | Option<**String**> | first name filter |  |
**last_name** | Option<**String**> | last name filter |  |
**q** | Option<**String**> |  |  |
**search** | Option<**String**> | arbitrary search string for all the fields below. Default search behavior is prefix-based (e.g., foo or foo*). Use *foo* for infix search and \"foo\" for exact search. |  |
**username** | Option<**String**> | username filter |  |

### Return type

**i32**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_realms_realm_users_get

> Vec<models::UserRepresentation> admin_realms_realm_users_get(realm, brief_representation, email, email_verified, enabled, exact, first, first_name, idp_alias, idp_user_id, last_name, max, q, search, username)
Get users Returns a stream of users, filtered according to query parameters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**realm** | **String** | realm name (not id!) | [required] |
**brief_representation** | Option<**bool**> | Boolean which defines whether brief representations are returned (default: false) |  |
**email** | Option<**String**> | A String contained in email, or the complete email, if param \"exact\" is true |  |
**email_verified** | Option<**bool**> | whether the email has been verified |  |
**enabled** | Option<**bool**> | Boolean representing if user is enabled or not |  |
**exact** | Option<**bool**> | Boolean which defines whether the params \"last\", \"first\", \"email\" and \"username\" must match exactly |  |
**first** | Option<**i32**> | Pagination offset |  |
**first_name** | Option<**String**> | A String contained in firstName, or the complete firstName, if param \"exact\" is true |  |
**idp_alias** | Option<**String**> | The alias of an Identity Provider linked to the user |  |
**idp_user_id** | Option<**String**> | The user_id at an Identity Provider linked to the user |  |
**last_name** | Option<**String**> | A String contained in lastName, or the complete lastName, if param \"exact\" is true |  |
**max** | Option<**i32**> | Maximum results size (defaults to 100) |  |
**q** | Option<**String**> | A query to search for custom attributes, in the format 'key1:value2 key2:value2' |  |
**search** | Option<**String**> | A String contained in username, first or last name, or email. Default search behavior is prefix-based (e.g., foo or foo*). Use *foo* for infix search and \"foo\" for exact search. |  |
**username** | Option<**String**> | A String contained in username, or the complete username, if param \"exact\" is true |  |

### Return type

[**Vec<models::UserRepresentation>**](UserRepresentation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_realms_realm_users_post

> admin_realms_realm_users_post(realm, user_representation)
Create a new user Username must be unique.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**realm** | **String** | realm name (not id!) | [required] |
**user_representation** | Option<[**UserRepresentation**](UserRepresentation.md)> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_realms_realm_users_profile_get

> models::UpConfig admin_realms_realm_users_profile_get(realm)


Get the configuration for the user profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**realm** | **String** | realm name (not id!) | [required] |

### Return type

[**models::UpConfig**](UPConfig.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_realms_realm_users_profile_metadata_get

> models::UserProfileMetadata admin_realms_realm_users_profile_metadata_get(realm)


Get the UserProfileMetadata from the configuration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**realm** | **String** | realm name (not id!) | [required] |

### Return type

[**models::UserProfileMetadata**](UserProfileMetadata.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_realms_realm_users_profile_put

> models::UpConfig admin_realms_realm_users_profile_put(realm, up_config)


Set the configuration for the user profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**realm** | **String** | realm name (not id!) | [required] |
**up_config** | Option<[**UpConfig**](UpConfig.md)> |  |  |

### Return type

[**models::UpConfig**](UPConfig.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_realms_realm_users_user_id_configured_user_storage_credential_types_get

> Vec<String> admin_realms_realm_users_user_id_configured_user_storage_credential_types_get(realm, user_id)
Return credential types, which are provided by the user storage where user is stored.

Returned values can contain for example \"password\", \"otp\" etc. This will always return empty list for \"local\" users, which are not backed by any user storage

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**realm** | **String** | realm name (not id!) | [required] |
**user_id** | **String** |  | [required] |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_realms_realm_users_user_id_consents_client_delete

> admin_realms_realm_users_user_id_consents_client_delete(realm, user_id, client)
Revoke consent and offline tokens for particular client from user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**realm** | **String** | realm name (not id!) | [required] |
**user_id** | **String** |  | [required] |
**client** | **String** | Client id | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_realms_realm_users_user_id_consents_get

> Vec<std::collections::HashMap<String, serde_json::Value>> admin_realms_realm_users_user_id_consents_get(realm, user_id)
Get consents granted by the user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**realm** | **String** | realm name (not id!) | [required] |
**user_id** | **String** |  | [required] |

### Return type

[**Vec<std::collections::HashMap<String, serde_json::Value>>**](std::collections::HashMap.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_realms_realm_users_user_id_credentials_credential_id_delete

> admin_realms_realm_users_user_id_credentials_credential_id_delete(realm, user_id, credential_id)
Remove a credential for a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**realm** | **String** | realm name (not id!) | [required] |
**user_id** | **String** |  | [required] |
**credential_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_realms_realm_users_user_id_credentials_credential_id_move_after_new_previous_credential_id_post

> admin_realms_realm_users_user_id_credentials_credential_id_move_after_new_previous_credential_id_post(realm, user_id, credential_id, new_previous_credential_id)
Move a credential to a position behind another credential

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**realm** | **String** | realm name (not id!) | [required] |
**user_id** | **String** |  | [required] |
**credential_id** | **String** | The credential to move | [required] |
**new_previous_credential_id** | **String** | The credential that will be the previous element in the list. If set to null, the moved credential will be the first element in the list. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_realms_realm_users_user_id_credentials_credential_id_move_to_first_post

> admin_realms_realm_users_user_id_credentials_credential_id_move_to_first_post(realm, user_id, credential_id)
Move a credential to a first position in the credentials list of the user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**realm** | **String** | realm name (not id!) | [required] |
**user_id** | **String** |  | [required] |
**credential_id** | **String** | The credential to move | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_realms_realm_users_user_id_credentials_credential_id_user_label_put

> admin_realms_realm_users_user_id_credentials_credential_id_user_label_put(realm, user_id, credential_id, body)
Update a credential label for a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**realm** | **String** | realm name (not id!) | [required] |
**user_id** | **String** |  | [required] |
**credential_id** | **String** |  | [required] |
**body** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_realms_realm_users_user_id_credentials_get

> Vec<models::CredentialRepresentation> admin_realms_realm_users_user_id_credentials_get(realm, user_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**realm** | **String** | realm name (not id!) | [required] |
**user_id** | **String** |  | [required] |

### Return type

[**Vec<models::CredentialRepresentation>**](CredentialRepresentation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_realms_realm_users_user_id_delete

> admin_realms_realm_users_user_id_delete(realm, user_id)
Delete the user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**realm** | **String** | realm name (not id!) | [required] |
**user_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_realms_realm_users_user_id_disable_credential_types_put

> admin_realms_realm_users_user_id_disable_credential_types_put(realm, user_id, request_body)
Disable all credentials for a user of a specific type

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**realm** | **String** | realm name (not id!) | [required] |
**user_id** | **String** |  | [required] |
**request_body** | Option<[**Vec<String>**](String.md)> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_realms_realm_users_user_id_execute_actions_email_put

> admin_realms_realm_users_user_id_execute_actions_email_put(realm, user_id, client_id, lifespan, redirect_uri, request_body)
Send an email to the user with a link they can click to execute particular actions.

An email contains a link the user can click to perform a set of required actions. The redirectUri and clientId parameters are optional. If no redirect is given, then there will be no link back to click after actions have completed. Redirect uri must be a valid uri for the particular clientId.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**realm** | **String** | realm name (not id!) | [required] |
**user_id** | **String** |  | [required] |
**client_id** | Option<**String**> | Client id |  |
**lifespan** | Option<**i32**> | Number of seconds after which the generated token expires |  |
**redirect_uri** | Option<**String**> | Redirect uri |  |
**request_body** | Option<[**Vec<String>**](String.md)> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_realms_realm_users_user_id_federated_identity_get

> Vec<models::FederatedIdentityRepresentation> admin_realms_realm_users_user_id_federated_identity_get(realm, user_id)
Get social logins associated with the user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**realm** | **String** | realm name (not id!) | [required] |
**user_id** | **String** |  | [required] |

### Return type

[**Vec<models::FederatedIdentityRepresentation>**](FederatedIdentityRepresentation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_realms_realm_users_user_id_federated_identity_provider_delete

> admin_realms_realm_users_user_id_federated_identity_provider_delete(realm, user_id, provider)
Remove a social login provider from user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**realm** | **String** | realm name (not id!) | [required] |
**user_id** | **String** |  | [required] |
**provider** | **String** | Social login provider id | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_realms_realm_users_user_id_federated_identity_provider_post

> admin_realms_realm_users_user_id_federated_identity_provider_post(realm, user_id, provider)
Add a social login provider to the user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**realm** | **String** | realm name (not id!) | [required] |
**user_id** | **String** |  | [required] |
**provider** | **String** | Social login provider id | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_realms_realm_users_user_id_get

> models::UserRepresentation admin_realms_realm_users_user_id_get(realm, user_id, user_profile_metadata)
Get representation of the user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**realm** | **String** | realm name (not id!) | [required] |
**user_id** | **String** |  | [required] |
**user_profile_metadata** | Option<**bool**> | Indicates if the user profile metadata should be added to the response |  |

### Return type

[**models::UserRepresentation**](UserRepresentation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_realms_realm_users_user_id_groups_count_get

> std::collections::HashMap<String, i64> admin_realms_realm_users_user_id_groups_count_get(realm, user_id, search)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**realm** | **String** | realm name (not id!) | [required] |
**user_id** | **String** |  | [required] |
**search** | Option<**String**> |  |  |

### Return type

**std::collections::HashMap<String, i64>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_realms_realm_users_user_id_groups_get

> Vec<models::GroupRepresentation> admin_realms_realm_users_user_id_groups_get(realm, user_id, brief_representation, first, max, search)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**realm** | **String** | realm name (not id!) | [required] |
**user_id** | **String** |  | [required] |
**brief_representation** | Option<**bool**> |  |  |[default to true]
**first** | Option<**i32**> |  |  |
**max** | Option<**i32**> |  |  |
**search** | Option<**String**> |  |  |

### Return type

[**Vec<models::GroupRepresentation>**](GroupRepresentation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_realms_realm_users_user_id_groups_group_id_delete

> admin_realms_realm_users_user_id_groups_group_id_delete(realm, user_id, group_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**realm** | **String** | realm name (not id!) | [required] |
**user_id** | **String** |  | [required] |
**group_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_realms_realm_users_user_id_groups_group_id_put

> admin_realms_realm_users_user_id_groups_group_id_put(realm, user_id, group_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**realm** | **String** | realm name (not id!) | [required] |
**user_id** | **String** |  | [required] |
**group_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_realms_realm_users_user_id_impersonation_post

> std::collections::HashMap<String, serde_json::Value> admin_realms_realm_users_user_id_impersonation_post(realm, user_id)
Impersonate the user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**realm** | **String** | realm name (not id!) | [required] |
**user_id** | **String** |  | [required] |

### Return type

[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_realms_realm_users_user_id_logout_post

> admin_realms_realm_users_user_id_logout_post(realm, user_id)
Remove all user sessions associated with the user Also send notification to all clients that have an admin URL to invalidate the sessions for the particular user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**realm** | **String** | realm name (not id!) | [required] |
**user_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_realms_realm_users_user_id_offline_sessions_client_uuid_get

> Vec<models::UserSessionRepresentation> admin_realms_realm_users_user_id_offline_sessions_client_uuid_get(realm, user_id, client_uuid)
Get offline sessions associated with the user and client

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**realm** | **String** | realm name (not id!) | [required] |
**user_id** | **String** |  | [required] |
**client_uuid** | **String** |  | [required] |

### Return type

[**Vec<models::UserSessionRepresentation>**](UserSessionRepresentation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_realms_realm_users_user_id_put

> admin_realms_realm_users_user_id_put(realm, user_id, user_representation)
Update the user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**realm** | **String** | realm name (not id!) | [required] |
**user_id** | **String** |  | [required] |
**user_representation** | Option<[**UserRepresentation**](UserRepresentation.md)> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_realms_realm_users_user_id_reset_password_email_put

> admin_realms_realm_users_user_id_reset_password_email_put(realm, user_id, client_id, redirect_uri)
Send an email to the user with a link they can click to reset their password.

The redirectUri and clientId parameters are optional. The default for the redirect is the account client. This endpoint has been deprecated.  Please use the execute-actions-email passing a list with UPDATE_PASSWORD within it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**realm** | **String** | realm name (not id!) | [required] |
**user_id** | **String** |  | [required] |
**client_id** | Option<**String**> | client id |  |
**redirect_uri** | Option<**String**> | redirect uri |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_realms_realm_users_user_id_reset_password_put

> admin_realms_realm_users_user_id_reset_password_put(realm, user_id, credential_representation)
Set up a new password for the user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**realm** | **String** | realm name (not id!) | [required] |
**user_id** | **String** |  | [required] |
**credential_representation** | Option<[**CredentialRepresentation**](CredentialRepresentation.md)> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_realms_realm_users_user_id_send_verify_email_put

> admin_realms_realm_users_user_id_send_verify_email_put(realm, user_id, client_id, lifespan, redirect_uri)
Send an email-verification email to the user An email contains a link the user can click to verify their email address.

The redirectUri, clientId and lifespan parameters are optional. The default for the redirect is the account client. The default for the lifespan is 12 hours

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**realm** | **String** | realm name (not id!) | [required] |
**user_id** | **String** |  | [required] |
**client_id** | Option<**String**> | Client id |  |
**lifespan** | Option<**i32**> | Number of seconds after which the generated token expires |  |
**redirect_uri** | Option<**String**> | Redirect uri |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_realms_realm_users_user_id_sessions_get

> Vec<models::UserSessionRepresentation> admin_realms_realm_users_user_id_sessions_get(realm, user_id)
Get sessions associated with the user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**realm** | **String** | realm name (not id!) | [required] |
**user_id** | **String** |  | [required] |

### Return type

[**Vec<models::UserSessionRepresentation>**](UserSessionRepresentation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

