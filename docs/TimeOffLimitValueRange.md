# TimeOffLimitValueRange

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**time_off_limit** | Option<[**crate::models::TimeOffLimitReference**](TimeOffLimitReference.md)> |  | [optional]
**start_date** | [**String**](string.md) | Start date of the requested date range, in ISO-8601 format. The end date is determined by the size of interval lists | 
**granularity** | **String** | Granularity choice for time off limit | 
**limit_minutes_per_interval** | Option<**Vec<i32>**> | A list of time off limit values in minutes per granularity interval | [optional]
**allocated_minutes_per_interval** | Option<**Vec<i32>**> | A list of allocated time off minutes per granularity interval | [optional]
**waitlisted_minutes_per_interval** | Option<**Vec<i32>**> | A list of waitlisted time off minutes per granularity interval | [optional]
**waitlisted_requests_per_interval** | Option<**Vec<i32>**> | The current number of waitlisted time off requests for every interval per granularity | [optional]
**metadata** | Option<[**crate::models::WfmVersionedEntityMetadata**](WfmVersionedEntityMetadata.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


