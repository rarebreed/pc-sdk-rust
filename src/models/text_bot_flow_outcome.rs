/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// TextBotFlowOutcome : Flow Outcome data related to a bot flow which is exiting gracefully.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TextBotFlowOutcome {
    /// The Flow Outcome ID.
    #[serde(rename = "outcomeId", skip_serializing_if = "Option::is_none")]
    pub outcome_id: Option<String>,
    /// The value of the FlowOutcome.
    #[serde(rename = "outcomeValue", skip_serializing_if = "Option::is_none")]
    pub outcome_value: Option<OutcomeValue>,
    /// The timestamp for when the Flow Outcome began. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "dateStart", skip_serializing_if = "Option::is_none")]
    pub date_start: Option<String>,
    /// The timestamp for when the Flow Outcome finished. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "dateEnd", skip_serializing_if = "Option::is_none")]
    pub date_end: Option<String>,
    /// The Flow Milestones for the Flow Outcome.
    #[serde(rename = "milestones", skip_serializing_if = "Option::is_none")]
    pub milestones: Option<Vec<crate::models::TextBotFlowMilestone>>,
}

impl TextBotFlowOutcome {
    /// Flow Outcome data related to a bot flow which is exiting gracefully.
    pub fn new() -> TextBotFlowOutcome {
        TextBotFlowOutcome {
            outcome_id: None,
            outcome_value: None,
            date_start: None,
            date_end: None,
            milestones: None,
        }
    }
}

/// The value of the FlowOutcome.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OutcomeValue {
    #[serde(rename = "SUCCESS")]
    SUCCESS,
    #[serde(rename = "FAILURE")]
    FAILURE,
}

impl Default for OutcomeValue {
    fn default() -> OutcomeValue {
        Self::SUCCESS
    }
}

