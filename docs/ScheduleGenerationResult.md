# ScheduleGenerationResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**failed** | Option<**bool**> | Whether the schedule generation run failed | [optional]
**run_id** | Option<**String**> | The ID of the schedule generation run. Reference this when requesting support | [optional]
**message_count** | Option<**i32**> | The number of schedule generation messages for this schedule generation run | [optional]
**messages** | Option<[**Vec<crate::models::ScheduleGenerationMessage>**](ScheduleGenerationMessage.md)> | User facing messages related to the schedule generation run | [optional]
**message_severities** | Option<[**Vec<crate::models::SchedulerMessageTypeSeverity>**](SchedulerMessageTypeSeverity.md)> | The list of messages by severity in this schedule generation run | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


