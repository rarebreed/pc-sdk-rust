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
pub struct RoutingSettings {
    /// Reset agent score when agent presence changes from off-queue to on-queue
    #[serde(rename = "resetAgentScoreOnPresenceChange", skip_serializing_if = "Option::is_none")]
    pub reset_agent_score_on_presence_change: Option<bool>,
}

impl RoutingSettings {
    pub fn new() -> RoutingSettings {
        RoutingSettings {
            reset_agent_score_on_presence_change: None,
        }
    }
}


