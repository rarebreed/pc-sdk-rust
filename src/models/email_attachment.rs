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
pub struct EmailAttachment {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "contentPath", skip_serializing_if = "Option::is_none")]
    pub content_path: Option<String>,
    #[serde(rename = "contentType", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(rename = "attachmentId", skip_serializing_if = "Option::is_none")]
    pub attachment_id: Option<String>,
    #[serde(rename = "contentLength", skip_serializing_if = "Option::is_none")]
    pub content_length: Option<i32>,
}

impl EmailAttachment {
    pub fn new() -> EmailAttachment {
        EmailAttachment {
            name: None,
            content_path: None,
            content_type: None,
            attachment_id: None,
            content_length: None,
        }
    }
}


