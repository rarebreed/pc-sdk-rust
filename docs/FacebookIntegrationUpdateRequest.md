# FacebookIntegrationUpdateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> | The name of the Facebook Integration | [optional]
**supported_content** | Option<[**crate::models::SupportedContentReference**](SupportedContentReference.md)> |  | [optional]
**page_access_token** | Option<**String**> | The long-lived Page Access Token of Facebook page.  See https://developers.facebook.com/docs/facebook-login/access-tokens.  Either pageAccessToken or userAccessToken should be provided. | [optional]
**user_access_token** | Option<**String**> | The short-lived User Access Token of the Facebook user logged into the Facebook app.  See https://developers.facebook.com/docs/facebook-login/access-tokens.  Either pageAccessToken or userAccessToken should be provided. | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


