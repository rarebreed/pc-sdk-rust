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
pub struct CampaignRuleAction {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Box<crate::models::CampaignRuleParameters>>,
    /// The action to take on the campaignRuleActionEntities.
    #[serde(rename = "actionType")]
    pub action_type: ActionType,
    #[serde(rename = "campaignRuleActionEntities")]
    pub campaign_rule_action_entities: Box<crate::models::CampaignRuleActionEntities>,
}

impl CampaignRuleAction {
    pub fn new(action_type: ActionType, campaign_rule_action_entities: crate::models::CampaignRuleActionEntities) -> CampaignRuleAction {
        CampaignRuleAction {
            id: None,
            parameters: None,
            action_type,
            campaign_rule_action_entities: Box::new(campaign_rule_action_entities),
        }
    }
}

/// The action to take on the campaignRuleActionEntities.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ActionType {
    #[serde(rename = "turnOnCampaign")]
    TurnOnCampaign,
    #[serde(rename = "turnOffCampaign")]
    TurnOffCampaign,
    #[serde(rename = "turnOnSequence")]
    TurnOnSequence,
    #[serde(rename = "turnOffSequence")]
    TurnOffSequence,
    #[serde(rename = "setCampaignPriority")]
    SetCampaignPriority,
    #[serde(rename = "recycleCampaign")]
    RecycleCampaign,
    #[serde(rename = "setCampaignDialingMode")]
    SetCampaignDialingMode,
}

impl Default for ActionType {
    fn default() -> ActionType {
        Self::TurnOnCampaign
    }
}
