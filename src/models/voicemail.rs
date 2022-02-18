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
pub struct Voicemail {
    /// The voicemail id
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// current state of the voicemail upload
    #[serde(rename = "uploadStatus", skip_serializing_if = "Option::is_none")]
    pub upload_status: Option<UploadStatus>,
}

impl Voicemail {
    pub fn new() -> Voicemail {
        Voicemail {
            id: None,
            upload_status: None,
        }
    }
}

/// current state of the voicemail upload
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum UploadStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "complete")]
    Complete,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "timeout")]
    Timeout,
    #[serde(rename = "none")]
    None,
}

impl Default for UploadStatus {
    fn default() -> UploadStatus {
        Self::Pending
    }
}

