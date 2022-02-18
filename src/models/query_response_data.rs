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
pub struct QueryResponseData {
    /// Interval with start and end represented as ISO-8601 string. i.e: yyyy-MM-dd'T'HH:mm:ss.SSS'Z'/yyyy-MM-dd'T'HH:mm:ss.SSS'Z'
    #[serde(rename = "interval", skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    /// A list of aggregated metrics
    #[serde(rename = "metrics", skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Vec<crate::models::QueryResponseMetric>>,
}

impl QueryResponseData {
    pub fn new() -> QueryResponseData {
        QueryResponseData {
            interval: None,
            metrics: None,
        }
    }
}


