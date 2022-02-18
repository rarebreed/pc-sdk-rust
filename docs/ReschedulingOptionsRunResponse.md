# ReschedulingOptionsRunResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**existing_schedule** | Option<[**crate::models::BuScheduleReference**](BuScheduleReference.md)> |  | [optional]
**start_date** | Option<**String**> | The start date of the period to reschedule. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**end_date** | Option<**String**> | The end date of the period to reschedule. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**management_units** | Option<[**Vec<crate::models::ReschedulingManagementUnitResponse>**](ReschedulingManagementUnitResponse.md)> | Per-management unit rescheduling options | [optional]
**agent_count** | Option<**i32**> | The number of agents to be considered in the reschedule | [optional]
**activity_code_ids** | Option<**Vec<String>**> | The IDs of the activity codes being considered for reschedule | [optional]
**do_not_change_weekly_paid_time** | Option<**bool**> | Whether weekly paid time is allowed to be changed | [optional]
**do_not_change_daily_paid_time** | Option<**bool**> | Whether daily paid time is allowed to be changed | [optional]
**do_not_change_shift_start_times** | Option<**bool**> | Whether shift start times are allowed to be changed | [optional]
**do_not_change_manually_edited_shifts** | Option<**bool**> | Whether manually edited shifts are allowed to be changed | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


