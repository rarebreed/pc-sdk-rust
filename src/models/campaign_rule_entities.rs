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
pub struct CampaignRuleEntities {
    /// The list of campaigns for a CampaignRule to monitor. Required if the CampaignRule has any conditions that run on a campaign.
    #[serde(rename = "campaigns", skip_serializing_if = "Option::is_none")]
    pub campaigns: Option<Vec<crate::models::DomainEntityRef>>,
    /// The list of sequences for a CampaignRule to monitor. Required if the CampaignRule has any conditions that run on a sequence.
    #[serde(rename = "sequences", skip_serializing_if = "Option::is_none")]
    pub sequences: Option<Vec<crate::models::DomainEntityRef>>,
}

impl CampaignRuleEntities {
    pub fn new() -> CampaignRuleEntities {
        CampaignRuleEntities {
            campaigns: None,
            sequences: None,
        }
    }
}

