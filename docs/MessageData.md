# MessageData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**provider_message_id** | Option<**String**> | The unique identifier of the message from provider | [optional]
**timestamp** | **String** | The time when the message was received or sent. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | 
**from_address** | Option<**String**> | The sender of the text message. | [optional]
**to_address** | Option<**String**> | The recipient of the text message. | [optional]
**direction** | Option<**String**> | The direction of the message. | [optional]
**messenger_type** | Option<**String**> | Type of text messenger. | [optional]
**text_body** | **String** | The body of the text message. | 
**status** | **String** | The status of the message. | 
**media** | Option<[**Vec<crate::models::MessageMedia>**](MessageMedia.md)> | The media details associated to a message. | [optional]
**stickers** | Option<[**Vec<crate::models::MessageSticker>**](MessageSticker.md)> | The sticker details associated to a message. | [optional]
**normalized_message** | Option<[**crate::models::ConversationNormalizedMessage**](ConversationNormalizedMessage.md)> |  | [optional]
**normalized_receipts** | Option<[**Vec<crate::models::ConversationNormalizedMessage>**](ConversationNormalizedMessage.md)> | The delivery event associated with this message in normalized format, if the message direction was outbound | [optional][readonly]
**created_by** | Option<[**crate::models::User**](User.md)> |  | [optional]
**conversation_id** | Option<**String**> | The id of the conversation of this message. | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


