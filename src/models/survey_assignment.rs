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
pub struct SurveyAssignment {
    #[serde(rename = "surveyForm", skip_serializing_if = "Option::is_none")]
    pub survey_form: Option<Box<crate::models::PublishedSurveyFormReference>>,
    #[serde(rename = "flow", skip_serializing_if = "Option::is_none")]
    pub flow: Option<Box<crate::models::DomainEntityRef>>,
    /// An ISO 8601 repeated interval consisting of the number of repetitions, the start datetime, and the interval (e.g. R2/2018-03-01T13:00:00Z/P1M10DT2H30M). Total duration must not exceed 90 days.
    #[serde(rename = "inviteTimeInterval", skip_serializing_if = "Option::is_none")]
    pub invite_time_interval: Option<String>,
    /// User together with sendingDomain used to send email, null to use no-reply
    #[serde(rename = "sendingUser", skip_serializing_if = "Option::is_none")]
    pub sending_user: Option<String>,
    /// Validated email domain, required
    #[serde(rename = "sendingDomain")]
    pub sending_domain: String,
}

impl SurveyAssignment {
    pub fn new(sending_domain: String) -> SurveyAssignment {
        SurveyAssignment {
            survey_form: None,
            flow: None,
            invite_time_interval: None,
            sending_user: None,
            sending_domain,
        }
    }
}


