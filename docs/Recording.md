# Recording

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**conversation_id** | Option<**String**> |  | [optional]
**path** | Option<**String**> |  | [optional]
**start_time** | Option<**String**> | The start time of the recording. Null when there is no playable media. | [optional]
**end_time** | Option<**String**> | The end time of the recording. Null when there is no playable media. | [optional]
**media** | Option<**String**> | The type of media that the recording is. At the moment that could be audio, chat, or email. | [optional]
**annotations** | Option<[**Vec<crate::models::Annotation>**](Annotation.md)> | Annotations that belong to the recording. | [optional]
**transcript** | Option<[**Vec<crate::models::ChatMessage>**](ChatMessage.md)> | Represents a chat transcript | [optional]
**email_transcript** | Option<[**Vec<crate::models::RecordingEmailMessage>**](RecordingEmailMessage.md)> | Represents an email transcript | [optional]
**messaging_transcript** | Option<[**Vec<crate::models::RecordingMessagingMessage>**](RecordingMessagingMessage.md)> | Represents a messaging transcript | [optional]
**file_state** | Option<**String**> | Represents the current file state for a recording. Examples: Uploading, Archived, etc | [optional]
**restore_expiration_time** | Option<**String**> | The amount of time a restored recording will remain restored before being archived again. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**media_uris** | Option<[**::std::collections::HashMap<String, crate::models::MediaResult>**](MediaResult.md)> | The different mediaUris for the recording. Null when there is no playable media. | [optional]
**estimated_transcode_time_ms** | Option<**i64**> |  | [optional]
**actual_transcode_time_ms** | Option<**i64**> |  | [optional]
**archive_date** | Option<**String**> | The date the recording will be archived. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**archive_medium** | Option<**String**> | The type of archive medium used. Example: CloudArchive | [optional]
**delete_date** | Option<**String**> | The date the recording will be deleted. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**export_date** | Option<**String**> | The date the recording will be exported. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**exported_date** | Option<**String**> | The date the recording was exported. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**output_duration_ms** | Option<**i32**> | Duration of transcoded media in milliseconds | [optional]
**output_size_in_bytes** | Option<**i32**> | Size of transcoded media in bytes. 0 if there is no transcoded media. | [optional]
**max_allowed_restorations_for_org** | Option<**i32**> | How many archive restorations the organization is allowed to have. | [optional]
**remaining_restorations_allowed_for_org** | Option<**i32**> | The remaining archive restorations the organization has. | [optional]
**session_id** | Option<**String**> | The session id represents an external resource id, such as email, call, chat, etc | [optional]
**users** | Option<[**Vec<crate::models::User>**](User.md)> | The users participating in the conversation | [optional]
**recording_file_role** | Option<**String**> | Role of the file recording. It can be either customer_experience or adhoc. | [optional]
**recording_error_status** | Option<**String**> | Status of a recording that cannot be returned because of an error | [optional]
**original_recording_start_time** | Option<**String**> | The start time of the full recording, before any segment access restrictions are applied. Null when there is no playable media. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


