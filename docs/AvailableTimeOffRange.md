# AvailableTimeOffRange

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**time_off_limit** | Option<[**crate::models::TimeOffLimitReference**](TimeOffLimitReference.md)> |  | [optional]
**start_date** | Option<[**String**](string.md)> | Start date of the requested date range. The end date is determined by the size of interval list. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [optional]
**granularity** | Option<**String**> | Granularity choice for time off limit | [optional]
**available_minutes_per_interval** | Option<**Vec<i32>**> | The list of available time off values in minutes per granularity interval | [optional]
**waitlisted_requests_per_interval** | Option<**Vec<i32>**> | The current number of waitlisted time off requests for every interval per granularity | [optional]
**waitlist_enabled** | Option<**bool**> | Whether the time off request can be waitlisted | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


