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
pub struct LearningAssessment {
    /// The Id of the assessment
    #[serde(rename = "assessmentId", skip_serializing_if = "Option::is_none")]
    pub assessment_id: Option<String>,
    /// The context Id of the related assessment form
    #[serde(rename = "contextId", skip_serializing_if = "Option::is_none")]
    pub context_id: Option<String>,
    /// The Id of the related assessment form
    #[serde(rename = "assessmentFormId", skip_serializing_if = "Option::is_none")]
    pub assessment_form_id: Option<String>,
    /// Status of the assessment
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(rename = "answers", skip_serializing_if = "Option::is_none")]
    pub answers: Option<Box<crate::models::AssessmentScoringSet>>,
    /// Date the assessment was created. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "dateCreated", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// Date the assessment was last updated. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "dateModified", skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<String>,
    /// Date the assessment was submitted. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "dateSubmitted", skip_serializing_if = "Option::is_none")]
    pub date_submitted: Option<String>,
}

impl LearningAssessment {
    pub fn new() -> LearningAssessment {
        LearningAssessment {
            assessment_id: None,
            context_id: None,
            assessment_form_id: None,
            status: None,
            answers: None,
            date_created: None,
            date_modified: None,
            date_submitted: None,
        }
    }
}

/// Status of the assessment
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "Pending")]
    Pending,
    #[serde(rename = "InProgress")]
    InProgress,
    #[serde(rename = "Finished")]
    Finished,
}

impl Default for Status {
    fn default() -> Status {
        Self::Pending
    }
}

