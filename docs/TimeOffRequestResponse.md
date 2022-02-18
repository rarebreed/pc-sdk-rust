# TimeOffRequestResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**user** | Option<[**crate::models::UserReference**](UserReference.md)> |  | [optional]
**is_full_day_request** | Option<**bool**> | Whether this is a full day request (false means partial day) | [optional]
**marked_as_read** | Option<**bool**> | Whether this request has been marked as read by the agent | [optional]
**activity_code_id** | Option<**String**> | The ID of the activity code associated with this time off request. Activity code must be of the TimeOff category | [optional]
**status** | Option<**String**> | The status of this time off request | [optional]
**partial_day_start_date_times** | Option<**Vec<String>**> | A set of start date-times in ISO-8601 format for partial day requests.  Will be not empty if isFullDayRequest == false | [optional]
**full_day_management_unit_dates** | Option<**Vec<String>**> | A set of dates in yyyy-MM-dd format.  Should be interpreted in the management unit's configured time zone.  Will be not empty if isFullDayRequest == true | [optional]
**daily_duration_minutes** | Option<**i32**> | The daily duration of this time off request in minutes | [optional]
**notes** | Option<**String**> | Notes about the time off request | [optional]
**submitted_by** | Option<[**crate::models::UserReference**](UserReference.md)> |  | [optional]
**submitted_date** | Option<**String**> | The timestamp when this request was submitted. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**reviewed_by** | Option<[**crate::models::UserReference**](UserReference.md)> |  | [optional]
**reviewed_date** | Option<**String**> | The timestamp when this request was reviewed. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**modified_by** | Option<[**crate::models::UserReference**](UserReference.md)> |  | [optional]
**modified_date** | Option<**String**> | The timestamp when this request was last modified. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**metadata** | Option<[**crate::models::WfmVersionedEntityMetadata**](WfmVersionedEntityMetadata.md)> |  | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


