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
pub struct TopicRequest {
    /// The topic name
    #[serde(rename = "name")]
    pub name: String,
    /// The topic description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The topic strictness, default value is 72
    #[serde(rename = "strictness", skip_serializing_if = "Option::is_none")]
    pub strictness: Option<Strictness>,
    /// The ids of programs associated to the topic
    #[serde(rename = "programIds", skip_serializing_if = "Option::is_none")]
    pub program_ids: Option<Vec<String>>,
    /// The topic tags
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// The topic dialect
    #[serde(rename = "dialect")]
    pub dialect: String,
    /// The topic participants, default value is All
    #[serde(rename = "participants", skip_serializing_if = "Option::is_none")]
    pub participants: Option<Participants>,
    /// The topic phrases
    #[serde(rename = "phrases", skip_serializing_if = "Option::is_none")]
    pub phrases: Option<Vec<crate::models::Phrase>>,
}

impl TopicRequest {
    pub fn new(name: String, dialect: String) -> TopicRequest {
        TopicRequest {
            name,
            description: None,
            strictness: None,
            program_ids: None,
            tags: None,
            dialect,
            participants: None,
            phrases: None,
        }
    }
}

/// The topic strictness, default value is 72
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Strictness {
    #[serde(rename = "1")]
    _1,
    #[serde(rename = "55")]
    _55,
    #[serde(rename = "65")]
    _65,
    #[serde(rename = "72")]
    _72,
    #[serde(rename = "85")]
    _85,
    #[serde(rename = "90")]
    _90,
}

impl Default for Strictness {
    fn default() -> Strictness {
        Self::_1
    }
}
/// The topic participants, default value is All
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Participants {
    #[serde(rename = "External")]
    External,
    #[serde(rename = "Internal")]
    Internal,
    #[serde(rename = "All")]
    All,
}

impl Default for Participants {
    fn default() -> Participants {
        Self::External
    }
}

