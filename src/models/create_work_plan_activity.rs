/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// CreateWorkPlanActivity : Activity configured for shift in work plan



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateWorkPlanActivity {
    /// ID of the activity code associated with this activity
    #[serde(rename = "activityCodeId", skip_serializing_if = "Option::is_none")]
    pub activity_code_id: Option<String>,
    /// Description of the activity
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Length of the activity in minutes
    #[serde(rename = "lengthMinutes", skip_serializing_if = "Option::is_none")]
    pub length_minutes: Option<i32>,
    /// Whether the start time of the activity is relative to the start time of the shift it belongs to
    #[serde(rename = "startTimeIsRelativeToShiftStart", skip_serializing_if = "Option::is_none")]
    pub start_time_is_relative_to_shift_start: Option<bool>,
    /// Whether the start time of the activity is flexible
    #[serde(rename = "flexibleStartTime", skip_serializing_if = "Option::is_none")]
    pub flexible_start_time: Option<bool>,
    /// Earliest activity start in offset minutes relative to shift start time if startTimeIsRelativeToShiftStart == true else its based on midnight. Used if flexibleStartTime == true
    #[serde(rename = "earliestStartTimeMinutes", skip_serializing_if = "Option::is_none")]
    pub earliest_start_time_minutes: Option<i32>,
    /// Latest activity start in offset minutes relative to shift start time if startTimeIsRelativeToShiftStart == true else its based on midnight. Used if flexibleStartTime == true
    #[serde(rename = "latestStartTimeMinutes", skip_serializing_if = "Option::is_none")]
    pub latest_start_time_minutes: Option<i32>,
    /// Exact activity start in offset minutes relative to shift start time if startTimeIsRelativeToShiftStart == true else its based on midnight. Used if flexibleStartTime == false
    #[serde(rename = "exactStartTimeMinutes", skip_serializing_if = "Option::is_none")]
    pub exact_start_time_minutes: Option<i32>,
    /// Increment in offset minutes that would contribute to different possible start times for the activity
    #[serde(rename = "startTimeIncrementMinutes", skip_serializing_if = "Option::is_none")]
    pub start_time_increment_minutes: Option<i32>,
    /// Whether the activity is paid
    #[serde(rename = "countsAsPaidTime", skip_serializing_if = "Option::is_none")]
    pub counts_as_paid_time: Option<bool>,
    /// Whether the activity duration is counted towards contiguous work time
    #[serde(rename = "countsAsContiguousWorkTime", skip_serializing_if = "Option::is_none")]
    pub counts_as_contiguous_work_time: Option<bool>,
    /// The minimum duration between shift start and shift item (e.g., break or meal) start in minutes
    #[serde(rename = "minimumLengthFromShiftStartMinutes", skip_serializing_if = "Option::is_none")]
    pub minimum_length_from_shift_start_minutes: Option<i32>,
    /// The minimum duration between shift item (e.g., break or meal) end and shift end in minutes
    #[serde(rename = "minimumLengthFromShiftEndMinutes", skip_serializing_if = "Option::is_none")]
    pub minimum_length_from_shift_end_minutes: Option<i32>,
}

impl CreateWorkPlanActivity {
    /// Activity configured for shift in work plan
    pub fn new() -> CreateWorkPlanActivity {
        CreateWorkPlanActivity {
            activity_code_id: None,
            description: None,
            length_minutes: None,
            start_time_is_relative_to_shift_start: None,
            flexible_start_time: None,
            earliest_start_time_minutes: None,
            latest_start_time_minutes: None,
            exact_start_time_minutes: None,
            start_time_increment_minutes: None,
            counts_as_paid_time: None,
            counts_as_contiguous_work_time: None,
            minimum_length_from_shift_start_minutes: None,
            minimum_length_from_shift_end_minutes: None,
        }
    }
}


