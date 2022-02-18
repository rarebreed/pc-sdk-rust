/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// UpdateCoachingAppointmentRequest : Update coaching appointment request



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UpdateCoachingAppointmentRequest {
    /// The name of coaching appointment.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The description of coaching appointment.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The date/time the coaching appointment starts. Times will be rounded down to the minute. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "dateStart", skip_serializing_if = "Option::is_none")]
    pub date_start: Option<String>,
    /// The duration of coaching appointment in minutes.
    #[serde(rename = "lengthInMinutes", skip_serializing_if = "Option::is_none")]
    pub length_in_minutes: Option<i32>,
    /// IDs of conversations associated with this coaching appointment.
    #[serde(rename = "conversationIds", skip_serializing_if = "Option::is_none")]
    pub conversation_ids: Option<Vec<String>>,
    /// IDs of documents associated with this coaching appointment.
    #[serde(rename = "documentIds", skip_serializing_if = "Option::is_none")]
    pub document_ids: Option<Vec<String>>,
    /// The status of the coaching appointment.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(rename = "wfmSchedule", skip_serializing_if = "Option::is_none")]
    pub wfm_schedule: Option<Box<crate::models::WfmScheduleReference>>,
    /// The list of external links related to the appointment
    #[serde(rename = "externalLinks", skip_serializing_if = "Option::is_none")]
    pub external_links: Option<Vec<String>>,
}

impl UpdateCoachingAppointmentRequest {
    /// Update coaching appointment request
    pub fn new() -> UpdateCoachingAppointmentRequest {
        UpdateCoachingAppointmentRequest {
            name: None,
            description: None,
            date_start: None,
            length_in_minutes: None,
            conversation_ids: None,
            document_ids: None,
            status: None,
            wfm_schedule: None,
            external_links: None,
        }
    }
}

/// The status of the coaching appointment.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "Scheduled")]
    Scheduled,
    #[serde(rename = "InProgress")]
    InProgress,
    #[serde(rename = "Completed")]
    Completed,
}

impl Default for Status {
    fn default() -> Status {
        Self::Scheduled
    }
}

