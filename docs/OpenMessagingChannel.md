# OpenMessagingChannel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The Messaging Platform integration ID. | [optional][readonly]
**platform** | Option<**String**> | The provider type. | [optional][readonly]
**_type** | Option<**String**> | Specifies if this message is part of a private or public conversation. | [optional]
**message_id** | **String** | Unique provider ID of the message such as a Facebook message ID. | 
**to** | [**crate::models::OpenMessagingToRecipient**](OpenMessagingToRecipient.md) |  | 
**from** | [**crate::models::OpenMessagingFromRecipient**](OpenMessagingFromRecipient.md) |  | 
**time** | **String** | Original time of the event. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | 
**metadata** | Option<[**serde_json::Value**](.md)> | Information about the channel. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


