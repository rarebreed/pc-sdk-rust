# ShiftTradeNotification

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**week_date** | Option<**String**> | The start date of the schedule with which this trade is associated | [optional]
**trade_id** | Option<**String**> | The ID of the shift trade | [optional]
**one_sided** | Option<**bool**> | Whether this is a one sided shift trade | [optional]
**new_state** | Option<**String**> | The new state of the shift trade, null if there was no change | [optional]
**initiating_user** | Option<[**crate::models::UserReference**](UserReference.md)> |  | [optional]
**initiating_shift_date** | Option<**String**> | The start date and time of the initiating shift. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**receiving_user** | Option<[**crate::models::UserReference**](UserReference.md)> |  | [optional]
**receiving_shift_date** | Option<**String**> | The start date and time of the receiving shift (null if not matched or if one-sided. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


