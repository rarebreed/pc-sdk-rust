# BuAgentScheduleHistoryResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**prior_published_schedules** | Option<[**Vec<crate::models::BuScheduleReference>**](BuScheduleReference.md)> | The list of previously published schedules | [optional]
**base_published_schedule** | Option<[**crate::models::BuAgentScheduleHistoryChange**](BuAgentScheduleHistoryChange.md)> |  | [optional]
**dropped_changes** | Option<[**Vec<crate::models::BuAgentScheduleHistoryDroppedChange>**](BuAgentScheduleHistoryDroppedChange.md)> | The changes dropped from the schedule history. This will happen if the schedule history is too large | [optional]
**changes** | Option<[**Vec<crate::models::BuAgentScheduleHistoryChange>**](BuAgentScheduleHistoryChange.md)> | The list of changes for the schedule history | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


