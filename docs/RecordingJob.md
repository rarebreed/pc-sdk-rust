# RecordingJob

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**state** | **String** | The current state of the job. | 
**recording_jobs_query** | Option<[**crate::models::RecordingJobsQuery**](RecordingJobsQuery.md)> |  | [optional]
**date_created** | Option<**String**> | Date when the job was created. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**total_conversations** | Option<**i32**> | Total number of conversations affected. | [optional][readonly]
**total_recordings** | Option<**i32**> | Total number of recordings affected. | [optional][readonly]
**total_skipped_recordings** | Option<**i32**> | Total number of recordings that have been skipped. | [optional][readonly]
**total_failed_recordings** | Option<**i32**> | Total number of recordings that the bulk job failed to process. | [optional][readonly]
**total_processed_recordings** | Option<**i32**> | Total number of recordings have been processed. | [optional][readonly]
**percent_progress** | Option<**i32**> | Progress in percentage based on the number of recordings | [optional][readonly]
**error_message** | Option<**String**> | Error occurred during the job execution | [optional][readonly]
**failed_recordings** | Option<**String**> | Get IDs of recordings that the bulk job failed for | [optional][readonly]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]
**user** | Option<[**crate::models::AddressableEntityRef**](AddressableEntityRef.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


