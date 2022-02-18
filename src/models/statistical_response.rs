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
pub struct StatisticalResponse {
    #[serde(rename = "interval", skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    #[serde(rename = "metrics", skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Vec<crate::models::AggregateMetricData>>,
    #[serde(rename = "views", skip_serializing_if = "Option::is_none")]
    pub views: Option<Vec<crate::models::AggregateViewData>>,
}

impl StatisticalResponse {
    pub fn new() -> StatisticalResponse {
        StatisticalResponse {
            interval: None,
            metrics: None,
            views: None,
        }
    }
}


