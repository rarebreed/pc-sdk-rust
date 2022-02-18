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
pub struct ForecastServiceGoalTemplateResponse {
    #[serde(rename = "serviceLevel", skip_serializing_if = "Option::is_none")]
    pub service_level: Option<Box<crate::models::ForecastServiceLevelResponse>>,
    #[serde(rename = "averageSpeedOfAnswer", skip_serializing_if = "Option::is_none")]
    pub average_speed_of_answer: Option<Box<crate::models::ForecastAverageSpeedOfAnswerResponse>>,
    #[serde(rename = "abandonRate", skip_serializing_if = "Option::is_none")]
    pub abandon_rate: Option<Box<crate::models::ForecastAbandonRateResponse>>,
}

impl ForecastServiceGoalTemplateResponse {
    pub fn new() -> ForecastServiceGoalTemplateResponse {
        ForecastServiceGoalTemplateResponse {
            service_level: None,
            average_speed_of_answer: None,
            abandon_rate: None,
        }
    }
}


