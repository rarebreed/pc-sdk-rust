# UpdateWorkPlanRotationRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Name of this work plan rotation | [optional]
**enabled** | Option<**bool**> | Whether the work plan rotation is enabled for scheduling | [optional]
**date_range** | Option<[**crate::models::DateRangeWithOptionalEnd**](DateRangeWithOptionalEnd.md)> |  | [optional]
**agents** | Option<[**Vec<crate::models::UpdateWorkPlanRotationAgentRequest>**](UpdateWorkPlanRotationAgentRequest.md)> | Agents in this work plan rotation | [optional]
**pattern** | Option<[**crate::models::WorkPlanPatternRequest**](WorkPlanPatternRequest.md)> |  | [optional]
**metadata** | [**crate::models::WfmVersionedEntityMetadata**](WfmVersionedEntityMetadata.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


