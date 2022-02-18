# DomainOrganizationRoleCreate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | **String** | The role name | 
**description** | Option<**String**> |  | [optional]
**default_role_id** | Option<**String**> |  | [optional]
**permissions** | Option<**Vec<String>**> |  | [optional]
**unused_permissions** | Option<**Vec<String>**> | A collection of the permissions the role is not using | [optional][readonly]
**permission_policies** | Option<[**Vec<crate::models::DomainPermissionPolicy>**](DomainPermissionPolicy.md)> |  | [optional]
**user_count** | Option<**i32**> |  | [optional]
**role_needs_update** | Option<**bool**> | Optional unless patch operation. | [optional]
**default** | Option<**bool**> |  | [optional]
**base** | Option<**bool**> |  | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


