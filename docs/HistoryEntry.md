# HistoryEntry

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**action** | Option<**String**> | The action performed | [optional]
**resource** | Option<**String**> | For actions performed not on the item itself, but on a sub-item, this field identifies the sub-item by name.  For example, for actions performed on prompt resources, this will be the prompt resource name. | [optional]
**timestamp** | Option<**String**> | Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**user** | Option<[**crate::models::User**](User.md)> |  | [optional]
**client** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**version** | Option<**String**> |  | [optional]
**secure** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


