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
pub struct Document {
    /// The globally unique identifier for the object.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "changeNumber", skip_serializing_if = "Option::is_none")]
    pub change_number: Option<i32>,
    /// Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "dateCreated", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "dateModified", skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<String>,
    /// Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "dateUploaded", skip_serializing_if = "Option::is_none")]
    pub date_uploaded: Option<String>,
    #[serde(rename = "contentUri", skip_serializing_if = "Option::is_none")]
    pub content_uri: Option<String>,
    #[serde(rename = "workspace", skip_serializing_if = "Option::is_none")]
    pub workspace: Option<Box<crate::models::DomainEntityRef>>,
    #[serde(rename = "createdBy", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<Box<crate::models::DomainEntityRef>>,
    #[serde(rename = "uploadedBy", skip_serializing_if = "Option::is_none")]
    pub uploaded_by: Option<Box<crate::models::DomainEntityRef>>,
    #[serde(rename = "sharingUri", skip_serializing_if = "Option::is_none")]
    pub sharing_uri: Option<String>,
    #[serde(rename = "contentType", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(rename = "contentLength", skip_serializing_if = "Option::is_none")]
    pub content_length: Option<i64>,
    #[serde(rename = "systemType", skip_serializing_if = "Option::is_none")]
    pub system_type: Option<SystemType>,
    #[serde(rename = "filename", skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(rename = "pageCount", skip_serializing_if = "Option::is_none")]
    pub page_count: Option<i64>,
    #[serde(rename = "read", skip_serializing_if = "Option::is_none")]
    pub read: Option<bool>,
    #[serde(rename = "callerAddress", skip_serializing_if = "Option::is_none")]
    pub caller_address: Option<String>,
    #[serde(rename = "receiverAddress", skip_serializing_if = "Option::is_none")]
    pub receiver_address: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "tagValues", skip_serializing_if = "Option::is_none")]
    pub tag_values: Option<Vec<crate::models::TagValue>>,
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<crate::models::DocumentAttribute>>,
    #[serde(rename = "thumbnails", skip_serializing_if = "Option::is_none")]
    pub thumbnails: Option<Vec<crate::models::DocumentThumbnail>>,
    #[serde(rename = "uploadStatus", skip_serializing_if = "Option::is_none")]
    pub upload_status: Option<Box<crate::models::DomainEntityRef>>,
    #[serde(rename = "uploadDestinationUri", skip_serializing_if = "Option::is_none")]
    pub upload_destination_uri: Option<String>,
    #[serde(rename = "uploadMethod", skip_serializing_if = "Option::is_none")]
    pub upload_method: Option<UploadMethod>,
    #[serde(rename = "lockInfo", skip_serializing_if = "Option::is_none")]
    pub lock_info: Option<Box<crate::models::LockInfo>>,
    /// A list of permitted action rights for the user making the request
    #[serde(rename = "acl", skip_serializing_if = "Option::is_none")]
    pub acl: Option<Vec<String>>,
    #[serde(rename = "sharingStatus", skip_serializing_if = "Option::is_none")]
    pub sharing_status: Option<SharingStatus>,
    #[serde(rename = "downloadSharingUri", skip_serializing_if = "Option::is_none")]
    pub download_sharing_uri: Option<String>,
    /// The URI for this object
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
}

impl Document {
    pub fn new() -> Document {
        Document {
            id: None,
            name: None,
            change_number: None,
            date_created: None,
            date_modified: None,
            date_uploaded: None,
            content_uri: None,
            workspace: None,
            created_by: None,
            uploaded_by: None,
            sharing_uri: None,
            content_type: None,
            content_length: None,
            system_type: None,
            filename: None,
            page_count: None,
            read: None,
            caller_address: None,
            receiver_address: None,
            tags: None,
            tag_values: None,
            attributes: None,
            thumbnails: None,
            upload_status: None,
            upload_destination_uri: None,
            upload_method: None,
            lock_info: None,
            acl: None,
            sharing_status: None,
            download_sharing_uri: None,
            self_uri: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SystemType {
    #[serde(rename = "DOCUMENT")]
    DOCUMENT,
    #[serde(rename = "FAX")]
    FAX,
    #[serde(rename = "RECORDING")]
    RECORDING,
}

impl Default for SystemType {
    fn default() -> SystemType {
        Self::DOCUMENT
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum UploadMethod {
    #[serde(rename = "SINGLE_PUT")]
    SINGLEPUT,
    #[serde(rename = "MULTIPART_POST")]
    MULTIPARTPOST,
}

impl Default for UploadMethod {
    fn default() -> UploadMethod {
        Self::SINGLEPUT
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SharingStatus {
    #[serde(rename = "NONE")]
    NONE,
    #[serde(rename = "LIMITED")]
    LIMITED,
    #[serde(rename = "PUBLIC")]
    PUBLIC,
}

impl Default for SharingStatus {
    fn default() -> SharingStatus {
        Self::NONE
    }
}

