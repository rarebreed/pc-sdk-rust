# ObservationValue

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**observation_date** | **String** | The time at which the observation occurred. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | 
**conversation_id** | Option<**String**> | Unique identifier for the conversation | [optional]
**session_id** | Option<**String**> | The unique identifier of this session | [optional]
**requested_routing_skill_ids** | Option<**Vec<String>**> | Unique identifier for a skill requested for an interaction | [optional]
**requested_language_id** | Option<**String**> | Unique identifier for the language requested for an interaction | [optional]
**routing_priority** | Option<**i64**> | Routing priority for the current interaction | [optional]
**participant_name** | Option<**String**> | A human readable name identifying the participant | [optional]
**user_id** | Option<**String**> | Unique identifier for the user | [optional]
**direction** | Option<**String**> | The direction of the communication | [optional]
**converted_from** | Option<**String**> | Session media type that was converted from in case of a media type conversion | [optional]
**converted_to** | Option<**String**> | Session media type that was converted to in case of a media type conversion | [optional]
**address_from** | Option<**String**> | The address that initiated an action | [optional]
**address_to** | Option<**String**> | The address receiving an action | [optional]
**ani** | Option<**String**> | Automatic Number Identification (caller's number) | [optional]
**dnis** | Option<**String**> | Dialed number identification service (number dialed by the calling party) | [optional]
**team_id** | Option<**String**> | The team id the user is a member of | [optional]
**requested_routings** | Option<**Vec<String>**> | All routing types for requested/attempted routing methods | [optional]
**used_routing** | Option<**String**> | Complete routing method | [optional]
**scored_agents** | Option<[**Vec<crate::models::AnalyticsScoredAgent>**](AnalyticsScoredAgent.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


