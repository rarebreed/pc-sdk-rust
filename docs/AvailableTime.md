# AvailableTime

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**date_start** | Option<**String**> | Start of the availability period. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**length_in_minutes** | Option<**i32**> | Length of availability period in minutes | [optional][readonly]
**is_paid** | Option<**bool**> | Indicates if this availability period is paid in Workforce Management schedule | [optional][readonly]
**activity_category** | Option<**String**> | Workforce Management activity category for this availability period | [optional][readonly]
**wfm_schedule** | Option<[**crate::models::WfmScheduleReference**](WfmScheduleReference.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


