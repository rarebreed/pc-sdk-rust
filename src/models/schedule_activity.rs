/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ScheduleActivity {
    /// The start date/time of this activity. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "dateStart", skip_serializing_if = "Option::is_none")]
    pub date_start: Option<String>,
    /// The length of this activity in minutes
    #[serde(rename = "lengthMinutes", skip_serializing_if = "Option::is_none")]
    pub length_minutes: Option<i32>,
    /// The description of this activity
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The ID of the activity code associated with this activity
    #[serde(rename = "activityCodeId", skip_serializing_if = "Option::is_none")]
    pub activity_code_id: Option<String>,
    /// Whether this activity is paid
    #[serde(rename = "paid", skip_serializing_if = "Option::is_none")]
    pub paid: Option<bool>,
    /// The ID of the time off request associated with this activity, if applicable
    #[serde(rename = "timeOffRequestId", skip_serializing_if = "Option::is_none")]
    pub time_off_request_id: Option<String>,
    /// The ID of the external activity associated with this activity, if applicable
    #[serde(rename = "externalActivityId", skip_serializing_if = "Option::is_none")]
    pub external_activity_id: Option<String>,
    /// The type of the external activity associated with this activity, if applicable
    #[serde(rename = "externalActivityType", skip_serializing_if = "Option::is_none")]
    pub external_activity_type: Option<ExternalActivityType>,
}

impl ScheduleActivity {
    pub fn new() -> ScheduleActivity {
        ScheduleActivity {
            date_start: None,
            length_minutes: None,
            description: None,
            activity_code_id: None,
            paid: None,
            time_off_request_id: None,
            external_activity_id: None,
            external_activity_type: None,
        }
    }
}

/// The type of the external activity associated with this activity, if applicable
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ExternalActivityType {
    #[serde(rename = "Coaching")]
    Coaching,
}

impl Default for ExternalActivityType {
    fn default() -> ExternalActivityType {
        Self::Coaching
    }
}

