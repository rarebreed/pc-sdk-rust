# WebChatDeployment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**authentication_required** | Option<**bool**> |  | [optional]
**authentication_url** | Option<**String**> | URL for third party service authenticating web chat clients. See https://github.com/MyPureCloud/authenticated-web-chat-server-examples | [optional]
**disabled** | Option<**bool**> |  | [optional]
**web_chat_config** | Option<[**crate::models::WebChatConfig**](WebChatConfig.md)> |  | [optional]
**allowed_domains** | Option<**Vec<String>**> |  | [optional]
**flow** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


