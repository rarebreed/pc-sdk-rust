# CallConversation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**participants** | Option<[**Vec<crate::models::CallMediaParticipant>**](CallMediaParticipant.md)> | The list of participants involved in the conversation. | [optional]
**other_media_uris** | Option<**Vec<String>**> | The list of other media channels involved in the conversation. | [optional]
**recording_state** | Option<**String**> |  | [optional]
**max_participants** | Option<**i32**> | If this is a conference conversation, then this field indicates the maximum number of participants allowed to participant in the conference. | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


