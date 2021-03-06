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
pub struct QualityAuditLogMessage {
    /// Id of the audit message.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Home Organization Id associated with this audit message.
    #[serde(rename = "userHomeOrgId", skip_serializing_if = "Option::is_none")]
    pub user_home_org_id: Option<String>,
    /// Trustee Organization Id if this audit message is from trustee access.
    #[serde(rename = "userTrusteeOrgId", skip_serializing_if = "Option::is_none")]
    pub user_trustee_org_id: Option<String>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<crate::models::DomainEntityRef>>,
    #[serde(rename = "client", skip_serializing_if = "Option::is_none")]
    pub client: Option<Box<crate::models::AddressableEntityRef>>,
    /// List of IP addresses of systems that originated or handled the request.
    #[serde(rename = "remoteIps", skip_serializing_if = "Option::is_none")]
    pub remote_ips: Option<Vec<String>>,
    /// Name of the service that logged this audit message.
    #[serde(rename = "serviceName", skip_serializing_if = "Option::is_none")]
    pub service_name: Option<ServiceName>,
    /// The level of this audit message.
    #[serde(rename = "level", skip_serializing_if = "Option::is_none")]
    pub level: Option<Level>,
    /// The status of the action of this audit message.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// Date and time of when the audit message was logged. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "eventDate", skip_serializing_if = "Option::is_none")]
    pub event_date: Option<String>,
    #[serde(rename = "messageInfo", skip_serializing_if = "Option::is_none")]
    pub message_info: Option<Box<crate::models::MessageInfo>>,
    /// Action that took place.
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<Action>,
    #[serde(rename = "entity", skip_serializing_if = "Option::is_none")]
    pub entity: Option<Box<crate::models::DomainEntityRef>>,
    /// Type of the entity that was impacted.
    #[serde(rename = "entityType", skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<EntityType>,
    /// List of properties that were changed and changes made to those properties.
    #[serde(rename = "propertyChanges", skip_serializing_if = "Option::is_none")]
    pub property_changes: Option<Vec<crate::models::PropertyChange>>,
    /// Additional context for this message.
    #[serde(rename = "context", skip_serializing_if = "Option::is_none")]
    pub context: Option<::std::collections::HashMap<String, String>>,
}

impl QualityAuditLogMessage {
    pub fn new() -> QualityAuditLogMessage {
        QualityAuditLogMessage {
            id: None,
            user_home_org_id: None,
            user_trustee_org_id: None,
            user: None,
            client: None,
            remote_ips: None,
            service_name: None,
            level: None,
            status: None,
            event_date: None,
            message_info: None,
            action: None,
            entity: None,
            entity_type: None,
            property_changes: None,
            context: None,
        }
    }
}

/// Name of the service that logged this audit message.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ServiceName {
    #[serde(rename = "RecordingService")]
    RecordingService,
    #[serde(rename = "RecordingPlaybackService")]
    RecordingPlaybackService,
    #[serde(rename = "QualityService")]
    QualityService,
}

impl Default for ServiceName {
    fn default() -> ServiceName {
        Self::RecordingService
    }
}
/// The level of this audit message.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Level {
    #[serde(rename = "User")]
    User,
    #[serde(rename = "System")]
    System,
}

impl Default for Level {
    fn default() -> Level {
        Self::User
    }
}
/// The status of the action of this audit message.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "Success")]
    Success,
    #[serde(rename = "Failure")]
    Failure,
    #[serde(rename = "Warning")]
    Warning,
}

impl Default for Status {
    fn default() -> Status {
        Self::Success
    }
}
/// Action that took place.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "Read")]
    Read,
    #[serde(rename = "Create")]
    Create,
    #[serde(rename = "Update")]
    Update,
    #[serde(rename = "Delete")]
    Delete,
    #[serde(rename = "Abandon")]
    Abandon,
    #[serde(rename = "Archive")]
    Archive,
    #[serde(rename = "Export")]
    Export,
    #[serde(rename = "RestoreRequest")]
    RestoreRequest,
    #[serde(rename = "RestoreComplete")]
    RestoreComplete,
    #[serde(rename = "ApplyProtection")]
    ApplyProtection,
    #[serde(rename = "RevokeProtection")]
    RevokeProtection,
    #[serde(rename = "UpdateRetention")]
    UpdateRetention,
}

impl Default for Action {
    fn default() -> Action {
        Self::Read
    }
}
/// Type of the entity that was impacted.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EntityType {
    #[serde(rename = "Recording")]
    Recording,
    #[serde(rename = "Evaluation")]
    Evaluation,
    #[serde(rename = "Calibration")]
    Calibration,
    #[serde(rename = "Annotation")]
    Annotation,
    #[serde(rename = "ScreenRecording")]
    ScreenRecording,
    #[serde(rename = "Survey")]
    Survey,
}

impl Default for EntityType {
    fn default() -> EntityType {
        Self::Recording
    }
}

