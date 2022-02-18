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
pub struct LockInfo {
    #[serde(rename = "lockedBy", skip_serializing_if = "Option::is_none")]
    pub locked_by: Option<Box<crate::models::DomainEntityRef>>,
    /// Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "dateCreated", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "dateExpires", skip_serializing_if = "Option::is_none")]
    pub date_expires: Option<String>,
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<Action>,
}

impl LockInfo {
    pub fn new() -> LockInfo {
        LockInfo {
            locked_by: None,
            date_created: None,
            date_expires: None,
            action: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "UPDATE")]
    UPDATE,
    #[serde(rename = "DELETE")]
    DELETE,
    #[serde(rename = "COPY")]
    COPY,
    #[serde(rename = "MOVE")]
    _MOVE,
    #[serde(rename = "REPLACE")]
    REPLACE,
    #[serde(rename = "THUMBNAIL")]
    THUMBNAIL,
    #[serde(rename = "TEXT_EXTRACTION")]
    TEXTEXTRACTION,
}

impl Default for Action {
    fn default() -> Action {
        Self::UPDATE
    }
}
