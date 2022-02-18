# WorkdayValuesTrend

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**date_start_workday** | Option<[**String**](string.md)> | The start workday for the query range for the metric value trend. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [optional][readonly]
**date_end_workday** | Option<[**String**](string.md)> | The end workday for the query range for the metric value trend. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [optional][readonly]
**date_reference_workday** | Option<[**String**](string.md)> | The reference workday used to determine the metric definition. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [optional][readonly]
**division** | Option<[**crate::models::Division**](Division.md)> |  | [optional]
**user** | Option<[**crate::models::UserReference**](UserReference.md)> |  | [optional]
**timezone** | Option<**String**> | The time zone used for aggregating metric values | [optional][readonly]
**results** | Option<[**Vec<crate::models::WorkdayValuesMetricItem>**](WorkdayValuesMetricItem.md)> | The metric value trends | [optional][readonly]
**performance_profile** | Option<[**crate::models::AddressableEntityRef**](AddressableEntityRef.md)> |  | [optional]
**metric** | Option<[**crate::models::AddressableEntityRef**](AddressableEntityRef.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


