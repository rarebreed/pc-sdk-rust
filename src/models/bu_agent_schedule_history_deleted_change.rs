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
pub struct BuAgentScheduleHistoryDeletedChange {
    /// The IDs of deleted shifts
    #[serde(rename = "shiftIds", skip_serializing_if = "Option::is_none")]
    pub shift_ids: Option<Vec<String>>,
    /// The dates of any deleted full day time off markers
    #[serde(rename = "fullDayTimeOffMarkerDates", skip_serializing_if = "Option::is_none")]
    pub full_day_time_off_marker_dates: Option<Vec<String>>,
    /// Whether the entire agent schedule was deleted
    #[serde(rename = "agentSchedule", skip_serializing_if = "Option::is_none")]
    pub agent_schedule: Option<bool>,
}

impl BuAgentScheduleHistoryDeletedChange {
    pub fn new() -> BuAgentScheduleHistoryDeletedChange {
        BuAgentScheduleHistoryDeletedChange {
            shift_ids: None,
            full_day_time_off_marker_dates: None,
            agent_schedule: None,
        }
    }
}


