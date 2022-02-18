# WeekScheduleGenerationResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**failed** | Option<**bool**> | Whether the schedule generation failed | [optional]
**run_id** | Option<**String**> | ID of the schedule run | [optional]
**agent_warnings** | Option<[**Vec<crate::models::ScheduleGenerationWarning>**](ScheduleGenerationWarning.md)> | Warning messages from the schedule run. This will be available only when requesting information for a single week schedule | [optional]
**agent_warning_count** | Option<**i32**> | Count of warning messages from the schedule run. This will be available only when requesting multiple week schedules | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


