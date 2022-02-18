# MessageContent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**content_type** | **String** | Type of this content element. If contentType = \"Attachment\" only one item is allowed. | 
**location** | Option<[**crate::models::ContentLocation**](ContentLocation.md)> |  | [optional]
**attachment** | Option<[**crate::models::ContentAttachment**](ContentAttachment.md)> |  | [optional]
**quick_reply** | Option<[**crate::models::ContentQuickReply**](ContentQuickReply.md)> |  | [optional]
**button_response** | Option<[**crate::models::ContentButtonResponse**](ContentButtonResponse.md)> |  | [optional]
**generic** | Option<[**crate::models::ContentGeneric**](ContentGeneric.md)> |  | [optional]
**list** | Option<[**crate::models::ContentList**](ContentList.md)> |  | [optional]
**template** | Option<[**crate::models::ContentNotificationTemplate**](ContentNotificationTemplate.md)> |  | [optional]
**reactions** | Option<[**Vec<crate::models::ContentReaction>**](ContentReaction.md)> | A set of reactions to a message. | [optional]
**mention** | Option<[**crate::models::MessagingRecipient**](MessagingRecipient.md)> |  | [optional]
**postback** | Option<[**crate::models::ContentPostback**](ContentPostback.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


