# OAuthClient

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | **String** | The name of the OAuth client. | 
**access_token_validity_seconds** | Option<**i64**> | The number of seconds, between 5mins and 48hrs, until tokens created with this client expire. If this field is omitted, a default of 24 hours will be applied. | [optional]
**description** | Option<**String**> |  | [optional]
**registered_redirect_uri** | Option<**Vec<String>**> | List of allowed callbacks for this client. For example: https://myap.example.com/auth/callback | [optional]
**secret** | Option<**String**> | System created secret assigned to this client. Secrets are required for code authorization and client credential grants. | [optional]
**role_ids** | Option<**Vec<String>**> | Deprecated. Use roleDivisions instead. | [optional]
**date_created** | Option<**String**> | Date this client was created. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**date_modified** | Option<**String**> | Date this client was last modified. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**created_by** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**modified_by** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**authorized_grant_type** | **String** | The OAuth Grant/Client type supported by this client. Code Authorization Grant/Client type - Preferred client type where the Client ID and Secret are required to create tokens. Used where the secret can be secured. PKCE-Enabled Code Authorization grant type - Code grant type which requires PKCE challenge and verifier to create tokens. Used in public clients for increased security. Implicit grant type - Client ID only is required to create tokens. Used in browser and mobile apps where the secret can not be secured. SAML2-Bearer extension grant type - SAML2 assertion provider for user authentication at the token endpoint. Client Credential grant type - Used to created access tokens that are tied only to the client.  | 
**scope** | Option<**Vec<String>**> | The scope requested by this client. Scopes only apply to clients not using the client_credential grant | [optional]
**role_divisions** | Option<[**Vec<crate::models::RoleDivision>**](RoleDivision.md)> | Set of roles and their corresponding divisions associated with this client. Roles and divisions only apply to clients using the client_credential grant | [optional]
**state** | Option<**String**> | The state of the OAuth client. Active: The OAuth client can be used to create access tokens. This is the default state. Disabled: Access tokens created by the client are invalid and new ones cannot be created. Inactive: Access tokens cannot be created with this OAuth client and it will be deleted. | [optional]
**date_to_delete** | Option<**String**> | The time at which this client will be deleted. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


