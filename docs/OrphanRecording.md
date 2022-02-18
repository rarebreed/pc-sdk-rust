# OrphanRecording

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**created_time** | Option<**String**> | Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**recovered_time** | Option<**String**> | Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**provider_type** | Option<**String**> |  | [optional]
**media_size_bytes** | Option<**i64**> |  | [optional]
**media_type** | Option<**String**> |  | [optional]
**file_state** | Option<**String**> |  | [optional]
**provider_endpoint** | Option<[**crate::models::Endpoint**](Endpoint.md)> |  | [optional]
**recording** | Option<[**crate::models::Recording**](Recording.md)> |  | [optional]
**orphan_status** | Option<**String**> | The status of the orphaned recording's conversation. | [optional]
**source_orphaning_id** | Option<**String**> | An identifier used during recovery operations by the supplying hybrid platform to track back and determine which interaction this recording is associated with | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


