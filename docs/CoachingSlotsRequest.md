# CoachingSlotsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**interval** | **String** | Range of time to get slots for scheduling coaching appointments. Intervals are represented as an ISO-8601 string. For example: YYYY-MM-DDThh:mm:ss/YYYY-MM-DDThh:mm:ss | 
**length_in_minutes** | **i32** | The duration of coaching appointment to schedule in 15 minutes granularity up to maximum of 60 minutes | 
**attendee_ids** | **Vec<String>** | List of attendees to determine coaching appointment slots | 
**facilitator_ids** | Option<**Vec<String>**> | List of facilitators to determine coaching appointment slots | [optional]
**interruptible_appointment_ids** | Option<**Vec<String>**> | List of appointment ids to exclude from consideration when determining blocked slots | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


