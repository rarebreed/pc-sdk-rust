# FaxDocument

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**date_created** | Option<**String**> | Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**date_modified** | Option<**String**> | Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**content_uri** | Option<**String**> |  | [optional]
**workspace** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**created_by** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**sharing_uri** | Option<**String**> |  | [optional]
**content_type** | Option<**String**> |  | [optional]
**content_length** | Option<**i64**> |  | [optional]
**filename** | Option<**String**> |  | [optional]
**read** | Option<**bool**> |  | [optional]
**page_count** | Option<**i64**> |  | [optional]
**caller_address** | Option<**String**> |  | [optional]
**receiver_address** | Option<**String**> |  | [optional]
**thumbnails** | Option<[**Vec<crate::models::DocumentThumbnail>**](DocumentThumbnail.md)> |  | [optional]
**download_sharing_uri** | Option<**String**> |  | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


