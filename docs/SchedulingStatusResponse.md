# SchedulingStatusResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The ID generated for the scheduling job.  Use to GET result when job is completed. | [optional][readonly]
**status** | Option<**String**> | The status of the scheduling job. | [optional][readonly]
**error_details** | Option<[**Vec<crate::models::SchedulingProcessingError>**](SchedulingProcessingError.md)> | If the request could not be properly processed, error details will be given here. | [optional][readonly]
**scheduling_result_uri** | Option<**String**> | The uri of the scheduling result. It has a value if the status is 'Success'. | [optional][readonly]
**percent_complete** | Option<**i32**> | The percentage of the job that is complete. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


