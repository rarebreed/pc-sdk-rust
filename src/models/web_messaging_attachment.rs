/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// WebMessagingAttachment : Attachment object.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct WebMessagingAttachment {
    /// The globally unique identifier for the object.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The type of attachment this instance represents.
    #[serde(rename = "mediaType", skip_serializing_if = "Option::is_none")]
    pub media_type: Option<MediaType>,
    /// URL of the attachment.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Attachment mime type (https://www.iana.org/assignments/media-types/media-types.xhtml).
    #[serde(rename = "mime", skip_serializing_if = "Option::is_none")]
    pub mime: Option<String>,
    /// Text associated with attachment such as an image caption.
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Secure hash of the attachment content.
    #[serde(rename = "sha256", skip_serializing_if = "Option::is_none")]
    pub sha256: Option<String>,
    /// Suggested file name for attachment.
    #[serde(rename = "filename", skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    /// The file size associated with the file
    #[serde(rename = "fileSize", skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i32>,
}

impl WebMessagingAttachment {
    /// Attachment object.
    pub fn new() -> WebMessagingAttachment {
        WebMessagingAttachment {
            id: None,
            media_type: None,
            url: None,
            mime: None,
            text: None,
            sha256: None,
            filename: None,
            file_size: None,
        }
    }
}

/// The type of attachment this instance represents.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MediaType {
    #[serde(rename = "Image")]
    Image,
    #[serde(rename = "Video")]
    Video,
    #[serde(rename = "Audio")]
    Audio,
    #[serde(rename = "File")]
    File,
}

impl Default for MediaType {
    fn default() -> MediaType {
        Self::Image
    }
}

