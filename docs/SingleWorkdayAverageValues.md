# SingleWorkdayAverageValues

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**date_workday** | Option<[**String**](string.md)> | The targeted workday for average value query. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [optional][readonly]
**division** | Option<[**crate::models::Division**](Division.md)> |  | [optional]
**user** | Option<[**crate::models::UserReference**](UserReference.md)> |  | [optional]
**timezone** | Option<**String**> | The time zone used for aggregating metric values | [optional][readonly]
**results** | Option<[**Vec<crate::models::WorkdayValuesMetricItem>**](WorkdayValuesMetricItem.md)> | The metric value averages | [optional][readonly]
**performance_profile** | Option<[**crate::models::AddressableEntityRef**](AddressableEntityRef.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


