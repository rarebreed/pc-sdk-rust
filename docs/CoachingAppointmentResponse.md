# CoachingAppointmentResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> | The name of coaching appointment | [optional][readonly]
**description** | Option<**String**> | The description of coaching appointment | [optional][readonly]
**date_start** | Option<**String**> | The date/time the coaching appointment starts. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**length_in_minutes** | Option<**i32**> | The duration of coaching appointment in minutes | [optional][readonly]
**status** | Option<**String**> | The status of coaching appointment | [optional][readonly]
**facilitator** | Option<[**crate::models::UserReference**](UserReference.md)> |  | [optional]
**attendees** | Option<[**Vec<crate::models::UserReference>**](UserReference.md)> | The list of attendees attending the coaching | [optional][readonly]
**created_by** | Option<[**crate::models::UserReference**](UserReference.md)> |  | [optional]
**date_created** | Option<**String**> | The date/time the coaching appointment was created. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**modified_by** | Option<[**crate::models::UserReference**](UserReference.md)> |  | [optional]
**date_modified** | Option<**String**> | The date/time the coaching appointment was last modified. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**conversations** | Option<[**Vec<crate::models::ConversationReference>**](ConversationReference.md)> | The list of conversations associated with coaching appointment. | [optional][readonly]
**documents** | Option<[**Vec<crate::models::DocumentReference>**](DocumentReference.md)> | The list of documents associated with coaching appointment. | [optional][readonly]
**is_overdue** | Option<**bool**> | Whether the appointment is overdue. | [optional][readonly]
**wfm_schedule** | Option<[**crate::models::WfmScheduleReference**](WfmScheduleReference.md)> |  | [optional]
**date_completed** | Option<**String**> | The date/time the coaching appointment was set to completed status. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**external_links** | Option<**Vec<String>**> | The list of external links related to the appointment | [optional][readonly]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


