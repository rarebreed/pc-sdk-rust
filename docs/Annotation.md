# Annotation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**_type** | Option<**String**> |  | [optional]
**location** | Option<**i64**> | Offset of annotation in milliseconds. | [optional]
**duration_ms** | Option<**i64**> | Duration of annotation in milliseconds. | [optional]
**absolute_location** | Option<**i64**> | Offset of annotation (milliseconds) from start of recording. | [optional]
**absolute_duration_ms** | Option<**i64**> | Duration of annotation (milliseconds). | [optional]
**recording_location** | Option<**i64**> | Offset of annotation (milliseconds) from start of recording, adjusted for any recording cuts | [optional]
**recording_duration_ms** | Option<**i64**> | Duration of annotation (milliseconds), adjusted for any recording cuts. | [optional]
**user** | Option<[**crate::models::User**](User.md)> |  | [optional]
**description** | Option<**String**> | Text of annotation. Maximum character limit is 500. | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


