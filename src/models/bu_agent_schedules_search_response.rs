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
pub struct BuAgentSchedulesSearchResponse {
    /// The requested agent schedules
    #[serde(rename = "agentSchedules", skip_serializing_if = "Option::is_none")]
    pub agent_schedules: Option<Vec<crate::models::BuAgentScheduleSearchResponse>>,
    /// The time zone configured for the business unit to which this schedule applies
    #[serde(rename = "businessUnitTimeZone", skip_serializing_if = "Option::is_none")]
    pub business_unit_time_zone: Option<String>,
    /// References to all published week schedules overlapping the start/end date query parameters
    #[serde(rename = "publishedSchedules", skip_serializing_if = "Option::is_none")]
    pub published_schedules: Option<Vec<crate::models::BuAgentSchedulePublishedScheduleReference>>,
}

impl BuAgentSchedulesSearchResponse {
    pub fn new() -> BuAgentSchedulesSearchResponse {
        BuAgentSchedulesSearchResponse {
            agent_schedules: None,
            business_unit_time_zone: None,
            published_schedules: None,
        }
    }
}

