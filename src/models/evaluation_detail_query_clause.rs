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
pub struct EvaluationDetailQueryClause {
    /// Boolean operation to apply to the provided predicates
    #[serde(rename = "type")]
    pub _type: Type,
    /// Like a three-word sentence: (attribute-name) (operator) (target-value).
    #[serde(rename = "predicates")]
    pub predicates: Vec<crate::models::EvaluationDetailQueryPredicate>,
}

impl EvaluationDetailQueryClause {
    pub fn new(_type: Type, predicates: Vec<crate::models::EvaluationDetailQueryPredicate>) -> EvaluationDetailQueryClause {
        EvaluationDetailQueryClause {
            _type,
            predicates,
        }
    }
}

/// Boolean operation to apply to the provided predicates
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "and")]
    And,
    #[serde(rename = "or")]
    Or,
}

impl Default for Type {
    fn default() -> Type {
        Self::And
    }
}

