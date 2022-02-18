# WhatsAppIntegration

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | A unique Integration Id. | [readonly]
**name** | **String** | The name of the WhatsApp integration. | 
**supported_content** | Option<[**crate::models::SupportedContentReference**](SupportedContentReference.md)> |  | [optional]
**phone_number** | **String** | The phone number associated to the whatsApp integration. | 
**status** | Option<**String**> | The status of the WhatsApp Integration | [optional]
**recipient** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**date_created** | Option<**String**> | Date this Integration was created. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**date_modified** | Option<**String**> | Date this Integration was last modified. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**created_by** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**modified_by** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**version** | **i32** | Version number required for updates. | 
**activation_status_code** | Option<**String**> | The status code of WhatsApp Integration activation process | [optional][readonly]
**activation_error_info** | Option<[**crate::models::ErrorBody**](ErrorBody.md)> |  | [optional]
**create_status** | Option<**String**> | Status of asynchronous create operation | [optional][readonly]
**create_error** | Option<[**crate::models::ErrorBody**](ErrorBody.md)> |  | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


