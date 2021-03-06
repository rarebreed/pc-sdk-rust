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
pub struct MessageMediaPolicyConditions {
    #[serde(rename = "forUsers", skip_serializing_if = "Option::is_none")]
    pub for_users: Option<Vec<crate::models::User>>,
    #[serde(rename = "dateRanges", skip_serializing_if = "Option::is_none")]
    pub date_ranges: Option<Vec<String>>,
    #[serde(rename = "forQueues", skip_serializing_if = "Option::is_none")]
    pub for_queues: Option<Vec<crate::models::Queue>>,
    #[serde(rename = "wrapupCodes", skip_serializing_if = "Option::is_none")]
    pub wrapup_codes: Option<Vec<crate::models::WrapupCode>>,
    #[serde(rename = "languages", skip_serializing_if = "Option::is_none")]
    pub languages: Option<Vec<crate::models::Language>>,
    #[serde(rename = "timeAllowed", skip_serializing_if = "Option::is_none")]
    pub time_allowed: Option<Box<crate::models::TimeAllowed>>,
    #[serde(rename = "customerParticipation", skip_serializing_if = "Option::is_none")]
    pub customer_participation: Option<CustomerParticipation>,
}

impl MessageMediaPolicyConditions {
    pub fn new() -> MessageMediaPolicyConditions {
        MessageMediaPolicyConditions {
            for_users: None,
            date_ranges: None,
            for_queues: None,
            wrapup_codes: None,
            languages: None,
            time_allowed: None,
            customer_participation: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CustomerParticipation {
    #[serde(rename = "YES")]
    YES,
    #[serde(rename = "NO")]
    NO,
}

impl Default for CustomerParticipation {
    fn default() -> CustomerParticipation {
        Self::YES
    }
}

