# SocialExpression

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**state** | Option<**String**> | The connection state of this communication. | [optional]
**id** | Option<**String**> | A globally unique identifier for this communication. | [optional]
**social_media_id** | Option<**String**> | A globally unique identifier for the social media. | [optional]
**social_media_hub** | Option<**String**> | The social network of the communication | [optional]
**social_user_name** | Option<**String**> | The user name for the communication. | [optional]
**preview_text** | Option<**String**> | The text preview of the communication contents | [optional]
**recording_id** | Option<**String**> | A globally unique identifier for the recording associated with this chat. | [optional]
**segments** | Option<[**Vec<crate::models::Segment>**](Segment.md)> | The time line of the participant's chat, divided into activity segments. | [optional]
**held** | Option<**bool**> | True if this call is held and the person on this side hears silence. | [optional]
**disconnect_type** | Option<**String**> | System defined string indicating what caused the communication to disconnect. Will be null until the communication disconnects. | [optional]
**start_hold_time** | Option<**String**> | The timestamp the chat was placed on hold in the cloud clock if the chat is currently on hold. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**start_alerting_time** | Option<**String**> | The timestamp the communication has when it is first put into an alerting state. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**connected_time** | Option<**String**> | The timestamp when this communication was connected in the cloud clock. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**disconnected_time** | Option<**String**> | The timestamp when this communication disconnected from the conversation in the provider clock. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**provider** | Option<**String**> | The source provider for the social expression. | [optional]
**script_id** | Option<**String**> | The UUID of the script to use. | [optional]
**peer_id** | Option<**String**> | The id of the peer communication corresponding to a matching leg for this communication. | [optional]
**wrapup** | Option<[**crate::models::Wrapup**](Wrapup.md)> |  | [optional]
**after_call_work** | Option<[**crate::models::AfterCallWork**](AfterCallWork.md)> |  | [optional]
**after_call_work_required** | Option<**bool**> | Indicates if after-call work is required for a communication. Only used when the ACW Setting is Agent Requested. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


