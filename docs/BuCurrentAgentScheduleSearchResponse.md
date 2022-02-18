# BuCurrentAgentScheduleSearchResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**agent_schedules** | Option<[**Vec<crate::models::BuAgentScheduleSearchResponse>**](BuAgentScheduleSearchResponse.md)> | The requested agent schedules | [optional]
**business_unit_time_zone** | Option<**String**> | The time zone configured for the business unit to which this schedule applies | [optional]
**published_schedules** | Option<[**Vec<crate::models::BuAgentSchedulePublishedScheduleReference>**](BuAgentSchedulePublishedScheduleReference.md)> | References to all published week schedules overlapping the start/end date query parameters | [optional]
**start_date** | Option<**String**> | The start date of the schedules. Only populated on notifications. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**end_date** | Option<**String**> | The end date of the schedules. Only populated on notifications. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**updates** | Option<[**Vec<crate::models::BuAgentScheduleUpdate>**](BuAgentScheduleUpdate.md)> | The list of updates for the schedule. Only used in notifications | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


