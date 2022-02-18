# ConversationMessagingChannel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The integration ID. | [optional][readonly]
**platform** | Option<**String**> | The provider type. | [optional][readonly]
**message_id** | Option<**String**> | Unique provider ID of the message such as a Facebook message ID. | [optional][readonly]
**to** | Option<[**crate::models::ConversationMessagingToRecipient**](ConversationMessagingToRecipient.md)> |  | [optional]
**from** | Option<[**crate::models::ConversationMessagingFromRecipient**](ConversationMessagingFromRecipient.md)> |  | [optional]
**time** | Option<**String**> | Original time of the event. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**date_modified** | Option<**String**> | Time the message was edited. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**date_deleted** | Option<**String**> | Time the message was deleted. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


