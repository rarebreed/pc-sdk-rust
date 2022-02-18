# BatchDownloadJobStatusResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**job_id** | Option<**String**> | JobId returned when job was initially submitted | [optional]
**expected_result_count** | Option<**i32**> | Number of results expected when job is completed | [optional]
**result_count** | Option<**i32**> | Current number of results available | [optional]
**error_count** | Option<**i32**> | Number of error results produced so far | [optional]
**results** | Option<[**Vec<crate::models::BatchDownloadJobResult>**](BatchDownloadJobResult.md)> | Current set of results for the job | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


