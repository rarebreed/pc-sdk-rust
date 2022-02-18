/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// ResponseQueryRequest : Used to query for responses



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ResponseQueryRequest {
    /// Query phrase to search response text and name. If not set will match all.
    #[serde(rename = "queryPhrase", skip_serializing_if = "Option::is_none")]
    pub query_phrase: Option<String>,
    /// The maximum number of hits to return. Default: 25, Maximum: 500.
    #[serde(rename = "pageSize", skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// Filter the query results.
    #[serde(rename = "filters", skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<crate::models::ResponseFilter>>,
}

impl ResponseQueryRequest {
    /// Used to query for responses
    pub fn new() -> ResponseQueryRequest {
        ResponseQueryRequest {
            query_phrase: None,
            page_size: None,
            filters: None,
        }
    }
}


