# UpdateActivityCodeRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the activity code | [optional]
**category** | Option<**String**> | The activity code's category. Attempting to change the category of a default activity code will return an error | [optional]
**length_in_minutes** | Option<**i32**> | The default length of the activity in minutes | [optional]
**counts_as_paid_time** | Option<**bool**> | Whether an agent is paid while performing this activity | [optional]
**counts_as_work_time** | Option<**bool**> | Indicates whether or not the activity should be counted as work time | [optional]
**agent_time_off_selectable** | Option<**bool**> | Whether an agent can select this activity code when creating or editing a time off request | [optional]
**metadata** | [**crate::models::WfmVersionedEntityMetadata**](WfmVersionedEntityMetadata.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


