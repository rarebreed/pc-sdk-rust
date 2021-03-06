# OpenNormalizedMessage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Unique ID of the message. This ID is generated by Messaging Platform. Message receipts will have the same ID as the message they reference, as such should only be set when sending a message receipt. | [optional]
**channel** | [**crate::models::OpenMessagingChannel**](OpenMessagingChannel.md) |  | 
**_type** | **String** | Message type. | 
**text** | Option<**String**> | Message text. | [optional]
**content** | Option<[**Vec<crate::models::OpenMessageContent>**](OpenMessageContent.md)> | List of content elements. | [optional]
**status** | Option<**String**> | Message receipt status, only used with type Receipt. | [optional]
**reasons** | Option<[**Vec<crate::models::Reason>**](Reason.md)> | List of reasons for a message receipt that indicates the message has failed. Only used with Failed status. | [optional]
**is_final_receipt** | Option<**bool**> | Indicates if this is the last message receipt for this message, or if another message receipt can be expected. | [optional]
**direction** | Option<**String**> | The direction of the message. | [optional]
**metadata** | Option<**::std::collections::HashMap<String, String>**> | Additional metadata about this message. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


