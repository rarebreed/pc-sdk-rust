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
pub struct LicenseOrgToggle {
    #[serde(rename = "featureName", skip_serializing_if = "Option::is_none")]
    pub feature_name: Option<String>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl LicenseOrgToggle {
    pub fn new() -> LicenseOrgToggle {
        LicenseOrgToggle {
            feature_name: None,
            enabled: None,
        }
    }
}


