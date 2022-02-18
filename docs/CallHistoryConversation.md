# CallHistoryConversation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**participants** | Option<[**Vec<crate::models::CallHistoryParticipant>**](CallHistoryParticipant.md)> | The list of participants involved in the conversation. | [optional]
**direction** | Option<**String**> | The direction of the call relating to the current user | [optional]
**went_to_voicemail** | Option<**bool**> | Did the call end in the current user's voicemail | [optional]
**missed_call** | Option<**bool**> | Did the user not answer this conversation | [optional]
**start_time** | Option<**String**> | The time the user joined the conversation. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**was_conference** | Option<**bool**> | Was this conversation a conference | [optional]
**was_callback** | Option<**bool**> | Was this conversation a callback | [optional]
**had_screen_share** | Option<**bool**> | Did this conversation have a screen share session | [optional]
**had_cobrowse** | Option<**bool**> | Did this conversation have a cobrowse session | [optional]
**was_outbound_campaign** | Option<**bool**> | Was this conversation associated with an outbound campaign | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


