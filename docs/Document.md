# Document

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**change_number** | Option<**i32**> |  | [optional]
**date_created** | Option<**String**> | Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**date_modified** | Option<**String**> | Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**date_uploaded** | Option<**String**> | Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**content_uri** | Option<**String**> |  | [optional]
**workspace** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**created_by** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**uploaded_by** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**sharing_uri** | Option<**String**> |  | [optional]
**content_type** | Option<**String**> |  | [optional]
**content_length** | Option<**i64**> |  | [optional]
**system_type** | Option<**String**> |  | [optional]
**filename** | Option<**String**> |  | [optional]
**page_count** | Option<**i64**> |  | [optional]
**read** | Option<**bool**> |  | [optional]
**caller_address** | Option<**String**> |  | [optional]
**receiver_address** | Option<**String**> |  | [optional]
**tags** | Option<**Vec<String>**> |  | [optional]
**tag_values** | Option<[**Vec<crate::models::TagValue>**](TagValue.md)> |  | [optional]
**attributes** | Option<[**Vec<crate::models::DocumentAttribute>**](DocumentAttribute.md)> |  | [optional]
**thumbnails** | Option<[**Vec<crate::models::DocumentThumbnail>**](DocumentThumbnail.md)> |  | [optional]
**upload_status** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**upload_destination_uri** | Option<**String**> |  | [optional]
**upload_method** | Option<**String**> |  | [optional]
**lock_info** | Option<[**crate::models::LockInfo**](LockInfo.md)> |  | [optional]
**acl** | Option<**Vec<String>**> | A list of permitted action rights for the user making the request | [optional]
**sharing_status** | Option<**String**> |  | [optional]
**download_sharing_uri** | Option<**String**> |  | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


