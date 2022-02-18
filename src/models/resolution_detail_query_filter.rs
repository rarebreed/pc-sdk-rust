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
pub struct ResolutionDetailQueryFilter {
    /// Boolean operation to apply to the provided predicates and clauses
    #[serde(rename = "type")]
    pub _type: Type,
    /// Boolean 'and/or' logic with up to two-levels of nesting
    #[serde(rename = "clauses", skip_serializing_if = "Option::is_none")]
    pub clauses: Option<Vec<crate::models::ResolutionDetailQueryClause>>,
    /// Like a three-word sentence: (attribute-name) (operator) (target-value).
    #[serde(rename = "predicates", skip_serializing_if = "Option::is_none")]
    pub predicates: Option<Vec<crate::models::ResolutionDetailQueryPredicate>>,
}

impl ResolutionDetailQueryFilter {
    pub fn new(_type: Type) -> ResolutionDetailQueryFilter {
        ResolutionDetailQueryFilter {
            _type,
            clauses: None,
            predicates: None,
        }
    }
}

/// Boolean operation to apply to the provided predicates and clauses
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
