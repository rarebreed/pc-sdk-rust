# BuIntradayResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**start_date** | Option<**String**> | The start of the date range for which this data applies.  This is also the start reference point for the intervals represented in the various arrays. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**end_date** | Option<**String**> | The end of the date range for which this data applies. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**interval_length_minutes** | Option<**i32**> | The aggregation period in minutes, which determines the interval duration of the returned data | [optional]
**no_data_reason** | Option<**String**> | If not null, the reason there was no data for the request | [optional]
**categories** | Option<**Vec<String>**> | The categories to which this data corresponds | [optional]
**short_term_forecast** | Option<[**crate::models::BuShortTermForecastReference**](BuShortTermForecastReference.md)> |  | [optional]
**schedule** | Option<[**crate::models::BuScheduleReference**](BuScheduleReference.md)> |  | [optional]
**intraday_data_groupings** | Option<[**Vec<crate::models::BuIntradayDataGroup>**](BuIntradayDataGroup.md)> | Intraday data grouped by a single media type and set of planning group IDs | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


