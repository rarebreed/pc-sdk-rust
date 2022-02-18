# CallHistoryParticipant

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The unique participant ID. | [optional]
**name** | Option<**String**> | The display friendly name of the participant. | [optional]
**address** | Option<**String**> | The participant address. | [optional]
**start_time** | Option<**String**> | The time when this participant first joined the conversation. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**end_time** | Option<**String**> | The time when this participant went disconnected for this media (eg: video disconnected time). Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**purpose** | Option<**String**> | The participant's purpose.  Values can be: 'agent', 'user', 'customer', 'external', 'acd', 'ivr | [optional]
**direction** | Option<**String**> | The participant's direction.  Values can be: 'inbound' or 'outbound' | [optional]
**ani** | Option<**String**> | The call ANI. | [optional]
**dnis** | Option<**String**> | The call DNIS. | [optional]
**user** | Option<[**crate::models::User**](User.md)> |  | [optional]
**queue** | Option<[**crate::models::Queue**](Queue.md)> |  | [optional]
**group** | Option<[**crate::models::Group**](Group.md)> |  | [optional]
**disconnect_type** | Option<**String**> | The reason the participant was disconnected from the conversation. | [optional]
**external_contact** | Option<[**crate::models::ExternalContact**](ExternalContact.md)> |  | [optional]
**external_organization** | Option<[**crate::models::ExternalOrganization**](ExternalOrganization.md)> |  | [optional]
**did_interact** | Option<**bool**> | Indicates whether the contact ever connected | [optional]
**sip_response_codes** | Option<**Vec<i64>**> | Indicates SIP Response codes associated with the participant | [optional]
**flagged_reason** | Option<**String**> | The reason specifying why participant flagged the conversation. | [optional]
**outbound_campaign** | Option<[**crate::models::Campaign**](Campaign.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


