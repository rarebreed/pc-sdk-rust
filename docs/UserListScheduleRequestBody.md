# UserListScheduleRequestBody

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user_ids** | **Vec<String>** | The user ids for which to fetch schedules | 
**start_date** | **String** | Beginning of the range of schedules to fetch, in ISO-8601 format | 
**end_date** | **String** | End of the range of schedules to fetch, in ISO-8601 format | 
**load_full_weeks** | Option<**bool**> | Whether to load the full week's schedule (for the requested users) of any week overlapping the start/end date query parameters, defaults to false | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


