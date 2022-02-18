/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// ClientAppConfigurationInfo : Configuration information for the integration



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ClientAppConfigurationInfo {
    #[serde(rename = "current", skip_serializing_if = "Option::is_none")]
    pub current: Option<Box<crate::models::IntegrationConfiguration>>,
    #[serde(rename = "effective", skip_serializing_if = "Option::is_none")]
    pub effective: Option<Box<crate::models::EffectiveConfiguration>>,
}

impl ClientAppConfigurationInfo {
    /// Configuration information for the integration
    pub fn new() -> ClientAppConfigurationInfo {
        ClientAppConfigurationInfo {
            current: None,
            effective: None,
        }
    }
}


