# TwitterIntegration

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | A unique Integration Id | [readonly]
**name** | **String** | The name of the Twitter Integration | 
**supported_content** | Option<[**crate::models::SupportedContentReference**](SupportedContentReference.md)> |  | [optional]
**access_token_key** | **String** | The Access Token Key from Twitter messenger | 
**consumer_key** | **String** | The Consumer Key from Twitter messenger | 
**username** | Option<**String**> | The Username from Twitter | [optional]
**user_id** | Option<**String**> | The UserId from Twitter | [optional]
**status** | Option<**String**> | The status of the Twitter Integration | [optional]
**tier** | **String** | The type of twitter account to be used for the integration | 
**env_name** | Option<**String**> | The Twitter environment name, e.g.: env-beta (required for premium tier) | [optional]
**recipient** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**date_created** | Option<**String**> | Date this Integration was created. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**date_modified** | Option<**String**> | Date this Integration was modified. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**created_by** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**modified_by** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**version** | **i32** | Version number required for updates. | 
**create_status** | Option<**String**> | Status of asynchronous create operation | [optional][readonly]
**create_error** | Option<[**crate::models::ErrorBody**](ErrorBody.md)> |  | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


