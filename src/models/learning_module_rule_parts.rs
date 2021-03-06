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
pub struct LearningModuleRuleParts {
    /// The learning module rule operation
    #[serde(rename = "operation")]
    pub operation: Operation,
    /// The learning module rule selector
    #[serde(rename = "selector")]
    pub selector: Selector,
    /// The value of rules
    #[serde(rename = "value")]
    pub value: Vec<String>,
    /// The order of rules in learning module rule
    #[serde(rename = "order")]
    pub order: i32,
}

impl LearningModuleRuleParts {
    pub fn new(operation: Operation, selector: Selector, value: Vec<String>, order: i32) -> LearningModuleRuleParts {
        LearningModuleRuleParts {
            operation,
            selector,
            value,
            order,
        }
    }
}

/// The learning module rule operation
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Operation {
    #[serde(rename = "Include")]
    Include,
    #[serde(rename = "Exclude")]
    Exclude,
}

impl Default for Operation {
    fn default() -> Operation {
        Self::Include
    }
}
/// The learning module rule selector
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Selector {
    #[serde(rename = "AcdSkills")]
    AcdSkills,
    #[serde(rename = "AgentName")]
    AgentName,
    #[serde(rename = "Division")]
    Division,
    #[serde(rename = "Group")]
    Group,
    #[serde(rename = "Location")]
    Location,
    #[serde(rename = "Queue")]
    Queue,
    #[serde(rename = "Role")]
    Role,
    #[serde(rename = "Team")]
    Team,
}

impl Default for Selector {
    fn default() -> Selector {
        Self::AcdSkills
    }
}

