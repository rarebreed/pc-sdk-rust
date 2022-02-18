# CampaignInteraction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [optional]
**campaign** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**agent** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**contact** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**destination_address** | Option<**String**> |  | [optional]
**active_preview_call** | Option<**bool**> | Boolean value if there is an active preview call on the interaction | [optional]
**last_active_preview_wrapup_time** | Option<**String**> | The time when the last preview of the interaction was wrapped up. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**creation_time** | Option<**String**> | The time when dialer created the interaction. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**call_placed_time** | Option<**String**> | The time when the agent or system places the call. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**call_routed_time** | Option<**String**> | The time when the agent was connected to the call. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**preview_connected_time** | Option<**String**> | The time when the customer and routing participant are connected. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**queue** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**script** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**disposition** | Option<**String**> | Describes what happened with call analysis for instance: disposition.classification.callable.person, disposition.classification.callable.noanswer | [optional]
**caller_name** | Option<**String**> |  | [optional]
**caller_address** | Option<**String**> |  | [optional]
**preview_pop_delivered_time** | Option<**String**> | Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**conversation** | Option<[**crate::models::ConversationBasic**](ConversationBasic.md)> |  | [optional]
**dialer_system_participant_id** | Option<**String**> | conversation participant id that is the dialer system participant to monitor the call from dialer perspective | [optional]
**dialing_mode** | Option<**String**> |  | [optional]
**skills** | Option<[**Vec<crate::models::DomainEntityRef>**](DomainEntityRef.md)> | Any skills that are attached to the call for routing | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


