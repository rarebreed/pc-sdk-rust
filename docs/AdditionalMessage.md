# AdditionalMessage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**text_body** | **String** | The body of the text message.  Maximum character counts are: SMS - 765 characters, other channels - 2000 characters. | 
**media_ids** | Option<**Vec<String>**> | The media ids associated with the text message. See https://developer.genesys.cloud/api/rest/v2/conversations/messaging-media-upload for example usage. | [optional]
**sticker_ids** | Option<**Vec<String>**> | The sticker ids associated with the text message. | [optional]
**messaging_template** | Option<[**crate::models::MessagingTemplateRequest**](MessagingTemplateRequest.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


