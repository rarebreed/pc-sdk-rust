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
pub struct VoicemailMessage {
    /// The globally unique identifier for the object.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "conversation", skip_serializing_if = "Option::is_none")]
    pub conversation: Option<Box<crate::models::Conversation>>,
    /// Whether the voicemail message is marked as read
    #[serde(rename = "read", skip_serializing_if = "Option::is_none")]
    pub read: Option<bool>,
    /// The voicemail message's audio recording duration in seconds
    #[serde(rename = "audioRecordingDurationSeconds", skip_serializing_if = "Option::is_none")]
    pub audio_recording_duration_seconds: Option<i32>,
    /// The voicemail message's audio recording size in bytes
    #[serde(rename = "audioRecordingSizeBytes", skip_serializing_if = "Option::is_none")]
    pub audio_recording_size_bytes: Option<i64>,
    /// The date the voicemail message was created. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "createdDate", skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    /// The date the voicemail message was last modified. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "modifiedDate", skip_serializing_if = "Option::is_none")]
    pub modified_date: Option<String>,
    /// The date the voicemail message deleted property was set to true. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "deletedDate", skip_serializing_if = "Option::is_none")]
    pub deleted_date: Option<String>,
    /// The caller address
    #[serde(rename = "callerAddress", skip_serializing_if = "Option::is_none")]
    pub caller_address: Option<String>,
    /// Optionally the name of the caller that left the voicemail message if the caller was a known user
    #[serde(rename = "callerName", skip_serializing_if = "Option::is_none")]
    pub caller_name: Option<String>,
    #[serde(rename = "callerUser", skip_serializing_if = "Option::is_none")]
    pub caller_user: Option<Box<crate::models::User>>,
    /// Whether the voicemail message has been marked as deleted
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// An optional note
    #[serde(rename = "note", skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<crate::models::User>>,
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: Option<Box<crate::models::Group>>,
    #[serde(rename = "queue", skip_serializing_if = "Option::is_none")]
    pub queue: Option<Box<crate::models::Queue>>,
    #[serde(rename = "copiedFrom", skip_serializing_if = "Option::is_none")]
    pub copied_from: Option<Box<crate::models::VoicemailCopyRecord>>,
    /// Represents where this voicemail has been copied to
    #[serde(rename = "copiedTo", skip_serializing_if = "Option::is_none")]
    pub copied_to: Option<Vec<crate::models::VoicemailCopyRecord>>,
    #[serde(rename = "deleteRetentionPolicy", skip_serializing_if = "Option::is_none")]
    pub delete_retention_policy: Option<Box<crate::models::VoicemailRetentionPolicy>>,
    /// The URI for this object
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
}

impl VoicemailMessage {
    pub fn new() -> VoicemailMessage {
        VoicemailMessage {
            id: None,
            conversation: None,
            read: None,
            audio_recording_duration_seconds: None,
            audio_recording_size_bytes: None,
            created_date: None,
            modified_date: None,
            deleted_date: None,
            caller_address: None,
            caller_name: None,
            caller_user: None,
            deleted: None,
            note: None,
            user: None,
            group: None,
            queue: None,
            copied_from: None,
            copied_to: None,
            delete_retention_policy: None,
            self_uri: None,
        }
    }
}


