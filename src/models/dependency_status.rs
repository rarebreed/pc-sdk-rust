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
pub struct DependencyStatus {
    /// The globally unique identifier for the object.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<crate::models::User>>,
    #[serde(rename = "client", skip_serializing_if = "Option::is_none")]
    pub client: Option<Box<crate::models::DomainEntityRef>>,
    #[serde(rename = "buildId", skip_serializing_if = "Option::is_none")]
    pub build_id: Option<String>,
    /// Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "dateStarted", skip_serializing_if = "Option::is_none")]
    pub date_started: Option<String>,
    /// Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "dateCompleted", skip_serializing_if = "Option::is_none")]
    pub date_completed: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(rename = "failedObjects", skip_serializing_if = "Option::is_none")]
    pub failed_objects: Option<Vec<crate::models::FailedObject>>,
    /// The URI for this object
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
}

impl DependencyStatus {
    pub fn new() -> DependencyStatus {
        DependencyStatus {
            id: None,
            name: None,
            user: None,
            client: None,
            build_id: None,
            date_started: None,
            date_completed: None,
            status: None,
            failed_objects: None,
            self_uri: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "BUILDINITIALIZING")]
    BUILDINITIALIZING,
    #[serde(rename = "BUILDINPROGRESS")]
    BUILDINPROGRESS,
    #[serde(rename = "NOTBUILT")]
    NOTBUILT,
    #[serde(rename = "OPERATIONAL")]
    OPERATIONAL,
    #[serde(rename = "OPERATIONALNEEDSREBUILD")]
    OPERATIONALNEEDSREBUILD,
}

impl Default for Status {
    fn default() -> Status {
        Self::BUILDINITIALIZING
    }
}
