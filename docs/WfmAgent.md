# WfmAgent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**user** | Option<[**crate::models::UserReference**](UserReference.md)> |  | [optional]
**work_plan** | Option<[**crate::models::WorkPlanReference**](WorkPlanReference.md)> |  | [optional]
**work_plan_rotation** | Option<[**crate::models::WorkPlanRotationReference**](WorkPlanRotationReference.md)> |  | [optional]
**accept_direct_shift_trades** | Option<**bool**> | Whether the agent accepts direct shift trade requests | [optional]
**queues** | Option<[**Vec<crate::models::QueueReference>**](QueueReference.md)> | List of queues to which this agent is capable of handling | [optional]
**languages** | Option<[**Vec<crate::models::LanguageReference>**](LanguageReference.md)> | The list of languages this agent is capable of handling | [optional]
**skills** | Option<[**Vec<crate::models::RoutingSkillReference>**](RoutingSkillReference.md)> | The list of skills this agent is capable of handling | [optional]
**schedulable** | Option<**bool**> | Whether the agent has the permission to be included in schedule generation | [optional]
**metadata** | Option<[**crate::models::WfmVersionedEntityMetadata**](WfmVersionedEntityMetadata.md)> |  | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


