# CallbackBasic

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**state** | Option<**String**> | The connection state of this communication. | [optional]
**id** | Option<**String**> | A globally unique identifier for this communication. | [optional]
**segments** | Option<[**Vec<crate::models::Segment>**](Segment.md)> | The time line of the participant's callback, divided into activity segments. | [optional]
**direction** | Option<**String**> | The direction of the call | [optional]
**held** | Option<**bool**> | True if this call is held and the person on this side hears silence. | [optional]
**disconnect_type** | Option<**String**> | System defined string indicating what caused the communication to disconnect. Will be null until the communication disconnects. | [optional]
**start_hold_time** | Option<**String**> | The timestamp the callback was placed on hold in the cloud clock if the callback is currently on hold. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**dialer_preview** | Option<[**crate::models::DialerPreview**](DialerPreview.md)> |  | [optional]
**voicemail** | Option<[**crate::models::Voicemail**](Voicemail.md)> |  | [optional]
**callback_numbers** | Option<**Vec<String>**> | The phone number(s) to use to place the callback. | [optional]
**callback_user_name** | Option<**String**> | The name of the user requesting a callback. | [optional]
**script_id** | Option<**String**> | The UUID of the script to use. | [optional]
**external_campaign** | Option<**bool**> | True if the call for the callback uses external dialing. | [optional]
**skip_enabled** | Option<**bool**> | True if the ability to skip a callback should be enabled. | [optional]
**timeout_seconds** | Option<**i32**> | The number of seconds before the system automatically places a call for a callback.  0 means the automatic placement is disabled. | [optional]
**start_alerting_time** | Option<**String**> | The timestamp the communication has when it is first put into an alerting state. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**connected_time** | Option<**String**> | The timestamp when this communication was connected in the cloud clock. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**disconnected_time** | Option<**String**> | The timestamp when this communication disconnected from the conversation in the provider clock. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**callback_scheduled_time** | Option<**String**> | The timestamp when this communication is scheduled in the provider clock. If this value is missing it indicates the callback will be placed immediately. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**automated_callback_config_id** | Option<**String**> | The id of the config for automatically placing the callback (and handling the disposition). If null, the callback will not be placed automatically but routed to an agent as per normal. | [optional]
**provider** | Option<**String**> | The source provider for the callback. | [optional]
**peer_id** | Option<**String**> | The id of the peer communication corresponding to a matching leg for this communication. | [optional]
**wrapup** | Option<[**crate::models::Wrapup**](Wrapup.md)> |  | [optional]
**after_call_work** | Option<[**crate::models::AfterCallWork**](AfterCallWork.md)> |  | [optional]
**after_call_work_required** | Option<**bool**> | Indicates if after-call work is required for a communication. Only used when the ACW Setting is Agent Requested. | [optional]
**caller_id** | Option<**String**> | The phone number displayed to recipients of the phone call. The value should conform to the E164 format. | [optional]
**caller_id_name** | Option<**String**> | The name displayed to recipients of the phone call. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


