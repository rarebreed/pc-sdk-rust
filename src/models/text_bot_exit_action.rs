/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// TextBotExitAction : Settings for a next-action of exiting the bot flow. Any output variables are available in the details.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TextBotExitAction {
    /// The reason for the exit.
    #[serde(rename = "reason")]
    pub reason: Reason,
    /// Extended information related to the reason, if available.
    #[serde(rename = "reasonExtendedInfo", skip_serializing_if = "Option::is_none")]
    pub reason_extended_info: Option<String>,
    /// The active intent at the time of the exit.
    #[serde(rename = "activeIntent", skip_serializing_if = "Option::is_none")]
    pub active_intent: Option<String>,
    #[serde(rename = "flowLocation", skip_serializing_if = "Option::is_none")]
    pub flow_location: Option<Box<crate::models::TextBotFlowLocation>>,
    #[serde(rename = "outputData", skip_serializing_if = "Option::is_none")]
    pub output_data: Option<Box<crate::models::TextBotInputOutputData>>,
    /// The list of Flow Outcomes for the bot flow and their details.
    #[serde(rename = "flowOutcomes", skip_serializing_if = "Option::is_none")]
    pub flow_outcomes: Option<Vec<crate::models::TextBotFlowOutcome>>,
}

impl TextBotExitAction {
    /// Settings for a next-action of exiting the bot flow. Any output variables are available in the details.
    pub fn new(reason: Reason) -> TextBotExitAction {
        TextBotExitAction {
            reason,
            reason_extended_info: None,
            active_intent: None,
            flow_location: None,
            output_data: None,
            flow_outcomes: None,
        }
    }
}

/// The reason for the exit.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Reason {
    #[serde(rename = "TriggeredByUser")]
    TriggeredByUser,
    #[serde(rename = "AgentRequestedByUser")]
    AgentRequestedByUser,
    #[serde(rename = "TriggeredByFlow")]
    TriggeredByFlow,
    #[serde(rename = "Error")]
    Error,
    #[serde(rename = "RecognitionFailure")]
    RecognitionFailure,
}

impl Default for Reason {
    fn default() -> Reason {
        Self::TriggeredByUser
    }
}

