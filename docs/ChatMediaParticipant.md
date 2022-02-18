# ChatMediaParticipant

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The unique participant ID. | [optional]
**name** | Option<**String**> | The display friendly name of the participant. | [optional]
**address** | Option<**String**> | The participant address. | [optional]
**start_time** | Option<**String**> | The time when this participant first joined the conversation. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**connected_time** | Option<**String**> | The time when this participant went connected for this media (eg: video connected time). Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**end_time** | Option<**String**> | The time when this participant went disconnected for this media (eg: video disconnected time). Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**start_hold_time** | Option<**String**> | The time when this participant's hold started. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**purpose** | Option<**String**> | The participant's purpose.  Values can be: 'agent', 'user', 'customer', 'external', 'acd', 'ivr | [optional]
**state** | Option<**String**> | The participant's state.  Values can be: 'alerting', 'connected', 'disconnected', 'dialing', 'contacting | [optional]
**direction** | Option<**String**> | The participant's direction.  Values can be: 'inbound' or 'outbound' | [optional]
**disconnect_type** | Option<**String**> | The reason the participant was disconnected from the conversation. | [optional]
**held** | Option<**bool**> | Value is true when the participant is on hold. | [optional]
**wrapup_required** | Option<**bool**> | Value is true when the participant requires wrap-up. | [optional]
**wrapup_prompt** | Option<**String**> | The wrap-up prompt indicating the type of wrap-up to be performed. | [optional]
**user** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**queue** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**team** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**attributes** | Option<**::std::collections::HashMap<String, String>**> | A list of ad-hoc attributes for the participant. | [optional]
**error_info** | Option<[**crate::models::ErrorInfo**](ErrorInfo.md)> |  | [optional]
**script** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**wrapup_timeout_ms** | Option<**i32**> | The amount of time the participant has to complete wrap-up. | [optional]
**wrapup_skipped** | Option<**bool**> | Value is true when the participant has skipped wrap-up. | [optional]
**alerting_timeout_ms** | Option<**i32**> | Specifies how long the agent has to answer an interaction before being marked as not responding. | [optional]
**provider** | Option<**String**> | The source provider for the communication. | [optional]
**external_contact** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**external_organization** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**wrapup** | Option<[**crate::models::Wrapup**](Wrapup.md)> |  | [optional]
**peer** | Option<**String**> | The peer communication corresponding to a matching leg for this communication. | [optional]
**flagged_reason** | Option<**String**> | The reason specifying why participant flagged the conversation. | [optional]
**journey_context** | Option<[**crate::models::JourneyContext**](JourneyContext.md)> |  | [optional]
**conversation_routing_data** | Option<[**crate::models::ConversationRoutingData**](ConversationRoutingData.md)> |  | [optional]
**start_acw_time** | Option<**String**> | The timestamp when this participant started after-call work. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**end_acw_time** | Option<**String**> | The timestamp when this participant ended after-call work. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**room_id** | Option<**String**> | The ID of the chat room. | [optional]
**avatar_image_url** | Option<**String**> | If available, the URI to the avatar image of this communication. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


