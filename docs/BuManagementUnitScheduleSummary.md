# BuManagementUnitScheduleSummary

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**management_unit** | Option<[**crate::models::ManagementUnitReference**](ManagementUnitReference.md)> |  | [optional]
**agent_count** | Option<**i32**> | The number of agents from this management unit that are in the schedule | [optional]
**start_date** | Option<**String**> | The start of the schedule change in the management unit. Only populated in schedule update notifications. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**end_date** | Option<**String**> | The end of the schedule change in the management unit. Only populated in schedule update notifications. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**agents** | Option<[**Vec<crate::models::UserReference>**](UserReference.md)> | The agents in the management unit who are part of this schedule, or in schedule change notifications, the agents that were changed. Note this will come back as an empty list unless the appropriate expand query parameter is passed | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


