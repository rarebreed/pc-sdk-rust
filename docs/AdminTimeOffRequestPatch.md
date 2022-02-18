# AdminTimeOffRequestPatch

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | Option<**String**> | The status of this time off request | [optional]
**activity_code_id** | Option<**String**> | The ID of the activity code associated with this time off request. Activity code must be of the TimeOff category | [optional]
**notes** | Option<**String**> | Notes about the time off request | [optional]
**full_day_management_unit_dates** | Option<**Vec<String>**> | A set of dates in yyyy-MM-dd format.  Should be interpreted in the management unit's configured time zone. | [optional]
**partial_day_start_date_times** | Option<**Vec<String>**> | A set of start date-times in ISO-8601 format for partial day requests. | [optional]
**daily_duration_minutes** | Option<**i32**> | The daily duration of this time off request in minutes | [optional]
**metadata** | [**crate::models::WfmVersionedEntityMetadata**](WfmVersionedEntityMetadata.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


