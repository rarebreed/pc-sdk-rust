# RoutingData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**queue_id** | **String** | The identifier of the routing queue | 
**language_id** | Option<**String**> | The identifier of a language to be considered in routing | [optional]
**priority** | Option<**i32**> | The priority for routing | [optional]
**skill_ids** | Option<**Vec<String>**> | A list of skill identifiers to be considered in routing | [optional]
**preferred_agent_ids** | Option<**Vec<String>**> | A list of agents to be preferred in routing | [optional]
**scored_agents** | Option<[**Vec<crate::models::ScoredAgent>**](ScoredAgent.md)> | A list of scored agents for routing decisions | [optional]
**routing_flags** | Option<**Vec<String>**> | An array of flags indicating how the conversation should be routed | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


