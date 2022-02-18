# WorkPlanRotationResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**enabled** | Option<**bool**> | Whether the work plan rotation is enabled for scheduling | [optional]
**date_range** | Option<[**crate::models::DateRangeWithOptionalEnd**](DateRangeWithOptionalEnd.md)> |  | [optional]
**pattern** | Option<[**crate::models::WorkPlanPatternResponse**](WorkPlanPatternResponse.md)> |  | [optional]
**agent_count** | Option<**i32**> | Number of agents in this work plan rotation | [optional]
**agents** | Option<[**Vec<crate::models::WorkPlanRotationAgentResponse>**](WorkPlanRotationAgentResponse.md)> | Agents in this work plan rotation. Populate with expand=agents for GET WorkPlanRotationsList (defaults to empty list) | [optional]
**metadata** | Option<[**crate::models::WfmVersionedEntityMetadata**](WfmVersionedEntityMetadata.md)> |  | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


