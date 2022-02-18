# VoicemailMessage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**conversation** | Option<[**crate::models::Conversation**](Conversation.md)> |  | [optional]
**read** | Option<**bool**> | Whether the voicemail message is marked as read | [optional]
**audio_recording_duration_seconds** | Option<**i32**> | The voicemail message's audio recording duration in seconds | [optional][readonly]
**audio_recording_size_bytes** | Option<**i64**> | The voicemail message's audio recording size in bytes | [optional][readonly]
**created_date** | Option<**String**> | The date the voicemail message was created. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**modified_date** | Option<**String**> | The date the voicemail message was last modified. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**deleted_date** | Option<**String**> | The date the voicemail message deleted property was set to true. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**caller_address** | Option<**String**> | The caller address | [optional][readonly]
**caller_name** | Option<**String**> | Optionally the name of the caller that left the voicemail message if the caller was a known user | [optional][readonly]
**caller_user** | Option<[**crate::models::User**](User.md)> |  | [optional]
**deleted** | Option<**bool**> | Whether the voicemail message has been marked as deleted | [optional]
**note** | Option<**String**> | An optional note | [optional]
**user** | Option<[**crate::models::User**](User.md)> |  | [optional]
**group** | Option<[**crate::models::Group**](Group.md)> |  | [optional]
**queue** | Option<[**crate::models::Queue**](Queue.md)> |  | [optional]
**copied_from** | Option<[**crate::models::VoicemailCopyRecord**](VoicemailCopyRecord.md)> |  | [optional]
**copied_to** | Option<[**Vec<crate::models::VoicemailCopyRecord>**](VoicemailCopyRecord.md)> | Represents where this voicemail has been copied to | [optional][readonly]
**delete_retention_policy** | Option<[**crate::models::VoicemailRetentionPolicy**](VoicemailRetentionPolicy.md)> |  | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


