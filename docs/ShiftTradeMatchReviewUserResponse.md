# ShiftTradeMatchReviewUserResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**weekly_minimum_paid_minutes** | Option<**i32**> | The minimum weekly paid minutes for this user per the work plan tied to the agent schedule | [optional]
**weekly_maximum_paid_minutes** | Option<**i32**> | The maximum weekly paid minutes for this user per the work plan tied to the agent schedule | [optional]
**pre_trade_schedule_paid_minutes** | Option<**i32**> | The paid minutes on the week schedule for this user prior to the shift trade | [optional]
**post_trade_schedule_paid_minutes** | Option<**i32**> | The paid minutes on the week schedule for this user if the shift trade is approved | [optional]
**post_trade_new_shift** | Option<[**crate::models::ShiftTradePreviewResponse**](ShiftTradePreviewResponse.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


