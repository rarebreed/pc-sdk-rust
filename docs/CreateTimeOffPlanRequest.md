# CreateTimeOffPlanRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name of this time off plan. | 
**activity_code_ids** | Option<**Vec<String>**> | The set of activity code IDs to associate with this time off plan. | [optional]
**time_off_limit_ids** | Option<**Vec<String>**> | The set of time off limit IDs to associate with this time off plan. | [optional]
**auto_approval_rule** | **String** | Auto approval rule for the time off plan. | 
**days_before_start_to_expire_from_waitlist** | Option<**i32**> | The number of days before the time off request start date for when the request will be expired from the waitlist. | [optional]
**active** | **bool** | Whether this time off plan should be used by agents. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


