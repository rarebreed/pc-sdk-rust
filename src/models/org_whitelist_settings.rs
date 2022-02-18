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
pub struct OrgWhitelistSettings {
    #[serde(rename = "enableWhitelist", skip_serializing_if = "Option::is_none")]
    pub enable_whitelist: Option<bool>,
    #[serde(rename = "domainWhitelist", skip_serializing_if = "Option::is_none")]
    pub domain_whitelist: Option<Vec<String>>,
}

impl OrgWhitelistSettings {
    pub fn new() -> OrgWhitelistSettings {
        OrgWhitelistSettings {
            enable_whitelist: None,
            domain_whitelist: None,
        }
    }
}

