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
pub struct OutcomeScoresResult {
    /// List of scored outcomes in the session.
    #[serde(rename = "outcomeScores", skip_serializing_if = "Option::is_none")]
    pub outcome_scores: Option<Vec<crate::models::OutcomeEventScore>>,
    /// Timestamp indicating the last time that the event was scored. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "modifiedDate", skip_serializing_if = "Option::is_none")]
    pub modified_date: Option<String>,
}

impl OutcomeScoresResult {
    pub fn new() -> OutcomeScoresResult {
        OutcomeScoresResult {
            outcome_scores: None,
            modified_date: None,
        }
    }
}


