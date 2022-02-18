# AnalyticsConversationWithoutAttributes

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**conversation_end** | Option<**String**> | The end time of a conversation. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**conversation_id** | Option<**String**> | Unique identifier for the conversation | [optional]
**conversation_initiator** | Option<**String**> | Indicates the participant purpose of the participant initiating a message conversation | [optional]
**conversation_start** | Option<**String**> | The start time of a conversation. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**customer_participation** | Option<**bool**> | Indicates a messaging conversation in which the customer participated by sending at least one message | [optional]
**division_ids** | Option<**Vec<String>**> | Identifier(s) of division(s) associated with a conversation | [optional]
**external_tag** | Option<**String**> | External tag for the conversation | [optional]
**knowledge_base_ids** | Option<**Vec<String>**> | The unique identifier(s) of the knowledge base(s) used | [optional]
**media_stats_min_conversation_mos** | Option<**f64**> | The lowest estimated average MOS among all the audio streams belonging to this conversation | [optional]
**media_stats_min_conversation_r_factor** | Option<**f64**> | The lowest R-factor value among all of the audio streams belonging to this conversation | [optional]
**originating_direction** | Option<**String**> | The original direction of the conversation | [optional]
**self_served** | Option<**bool**> | Indicates whether all flow sessions were self serviced | [optional]
**evaluations** | Option<[**Vec<crate::models::AnalyticsEvaluation>**](AnalyticsEvaluation.md)> | Evaluations associated with this conversation | [optional]
**surveys** | Option<[**Vec<crate::models::AnalyticsSurvey>**](AnalyticsSurvey.md)> | Surveys associated with this conversation | [optional]
**resolutions** | Option<[**Vec<crate::models::AnalyticsResolution>**](AnalyticsResolution.md)> | Resolutions associated with this conversation | [optional]
**participants** | Option<[**Vec<crate::models::AnalyticsParticipantWithoutAttributes>**](AnalyticsParticipantWithoutAttributes.md)> | Participants in the conversation | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


