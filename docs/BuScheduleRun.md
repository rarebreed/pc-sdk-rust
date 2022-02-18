# BuScheduleRun

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**scheduler_run_id** | Option<**String**> | The scheduler run ID.  Reference this value for support | [optional]
**intraday_rescheduling** | Option<**bool**> | Whether this is an intraday rescheduling run | [optional]
**state** | Option<**String**> | The state of the generation run | [optional]
**week_count** | Option<**i32**> | The number of weeks spanned by the schedule | [optional]
**percent_complete** | Option<**f64**> | Percent completion of the schedule run | [optional]
**target_week** | Option<[**String**](string.md)> | The start date of the target week. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [optional]
**schedule** | Option<[**crate::models::BuScheduleReference**](BuScheduleReference.md)> |  | [optional]
**schedule_description** | Option<**String**> | The description of the generated schedule | [optional]
**scheduling_start_time** | Option<**String**> | When the schedule generation run started. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**scheduling_started_by** | Option<[**crate::models::UserReference**](UserReference.md)> |  | [optional]
**scheduling_canceled_by** | Option<[**crate::models::UserReference**](UserReference.md)> |  | [optional]
**scheduling_completed_time** | Option<**String**> | When the scheduling run was completed, if applicable. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**message_count** | Option<**i32**> | The number of schedule generation messages for this schedule generation run | [optional]
**message_severity_counts** | Option<[**Vec<crate::models::SchedulerMessageSeverityCount>**](SchedulerMessageSeverityCount.md)> | The list of schedule generation message counts by severity for this schedule generation run | [optional]
**rescheduling_options** | Option<[**crate::models::ReschedulingOptionsRunResponse**](ReschedulingOptionsRunResponse.md)> |  | [optional]
**rescheduling_result_expiration** | Option<**String**> | When the reschedule result will expire.  Null unless intradayRescheduling is true. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


