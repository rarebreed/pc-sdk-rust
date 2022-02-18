# UserSchedule

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**shifts** | Option<[**Vec<crate::models::UserScheduleShift>**](UserScheduleShift.md)> | The shifts that belong to this schedule | [optional]
**full_day_time_off_markers** | Option<[**Vec<crate::models::UserScheduleFullDayTimeOffMarker>**](UserScheduleFullDayTimeOffMarker.md)> | Markers to indicate a full day time off request, relative to the management unit time zone | [optional]
**delete** | Option<**bool**> | If marked true for updating an existing user schedule, it will be deleted | [optional]
**metadata** | [**crate::models::WfmVersionedEntityMetadata**](WfmVersionedEntityMetadata.md) |  | 
**work_plan_id** | Option<**String**> | ID of the work plan associated with the user during schedule creation | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


