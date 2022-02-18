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
pub struct CalibrationCreate {
    /// The globally unique identifier for the object.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "calibrator", skip_serializing_if = "Option::is_none")]
    pub calibrator: Option<Box<crate::models::User>>,
    #[serde(rename = "agent", skip_serializing_if = "Option::is_none")]
    pub agent: Option<Box<crate::models::User>>,
    #[serde(rename = "conversation")]
    pub conversation: Box<crate::models::Conversation>,
    #[serde(rename = "evaluationForm", skip_serializing_if = "Option::is_none")]
    pub evaluation_form: Option<Box<crate::models::EvaluationForm>>,
    #[serde(rename = "contextId", skip_serializing_if = "Option::is_none")]
    pub context_id: Option<String>,
    #[serde(rename = "averageScore", skip_serializing_if = "Option::is_none")]
    pub average_score: Option<i32>,
    #[serde(rename = "highScore", skip_serializing_if = "Option::is_none")]
    pub high_score: Option<i32>,
    #[serde(rename = "lowScore", skip_serializing_if = "Option::is_none")]
    pub low_score: Option<i32>,
    /// Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "createdDate", skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    #[serde(rename = "evaluations", skip_serializing_if = "Option::is_none")]
    pub evaluations: Option<Vec<crate::models::Evaluation>>,
    #[serde(rename = "evaluators", skip_serializing_if = "Option::is_none")]
    pub evaluators: Option<Vec<crate::models::User>>,
    #[serde(rename = "scoringIndex", skip_serializing_if = "Option::is_none")]
    pub scoring_index: Option<Box<crate::models::Evaluation>>,
    #[serde(rename = "expertEvaluator", skip_serializing_if = "Option::is_none")]
    pub expert_evaluator: Option<Box<crate::models::User>>,
    /// The URI for this object
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
}

impl CalibrationCreate {
    pub fn new(conversation: crate::models::Conversation) -> CalibrationCreate {
        CalibrationCreate {
            id: None,
            name: None,
            calibrator: None,
            agent: None,
            conversation: Box::new(conversation),
            evaluation_form: None,
            context_id: None,
            average_score: None,
            high_score: None,
            low_score: None,
            created_date: None,
            evaluations: None,
            evaluators: None,
            scoring_index: None,
            expert_evaluator: None,
            self_uri: None,
        }
    }
}


