# BuRescheduleResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**generation_results** | Option<[**crate::models::ScheduleGenerationResult**](ScheduleGenerationResult.md)> |  | [optional]
**generation_results_download_url** | Option<**String**> | The download URL from which to fetch the generation results for the rescheduling run | [optional]
**headcount_forecast** | Option<[**crate::models::BuHeadcountForecast**](BuHeadcountForecast.md)> |  | [optional]
**headcount_forecast_download_url** | Option<**String**> | The download URL from which to fetch the headcount forecast for the rescheduling run | [optional]
**agent_schedules** | Option<[**Vec<crate::models::BuRescheduleAgentScheduleResult>**](BuRescheduleAgentScheduleResult.md)> | List of download links for agent schedules produced by the rescheduling run | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


