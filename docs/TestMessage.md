# TestMessage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | After the message has been sent, this is the value of the Message-ID email header. | [optional][readonly]
**to** | [**Vec<crate::models::EmailAddress>**](EmailAddress.md) | The recipients of the email message. | 
**from** | [**crate::models::EmailAddress**](EmailAddress.md) |  | 
**subject** | Option<**String**> | The subject of the email message. | [optional]
**text_body** | **String** | The text body of the email message. | 
**html_body** | Option<**String**> | The html body of the email message | [optional]
**time** | Option<**String**> | The time when the message was sent. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


