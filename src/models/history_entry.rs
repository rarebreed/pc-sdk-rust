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
pub struct HistoryEntry {
    /// The action performed
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<Action>,
    /// For actions performed not on the item itself, but on a sub-item, this field identifies the sub-item by name.  For example, for actions performed on prompt resources, this will be the prompt resource name.
    #[serde(rename = "resource", skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    /// Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<crate::models::User>>,
    #[serde(rename = "client", skip_serializing_if = "Option::is_none")]
    pub client: Option<Box<crate::models::DomainEntityRef>>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "secure", skip_serializing_if = "Option::is_none")]
    pub secure: Option<bool>,
}

impl HistoryEntry {
    pub fn new() -> HistoryEntry {
        HistoryEntry {
            action: None,
            resource: None,
            timestamp: None,
            user: None,
            client: None,
            version: None,
            secure: None,
        }
    }
}

/// The action performed
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "CHECKIN")]
    CHECKIN,
    #[serde(rename = "CHECKOUT")]
    CHECKOUT,
    #[serde(rename = "CREATE")]
    CREATE,
    #[serde(rename = "DEACTIVATE")]
    DEACTIVATE,
    #[serde(rename = "DEBUG")]
    DEBUG,
    #[serde(rename = "DELETE")]
    DELETE,
    #[serde(rename = "PUBLISH")]
    PUBLISH,
    #[serde(rename = "REVERT")]
    REVERT,
    #[serde(rename = "SAVE")]
    SAVE,
    #[serde(rename = "TRANSCODE")]
    TRANSCODE,
    #[serde(rename = "UPDATE")]
    UPDATE,
    #[serde(rename = "UPLOAD")]
    UPLOAD,
}

impl Default for Action {
    fn default() -> Action {
        Self::CHECKIN
    }
}

