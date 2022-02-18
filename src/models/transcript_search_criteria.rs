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
pub struct TranscriptSearchCriteria {
    /// The end value of the range. This field is used for range search types.
    #[serde(rename = "endValue", skip_serializing_if = "Option::is_none")]
    pub end_value: Option<String>,
    /// A list of values for the search to match against
    #[serde(rename = "values", skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
    /// The start value of the range. This field is used for range search types.
    #[serde(rename = "startValue", skip_serializing_if = "Option::is_none")]
    pub start_value: Option<String>,
    /// Field names to search against
    #[serde(rename = "fields", skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<String>>,
    /// A value for the search to match against
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// How to apply this search criteria against other criteria
    #[serde(rename = "operator", skip_serializing_if = "Option::is_none")]
    pub operator: Option<Operator>,
    /// Groups multiple conditions
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: Option<Vec<crate::models::TranscriptSearchCriteria>>,
    /// Set date format for criteria values when using date range search type.  Supports Java date format syntax, example yyyy-MM-dd'T'HH:mm:ss.SSSX.
    #[serde(rename = "dateFormat", skip_serializing_if = "Option::is_none")]
    pub date_format: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Type>,
}

impl TranscriptSearchCriteria {
    pub fn new() -> TranscriptSearchCriteria {
        TranscriptSearchCriteria {
            end_value: None,
            values: None,
            start_value: None,
            fields: None,
            value: None,
            operator: None,
            group: None,
            date_format: None,
            _type: None,
        }
    }
}

/// How to apply this search criteria against other criteria
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Operator {
    #[serde(rename = "AND")]
    AND,
    #[serde(rename = "OR")]
    OR,
    #[serde(rename = "NOT")]
    NOT,
}

impl Default for Operator {
    fn default() -> Operator {
        Self::AND
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "EXACT")]
    EXACT,
    #[serde(rename = "EXACT_PHRASE")]
    EXACTPHRASE,
    #[serde(rename = "PHRASE")]
    PHRASE,
    #[serde(rename = "DATE_RANGE")]
    DATERANGE,
    #[serde(rename = "RANGE")]
    RANGE,
    #[serde(rename = "GREATER_THAN")]
    GREATERTHAN,
    #[serde(rename = "LESS_THAN")]
    LESSTHAN,
}

impl Default for Type {
    fn default() -> Type {
        Self::EXACT
    }
}

