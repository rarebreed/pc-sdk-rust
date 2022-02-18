# WeekSchedule

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]
**week_date** | Option<**String**> | First day of this week schedule in yyyy-MM-dd format | [optional]
**description** | Option<**String**> | Description of the week schedule | [optional]
**published** | Option<**bool**> | Whether the week schedule is published | [optional]
**generation_results** | Option<[**crate::models::WeekScheduleGenerationResult**](WeekScheduleGenerationResult.md)> |  | [optional]
**short_term_forecast** | Option<[**crate::models::ShortTermForecastReference**](ShortTermForecastReference.md)> |  | [optional]
**metadata** | Option<[**crate::models::WfmVersionedEntityMetadata**](WfmVersionedEntityMetadata.md)> |  | [optional]
**user_schedules** | Option<[**::std::collections::HashMap<String, crate::models::UserSchedule>**](UserSchedule.md)> | User schedules in the week | [optional]
**headcount_forecast** | Option<[**crate::models::HeadcountForecast**](HeadcountForecast.md)> |  | [optional]
**agent_schedules_version** | Option<**i32**> | Version of agent schedules in the week schedule | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


