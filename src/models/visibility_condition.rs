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
pub struct VisibilityCondition {
    #[serde(rename = "combiningOperation", skip_serializing_if = "Option::is_none")]
    pub combining_operation: Option<CombiningOperation>,
    /// A list of strings, each representing the location in the form of the Answer Option to depend on. In the format of \"/form/questionGroup/{questionGroupIndex}/question/{questionIndex}/answer/{answerIndex}\" or, to assume the current question group, \"../question/{questionIndex}/answer/{answerIndex}\". Note: Indexes are zero-based
    #[serde(rename = "predicates", skip_serializing_if = "Option::is_none")]
    pub predicates: Option<Vec<serde_json::Value>>,
}

impl VisibilityCondition {
    pub fn new() -> VisibilityCondition {
        VisibilityCondition {
            combining_operation: None,
            predicates: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CombiningOperation {
    #[serde(rename = "AND")]
    AND,
    #[serde(rename = "OR")]
    OR,
}

impl Default for CombiningOperation {
    fn default() -> CombiningOperation {
        Self::AND
    }
}

