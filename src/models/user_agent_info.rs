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
pub struct UserAgentInfo {
    /// The firmware version of the phone.
    #[serde(rename = "firmwareVersion", skip_serializing_if = "Option::is_none")]
    pub firmware_version: Option<String>,
    /// The manufacturer of the phone.
    #[serde(rename = "manufacturer", skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    /// The model of the phone.
    #[serde(rename = "model", skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
}

impl UserAgentInfo {
    pub fn new() -> UserAgentInfo {
        UserAgentInfo {
            firmware_version: None,
            manufacturer: None,
            model: None,
        }
    }
}


