# SendAgentlessOutboundMessageResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**conversation_id** | Option<**String**> | The identifier of the conversation. | [optional]
**from_address** | Option<**String**> | The sender of the message. | [optional]
**to_address** | Option<**String**> | The recipient of the message. | [optional]
**messenger_type** | Option<**String**> | Type of messenger. | [optional]
**text_body** | Option<**String**> | The body of the text message. | [optional]
**messaging_template** | Option<[**crate::models::MessagingTemplateRequest**](MessagingTemplateRequest.md)> |  | [optional]
**use_existing_active_conversation** | Option<**bool**> | Use an existing active conversation to send the agentless outbound message. Set this parameter to 'true' to use active conversation. Default value: false | [optional]
**timestamp** | Option<**String**> | The time when the message was sent. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]
**user** | Option<[**crate::models::AddressableEntityRef**](AddressableEntityRef.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


