/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// CobrowseSettings : Settings concerning cobrowse



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CobrowseSettings {
    /// Whether or not cobrowse is enabled
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Whether the viewer should have option to request control
    #[serde(rename = "allowAgentControl", skip_serializing_if = "Option::is_none")]
    pub allow_agent_control: Option<bool>,
    /// Mask patterns that will apply to pages being shared
    #[serde(rename = "maskSelectors", skip_serializing_if = "Option::is_none")]
    pub mask_selectors: Option<Vec<String>>,
}

impl CobrowseSettings {
    /// Settings concerning cobrowse
    pub fn new() -> CobrowseSettings {
        CobrowseSettings {
            enabled: None,
            allow_agent_control: None,
            mask_selectors: None,
        }
    }
}


