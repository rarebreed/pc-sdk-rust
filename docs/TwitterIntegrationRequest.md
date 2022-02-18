# TwitterIntegrationRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | **String** | The name of the Twitter Integration | 
**supported_content** | Option<[**crate::models::SupportedContentReference**](SupportedContentReference.md)> |  | [optional]
**access_token_key** | **String** | The Access Token Key from Twitter messenger | 
**access_token_secret** | **String** | The Access Token Secret from Twitter messenger | 
**consumer_key** | **String** | The Consumer Key from Twitter messenger | 
**consumer_secret** | **String** | The Consumer Secret from Twitter messenger | 
**tier** | **String** | The type of twitter account to be used for the integration | 
**env_name** | Option<**String**> | The Twitter environment name, e.g.: env-beta (required for premium tier) | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


