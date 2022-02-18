# UpdateWorkPlanRotationAgentRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user_id** | **String** | The ID of an agent in this work plan rotation | 
**date_range** | Option<[**crate::models::DateRangeWithOptionalEnd**](DateRangeWithOptionalEnd.md)> |  | [optional]
**position** | Option<**i32**> | Start position of the work plan in the pattern for this agent in the work plan rotation. Position value starts from 0 | [optional]
**delete** | Option<**bool**> | If marked true for this agent when updating, then this agent will be removed from this work plan rotation | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


