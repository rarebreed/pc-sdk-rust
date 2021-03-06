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
pub struct GroupsSearchResponse {
    /// The total number of results found
    #[serde(rename = "total")]
    pub total: i64,
    /// The total number of pages
    #[serde(rename = "pageCount")]
    pub page_count: i32,
    /// The current page size
    #[serde(rename = "pageSize")]
    pub page_size: i32,
    /// The current page number
    #[serde(rename = "pageNumber")]
    pub page_number: i32,
    /// Q64 value for the previous page of results
    #[serde(rename = "previousPage", skip_serializing_if = "Option::is_none")]
    pub previous_page: Option<String>,
    /// Q64 value for the current page of results
    #[serde(rename = "currentPage", skip_serializing_if = "Option::is_none")]
    pub current_page: Option<String>,
    /// Q64 value for the next page of results
    #[serde(rename = "nextPage", skip_serializing_if = "Option::is_none")]
    pub next_page: Option<String>,
    /// Resource types the search was performed against
    #[serde(rename = "types")]
    pub types: Vec<String>,
    /// Search results
    #[serde(rename = "results")]
    pub results: Vec<crate::models::Group>,
}

impl GroupsSearchResponse {
    pub fn new(total: i64, page_count: i32, page_size: i32, page_number: i32, types: Vec<String>, results: Vec<crate::models::Group>) -> GroupsSearchResponse {
        GroupsSearchResponse {
            total,
            page_count,
            page_size,
            page_number,
            previous_page: None,
            current_page: None,
            next_page: None,
            types,
            results,
        }
    }
}


