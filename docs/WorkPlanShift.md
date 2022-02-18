# WorkPlanShift

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Name of the shift | 
**days** | Option<[**crate::models::SetWrapperDayOfWeek**](SetWrapperDayOfWeek.md)> |  | [optional]
**flexible_start_time** | Option<**bool**> | Whether the start time of the shift is flexible | [optional]
**exact_start_time_minutes_from_midnight** | Option<**i32**> | Exact start time of the shift defined as offset minutes from midnight. Used if flexibleStartTime == false | [optional]
**earliest_start_time_minutes_from_midnight** | Option<**i32**> | Earliest start time of the shift defined as offset minutes from midnight. Used if flexibleStartTime == true | [optional]
**latest_start_time_minutes_from_midnight** | Option<**i32**> | Latest start time of the shift defined as offset minutes from midnight. Used if flexibleStartTime == true | [optional]
**constrain_stop_time** | Option<**bool**> | Whether the latest stop time constraint for the shift is enabled.  Deprecated, use constrainLatestStopTime instead | [optional]
**constrain_latest_stop_time** | Option<**bool**> | Whether the latest stop time constraint for the shift is enabled | [optional]
**latest_stop_time_minutes_from_midnight** | Option<**i32**> | Latest stop time of the shift defined as offset minutes from midnight. Used if constrainStopTime == true | [optional]
**constrain_earliest_stop_time** | Option<**bool**> | Whether the earliest stop time constraint for the shift is enabled | [optional]
**earliest_stop_time_minutes_from_midnight** | Option<**i32**> | This is the earliest time a shift can end | [optional]
**start_increment_minutes** | Option<**i32**> | Increment in offset minutes that would contribute to different possible start times for the shift. Used if flexibleStartTime == true | [optional]
**flexible_paid_time** | Option<**bool**> | Whether the paid time setting for the shift is flexible | [optional]
**exact_paid_time_minutes** | Option<**i32**> | Exact paid time in minutes configured for the shift. Used if flexiblePaidTime == false | [optional]
**minimum_paid_time_minutes** | Option<**i32**> | Minimum paid time in minutes configured for the shift. Used if flexiblePaidTime == true | [optional]
**maximum_paid_time_minutes** | Option<**i32**> | Maximum paid time in minutes configured for the shift. Used if flexiblePaidTime == true | [optional]
**constrain_contiguous_work_time** | Option<**bool**> | Whether the contiguous time constraint for the shift is enabled | [optional]
**minimum_contiguous_work_time_minutes** | Option<**i32**> | Minimum contiguous time in minutes configured for the shift. Used if constrainContiguousWorkTime == true | [optional]
**maximum_contiguous_work_time_minutes** | Option<**i32**> | Maximum contiguous time in minutes configured for the shift. Used if constrainContiguousWorkTime == true | [optional]
**activities** | Option<[**Vec<crate::models::WorkPlanActivity>**](WorkPlanActivity.md)> | Activities configured for this shift | [optional]
**id** | Option<**String**> | ID of the shift. This is required only for the case of updating an existing shift | [optional]
**delete** | Option<**bool**> | If marked true for updating an existing shift, the shift will be permanently deleted | [optional]
**validation_id** | Option<**String**> | ID of shift in the context of work plan validation | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


