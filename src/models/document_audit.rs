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
pub struct DocumentAudit {
    /// The globally unique identifier for the object.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<crate::models::DomainEntityRef>>,
    #[serde(rename = "workspace", skip_serializing_if = "Option::is_none")]
    pub workspace: Option<Box<crate::models::DomainEntityRef>>,
    #[serde(rename = "transactionId", skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    #[serde(rename = "transactionInitiator", skip_serializing_if = "Option::is_none")]
    pub transaction_initiator: Option<bool>,
    #[serde(rename = "application", skip_serializing_if = "Option::is_none")]
    pub application: Option<String>,
    #[serde(rename = "serviceName", skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    #[serde(rename = "level", skip_serializing_if = "Option::is_none")]
    pub level: Option<Level>,
    /// Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(rename = "actionContext", skip_serializing_if = "Option::is_none")]
    pub action_context: Option<ActionContext>,
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<Action>,
    #[serde(rename = "entity", skip_serializing_if = "Option::is_none")]
    pub entity: Option<Box<crate::models::AuditEntityReference>>,
    #[serde(rename = "changes", skip_serializing_if = "Option::is_none")]
    pub changes: Option<Vec<crate::models::AuditChange>>,
    /// The URI for this object
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
}

impl DocumentAudit {
    pub fn new() -> DocumentAudit {
        DocumentAudit {
            id: None,
            name: None,
            user: None,
            workspace: None,
            transaction_id: None,
            transaction_initiator: None,
            application: None,
            service_name: None,
            level: None,
            timestamp: None,
            status: None,
            action_context: None,
            action: None,
            entity: None,
            changes: None,
            self_uri: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Level {
    #[serde(rename = "USER")]
    USER,
    #[serde(rename = "SYSTEM")]
    SYSTEM,
}

impl Default for Level {
    fn default() -> Level {
        Self::USER
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "SUCCESS")]
    SUCCESS,
    #[serde(rename = "FAILURE")]
    FAILURE,
    #[serde(rename = "WARNING")]
    WARNING,
}

impl Default for Status {
    fn default() -> Status {
        Self::SUCCESS
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ActionContext {
    #[serde(rename = "CREATE")]
    CREATE,
    #[serde(rename = "READ")]
    READ,
    #[serde(rename = "UPDATE")]
    UPDATE,
    #[serde(rename = "DELETE")]
    DELETE,
    #[serde(rename = "DOWNLOAD")]
    DOWNLOAD,
    #[serde(rename = "VIEW")]
    VIEW,
    #[serde(rename = "UPLOAD")]
    UPLOAD,
    #[serde(rename = "SAVE")]
    SAVE,
    #[serde(rename = "MOVE")]
    _MOVE,
    #[serde(rename = "COPY")]
    COPY,
    #[serde(rename = "ADD")]
    ADD,
    #[serde(rename = "REMOVE")]
    REMOVE,
    #[serde(rename = "RECEIVE")]
    RECEIVE,
    #[serde(rename = "CONVERT")]
    CONVERT,
    #[serde(rename = "FAX")]
    FAX,
    #[serde(rename = "CREATE_COVERPAGE")]
    CREATECOVERPAGE,
    #[serde(rename = "USER_ADD")]
    USERADD,
    #[serde(rename = "USER_REMOVE")]
    USERREMOVE,
    #[serde(rename = "MEMBER_ADD")]
    MEMBERADD,
    #[serde(rename = "MEMBER_REMOVE")]
    MEMBERREMOVE,
    #[serde(rename = "MEMBER_UPDATE")]
    MEMBERUPDATE,
    #[serde(rename = "TAG_ADD")]
    TAGADD,
    #[serde(rename = "TAG_REMOVE")]
    TAGREMOVE,
    #[serde(rename = "TAG_UPDATE")]
    TAGUPDATE,
    #[serde(rename = "ATTRIBUTE_ADD")]
    ATTRIBUTEADD,
    #[serde(rename = "ATTRIBUTE_REMOVE")]
    ATTRIBUTEREMOVE,
    #[serde(rename = "ATTRIBUTE_UPDATE")]
    ATTRIBUTEUPDATE,
    #[serde(rename = "ATTRIBUTE_GROUP_INSTANCE_ADD")]
    ATTRIBUTEGROUPINSTANCEADD,
    #[serde(rename = "ATTRIBUTE_GROUP_INSTANCE_REMOVE")]
    ATTRIBUTEGROUPINSTANCEREMOVE,
    #[serde(rename = "ATTRIBUTE_GROUP_INSTANCE_UPDATE")]
    ATTRIBUTEGROUPINSTANCEUPDATE,
    #[serde(rename = "INDEX_SAVE")]
    INDEXSAVE,
    #[serde(rename = "INDEX_DELETE")]
    INDEXDELETE,
    #[serde(rename = "INDEX_CREATE")]
    INDEXCREATE,
    #[serde(rename = "FILE_SAVE")]
    FILESAVE,
    #[serde(rename = "FILE_DELETE")]
    FILEDELETE,
    #[serde(rename = "FILE_READ")]
    FILEREAD,
    #[serde(rename = "THUMBNAIL_CREATE")]
    THUMBNAILCREATE,
    #[serde(rename = "TEXT_EXTRACT")]
    TEXTEXTRACT,
    #[serde(rename = "SHARE_ADD")]
    SHAREADD,
    #[serde(rename = "SHARE_REMOVE")]
    SHAREREMOVE,
    #[serde(rename = "VERSION_CREATE")]
    VERSIONCREATE,
}

impl Default for ActionContext {
    fn default() -> ActionContext {
        Self::CREATE
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "CREATE")]
    CREATE,
    #[serde(rename = "READ")]
    READ,
    #[serde(rename = "UPDATE")]
    UPDATE,
    #[serde(rename = "DELETE")]
    DELETE,
    #[serde(rename = "DOWNLOAD")]
    DOWNLOAD,
    #[serde(rename = "VIEW")]
    VIEW,
    #[serde(rename = "UPLOAD")]
    UPLOAD,
    #[serde(rename = "SAVE")]
    SAVE,
    #[serde(rename = "MOVE")]
    _MOVE,
    #[serde(rename = "COPY")]
    COPY,
    #[serde(rename = "ADD")]
    ADD,
    #[serde(rename = "REMOVE")]
    REMOVE,
    #[serde(rename = "RECEIVE")]
    RECEIVE,
    #[serde(rename = "CONVERT")]
    CONVERT,
    #[serde(rename = "FAX")]
    FAX,
    #[serde(rename = "CREATE_COVERPAGE")]
    CREATECOVERPAGE,
    #[serde(rename = "USER_ADD")]
    USERADD,
    #[serde(rename = "USER_REMOVE")]
    USERREMOVE,
    #[serde(rename = "MEMBER_ADD")]
    MEMBERADD,
    #[serde(rename = "MEMBER_REMOVE")]
    MEMBERREMOVE,
    #[serde(rename = "MEMBER_UPDATE")]
    MEMBERUPDATE,
    #[serde(rename = "TAG_ADD")]
    TAGADD,
    #[serde(rename = "TAG_REMOVE")]
    TAGREMOVE,
    #[serde(rename = "TAG_UPDATE")]
    TAGUPDATE,
    #[serde(rename = "ATTRIBUTE_ADD")]
    ATTRIBUTEADD,
    #[serde(rename = "ATTRIBUTE_REMOVE")]
    ATTRIBUTEREMOVE,
    #[serde(rename = "ATTRIBUTE_UPDATE")]
    ATTRIBUTEUPDATE,
    #[serde(rename = "ATTRIBUTE_GROUP_INSTANCE_ADD")]
    ATTRIBUTEGROUPINSTANCEADD,
    #[serde(rename = "ATTRIBUTE_GROUP_INSTANCE_REMOVE")]
    ATTRIBUTEGROUPINSTANCEREMOVE,
    #[serde(rename = "ATTRIBUTE_GROUP_INSTANCE_UPDATE")]
    ATTRIBUTEGROUPINSTANCEUPDATE,
    #[serde(rename = "INDEX_SAVE")]
    INDEXSAVE,
    #[serde(rename = "INDEX_DELETE")]
    INDEXDELETE,
    #[serde(rename = "INDEX_CREATE")]
    INDEXCREATE,
    #[serde(rename = "FILE_SAVE")]
    FILESAVE,
    #[serde(rename = "FILE_DELETE")]
    FILEDELETE,
    #[serde(rename = "FILE_READ")]
    FILEREAD,
    #[serde(rename = "THUMBNAIL_CREATE")]
    THUMBNAILCREATE,
    #[serde(rename = "TEXT_EXTRACT")]
    TEXTEXTRACT,
    #[serde(rename = "SHARE_ADD")]
    SHAREADD,
    #[serde(rename = "SHARE_REMOVE")]
    SHAREREMOVE,
    #[serde(rename = "VERSION_CREATE")]
    VERSIONCREATE,
}

impl Default for Action {
    fn default() -> Action {
        Self::CREATE
    }
}

