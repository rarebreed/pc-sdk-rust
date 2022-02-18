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
pub struct BuAgentScheduleShift {
    /// The ID of the shift
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The start date of this shift. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "startDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// The length of this shift in minutes
    #[serde(rename = "lengthMinutes", skip_serializing_if = "Option::is_none")]
    pub length_minutes: Option<i32>,
    /// The activities associated with this shift
    #[serde(rename = "activities", skip_serializing_if = "Option::is_none")]
    pub activities: Option<Vec<crate::models::BuAgentScheduleActivity>>,
    /// Whether this shift was manually edited. This is only set by clients and is used for rescheduling
    #[serde(rename = "manuallyEdited", skip_serializing_if = "Option::is_none")]
    pub manually_edited: Option<bool>,
    #[serde(rename = "schedule", skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Box<crate::models::BuScheduleReference>>,
}

impl BuAgentScheduleShift {
    pub fn new() -> BuAgentScheduleShift {
        BuAgentScheduleShift {
            id: None,
            start_date: None,
            length_minutes: None,
            activities: None,
            manually_edited: None,
            schedule: None,
        }
    }
}

