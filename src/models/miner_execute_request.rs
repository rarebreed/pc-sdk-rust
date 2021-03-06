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
pub struct MinerExecuteRequest {
    /// Start date for the date range to mine. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd
    #[serde(rename = "dateStart", skip_serializing_if = "Option::is_none")]
    pub date_start: Option<String>,
    /// End date for the date range to mine. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd
    #[serde(rename = "dateEnd", skip_serializing_if = "Option::is_none")]
    pub date_end: Option<String>,
    /// Location of input conversations.
    #[serde(rename = "uploadKey", skip_serializing_if = "Option::is_none")]
    pub upload_key: Option<String>,
    /// Media type for filtering conversations.
    #[serde(rename = "mediaType", skip_serializing_if = "Option::is_none")]
    pub media_type: Option<MediaType>,
    /// List of queue IDs for filtering conversations.
    #[serde(rename = "queueIds", skip_serializing_if = "Option::is_none")]
    pub queue_ids: Option<Vec<String>>,
}

impl MinerExecuteRequest {
    pub fn new() -> MinerExecuteRequest {
        MinerExecuteRequest {
            date_start: None,
            date_end: None,
            upload_key: None,
            media_type: None,
            queue_ids: None,
        }
    }
}

/// Media type for filtering conversations.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MediaType {
    #[serde(rename = "Chat")]
    Chat,
    #[serde(rename = "Call")]
    Call,
    #[serde(rename = "Message")]
    Message,
}

impl Default for MediaType {
    fn default() -> MediaType {
        Self::Chat
    }
}

