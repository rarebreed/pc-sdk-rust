# UpdateCoachingAppointmentRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of coaching appointment. | [optional]
**description** | Option<**String**> | The description of coaching appointment. | [optional]
**date_start** | Option<**String**> | The date/time the coaching appointment starts. Times will be rounded down to the minute. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**length_in_minutes** | Option<**i32**> | The duration of coaching appointment in minutes. | [optional]
**conversation_ids** | Option<**Vec<String>**> | IDs of conversations associated with this coaching appointment. | [optional]
**document_ids** | Option<**Vec<String>**> | IDs of documents associated with this coaching appointment. | [optional]
**status** | Option<**String**> | The status of the coaching appointment. | [optional]
**wfm_schedule** | Option<[**crate::models::WfmScheduleReference**](WfmScheduleReference.md)> |  | [optional]
**external_links** | Option<**Vec<String>**> | The list of external links related to the appointment | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


