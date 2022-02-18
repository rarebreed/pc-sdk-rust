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
pub struct MessagingIntegration {
    /// A unique Integration Id
    #[serde(rename = "id")]
    pub id: String,
    /// The name of the Integration
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "supportedContent", skip_serializing_if = "Option::is_none")]
    pub supported_content: Option<Box<crate::models::SupportedContentReference>>,
    /// The status of the Integration
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// The type of Messaging Integration
    #[serde(rename = "messengerType")]
    pub messenger_type: MessengerType,
    #[serde(rename = "recipient", skip_serializing_if = "Option::is_none")]
    pub recipient: Option<Box<crate::models::DomainEntityRef>>,
    /// Date this Integration was created. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "dateCreated", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// Date this Integration was modified. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "dateModified", skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<String>,
    #[serde(rename = "createdBy", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<Box<crate::models::DomainEntityRef>>,
    #[serde(rename = "modifiedBy", skip_serializing_if = "Option::is_none")]
    pub modified_by: Option<Box<crate::models::DomainEntityRef>>,
    /// Version number required for updates.
    #[serde(rename = "version")]
    pub version: i32,
    /// The URI for this object
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
}

impl MessagingIntegration {
    pub fn new(id: String, name: String, messenger_type: MessengerType, version: i32) -> MessagingIntegration {
        MessagingIntegration {
            id,
            name,
            supported_content: None,
            status: None,
            messenger_type,
            recipient: None,
            date_created: None,
            date_modified: None,
            created_by: None,
            modified_by: None,
            version,
            self_uri: None,
        }
    }
}

/// The status of the Integration
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "Active")]
    Active,
    #[serde(rename = "Inactive")]
    Inactive,
    #[serde(rename = "Error")]
    Error,
    #[serde(rename = "Starting")]
    Starting,
    #[serde(rename = "Incomplete")]
    Incomplete,
    #[serde(rename = "Deleting")]
    Deleting,
    #[serde(rename = "DeletionFailed")]
    DeletionFailed,
}

impl Default for Status {
    fn default() -> Status {
        Self::Active
    }
}
/// The type of Messaging Integration
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MessengerType {
    #[serde(rename = "sms")]
    Sms,
    #[serde(rename = "facebook")]
    Facebook,
    #[serde(rename = "twitter")]
    Twitter,
    #[serde(rename = "line")]
    Line,
    #[serde(rename = "whatsapp")]
    Whatsapp,
    #[serde(rename = "webmessaging")]
    Webmessaging,
    #[serde(rename = "instagram")]
    Instagram,
    #[serde(rename = "open")]
    Open,
}

impl Default for MessengerType {
    fn default() -> MessengerType {
        Self::Sms
    }
}

