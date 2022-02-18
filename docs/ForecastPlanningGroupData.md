# ForecastPlanningGroupData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**planning_group_id** | **String** | The ID of the planning group to which this data applies. Note this is a snapshot of the planning group at the time of forecast creation and may not correspond to the current configuration | 
**offered_per_interval** | **Vec<f64>** | Forecast offered counts per 15 minute interval for this week of the forecast | 
**average_handle_time_seconds_per_interval** | **Vec<f64>** | Forecast average handle time per 15 minute interval in seconds | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


