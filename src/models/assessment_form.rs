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
pub struct AssessmentForm {
    /// The globally unique identifier for the object.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Last modified date of the assessment form. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "dateModified", skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<String>,
    /// The unique Id for all versions of this assessment form
    #[serde(rename = "contextId", skip_serializing_if = "Option::is_none")]
    pub context_id: Option<String>,
    /// The URI for this object
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
    /// If true, assessment form is published
    #[serde(rename = "published", skip_serializing_if = "Option::is_none")]
    pub published: Option<bool>,
    /// The pass percent for the assessment form
    #[serde(rename = "passPercent")]
    pub pass_percent: i32,
    /// A list of question groups
    #[serde(rename = "questionGroups")]
    pub question_groups: Vec<crate::models::AssessmentFormQuestionGroup>,
}

impl AssessmentForm {
    pub fn new(pass_percent: i32, question_groups: Vec<crate::models::AssessmentFormQuestionGroup>) -> AssessmentForm {
        AssessmentForm {
            id: None,
            date_modified: None,
            context_id: None,
            self_uri: None,
            published: None,
            pass_percent,
            question_groups,
        }
    }
}


