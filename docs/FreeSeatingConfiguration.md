# FreeSeatingConfiguration

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**free_seating_state** | Option<**String**> | The FreeSeatingState for FreeSeatingConfiguration. Can be ON, OFF, or PARTIAL. ON meaning disassociate the user after the ttl expires, OFF meaning never disassociate the user, and PARTIAL meaning only disassociate when a user explicitly clicks log out. | [optional]
**ttl_minutes** | Option<**i32**> | The amount of time in minutes until an offline user is disassociated from their station | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


