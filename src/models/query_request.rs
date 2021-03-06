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
pub struct QueryRequest {
    #[serde(rename = "queryPhrase", skip_serializing_if = "Option::is_none")]
    pub query_phrase: Option<String>,
    #[serde(rename = "pageNumber", skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    #[serde(rename = "pageSize", skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(rename = "facetNameRequests", skip_serializing_if = "Option::is_none")]
    pub facet_name_requests: Option<Vec<String>>,
    #[serde(rename = "sort", skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<crate::models::SortItem>>,
    #[serde(rename = "filters", skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<crate::models::ContentFilterItem>>,
    #[serde(rename = "attributeFilters", skip_serializing_if = "Option::is_none")]
    pub attribute_filters: Option<Vec<crate::models::AttributeFilterItem>>,
    #[serde(rename = "includeShares", skip_serializing_if = "Option::is_none")]
    pub include_shares: Option<bool>,
}

impl QueryRequest {
    pub fn new() -> QueryRequest {
        QueryRequest {
            query_phrase: None,
            page_number: None,
            page_size: None,
            facet_name_requests: None,
            sort: None,
            filters: None,
            attribute_filters: None,
            include_shares: None,
        }
    }
}


