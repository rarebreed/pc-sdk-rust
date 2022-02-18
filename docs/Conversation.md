# Conversation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**external_tag** | Option<**String**> | The external tag associated with the conversation. | [optional]
**start_time** | **String** | The time when the conversation started. This will be the time when the first participant joined the conversation. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | 
**end_time** | Option<**String**> | The time when the conversation ended. This will be the time when the last participant left the conversation, or null when the conversation is still active. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**address** | Option<**String**> | The address of the conversation as seen from an external participant. For phone calls this will be the DNIS for inbound calls and the ANI for outbound calls. For other media types this will be the address of the destination participant for inbound and the address of the initiating participant for outbound. | [optional]
**participants** | [**Vec<crate::models::Participant>**](Participant.md) | The list of all participants in the conversation. | 
**conversation_ids** | Option<**Vec<String>**> | A list of conversations to merge into this conversation to create a conference. This field is null except when being used to create a conference. | [optional]
**max_participants** | Option<**i32**> | If this is a conference conversation, then this field indicates the maximum number of participants allowed to participant in the conference. | [optional]
**recording_state** | Option<**String**> | On update, 'paused' initiates a secure pause, 'active' resumes any paused recordings; otherwise indicates state of conversation recording. | [optional]
**state** | Option<**String**> | The conversation's state | [optional]
**divisions** | Option<[**Vec<crate::models::ConversationDivisionMembership>**](ConversationDivisionMembership.md)> | Identifiers of divisions associated with this conversation | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


