# FacebookIntegrationRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | **String** | The name of the Facebook Integration | 
**supported_content** | Option<[**crate::models::SupportedContentReference**](SupportedContentReference.md)> |  | [optional]
**page_access_token** | Option<**String**> | The long-lived Page Access Token of Facebook page.  See https://developers.facebook.com/docs/facebook-login/access-tokens.  When a pageAccessToken is provided, pageId and userAccessToken are not required. | [optional]
**user_access_token** | Option<**String**> | The short-lived User Access Token of the Facebook user logged into the Facebook app.  See https://developers.facebook.com/docs/facebook-login/access-tokens.  When userAccessToken is provided, pageId is mandatory.  When userAccessToken/pageId combination is provided, pageAccessToken is not required. | [optional]
**page_id** | Option<**String**> | The page Id of Facebook page. The pageId is required when userAccessToken is provided. | [optional]
**app_id** | Option<**String**> | The app Id of Facebook app. The appId is required when a customer wants to use their own approved Facebook app. | [optional]
**app_secret** | Option<**String**> | The app Secret of Facebook app. The appSecret is required when appId is provided. | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


