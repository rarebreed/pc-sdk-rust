# RecordingMessagingMessage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**from** | Option<**String**> | The message sender session id. | [optional]
**from_user** | Option<[**crate::models::User**](User.md)> |  | [optional]
**from_external_contact** | Option<[**crate::models::ExternalContact**](ExternalContact.md)> |  | [optional]
**to** | Option<**String**> | The message recipient. | [optional]
**timestamp** | Option<**String**> | The time when the message was sent. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**id** | Option<**String**> | A globally unique identifier for this communication. | [optional]
**message_text** | Option<**String**> | The content of this message. | [optional]
**message_media_attachments** | Option<[**Vec<crate::models::MessageMediaAttachment>**](MessageMediaAttachment.md)> | List of media objects attached  with this message. | [optional]
**message_sticker_attachments** | Option<[**Vec<crate::models::MessageStickerAttachment>**](MessageStickerAttachment.md)> | List of message stickers attached with this message. | [optional]
**quick_replies** | Option<[**Vec<crate::models::QuickReply>**](QuickReply.md)> | List of quick reply options offered with this message. | [optional]
**button_response** | Option<[**crate::models::ButtonResponse**](ButtonResponse.md)> |  | [optional]
**story** | Option<[**crate::models::RecordingContentStory**](RecordingContentStory.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


