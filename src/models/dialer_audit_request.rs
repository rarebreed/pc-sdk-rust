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
pub struct DialerAuditRequest {
    /// The word or words to search for.
    #[serde(rename = "queryPhrase", skip_serializing_if = "Option::is_none")]
    pub query_phrase: Option<String>,
    /// The fields in which to search for the queryPhrase.
    #[serde(rename = "queryFields", skip_serializing_if = "Option::is_none")]
    pub query_fields: Option<Vec<String>>,
    /// The fields to facet on.
    #[serde(rename = "facets", skip_serializing_if = "Option::is_none")]
    pub facets: Option<Vec<crate::models::AuditFacet>>,
    /// The fields to filter on.
    #[serde(rename = "filters", skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<crate::models::AuditFilter>>,
}

impl DialerAuditRequest {
    pub fn new() -> DialerAuditRequest {
        DialerAuditRequest {
            query_phrase: None,
            query_fields: None,
            facets: None,
            filters: None,
        }
    }
}

