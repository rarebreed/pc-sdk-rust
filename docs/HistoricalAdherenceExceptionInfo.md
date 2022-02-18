# HistoricalAdherenceExceptionInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**start_offset_seconds** | Option<**i32**> | Exception start offset in seconds relative to query start time | [optional]
**end_offset_seconds** | Option<**i32**> | Exception end offset in seconds relative to query start time | [optional]
**scheduled_activity_code_id** | Option<**String**> | The ID of the scheduled activity code for this user | [optional]
**scheduled_activity_category** | Option<**String**> | Activity for which the user is scheduled | [optional]
**actual_activity_category** | Option<**String**> | Activity for which the user is actually engaged | [optional]
**system_presence** | Option<**String**> | Actual underlying system presence value | [optional]
**routing_status** | Option<[**crate::models::RoutingStatus**](RoutingStatus.md)> |  | [optional]
**impact** | Option<**String**> | The impact of the current adherence state for this user | [optional]
**secondary_presence_lookup_id** | Option<**String**> | The lookup ID used to retrieve the actual secondary status from map of lookup ID to corresponding secondary presence ID | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


