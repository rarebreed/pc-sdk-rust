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
pub struct ResolutionDetailQueryPredicate {
    /// Optional type, can usually be inferred
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Type>,
    /// Left hand side for metric predicates
    #[serde(rename = "metric", skip_serializing_if = "Option::is_none")]
    pub metric: Option<Metric>,
    /// Optional operator, default is matches
    #[serde(rename = "operator", skip_serializing_if = "Option::is_none")]
    pub operator: Option<Operator>,
    /// Right hand side for metric predicates
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "range", skip_serializing_if = "Option::is_none")]
    pub range: Option<Box<crate::models::NumericRange>>,
}

impl ResolutionDetailQueryPredicate {
    pub fn new() -> ResolutionDetailQueryPredicate {
        ResolutionDetailQueryPredicate {
            _type: None,
            metric: None,
            operator: None,
            value: None,
            range: None,
        }
    }
}

/// Optional type, can usually be inferred
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "dimension")]
    Dimension,
    #[serde(rename = "property")]
    Property,
    #[serde(rename = "metric")]
    Metric,
}

impl Default for Type {
    fn default() -> Type {
        Self::Dimension
    }
}
/// Left hand side for metric predicates
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Metric {
    #[serde(rename = "nNextContactAvoided")]
    NNextContactAvoided,
}

impl Default for Metric {
    fn default() -> Metric {
        Self::NNextContactAvoided
    }
}
/// Optional operator, default is matches
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Operator {
    #[serde(rename = "matches")]
    Matches,
    #[serde(rename = "exists")]
    Exists,
    #[serde(rename = "notExists")]
    NotExists,
}

impl Default for Operator {
    fn default() -> Operator {
        Self::Matches
    }
}
