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
pub struct ScorableSurvey {
    /// The globally unique identifier for the object.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "surveyForm", skip_serializing_if = "Option::is_none")]
    pub survey_form: Option<Box<crate::models::SurveyForm>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(rename = "answers", skip_serializing_if = "Option::is_none")]
    pub answers: Option<Box<crate::models::SurveyScoringSet>>,
    /// The URI for this object
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
}

impl ScorableSurvey {
    pub fn new() -> ScorableSurvey {
        ScorableSurvey {
            id: None,
            name: None,
            survey_form: None,
            status: None,
            answers: None,
            self_uri: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "Pending")]
    Pending,
    #[serde(rename = "Sent")]
    Sent,
    #[serde(rename = "InProgress")]
    InProgress,
    #[serde(rename = "Finished")]
    Finished,
    #[serde(rename = "OptOut")]
    OptOut,
    #[serde(rename = "Error")]
    Error,
    #[serde(rename = "Expired")]
    Expired,
}

impl Default for Status {
    fn default() -> Status {
        Self::Pending
    }
}

