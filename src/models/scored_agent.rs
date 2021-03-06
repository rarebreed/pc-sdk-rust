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
pub struct ScoredAgent {
    #[serde(rename = "agent", skip_serializing_if = "Option::is_none")]
    pub agent: Option<Box<crate::models::DomainEntityRef>>,
    /// Agent's score for the current conversation, from 0 - 100, higher being better
    #[serde(rename = "score", skip_serializing_if = "Option::is_none")]
    pub score: Option<i32>,
}

impl ScoredAgent {
    pub fn new() -> ScoredAgent {
        ScoredAgent {
            agent: None,
            score: None,
        }
    }
}


