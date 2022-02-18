# WebChatMessage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**conversation** | [**crate::models::WebChatConversation**](WebChatConversation.md) |  | 
**sender** | [**crate::models::WebChatMemberInfo**](WebChatMemberInfo.md) |  | 
**body** | **String** | The message body. | 
**body_type** | **String** | The purpose of the message within the conversation, such as a standard text entry versus a greeting. | 
**timestamp** | **String** | The timestamp of the message, in ISO-8601 format | 
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


