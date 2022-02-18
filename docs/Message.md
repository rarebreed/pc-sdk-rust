# Message

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**state** | Option<**String**> | The connection state of this communication. | [optional]
**id** | Option<**String**> | A globally unique identifier for this communication. | [optional]
**held** | Option<**bool**> | True if this call is held and the person on this side hears silence. | [optional]
**segments** | Option<[**Vec<crate::models::Segment>**](Segment.md)> | The time line of the participant's message, divided into activity segments. | [optional]
**direction** | Option<**String**> | The direction of the message. | [optional]
**recording_id** | Option<**String**> | A globally unique identifier for the recording associated with this message. | [optional]
**error_info** | Option<[**crate::models::ErrorBody**](ErrorBody.md)> |  | [optional]
**disconnect_type** | Option<**String**> | System defined string indicating what caused the communication to disconnect. Will be null until the communication disconnects. | [optional]
**start_hold_time** | Option<**String**> | The timestamp the message was placed on hold in the cloud clock if the message is currently on hold. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**start_alerting_time** | Option<**String**> | The timestamp the communication has when it is first put into an alerting state. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**connected_time** | Option<**String**> | The timestamp when this communication was connected in the cloud clock. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**disconnected_time** | Option<**String**> | The timestamp when this communication disconnected from the conversation in the provider clock. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**provider** | Option<**String**> | The source provider for the message. | [optional]
**authenticated** | Option<**bool**> | If true, the participant member is authenticated. | [optional]
**_type** | Option<**String**> | Indicates the type of message platform from which the message originated. | [optional]
**recipient_country** | Option<**String**> | Indicates the country where the recipient is associated in ISO 3166-1 alpha-2 format. | [optional]
**recipient_type** | Option<**String**> | The type of the recipient. Eg: Provisioned phoneNumber is the recipient for sms message type. | [optional]
**script_id** | Option<**String**> | The UUID of the script to use. | [optional]
**peer_id** | Option<**String**> | The id of the peer communication corresponding to a matching leg for this communication. | [optional]
**to_address** | Option<[**crate::models::Address**](Address.md)> |  | [optional]
**from_address** | Option<[**crate::models::Address**](Address.md)> |  | [optional]
**messages** | Option<[**Vec<crate::models::MessageDetails>**](MessageDetails.md)> | The messages sent on this communication channel. | [optional]
**journey_context** | Option<[**crate::models::JourneyContext**](JourneyContext.md)> |  | [optional]
**wrapup** | Option<[**crate::models::Wrapup**](Wrapup.md)> |  | [optional]
**after_call_work** | Option<[**crate::models::AfterCallWork**](AfterCallWork.md)> |  | [optional]
**after_call_work_required** | Option<**bool**> | Indicates if after-call work is required for a communication. Only used when the ACW Setting is Agent Requested. | [optional]
**agent_assistant_id** | Option<**String**> | UUID of virtual agent assistant that provide suggestions to the agent participant during the conversation. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


