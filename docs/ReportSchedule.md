# ReportSchedule

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**quartz_cron_expression** | **String** | Quartz Cron Expression | 
**next_fire_time** | Option<**String**> | Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**date_created** | Option<**String**> | Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**date_modified** | Option<**String**> | Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**description** | Option<**String**> |  | [optional]
**time_zone** | Option<**String**> |  | [optional]
**time_period** | Option<**String**> |  | [optional]
**interval** | **String** | Interval. Intervals are represented as an ISO-8601 string. For example: YYYY-MM-DDThh:mm:ss/YYYY-MM-DDThh:mm:ss | 
**report_format** | Option<**String**> |  | [optional]
**locale** | Option<**String**> |  | [optional]
**enabled** | Option<**bool**> |  | [optional]
**report_id** | **String** | Report ID | 
**parameters** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**last_run** | Option<[**crate::models::ReportRunEntry**](ReportRunEntry.md)> |  | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


