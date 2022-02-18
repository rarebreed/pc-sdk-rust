# WaitlistPosition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**time_off_request** | Option<[**crate::models::TimeOffRequestReference**](TimeOffRequestReference.md)> |  | [optional]
**time_off_limit** | Option<[**crate::models::TimeOffLimitReference**](TimeOffLimitReference.md)> |  | [optional]
**date** | Option<[**String**](string.md)> | The date to which this wait list position applies, as defined by the time zone of the business unit. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [optional]
**waitlist_position** | Option<**i32**> | The time off request's position in the waitlist on the date. 1 means time off is the first in the waitlist | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


