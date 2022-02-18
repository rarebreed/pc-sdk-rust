# TimeOffLimitRange

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**start_date** | [**String**](string.md) | Start date of the range. The end date is determined by 'granularity' and the size of 'limitMinutesPerInterval'. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | 
**granularity** | **String** | Granularity choice for the time off limit | 
**limit_minutes_per_interval** | **Vec<i32>** | The list of time off limit values in minutes per granularity interval. If 'null' is specified, then interval specific value is cleared. Such interval will have 'defaultLimitMinutes' value | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


