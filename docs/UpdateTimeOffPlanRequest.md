# UpdateTimeOffPlanRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of this time off plan. | [optional]
**activity_code_ids** | Option<[**crate::models::SetWrapperString**](SetWrapperString.md)> |  | [optional]
**time_off_limit_ids** | Option<[**crate::models::SetWrapperString**](SetWrapperString.md)> |  | [optional]
**auto_approval_rule** | Option<**String**> | Auto approval rule for the time off plan. | [optional]
**days_before_start_to_expire_from_waitlist** | Option<**i32**> | The number of days before the time off request start date for when the request will be expired from the waitlist. | [optional]
**active** | Option<**bool**> | Whether this time off plan should be used by agents. | [optional]
**metadata** | [**crate::models::WfmVersionedEntityMetadata**](WfmVersionedEntityMetadata.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


