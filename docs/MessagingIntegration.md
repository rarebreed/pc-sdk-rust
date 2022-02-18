# MessagingIntegration

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | A unique Integration Id | [readonly]
**name** | **String** | The name of the Integration | [readonly]
**supported_content** | Option<[**crate::models::SupportedContentReference**](SupportedContentReference.md)> |  | [optional]
**status** | Option<**String**> | The status of the Integration | [optional][readonly]
**messenger_type** | **String** | The type of Messaging Integration | [readonly]
**recipient** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**date_created** | Option<**String**> | Date this Integration was created. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**date_modified** | Option<**String**> | Date this Integration was modified. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**created_by** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**modified_by** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**version** | **i32** | Version number required for updates. | [readonly]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


