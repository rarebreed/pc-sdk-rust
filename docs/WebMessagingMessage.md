# WebMessagingMessage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Unique ID of the message. This ID is generated by Messaging Platform. Message receipts will have the same ID as the message they reference. | [optional]
**channel** | Option<[**crate::models::WebMessagingChannel**](WebMessagingChannel.md)> |  | [optional]
**_type** | Option<**String**> | Message type. | [optional]
**text** | Option<**String**> | Message text. | [optional]
**content** | Option<[**Vec<crate::models::WebMessagingContent>**](WebMessagingContent.md)> | List of content elements. | [optional]
**events** | Option<[**Vec<crate::models::WebMessagingEvent>**](WebMessagingEvent.md)> | List of event elements. | [optional]
**direction** | Option<**String**> | The direction of the message.  Direction is always from the perspective of the Genesys Cloud platform.  An Inbound message is one sent from a guest to the Genesys Cloud Platform.  An Outbound message is one sent from the Genesys Cloud Platform to a guest. | [optional]
**originating_entity** | Option<**String**> | Specifies if this message was sent by a human agent or bot. The platform may use this to apply appropriate provider policies. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


