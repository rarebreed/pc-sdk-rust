# CoachingSlot

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**date_start** | Option<**String**> | Start date and time of scheduled coaching appointment slot. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**length_in_minutes** | Option<**i32**> | Length of coaching appointment slot in minutes | [optional][readonly]
**staffing_difference** | Option<**f64**> | Difference between scheduled and forecast headcount for this slot after scheduling the coaching appointment | [optional][readonly]
**difference_rating** | Option<**String**> | Rating based on the staffing difference for scheduled slot | [optional][readonly]
**wfm_schedule** | Option<[**crate::models::WfmScheduleReference**](WfmScheduleReference.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


