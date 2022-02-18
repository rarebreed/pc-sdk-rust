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
pub struct FaxDocument {
    /// The globally unique identifier for the object.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "dateCreated", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "dateModified", skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<String>,
    #[serde(rename = "contentUri", skip_serializing_if = "Option::is_none")]
    pub content_uri: Option<String>,
    #[serde(rename = "workspace", skip_serializing_if = "Option::is_none")]
    pub workspace: Option<Box<crate::models::DomainEntityRef>>,
    #[serde(rename = "createdBy", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<Box<crate::models::DomainEntityRef>>,
    #[serde(rename = "sharingUri", skip_serializing_if = "Option::is_none")]
    pub sharing_uri: Option<String>,
    #[serde(rename = "contentType", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(rename = "contentLength", skip_serializing_if = "Option::is_none")]
    pub content_length: Option<i64>,
    #[serde(rename = "filename", skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(rename = "read", skip_serializing_if = "Option::is_none")]
    pub read: Option<bool>,
    #[serde(rename = "pageCount", skip_serializing_if = "Option::is_none")]
    pub page_count: Option<i64>,
    #[serde(rename = "callerAddress", skip_serializing_if = "Option::is_none")]
    pub caller_address: Option<String>,
    #[serde(rename = "receiverAddress", skip_serializing_if = "Option::is_none")]
    pub receiver_address: Option<String>,
    #[serde(rename = "thumbnails", skip_serializing_if = "Option::is_none")]
    pub thumbnails: Option<Vec<crate::models::DocumentThumbnail>>,
    #[serde(rename = "downloadSharingUri", skip_serializing_if = "Option::is_none")]
    pub download_sharing_uri: Option<String>,
    /// The URI for this object
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
}

impl FaxDocument {
    pub fn new() -> FaxDocument {
        FaxDocument {
            id: None,
            name: None,
            date_created: None,
            date_modified: None,
            content_uri: None,
            workspace: None,
            created_by: None,
            sharing_uri: None,
            content_type: None,
            content_length: None,
            filename: None,
            read: None,
            page_count: None,
            caller_address: None,
            receiver_address: None,
            thumbnails: None,
            download_sharing_uri: None,
            self_uri: None,
        }
    }
}

