/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// OpenMessagingFromRecipient : Information about the recipient the message is received from.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OpenMessagingFromRecipient {
    /// Nickname or display name of the recipient.
    #[serde(rename = "nickname", skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    /// The recipient ID specific to the provider.
    #[serde(rename = "id")]
    pub id: String,
    /// The recipient ID type. This is used to indicate the format used for the ID.
    #[serde(rename = "idType")]
    pub id_type: IdType,
    /// First name of the recipient.
    #[serde(rename = "firstName", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Last name of the recipient.
    #[serde(rename = "lastName", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// URL of an image that represents the recipient.
    #[serde(rename = "image", skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// E-mail address of the recipient.
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

impl OpenMessagingFromRecipient {
    /// Information about the recipient the message is received from.
    pub fn new(id: String, id_type: IdType) -> OpenMessagingFromRecipient {
        OpenMessagingFromRecipient {
            nickname: None,
            id,
            id_type,
            first_name: None,
            last_name: None,
            image: None,
            email: None,
        }
    }
}

/// The recipient ID type. This is used to indicate the format used for the ID.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IdType {
    #[serde(rename = "Email")]
    Email,
    #[serde(rename = "Phone")]
    Phone,
    #[serde(rename = "Opaque")]
    Opaque,
}

impl Default for IdType {
    fn default() -> IdType {
        Self::Email
    }
}
