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
pub struct RecallEntry {
    #[serde(rename = "nbrAttempts", skip_serializing_if = "Option::is_none")]
    pub nbr_attempts: Option<i32>,
    #[serde(rename = "minutesBetweenAttempts", skip_serializing_if = "Option::is_none")]
    pub minutes_between_attempts: Option<i32>,
}

impl RecallEntry {
    pub fn new() -> RecallEntry {
        RecallEntry {
            nbr_attempts: None,
            minutes_between_attempts: None,
        }
    }
}

