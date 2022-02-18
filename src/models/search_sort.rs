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
pub struct SearchSort {
    /// The sort order for results
    #[serde(rename = "sortOrder", skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<SortOrder>,
    /// The field in the resource that you want to sort the results by
    #[serde(rename = "sortBy", skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
}

impl SearchSort {
    pub fn new() -> SearchSort {
        SearchSort {
            sort_order: None,
            sort_by: None,
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

