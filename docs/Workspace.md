# Workspace

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | **String** | The current name of the workspace. | 
**_type** | Option<**String**> |  | [optional]
**is_current_user_workspace** | Option<**bool**> |  | [optional]
**user** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**bucket** | Option<**String**> |  | [optional]
**date_created** | Option<**String**> | Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**date_modified** | Option<**String**> | Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**summary** | Option<[**crate::models::WorkspaceSummary**](WorkspaceSummary.md)> |  | [optional]
**acl** | Option<**Vec<String>**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


