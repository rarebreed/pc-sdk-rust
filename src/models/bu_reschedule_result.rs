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
pub struct BuRescheduleResult {
    #[serde(rename = "generationResults", skip_serializing_if = "Option::is_none")]
    pub generation_results: Option<Box<crate::models::ScheduleGenerationResult>>,
    /// The download URL from which to fetch the generation results for the rescheduling run
    #[serde(rename = "generationResultsDownloadUrl", skip_serializing_if = "Option::is_none")]
    pub generation_results_download_url: Option<String>,
    #[serde(rename = "headcountForecast", skip_serializing_if = "Option::is_none")]
    pub headcount_forecast: Option<Box<crate::models::BuHeadcountForecast>>,
    /// The download URL from which to fetch the headcount forecast for the rescheduling run
    #[serde(rename = "headcountForecastDownloadUrl", skip_serializing_if = "Option::is_none")]
    pub headcount_forecast_download_url: Option<String>,
    /// List of download links for agent schedules produced by the rescheduling run
    #[serde(rename = "agentSchedules", skip_serializing_if = "Option::is_none")]
    pub agent_schedules: Option<Vec<crate::models::BuRescheduleAgentScheduleResult>>,
}

impl BuRescheduleResult {
    pub fn new() -> BuRescheduleResult {
        BuRescheduleResult {
            generation_results: None,
            generation_results_download_url: None,
            headcount_forecast: None,
            headcount_forecast_download_url: None,
            agent_schedules: None,
        }
    }
}


