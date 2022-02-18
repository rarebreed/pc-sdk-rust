/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// UserScheduleActivity : Represents a single activity in a user's shift



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UserScheduleActivity {
    /// The id for the activity code.  Look up a map of activity codes with the activities route
    #[serde(rename = "activityCodeId", skip_serializing_if = "Option::is_none")]
    pub activity_code_id: Option<String>,
    /// Start time in UTC for this activity, in ISO-8601 format
    #[serde(rename = "startDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// Length in minutes for this activity
    #[serde(rename = "lengthInMinutes", skip_serializing_if = "Option::is_none")]
    pub length_in_minutes: Option<i32>,
    /// Description for this activity
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Whether this activity is paid
    #[serde(rename = "countsAsPaidTime", skip_serializing_if = "Option::is_none")]
    pub counts_as_paid_time: Option<bool>,
    /// Whether this activity spans a DST fallback
    #[serde(rename = "isDstFallback", skip_serializing_if = "Option::is_none")]
    pub is_dst_fallback: Option<bool>,
    /// Time off request id of this activity
    #[serde(rename = "timeOffRequestId", skip_serializing_if = "Option::is_none")]
    pub time_off_request_id: Option<String>,
}

impl UserScheduleActivity {
    /// Represents a single activity in a user's shift
    pub fn new() -> UserScheduleActivity {
        UserScheduleActivity {
            activity_code_id: None,
            start_date: None,
            length_in_minutes: None,
            description: None,
            counts_as_paid_time: None,
            is_dst_fallback: None,
            time_off_request_id: None,
        }
    }
}

