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
pub struct ManagementUnitListing {
    #[serde(rename = "entities", skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<crate::models::ManagementUnit>>,
    /// Deprecated, paging is not supported
    #[serde(rename = "pageSize", skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// Deprecated, paging is not supported
    #[serde(rename = "pageNumber", skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// Deprecated, paging is not supported
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    /// Deprecated, paging is not supported
    #[serde(rename = "firstUri", skip_serializing_if = "Option::is_none")]
    pub first_uri: Option<String>,
    /// Deprecated, paging is not supported
    #[serde(rename = "nextUri", skip_serializing_if = "Option::is_none")]
    pub next_uri: Option<String>,
    /// Deprecated, paging is not supported
    #[serde(rename = "pageCount", skip_serializing_if = "Option::is_none")]
    pub page_count: Option<i32>,
    /// Deprecated, paging is not supported
    #[serde(rename = "previousUri", skip_serializing_if = "Option::is_none")]
    pub previous_uri: Option<String>,
    /// Deprecated, paging is not supported
    #[serde(rename = "lastUri", skip_serializing_if = "Option::is_none")]
    pub last_uri: Option<String>,
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
}

impl ManagementUnitListing {
    pub fn new() -> ManagementUnitListing {
        ManagementUnitListing {
            entities: None,
            page_size: None,
            page_number: None,
            total: None,
            first_uri: None,
            next_uri: None,
            page_count: None,
            previous_uri: None,
            last_uri: None,
            self_uri: None,
        }
    }
}

