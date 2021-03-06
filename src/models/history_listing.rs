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
pub struct HistoryListing {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "complete", skip_serializing_if = "Option::is_none")]
    pub complete: Option<bool>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<crate::models::User>>,
    #[serde(rename = "client", skip_serializing_if = "Option::is_none")]
    pub client: Option<Box<crate::models::DomainEntityRef>>,
    #[serde(rename = "errorMessage", skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "errorCode", skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "errorDetails", skip_serializing_if = "Option::is_none")]
    pub error_details: Option<Vec<crate::models::Detail>>,
    #[serde(rename = "errorMessageParams", skip_serializing_if = "Option::is_none")]
    pub error_message_params: Option<::std::collections::HashMap<String, String>>,
    /// Action name
    #[serde(rename = "actionName", skip_serializing_if = "Option::is_none")]
    pub action_name: Option<ActionName>,
    /// Action status
    #[serde(rename = "actionStatus", skip_serializing_if = "Option::is_none")]
    pub action_status: Option<ActionStatus>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "system", skip_serializing_if = "Option::is_none")]
    pub system: Option<bool>,
    /// Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "started", skip_serializing_if = "Option::is_none")]
    pub started: Option<String>,
    /// Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "completed", skip_serializing_if = "Option::is_none")]
    pub completed: Option<String>,
    #[serde(rename = "pageSize", skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(rename = "pageNumber", skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    #[serde(rename = "entities", skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<crate::models::HistoryEntry>>,
    #[serde(rename = "pageCount", skip_serializing_if = "Option::is_none")]
    pub page_count: Option<i32>,
}

impl HistoryListing {
    pub fn new() -> HistoryListing {
        HistoryListing {
            id: None,
            complete: None,
            user: None,
            client: None,
            error_message: None,
            error_code: None,
            error_details: None,
            error_message_params: None,
            action_name: None,
            action_status: None,
            name: None,
            description: None,
            system: None,
            started: None,
            completed: None,
            page_size: None,
            page_number: None,
            total: None,
            entities: None,
            page_count: None,
        }
    }
}

/// Action name
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ActionName {
    #[serde(rename = "CREATE")]
    CREATE,
    #[serde(rename = "CHECKIN")]
    CHECKIN,
    #[serde(rename = "DEBUG")]
    DEBUG,
    #[serde(rename = "DELETE")]
    DELETE,
    #[serde(rename = "HISTORY")]
    HISTORY,
    #[serde(rename = "PUBLISH")]
    PUBLISH,
    #[serde(rename = "STATE_CHANGE")]
    STATECHANGE,
    #[serde(rename = "UPDATE")]
    UPDATE,
    #[serde(rename = "VALIDATE")]
    VALIDATE,
}

impl Default for ActionName {
    fn default() -> ActionName {
        Self::CREATE
    }
}
/// Action status
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ActionStatus {
    #[serde(rename = "LOCKED")]
    LOCKED,
    #[serde(rename = "UNLOCKED")]
    UNLOCKED,
    #[serde(rename = "STARTED")]
    STARTED,
    #[serde(rename = "PENDING_GENERATION")]
    PENDINGGENERATION,
    #[serde(rename = "PENDING_BACKEND_NOTIFICATION")]
    PENDINGBACKENDNOTIFICATION,
    #[serde(rename = "SUCCESS")]
    SUCCESS,
    #[serde(rename = "FAILURE")]
    FAILURE,
}

impl Default for ActionStatus {
    fn default() -> ActionStatus {
        Self::LOCKED
    }
}

