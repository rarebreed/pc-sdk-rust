# \RecordingApi

All URIs are relative to *https://api.mypurecloud.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_conversation_recording_annotation**](RecordingApi.md#delete_conversation_recording_annotation) | **DELETE** /api/v2/conversations/{conversationId}/recordings/{recordingId}/annotations/{annotationId} | Delete annotation
[**delete_orphanrecording**](RecordingApi.md#delete_orphanrecording) | **DELETE** /api/v2/orphanrecordings/{orphanId} | Deletes a single orphan recording
[**delete_recording_crossplatform_mediaretentionpolicies**](RecordingApi.md#delete_recording_crossplatform_mediaretentionpolicies) | **DELETE** /api/v2/recording/crossplatform/mediaretentionpolicies | Delete media retention policies
[**delete_recording_crossplatform_mediaretentionpolicy**](RecordingApi.md#delete_recording_crossplatform_mediaretentionpolicy) | **DELETE** /api/v2/recording/crossplatform/mediaretentionpolicies/{policyId} | Delete a media retention policy
[**delete_recording_job**](RecordingApi.md#delete_recording_job) | **DELETE** /api/v2/recording/jobs/{jobId} | Delete the recording bulk job
[**delete_recording_mediaretentionpolicies**](RecordingApi.md#delete_recording_mediaretentionpolicies) | **DELETE** /api/v2/recording/mediaretentionpolicies | Delete media retention policies
[**delete_recording_mediaretentionpolicy**](RecordingApi.md#delete_recording_mediaretentionpolicy) | **DELETE** /api/v2/recording/mediaretentionpolicies/{policyId} | Delete a media retention policy
[**get_conversation_recording**](RecordingApi.md#get_conversation_recording) | **GET** /api/v2/conversations/{conversationId}/recordings/{recordingId} | Gets a specific recording.
[**get_conversation_recording_annotation**](RecordingApi.md#get_conversation_recording_annotation) | **GET** /api/v2/conversations/{conversationId}/recordings/{recordingId}/annotations/{annotationId} | Get annotation
[**get_conversation_recording_annotations**](RecordingApi.md#get_conversation_recording_annotations) | **GET** /api/v2/conversations/{conversationId}/recordings/{recordingId}/annotations | Get annotations for recording
[**get_conversation_recordingmetadata**](RecordingApi.md#get_conversation_recordingmetadata) | **GET** /api/v2/conversations/{conversationId}/recordingmetadata | Get recording metadata for a conversation. Does not return playable media. Annotations won't be included in the response if recording:recording:view permission is missing.
[**get_conversation_recordingmetadata_recording_id**](RecordingApi.md#get_conversation_recordingmetadata_recording_id) | **GET** /api/v2/conversations/{conversationId}/recordingmetadata/{recordingId} | Get metadata for a specific recording. Does not return playable media.
[**get_conversation_recordings**](RecordingApi.md#get_conversation_recordings) | **GET** /api/v2/conversations/{conversationId}/recordings | Get all of a Conversation's Recordings.
[**get_orphanrecording**](RecordingApi.md#get_orphanrecording) | **GET** /api/v2/orphanrecordings/{orphanId} | Gets a single orphan recording
[**get_orphanrecording_media**](RecordingApi.md#get_orphanrecording_media) | **GET** /api/v2/orphanrecordings/{orphanId}/media | Gets the media of a single orphan recording
[**get_orphanrecordings**](RecordingApi.md#get_orphanrecordings) | **GET** /api/v2/orphanrecordings | Gets all orphan recordings
[**get_recording_batchrequest**](RecordingApi.md#get_recording_batchrequest) | **GET** /api/v2/recording/batchrequests/{jobId} | Get the status and results for a batch request job, only the user that submitted the job may retrieve results
[**get_recording_crossplatform_mediaretentionpolicies**](RecordingApi.md#get_recording_crossplatform_mediaretentionpolicies) | **GET** /api/v2/recording/crossplatform/mediaretentionpolicies | Gets media retention policy list with query options to filter on name and enabled.
[**get_recording_crossplatform_mediaretentionpolicy**](RecordingApi.md#get_recording_crossplatform_mediaretentionpolicy) | **GET** /api/v2/recording/crossplatform/mediaretentionpolicies/{policyId} | Get a media retention policy
[**get_recording_job**](RecordingApi.md#get_recording_job) | **GET** /api/v2/recording/jobs/{jobId} | Get the status of the job associated with the job id.
[**get_recording_job_failedrecordings**](RecordingApi.md#get_recording_job_failedrecordings) | **GET** /api/v2/recording/jobs/{jobId}/failedrecordings | Get IDs of recordings that the bulk job failed for
[**get_recording_jobs**](RecordingApi.md#get_recording_jobs) | **GET** /api/v2/recording/jobs | Get the status of all jobs within the user's organization
[**get_recording_localkeys_setting**](RecordingApi.md#get_recording_localkeys_setting) | **GET** /api/v2/recording/localkeys/settings/{settingsId} | Get the local encryption settings
[**get_recording_localkeys_settings**](RecordingApi.md#get_recording_localkeys_settings) | **GET** /api/v2/recording/localkeys/settings | gets a list local key settings data
[**get_recording_mediaretentionpolicies**](RecordingApi.md#get_recording_mediaretentionpolicies) | **GET** /api/v2/recording/mediaretentionpolicies | Gets media retention policy list with query options to filter on name and enabled.
[**get_recording_mediaretentionpolicy**](RecordingApi.md#get_recording_mediaretentionpolicy) | **GET** /api/v2/recording/mediaretentionpolicies/{policyId} | Get a media retention policy
[**get_recording_recordingkeys**](RecordingApi.md#get_recording_recordingkeys) | **GET** /api/v2/recording/recordingkeys | Get encryption key list
[**get_recording_recordingkeys_rotationschedule**](RecordingApi.md#get_recording_recordingkeys_rotationschedule) | **GET** /api/v2/recording/recordingkeys/rotationschedule | Get key rotation schedule
[**get_recording_settings**](RecordingApi.md#get_recording_settings) | **GET** /api/v2/recording/settings | Get the Recording Settings for the Organization
[**get_recordings_screensessions**](RecordingApi.md#get_recordings_screensessions) | **GET** /api/v2/recordings/screensessions | Retrieves a paged listing of screen recording sessions
[**patch_recording_crossplatform_mediaretentionpolicy**](RecordingApi.md#patch_recording_crossplatform_mediaretentionpolicy) | **PATCH** /api/v2/recording/crossplatform/mediaretentionpolicies/{policyId} | Patch a media retention policy
[**patch_recording_mediaretentionpolicy**](RecordingApi.md#patch_recording_mediaretentionpolicy) | **PATCH** /api/v2/recording/mediaretentionpolicies/{policyId} | Patch a media retention policy
[**patch_recordings_screensession**](RecordingApi.md#patch_recordings_screensession) | **PATCH** /api/v2/recordings/screensessions/{recordingSessionId} | Update a screen recording session
[**post_conversation_recording_annotations**](RecordingApi.md#post_conversation_recording_annotations) | **POST** /api/v2/conversations/{conversationId}/recordings/{recordingId}/annotations | Create annotation
[**post_recording_batchrequests**](RecordingApi.md#post_recording_batchrequests) | **POST** /api/v2/recording/batchrequests | Submit a batch download request for recordings. Recordings in response will be in their original format/codec - configured in the Trunk configuration.
[**post_recording_crossplatform_mediaretentionpolicies**](RecordingApi.md#post_recording_crossplatform_mediaretentionpolicies) | **POST** /api/v2/recording/crossplatform/mediaretentionpolicies | Create media retention policy
[**post_recording_jobs**](RecordingApi.md#post_recording_jobs) | **POST** /api/v2/recording/jobs | Create a recording bulk job
[**post_recording_localkeys**](RecordingApi.md#post_recording_localkeys) | **POST** /api/v2/recording/localkeys | create a local recording key
[**post_recording_localkeys_settings**](RecordingApi.md#post_recording_localkeys_settings) | **POST** /api/v2/recording/localkeys/settings | create settings for local key creation
[**post_recording_mediaretentionpolicies**](RecordingApi.md#post_recording_mediaretentionpolicies) | **POST** /api/v2/recording/mediaretentionpolicies | Create media retention policy
[**post_recording_recordingkeys**](RecordingApi.md#post_recording_recordingkeys) | **POST** /api/v2/recording/recordingkeys | Create encryption key
[**post_recordings_deletionprotection**](RecordingApi.md#post_recordings_deletionprotection) | **POST** /api/v2/recordings/deletionprotection | Get a list of conversations with protected recordings
[**post_recordings_screensessions_acknowledge**](RecordingApi.md#post_recordings_screensessions_acknowledge) | **POST** /api/v2/recordings/screensessions/acknowledge | Acknowledge a screen recording.
[**post_recordings_screensessions_metadata**](RecordingApi.md#post_recordings_screensessions_metadata) | **POST** /api/v2/recordings/screensessions/metadata | Provide meta-data a screen recording.
[**put_conversation_recording**](RecordingApi.md#put_conversation_recording) | **PUT** /api/v2/conversations/{conversationId}/recordings/{recordingId} | Updates the retention records on a recording.
[**put_conversation_recording_annotation**](RecordingApi.md#put_conversation_recording_annotation) | **PUT** /api/v2/conversations/{conversationId}/recordings/{recordingId}/annotations/{annotationId} | Update annotation
[**put_orphanrecording**](RecordingApi.md#put_orphanrecording) | **PUT** /api/v2/orphanrecordings/{orphanId} | Updates an orphan recording to a regular recording with retention values
[**put_recording_crossplatform_mediaretentionpolicy**](RecordingApi.md#put_recording_crossplatform_mediaretentionpolicy) | **PUT** /api/v2/recording/crossplatform/mediaretentionpolicies/{policyId} | Update a media retention policy
[**put_recording_job**](RecordingApi.md#put_recording_job) | **PUT** /api/v2/recording/jobs/{jobId} | Execute the recording bulk job.
[**put_recording_localkeys_setting**](RecordingApi.md#put_recording_localkeys_setting) | **PUT** /api/v2/recording/localkeys/settings/{settingsId} | Update the local encryption settings
[**put_recording_mediaretentionpolicy**](RecordingApi.md#put_recording_mediaretentionpolicy) | **PUT** /api/v2/recording/mediaretentionpolicies/{policyId} | Update a media retention policy
[**put_recording_recordingkeys_rotationschedule**](RecordingApi.md#put_recording_recordingkeys_rotationschedule) | **PUT** /api/v2/recording/recordingkeys/rotationschedule | Update key rotation schedule
[**put_recording_settings**](RecordingApi.md#put_recording_settings) | **PUT** /api/v2/recording/settings | Update the Recording Settings for the Organization
[**put_recordings_deletionprotection**](RecordingApi.md#put_recordings_deletionprotection) | **PUT** /api/v2/recordings/deletionprotection | Apply or revoke recording protection for conversations



## delete_conversation_recording_annotation

> delete_conversation_recording_annotation(conversation_id, recording_id, annotation_id)
Delete annotation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | Conversation ID | [required] |
**recording_id** | **String** | Recording ID | [required] |
**annotation_id** | **String** | Annotation ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_orphanrecording

> crate::models::OrphanRecording delete_orphanrecording(orphan_id)
Deletes a single orphan recording

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**orphan_id** | **String** | Orphan ID | [required] |

### Return type

[**crate::models::OrphanRecording**](OrphanRecording.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_recording_crossplatform_mediaretentionpolicies

> delete_recording_crossplatform_mediaretentionpolicies(ids)
Delete media retention policies

Bulk delete of media retention policies, this will only delete the polices that match the ids specified in the query param.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_recording_crossplatform_mediaretentionpolicy

> delete_recording_crossplatform_mediaretentionpolicy(policy_id)
Delete a media retention policy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **String** | Policy ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_recording_job

> delete_recording_job(job_id)
Delete the recording bulk job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | jobId | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_recording_mediaretentionpolicies

> delete_recording_mediaretentionpolicies(ids)
Delete media retention policies

Bulk delete of media retention policies, this will only delete the polices that match the ids specified in the query param.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_recording_mediaretentionpolicy

> delete_recording_mediaretentionpolicy(policy_id)
Delete a media retention policy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **String** | Policy ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conversation_recording

> crate::models::Recording get_conversation_recording(conversation_id, recording_id, format_id, email_format_id, chat_format_id, message_format_id, download, file_name, locale)
Gets a specific recording.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | Conversation ID | [required] |
**recording_id** | **String** | Recording ID | [required] |
**format_id** | Option<**String**> | The desired media format. Valid values:WAV,WEBM,WAV_ULAW,OGG_VORBIS,OGG_OPUS,MP3,NONE |  |[default to WEBM]
**email_format_id** | Option<**String**> | The desired media format when downloading an email recording. Valid values:EML,NONE |  |[default to EML]
**chat_format_id** | Option<**String**> | The desired media format when downloading a chat recording. Valid values:ZIP,NONE  |  |[default to ZIP]
**message_format_id** | Option<**String**> | The desired media format when downloading a message recording. Valid values:ZIP,NONE |  |[default to ZIP]
**download** | Option<**bool**> | requesting a download format of the recording. Valid values:true,false |  |[default to false]
**file_name** | Option<**String**> | the name of the downloaded fileName |  |
**locale** | Option<**String**> | The locale for the requested file when downloading, as an ISO 639-1 code |  |

### Return type

[**crate::models::Recording**](Recording.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conversation_recording_annotation

> crate::models::Annotation get_conversation_recording_annotation(conversation_id, recording_id, annotation_id)
Get annotation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | Conversation ID | [required] |
**recording_id** | **String** | Recording ID | [required] |
**annotation_id** | **String** | Annotation ID | [required] |

### Return type

[**crate::models::Annotation**](Annotation.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conversation_recording_annotations

> Vec<crate::models::Annotation> get_conversation_recording_annotations(conversation_id, recording_id)
Get annotations for recording

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | Conversation ID | [required] |
**recording_id** | **String** | Recording ID | [required] |

### Return type

[**Vec<crate::models::Annotation>**](Annotation.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conversation_recordingmetadata

> Vec<crate::models::RecordingMetadata> get_conversation_recordingmetadata(conversation_id)
Get recording metadata for a conversation. Does not return playable media. Annotations won't be included in the response if recording:recording:view permission is missing.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | Conversation ID | [required] |

### Return type

[**Vec<crate::models::RecordingMetadata>**](RecordingMetadata.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conversation_recordingmetadata_recording_id

> crate::models::RecordingMetadata get_conversation_recordingmetadata_recording_id(conversation_id, recording_id)
Get metadata for a specific recording. Does not return playable media.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | Conversation ID | [required] |
**recording_id** | **String** | Recording ID | [required] |

### Return type

[**crate::models::RecordingMetadata**](RecordingMetadata.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conversation_recordings

> Vec<crate::models::Recording> get_conversation_recordings(conversation_id, max_wait_ms, format_id)
Get all of a Conversation's Recordings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | Conversation ID | [required] |
**max_wait_ms** | Option<**i32**> | The maximum number of milliseconds to wait for the recording to be ready. Must be a positive value. |  |[default to 5000]
**format_id** | Option<**String**> | The desired media format . Valid values:WAV,WEBM,WAV_ULAW,OGG_VORBIS,OGG_OPUS,MP3,NONE. |  |[default to WEBM]

### Return type

[**Vec<crate::models::Recording>**](Recording.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_orphanrecording

> crate::models::OrphanRecording get_orphanrecording(orphan_id)
Gets a single orphan recording

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**orphan_id** | **String** | Orphan ID | [required] |

### Return type

[**crate::models::OrphanRecording**](OrphanRecording.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_orphanrecording_media

> crate::models::Recording get_orphanrecording_media(orphan_id, format_id, email_format_id, chat_format_id, message_format_id, download, file_name, locale)
Gets the media of a single orphan recording

A 202 response means the orphaned media is currently transcoding and will be available shortly.A 200 response denotes the transcoded orphan media is available now and is contained in the response body.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**orphan_id** | **String** | Orphan ID | [required] |
**format_id** | Option<**String**> | The desired media format. |  |[default to WEBM]
**email_format_id** | Option<**String**> | The desired media format when downloading an email recording. |  |[default to EML]
**chat_format_id** | Option<**String**> | The desired media format when downloading a chat recording. |  |[default to ZIP]
**message_format_id** | Option<**String**> | The desired media format when downloading a message recording. |  |[default to ZIP]
**download** | Option<**bool**> | requesting a download format of the recording |  |[default to false]
**file_name** | Option<**String**> | the name of the downloaded fileName |  |
**locale** | Option<**String**> | The locale for the requested file when downloading, as an ISO 639-1 code |  |

### Return type

[**crate::models::Recording**](Recording.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_orphanrecordings

> crate::models::OrphanRecordingListing get_orphanrecordings(page_size, page_number, sort_by, expand, next_page, previous_page, has_conversation, media)
Gets all orphan recordings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | The total page size requested |  |[default to 25]
**page_number** | Option<**i32**> | The page number requested |  |[default to 1]
**sort_by** | Option<**String**> | variable name requested to sort by |  |
**expand** | Option<[**Vec<String>**](String.md)> | variable name requested by expand list |  |
**next_page** | Option<**String**> | next page token |  |
**previous_page** | Option<**String**> | Previous page token |  |
**has_conversation** | Option<**bool**> | Filter resulting orphans by whether the conversation is known. False returns all orphans for the organization. |  |[default to false]
**media** | Option<**String**> | Filter resulting orphans based on their media type |  |

### Return type

[**crate::models::OrphanRecordingListing**](OrphanRecordingListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_recording_batchrequest

> crate::models::BatchDownloadJobStatusResult get_recording_batchrequest(job_id)
Get the status and results for a batch request job, only the user that submitted the job may retrieve results

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | jobId | [required] |

### Return type

[**crate::models::BatchDownloadJobStatusResult**](BatchDownloadJobStatusResult.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_recording_crossplatform_mediaretentionpolicies

> crate::models::PolicyEntityListing get_recording_crossplatform_mediaretentionpolicies(page_size, page_number, sort_by, expand, next_page, previous_page, name, enabled, summary, has_errors)
Gets media retention policy list with query options to filter on name and enabled.

for a less verbose response, add summary=true to this endpoint

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | The total page size requested |  |[default to 25]
**page_number** | Option<**i32**> | The page number requested |  |[default to 1]
**sort_by** | Option<**String**> | variable name requested to sort by |  |
**expand** | Option<[**Vec<String>**](String.md)> | variable name requested by expand list |  |
**next_page** | Option<**String**> | next page token |  |
**previous_page** | Option<**String**> | Previous page token |  |
**name** | Option<**String**> | the policy name - used for filtering results in searches. |  |
**enabled** | Option<**bool**> | checks to see if policy is enabled - use enabled = true or enabled = false |  |
**summary** | Option<**bool**> | provides a less verbose response of policy lists. |  |[default to false]
**has_errors** | Option<**bool**> | provides a way to fetch all policies with errors or policies that do not have errors |  |

### Return type

[**crate::models::PolicyEntityListing**](PolicyEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_recording_crossplatform_mediaretentionpolicy

> crate::models::CrossPlatformPolicy get_recording_crossplatform_mediaretentionpolicy(policy_id)
Get a media retention policy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **String** | Policy ID | [required] |

### Return type

[**crate::models::CrossPlatformPolicy**](CrossPlatformPolicy.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_recording_job

> crate::models::RecordingJob get_recording_job(job_id)
Get the status of the job associated with the job id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | jobId | [required] |

### Return type

[**crate::models::RecordingJob**](RecordingJob.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_recording_job_failedrecordings

> crate::models::FailedRecordingEntityListing get_recording_job_failedrecordings(job_id, page_size, page_number, include_total, cursor)
Get IDs of recordings that the bulk job failed for

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | jobId | [required] |
**page_size** | Option<**i32**> | Page size. Maximum is 100. |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**include_total** | Option<**bool**> | If false, cursor will be used to locate the page instead of pageNumber. |  |
**cursor** | Option<**String**> | Indicates where to resume query results (not required for first page) |  |

### Return type

[**crate::models::FailedRecordingEntityListing**](FailedRecordingEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_recording_jobs

> crate::models::RecordingJobEntityListing get_recording_jobs(page_size, page_number, sort_by, state, show_only_my_jobs, job_type, include_total, cursor)
Get the status of all jobs within the user's organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**sort_by** | Option<**String**> | Sort by |  |[default to userId]
**state** | Option<**String**> | Filter by state |  |
**show_only_my_jobs** | Option<**bool**> | Show only my jobs |  |
**job_type** | Option<**String**> | Job Type (Can be left empty for both) |  |
**include_total** | Option<**bool**> | If false, cursor will be used to locate the page instead of pageNumber. |  |
**cursor** | Option<**String**> | Indicates where to resume query results (not required for first page) |  |

### Return type

[**crate::models::RecordingJobEntityListing**](RecordingJobEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_recording_localkeys_setting

> crate::models::LocalEncryptionConfiguration get_recording_localkeys_setting(settings_id)
Get the local encryption settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**settings_id** | **String** | Settings Id | [required] |

### Return type

[**crate::models::LocalEncryptionConfiguration**](LocalEncryptionConfiguration.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_recording_localkeys_settings

> crate::models::LocalEncryptionConfigurationListing get_recording_localkeys_settings()
gets a list local key settings data

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LocalEncryptionConfigurationListing**](LocalEncryptionConfigurationListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_recording_mediaretentionpolicies

> crate::models::PolicyEntityListing get_recording_mediaretentionpolicies(page_size, page_number, sort_by, expand, next_page, previous_page, name, enabled, summary, has_errors)
Gets media retention policy list with query options to filter on name and enabled.

for a less verbose response, add summary=true to this endpoint

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | The total page size requested |  |[default to 25]
**page_number** | Option<**i32**> | The page number requested |  |[default to 1]
**sort_by** | Option<**String**> | variable name requested to sort by |  |
**expand** | Option<[**Vec<String>**](String.md)> | variable name requested by expand list |  |
**next_page** | Option<**String**> | next page token |  |
**previous_page** | Option<**String**> | Previous page token |  |
**name** | Option<**String**> | the policy name - used for filtering results in searches. |  |
**enabled** | Option<**bool**> | checks to see if policy is enabled - use enabled = true or enabled = false |  |
**summary** | Option<**bool**> | provides a less verbose response of policy lists. |  |[default to false]
**has_errors** | Option<**bool**> | provides a way to fetch all policies with errors or policies that do not have errors |  |

### Return type

[**crate::models::PolicyEntityListing**](PolicyEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_recording_mediaretentionpolicy

> crate::models::Policy get_recording_mediaretentionpolicy(policy_id)
Get a media retention policy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **String** | Policy ID | [required] |

### Return type

[**crate::models::Policy**](Policy.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_recording_recordingkeys

> crate::models::EncryptionKeyEntityListing get_recording_recordingkeys(page_size, page_number)
Get encryption key list

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]

### Return type

[**crate::models::EncryptionKeyEntityListing**](EncryptionKeyEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_recording_recordingkeys_rotationschedule

> crate::models::KeyRotationSchedule get_recording_recordingkeys_rotationschedule()
Get key rotation schedule

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::KeyRotationSchedule**](KeyRotationSchedule.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_recording_settings

> crate::models::RecordingSettings get_recording_settings(create_default)
Get the Recording Settings for the Organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_default** | Option<**bool**> | If no settings are found, a new one is created with default values |  |[default to false]

### Return type

[**crate::models::RecordingSettings**](RecordingSettings.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_recordings_screensessions

> crate::models::ScreenRecordingSessionListing get_recordings_screensessions(page_size, page_number)
Retrieves a paged listing of screen recording sessions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]

### Return type

[**crate::models::ScreenRecordingSessionListing**](ScreenRecordingSessionListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_recording_crossplatform_mediaretentionpolicy

> crate::models::CrossPlatformPolicy patch_recording_crossplatform_mediaretentionpolicy(policy_id, body)
Patch a media retention policy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **String** | Policy ID | [required] |
**body** | [**CrossPlatformPolicyUpdate**](CrossPlatformPolicyUpdate.md) | Policy | [required] |

### Return type

[**crate::models::CrossPlatformPolicy**](CrossPlatformPolicy.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_recording_mediaretentionpolicy

> crate::models::Policy patch_recording_mediaretentionpolicy(policy_id, body)
Patch a media retention policy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **String** | Policy ID | [required] |
**body** | [**PolicyUpdate**](PolicyUpdate.md) | Policy | [required] |

### Return type

[**crate::models::Policy**](Policy.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_recordings_screensession

> patch_recordings_screensession(recording_session_id, body)
Update a screen recording session

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**recording_session_id** | **String** | Screen recording session ID | [required] |
**body** | Option<[**ScreenRecordingSessionRequest**](ScreenRecordingSessionRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_conversation_recording_annotations

> crate::models::Annotation post_conversation_recording_annotations(conversation_id, recording_id, body)
Create annotation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | Conversation ID | [required] |
**recording_id** | **String** | Recording ID | [required] |
**body** | [**Annotation**](Annotation.md) | annotation | [required] |

### Return type

[**crate::models::Annotation**](Annotation.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_recording_batchrequests

> crate::models::BatchDownloadJobSubmissionResult post_recording_batchrequests(body)
Submit a batch download request for recordings. Recordings in response will be in their original format/codec - configured in the Trunk configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**BatchDownloadJobSubmission**](BatchDownloadJobSubmission.md) | Job submission criteria | [required] |

### Return type

[**crate::models::BatchDownloadJobSubmissionResult**](BatchDownloadJobSubmissionResult.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_recording_crossplatform_mediaretentionpolicies

> crate::models::CrossPlatformPolicy post_recording_crossplatform_mediaretentionpolicies(body)
Create media retention policy

Policy does not work retroactively

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**CrossPlatformPolicyCreate**](CrossPlatformPolicyCreate.md) | Policy | [required] |

### Return type

[**crate::models::CrossPlatformPolicy**](CrossPlatformPolicy.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_recording_jobs

> crate::models::RecordingJob post_recording_jobs(body)
Create a recording bulk job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**RecordingJobsQuery**](RecordingJobsQuery.md) | query | [required] |

### Return type

[**crate::models::RecordingJob**](RecordingJob.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_recording_localkeys

> crate::models::EncryptionKey post_recording_localkeys(body)
create a local recording key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**LocalEncryptionKeyRequest**](LocalEncryptionKeyRequest.md) | Local Encryption body | [required] |

### Return type

[**crate::models::EncryptionKey**](EncryptionKey.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_recording_localkeys_settings

> crate::models::LocalEncryptionConfiguration post_recording_localkeys_settings(body)
create settings for local key creation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**LocalEncryptionConfiguration**](LocalEncryptionConfiguration.md) | Local Encryption Configuration | [required] |

### Return type

[**crate::models::LocalEncryptionConfiguration**](LocalEncryptionConfiguration.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_recording_mediaretentionpolicies

> crate::models::Policy post_recording_mediaretentionpolicies(body)
Create media retention policy

Policy does not work retroactively

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**PolicyCreate**](PolicyCreate.md) | Policy | [required] |

### Return type

[**crate::models::Policy**](Policy.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_recording_recordingkeys

> crate::models::EncryptionKey post_recording_recordingkeys()
Create encryption key

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::EncryptionKey**](EncryptionKey.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_recordings_deletionprotection

> Vec<crate::models::AddressableEntityRef> post_recordings_deletionprotection(body)
Get a list of conversations with protected recordings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ConversationDeletionProtectionQuery**](ConversationDeletionProtectionQuery.md) | conversationIds | [required] |

### Return type

[**Vec<crate::models::AddressableEntityRef>**](AddressableEntityRef.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_recordings_screensessions_acknowledge

> post_recordings_screensessions_acknowledge(body)
Acknowledge a screen recording.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**AcknowledgeScreenRecordingRequest**](AcknowledgeScreenRecordingRequest.md) | AcknowledgeScreenRecordingRequest | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_recordings_screensessions_metadata

> post_recordings_screensessions_metadata(body)
Provide meta-data a screen recording.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ScreenRecordingMetaDataRequest**](ScreenRecordingMetaDataRequest.md) | ScreenRecordingMetaDataRequest | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_conversation_recording

> crate::models::Recording put_conversation_recording(conversation_id, recording_id, body)
Updates the retention records on a recording.

Currently supports updating and removing both archive and delete dates for eligible recordings. A request to change the archival date of an archived recording will result in a restoration of the recording until the new date set. The recording:recording:view permission is required for the recording, as well as either the recording:recording:editRetention or recording:screenRecording:editRetention permissions depending on the type of recording.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | Conversation ID | [required] |
**recording_id** | **String** | Recording ID | [required] |
**body** | [**Recording**](Recording.md) | recording | [required] |

### Return type

[**crate::models::Recording**](Recording.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_conversation_recording_annotation

> crate::models::Annotation put_conversation_recording_annotation(conversation_id, recording_id, annotation_id, body)
Update annotation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | Conversation ID | [required] |
**recording_id** | **String** | Recording ID | [required] |
**annotation_id** | **String** | Annotation ID | [required] |
**body** | [**Annotation**](Annotation.md) | annotation | [required] |

### Return type

[**crate::models::Annotation**](Annotation.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_orphanrecording

> crate::models::Recording put_orphanrecording(orphan_id, body)
Updates an orphan recording to a regular recording with retention values

If this operation is successful the orphan will no longer exist. It will be replaced by the resulting recording in the response. This replacement recording is accessible by the normal Recording api.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**orphan_id** | **String** | Orphan ID | [required] |
**body** | Option<[**OrphanUpdateRequest**](OrphanUpdateRequest.md)> |  |  |

### Return type

[**crate::models::Recording**](Recording.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_recording_crossplatform_mediaretentionpolicy

> crate::models::CrossPlatformPolicy put_recording_crossplatform_mediaretentionpolicy(policy_id, body)
Update a media retention policy

Policy does not work retroactively

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **String** | Policy ID | [required] |
**body** | [**CrossPlatformPolicy**](CrossPlatformPolicy.md) | Policy | [required] |

### Return type

[**crate::models::CrossPlatformPolicy**](CrossPlatformPolicy.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_recording_job

> crate::models::RecordingJob put_recording_job(job_id, body)
Execute the recording bulk job.

A job must be executed by the same user whom originally created the job.  In addition, the user must have permission to update the recording's retention.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | jobId | [required] |
**body** | [**ExecuteRecordingJobsQuery**](ExecuteRecordingJobsQuery.md) | query | [required] |

### Return type

[**crate::models::RecordingJob**](RecordingJob.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_recording_localkeys_setting

> crate::models::LocalEncryptionConfiguration put_recording_localkeys_setting(settings_id, body)
Update the local encryption settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**settings_id** | **String** | Settings Id | [required] |
**body** | [**LocalEncryptionConfiguration**](LocalEncryptionConfiguration.md) | Local Encryption metadata | [required] |

### Return type

[**crate::models::LocalEncryptionConfiguration**](LocalEncryptionConfiguration.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_recording_mediaretentionpolicy

> crate::models::Policy put_recording_mediaretentionpolicy(policy_id, body)
Update a media retention policy

Policy does not work retroactively

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **String** | Policy ID | [required] |
**body** | [**Policy**](Policy.md) | Policy | [required] |

### Return type

[**crate::models::Policy**](Policy.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_recording_recordingkeys_rotationschedule

> crate::models::KeyRotationSchedule put_recording_recordingkeys_rotationschedule(body)
Update key rotation schedule

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**KeyRotationSchedule**](KeyRotationSchedule.md) | KeyRotationSchedule | [required] |

### Return type

[**crate::models::KeyRotationSchedule**](KeyRotationSchedule.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_recording_settings

> crate::models::RecordingSettings put_recording_settings(body)
Update the Recording Settings for the Organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**RecordingSettings**](RecordingSettings.md) | Recording settings | [required] |

### Return type

[**crate::models::RecordingSettings**](RecordingSettings.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_recordings_deletionprotection

> put_recordings_deletionprotection(protect, body)
Apply or revoke recording protection for conversations

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**protect** | Option<**bool**> | Check for apply, uncheck for revoke (each action requires the respective permission) |  |[default to true]
**body** | Option<[**ConversationDeletionProtectionQuery**](ConversationDeletionProtectionQuery.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

