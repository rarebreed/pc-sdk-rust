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
pub struct BuAgentScheduleRescheduleResponse {
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<crate::models::UserReference>>,
    /// The shift definitions for this agent schedule
    #[serde(rename = "shifts", skip_serializing_if = "Option::is_none")]
    pub shifts: Option<Vec<crate::models::BuAgentScheduleShift>>,
    /// Full day time off markers which apply to this agent schedule
    #[serde(rename = "fullDayTimeOffMarkers", skip_serializing_if = "Option::is_none")]
    pub full_day_time_off_markers: Option<Vec<crate::models::BuFullDayTimeOffMarker>>,
    #[serde(rename = "workPlan", skip_serializing_if = "Option::is_none")]
    pub work_plan: Option<Box<crate::models::WorkPlanReference>>,
    /// The work plans per week for this user from the work plan rotation. Null values in the list denotes that user is not part of any work plan for that week
    #[serde(rename = "workPlansPerWeek", skip_serializing_if = "Option::is_none")]
    pub work_plans_per_week: Option<Vec<crate::models::WorkPlanReference>>,
}

impl BuAgentScheduleRescheduleResponse {
    pub fn new() -> BuAgentScheduleRescheduleResponse {
        BuAgentScheduleRescheduleResponse {
            user: None,
            shifts: None,
            full_day_time_off_markers: None,
            work_plan: None,
            work_plans_per_week: None,
        }
    }
}

