/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// ResponseText : Contains information about the text associated with a response.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ResponseText {
    /// Response text content.
    #[serde(rename = "content")]
    pub content: String,
    /// Response text content type.
    #[serde(rename = "contentType", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<ContentType>,
}

impl ResponseText {
    /// Contains information about the text associated with a response.
    pub fn new(content: String) -> ResponseText {
        ResponseText {
            content,
            content_type: None,
        }
    }
}

/// Response text content type.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ContentType {
    #[serde(rename = "text/plain")]
    Plain,
    #[serde(rename = "text/html")]
    Html,
}

impl Default for ContentType {
    fn default() -> ContentType {
        Self::Plain
    }
}
