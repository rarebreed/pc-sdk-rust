# ModelingStatusResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The ID generated for the modeling job.  Use to GET result when job is completed. | [optional][readonly]
**status** | Option<**String**> | The status of the modeling job. | [optional][readonly]
**error_details** | Option<[**Vec<crate::models::ModelingProcessingError>**](ModelingProcessingError.md)> | If the request could not be properly processed, error details will be given here. | [optional][readonly]
**modeling_result_uri** | Option<**String**> | The uri of the modeling result. It has a value if the status is either 'Success', 'PartialFailure', or 'Failed'. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


