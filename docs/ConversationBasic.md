# ConversationBasic

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**external_tag** | Option<**String**> | The external tag associated with the conversation. | [optional]
**start_time** | **String** | The time when the conversation started. This will be the time when the first participant joined the conversation. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | 
**end_time** | Option<**String**> | The time when the conversation ended. This will be the time when the last participant left the conversation, or null when the conversation is still active. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**divisions** | Option<[**Vec<crate::models::ConversationDivisionMembership>**](ConversationDivisionMembership.md)> | Identifiers of divisions associated with this conversation | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]
**participants** | Option<[**Vec<crate::models::ParticipantBasic>**](ParticipantBasic.md)> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


