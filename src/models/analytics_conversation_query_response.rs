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
pub struct AnalyticsConversationQueryResponse {
    #[serde(rename = "aggregations", skip_serializing_if = "Option::is_none")]
    pub aggregations: Option<Vec<crate::models::AggregationResult>>,
    #[serde(rename = "conversations", skip_serializing_if = "Option::is_none")]
    pub conversations: Option<Vec<crate::models::AnalyticsConversationWithoutAttributes>>,
    #[serde(rename = "totalHits", skip_serializing_if = "Option::is_none")]
    pub total_hits: Option<i32>,
}

impl AnalyticsConversationQueryResponse {
    pub fn new() -> AnalyticsConversationQueryResponse {
        AnalyticsConversationQueryResponse {
            aggregations: None,
            conversations: None,
            total_hits: None,
        }
    }
}

