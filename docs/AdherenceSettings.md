# AdherenceSettings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**severe_alert_threshold_minutes** | Option<**i32**> | The threshold in minutes where an alert will be triggered when an agent is considered severely out of adherence | [optional]
**adherence_target_percent** | Option<**i32**> | Target adherence percentage | [optional]
**adherence_exception_threshold_seconds** | Option<**i32**> | The threshold in seconds for which agents should not be penalized for being momentarily out of adherence | [optional]
**non_on_queue_activities_equivalent** | Option<**bool**> | Whether to treat all non-on-queue activities as equivalent for adherence purposes | [optional]
**track_on_queue_activity** | Option<**bool**> | Whether to track on-queue activities | [optional]
**ignored_activity_categories** | Option<[**crate::models::IgnoredActivityCategories**](IgnoredActivityCategories.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


