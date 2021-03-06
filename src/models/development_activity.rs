/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// DevelopmentActivity : Development Activity object



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DevelopmentActivity {
    /// The globally unique identifier for the object.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Date that activity was completed. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "dateCompleted", skip_serializing_if = "Option::is_none")]
    pub date_completed: Option<String>,
    #[serde(rename = "createdBy", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<Box<crate::models::UserReference>>,
    /// Date activity was created. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "dateCreated", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The user's percentage score for this activity
    #[serde(rename = "percentageScore", skip_serializing_if = "Option::is_none")]
    pub percentage_score: Option<f32>,
    /// True if the activity was passed
    #[serde(rename = "isPassed", skip_serializing_if = "Option::is_none")]
    pub is_passed: Option<bool>,
    /// The URI for this object
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
    /// The name of the activity
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The type of activity
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Type>,
    /// The status of the activity
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// Due date for completion of the activity. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "dateDue", skip_serializing_if = "Option::is_none")]
    pub date_due: Option<String>,
    #[serde(rename = "facilitator", skip_serializing_if = "Option::is_none")]
    pub facilitator: Option<Box<crate::models::UserReference>>,
    /// List of users attending the activity
    #[serde(rename = "attendees", skip_serializing_if = "Option::is_none")]
    pub attendees: Option<Vec<crate::models::UserReference>>,
    /// Indicates if the activity is overdue
    #[serde(rename = "isOverdue", skip_serializing_if = "Option::is_none")]
    pub is_overdue: Option<bool>,
}

impl DevelopmentActivity {
    /// Development Activity object
    pub fn new() -> DevelopmentActivity {
        DevelopmentActivity {
            id: None,
            date_completed: None,
            created_by: None,
            date_created: None,
            percentage_score: None,
            is_passed: None,
            self_uri: None,
            name: None,
            _type: None,
            status: None,
            date_due: None,
            facilitator: None,
            attendees: None,
            is_overdue: None,
        }
    }
}

/// The type of activity
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "Informational")]
    Informational,
    #[serde(rename = "Coaching")]
    Coaching,
    #[serde(rename = "AssessedContent")]
    AssessedContent,
    #[serde(rename = "Assessment")]
    Assessment,
}

impl Default for Type {
    fn default() -> Type {
        Self::Informational
    }
}
/// The status of the activity
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "Planned")]
    Planned,
    #[serde(rename = "InProgress")]
    InProgress,
    #[serde(rename = "Completed")]
    Completed,
    #[serde(rename = "InvalidSchedule")]
    InvalidSchedule,
    #[serde(rename = "NotCompleted")]
    NotCompleted,
}

impl Default for Status {
    fn default() -> Status {
        Self::Planned
    }
}

