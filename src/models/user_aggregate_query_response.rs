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
pub struct UserAggregateQueryResponse {
    /// A mapping from system presence to a list of organization presence ids
    #[serde(rename = "systemToOrganizationMappings", skip_serializing_if = "Option::is_none")]
    pub system_to_organization_mappings: Option<::std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "results", skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<crate::models::UserAggregateDataContainer>>,
}

impl UserAggregateQueryResponse {
    pub fn new() -> UserAggregateQueryResponse {
        UserAggregateQueryResponse {
            system_to_organization_mappings: None,
            results: None,
        }
    }
}


