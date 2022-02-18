# ScimUserExtensions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**routing_skills** | Option<[**Vec<crate::models::ScimUserRoutingSkill>**](ScimUserRoutingSkill.md)> | The list of routing skills assigned to a user. Maximum 50 skills. | [optional]
**routing_languages** | Option<[**Vec<crate::models::ScimUserRoutingLanguage>**](ScimUserRoutingLanguage.md)> | The list of routing languages assigned to a user. Maximum 50 languages. | [optional]
**external_ids** | Option<[**Vec<crate::models::ScimGenesysUserExternalId>**](ScimGenesysUserExternalId.md)> | The list of external identifiers assigned to user. Always includes an immutable SCIM authority prefixed with \"x-pc:scimv2:v1\". ExternalIds are searchable with complex filter query parameter using 'authority' and 'value', e.g., filter=urn:ietf:params:scim:schemas:extension:genesys:purecloud:2.0:User:externalIds[authority eq \"matchAuthName\" and value eq \"matchingExternalKeyValue\"]. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


