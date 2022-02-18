# RecordingMetadata

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**conversation_id** | Option<**String**> |  | [optional]
**path** | Option<**String**> |  | [optional]
**start_time** | Option<**String**> | The start time of the recording for screen recordings. Null for other types. | [optional]
**end_time** | Option<**String**> |  | [optional]
**media** | Option<**String**> | The type of media that the recording is. At the moment that could be audio, chat, email, or message. | [optional]
**annotations** | Option<[**Vec<crate::models::Annotation>**](Annotation.md)> | Annotations that belong to the recording. Populated when recording filestate is AVAILABLE. | [optional]
**file_state** | Option<**String**> | Represents the current file state for a recording. Examples: Uploading, Archived, etc | [optional]
**restore_expiration_time** | Option<**String**> | The amount of time a restored recording will remain restored before being archived again. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**archive_date** | Option<**String**> | The date the recording will be archived. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**archive_medium** | Option<**String**> | The type of archive medium used. Example: CloudArchive | [optional]
**delete_date** | Option<**String**> | The date the recording will be deleted. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**export_date** | Option<**String**> | The date the recording will be exported. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**exported_date** | Option<**String**> | The date the recording was exported. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**max_allowed_restorations_for_org** | Option<**i32**> | How many archive restorations the organization is allowed to have. | [optional]
**remaining_restorations_allowed_for_org** | Option<**i32**> | The remaining archive restorations the organization has. | [optional]
**session_id** | Option<**String**> | The session id represents an external resource id, such as email, call, chat, etc | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


