# Email

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**state** | Option<**String**> | The connection state of this communication. | [optional]
**id** | Option<**String**> | A globally unique identifier for this communication. | [optional]
**held** | Option<**bool**> | True if this call is held and the person on this side hears silence. | [optional]
**subject** | Option<**String**> | The subject for the initial email that started this conversation. | [optional]
**messages_sent** | Option<**i32**> | The number of email messages sent by this participant. | [optional]
**segments** | Option<[**Vec<crate::models::Segment>**](Segment.md)> | The time line of the participant's email, divided into activity segments. | [optional]
**direction** | Option<**String**> | The direction of the email | [optional]
**recording_id** | Option<**String**> | A globally unique identifier for the recording associated with this call. | [optional]
**error_info** | Option<[**crate::models::ErrorBody**](ErrorBody.md)> |  | [optional]
**disconnect_type** | Option<**String**> | System defined string indicating what caused the communication to disconnect. Will be null until the communication disconnects. | [optional]
**start_hold_time** | Option<**String**> | The timestamp the email was placed on hold in the cloud clock if the email is currently on hold. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**start_alerting_time** | Option<**String**> | The timestamp the communication has when it is first put into an alerting state. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**connected_time** | Option<**String**> | The timestamp when this communication was connected in the cloud clock. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**disconnected_time** | Option<**String**> | The timestamp when this communication disconnected from the conversation in the provider clock. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**auto_generated** | Option<**bool**> | Indicates that the email was auto-generated like an Out of Office reply. | [optional]
**provider** | Option<**String**> | The source provider for the email. | [optional]
**script_id** | Option<**String**> | The UUID of the script to use. | [optional]
**peer_id** | Option<**String**> | The id of the peer communication corresponding to a matching leg for this communication. | [optional]
**message_id** | Option<**String**> | A globally unique identifier for the stored content of this communication. | [optional]
**draft_attachments** | Option<[**Vec<crate::models::Attachment>**](Attachment.md)> | A list of uploaded attachments on the email draft. | [optional]
**spam** | Option<**bool**> | Indicates if the inbound email was marked as spam. | [optional]
**wrapup** | Option<[**crate::models::Wrapup**](Wrapup.md)> |  | [optional]
**after_call_work** | Option<[**crate::models::AfterCallWork**](AfterCallWork.md)> |  | [optional]
**after_call_work_required** | Option<**bool**> | Indicates if after-call work is required for a communication. Only used when the ACW Setting is Agent Requested. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


