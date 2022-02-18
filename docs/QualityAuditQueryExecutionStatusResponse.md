# QualityAuditQueryExecutionStatusResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Id of the audit query execution request. | [optional]
**state** | Option<**String**> | Status of the audit query execution request. | [optional]
**date_start** | Option<**String**> | Start date and time of the audit query execution. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**interval** | Option<**String**> | Interval for the audit query. Intervals are represented as an ISO-8601 string. For example: YYYY-MM-DDThh:mm:ss/YYYY-MM-DDThh:mm:ss | [optional]
**filters** | Option<[**Vec<crate::models::QualityAuditQueryFilter>**](QualityAuditQueryFilter.md)> | Filters for the audit query. | [optional]
**sort** | Option<[**Vec<crate::models::AuditQuerySort>**](AuditQuerySort.md)> | Sort parameter for the audit query. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


