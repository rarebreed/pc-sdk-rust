# AuditQueryExecutionResultsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Id of the audit query execution request. | [optional]
**page_size** | Option<**i32**> | Number of results in a page. | [optional]
**cursor** | Option<**String**> | Optional cursor to indicate where to resume the results. | [optional]
**entities** | Option<[**Vec<crate::models::AuditLogMessage>**](AuditLogMessage.md)> | List of audit messages. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


