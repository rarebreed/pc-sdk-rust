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
pub struct MediaPolicies {
    #[serde(rename = "callPolicy", skip_serializing_if = "Option::is_none")]
    pub call_policy: Option<Box<crate::models::CallMediaPolicy>>,
    #[serde(rename = "chatPolicy", skip_serializing_if = "Option::is_none")]
    pub chat_policy: Option<Box<crate::models::ChatMediaPolicy>>,
    #[serde(rename = "emailPolicy", skip_serializing_if = "Option::is_none")]
    pub email_policy: Option<Box<crate::models::EmailMediaPolicy>>,
    #[serde(rename = "messagePolicy", skip_serializing_if = "Option::is_none")]
    pub message_policy: Option<Box<crate::models::MessageMediaPolicy>>,
}

impl MediaPolicies {
    pub fn new() -> MediaPolicies {
        MediaPolicies {
            call_policy: None,
            chat_policy: None,
            email_policy: None,
            message_policy: None,
        }
    }
}


