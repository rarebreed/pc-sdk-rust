# AnalyticsConversationSegment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**audio_muted** | Option<**bool**> | Flag indicating if audio is muted or not (true/false) | [optional]
**conference** | Option<**bool**> | Indicates whether the segment was a conference | [optional]
**destination_conversation_id** | Option<**String**> | The unique identifier of a new conversation when a conversation is ended for a conference | [optional]
**destination_session_id** | Option<**String**> | The unique identifier of a new session when a session is ended for a conference | [optional]
**disconnect_type** | Option<**String**> | The session disconnect type | [optional]
**error_code** | Option<**String**> | A code corresponding to the error that occurred | [optional]
**group_id** | Option<**String**> | Unique identifier for a PureCloud group | [optional]
**q850_response_codes** | Option<**Vec<i64>**> | Q.850 response code(s) | [optional]
**queue_id** | Option<**String**> | Queue identifier | [optional]
**requested_language_id** | Option<**String**> | Unique identifier for the language requested for an interaction | [optional]
**requested_routing_skill_ids** | Option<**Vec<String>**> | Unique identifier(s) for skill(s) requested for an interaction | [optional]
**requested_routing_user_ids** | Option<**Vec<String>**> | Unique identifier(s) for agent(s) requested for an interaction | [optional]
**segment_end** | Option<**String**> | The end time of a segment. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**segment_start** | Option<**String**> | The start time of a segment. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**segment_type** | Option<**String**> | The activity that takes place in the segment, such as hold or interact | [optional]
**sip_response_codes** | Option<**Vec<i64>**> | SIP response code(s) | [optional]
**source_conversation_id** | Option<**String**> | The unique identifier of the previous conversation when a new conversation is created for a conference | [optional]
**source_session_id** | Option<**String**> | The unique identifier of the previous session when a new session is created for a conference | [optional]
**subject** | Option<**String**> | The subject for the initial email that started this conversation | [optional]
**video_muted** | Option<**bool**> | Flag indicating if video is muted/paused or not (true/false) | [optional]
**wrap_up_code** | Option<**String**> | Wrap up code | [optional]
**wrap_up_note** | Option<**String**> | Note entered by an agent during after-call work | [optional]
**wrap_up_tags** | Option<**Vec<String>**> | Tag(s) assigned during after-call work | [optional]
**scored_agents** | Option<[**Vec<crate::models::AnalyticsScoredAgent>**](AnalyticsScoredAgent.md)> | Scored agents | [optional]
**properties** | Option<[**Vec<crate::models::AnalyticsProperty>**](AnalyticsProperty.md)> | Additional segment properties | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


