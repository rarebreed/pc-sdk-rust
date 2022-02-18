# BuAgentScheduleQueryResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user** | Option<[**crate::models::UserReference**](UserReference.md)> |  | [optional]
**shifts** | Option<[**Vec<crate::models::BuAgentScheduleShift>**](BuAgentScheduleShift.md)> | The shift definitions for this agent schedule | [optional]
**full_day_time_off_markers** | Option<[**Vec<crate::models::BuFullDayTimeOffMarker>**](BuFullDayTimeOffMarker.md)> | Full day time off markers which apply to this agent schedule | [optional]
**work_plan** | Option<[**crate::models::WorkPlanReference**](WorkPlanReference.md)> |  | [optional]
**work_plans_per_week** | Option<[**Vec<crate::models::WorkPlanReference>**](WorkPlanReference.md)> | The work plans per week for this user from the work plan rotation. Null values in the list denotes that user is not part of any work plan for that week | [optional]
**metadata** | Option<[**crate::models::WfmVersionedEntityMetadata**](WfmVersionedEntityMetadata.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


