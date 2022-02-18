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
pub struct AggregationResult {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Type>,
    /// For termFrequency aggregations
    #[serde(rename = "dimension", skip_serializing_if = "Option::is_none")]
    pub dimension: Option<String>,
    /// For numericRange aggregations
    #[serde(rename = "metric", skip_serializing_if = "Option::is_none")]
    pub metric: Option<String>,
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(rename = "results", skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<crate::models::AggregationResultEntry>>,
}

impl AggregationResult {
    pub fn new() -> AggregationResult {
        AggregationResult {
            _type: None,
            dimension: None,
            metric: None,
            count: None,
            results: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "termFrequency")]
    TermFrequency,
    #[serde(rename = "numericRange")]
    NumericRange,
}

impl Default for Type {
    fn default() -> Type {
        Self::TermFrequency
    }
}
