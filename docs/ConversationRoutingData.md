# ConversationRoutingData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**queue** | Option<[**crate::models::AddressableEntityRef**](AddressableEntityRef.md)> |  | [optional]
**language** | Option<[**crate::models::AddressableEntityRef**](AddressableEntityRef.md)> |  | [optional]
**priority** | Option<**i32**> | The priority of the conversation to use for routing decisions | [optional]
**skills** | Option<[**Vec<crate::models::AddressableEntityRef>**](AddressableEntityRef.md)> | The skills to use for routing decisions | [optional]
**scored_agents** | Option<[**Vec<crate::models::ScoredAgent>**](ScoredAgent.md)> | A collection of agents and their assigned scores for this conversation (0 - 100, higher being better), for use in routing to preferred agents | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


