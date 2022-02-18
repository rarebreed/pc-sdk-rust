# UserAuthorization

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**roles** | Option<[**Vec<crate::models::DomainRole>**](DomainRole.md)> |  | [optional]
**unused_roles** | Option<[**Vec<crate::models::DomainRole>**](DomainRole.md)> | A collection of the roles the user is not using | [optional][readonly]
**permissions** | Option<**Vec<String>**> | A collection of the permissions granted by all assigned roles | [optional][readonly]
**permission_policies** | Option<[**Vec<crate::models::ResourcePermissionPolicy>**](ResourcePermissionPolicy.md)> | The policies configured for assigned permissions. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


