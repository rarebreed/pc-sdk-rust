# UserScheduleShift

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**week_schedule** | Option<[**crate::models::WeekScheduleReference**](WeekScheduleReference.md)> |  | [optional]
**id** | Option<**String**> | ID of the schedule shift. This is only for the case of updating and deleting an existing shift | [optional]
**start_date** | Option<**String**> | Start time in UTC for this shift. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**length_in_minutes** | Option<**i32**> | Length of this shift in minutes | [optional][readonly]
**activities** | Option<[**Vec<crate::models::UserScheduleActivity>**](UserScheduleActivity.md)> | List of activities in this shift | [optional]
**delete** | Option<**bool**> | If marked true for updating this schedule shift, it will be deleted | [optional]
**manually_edited** | Option<**bool**> | Whether the shift was set as manually edited | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


