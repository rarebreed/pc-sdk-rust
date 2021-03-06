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
pub struct AuditSearchResult {
    /// Which page was returned.
    #[serde(rename = "pageNumber", skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// The number of results in a page.
    #[serde(rename = "pageSize", skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// The total number of results.
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
    /// The number of pages of results.
    #[serde(rename = "pageCount", skip_serializing_if = "Option::is_none")]
    pub page_count: Option<i32>,
    #[serde(rename = "facetInfo", skip_serializing_if = "Option::is_none")]
    pub facet_info: Option<Vec<crate::models::FacetInfo>>,
    #[serde(rename = "auditMessages", skip_serializing_if = "Option::is_none")]
    pub audit_messages: Option<Vec<crate::models::AuditMessage>>,
}

impl AuditSearchResult {
    pub fn new() -> AuditSearchResult {
        AuditSearchResult {
            page_number: None,
            page_size: None,
            total: None,
            page_count: None,
            facet_info: None,
            audit_messages: None,
        }
    }
}


