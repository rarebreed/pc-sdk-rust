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
pub struct DisconnectReason {
    /// Disconnect reason protocol type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Type>,
    /// Protocol specific reason code. See the Q.850 and SIP specs.
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,
    /// Human readable English description of the disconnect reason.
    #[serde(rename = "phrase", skip_serializing_if = "Option::is_none")]
    pub phrase: Option<String>,
}

impl DisconnectReason {
    pub fn new() -> DisconnectReason {
        DisconnectReason {
            _type: None,
            code: None,
            phrase: None,
        }
    }
}

/// Disconnect reason protocol type.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "q850")]
    Q850,
    #[serde(rename = "sip")]
    Sip,
}

impl Default for Type {
    fn default() -> Type {
        Self::Q850
    }
}

