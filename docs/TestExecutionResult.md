# TestExecutionResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**operations** | Option<[**Vec<crate::models::TestExecutionOperationResult>**](TestExecutionOperationResult.md)> | Execution operations performed as part of the test | [optional]
**error** | Option<[**crate::models::ErrorBody**](ErrorBody.md)> |  | [optional]
**final_result** | Option<[**serde_json::Value**](.md)> | The final result of the test. This is the response that would be returned during normal action execution | [optional]
**success** | Option<**bool**> | Indicates whether or not the test was a success | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


