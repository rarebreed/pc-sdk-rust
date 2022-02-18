# ActivityCode

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]
**name** | Option<**String**> | The name of the activity code. Default activity codes will be created with an empty name | [optional]
**is_active** | Option<**bool**> | Whether this activity code is active or has been deleted | [optional]
**is_default** | Option<**bool**> | Whether this is a default activity code | [optional]
**category** | Option<**String**> | The activity code's category. | [optional]
**length_in_minutes** | Option<**i32**> | The default length of the activity in minutes | [optional]
**counts_as_paid_time** | Option<**bool**> | Whether an agent is paid while performing this activity | [optional]
**counts_as_work_time** | Option<**bool**> | Indicates whether or not the activity should be counted as contiguous work time for calculating daily constraints | [optional]
**agent_time_off_selectable** | Option<**bool**> | Whether an agent can select this activity code when creating or editing a time off request. Null if the activity's category is not time off. | [optional]
**metadata** | [**crate::models::WfmVersionedEntityMetadata**](WfmVersionedEntityMetadata.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


