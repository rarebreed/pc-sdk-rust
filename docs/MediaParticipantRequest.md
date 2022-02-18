# MediaParticipantRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**wrapup** | Option<[**crate::models::Wrapup**](Wrapup.md)> |  | [optional]
**state** | Option<**String**> | The state to update to set for this participant's communications.  Possible values are: 'connected' and 'disconnected'. | [optional]
**recording** | Option<**bool**> | True to enable recording of this participant, otherwise false to disable recording. | [optional]
**muted** | Option<**bool**> | True to mute this conversation participant. | [optional]
**confined** | Option<**bool**> | True to confine this conversation participant.  Should only be used for ad-hoc conferences | [optional]
**held** | Option<**bool**> | True to hold this conversation participant. | [optional]
**wrapup_skipped** | Option<**bool**> | True to skip wrap-up for this participant. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


