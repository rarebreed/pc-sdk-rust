/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// ScimUserListResponse : Defines a response for a list of SCIM users.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ScimUserListResponse {
    /// The list of supported schemas.
    #[serde(rename = "schemas", skip_serializing_if = "Option::is_none")]
    pub schemas: Option<Vec<String>>,
    /// The total number of results.
    #[serde(rename = "totalResults", skip_serializing_if = "Option::is_none")]
    pub total_results: Option<i64>,
    /// The 1-based index of the first result returned by this request. Add this to \"itemsPerPage\" when requesting the next page of results.
    #[serde(rename = "startIndex", skip_serializing_if = "Option::is_none")]
    pub start_index: Option<i64>,
    /// The number of resources returned per page.
    #[serde(rename = "itemsPerPage", skip_serializing_if = "Option::is_none")]
    pub items_per_page: Option<i64>,
    /// The list of requested resources. If \"count\" is 0, then the list will be empty.
    #[serde(rename = "Resources", skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<crate::models::ScimV2User>>,
}

impl ScimUserListResponse {
    /// Defines a response for a list of SCIM users.
    pub fn new() -> ScimUserListResponse {
        ScimUserListResponse {
            schemas: None,
            total_results: None,
            start_index: None,
            items_per_page: None,
            resources: None,
        }
    }
}


