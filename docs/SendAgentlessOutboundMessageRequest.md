# SendAgentlessOutboundMessageRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**from_address** | **String** | The messaging address of the sender of the message. For an SMS messenger type, this must be a currently provisioned SMS phone number. For a WhatsApp messenger type use the provisioned WhatsApp integration’s ID | 
**to_address** | **String** | The messaging address of the recipient of the message. For an SMS messenger type, the phone number address must be in E.164 format. E.g. +13175555555 or +34234234234. | 
**to_address_messenger_type** | **String** | The recipient messaging address messenger type. Currently SMS and Open are the only supported types. WhatsApp will be supported in a future release | 
**text_body** | Option<**String**> | The text of the message to send. This field is required in the case of SMS messenger type. Maximum character counts are: SMS - 765 characters, other channels - 2000 characters. | [optional]
**messaging_template** | Option<[**crate::models::MessagingTemplateRequest**](MessagingTemplateRequest.md)> |  | [optional]
**use_existing_active_conversation** | Option<**bool**> | Use an existing active conversation to send the agentless outbound message. Set this parameter to 'true' to use active conversation. Default value: false | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


