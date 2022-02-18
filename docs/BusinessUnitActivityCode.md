# BusinessUnitActivityCode

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**active** | Option<**bool**> | Whether this activity code is active or has been deleted | [optional]
**default_code** | Option<**bool**> | Whether this is a default activity code | [optional]
**category** | Option<**String**> | The category of the activity code | [optional]
**length_in_minutes** | Option<**i32**> | The default length of the activity in minutes | [optional]
**counts_as_paid_time** | Option<**bool**> | Whether an agent is paid while performing this activity | [optional]
**counts_as_work_time** | Option<**bool**> | Indicates whether or not the activity should be counted as contiguous work time for calculating daily constraints | [optional]
**agent_time_off_selectable** | Option<**bool**> | Whether an agent can select this activity code when creating or editing a time off request. Null if the activity's category is not time off. | [optional]
**metadata** | Option<[**crate::models::WfmVersionedEntityMetadata**](WfmVersionedEntityMetadata.md)> |  | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


