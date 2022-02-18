# EmailMessage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**to** | [**Vec<crate::models::EmailAddress>**](EmailAddress.md) | The recipients of the email message. | 
**cc** | Option<[**Vec<crate::models::EmailAddress>**](EmailAddress.md)> | The recipients that were copied on the email message. | [optional]
**bcc** | Option<[**Vec<crate::models::EmailAddress>**](EmailAddress.md)> | The recipients that were blind copied on the email message. | [optional]
**from** | [**crate::models::EmailAddress**](EmailAddress.md) |  | 
**reply_to** | Option<[**crate::models::EmailAddress**](EmailAddress.md)> |  | [optional]
**subject** | Option<**String**> | The subject of the email message. | [optional]
**attachments** | Option<[**Vec<crate::models::Attachment>**](Attachment.md)> | The attachments of the email message. | [optional]
**text_body** | **String** | The text body of the email message. | 
**html_body** | Option<**String**> | The html body of the email message. | [optional]
**time** | Option<**String**> | The time when the message was received or sent. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**history_included** | Option<**bool**> | Indicates whether the history of previous emails of the conversation is included within the email bodies of this message. | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


