# QueryTimeOffLimitValuesRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**time_off_limit_id** | Option<**String**> | The time off limit object id to retrieve values for. Required if activityCodeId is not specified | [optional]
**activity_code_id** | Option<**String**> | The activity code id to filter the affected limit objects by. Required if timeOffLimitId is not specified | [optional]
**date_ranges** | [**Vec<crate::models::LocalDateRange>**](LocalDateRange.md) | The list of the date ranges to return time off limit, allocated and waitlisted minutes. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


