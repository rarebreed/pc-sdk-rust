# WfmVersionedEntityMetadata

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**version** | **i32** | The version of the associated entity.  Used to prevent conflicts on concurrent edits | 
**modified_by** | Option<[**crate::models::UserReference**](UserReference.md)> |  | [optional]
**date_modified** | Option<**String**> | The date the associated entity was last modified. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**created_by** | Option<[**crate::models::UserReference**](UserReference.md)> |  | [optional]
**date_created** | Option<**String**> | The date the associated entity was created, if available. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


