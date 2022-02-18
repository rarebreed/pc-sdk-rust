# TimeOffPlan

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> | The name of this time off plan. | [optional]
**activity_code_ids** | Option<**Vec<String>**> | The set of activity code IDs associated with this time off plan. | [optional]
**time_off_limits** | Option<[**Vec<crate::models::TimeOffLimitReference>**](TimeOffLimitReference.md)> | The set of time off limit IDs associated with this time off plan. | [optional]
**auto_approval_rule** | Option<**String**> | Auto approval rule for this time off plan | [optional]
**days_before_start_to_expire_from_waitlist** | Option<**i32**> | The number of days before the time off request start date for when the request will be expired from the waitlist. | [optional]
**active** | Option<**bool**> | Whether this time off plan is currently being used by agents. | [optional]
**metadata** | Option<[**crate::models::WfmVersionedEntityMetadata**](WfmVersionedEntityMetadata.md)> |  | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


