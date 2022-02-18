# RoutingConversationAttributesResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**priority** | Option<**i32**> | Current priority value on in-queue conversation. Range:[-25000000, 25000000] | [optional]
**skills** | Option<[**Vec<crate::models::RoutingSkill>**](RoutingSkill.md)> | Current routing skills on in-queue conversation | [optional]
**language** | Option<[**crate::models::Language**](Language.md)> |  | [optional]
**scored_agents** | Option<[**Vec<crate::models::ScoredAgent>**](ScoredAgent.md)> | Current scored agents on in-queue conversation | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


