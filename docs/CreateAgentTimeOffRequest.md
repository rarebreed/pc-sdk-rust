# CreateAgentTimeOffRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**activity_code_id** | **String** | The ID of the activity code associated with this time off request. Activity code must be of the TimeOff category | 
**notes** | Option<**String**> | Notes about the time off request | [optional]
**full_day_management_unit_dates** | Option<**Vec<String>**> | A set of dates in yyyy-MM-dd format.  Should be interpreted in the management unit's configured time zone. | [optional]
**partial_day_start_date_times** | Option<**Vec<String>**> | A set of start date-times in ISO-8601 format for partial day requests. | [optional]
**daily_duration_minutes** | **i32** | The daily duration of this time off request in minutes | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


