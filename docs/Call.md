# Call

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**state** | Option<**String**> | The connection state of this communication. | [optional]
**id** | Option<**String**> | A globally unique identifier for this communication. | [optional]
**direction** | Option<**String**> | The direction of the call | [optional]
**recording** | Option<**bool**> | True if this call is being recorded. | [optional]
**recording_state** | Option<**String**> | State of recording on this call. | [optional]
**muted** | Option<**bool**> | True if this call is muted so that remote participants can't hear any audio from this end. | [optional]
**confined** | Option<**bool**> | True if this call is held and the person on this side hears hold music. | [optional]
**held** | Option<**bool**> | True if this call is held and the person on this side hears silence. | [optional]
**recording_id** | Option<**String**> | A globally unique identifier for the recording associated with this call. | [optional]
**segments** | Option<[**Vec<crate::models::Segment>**](Segment.md)> | The time line of the participant's call, divided into activity segments. | [optional]
**error_info** | Option<[**crate::models::ErrorInfo**](ErrorInfo.md)> |  | [optional]
**disconnect_type** | Option<**String**> | System defined string indicating what caused the communication to disconnect. Will be null until the communication disconnects. | [optional]
**start_hold_time** | Option<**String**> | The timestamp the call was placed on hold in the cloud clock if the call is currently on hold. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**document_id** | Option<**String**> | If call is an outbound fax of a document from content management, then this is the id in content management. | [optional]
**start_alerting_time** | Option<**String**> | The timestamp the communication has when it is first put into an alerting state. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**connected_time** | Option<**String**> | The timestamp when this communication was connected in the cloud clock. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**disconnected_time** | Option<**String**> | The timestamp when this communication disconnected from the conversation in the provider clock. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**disconnect_reasons** | Option<[**Vec<crate::models::DisconnectReason>**](DisconnectReason.md)> | List of reasons that this call was disconnected. This will be set once the call disconnects. | [optional]
**fax_status** | Option<[**crate::models::FaxStatus**](FaxStatus.md)> |  | [optional]
**provider** | Option<**String**> | The source provider for the call. | [optional]
**script_id** | Option<**String**> | The UUID of the script to use. | [optional]
**peer_id** | Option<**String**> | The id of the peer communication corresponding to a matching leg for this communication. | [optional]
**uui_data** | Option<**String**> | User to User Information (UUI) data managed by SIP session application. | [optional]
**_self** | Option<[**crate::models::Address**](Address.md)> |  | [optional]
**other** | Option<[**crate::models::Address**](Address.md)> |  | [optional]
**wrapup** | Option<[**crate::models::Wrapup**](Wrapup.md)> |  | [optional]
**after_call_work** | Option<[**crate::models::AfterCallWork**](AfterCallWork.md)> |  | [optional]
**after_call_work_required** | Option<**bool**> | Indicates if after-call work is required for a communication. Only used when the ACW Setting is Agent Requested. | [optional]
**agent_assistant_id** | Option<**String**> | UUID of virtual agent assistant that provide suggestions to the agent participant during the conversation. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


