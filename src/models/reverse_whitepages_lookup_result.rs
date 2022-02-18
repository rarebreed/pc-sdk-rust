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
pub struct ReverseWhitepagesLookupResult {
    #[serde(rename = "contacts", skip_serializing_if = "Option::is_none")]
    pub contacts: Option<Vec<crate::models::ExternalContact>>,
    #[serde(rename = "externalOrganizations", skip_serializing_if = "Option::is_none")]
    pub external_organizations: Option<Vec<crate::models::ExternalOrganization>>,
}

impl ReverseWhitepagesLookupResult {
    pub fn new() -> ReverseWhitepagesLookupResult {
        ReverseWhitepagesLookupResult {
            contacts: None,
            external_organizations: None,
        }
    }
}


