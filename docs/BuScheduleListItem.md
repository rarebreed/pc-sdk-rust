# BuScheduleListItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**week_date** | Option<[**String**](string.md)> | The start week date for this schedule. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [optional]
**week_count** | Option<**i32**> | The number of weeks spanned by this schedule | [optional]
**description** | Option<**String**> | The description of this schedule | [optional]
**published** | Option<**bool**> | Whether this schedule is published | [optional]
**short_term_forecast** | Option<[**crate::models::BuShortTermForecastReference**](BuShortTermForecastReference.md)> |  | [optional]
**generation_results** | Option<[**crate::models::ScheduleGenerationResultSummary**](ScheduleGenerationResultSummary.md)> |  | [optional]
**metadata** | Option<[**crate::models::WfmVersionedEntityMetadata**](WfmVersionedEntityMetadata.md)> |  | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


