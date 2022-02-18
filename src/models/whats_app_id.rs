/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// WhatsAppId : User information for a WhatsApp account



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct WhatsAppId {
    #[serde(rename = "phoneNumber", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<Box<crate::models::PhoneNumber>>,
    /// The displayName of this person's account in WhatsApp
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}

impl WhatsAppId {
    /// User information for a WhatsApp account
    pub fn new() -> WhatsAppId {
        WhatsAppId {
            phone_number: None,
            display_name: None,
        }
    }
}


