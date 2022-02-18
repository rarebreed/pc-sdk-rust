# CreateWorkPlanActivity

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**activity_code_id** | Option<**String**> | ID of the activity code associated with this activity | [optional]
**description** | Option<**String**> | Description of the activity | [optional]
**length_minutes** | Option<**i32**> | Length of the activity in minutes | [optional]
**start_time_is_relative_to_shift_start** | Option<**bool**> | Whether the start time of the activity is relative to the start time of the shift it belongs to | [optional]
**flexible_start_time** | Option<**bool**> | Whether the start time of the activity is flexible | [optional]
**earliest_start_time_minutes** | Option<**i32**> | Earliest activity start in offset minutes relative to shift start time if startTimeIsRelativeToShiftStart == true else its based on midnight. Used if flexibleStartTime == true | [optional]
**latest_start_time_minutes** | Option<**i32**> | Latest activity start in offset minutes relative to shift start time if startTimeIsRelativeToShiftStart == true else its based on midnight. Used if flexibleStartTime == true | [optional]
**exact_start_time_minutes** | Option<**i32**> | Exact activity start in offset minutes relative to shift start time if startTimeIsRelativeToShiftStart == true else its based on midnight. Used if flexibleStartTime == false | [optional]
**start_time_increment_minutes** | Option<**i32**> | Increment in offset minutes that would contribute to different possible start times for the activity | [optional]
**counts_as_paid_time** | Option<**bool**> | Whether the activity is paid | [optional]
**counts_as_contiguous_work_time** | Option<**bool**> | Whether the activity duration is counted towards contiguous work time | [optional]
**minimum_length_from_shift_start_minutes** | Option<**i32**> | The minimum duration between shift start and shift item (e.g., break or meal) start in minutes | [optional]
**minimum_length_from_shift_end_minutes** | Option<**i32**> | The minimum duration between shift item (e.g., break or meal) end and shift end in minutes | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


