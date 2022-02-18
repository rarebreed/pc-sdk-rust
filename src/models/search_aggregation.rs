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
pub struct SearchAggregation {
    /// The field used for aggregation
    #[serde(rename = "field", skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    /// The name of the aggregation. The response aggregation uses this name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The type of aggregation to perform
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Type>,
    /// A value to use for aggregation
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// The number aggregations results to return out of the entire result set
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    /// The order in which aggregation results are sorted
    #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
    pub order: Option<Vec<Order>>,
}

impl SearchAggregation {
    pub fn new() -> SearchAggregation {
        SearchAggregation {
            field: None,
            name: None,
            _type: None,
            value: None,
            size: None,
            order: None,
        }
    }
}

/// The type of aggregation to perform
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "COUNT")]
    COUNT,
    #[serde(rename = "SUM")]
    SUM,
    #[serde(rename = "AVERAGE")]
    AVERAGE,
    #[serde(rename = "TERM")]
    TERM,
    #[serde(rename = "CONTAINS")]
    CONTAINS,
    #[serde(rename = "STARTS_WITH")]
    STARTSWITH,
    #[serde(rename = "ENDS_WITH")]
    ENDSWITH,
}

impl Default for Type {
    fn default() -> Type {
        Self::COUNT
    }
}
/// The order in which aggregation results are sorted
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Order {
    #[serde(rename = "VALUE_DESC")]
    VALUEDESC,
    #[serde(rename = "VALUE_ASC")]
    VALUEASC,
    #[serde(rename = "COUNT_DESC")]
    COUNTDESC,
    #[serde(rename = "COUNT_ASC")]
    COUNTASC,
}

impl Default for Order {
    fn default() -> Order {
        Self::VALUEDESC
    }
}
