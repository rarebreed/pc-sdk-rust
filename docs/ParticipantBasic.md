# ParticipantBasic

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | A globally unique identifier for this conversation. | [optional]
**start_time** | Option<**String**> | The timestamp when this participant joined the conversation in the provider clock. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**end_time** | Option<**String**> | The timestamp when this participant disconnected from the conversation in the provider clock. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**connected_time** | Option<**String**> | The timestamp when this participant was connected to the conversation in the provider clock. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**name** | Option<**String**> | A human readable name identifying the participant. | [optional]
**user_uri** | Option<**String**> | If this participant represents a user, then this will be an URI that can be used to fetch the user. | [optional]
**user_id** | Option<**String**> | If this participant represents a user, then this will be the globally unique identifier for the user. | [optional]
**external_contact_id** | Option<**String**> | If this participant represents an external contact, then this will be the globally unique identifier for the external contact. | [optional]
**external_organization_id** | Option<**String**> | If this participant represents an external org, then this will be the globally unique identifier for the external org. | [optional]
**queue_id** | Option<**String**> | If present, the queue id that the communication channel came in on. | [optional]
**group_id** | Option<**String**> | If present, group of users the participant represents. | [optional]
**team_id** | Option<**String**> | The team id that this participant is a member of when added to the conversation. | [optional]
**queue_name** | Option<**String**> | If present, the queue name that the communication channel came in on. | [optional]
**purpose** | Option<**String**> | A well known string that specifies the purpose of this participant. | [optional]
**participant_type** | Option<**String**> | A well known string that specifies the type of this participant. | [optional]
**consult_participant_id** | Option<**String**> | If this participant is part of a consult transfer, then this will be the participant id of the participant being transferred. | [optional]
**address** | Option<**String**> | The address for the this participant. For a phone call this will be the ANI. | [optional]
**ani** | Option<**String**> | The address for the this participant. For a phone call this will be the ANI. | [optional]
**ani_name** | Option<**String**> | The ani-based name for this participant. | [optional]
**dnis** | Option<**String**> | The address for the this participant. For a phone call this will be the ANI. | [optional]
**locale** | Option<**String**> | An ISO 639 language code specifying the locale for this participant | [optional]
**wrapup_required** | Option<**bool**> | True iff this participant is required to enter wrapup for this conversation. | [optional]
**wrapup_prompt** | Option<**String**> | This field controls how the UI prompts the agent for a wrapup. | [optional]
**wrapup_timeout_ms** | Option<**i32**> | Specifies how long a timed ACW session will last. | [optional]
**wrapup_skipped** | Option<**bool**> | The UI sets this field when the agent chooses to skip entering a wrapup for this participant. | [optional]
**wrapup** | Option<[**crate::models::Wrapup**](Wrapup.md)> |  | [optional]
**conversation_routing_data** | Option<[**crate::models::ConversationRoutingData**](ConversationRoutingData.md)> |  | [optional]
**alerting_timeout_ms** | Option<**i32**> | Specifies how long the agent has to answer an interaction before being marked as not responding. | [optional]
**monitored_participant_id** | Option<**String**> | If this participant is a monitor, then this will be the id of the participant that is being monitored. | [optional]
**coached_participant_id** | Option<**String**> | If this participant is a coach, then this will be the id of the participant that is being coached. | [optional]
**attributes** | Option<**::std::collections::HashMap<String, String>**> | Additional participant attributes | [optional]
**calls** | Option<[**Vec<crate::models::CallBasic>**](CallBasic.md)> |  | [optional]
**callbacks** | Option<[**Vec<crate::models::CallbackBasic>**](CallbackBasic.md)> |  | [optional]
**chats** | Option<[**Vec<crate::models::ConversationChat>**](ConversationChat.md)> |  | [optional]
**cobrowsesessions** | Option<[**Vec<crate::models::Cobrowsesession>**](Cobrowsesession.md)> |  | [optional]
**emails** | Option<[**Vec<crate::models::Email>**](Email.md)> |  | [optional]
**messages** | Option<[**Vec<crate::models::Message>**](Message.md)> |  | [optional]
**screenshares** | Option<[**Vec<crate::models::Screenshare>**](Screenshare.md)> |  | [optional]
**social_expressions** | Option<[**Vec<crate::models::SocialExpression>**](SocialExpression.md)> |  | [optional]
**videos** | Option<[**Vec<crate::models::Video>**](Video.md)> |  | [optional]
**evaluations** | Option<[**Vec<crate::models::Evaluation>**](Evaluation.md)> |  | [optional]
**screen_recording_state** | Option<**String**> | The current screen recording state for this participant. | [optional]
**flagged_reason** | Option<**String**> | The reason specifying why participant flagged the conversation. | [optional]
**start_acw_time** | Option<**String**> | The timestamp when this participant started after-call work. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**end_acw_time** | Option<**String**> | The timestamp when this participant ended after-call work. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

