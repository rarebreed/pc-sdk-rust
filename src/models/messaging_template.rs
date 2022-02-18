/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// MessagingTemplate : The messaging template identifies a structured message templates supported by a messaging channel.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MessagingTemplate {
    #[serde(rename = "whatsApp", skip_serializing_if = "Option::is_none")]
    pub whats_app: Option<Box<crate::models::WhatsAppDefinition>>,
}

impl MessagingTemplate {
    /// The messaging template identifies a structured message templates supported by a messaging channel.
    pub fn new() -> MessagingTemplate {
        MessagingTemplate {
            whats_app: None,
        }
    }
}


