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
pub struct SuggestSearchRequest {
    /// Provides more details about a specified resource
    #[serde(rename = "expand", skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
    /// Resource domain type to search
    #[serde(rename = "types")]
    pub types: Vec<String>,
    /// Suggest query
    #[serde(rename = "query")]
    pub query: Vec<crate::models::SuggestSearchCriteria>,
}

impl SuggestSearchRequest {
    pub fn new(types: Vec<String>, query: Vec<crate::models::SuggestSearchCriteria>) -> SuggestSearchRequest {
        SuggestSearchRequest {
            expand: None,
            types,
            query,
        }
    }
}

