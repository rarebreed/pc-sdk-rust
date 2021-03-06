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
pub struct OrphanRecording {
    /// The globally unique identifier for the object.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "createdTime", skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    /// Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "recoveredTime", skip_serializing_if = "Option::is_none")]
    pub recovered_time: Option<String>,
    #[serde(rename = "providerType", skip_serializing_if = "Option::is_none")]
    pub provider_type: Option<ProviderType>,
    #[serde(rename = "mediaSizeBytes", skip_serializing_if = "Option::is_none")]
    pub media_size_bytes: Option<i64>,
    #[serde(rename = "mediaType", skip_serializing_if = "Option::is_none")]
    pub media_type: Option<MediaType>,
    #[serde(rename = "fileState", skip_serializing_if = "Option::is_none")]
    pub file_state: Option<FileState>,
    #[serde(rename = "providerEndpoint", skip_serializing_if = "Option::is_none")]
    pub provider_endpoint: Option<Box<crate::models::Endpoint>>,
    #[serde(rename = "recording", skip_serializing_if = "Option::is_none")]
    pub recording: Option<Box<crate::models::Recording>>,
    /// The status of the orphaned recording's conversation.
    #[serde(rename = "orphanStatus", skip_serializing_if = "Option::is_none")]
    pub orphan_status: Option<OrphanStatus>,
    /// An identifier used during recovery operations by the supplying hybrid platform to track back and determine which interaction this recording is associated with
    #[serde(rename = "sourceOrphaningId", skip_serializing_if = "Option::is_none")]
    pub source_orphaning_id: Option<String>,
    /// The URI for this object
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
}

impl OrphanRecording {
    pub fn new() -> OrphanRecording {
        OrphanRecording {
            id: None,
            name: None,
            created_time: None,
            recovered_time: None,
            provider_type: None,
            media_size_bytes: None,
            media_type: None,
            file_state: None,
            provider_endpoint: None,
            recording: None,
            orphan_status: None,
            source_orphaning_id: None,
            self_uri: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ProviderType {
    #[serde(rename = "EDGE")]
    EDGE,
    #[serde(rename = "CHAT")]
    CHAT,
    #[serde(rename = "EMAIL")]
    EMAIL,
    #[serde(rename = "SCREEN_RECORDING")]
    SCREENRECORDING,
    #[serde(rename = "PUREENGAGE")]
    PUREENGAGE,
    #[serde(rename = "PURECONNECT")]
    PURECONNECT,
}

impl Default for ProviderType {
    fn default() -> ProviderType {
        Self::EDGE
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MediaType {
    #[serde(rename = "CALL")]
    CALL,
    #[serde(rename = "CHAT")]
    CHAT,
    #[serde(rename = "EMAIL")]
    EMAIL,
    #[serde(rename = "SCREEN")]
    SCREEN,
}

impl Default for MediaType {
    fn default() -> MediaType {
        Self::CALL
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FileState {
    #[serde(rename = "ARCHIVED")]
    ARCHIVED,
    #[serde(rename = "AVAILABLE")]
    AVAILABLE,
    #[serde(rename = "DELETED")]
    DELETED,
    #[serde(rename = "RESTORED")]
    RESTORED,
    #[serde(rename = "RESTORING")]
    RESTORING,
    #[serde(rename = "UPLOADING")]
    UPLOADING,
}

impl Default for FileState {
    fn default() -> FileState {
        Self::ARCHIVED
    }
}
/// The status of the orphaned recording's conversation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OrphanStatus {
    #[serde(rename = "NO_CONVERSATION")]
    NOCONVERSATION,
    #[serde(rename = "UNKNOWN_CONVERSATION")]
    UNKNOWNCONVERSATION,
    #[serde(rename = "CONVERSATION_NOT_COMPLETE")]
    CONVERSATIONNOTCOMPLETE,
    #[serde(rename = "CONVERSATION_NOT_EVALUATED")]
    CONVERSATIONNOTEVALUATED,
    #[serde(rename = "EVALUATED")]
    EVALUATED,
}

impl Default for OrphanStatus {
    fn default() -> OrphanStatus {
        Self::NOCONVERSATION
    }
}

