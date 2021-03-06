/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// ActionConfig : Defines components of the Action Config.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ActionConfig {
    #[serde(rename = "request", skip_serializing_if = "Option::is_none")]
    pub request: Option<Box<crate::models::RequestConfig>>,
    #[serde(rename = "response", skip_serializing_if = "Option::is_none")]
    pub response: Option<Box<crate::models::ResponseConfig>>,
}

impl ActionConfig {
    /// Defines components of the Action Config.
    pub fn new() -> ActionConfig {
        ActionConfig {
            request: None,
            response: None,
        }
    }
}


