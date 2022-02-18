# OpenIntegrationRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | **String** | The name of the Open messaging integration. | 
**supported_content** | Option<[**crate::models::SupportedContentReference**](SupportedContentReference.md)> |  | [optional]
**outbound_notification_webhook_url** | **String** | The outbound notification webhook URL for the Open messaging integration. | 
**outbound_notification_webhook_signature_secret_token** | **String** | The outbound notification webhook signature secret token. | 
**webhook_headers** | Option<**::std::collections::HashMap<String, String>**> | The user specified headers for the Open messaging integration. | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


