# HistoricalAdherenceDayMetrics

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**day_start_offset_secs** | Option<**i32**> | Start of day offset in seconds relative to query start time | [optional]
**adherence_schedule_secs** | Option<**i32**> | Duration of schedule in seconds included for adherence percentage calculation | [optional]
**conformance_schedule_secs** | Option<**i32**> | Total scheduled duration in seconds for OnQueue activities | [optional]
**conformance_actual_secs** | Option<**i32**> | Total actually worked duration in seconds for OnQueue activities | [optional]
**exception_count** | Option<**i32**> | Total number of adherence exceptions for this user | [optional]
**exception_duration_secs** | Option<**i32**> | Total duration in seconds of adherence exceptions for this user | [optional]
**impact_seconds** | Option<**i32**> | The impact duration in seconds of current adherence state for this user | [optional]
**schedule_length_secs** | Option<**i32**> | Total duration in seconds for all scheduled activities | [optional]
**actual_length_secs** | Option<**i32**> | Total duration in seconds for all actually worked activities | [optional]
**adherence_percentage** | Option<**f64**> | Total adherence percentage for this user, in the scale of 0 - 100 | [optional]
**conformance_percentage** | Option<**f64**> | Total conformance percentage for this user, in the scale of 0 - 100. Conformance percentage can be greater than 100 when the actual on queue time is greater than the scheduled on queue time for the same period. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


