# AddShiftTradeRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**schedule_id** | **String** | The ID of the schedule to which the initiating and receiving shifts belong | 
**initiating_shift_id** | **String** | The ID of the shift that the initiating user wants to give up | 
**receiving_user_id** | Option<**String**> | The ID of the user to whom to send the request (for use in direct trade requests) | [optional]
**expiration** | Option<**String**> | When this shift trade request should expire. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**acceptable_intervals** | Option<**Vec<String>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


