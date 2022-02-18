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
pub struct MessageInfo {
    /// Key that can be used to localize the message.
    #[serde(rename = "localizableMessageCode", skip_serializing_if = "Option::is_none")]
    pub localizable_message_code: Option<String>,
    /// Description of the message.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Message with template fields for variable replacement.
    #[serde(rename = "messageWithParams", skip_serializing_if = "Option::is_none")]
    pub message_with_params: Option<String>,
    /// Map with fields for variable replacement.
    #[serde(rename = "messageParams", skip_serializing_if = "Option::is_none")]
    pub message_params: Option<::std::collections::HashMap<String, String>>,
}

impl MessageInfo {
    pub fn new() -> MessageInfo {
        MessageInfo {
            localizable_message_code: None,
            message: None,
            message_with_params: None,
            message_params: None,
        }
    }
}

