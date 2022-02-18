# MessageDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**message_id** | Option<**String**> | UUID identifying the message media. | [optional]
**message_uri** | Option<**String**> | A URI for this message entity. | [optional]
**message_status** | Option<**String**> | Indicates the delivery status of the message. | [optional]
**message_segment_count** | Option<**i32**> | The message segment count, greater than 1 if the message content was split into multiple parts for this message type, e.g. SMS character limits. | [optional]
**message_time** | Option<**String**> | The time when the message was sent or received. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**media** | Option<[**Vec<crate::models::MessageMedia>**](MessageMedia.md)> | The media (images, files, etc) associated with this message, if any | [optional]
**stickers** | Option<[**Vec<crate::models::MessageSticker>**](MessageSticker.md)> | One or more stickers associated with this message, if any | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


