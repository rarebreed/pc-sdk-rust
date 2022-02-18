# HistoryListing

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [optional]
**complete** | Option<**bool**> |  | [optional]
**user** | Option<[**crate::models::User**](User.md)> |  | [optional]
**client** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**error_message** | Option<**String**> |  | [optional]
**error_code** | Option<**String**> |  | [optional]
**error_details** | Option<[**Vec<crate::models::Detail>**](Detail.md)> |  | [optional]
**error_message_params** | Option<**::std::collections::HashMap<String, String>**> |  | [optional]
**action_name** | Option<**String**> | Action name | [optional]
**action_status** | Option<**String**> | Action status | [optional]
**name** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**system** | Option<**bool**> |  | [optional]
**started** | Option<**String**> | Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**completed** | Option<**String**> | Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**page_size** | Option<**i32**> |  | [optional]
**page_number** | Option<**i32**> |  | [optional]
**total** | Option<**i64**> |  | [optional]
**entities** | Option<[**Vec<crate::models::HistoryEntry>**](HistoryEntry.md)> |  | [optional]
**page_count** | Option<**i32**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


