# TimeOffRequestNotification

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**time_off_request_id** | Option<**String**> | The ID of this time off request | [optional]
**user** | Option<[**crate::models::UserReference**](UserReference.md)> |  | [optional]
**is_full_day_request** | Option<**bool**> | Whether this is a full day request (false means partial day) | [optional]
**status** | Option<**String**> | The status of this time off request | [optional]
**partial_day_start_date_times** | Option<**Vec<String>**> | A set of start date-times in ISO-8601 format for partial day requests.  Will be not empty if isFullDayRequest == false | [optional]
**full_day_management_unit_dates** | Option<**Vec<String>**> | A set of dates in yyyy-MM-dd format.  Should be interpreted in the management unit's configured time zone.  Will be not empty if isFullDayRequest == true | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


