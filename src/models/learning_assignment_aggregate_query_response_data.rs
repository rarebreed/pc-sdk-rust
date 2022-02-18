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
pub struct LearningAssignmentAggregateQueryResponseData {
    /// Specifies the range of due dates to be used for filtering. A maximum of 1 year can be specified in the range. Intervals are represented as an ISO-8601 string. For example: YYYY-MM-DDThh:mm:ss/YYYY-MM-DDThh:mm:ss
    #[serde(rename = "interval", skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    /// The list of aggregated metrics
    #[serde(rename = "metrics", skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Vec<crate::models::LearningAssignmentAggregateQueryResponseMetric>>,
}

impl LearningAssignmentAggregateQueryResponseData {
    pub fn new() -> LearningAssignmentAggregateQueryResponseData {
        LearningAssignmentAggregateQueryResponseData {
            interval: None,
            metrics: None,
        }
    }
}

