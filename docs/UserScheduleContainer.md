# UserScheduleContainer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**management_unit_time_zone** | Option<**String**> | The reference time zone used for the management unit | [optional]
**published_schedules** | Option<[**Vec<crate::models::WeekScheduleReference>**](WeekScheduleReference.md)> | References to all published week schedules overlapping the start/end date query parameters | [optional]
**user_schedules** | Option<[**::std::collections::HashMap<String, crate::models::UserSchedule>**](UserSchedule.md)> | Map of user id to user schedule | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


