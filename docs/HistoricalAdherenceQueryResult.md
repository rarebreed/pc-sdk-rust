# HistoricalAdherenceQueryResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user_id** | Option<**String**> | The ID of the user for whom the adherence is queried | [optional]
**start_date** | Option<**String**> | Beginning of the date range that was queried, in ISO-8601 format | [optional]
**end_date** | Option<**String**> | End of the date range that was queried, in ISO-8601 format. If it was not set, end date will be set to the queried time | [optional]
**adherence_percentage** | Option<**f64**> | Adherence percentage for this user, in the scale of 0 - 100 | [optional]
**conformance_percentage** | Option<**f64**> | Conformance percentage for this user, in the scale of 0 - 100. Conformance percentage can be greater than 100 when the actual on queue time is greater than the scheduled on queue time for the same period. | [optional]
**impact** | Option<**String**> | The impact of the current adherence state for this user | [optional]
**exception_info** | Option<[**Vec<crate::models::HistoricalAdherenceExceptionInfo>**](HistoricalAdherenceExceptionInfo.md)> | List of adherence exceptions for this user | [optional]
**day_metrics** | Option<[**Vec<crate::models::HistoricalAdherenceDayMetrics>**](HistoricalAdherenceDayMetrics.md)> | Adherence and conformance metrics for days in query range | [optional]
**actuals** | Option<[**Vec<crate::models::HistoricalAdherenceActuals>**](HistoricalAdherenceActuals.md)> | List of actual activity with offset for this user | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


