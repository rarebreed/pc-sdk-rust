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
pub struct WorkPlanConstraintConflictMessage {
    /// Type of constraint conflict that can be resolved by clients in order to generate agent schedules
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Type>,
    /// The arguments to the type of the message that can help clients resolve validation issues
    #[serde(rename = "arguments", skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Vec<crate::models::WorkPlanValidationMessageArgument>>,
}

impl WorkPlanConstraintConflictMessage {
    pub fn new() -> WorkPlanConstraintConflictMessage {
        WorkPlanConstraintConflictMessage {
            _type: None,
            arguments: None,
        }
    }
}

/// Type of constraint conflict that can be resolved by clients in order to generate agent schedules
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "UnknownFix")]
    UnknownFix,
    #[serde(rename = "WithPotentialFixes")]
    WithPotentialFixes,
}

impl Default for Type {
    fn default() -> Type {
        Self::UnknownFix
    }
}
