# BuAgentScheduleShift

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The ID of the shift | [optional]
**start_date** | Option<**String**> | The start date of this shift. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**length_minutes** | Option<**i32**> | The length of this shift in minutes | [optional][readonly]
**activities** | Option<[**Vec<crate::models::BuAgentScheduleActivity>**](BuAgentScheduleActivity.md)> | The activities associated with this shift | [optional]
**manually_edited** | Option<**bool**> | Whether this shift was manually edited. This is only set by clients and is used for rescheduling | [optional]
**schedule** | Option<[**crate::models::BuScheduleReference**](BuScheduleReference.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


