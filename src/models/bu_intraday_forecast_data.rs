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
pub struct BuIntradayForecastData {
    /// The number of interactions routed into the queues in the selected planning groups for the given media type for an agent to answer
    #[serde(rename = "offered", skip_serializing_if = "Option::is_none")]
    pub offered: Option<f64>,
    /// The average handle time in seconds an agent spent handling interactions
    #[serde(rename = "averageHandleTimeSeconds", skip_serializing_if = "Option::is_none")]
    pub average_handle_time_seconds: Option<f64>,
}

impl BuIntradayForecastData {
    pub fn new() -> BuIntradayForecastData {
        BuIntradayForecastData {
            offered: None,
            average_handle_time_seconds: None,
        }
    }
}


