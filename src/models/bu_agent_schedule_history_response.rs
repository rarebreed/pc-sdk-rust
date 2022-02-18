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
pub struct BuAgentScheduleHistoryResponse {
    /// The list of previously published schedules
    #[serde(rename = "priorPublishedSchedules", skip_serializing_if = "Option::is_none")]
    pub prior_published_schedules: Option<Vec<crate::models::BuScheduleReference>>,
    #[serde(rename = "basePublishedSchedule", skip_serializing_if = "Option::is_none")]
    pub base_published_schedule: Option<Box<crate::models::BuAgentScheduleHistoryChange>>,
    /// The changes dropped from the schedule history. This will happen if the schedule history is too large
    #[serde(rename = "droppedChanges", skip_serializing_if = "Option::is_none")]
    pub dropped_changes: Option<Vec<crate::models::BuAgentScheduleHistoryDroppedChange>>,
    /// The list of changes for the schedule history
    #[serde(rename = "changes", skip_serializing_if = "Option::is_none")]
    pub changes: Option<Vec<crate::models::BuAgentScheduleHistoryChange>>,
}

impl BuAgentScheduleHistoryResponse {
    pub fn new() -> BuAgentScheduleHistoryResponse {
        BuAgentScheduleHistoryResponse {
            prior_published_schedules: None,
            base_published_schedule: None,
            dropped_changes: None,
            changes: None,
        }
    }
}

