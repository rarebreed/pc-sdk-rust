# ConversationNormalizedMessage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Unique ID of the message. Message receipts will have the same ID as the message they reference. | [optional][readonly]
**channel** | Option<[**crate::models::ConversationMessagingChannel**](ConversationMessagingChannel.md)> |  | [optional]
**_type** | **String** | Message type. | 
**text** | Option<**String**> | Message text. | [optional]
**content** | Option<[**Vec<crate::models::ConversationMessageContent>**](ConversationMessageContent.md)> | List of content elements | [optional]
**events** | Option<[**Vec<crate::models::ConversationMessageEvent>**](ConversationMessageEvent.md)> | List of event elements. | [optional]
**status** | Option<**String**> | Message receipt status, only used with type Receipt. | [optional][readonly]
**reasons** | Option<[**Vec<crate::models::ConversationReason>**](ConversationReason.md)> | List of reasons for a message receipt that indicates the message has failed. Only used with Failed status. | [optional][readonly]
**originating_entity** | Option<**String**> | Specifies if this message was sent by a human agent or bot. The platform may use this to apply appropriate provider policies. | [optional]
**is_final_receipt** | Option<**bool**> | Indicates if this is the last message receipt for this message, or if another message receipt can be expected. | [optional][readonly]
**direction** | Option<**String**> | The direction of the message. | [optional][readonly]
**metadata** | Option<**::std::collections::HashMap<String, String>**> | Additional metadata about this message. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


