# RoutingConversationAttributesRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**priority** | Option<**i32**> | Priority for the conversation.  Each point of priority is equivalent to one minute of time in queue.  Range:[-25000000, 25000000].  To reset, specify 0. | [optional]
**skill_ids** | Option<**Vec<String>**> | Skill requirements for the conversation.  To remove all skill requirements, specify an empty list, i.e. []. | [optional]
**language_id** | Option<**String**> | Language requirement for the conversation.  To remove the language requirement, specify an empty string, i.e., \"\". | [optional]
**request_scored_agents** | Option<[**Vec<crate::models::RequestScoredAgent>**](RequestScoredAgent.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


