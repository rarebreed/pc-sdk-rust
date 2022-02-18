# WorkPlanValidationRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**enabled** | Option<**bool**> | Whether the work plan is enabled for scheduling | [optional]
**valid** | Option<**bool**> | Whether the work plan is valid or not | [optional][readonly]
**constrain_weekly_paid_time** | Option<**bool**> | Whether the weekly paid time constraint is enabled for this work plan | [optional]
**flexible_weekly_paid_time** | Option<**bool**> | Whether the weekly paid time constraint is flexible for this work plan | [optional]
**weekly_exact_paid_minutes** | Option<**i32**> | Exact weekly paid time in minutes for this work plan. Used if flexibleWeeklyPaidTime == false | [optional]
**weekly_minimum_paid_minutes** | Option<**i32**> | Minimum weekly paid time in minutes for this work plan. Used if flexibleWeeklyPaidTime == true | [optional]
**weekly_maximum_paid_minutes** | Option<**i32**> | Maximum weekly paid time in minutes for this work plan. Used if flexibleWeeklyPaidTime == true | [optional]
**constrain_paid_time_granularity** | Option<**bool**> | Whether paid time granularity is constrained for this work plan | [optional]
**paid_time_granularity_minutes** | Option<**i32**> | Granularity in minutes allowed for shift paid time in this work plan. Used if constrainPaidTimeGranularity == true | [optional]
**constrain_minimum_time_between_shifts** | Option<**bool**> | Whether the minimum time between shifts constraint is enabled for this work plan | [optional]
**minimum_time_between_shifts_minutes** | Option<**i32**> | Minimum time between shifts in minutes defined in this work plan. Used if constrainMinimumTimeBetweenShifts == true | [optional]
**maximum_days** | Option<**i32**> | Maximum number days in a week allowed to be scheduled for this work plan | [optional]
**minimum_consecutive_non_working_minutes_per_week** | Option<**i32**> | Minimum amount of consecutive non working minutes per week that agents who are assigned this work plan are allowed to have off | [optional]
**constrain_maximum_consecutive_working_weekends** | Option<**bool**> | Whether to constrain the maximum consecutive working weekends | [optional]
**maximum_consecutive_working_weekends** | Option<**i32**> | The maximum number of consecutive weekends that agents who are assigned to this work plan are allowed to work | [optional]
**minimum_working_days_per_week** | Option<**i32**> | The minimum number of days that agents assigned to a work plan must work per week | [optional]
**constrain_maximum_consecutive_working_days** | Option<**bool**> | Whether to constrain the maximum consecutive working days | [optional]
**maximum_consecutive_working_days** | Option<**i32**> | The maximum number of consecutive days that agents assigned to this work plan are allowed to work. Used if constrainMaximumConsecutiveWorkingDays == true | [optional]
**minimum_shift_start_distance_minutes** | Option<**i32**> | The time period in minutes for the duration between the start times of two consecutive working days | [optional]
**minimum_days_off_per_planning_period** | Option<**i32**> | Minimum days off in the planning period | [optional]
**maximum_days_off_per_planning_period** | Option<**i32**> | Maximum days off in the planning period | [optional]
**minimum_paid_minutes_per_planning_period** | Option<**i32**> | Minimum paid minutes in the planning period | [optional]
**maximum_paid_minutes_per_planning_period** | Option<**i32**> | Maximum paid minutes in the planning period | [optional]
**optional_days** | Option<[**crate::models::SetWrapperDayOfWeek**](SetWrapperDayOfWeek.md)> |  | [optional]
**shift_start_variance_type** | Option<**String**> | This constraint ensures that an agent starts each workday within a user-defined time threshold | [optional]
**shift_start_variances** | Option<[**crate::models::ListWrapperShiftStartVariance**](ListWrapperShiftStartVariance.md)> |  | [optional]
**shifts** | Option<[**Vec<crate::models::WorkPlanShift>**](WorkPlanShift.md)> | Shifts in this work plan | [optional]
**agents** | Option<[**Vec<crate::models::DeletableUserReference>**](DeletableUserReference.md)> | Agents in this work plan | [optional]
**agent_count** | Option<**i32**> | Number of agents in this work plan | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


