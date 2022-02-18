/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// LearningAssignment : Learning module assignment with user information



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LearningAssignment {
    /// The globally unique identifier for the object.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "assessment", skip_serializing_if = "Option::is_none")]
    pub assessment: Option<Box<crate::models::LearningAssessment>>,
    #[serde(rename = "createdBy", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<Box<crate::models::UserReference>>,
    /// The date when the assignment was created. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "dateCreated", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    #[serde(rename = "modifiedBy", skip_serializing_if = "Option::is_none")]
    pub modified_by: Option<Box<crate::models::UserReference>>,
    /// The date when the assignment was last modified. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "dateModified", skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<String>,
    /// True if the assignment is overdue
    #[serde(rename = "isOverdue", skip_serializing_if = "Option::is_none")]
    pub is_overdue: Option<bool>,
    /// The user's percentage score for this assignment
    #[serde(rename = "percentageScore", skip_serializing_if = "Option::is_none")]
    pub percentage_score: Option<f32>,
    /// True if this assignment was created by a Rule
    #[serde(rename = "isRule", skip_serializing_if = "Option::is_none")]
    pub is_rule: Option<bool>,
    /// True if this assignment was created manually
    #[serde(rename = "isManual", skip_serializing_if = "Option::is_none")]
    pub is_manual: Option<bool>,
    /// True if the assessment was passed
    #[serde(rename = "isPassed", skip_serializing_if = "Option::is_none")]
    pub is_passed: Option<bool>,
    /// The URI for this object
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
    /// The Learning Assignment state
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
    /// The recommended completion date of the assignment. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "dateRecommendedForCompletion", skip_serializing_if = "Option::is_none")]
    pub date_recommended_for_completion: Option<String>,
    /// The version of Learning module assigned
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
    #[serde(rename = "module", skip_serializing_if = "Option::is_none")]
    pub module: Option<Box<crate::models::LearningModule>>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<crate::models::UserReference>>,
    #[serde(rename = "assessmentForm", skip_serializing_if = "Option::is_none")]
    pub assessment_form: Option<Box<crate::models::AssessmentForm>>,
}

impl LearningAssignment {
    /// Learning module assignment with user information
    pub fn new() -> LearningAssignment {
        LearningAssignment {
            id: None,
            assessment: None,
            created_by: None,
            date_created: None,
            modified_by: None,
            date_modified: None,
            is_overdue: None,
            percentage_score: None,
            is_rule: None,
            is_manual: None,
            is_passed: None,
            self_uri: None,
            state: None,
            date_recommended_for_completion: None,
            version: None,
            module: None,
            user: None,
            assessment_form: None,
        }
    }
}

/// The Learning Assignment state
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "Assigned")]
    Assigned,
    #[serde(rename = "InProgress")]
    InProgress,
    #[serde(rename = "Completed")]
    Completed,
    #[serde(rename = "Deleted")]
    Deleted,
    #[serde(rename = "NotCompleted")]
    NotCompleted,
}

impl Default for State {
    fn default() -> State {
        Self::Assigned
    }
}

