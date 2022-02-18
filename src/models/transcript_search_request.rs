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
pub struct TranscriptSearchRequest {
    /// The sort order for results
    #[serde(rename = "sortOrder", skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<SortOrder>,
    /// The field in the resource that you want to sort the results by
    #[serde(rename = "sortBy", skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// The number of results per page
    #[serde(rename = "pageSize", skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// The page of resources you want to retrieve
    #[serde(rename = "pageNumber", skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// Multi-value sort order, list of multiple sort values
    #[serde(rename = "sort", skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<crate::models::SearchSort>>,
    #[serde(rename = "returnFields", skip_serializing_if = "Option::is_none")]
    pub return_fields: Option<Vec<String>>,
    /// Resource domain type to search
    #[serde(rename = "types")]
    pub types: Vec<String>,
    /// The search criteria
    #[serde(rename = "query", skip_serializing_if = "Option::is_none")]
    pub query: Option<Vec<crate::models::TranscriptSearchCriteria>>,
}

impl TranscriptSearchRequest {
    pub fn new(types: Vec<String>) -> TranscriptSearchRequest {
        TranscriptSearchRequest {
            sort_order: None,
            sort_by: None,
            page_size: None,
            page_number: None,
            sort: None,
            return_fields: None,
            types,
            query: None,
        }
    }
}

/// The sort order for results
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SortOrder {
    #[serde(rename = "ASC")]
    ASC,
    #[serde(rename = "DESC")]
    DESC,
    #[serde(rename = "SCORE")]
    SCORE,
}

impl Default for SortOrder {
    fn default() -> SortOrder {
        Self::ASC
    }
}

