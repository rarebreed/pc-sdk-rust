# AuditQueryRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**interval** | **String** | Date and time range of data to query. Intervals are represented as an ISO-8601 string. For example: YYYY-MM-DDThh:mm:ss/YYYY-MM-DDThh:mm:ss | 
**service_name** | **String** | Name of the service to query audits for. | 
**filters** | Option<[**Vec<crate::models::AuditQueryFilter>**](AuditQueryFilter.md)> | Additional filters for the query. | [optional]
**sort** | Option<[**Vec<crate::models::AuditQuerySort>**](AuditQuerySort.md)> | Sort parameter for the query. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


