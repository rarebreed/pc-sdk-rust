# FacebookIntegration

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | A unique Integration Id. | [readonly]
**name** | **String** | The name of the Facebook Integration | 
**supported_content** | Option<[**crate::models::SupportedContentReference**](SupportedContentReference.md)> |  | [optional]
**app_id** | **String** | The App Id from Facebook messenger | 
**page_id** | Option<**String**> | The Page Id from Facebook messenger | [optional]
**page_name** | Option<**String**> | The name of the Facebook page | [optional][readonly]
**page_profile_image_url** | Option<**String**> | The url of the profile image of the Facebook page | [optional][readonly]
**status** | Option<**String**> | The status of the Facebook Integration | [optional]
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


