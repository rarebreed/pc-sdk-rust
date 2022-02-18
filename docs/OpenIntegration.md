# OpenIntegration

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | A unique Integration Id. | [readonly]
**name** | **String** | The name of the Open messaging integration. | 
**supported_content** | Option<[**crate::models::SupportedContentReference**](SupportedContentReference.md)> |  | [optional]
**outbound_notification_webhook_url** | **String** | The outbound notification webhook URL for the Open messaging integration. | 
**outbound_notification_webhook_signature_secret_token** | **String** | The outbound notification webhook signature secret token. | 
**webhook_headers** | Option<**::std::collections::HashMap<String, String>**> | The user specified headers for the Open messaging integration. | [optional]
**status** | Option<**String**> | The status of the Open Integration | [optional]
**recipient** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**date_created** | Option<**String**> | Date this Integration was created. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**date_modified** | Option<**String**> | Date this Integration was last modified. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**created_by** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**modified_by** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**create_status** | Option<**String**> | Status of asynchronous create operation | [optional][readonly]
**create_error** | Option<[**crate::models::ErrorBody**](ErrorBody.md)> |  | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


