# ShiftTradeMatchReviewResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**initiating_user** | Option<[**crate::models::ShiftTradeMatchReviewUserResponse**](ShiftTradeMatchReviewUserResponse.md)> |  | [optional]
**receiving_user** | Option<[**crate::models::ShiftTradeMatchReviewUserResponse**](ShiftTradeMatchReviewUserResponse.md)> |  | [optional]
**violations** | Option<[**Vec<crate::models::ShiftTradeMatchViolation>**](ShiftTradeMatchViolation.md)> | Constraint violations introduced after being matched that would normally disallow a trade, but which can still be overridden by the shift trade administrator | [optional]
**admin_review_violations** | Option<[**Vec<crate::models::ShiftTradeMatchViolation>**](ShiftTradeMatchViolation.md)> | Constraint violations associated with this shift trade which require shift trade administrator review | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


