# SecureSession

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**flow** | [**crate::models::DomainEntityRef**](DomainEntityRef.md) |  | 
**user_data** | Option<**String**> | Customer-provided data | [optional]
**state** | **String** | The current state of a secure session | 
**source_participant_id** | Option<**String**> | Unique identifier for the participant initiating the secure session. | [optional]
**disconnect** | Option<**bool**> | If true, disconnect the agent after creating the session | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


