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
pub struct EdgeLogsJobFile {
    /// The globally unique identifier for the object.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of the entity.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "division", skip_serializing_if = "Option::is_none")]
    pub division: Option<Box<crate::models::Division>>,
    /// The resource's description.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The current version of the resource.
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
    /// The date the resource was created. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "dateCreated", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The date of the last modification to the resource. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "dateModified", skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<String>,
    /// The ID of the user that last modified the resource.
    #[serde(rename = "modifiedBy", skip_serializing_if = "Option::is_none")]
    pub modified_by: Option<String>,
    /// The ID of the user that created the resource.
    #[serde(rename = "createdBy", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    /// Indicates if the resource is active, inactive, or deleted.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
    /// The application that last modified the resource.
    #[serde(rename = "modifiedByApp", skip_serializing_if = "Option::is_none")]
    pub modified_by_app: Option<String>,
    /// The application that created the resource.
    #[serde(rename = "createdByApp", skip_serializing_if = "Option::is_none")]
    pub created_by_app: Option<String>,
    /// Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "timeCreated", skip_serializing_if = "Option::is_none")]
    pub time_created: Option<String>,
    /// The time this log file was last modified on the Edge. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "timeModified", skip_serializing_if = "Option::is_none")]
    pub time_modified: Option<String>,
    /// The size of this file in bytes.
    #[serde(rename = "sizeBytes", skip_serializing_if = "Option::is_none")]
    pub size_bytes: Option<f64>,
    /// The status of the upload of this file from the Edge to the cloud.  Use /upload to start an upload.
    #[serde(rename = "uploadStatus", skip_serializing_if = "Option::is_none")]
    pub upload_status: Option<UploadStatus>,
    /// The path of this file on the Edge.
    #[serde(rename = "edgePath", skip_serializing_if = "Option::is_none")]
    pub edge_path: Option<String>,
    /// The download ID to use with the downloads API.
    #[serde(rename = "downloadId", skip_serializing_if = "Option::is_none")]
    pub download_id: Option<String>,
    /// The URI for this object
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
}

impl EdgeLogsJobFile {
    pub fn new(name: String) -> EdgeLogsJobFile {
        EdgeLogsJobFile {
            id: None,
            name,
            division: None,
            description: None,
            version: None,
            date_created: None,
            date_modified: None,
            modified_by: None,
            created_by: None,
            state: None,
            modified_by_app: None,
            created_by_app: None,
            time_created: None,
            time_modified: None,
            size_bytes: None,
            upload_status: None,
            edge_path: None,
            download_id: None,
            self_uri: None,
        }
    }
}

/// Indicates if the resource is active, inactive, or deleted.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(rename = "deleted")]
    Deleted,
}

impl Default for State {
    fn default() -> State {
        Self::Active
    }
}
/// The status of the upload of this file from the Edge to the cloud.  Use /upload to start an upload.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum UploadStatus {
    #[serde(rename = "UPLOADING")]
    UPLOADING,
    #[serde(rename = "NOT_UPLOADED")]
    NOTUPLOADED,
    #[serde(rename = "UPLOADED")]
    UPLOADED,
    #[serde(rename = "ERROR_ON_UPLOAD")]
    ERRORONUPLOAD,
}

impl Default for UploadStatus {
    fn default() -> UploadStatus {
        Self::UPLOADING
    }
}

