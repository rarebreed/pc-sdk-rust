# CreateCallRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**phone_number** | Option<**String**> | The phone number to dial. | [optional]
**caller_id** | Option<**String**> | The caller id phone number for this outbound call. | [optional]
**caller_id_name** | Option<**String**> | The caller id name for this outbound call. | [optional]
**call_from_queue_id** | Option<**String**> | The queue ID to call on behalf of. | [optional]
**call_queue_id** | Option<**String**> | The queue ID to call. | [optional]
**call_user_id** | Option<**String**> | The user ID to call. | [optional]
**priority** | Option<**i32**> | The priority to assign to this call (if calling a queue). | [optional]
**language_id** | Option<**String**> | The language skill ID to use for routing this call (if calling a queue). | [optional]
**routing_skills_ids** | Option<**Vec<String>**> | The skill ID's to use for routing this call (if calling a queue). | [optional]
**conversation_ids** | Option<**Vec<String>**> | The list of existing call conversations to merge into a new ad-hoc conference. | [optional]
**participants** | Option<[**Vec<crate::models::Destination>**](Destination.md)> | The list of participants to call to create a new ad-hoc conference. | [optional]
**uui_data** | Option<**String**> | User to User Information (UUI) data managed by SIP session application. | [optional]
**external_contact_id** | Option<**String**> | The external contact with which to associate the call. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


