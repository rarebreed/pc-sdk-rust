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
pub struct PunctualityEvent {
    /// The scheduled activity start time. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "dateScheduleStart", skip_serializing_if = "Option::is_none")]
    pub date_schedule_start: Option<String>,
    /// The time the user started the activity. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "dateStart", skip_serializing_if = "Option::is_none")]
    pub date_start: Option<String>,
    /// The length of the activity in minutes
    #[serde(rename = "lengthMinutes", skip_serializing_if = "Option::is_none")]
    pub length_minutes: Option<i32>,
    /// The description of the activity
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The ID of the activity code associated with this activity
    #[serde(rename = "activityCodeId", skip_serializing_if = "Option::is_none")]
    pub activity_code_id: Option<String>,
    /// The activity code
    #[serde(rename = "activityCode", skip_serializing_if = "Option::is_none")]
    pub activity_code: Option<String>,
    /// The category for the activity
    #[serde(rename = "category", skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// The points earned for this activity
    #[serde(rename = "points", skip_serializing_if = "Option::is_none")]
    pub points: Option<i32>,
    /// Difference between this activity and the last activity in seconds
    #[serde(rename = "delta", skip_serializing_if = "Option::is_none")]
    pub delta: Option<f64>,
    #[serde(rename = "bullseye", skip_serializing_if = "Option::is_none")]
    pub bullseye: Option<bool>,
}

impl PunctualityEvent {
    pub fn new() -> PunctualityEvent {
        PunctualityEvent {
            date_schedule_start: None,
            date_start: None,
            length_minutes: None,
            description: None,
            activity_code_id: None,
            activity_code: None,
            category: None,
            points: None,
            delta: None,
            bullseye: None,
        }
    }
}


