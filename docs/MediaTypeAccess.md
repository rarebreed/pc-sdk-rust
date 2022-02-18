# MediaTypeAccess

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**inbound** | Option<[**Vec<crate::models::MediaType>**](MediaType.md)> | List of media types allowed for inbound messages from customers. If inbound messages from a customer contain media that is not in this list, the media will be dropped from the outbound message. | [optional]
**outbound** | Option<[**Vec<crate::models::MediaType>**](MediaType.md)> | List of media types allowed for outbound messages to customers. If an outbound message is sent that contains media that is not in this list, the message will not be sent. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


