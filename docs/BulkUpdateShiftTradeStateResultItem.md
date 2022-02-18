# BulkUpdateShiftTradeStateResultItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**state** | Option<**String**> | The state of the shift trade after the update request is processed | [optional]
**reviewed_by** | Option<[**crate::models::UserReference**](UserReference.md)> |  | [optional]
**reviewed_date** | Option<**String**> | The date the request was reviewed, if applicable. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**failure_reason** | Option<**String**> | The reason the update failed, if applicable | [optional]
**metadata** | Option<[**crate::models::WfmVersionedEntityMetadata**](WfmVersionedEntityMetadata.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


